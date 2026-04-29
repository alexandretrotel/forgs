use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::models::organization::OrganizationRank;
use crate::models::repository::Repository;

pub fn write_organizations(
    repository: &Repository,
    organizations: &[OrganizationRank],
) -> Result<()> {
    let output_path = PathBuf::from(output_filename(repository));
    let json =
        serde_json::to_string_pretty(organizations).context("failed to serialize results")?;

    fs::write(&output_path, json).context("failed to write result file")?;

    println!("Wrote {}", output_path.display());

    Ok(())
}

fn output_filename(repository: &Repository) -> String {
    format!("{}-{}.json", repository.owner, repository.name)
}
