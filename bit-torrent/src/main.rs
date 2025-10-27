mod torrent;
mod tracker;
mod utils;


use crate::torrent::{parse_torrent};
use crate::tracker::udp::udp_announce_from_torrent;
use crate::utils::sha1::sha1_batch;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Parse a .torrent file and print its contents")]
struct Args {
    /// Path to the .torrent file
    torrent: PathBuf,
}

fn main() {
    let args = Args::parse();

    // Read the torrent file.
    // Parse its contents.
    // Use UDP if tracker URL starts with "udp://".
    // Get peer list from tracker.
    // Get and Store the individual pieces sha1 hashes.

    let data = match std::fs::read(&args.torrent) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to read {}: {}", args.torrent.display(), e);
            std::process::exit(1);
        }
    };

    // Call the parser and print the result to stdout.
    // This will work whether parse_torrent returns a Torrent or a Result<Torrent, E>
    // as long as the returned type implements Debug.
    let parsed = parse_torrent(&data);

    let torrent = match parsed {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to parse torrent: {:?}", e);
            std::process::exit(1);
        }
    };

    // Use UDP tracker to get peers
    let peers = match udp_announce_from_torrent(&torrent, 6881) {
        Ok(peers) => {
            peers
        }
        Err(e) => {
            eprintln!("Failed to get peers from UDP tracker: {}", e);
            std::process::exit(1);
        }
    };

    let pieces = match &torrent.info.pieces {
        Some(p) => p.as_slice(),
        None => {
            eprintln!("Torrent file has no 'pieces' data");
            std::process::exit(1);
        }
    };

    // Split the concatenated pieces blob into 20-byte chunks (SHA1 hashes)
    let piece_slices: Vec<&[u8]> = pieces.chunks(20).collect();
    let piece_sha1s = sha1_batch(&piece_slices);

    println!("Piece SHA1 hashes: {:#?}", piece_sha1s);


}
