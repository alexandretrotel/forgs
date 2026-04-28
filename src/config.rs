use anyhow::{Context, Result};
use dotenvy::from_filename;

pub struct Config {
    pub github_token: String,
    pub repositories: Vec<Repository>,
}

pub struct Repository {
    pub owner: &'static str,
    pub name: &'static str,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        from_filename(".env.local").ok();

        Ok(Self {
            github_token: std::env::var("GITHUB_TOKEN").context("missing GITHUB_TOKEN")?,
            repositories: vec![
                Repository {
                    owner: "zap-studio",
                    name: "monorepo",
                },
                Repository {
                    owner: "atrtde",
                    name: "todo-tree",
                },
                Repository {
                    owner: "atrtde",
                    name: "mntn",
                },
                Repository {
                    owner: "atrtde",
                    name: "feedyourai",
                },
            ],
        })
    }
}
