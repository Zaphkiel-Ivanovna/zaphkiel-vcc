use std::collections::{BTreeMap, HashMap};

use serde_json::Value;

use super::document::Package;
use crate::{
    github::{Readme, Repo},
    pages::PackageView,
};

pub(super) fn build<'a>(
    groups: &BTreeMap<&'a str, Package<'a>>,
    repos: &'a [Repo],
) -> Vec<PackageView<'a>> {
    let readmes = readmes(repos);
    groups
        .iter()
        .filter_map(|(id, package)| {
            Some(PackageView {
                id,
                latest: package.latest()?,
                versions: package.manifests().collect(),
                readme: readmes.get(id).copied(),
            })
        })
        .collect()
}

fn readmes(repos: &[Repo]) -> HashMap<&str, &Readme> {
    repos
        .iter()
        .filter_map(|repo| Some((repo, repo.readme.as_ref()?)))
        .flat_map(|(repo, readme)| {
            repo.manifests
                .iter()
                .filter_map(|manifest| manifest.get("name").and_then(Value::as_str))
                .map(move |name| (name, readme))
        })
        .collect()
}
