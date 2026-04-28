use anyhow::{Context, Result};
use dotenvy::from_filename;

pub struct Config {
    pub github_token: String,
    pub repo_owner: &'static str,
    pub repo_name: &'static str,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        from_filename(".env.local").ok();

        Ok(Self {
            github_token: std::env::var("GITHUB_TOKEN").context("missing GITHUB_TOKEN")?,
            repo_owner: "zap-studio",
            repo_name: "monorepo",
        })
    }
}
