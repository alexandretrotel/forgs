use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "forgs",
    version,
    about = "A tool to scan famous organizations from a list of stargazers."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Scan(ScanArgs),
    TokenSet(SetTokenArgs),
    TokenDelete,
}

#[derive(Debug, Args)]
pub struct ScanArgs {
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    pub repositories: Vec<String>,
}

#[derive(Debug, Args)]
pub struct SetTokenArgs {
    pub token: String,
}
