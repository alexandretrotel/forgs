mod token;

use anyhow::Result;

use crate::cli::args::{Cli, Commands};

pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Token(args) => token::run(args),
    }
}
