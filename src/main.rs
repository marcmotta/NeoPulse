// src/main.rs
/*
 * Main executable for NeoPulse
 */

use clap::Parser;
use neopulse::{Result, run};

#[derive(Parser)]
#[command(version, about = "NeoPulse - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
