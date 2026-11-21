mod fallback;
mod html;
mod package;
mod sitemap;
mod structured_data;
mod view;

use std::collections::HashMap;

use axum::body::Bytes;

pub(crate) use self::view::PackageView;
use crate::{config::ListingConfig, embedded::Assets};

const BUILD_ORIGIN: &str = "https://vcc.invalid";
const STRUCTURED_DATA_SLOT: &str = "<!--vcc:structured-data-->";
const PACKAGES_SLOT: &str = "<!--vcc:packages-->";
const ANALYTICS_SLOT: &str = "<!--vcc:analytics-->";
const PAGE_EXTENSION: &str = ".html";
const PACKAGE_TEMPLATE: &str = "package-template/index.html";
const HTML: &str = "text/html; charset=utf-8";
const XML: &str = "application/xml; charset=utf-8";
const TEXT: &str = "text/plain; charset=utf-8";
const SITEMAP_PATH: &str = "sitemap.xml";
const ROBOTS_PATH: &str = "robots.txt";

pub(crate) struct Page {
    pub(crate) body: Bytes,
    pub(crate) content_type: &'static str,
}

pub(crate) struct Pages(HashMap<String, Page>);

impl Pages {
    pub(crate) fn render(
        config: &ListingConfig,
        packages: &[PackageView<'_>],
    ) -> serde_json::Result<Self> {
        let list_data = structured_data::list(&config.name, packages)?;
        let fallback = fallback::items(packages);
        let analytics = analytics_script(config);
        let mut html = HashMap::new();
        for path in Assets::iter().filter(|path| path.ends_with(PAGE_EXTENSION)) {
            let Some(file) = Assets::get(&path) else {
                continue;
            };
            let source = String::from_utf8_lossy(&file.data)
                .replace(BUILD_ORIGIN, &config.public_url)
                .replace(ANALYTICS_SLOT, &analytics);
            if let Some(prefix) = path.strip_suffix(PACKAGE_TEMPLATE) {
                for view in packages.iter().filter(|view| view.has_safe_id()) {
                    let page = package::fill(&source, config, prefix, view)?;
                    html.insert(format!("{prefix}{}", view.page_path()), page);
                }
            } else {
                let page = source
                    .replace(STRUCTURED_DATA_SLOT, &list_data)
                    .replace(PACKAGES_SLOT, &fallback);
                html.insert(path.into_owned(), page);
            }
        }
        Ok(Self::assemble(config, html))
    }

    pub(crate) fn get(&self, path: &str) -> Option<&Page> {
        self.0.get(path)
    }

    fn assemble(config: &ListingConfig, html: HashMap<String, String>) -> Self {
        let sitemap = sitemap::render(&config.public_url, html.keys());
        let robots = format!(
            "User-agent: *\nAllow: /\n\nSitemap: {}/{SITEMAP_PATH}\n",
            config.public_url
        );
        let mut pages: HashMap<String, Page> = html
            .into_iter()
            .map(|(path, body)| (path, Page::new(body, HTML)))
            .collect();
        pages.insert(SITEMAP_PATH.to_owned(), Page::new(sitemap, XML));
        pages.insert(ROBOTS_PATH.to_owned(), Page::new(robots, TEXT));
        Self(pages)
    }
}

impl Page {
    fn new(body: String, content_type: &'static str) -> Self {
        Self {
            body: body.into(),
            content_type,
        }
    }
}

fn analytics_script(config: &ListingConfig) -> String {
    config
        .analytics_script_url
        .as_deref()
        .map(|url| format!("<script src=\"{}\" defer></script>", html::escape(url)))
        .unwrap_or_default()
}
