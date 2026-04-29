use anyhow::Result;
use octocrab::Octocrab;

use crate::models::organization::OrganizationRank;

use super::orgs::{fetch_organization_rank, fetch_unique_organization_names};
use super::progress::print_progress;
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
        println!("Fetching stargazers...");
        let stargazer_names = fetch_stargazer_names(&self.octocrab, repo_owner, repo_name).await?;
        let organization_names =
            fetch_unique_organization_names(&self.octocrab, &stargazer_names).await?;
        let total_organizations = organization_names.len();
        let mut organizations = Vec::with_capacity(total_organizations);

        println!("Fetching organization followers...");
        for (index, organization_name) in organization_names.into_iter().enumerate() {
            organizations.push(fetch_organization_rank(&self.octocrab, &organization_name).await?);
            print_progress("Organization followers", index + 1, total_organizations);
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
