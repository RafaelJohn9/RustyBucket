use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::utils::bitfield::Bitfield;

/// Default block size used for requests (16 KiB).
pub const BLOCK_SIZE: usize = 16 * 1024;

#[derive(Debug)]
pub enum PieceManagerError {
    InvalidPieceIndex,
    InvalidBlockOffset,
    InvalidBlockSize,
    VerificationFailed,
}

/// A request for a block from a peer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub piece_index: usize,
    pub begin: usize,
    pub length: usize,
}

/// Metadata for a single piece.
struct PieceMeta {
    hash: Vec<u8>,
    length: usize,
    data: Vec<u8>,           // allocated to piece length; zeros when not set
    downloaded: Vec<bool>,   // per-block downloaded flags
    requested: HashSet<usize>, // block indices currently requested
    complete: bool,
}

impl PieceMeta {
    fn new(hash: Vec<u8>, length: usize) -> Self {
        let blocks = (length + BLOCK_SIZE - 1) / BLOCK_SIZE;
        Self {
            hash,
            length,
            data: vec![0u8; length],
            downloaded: vec![false; blocks],
            requested: HashSet::new(),
            complete: false,
        }
    }

    fn blocks_count(&self) -> usize {
        self.downloaded.len()
    }

    fn block_len(&self, block_idx: usize) -> Option<usize> {
        if block_idx >= self.blocks_count() {
            return None;
        }
        let start = block_idx * BLOCK_SIZE;
        let end = std::cmp::min(self.length, start + BLOCK_SIZE);
        Some(end - start)
    }

    fn is_fully_downloaded(&self) -> bool {
        self.downloaded.iter().all(|&b| b)
    }

    fn reset_partial(&mut self) {
        self.data.fill(0);
        for b in &mut self.downloaded {
            *b = false;
        }
        self.requested.clear();
        self.complete = false;
    }
}

/// PieceManager manages which pieces/blocks are needed, requested, and verifies them.
///
/// Design note:
/// - A sha1-like hashing function must be provided by the caller as `hasher`.
///   The hasher takes a slice of piece bytes and returns a Vec<u8> containing the hash bytes.
/// - The manager stores pieces' data in memory. For large torrents you may want to replace
///   that with on-disk storage.
pub struct PieceManager {
    pieces: Vec<Mutex<PieceMeta>>,
    piece_length: usize,
    total_length: usize,
    hasher: Arc<dyn Fn(&[u8]) -> Vec<u8> + Send + Sync>,
}

impl PieceManager {
    /// Create a new PieceManager.
    /// - `piece_hashes` must contain the SHA-1 (or matching) hashes for each piece in order.
    /// - `piece_length` is the standard piece length (last piece may be smaller).
    /// - `total_length` is the total size of the file(s).
    /// - `hasher` is a closure that computes the piece hash from bytes.
    pub fn new<F>(
        piece_hashes: Vec<Vec<u8>>,
        piece_length: usize,
        total_length: usize,
        hasher: F,
    ) -> Self
    where
        F: Fn(&[u8]) -> Vec<u8> + Send + Sync + 'static,
    {
        let mut pieces = Vec::with_capacity(piece_hashes.len());
        for (i, h) in piece_hashes.into_iter().enumerate() {
            let this_len = if i + 1 == piece_hashes.len() {
                // last piece: might be shorter
                let n_full = piece_length * (piece_hashes.len() - 1);
                total_length - n_full
            } else {
                piece_length
            };
            pieces.push(Mutex::new(PieceMeta::new(h, this_len)));
        }
        Self {
            pieces,
            piece_length,
            total_length,
            hasher: Arc::new(hasher),
        }
    }

    /// Returns how many pieces exist.
    pub fn pieces_count(&self) -> usize {
        self.pieces.len()
    }

    /// Returns a Bitfield of pieces we already have (complete and verified).
    pub fn have_bitfield(&self) -> Bitfield {
        let mut bf = Bitfield::new(self.pieces.len());
        for (i, pm_mutex) in self.pieces.iter().enumerate() {
            let pm = pm_mutex.lock().unwrap();
            if pm.complete {
                bf.set(i);
            }
        }
        bf
    }

    /// Given a peer bitfield, returns a list of piece indices that the peer has and we don't.
    pub fn interesting_pieces(&self, peer: &Bitfield) -> Vec<usize> {
        let mut ret = Vec::new();
        for i in 0..self.pieces_count() {
            if peer.has(i) {
                let pm = self.pieces[i].lock().unwrap();
                if !pm.complete {
                    ret.push(i);
                }
            }
        }
        ret
    }

