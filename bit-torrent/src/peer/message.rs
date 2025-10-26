use std::io::{self, Read, Write, ErrorKind};
use std::io::Cursor;

/// BitTorrent peer-to-peer protocol messages (not including handshake).
/// This module provides a `Message` enum and helpers to serialize/deserialize
/// messages to/from the wire format (length prefix big-endian u32 + id + payload).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    KeepAlive,
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have(u32),
    Bitfield(Vec<u8>),
    Request { index: u32, begin: u32, length: u32 },
    Piece { index: u32, begin: u32, block: Vec<u8> },
    Cancel { index: u32, begin: u32, length: u32 },
    Port(u16),
    Extension(Vec<u8>), // extension message (id = 20), payload is raw
}

impl Message {
    // Message IDs as per the protocol
    const ID_CHOKE: u8 = 0;
    const ID_UNCHOKE: u8 = 1;
    const ID_INTERESTED: u8 = 2;
    const ID_NOT_INTERESTED: u8 = 3;
    const ID_HAVE: u8 = 4;
    const ID_BITFIELD: u8 = 5;
    const ID_REQUEST: u8 = 6;
    const ID_PIECE: u8 = 7;
    const ID_CANCEL: u8 = 8;
    const ID_PORT: u8 = 9;
    const ID_EXTENSION: u8 = 20;

    /// Serialize message into wire bytes (length prefix + id + payload).
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Message::KeepAlive => 0u32.to_be_bytes().to_vec(),
            Message::Choke => message_with_id(Self::ID_CHOKE, &[]),
            Message::Unchoke => message_with_id(Self::ID_UNCHOKE, &[]),
            Message::Interested => message_with_id(Self::ID_INTERESTED, &[]),
            Message::NotInterested => message_with_id(Self::ID_NOT_INTERESTED, &[]),
            Message::Have(index) => {
                let mut payload = index.to_be_bytes().to_vec();
                message_with_id(Self::ID_HAVE, &payload)
            }
            Message::Bitfield(bits) => {
                message_with_id(Self::ID_BITFIELD, bits)
            }
            Message::Request { index, begin, length } => {
                let mut payload = Vec::with_capacity(12);
                payload.extend_from_slice(&index.to_be_bytes());
                payload.extend_from_slice(&begin.to_be_bytes());
                payload.extend_from_slice(&length.to_be_bytes());
                message_with_id(Self::ID_REQUEST, &payload)
            }
            Message::Piece { index, begin, block } => {
                let mut payload = Vec::with_capacity(8 + block.len());
                payload.extend_from_slice(&index.to_be_bytes());
                payload.extend_from_slice(&begin.to_be_bytes());
                payload.extend_from_slice(block);
                message_with_id(Self::ID_PIECE, &payload)
            }
            Message::Cancel { index, begin, length } => {
                let mut payload = Vec::with_capacity(12);
                payload.extend_from_slice(&index.to_be_bytes());
                payload.extend_from_slice(&begin.to_be_bytes());
                payload.extend_from_slice(&length.to_be_bytes());
                message_with_id(Self::ID_CANCEL, &payload)
            }
            Message::Port(port) => {
                message_with_id(Self::ID_PORT, &port.to_be_bytes())
            }
            Message::Extension(payload) => {
                message_with_id(Self::ID_EXTENSION, payload)
            }
        }
    }

    /// Read a single message from a reader. Blocks until full message is read or returns an error.
    pub fn from_reader<R: Read>(r: &mut R) -> io::Result<Message> {
        let mut len_buf = [0u8; 4];
        r.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf);

        if len == 0 {
            return Ok(Message::KeepAlive);
        }

        if len < 1 {
            return Err(io::Error::new(ErrorKind::InvalidData, "invalid message length"));
        }

        // read id
        let mut id_buf = [0u8; 1];
        r.read_exact(&mut id_buf)?;
        let id = id_buf[0];
        let payload_len = (len - 1) as usize;
        let mut payload = vec![0u8; payload_len];
        if payload_len > 0 {
            r.read_exact(&mut payload)?;
        }

        match id {
            Self::ID_CHOKE => Ok(Message::Choke),
            Self::ID_UNCHOKE => Ok(Message::Unchoke),
            Self::ID_INTERESTED => Ok(Message::Interested),
            Self::ID_NOT_INTERESTED => Ok(Message::NotInterested),
            Self::ID_HAVE => {
                if payload.len() != 4 {
                    return Err(io::Error::new(ErrorKind::InvalidData, "have message payload size"));
                }
                let idx = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
                Ok(Message::Have(idx))
            }
            Self::ID_BITFIELD => Ok(Message::Bitfield(payload)),
            Self::ID_REQUEST => {
                if payload.len() != 12 {
                    return Err(io::Error::new(ErrorKind::InvalidData, "request payload size"));
                }
                let index = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
                let begin = u32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]);
                let length = u32::from_be_bytes([payload[8], payload[9], payload[10], payload[11]]);
                Ok(Message::Request { index, begin, length })
            }
            Self::ID_PIECE => {
                if payload.len() < 8 {
                    return Err(io::Error::new(ErrorKind::InvalidData, "piece payload too small"));
                }
                let index = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
                let begin = u32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]);
                let block = payload[8..].to_vec();
                Ok(Message::Piece { index, begin, block })
            }
            Self::ID_CANCEL => {
                if payload.len() != 12 {
                    return Err(io::Error::new(ErrorKind::InvalidData, "cancel payload size"));
                }
                let index = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
                let begin = u32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]);
                let length = u32::from_be_bytes([payload[8], payload[9], payload[10], payload[11]]);
                Ok(Message::Cancel { index, begin, length })
            }
            Self::ID_PORT => {
                if payload.len() != 2 {
                    return Err(io::Error::new(ErrorKind::InvalidData, "port payload size"));
                }
                let port = u16::from_be_bytes([payload[0], payload[1]]);
                Ok(Message::Port(port))
            }
            Self::ID_EXTENSION => Ok(Message::Extension(payload)),
            other => Err(io::Error::new(ErrorKind::InvalidData, format!("unknown message id {}", other))),
        }
    }

    /// Write serialized message to a writer.
    pub fn write_to<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let buf = self.to_bytes();
        w.write_all(&buf)
    }
}

fn message_with_id(id: u8, payload: &[u8]) -> Vec<u8> {
    let len = 1u32 + (payload.len() as u32);
    let mut v = Vec::with_capacity(4 + 1 + payload.len());
    v.extend_from_slice(&len.to_be_bytes());
    v.push(id);
    v.extend_from_slice(payload);
    v
}