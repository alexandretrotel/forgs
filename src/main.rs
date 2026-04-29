use anyhow::Result;
use clap::Parser;

use forgs::cli::args::Cli;
use forgs::commands;

#[tokio::main]
async fn main() -> Result<()> {
    commands::run(Cli::parse()).await
}
