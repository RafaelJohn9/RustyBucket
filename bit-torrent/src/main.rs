mod torrent;


use crate::torrent::{parse_torrent, Torrent};
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
    println!("{:#?}", parsed);
}
