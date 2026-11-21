use std::time::Duration;

use anyhow::Result;
use futures_util::future::try_join_all;
use reqwest::{
    Client, RequestBuilder, StatusCode,
    header::{ACCEPT, ETAG, HeaderValue, IF_NONE_MATCH},
};
use tokio::sync::Semaphore;

use super::{
    manifest::{self, Manifest},
    release::Release,
    repo::Repo,
};

const USER_AGENT: &str = "vcc-api";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const RELEASES_PER_PAGE: usize = 100;
const MAX_CONCURRENT_DOWNLOADS: usize = 16;

struct Releases {
    items: Vec<Release>,
    etag: Option<HeaderValue>,
}

pub(crate) struct GitHub {
    http: Client,
    token: Option<String>,
    downloads: Semaphore,
}

impl GitHub {
    pub(crate) fn new(token: Option<String>) -> Result<Self> {
        let http = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(REQUEST_TIMEOUT)
            .build()?;
        Ok(Self {
            http,
            token,
            downloads: Semaphore::new(MAX_CONCURRENT_DOWNLOADS),
        })
    }

    pub(crate) async fn refresh(&self, repo: &mut Repo) -> Result<bool> {
        let Some(releases) = self.releases(repo).await? else {
            return Ok(false);
        };
        let published = releases.items.iter().filter(|release| !release.draft);
        let manifests = try_join_all(published.map(|release| self.manifest(release))).await?;
        repo.manifests = manifests.into_iter().flatten().collect();
        repo.etag = releases.etag;
        Ok(true)
    }

    async fn releases(&self, repo: &Repo) -> Result<Option<Releases>> {
        let mut items = Vec::new();
        let mut etag = None;
        for page in 1_u32.. {
            let mut request = self.api_request(&repo.slug, page);
            if page == 1
                && let Some(known) = &repo.etag
            {
                request = request.header(IF_NONE_MATCH, known);
            }
            let response = request.send().await?.error_for_status()?;
            if response.status() == StatusCode::NOT_MODIFIED {
                return Ok(None);
            }
            if page == 1 {
                etag = response.headers().get(ETAG).cloned();
            }
            let batch: Vec<Release> = response.json().await?;
            let is_last_page = batch.len() < RELEASES_PER_PAGE;
            items.extend(batch);
            if is_last_page {
                break;
            }
        }
        Ok(Some(Releases { items, etag }))
    }

    fn api_request(&self, slug: &str, page: u32) -> RequestBuilder {
        let url = format!(
            "https://api.github.com/repos/{slug}/releases?per_page={RELEASES_PER_PAGE}&page={page}"
        );
        let request = self
            .http
            .get(url)
            .header(ACCEPT, "application/vnd.github+json");
        match &self.token {
            Some(token) => request.bearer_auth(token),
            None => request,
        }
    }

    async fn manifest(&self, release: &Release) -> Result<Option<Manifest>> {
        let Some(asset) = release.manifest_asset() else {
            return Ok(None);
        };
        let _permit = self.downloads.acquire().await?;
        let package_json = self
            .http
            .get(&asset.browser_download_url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        let manifest = manifest::build(release, &package_json);
        if manifest.is_none() {
            tracing::warn!(
                release = release.html_url,
                "release skipped: invalid package.json or missing archive"
            );
        }
        Ok(manifest)
    }
}
