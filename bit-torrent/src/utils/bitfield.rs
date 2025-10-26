use std::fmt;

/// Bitfield used in the BitTorrent protocol.
/// Bits in each byte are ordered most-significant-bit first:
/// - piece index 0 -> byte 0, bit 7 (0x80)
/// - piece index 1 -> byte 0, bit 6 (0x40)
/// and so on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bitfield {
    bytes: Vec<u8>,
    pieces: usize,
}

#[derive(Debug)]
pub enum BitfieldError {
    OutOfBounds(usize),
    InvalidBytesLength { expected: usize, found: usize },
}

impl fmt::Display for BitfieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitfieldError::OutOfBounds(i) => write!(f, "index {} out of bounds", i),
            BitfieldError::InvalidBytesLength { expected, found } => {
                write!(f, "invalid bytes length: expected >= {}, found {}", expected, found)
            }
        }
    }
}

impl Bitfield {
    /// Create a new empty bitfield for `pieces` pieces (all bits unset).
    pub fn new(pieces: usize) -> Self {
        let len = (pieces + 7) / 8;
        Bitfield {
            bytes: vec![0u8; len],
            pieces,
        }
    }

    /// Create a bitfield from raw bytes (wire format). `bytes` must contain at least
    /// ceil(pieces/8) bytes. Extra bytes are ignored. Unused low-order bits of the
    /// last byte (if any) are cleared.
    pub fn from_bytes(bytes: &[u8], pieces: usize) -> Result<Self, BitfieldError> {
        let required = (pieces + 7) / 8;
        if bytes.len() < required {
            return Err(BitfieldError::InvalidBytesLength {
                expected: required,
                found: bytes.len(),
            });
        }
        let mut vec = bytes[..required].to_vec();

        // Clear unused bits in the last byte (bits corresponding to non-existent pieces).
        if pieces % 8 != 0 && required > 0 {
            let used_bits = pieces % 8;
            // mask keeps the high `used_bits` bits: e.g., used_bits=3 -> 0b1110_0000
            let mask: u8 = 0xFFu8 << (8 - used_bits);
            let last = required - 1;
            vec[last] &= mask;
        }

        Ok(Bitfield { bytes: vec, pieces })
    }

    /// Return a copy of the underlying bytes suitable for wire transmission.
    /// The last byte will have unused bits cleared.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = self.bytes.clone();
        if self.pieces % 8 != 0 && !out.is_empty() {
            let used_bits = self.pieces % 8;
            let mask: u8 = 0xFFu8 << (8 - used_bits);
            let last = out.len() - 1;
            out[last] &= mask;
        }
        out
    }

    /// Number of pieces represented by this bitfield.
    pub fn len(&self) -> usize {
        self.pieces
    }

    /// Whether the bitfield represents zero pieces.
    pub fn is_empty(&self) -> bool {
        self.pieces == 0
    }

    /// Get whether `index` piece is available (true) or not (false).
    pub fn get(&self, index: usize) -> Result<bool, BitfieldError> {
        if index >= self.pieces {
            return Err(BitfieldError::OutOfBounds(index));
        }
        let byte_idx = index / 8;
        let bit = 7 - (index % 8);
        Ok((self.bytes[byte_idx] & (1 << bit)) != 0)
    }

    /// Set or clear the bit for `index`.
    pub fn set(&mut self, index: usize, have: bool) -> Result<(), BitfieldError> {
        if index >= self.pieces {
            return Err(BitfieldError::OutOfBounds(index));
        }
        let byte_idx = index / 8;
        let bit = 7 - (index % 8);
        let mask = 1u8 << bit;
        if have {
            self.bytes[byte_idx] |= mask;
        } else {
            self.bytes[byte_idx] &= !mask;
        }
        Ok(())
    }

    /// Count number of pieces marked as available.
    pub fn count_ones(&self) -> usize {
        self.bytes.iter().map(|b| b.count_ones() as usize).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_set_get() {
        let mut bf = Bitfield::new(10);
        assert_eq!(bf.len(), 10);
        for i in 0..10 {
            assert_eq!(bf.get(i).unwrap(), false);
        }

        bf.set(0, true).unwrap();
        bf.set(3, true).unwrap();
        bf.set(9, true).unwrap();

        assert_eq!(bf.get(0).unwrap(), true);
        assert_eq!(bf.get(1).unwrap(), false);
        assert_eq!(bf.get(3).unwrap(), true);
        assert_eq!(bf.get(9).unwrap(), true);
        assert_eq!(bf.count_ones(), 3);
    }

    #[test]
    fn to_from_bytes_roundtrip() {
        let mut bf = Bitfield::new(13); // requires 2 bytes, 13 bits
        bf.set(0, true).unwrap();
        bf.set(7, true).unwrap(); // last bit of first byte
        bf.set(8, true).unwrap(); // first bit of second byte
        bf.set(12, true).unwrap(); // bit index 12

        let bytes = bf.to_bytes();
        let bf2 = Bitfield::from_bytes(&bytes, 13).unwrap();
        assert_eq!(bf, bf2);

        // Ensure unused low-order bits in last byte are zero
        let last = bytes[bytes.len() - 1];
        let unused_bits = 8 - (13 % 8); // 8 - 5 = 3 unused bits
        assert_eq!(last & ((1 << unused_bits) - 1), 0);
    }

    #[test]
    fn from_bytes_too_short() {
        let res = Bitfield::from_bytes(&[0u8], 9); // needs 2 bytes
        assert!(res.is_err());
    }

    #[test]
    fn out_of_bounds() {
        let mut bf = Bitfield::new(5);
        assert!(bf.set(5, true).is_err());
        assert!(bf.get(5).is_err());
    }
}