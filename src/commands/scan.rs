use anyhow::{Context, Result, anyhow};

use crate::github::GitHubClient;
use crate::infra::output::writer::write_organizations;
use crate::models::repository::Repository;
use crate::services::token_store::GithubTokenStore;

pub async fn run(repositories: Vec<String>) -> Result<()> {
    if repositories.is_empty() {
        return Err(anyhow!(
            "missing repositories: pass one or more values like `forgs owner/name`"
        ));
    }

    let token = GithubTokenStore::new()?.get()?;
    let github = GitHubClient::new(token)?;

    for repository in repositories
        .into_iter()
        .map(|value| parse_repository(&value))
        .collect::<Result<Vec<_>>>()?
    {
        println!("Processing {}/{}...", repository.owner, repository.name);

        let organizations = github
            .rank_organizations_by_followers(&repository.owner, &repository.name)
            .await?;

        write_organizations(&repository, &organizations)?;
    }

    Ok(())
}

fn parse_repository(value: &str) -> Result<Repository> {
    let (owner, name) = value
        .split_once('/')
        .context("repository must be formatted as owner/name")?;

    if owner.is_empty() || name.is_empty() {
        return Err(anyhow!(
            "repository `{value}` must be formatted as owner/name"
        ));
    }

    Ok(Repository {
        owner: owner.to_string(),
        name: name.to_string(),
    })
}
