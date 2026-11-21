use reqwest::header::HeaderValue;

use super::Manifest;

pub(crate) struct Repo {
    pub(crate) slug: String,
    pub(crate) manifests: Vec<Manifest>,
    pub(super) etag: Option<HeaderValue>,
}

impl Repo {
    pub(crate) const fn new(slug: String) -> Self {
        Self {
            slug,
            manifests: Vec::new(),
            etag: None,
        }
    }
}
