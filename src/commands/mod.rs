mod scan;
mod token;

use anyhow::Result;

use crate::cli::args::{Cli, Commands};

pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Scan(args) => scan::run(args.repositories, args.output).await,
        Commands::TokenSet(args) => token::set::run(args),
        Commands::TokenDelete => token::delete::run(),
    }
}
