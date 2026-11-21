use std::{env, error::Error, str::FromStr, time::Duration};

use anyhow::{Context, Result, ensure};

pub(crate) struct Config {
    pub(crate) repos: Vec<String>,
    pub(crate) github_token: Option<String>,
    pub(crate) listing: ListingConfig,
    pub(crate) refresh_period: Duration,
    pub(crate) port: u16,
}

pub(crate) struct ListingConfig {
    pub(crate) name: String,
    pub(crate) author: String,
    pub(crate) id: String,
    pub(crate) public_url: String,
}

impl Config {
    pub(crate) fn from_env() -> Result<Self> {
        let repos = env::var("REPOS")
            .context("REPOS must list the GitHub repositories to publish")?
            .split(',')
            .map(str::trim)
            .filter(|link| !link.is_empty())
            .map(repo_slug)
            .collect::<Result<_>>()?;

        Ok(Self {
            repos,
            github_token: env::var("GITHUB_TOKEN")
                .ok()
                .filter(|token| !token.is_empty()),
            listing: ListingConfig {
                name: var("LISTING_NAME", "Zaphkiel VCC"),
                author: var("LISTING_AUTHOR", "Zaphkiel"),
                id: var("LISTING_ID", "dev.zaphkiel.vcc"),
                public_url: var("PUBLIC_URL", "https://vcc.zaphkiel.dev")
                    .trim_end_matches('/')
                    .to_owned(),
            },
            refresh_period: Duration::from_secs(parsed("REFRESH_SECONDS", "300")?),
            port: parsed("PORT", "8080")?,
        })
    }
}

fn var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_owned())
}

fn parsed<T>(key: &str, default: &str) -> Result<T>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    var(key, default)
        .parse()
        .with_context(|| format!("{key} is not a valid number"))
}

fn repo_slug(link: &str) -> Result<String> {
    let slug = link
        .trim_start_matches("https://github.com/")
        .trim_end_matches('/')
        .trim_end_matches(".git");
    ensure!(
        slug.split('/').filter(|part| !part.is_empty()).count() == 2,
        "REPOS entry `{link}` is not a GitHub repository link or `owner/repo`"
    );
    Ok(slug.to_owned())
}
