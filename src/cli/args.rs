use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "forgs",
    version,
    about = "A tool to scan the organizations behind a repository's stargazers.",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Scan(ScanArgs),
    Token(TokenArgs),
    Version,
}

#[derive(Debug, Args)]
pub struct ScanArgs {
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    pub repositories: Vec<String>,
}

#[derive(Debug, Args)]
pub struct TokenArgs {
    #[command(subcommand)]
    pub action: TokenAction,
}

#[derive(Debug, Subcommand)]
pub enum TokenAction {
    Set(SetTokenArgs),
    Delete,
}

#[derive(Debug, Args)]
pub struct SetTokenArgs {
    pub token: String,
}
