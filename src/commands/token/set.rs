use anyhow::Result;

use crate::cli::args::SetTokenArgs;
use crate::services::token_store::GithubTokenStore;

pub fn run(store: &GithubTokenStore, args: SetTokenArgs) -> Result<()> {
    store.set(&args.token)
}
