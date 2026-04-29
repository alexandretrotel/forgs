use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::infra::output::stdout;
use crate::models::scan_result::ScanResult;

pub fn write_results(results: &[ScanResult], output_path: Option<&Path>) -> Result<()> {
    match output_path {
        Some(path) if path.is_dir() => write_results_to_directory(results, path),
        Some(path) => write_results_to_file(results, path),
        None => {
            stdout::write_results(results);
            Ok(())
        }
    }
}

fn write_results_to_directory(results: &[ScanResult], directory: &Path) -> Result<()> {
    println!(
        "Warning: output path is a directory. Writing one file per repository into {}.",
        directory.display()
    );

    for result in results {
        let output_path = directory.join(output_filename(&result.repository));
        let json = serde_json::to_string_pretty(result).context("failed to serialize results")?;
        fs::write(&output_path, &json).context("failed to write result file")?;
        println!("Wrote {}", output_path.display());
    }

    Ok(())
}

fn write_results_to_file(results: &[ScanResult], path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(results).context("failed to serialize results")?;
    fs::write(path, &json).context("failed to write result file")?;
    println!("Wrote {}", path.display());
    Ok(())
}

fn output_filename(repository: &crate::models::repository::Repository) -> PathBuf {
    PathBuf::from(format!("{}-{}.json", repository.owner, repository.name))
}
