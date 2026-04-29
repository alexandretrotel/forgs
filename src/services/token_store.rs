use anyhow::{Context, Result};
use keyring::use_native_store;
use keyring_core::{Entry, Error as KeyringError};

const SERVICE_NAME: &str = "forgs";
const GITHUB_TOKEN_ENTRY: &str = "github-token";

pub struct GithubTokenStore {
    entry: Entry,
}

impl GithubTokenStore {
    pub fn new() -> Result<Self> {
        use_native_store(false).context("failed to initialize the native keyring store")?;

        let entry = Entry::new(SERVICE_NAME, GITHUB_TOKEN_ENTRY)
            .context("failed to create keyring entry")?;

        Ok(Self { entry })
    }

    pub fn get_optional(&self) -> Result<Option<String>> {
        match self.entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(KeyringError::NoEntry) => Ok(None),
            Err(error) => {
                Err(anyhow::Error::new(error).context("failed to read GitHub token from keyring"))
            }
        }
    }

    pub fn set(&self, token: &str) -> Result<()> {
        self.entry
            .set_password(token)
            .context("failed to store GitHub token in keyring")?;

        println!("Stored GitHub token in keyring.");
        Ok(())
    }

    pub fn delete(&self) -> Result<()> {
        match self.entry.delete_credential() {
            Ok(()) => {
                println!("Deleted GitHub token from keyring.");
                Ok(())
            }
            Err(KeyringError::NoEntry) => {
                println!("No GitHub token was stored in keyring.");
                Ok(())
            }
            Err(error) => {
                Err(anyhow::Error::new(error).context("failed to delete GitHub token from keyring"))
            }
        }
    }
}
