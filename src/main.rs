// src/main.rs
/*
 * Main executable for ChaincodeAuditor
 */

use clap::Parser;
use chaincodeauditor::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChaincodeAuditor - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
