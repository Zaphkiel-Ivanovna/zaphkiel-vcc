use axum::{
    http::{
        StatusCode, Uri,
        header::{CACHE_CONTROL, CONTENT_TYPE},
    },
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

const INDEX_FILE: &str = "index.html";
const HASHED_ASSETS_PREFIX: &str = "_astro/";
const HASHED_ASSET_CACHE: &str = "public, max-age=31536000, immutable";
const PAGE_CACHE: &str = "public, max-age=300";

#[derive(RustEmbed)]
#[folder = "../web/dist"]
struct Assets;

pub(super) async fn handler(uri: Uri) -> Response {
    let path = match uri.path().trim_start_matches('/') {
        "" => INDEX_FILE,
        path => path,
    };
    let Some(file) = Assets::get(path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let cache = if path.starts_with(HASHED_ASSETS_PREFIX) {
        HASHED_ASSET_CACHE
    } else {
        PAGE_CACHE
    };
    (
        [
            (CONTENT_TYPE, file.metadata.mimetype()),
            (CACHE_CONTROL, cache),
        ],
        file.data,
    )
        .into_response()
}
