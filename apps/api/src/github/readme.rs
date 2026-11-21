const IMAGE_TAG: &str = "<img";
const SOURCE_ATTRIBUTE: &str = "src";
const LINK_ATTRIBUTE: &str = "href";
const ANCHOR_PREFIX: &str = "user-content-";
const ABSOLUTE_PREFIXES: [&str; 5] = ["http://", "https://", "//", "data:", "mailto:"];
const HEADING_DEMOTIONS: [(&str, &str); 5] = [
    ("h5", "h6"),
    ("h4", "h5"),
    ("h3", "h4"),
    ("h2", "h3"),
    ("h1", "h2"),
];
const BADGE_MARKERS: [&str; 3] = ["shields.io", "badge", ".svg"];

pub(crate) struct Readme {
    pub(crate) html: String,
    pub(crate) banner: Option<String>,
}

impl Readme {
    pub(super) fn from_html(slug: &str, html: &str) -> Self {
        let images = format!("https://raw.githubusercontent.com/{slug}/HEAD");
        let files = format!("https://github.com/{slug}/blob/HEAD");
        let html = rewrite(html, SOURCE_ATTRIBUTE, |url| absolute(&images, url));
        let html = rewrite(&html, LINK_ATTRIBUTE, |url| {
            url.strip_prefix('#').map_or_else(
                || absolute(&files, url),
                |anchor| format!("#{ANCHOR_PREFIX}{anchor}"),
            )
        });
        let html = demote_headings(html);
        Self {
            banner: first_image(&html),
            html,
        }
    }
}

fn rewrite(html: &str, attribute: &str, resolve: impl Fn(&str) -> String) -> String {
    let needle = format!(" {attribute}=\"");
    let mut parts = html.split(needle.as_str());
    let mut output = parts.next().unwrap_or_default().to_owned();
    for part in parts {
        let (value, rest) = part.split_once('"').unwrap_or((part, ""));
        output.push_str(&needle);
        output.push_str(&resolve(value));
        output.push('"');
        output.push_str(rest);
    }
    output
}

fn absolute(base: &str, url: &str) -> String {
    if ABSOLUTE_PREFIXES
        .iter()
        .any(|prefix| url.starts_with(prefix))
    {
        return url.to_owned();
    }
    let path = url.trim_start_matches("./").trim_start_matches('/');
    format!("{base}/{path}")
}

fn first_image(html: &str) -> Option<String> {
    html.split(IMAGE_TAG)
        .skip(1)
        .filter_map(|rest| rest.split_once('>'))
        .map(|(tag, _)| tag)
        .filter(|tag| !BADGE_MARKERS.iter().any(|marker| tag.contains(marker)))
        .find_map(|tag| {
            let (_, source) = tag.split_once(" src=\"")?;
            let (url, _) = source.split_once('"')?;
            Some(url.to_owned())
        })
}

fn demote_headings(html: String) -> String {
    HEADING_DEMOTIONS.iter().fold(html, |html, (from, to)| {
        html.replace(&format!("<{from} "), &format!("<{to} "))
            .replace(&format!("<{from}>"), &format!("<{to}>"))
            .replace(&format!("</{from}>"), &format!("</{to}>"))
    })
}
