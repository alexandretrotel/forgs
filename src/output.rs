use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config::Repository;
use crate::organization::OrganizationRank;

pub fn write_organizations(
    repository: &Repository,
    organizations: &[OrganizationRank],
) -> Result<()> {
    let output_dir = Path::new("output");
    let output_path = output_dir.join(output_filename(repository));

    fs::create_dir_all(output_dir).context("failed to create output directory")?;
    let json =
        serde_json::to_string_pretty(organizations).context("failed to serialize results")?;
    fs::write(&output_path, json).context("failed to write result file")?;

    println!("Wrote {}", output_path.display());

    Ok(())
}

fn output_filename(repository: &Repository) -> String {
    format!("{}-{}.json", repository.owner, repository.name)
}
