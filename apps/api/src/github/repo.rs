use reqwest::header::HeaderValue;

use super::{Manifest, Readme};

pub(crate) struct Repo {
    pub(crate) slug: String,
    pub(crate) manifests: Vec<Manifest>,
    pub(crate) readme: Option<Readme>,
    pub(super) releases_etag: Option<HeaderValue>,
    pub(super) readme_etag: Option<HeaderValue>,
}

impl Repo {
    pub(crate) const fn new(slug: String) -> Self {
        Self {
            slug,
            manifests: Vec::new(),
            readme: None,
            releases_etag: None,
            readme_etag: None,
        }
    }
}
