use std::path::Path;

use serde::Deserialize;

const MANIFEST_FILE_NAME: &str = "package.json";
const ARCHIVE_EXTENSION: &str = "zip";
const SHA256_PREFIX: &str = "sha256:";

#[derive(Deserialize)]
pub(super) struct Release {
    pub(super) html_url: String,
    pub(super) draft: bool,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
pub(super) struct Asset {
    name: String,
    pub(super) browser_download_url: String,
    digest: Option<String>,
}

impl Release {
    pub(super) fn manifest_asset(&self) -> Option<&Asset> {
        self.assets
            .iter()
            .find(|asset| asset.name == MANIFEST_FILE_NAME)
    }

    pub(super) fn archive(&self, package_name: &str) -> Option<&Asset> {
        let mut archives = self.assets.iter().filter(|asset| asset.is_archive());
        archives
            .clone()
            .find(|asset| asset.name.starts_with(package_name))
            .or_else(|| archives.next())
    }
}

impl Asset {
    pub(super) fn sha256(&self) -> Option<&str> {
        self.digest.as_deref()?.strip_prefix(SHA256_PREFIX)
    }

    fn is_archive(&self) -> bool {
        Path::new(&self.name)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case(ARCHIVE_EXTENSION))
    }
}
