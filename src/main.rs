// src/main.rs
/*
 * Main executable for ErcNftMetadataAnalyzer
 */

use clap::Parser;
use ercnftmetadataanalyzer::{Result, run};

#[derive(Parser)]
#[command(version, about = "ErcNftMetadataAnalyzer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
