use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Serialize;

pub fn write_results<T>(value: &T, output_path: Option<&Path>) -> Result<()>
where
    T: Serialize,
{
    let json = serde_json::to_string_pretty(value).context("failed to serialize results")?;

    if let Some(path) = output_path {
        fs::write(path, &json).context("failed to write result file")?;
        println!("Wrote {}", path.display());
    } else {
        println!("{json}");
    }

    Ok(())
}
