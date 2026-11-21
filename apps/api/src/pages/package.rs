use std::fmt::Write;

use super::{
    STRUCTURED_DATA_SLOT,
    html::{encode_component, escape},
    structured_data,
    view::{PackageView, text, web_url},
};
use crate::config::ListingConfig;

const README_SLOT: &str = "<!--vcc:package-readme-->";
const VERSIONS_SLOT: &str = "<!--vcc:package-versions-->";
const DEFAULT_IMAGE: &str = "og.png";

pub(super) fn fill(
    template: &str,
    config: &ListingConfig,
    prefix: &str,
    view: &PackageView<'_>,
) -> serde_json::Result<String> {
    let home = format!("{}/{prefix}", config.public_url);
    let url = format!("{home}packages/{}/", view.id);
    let page = fields(&config.public_url, view)
        .iter()
        .fold(template.to_owned(), |page, (name, value)| {
            page.replace(&format!("%%{name}%%"), &escape(value))
        });
    let readme = view.readme.map_or("", |readme| readme.html.as_str());
    Ok(page
        .replace(
            STRUCTURED_DATA_SLOT,
            &structured_data::package(view, &home, &url)?,
        )
        .replace(VERSIONS_SLOT, &versions(view))
        .replace(README_SLOT, readme))
}

fn fields(origin: &str, view: &PackageView<'_>) -> [(&'static str, String); 14] {
    let all_listing_url = format!("{origin}/index.json");
    let listing_url = format!("{all_listing_url}?p={}", view.id);
    let banner = view.readme.and_then(|readme| readme.banner.clone());
    let field = |key| view.text(key).unwrap_or_default().to_owned();
    [
        ("id", view.id.to_owned()),
        ("name", view.display_name().to_owned()),
        ("version", field("version")),
        ("description", description(view)),
        ("unity", field("unity")),
        ("license", field("license")),
        ("author", view.author().unwrap_or_default().to_owned()),
        ("requires", view.requirements().join("\n")),
        (
            "repository",
            view.repository().unwrap_or_default().to_owned(),
        ),
        (
            "download",
            web_url(view.latest, "url").unwrap_or_default().to_owned(),
        ),
        (
            "image",
            banner.unwrap_or_else(|| format!("{origin}/{DEFAULT_IMAGE}")),
        ),
        ("add-all-url", add_repo_url(&all_listing_url)),
        ("add-url", add_repo_url(&listing_url)),
        ("all-listing-url", all_listing_url),
    ]
}

fn add_repo_url(listing_url: &str) -> String {
    format!("vcc://vpm/addRepo?url={}", encode_component(listing_url))
}

fn description(view: &PackageView<'_>) -> String {
    view.text("description")
        .unwrap_or_else(|| view.display_name())
        .to_owned()
}

fn versions(view: &PackageView<'_>) -> String {
    view.versions
        .iter()
        .fold(String::new(), |mut html, release| {
            let version = escape(text(release, "version").unwrap_or_default());
            let download = escape(web_url(release, "url").unwrap_or_default());
            let _ = write!(html, "<li><a href=\"{download}\">{version}</a></li>");
            html
        })
}
