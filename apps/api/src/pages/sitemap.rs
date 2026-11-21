use std::fmt::Write;

use super::html::escape;

const INDEX_FILE: &str = "index.html";
const HEADER: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n";

pub(super) fn render<'a>(origin: &str, paths: impl Iterator<Item = &'a String>) -> String {
    let mut directories: Vec<&str> = paths
        .filter_map(|path| path.strip_suffix(INDEX_FILE))
        .collect();
    directories.sort_unstable();
    let mut xml = directories
        .iter()
        .fold(HEADER.to_owned(), |mut xml, directory| {
            let location = escape(&format!("{origin}/{directory}"));
            let _ = writeln!(xml, "  <url><loc>{location}</loc></url>");
            xml
        });
    xml.push_str("</urlset>\n");
    xml
}
