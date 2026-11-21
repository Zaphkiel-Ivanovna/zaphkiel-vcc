use serde_json::{Map, Value, json};

use super::view::{PackageView, web_url};

const CONTEXT: &str = "https://schema.org";

pub(super) fn list(listing_name: &str, packages: &[PackageView<'_>]) -> serde_json::Result<String> {
    let items: Vec<Value> = packages
        .iter()
        .zip(1_u32..)
        .map(|(view, position)| {
            json!({ "@type": "ListItem", "position": position, "item": source_code(view) })
        })
        .collect();
    script(&json!({
        "@context": CONTEXT,
        "@type": "ItemList",
        "name": listing_name,
        "numberOfItems": items.len(),
        "itemListElement": items,
    }))
}

pub(super) fn package(view: &PackageView<'_>, home: &str, url: &str) -> serde_json::Result<String> {
    let mut code = source_code(view);
    if let Some(properties) = code.as_object_mut() {
        properties.insert("@id".to_owned(), format!("{url}#package").into());
        properties.insert("url".to_owned(), url.into());
        properties.insert("mainEntityOfPage".to_owned(), url.into());
    }
    script(&json!({
        "@context": CONTEXT,
        "@graph": [
            code,
            {
                "@type": "BreadcrumbList",
                "itemListElement": [
                    { "@type": "ListItem", "position": 1, "name": "VCC Listing", "item": home },
                    { "@type": "ListItem", "position": 2, "name": view.display_name(), "item": url },
                ],
            },
        ],
    }))
}

fn script(data: &Value) -> serde_json::Result<String> {
    let json = serde_json::to_string(data)?.replace('<', "\\u003c");
    Ok(format!(
        r#"<script type="application/ld+json">{json}</script>"#
    ))
}

fn source_code(view: &PackageView<'_>) -> Value {
    let runtime = view.text("unity").map(|unity| format!("Unity {unity}"));
    let banner = view.readme.and_then(|readme| readme.banner.as_deref());
    let properties = [
        ("name", Some(view.display_name())),
        ("identifier", Some(view.id)),
        ("description", view.text("description")),
        ("version", view.text("version")),
        ("license", view.text("license")),
        ("runtimePlatform", runtime.as_deref()),
        ("codeRepository", view.repository()),
        ("downloadUrl", web_url(view.latest, "url")),
        ("image", banner),
    ];

    let mut code = Map::new();
    code.insert("@type".to_owned(), "SoftwareSourceCode".into());
    code.insert("programmingLanguage".to_owned(), "C#".into());
    for (property, value) in properties {
        if let Some(value) = value {
            code.insert(property.to_owned(), value.into());
        }
    }
    if let Some(author) = view.author() {
        code.insert(
            "author".to_owned(),
            json!({ "@type": "Person", "name": author }),
        );
    }
    Value::Object(code)
}
