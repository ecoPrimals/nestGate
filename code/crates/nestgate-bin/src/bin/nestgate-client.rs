//! Command-line tool for managing and monitoring the NestGate NAS node

use clap::Parser;
use nestgate_bin::{cli::Cli, error::BinResult};

#[tokio::main]
async fn main() -> BinResult<()> {
    let cli = Cli::parse();
    cli.run().await
}
