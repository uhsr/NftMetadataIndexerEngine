// src/main.rs
/*
 * Main executable for NftMetadataIndexerEngine
 */

use clap::Parser;
use nftmetadataindexerengine::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMetadataIndexerEngine - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
