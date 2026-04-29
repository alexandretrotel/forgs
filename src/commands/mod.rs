mod scan;
mod token;

use anyhow::Result;

use crate::cli::args::{Cli, Commands, TokenAction};

pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Scan(args) => scan::run(args.repositories, args.output).await,
        Commands::Token(args) => match args.action {
            TokenAction::Set(args) => token::set::run(args),
            TokenAction::Delete => token::delete::run(),
        },
        Commands::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}
