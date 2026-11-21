use serde_json::Value;

use crate::github::{Manifest, Readme};

const REPOSITORY_SUFFIX: &str = "/releases/";
const DEPENDENCY_FIELDS: [&str; 2] = ["dependencies", "vpmDependencies"];
const WEB_SCHEMES: [&str; 2] = ["https://", "http://"];

pub(crate) struct PackageView<'a> {
    pub(crate) id: &'a str,
    pub(crate) latest: &'a Manifest,
    pub(crate) versions: Vec<&'a Manifest>,
    pub(crate) readme: Option<&'a Readme>,
}

impl<'a> PackageView<'a> {
    pub(super) fn text(&self, key: &str) -> Option<&'a str> {
        text(self.latest, key)
    }

    pub(super) fn display_name(&self) -> &'a str {
        self.text("displayName").unwrap_or(self.id)
    }

    pub(super) fn author(&self) -> Option<&'a str> {
        let author = self.latest.get("author")?;
        author
            .as_str()
            .or_else(|| author.get("name").and_then(Value::as_str))
    }

    pub(super) fn repository(&self) -> Option<&'a str> {
        let (repository, _) = web_url(self.latest, "url")?.split_once(REPOSITORY_SUFFIX)?;
        Some(repository)
    }

    pub(super) fn requirements(&self) -> Vec<String> {
        DEPENDENCY_FIELDS
            .iter()
            .filter_map(|field| self.latest.get(*field)?.as_object())
            .flatten()
            .filter_map(|(name, range)| Some(format!("{name} {}", range.as_str()?)))
            .collect()
    }

    pub(super) fn has_safe_id(&self) -> bool {
        !self.id.is_empty()
            && self
                .id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
    }

    pub(super) fn page_path(&self) -> String {
        format!("packages/{}/index.html", self.id)
    }
}

pub(super) fn text<'a>(manifest: &'a Manifest, key: &str) -> Option<&'a str> {
    manifest
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
}

pub(super) fn web_url<'a>(manifest: &'a Manifest, key: &str) -> Option<&'a str> {
    text(manifest, key).filter(|url| WEB_SCHEMES.iter().any(|scheme| url.starts_with(scheme)))
}
