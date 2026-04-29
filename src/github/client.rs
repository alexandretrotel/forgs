use anyhow::Result;
use octocrab::Octocrab;

use crate::models::organization::OrganizationRank;

use super::orgs::{fetch_organization_rank, fetch_unique_organization_names};
use super::progress::{progress_bar, spinner};
use super::stargazers::fetch_stargazer_names;

pub struct GitHubClient {
    octocrab: Octocrab,
}

impl GitHubClient {
    pub fn new(token: Option<String>) -> Result<Self> {
        let mut builder = Octocrab::builder();

        if let Some(token) = token {
            builder = builder.personal_token(token);
        }

        let octocrab = builder.build()?;
        Ok(Self { octocrab })
    }

    pub async fn rank_organizations_by_followers(
        &self,
        repo_owner: &str,
        repo_name: &str,
    ) -> Result<Vec<OrganizationRank>> {
        let repo_label = format!("{repo_owner}/{repo_name}");

        let stargazers_progress = spinner(&format!("{repo_label}: Fetching stargazers"));
        let stargazer_names =
            fetch_stargazer_names(&self.octocrab, repo_owner, repo_name, &stargazers_progress)
                .await?;

        drop(stargazers_progress);

        let memberships_progress = progress_bar(
            stargazer_names.len(),
            &format!("{repo_label}: Fetching org memberships"),
        );
        let organization_names = fetch_unique_organization_names(
            &self.octocrab,
            &stargazer_names,
            &memberships_progress,
        )
        .await?;

        let total_organizations = organization_names.len();
        let mut organizations = Vec::with_capacity(total_organizations);

        drop(memberships_progress);

        let followers_progress = progress_bar(
            total_organizations,
            &format!("{repo_label}: Fetching organization followers"),
        );
        for (index, organization_name) in organization_names.into_iter().enumerate() {
            organizations.push(fetch_organization_rank(&self.octocrab, &organization_name).await?);
            followers_progress.set_position((index + 1) as u64);
        }

        organizations.sort_by(|left, right| {
            right
                .followers
                .cmp(&left.followers)
                .then_with(|| left.name.cmp(&right.name))
        });

        Ok(organizations)
    }
}
