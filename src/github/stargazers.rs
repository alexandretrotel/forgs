use anyhow::Result;
use octocrab::Octocrab;

use super::progress::ProgressHandle;

const STARS_PER_PAGE: u8 = 100;

pub async fn fetch_stargazer_names(
    octocrab: &Octocrab,
    repo_owner: &str,
    repo_name: &str,
    progress: &ProgressHandle,
) -> Result<Vec<String>> {
    let mut stargazer_names = Vec::new();
    let mut page_num: u32 = 1;

    loop {
        let page = octocrab
            .repos(repo_owner, repo_name)
            .list_stargazers()
            .per_page(STARS_PER_PAGE)
            .page(page_num)
            .send()
            .await?;

        if page.items.is_empty() {
            break;
        }

        let page_len = page.items.len();

        for stargazer in page.items {
            let Some(user) = stargazer.user else {
                continue;
            };

            stargazer_names.push(user.login);
        }

        progress.set_message(format!("Fetching stargazers ({})", stargazer_names.len()));

        if page_len < STARS_PER_PAGE as usize {
            break;
        }

        page_num += 1;
    }

    Ok(stargazer_names)
}
