use std::collections::BTreeMap;

use anyhow::Result;
use serde_json::value::{RawValue, to_raw_value};

use super::{
    document::{self, Document, RawPackages},
    encoded::Encoded,
    hash::fnv1a,
};
use crate::{config::ListingConfig, github::Manifest};

pub(crate) struct Snapshot {
    pub(crate) hash: u64,
    pub(crate) full: Encoded,
    packages: BTreeMap<String, Box<RawValue>>,
}

pub(crate) struct Selection {
    pub(crate) hash: u64,
    pub(crate) body: Vec<u8>,
}

impl Snapshot {
    pub(crate) fn build<'a>(
        config: &ListingConfig,
        manifests: impl Iterator<Item = &'a Manifest>,
    ) -> Result<Self> {
        let packages = document::group(manifests)
            .iter()
            .map(|(name, package)| Ok(((*name).to_owned(), to_raw_value(package)?)))
            .collect::<serde_json::Result<BTreeMap<_, _>>>()?;
        let body = serde_json::to_vec(&Document {
            name: &config.name,
            id: &config.id,
            url: &format!("{}/index.json", config.public_url),
            author: &config.author,
            packages: raw_packages(packages.iter()),
        })?;
        Ok(Self {
            hash: fnv1a(&body),
            full: Encoded::new(body)?,
            packages,
        })
    }

    pub(crate) fn select(
        &self,
        config: &ListingConfig,
        requested: &str,
    ) -> serde_json::Result<Option<Selection>> {
        let found = requested
            .split(',')
            .filter_map(|name| self.packages.get_key_value(name));
        let packages = raw_packages(found);
        if packages.is_empty() {
            return Ok(None);
        }
        let names = packages.keys().copied().collect::<Vec<_>>().join(",");
        let hash = fnv1a(names.as_bytes());
        let body = serde_json::to_vec(&Document {
            name: &format!("{} (custom)", config.name),
            id: &format!("{}.{hash:x}", config.id),
            url: &format!("{}/index.json?p={names}", config.public_url),
            author: &config.author,
            packages,
        })?;
        Ok(Some(Selection { hash, body }))
    }
}

fn raw_packages<'a>(
    packages: impl Iterator<Item = (&'a String, &'a Box<RawValue>)>,
) -> RawPackages<'a> {
    packages
        .map(|(name, json)| (name.as_str(), &**json))
        .collect()
}
