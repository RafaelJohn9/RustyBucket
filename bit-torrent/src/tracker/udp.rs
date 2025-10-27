use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashSet;

use crate::torrent::Torrent;
use crate::utils::sha1;

const PROTOCOL_ID: u64 = 0x41727101980u64;
const CONNECT_ACTION: u32 = 0;
const ANNOUNCE_ACTION: u32 = 1;
const RECV_BUF: usize = 1500;
const READ_TIMEOUT_SECS: u64 = 5;

fn parse_udp_addr(url: &str) -> Result<SocketAddr, Box<dyn Error>> {
    if !url.starts_with("udp://") {
        return Err(format!("expected udp tracker url, got {}", url).into());
    }
    let no_scheme = url.trim_start_matches("udp://");
    let host_port = no_scheme.splitn(2, '/').next().ok_or("malformed udp url")?;
    let mut addrs = host_port.to_socket_addrs()?;
    addrs.next().ok_or_else(|| "could not resolve udp address".into())
}

static TX_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn gen_tx_id() -> u32 {
    let c = TX_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let pid = std::process::id();
    // mix time, pid and a counter for higher uniqueness
    time ^ (pid as u32).rotate_left(16) ^ c.wrapping_mul(0x9E3779B1)
}

fn send_connect(socket: &UdpSocket, tx_id: u32) -> Result<(), Box<dyn Error>> {
    let mut conn_req = [0u8; 16];
    conn_req[..8].copy_from_slice(&PROTOCOL_ID.to_be_bytes());
    conn_req[8..12].copy_from_slice(&CONNECT_ACTION.to_be_bytes());
    conn_req[12..16].copy_from_slice(&tx_id.to_be_bytes());
    socket.send(&conn_req)?;
    Ok(())
}

fn recv_connect(socket: &UdpSocket, expected_tx: u32) -> Result<u64, Box<dyn Error>> {
    let mut buf = [0u8; RECV_BUF];
    let n = socket.recv(&mut buf)?;
    if n < 16 {
        return Err(format!("short connection response: {} bytes", n).into());
    }
    let action = u32::from_be_bytes(buf[0..4].try_into().unwrap());
    let resp_tx = u32::from_be_bytes(buf[4..8].try_into().unwrap());
    if action != CONNECT_ACTION || resp_tx != expected_tx {
        return Err("invalid connection response".into());
    }
    Ok(u64::from_be_bytes(buf[8..16].try_into().unwrap()))
}

fn build_announce_request(
    connection_id: u64,
    tx_id: u32,
    info_hash: &[u8; 20],
    peer_id: &[u8; 20],
    downloaded: u64,
    left: u64,
    uploaded: u64,
    port: u16,
) -> Vec<u8> {
    let mut v = Vec::with_capacity(98);
    v.extend_from_slice(&connection_id.to_be_bytes());
    v.extend_from_slice(&ANNOUNCE_ACTION.to_be_bytes());
    v.extend_from_slice(&tx_id.to_be_bytes());
    v.extend_from_slice(info_hash);
    v.extend_from_slice(peer_id);
    v.extend_from_slice(&downloaded.to_be_bytes());
    v.extend_from_slice(&left.to_be_bytes());
    v.extend_from_slice(&uploaded.to_be_bytes());
    v.extend_from_slice(&0u32.to_be_bytes()); // event
    v.extend_from_slice(&0u32.to_be_bytes()); // ip
    let key = (tx_id ^ 0xA5A5A5A5) as u32;
    v.extend_from_slice(&key.to_be_bytes());
    v.extend_from_slice(&(-1i32 as u32).to_be_bytes()); // num_want
    v.extend_from_slice(&port.to_be_bytes());
    v
}

