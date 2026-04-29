use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "forgs", version, about = "GitHub Token config for forgs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Token(TokenArgs),
}

#[derive(Debug, Args)]
pub struct TokenArgs {
    #[command(subcommand)]
    pub action: Option<TokenAction>,
    pub token: Option<String>,
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
