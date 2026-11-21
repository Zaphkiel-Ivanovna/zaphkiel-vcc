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
    readme::Readme,
    release::Release,
    repo::Repo,
};

const USER_AGENT: &str = "vcc-api";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const RELEASES_PER_PAGE: usize = 100;
const MAX_CONCURRENT_DOWNLOADS: usize = 16;
const JSON_MEDIA_TYPE: &str = "application/vnd.github+json";
const HTML_MEDIA_TYPE: &str = "application/vnd.github.html+json";

enum Fetched<T> {
    NotModified,
    Fresh { value: T, etag: Option<HeaderValue> },
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
        let (releases, readme) = tokio::try_join!(self.releases(repo), self.readme(repo))?;
        let releases_changed = self.apply_releases(repo, releases).await?;
        let readme_changed = apply_readme(repo, readme);
        Ok(releases_changed || readme_changed)
    }

    async fn apply_releases(
        &self,
        repo: &mut Repo,
        releases: Fetched<Vec<Release>>,
    ) -> Result<bool> {
        let Fetched::Fresh { value, etag } = releases else {
            return Ok(false);
        };
        let published = value.iter().filter(|release| !release.draft);
        let manifests = try_join_all(published.map(|release| self.manifest(release))).await?;
        repo.manifests = manifests.into_iter().flatten().collect();
        repo.releases_etag = etag;
        Ok(true)
    }

    async fn releases(&self, repo: &Repo) -> Result<Fetched<Vec<Release>>> {
        let mut value = Vec::new();
        let mut etag = None;
        for page in 1_u32.. {
            let url = format!(
                "https://api.github.com/repos/{}/releases?per_page={RELEASES_PER_PAGE}&page={page}",
                repo.slug
            );
            let known = repo.releases_etag.as_ref().filter(|_| page == 1);
            let response = self
                .api_request(&url, JSON_MEDIA_TYPE, known)
                .send()
                .await?;
            if response.status() == StatusCode::NOT_MODIFIED {
                return Ok(Fetched::NotModified);
            }
            let response = response.error_for_status()?;
            if page == 1 {
                etag = response.headers().get(ETAG).cloned();
            }
            let batch: Vec<Release> = response.json().await?;
            let is_last_page = batch.len() < RELEASES_PER_PAGE;
            value.extend(batch);
            if is_last_page {
                break;
            }
        }
        Ok(Fetched::Fresh { value, etag })
    }

    async fn readme(&self, repo: &Repo) -> Result<Fetched<Option<Readme>>> {
        let url = format!("https://api.github.com/repos/{}/readme", repo.slug);
        let known = repo.readme_etag.as_ref();
        let response = self
            .api_request(&url, HTML_MEDIA_TYPE, known)
            .send()
            .await?;
        match response.status() {
            StatusCode::NOT_MODIFIED => Ok(Fetched::NotModified),
            StatusCode::NOT_FOUND => Ok(Fetched::Fresh {
                value: None,
                etag: None,
            }),
            _ => {
                let response = response.error_for_status()?;
                let etag = response.headers().get(ETAG).cloned();
                let html = response.text().await?;
                Ok(Fetched::Fresh {
                    value: Some(Readme::from_html(&repo.slug, &html)),
                    etag,
                })
            }
        }
    }

    fn api_request(
        &self,
        url: &str,
        media_type: &str,
        known_etag: Option<&HeaderValue>,
    ) -> RequestBuilder {
        let mut request = self.http.get(url).header(ACCEPT, media_type);
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        if let Some(etag) = known_etag {
            request = request.header(IF_NONE_MATCH, etag);
        }
        request
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

fn apply_readme(repo: &mut Repo, readme: Fetched<Option<Readme>>) -> bool {
    let Fetched::Fresh { value, etag } = readme else {
        return false;
    };
    let changed = value.is_some() || repo.readme.is_some();
    repo.readme = value;
    repo.readme_etag = etag;
    changed
}