    /// Choose the next block request to make to a peer described by `peer` bitfield.
    /// This will avoid requesting blocks already requested locally.
    /// `max_request_size` limits the returned block length (typically <= BLOCK_SIZE).
    pub fn next_request(&self, peer: &Bitfield, max_request_size: usize) -> Option<Request> {
        let max_request_size = std::cmp::min(max_request_size, BLOCK_SIZE);
        // naive strategy: first-available interesting piece, first missing block
        for i in 0..self.pieces_count() {
            if !peer.has(i) {
                continue;
            }
            let mut pm = self.pieces[i].lock().unwrap();
            if pm.complete {
                continue;
            }
            for block_idx in 0..pm.blocks_count() {
                if pm.downloaded[block_idx] || pm.requested.contains(&block_idx) {
                    continue;
                }
                let begin = block_idx * BLOCK_SIZE;
                let block_len = pm.block_len(block_idx).unwrap();
                let length = std::cmp::min(block_len, max_request_size);
                pm.requested.insert(block_idx);
                return Some(Request {
                    piece_index: i,
                    begin,
                    length,
                });
            }
        }
        None
    }

    /// Mark a block as requested (useful if you send a request through a different path).
    pub fn mark_requested(&self, piece_index: usize, begin: usize) -> Result<(), PieceManagerError> {
        if piece_index >= self.pieces_count() {
            return Err(PieceManagerError::InvalidPieceIndex);
        }
        let block_idx = begin / BLOCK_SIZE;
        let mut pm = self.pieces[piece_index].lock().unwrap();
        if block_idx >= pm.blocks_count() {
            return Err(PieceManagerError::InvalidBlockOffset);
        }
        pm.requested.insert(block_idx);
        Ok(())
    }

    /// Add a received block of data. If the block completes the piece and verification passes,
    /// returns Ok(Some(piece_bytes)). If the block is accepted but piece not yet complete, returns Ok(None).
    /// If verification fails, returns Err(VerificationFailed) and resets the partial data for that piece.
    pub fn add_block(
        &self,
        piece_index: usize,
        begin: usize,
        block: &[u8],
    ) -> Result<Option<Vec<u8>>, PieceManagerError> {
        if piece_index >= self.pieces_count() {
            return Err(PieceManagerError::InvalidPieceIndex);
        }
        let block_idx = begin / BLOCK_SIZE;
        let mut pm = self.pieces[piece_index].lock().unwrap();
        if block_idx >= pm.blocks_count() {
            return Err(PieceManagerError::InvalidBlockOffset);
        }
        let expected_len = pm.block_len(block_idx).ok_or(PieceManagerError::InvalidBlockOffset)?;
        if block.len() != expected_len {
            return Err(PieceManagerError::InvalidBlockSize);
        }

        // copy into buffer
        pm.data[begin..begin + block.len()].copy_from_slice(block);
        pm.downloaded[block_idx] = true;
        pm.requested.remove(&block_idx);

        if pm.is_fully_downloaded() {
            // verify
            let digest = (self.hasher)(&pm.data);
            if digest == pm.hash {
                pm.complete = true;
                return Ok(Some(pm.data.clone()));
            } else {
                // verification failed: reset piece so it will be retried
                pm.reset_partial();
                return Err(PieceManagerError::VerificationFailed);
            }
        }
        Ok(None)
    }

    /// Mark a piece as "have" (useful if data was loaded from disk). This overwrites partial state.
    /// If provided `data` length must match piece length, and will be verified before marking complete.
    pub fn mark_have_from_data(
        &self,
        piece_index: usize,
        data: &[u8],
    ) -> Result<(), PieceManagerError> {
        if piece_index >= self.pieces_count() {
            return Err(PieceManagerError::InvalidPieceIndex);
        }
        let mut pm = self.pieces[piece_index].lock().unwrap();
        if data.len() != pm.length {
            return Err(PieceManagerError::InvalidBlockSize);
        }
        let digest = (self.hasher)(data);
        if digest != pm.hash {
            return Err(PieceManagerError::VerificationFailed);
        }
        pm.data.copy_from_slice(data);
        for b in &mut pm.downloaded {
            *b = true;
        }
        pm.requested.clear();
        pm.complete = true;
        Ok(())
    }

    /// Check whether a piece is complete.
    pub fn piece_complete(&self, piece_index: usize) -> Result<bool, PieceManagerError> {
        if piece_index >= self.pieces_count() {
            return Err(PieceManagerError::InvalidPieceIndex);
        }
        let pm = self.pieces[piece_index].lock().unwrap();
        Ok(pm.complete)
    }
}