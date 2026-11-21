use std::{cmp::Reverse, collections::BTreeMap};

use semver::Version;
use serde::Serialize;
use serde_json::value::RawValue;

use crate::github::Manifest;

pub(super) type RawPackages<'a> = BTreeMap<&'a str, &'a RawValue>;

#[derive(Serialize)]
pub(super) struct Document<'a> {
    pub(super) name: &'a str,
    pub(super) id: &'a str,
    pub(super) url: &'a str,
    pub(super) author: &'a str,
    pub(super) packages: RawPackages<'a>,
}

#[derive(Default, Serialize)]
pub(super) struct Package<'a> {
    versions: BTreeMap<Reverse<Version>, &'a Manifest>,
}

pub(super) fn group<'a>(
    manifests: impl Iterator<Item = &'a Manifest>,
) -> BTreeMap<&'a str, Package<'a>> {
    let mut packages: BTreeMap<&str, Package<'_>> = BTreeMap::new();
    for manifest in manifests {
        let field = |key| manifest.get(key).and_then(|value| value.as_str());
        let version = field("version").and_then(|version| Version::parse(version).ok());
        if let (Some(name), Some(version)) = (field("name"), version) {
            packages
                .entry(name)
                .or_default()
                .versions
                .insert(Reverse(version), manifest);
        }
    }
    packages
}
