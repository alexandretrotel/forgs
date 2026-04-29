use std::collections::HashSet;

use anyhow::Result;
use octocrab::{Octocrab, Page, models};

use crate::models::organization::OrganizationRank;

use super::progress::print_progress;

const ORGS_PER_PAGE: u8 = 100;

pub async fn fetch_unique_organization_names(
    octocrab: &Octocrab,
    stargazer_names: &[String],
) -> Result<Vec<String>> {
    let mut organization_names = HashSet::new();
    let total_stargazers = stargazer_names.len();

    println!("Fetching org memberships...");
    for (index, stargazer_name) in stargazer_names.iter().enumerate() {
        organization_names.extend(fetch_user_organizations(octocrab, stargazer_name).await?);
        print_progress("Org memberships", index + 1, total_stargazers);
    }

    let mut organization_names: Vec<_> = organization_names.into_iter().collect();
    organization_names.sort();
    Ok(organization_names)
}

pub async fn fetch_organization_rank(
    octocrab: &Octocrab,
    organization_name: &str,
) -> Result<OrganizationRank> {
    let route = format!("/orgs/{organization_name}");
    let organization: models::orgs::Organization = octocrab.get(route, None::<&()>).await?;

    Ok(OrganizationRank {
        name: organization.login,
        followers: organization.followers.unwrap_or(0),
        organization_url: organization
            .html_url
            .map(|url| url.to_string())
            .unwrap_or_else(|| organization.url.to_string()),
    })
}

async fn fetch_user_organizations(octocrab: &Octocrab, username: &str) -> Result<Vec<String>> {
    let route = format!("/users/{username}/orgs");
    let mut organizations = Vec::new();
    let mut page_num: u32 = 1;

    loop {
        let query = vec![
            ("per_page", ORGS_PER_PAGE.to_string()),
            ("page", page_num.to_string()),
        ];

        let page: Page<models::orgs::Organization> =
            octocrab.get(route.as_str(), Some(&query)).await?;

        if page.items.is_empty() {
            break;
        }

        let page_len = page.items.len();

        organizations.extend(page.items.into_iter().map(|org| org.login));

        if page_len < ORGS_PER_PAGE as usize {
            break;
        }

        page_num += 1;
    }

    Ok(organizations)
}
