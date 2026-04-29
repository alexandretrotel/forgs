use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use serde::Serialize;

use crate::github::GitHubClient;
use crate::github::progress::spinner;
use crate::infra::output::writer::write_results;
use crate::models::repository::Repository;
use crate::services::token_store::GithubTokenStore;

#[derive(Serialize)]
struct ScanResult {
    repository: Repository,
    organizations: Vec<crate::models::organization::OrganizationRank>,
}

pub async fn run(repositories: Vec<String>, output: Option<PathBuf>) -> Result<()> {
    if repositories.is_empty() {
        return Err(anyhow!(
            "missing repositories: pass one or more values like `forgs owner/name`"
        ));
    }

    let token_store = GithubTokenStore::new()?;
    let token = token_store.get_optional()?;
    let scan_progress = spinner("Preparing scan");

    if token.is_none() {
        scan_progress.set_message(
            "No GitHub token configured. Continuing unauthenticated; rate limits may cause errors. `forgs token set <token>` is recommended.".to_string(),
        );
    }

    let github = GitHubClient::new(token)?;
    let mut results = Vec::new();

    for repository in repositories
        .into_iter()
        .map(|value| parse_repository(&value))
        .collect::<Result<Vec<_>>>()?
    {
        scan_progress.set_message(format!(
            "Processing {}/{}...",
            repository.owner, repository.name
        ));

        let organizations = github
            .rank_organizations_by_followers(&repository.owner, &repository.name)
            .await?;

        results.push(ScanResult {
            repository,
            organizations,
        });
    }

    scan_progress.finish_and_clear();
    write_results(&results, output.as_deref())?;

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
