use std::fmt::Write;

use super::{html::escape, view::PackageView};

pub(super) fn items(packages: &[PackageView<'_>]) -> String {
    packages.iter().fold(String::new(), |mut html, view| {
        let _ = write!(
            html,
            "<li><h3><a href=\"/{}\">{}</a></h3><p>{}</p><p><code>{}</code> {}</p></li>",
            escape(view.page_path().trim_end_matches("index.html")),
            escape(view.display_name()),
            escape(view.text("description").unwrap_or_default()),
            escape(view.id),
            escape(view.text("version").unwrap_or_default()),
        );
        html
    })
}
