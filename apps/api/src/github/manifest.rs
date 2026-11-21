use serde_json::Value;

use super::release::Release;

pub(crate) type Manifest = serde_json::Map<String, Value>;

pub(super) fn build(release: &Release, package_json: &[u8]) -> Option<Manifest> {
    let mut manifest: Manifest = serde_json::from_slice(package_json).ok()?;
    let name = manifest
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let archive = release.archive(name)?;

    manifest.insert(
        "url".to_owned(),
        archive.browser_download_url.as_str().into(),
    );
    if let Some(sha256) = archive.sha256() {
        manifest.insert("zipSHA256".to_owned(), sha256.into());
    }
    manifest
        .entry("changelogUrl")
        .or_insert_with(|| release.html_url.as_str().into());
    Some(manifest)
}
