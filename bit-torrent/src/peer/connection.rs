use std::net::SocketAddr;
use std::time::Duration;
use std::{fmt, io};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::peer::handshake::Handshake;
use crate::peer::message::Message;

/// Simple connection layer for a BitTorrent peer.
///
/// Responsibilities:
/// - open or wrap a TcpStream
/// - perform the BitTorrent handshake
/// - send and receive length-prefixed messages
///
/// This file intentionally keeps the surface small; it expects:
/// - crate::peer::handshake::Handshake provides:
///     - fn encode(&self) -> Vec<u8>
///     - fn decode(bytes: &[u8]) -> Result<Handshake, _>
///     - fields `info_hash: [u8;20]` and `peer_id: [u8;20]` (or accessible)
/// - crate::peer::message::Message provides:
///     - fn encode(&self) -> Vec<u8>
///     - fn decode(bytes: &[u8]) -> Result<Message, _>
///
/// The handshake length for BitTorrent is fixed (68 bytes) in common implementations:
/// pstrlen(1) + pstr(19) + reserved(8) + info_hash(20) + peer_id(20) = 68
const HANDSHAKE_LEN: usize = 68;
const IO_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub enum ConnectionError {
    Io(io::Error),
    Timeout,
    Handshake(String),
    Message(String),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::Io(e) => write!(f, "io error: {}", e),
            ConnectionError::Timeout => write!(f, "operation timed out"),
            ConnectionError::Handshake(s) => write!(f, "handshake error: {}", s),
            ConnectionError::Message(s) => write!(f, "message error: {}", s),
        }
    }
}

impl std::error::Error for ConnectionError {}

impl From<io::Error> for ConnectionError {
    fn from(e: io::Error) -> Self {
        ConnectionError::Io(e)
    }
}

/// Represents a P2P connection to a single peer.
pub struct PeerConnection {
    stream: TcpStream,
    pub addr: SocketAddr,
    pub remote_peer_id: Option<[u8; 20]>,
    pub info_hash: [u8; 20],
    pub local_peer_id: [u8; 20],
}

impl PeerConnection {
    /// Connect to a peer and perform handshake.
    ///
    /// - addr: peer address to connect
    /// - info_hash: torrent info_hash (20 bytes)
    /// - local_peer_id: our 20 byte peer id
    pub async fn connect(
        addr: SocketAddr,
        info_hash: [u8; 20],
        local_peer_id: [u8; 20],
    ) -> Result<Self, ConnectionError> {
        let stream = timeout(IO_TIMEOUT, TcpStream::connect(addr))
            .await
            .map_err(|_| ConnectionError::Timeout)??;

        let mut conn = PeerConnection {
            stream,
            addr,
            remote_peer_id: None,
            info_hash,
            local_peer_id,
        };

        conn.perform_handshake().await?;
        Ok(conn)
    }

    /// Wrap an accepted stream (incoming connection) and perform handshake.
    pub async fn from_stream(
        stream: TcpStream,
        addr: SocketAddr,
        info_hash: [u8; 20],
        local_peer_id: [u8; 20],
    ) -> Result<Self, ConnectionError> {
        let mut conn = PeerConnection {
            stream,
            addr,
            remote_peer_id: None,
            info_hash,
            local_peer_id,
        };

        conn.perform_handshake().await?;
        Ok(conn)
    }

    /// Perform BitTorrent handshake exchange:
    /// - send our handshake
    /// - read peer handshake and validate info_hash
    async fn perform_handshake(&mut self) -> Result<(), ConnectionError> {
        // Build handshake struct (details kept in handshake module).
        let hs = Handshake {
            info_hash: self.info_hash,
            peer_id: self.local_peer_id,
        };

        let out = hs.encode();

        // send handshake with timeout
        timeout(IO_TIMEOUT, self.stream.write_all(&out))
            .await
            .map_err(|_| ConnectionError::Timeout)??;

        // read peer handshake (fixed size)
        let mut buf = vec![0u8; HANDSHAKE_LEN];
        timeout(IO_TIMEOUT, self.stream.read_exact(&mut buf))
            .await
            .map_err(|_| ConnectionError::Timeout)??;

        // parse handshake
        let peer_hs = Handshake::decode(&buf).map_err(|e| ConnectionError::Handshake(format!("{:?}", e)))?;

        // validate info_hash
        if peer_hs.info_hash != self.info_hash {
            return Err(ConnectionError::Handshake("info_hash mismatch".into()));
        }

        self.remote_peer_id = Some(peer_hs.peer_id);
        Ok(())
    }

    /// Send a message (length-prefixed). A keep-alive is encoded as length 0.
    pub async fn send_message(&mut self, msg: &Message) -> Result<(), ConnectionError> {
        let payload = msg.encode();
        // 4 byte big-endian length prefix
        let len = payload.len() as u32;
        let mut buf = Vec::with_capacity(4 + payload.len());
        buf.extend_from_slice(&len.to_be_bytes());
        buf.extend_from_slice(&payload);

        timeout(IO_TIMEOUT, self.stream.write_all(&buf))
            .await
            .map_err(|_| ConnectionError::Timeout)??;

        Ok(())
    }

    /// Read next message. Returns Ok(None) if the peer closed the connection cleanly.
    pub async fn read_message(&mut self) -> Result<Option<Message>, ConnectionError> {
        // read length prefix
        let mut len_buf = [0u8; 4];
        match timeout(IO_TIMEOUT, self.stream.read_exact(&mut len_buf)).await {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    return Ok(None);
                } else {
                    return Err(ConnectionError::Io(e));
                }
            }
            Err(_) => return Err(ConnectionError::Timeout),
        }

        let len = u32::from_be_bytes(len_buf);
        if len == 0 {
            // keep-alive
            return Ok(Some(Message::keep_alive()));
        }

        let mut payload = vec![0u8; len as usize];
        timeout(IO_TIMEOUT, self.stream.read_exact(&mut payload))
            .await
            .map_err(|_| ConnectionError::Timeout)??;

        let msg = Message::decode(&payload).map_err(|e| ConnectionError::Message(format!("{:?}", e)))?;
        Ok(Some(msg))
    }

    /// Gracefully shut down the connection.
    pub async fn close(&mut self) -> Result<(), ConnectionError> {
        // Shutdown both directions
        self.stream.shutdown().await.map_err(|e| ConnectionError::Io(e))
    }
}