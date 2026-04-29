use anyhow::Result;

use crate::services::token_store::GithubTokenStore;

pub fn run() -> Result<()> {
    let store = GithubTokenStore::new()?;
    store.delete()
}