fn recv_announce(socket: &UdpSocket, expected_tx: u32) -> Result<Vec<std::net::SocketAddr>, Box<dyn Error>> {
    let mut buf = [0u8; RECV_BUF];
    let n = socket.recv(&mut buf)?;
    if n < 20 {
        return Err(format!("short announce response: {} bytes", n).into());
    }
    let action = u32::from_be_bytes(buf[0..4].try_into().unwrap());
    let resp_tx = u32::from_be_bytes(buf[4..8].try_into().unwrap());
    if action != ANNOUNCE_ACTION || resp_tx != expected_tx {
        return Err("invalid announce response".into());
    }
    if n == 20 {
        return Ok(Vec::new());
    }
    if (n - 20) % 6 != 0 {
        return Err("announce response peers section not compact IPv4 (length not multiple of 6)".into());
    }
    let mut peers = Vec::new();
    let mut offset = 20usize;
    while offset + 6 <= n {
        let ip = std::net::Ipv4Addr::new(buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]);
        let port = u16::from_be_bytes([buf[offset + 4], buf[offset + 5]]);
        peers.push(std::net::SocketAddr::from((ip, port)));
        offset += 6;
    }
    Ok(peers)
}

pub fn udp_trackers_announce(
    url: &str,
    info_hash: &[u8; 20],
    peer_id: &[u8; 20],
    port: u16,
    left: u64,
    downloaded: u64,
    uploaded: u64,
) -> Result<Vec<std::net::SocketAddr>, Box<dyn Error>> {
    let addr = parse_udp_addr(url)?;

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(READ_TIMEOUT_SECS)))?;
    socket.connect(addr)?;

    let tx1 = gen_tx_id();
    send_connect(&socket, tx1)?;
    let connection_id = recv_connect(&socket, tx1)?;

    let tx2 = gen_tx_id().wrapping_add(1);
    let announce_req = build_announce_request(connection_id, tx2, info_hash, peer_id, downloaded, left, uploaded, port);
    socket.send(&announce_req)?;
    recv_announce(&socket, tx2)
}

pub fn udp_announce_from_torrent(
    torrent: &Torrent,
    port: u16,
) -> Result<Vec<std::net::SocketAddr>, Box<dyn Error>> {

    // compute info_hash = SHA1(info.raw) using crate::utils::sha1
    let info_raw = &torrent.info.raw;
    let digest = sha1(info_raw);
    let mut info_hash = [0u8; 20];
    info_hash.copy_from_slice(&digest[..20]);

    // generate a 20-byte peer id: prefix + time/pid/counter mix
    let mut peer_id = [0u8; 20];
    let prefix = b"-RB0001-"; // client id
    let mut off = 0usize;
    peer_id[..prefix.len()].copy_from_slice(prefix);
    off += prefix.len();

    let time_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u128)
        .unwrap_or(0);
    let pid = std::process::id() as u128;
    let counter = TX_COUNTER.load(std::sync::atomic::Ordering::Relaxed) as u128;

    let mut mix = time_nanos ^ (pid << 16) ^ (counter.wrapping_mul(0x9E3779B1u128));
    while off < 20 {
        peer_id[off] = (mix & 0xFF) as u8;
        mix >>= 8;
        off += 1;
    }

    // prepare announce candidate URLs (announce + announce_list)
    let mut urls = Vec::new();
    if let Some(ref a) = torrent.announce {
        urls.push(a.clone());
    }
    for a in &torrent.announce_list {
        urls.push(a.clone());
    }

    if urls.is_empty() {
        return Err("no trackers in torrent metadata".into());
    }

    // announce parameters
    let left = torrent.info.length.unwrap_or(0) as u64;
    let downloaded = 0u64;
    let uploaded = 0u64;

    let mut seen = HashSet::new();
    let mut peers = Vec::new();
    let mut saw_udp = false;

    for url in urls {
        if !url.starts_with("udp://") {
            continue;
        }
        saw_udp = true;
        match udp_trackers_announce(&url, &info_hash, &peer_id, port, left, downloaded, uploaded) {
            Ok(found) => {
                for p in found {
                    if seen.insert(p) {
                        peers.push(p);
                    }
                }
            }
            Err(_) => {
                // ignore individual tracker errors and try others
            }
        }
    }

    if !saw_udp {
        return Err("no udp trackers found in announce/announce_list".into());
    }

    Ok(peers)
}
