use anyhow::Result;

use crate::services::token_store::GithubTokenStore;

pub fn run(store: &GithubTokenStore) -> Result<()> {
    store.delete()
}
