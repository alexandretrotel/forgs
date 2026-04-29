use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "forgs",
    version,
    about = "A tool to scan famous organizations from a list of stargazers."
)]
pub struct Cli {
    pub repositories: Vec<String>,
    #[command(subcommand)]
    pub command: Option<Commands>,
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
