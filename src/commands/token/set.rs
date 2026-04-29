use anyhow::Result;

use crate::cli::args::SetTokenArgs;
use crate::services::token_store::GithubTokenStore;

pub fn run(args: SetTokenArgs) -> Result<()> {
    let store = GithubTokenStore::new()?;
    store.set(&args.token)
}
