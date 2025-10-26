use std::io::{self, Read, Write};
use super::{Handshake, HANDSHAKE_LEN};
use std::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::io::duplex;
use super::async_impl::{perform_handshake_async, read_from_async, write_to_async};

//! Generic BitTorrent handshake utilities.
//!
//! This module provides a small, well-typed representation of the
//! BitTorrent handshake plus sync and async helper functions that operate
//! on generic Read/Write and AsyncRead/AsyncWrite streams so it can be
//! integrated easily with other modules.


/// The canonical protocol string for BitTorrent handshakes.
pub const BT_PROTOCOL_STR: &str = "BitTorrent protocol";

/// Length of the protocol string (usually 19).
pub const BT_PSTR_LEN: u8 = BT_PROTOCOL_STR.len() as u8;

/// Total byte length of a wire handshake:
/// 1 (pstrlen) + pstrlen + 8 (reserved) + 20 (info_hash) + 20 (peer_id)
pub const HANDSHAKE_LEN: usize = 1 + (BT_PSTR_LEN as usize) + 8 + 20 + 20;

/// Representation of a BitTorrent handshake.
///
/// Fields:
/// - pstr (protocol string) — normally "BitTorrent protocol"
/// - reserved — 8 reserved bytes (RFC 959 style; client extensions set bits here)
/// - info_hash — 20-byte SHA1 hash identifying the torrent
/// - peer_id — 20-byte peer id (peer identity)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handshake {
    pub pstr: String,
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl Handshake {
    /// Create a standard handshake with default reserved bytes (all zero).
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            pstr: BT_PROTOCOL_STR.to_string(),
            reserved: [0u8; 8],
            info_hash,
            peer_id,
        }
    }

    /// Serialize handshake to bytes (wire format).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(HANDSHAKE_LEN);
        buf.push(self.pstr.len() as u8);
        buf.extend_from_slice(self.pstr.as_bytes());
        buf.extend_from_slice(&self.reserved);
        buf.extend_from_slice(&self.info_hash);
        buf.extend_from_slice(&self.peer_id);
        buf
    }

    /// Parse a handshake from a byte slice.
    ///
    /// Returns io::Error if the slice is malformed or too short.
    pub fn from_bytes(buf: &[u8]) -> io::Result<Self> {
        if buf.is_empty() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "empty buffer"));
        }
        let pstrlen = buf[0] as usize;
        let expected_len = 1 + pstrlen + 8 + 20 + 20;
        if buf.len() < expected_len {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("buffer too short: need {} bytes", expected_len),
            ));
        }
        let mut offset = 1usize;
        let pstr = String::from_utf8(buf[offset..offset + pstrlen].to_vec())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid pstr utf8"))?;
        offset += pstrlen;
        let mut reserved = [0u8; 8];
        reserved.copy_from_slice(&buf[offset..offset + 8]);
        offset += 8;
        let mut info_hash = [0u8; 20];
        info_hash.copy_from_slice(&buf[offset..offset + 20]);
        offset += 20;
        let mut peer_id = [0u8; 20];
        peer_id.copy_from_slice(&buf[offset..offset + 20]);
        Ok(Self {
            pstr,
            reserved,
            info_hash,
            peer_id,
        })
    }

    /// Read a handshake synchronously from a Read stream.
    pub fn read_from<R: Read>(r: &mut R) -> io::Result<Self> {
        // Read the first byte to know pstrlen.
        let mut pstrlen_b = [0u8; 1];
        r.read_exact(&mut pstrlen_b)?;
        let pstrlen = pstrlen_b[0] as usize;

        // allocate buffer for rest
        let mut rest = vec![0u8; pstrlen + 8 + 20 + 20];
        r.read_exact(&mut rest)?;
        let mut buf = Vec::with_capacity(1 + rest.len());
        buf.push(pstrlen_b[0]);
        buf.extend(rest);
        Self::from_bytes(&buf)
    }

    /// Write a handshake synchronously to a Write stream.
    pub fn write_to<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let b = self.to_bytes();
        w.write_all(&b)
    }
}

#[cfg(feature = "async")]
mod async_impl {
    //! Async helpers when the "async" feature is enabled.
    //!
    //! This module uses tokio's AsyncReadExt/AsyncWriteExt. It is behind a
    //! feature flag to avoid forcing async runtime dependencies when not needed.


    /// Read a handshake from an async stream (Tokio-compatible).
    pub async fn read_from_async<S: AsyncRead + Unpin + Send>(s: &mut S) -> io::Result<Handshake> {
        // read first byte
        let mut pstrlen_b = [0u8; 1];
        s.read_exact(&mut pstrlen_b).await?;
        let pstrlen = pstrlen_b[0] as usize;
        let mut rest = vec![0u8; pstrlen + 8 + 20 + 20];
        s.read_exact(&mut rest).await?;
        let mut buf = Vec::with_capacity(1 + rest.len());
        buf.push(pstrlen_b[0]);
        buf.extend(rest);
        Handshake::from_bytes(&buf)
    }

    /// Write a handshake to an async stream (Tokio-compatible).
    pub async fn write_to_async<S: AsyncWrite + Unpin + Send>(
        s: &mut S,
        hs: &Handshake,
    ) -> io::Result<()> {
        let b = hs.to_bytes();
        s.write_all(&b).await
    }

    /// Perform the usual handshake: write our handshake, then read their handshake.
    ///
    /// If expected_info_hash is Some, the peer's info_hash will be validated and an
    /// error returned if it does not match.
    pub async fn perform_handshake_async<S: AsyncRead + AsyncWrite + Unpin + Send>(
        s: &mut S,
        local: &Handshake,
        expected_info_hash: Option<[u8; 20]>,
    ) -> io::Result<Handshake> {
        write_to_async(s, local).await?;
        let remote = read_from_async(s).await?;
        if let Some(expected) = expected_info_hash {
            if remote.info_hash != expected {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "info_hash mismatch",
                ));
            }
        }
        Ok(remote)
    }
}
