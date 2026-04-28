mod config;
mod github;
mod organization;
mod output;

use anyhow::Result;

use crate::config::Config;
use crate::github::GitHubClient;
use crate::output::write_organizations;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;
    let github = GitHubClient::new(config.github_token)?;

    let organizations = github
        .rank_organizations_by_followers(config.repo_owner, config.repo_name)
        .await?;

    write_organizations(&organizations)?;

    Ok(())
}
