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

    for repository in &config.repositories {
        println!("Processing {}/{}...", repository.owner, repository.name);

        let organizations = github
            .rank_organizations_by_followers(repository.owner, repository.name)
            .await?;

        write_organizations(repository, &organizations)?;
    }

    Ok(())
}
