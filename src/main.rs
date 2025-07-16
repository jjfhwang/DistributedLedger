// src/main.rs
/*
 * Main executable for DistributedLedger
 */

use clap::Parser;
use distributedledger::{Result, run};

#[derive(Parser)]
#[command(version, about = "DistributedLedger - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
