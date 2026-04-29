mod delete;
mod set;

use anyhow::{Result, anyhow};

use crate::cli::args::{SetTokenArgs, TokenAction, TokenArgs};
use crate::services::token_store::GithubTokenStore;

pub fn run(args: TokenArgs) -> Result<()> {
    let store = GithubTokenStore::new()?;

    match args.action {
        Some(TokenAction::Set(args)) => set::run(&store, args),
        Some(TokenAction::Delete) => delete::run(&store),
        None => {
            let token = args.token.ok_or_else(|| {
                anyhow!("missing token: use `forgs token <token>` or `forgs token set <token>`")
            })?;

            set::run(&store, SetTokenArgs { token })
        }
    }
}
