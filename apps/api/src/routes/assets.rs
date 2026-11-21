use std::{borrow::Cow, sync::Arc};

use axum::{
    body::Bytes,
    extract::State,
    http::{
        StatusCode, Uri,
        header::{CACHE_CONTROL, CONTENT_TYPE},
    },
    response::{IntoResponse, Response},
};

use crate::{embedded::Assets, pages::Page, state::AppState};

const INDEX_FILE: &str = "index.html";
const NOT_FOUND_PAGE: &str = "404.html";
const PAGE_EXTENSION: &str = ".html";
const HASHED_ASSETS_PREFIX: &str = "_astro/";
const HASHED_ASSET_CACHE: &str = "public, max-age=31536000, immutable";
const PAGE_CACHE: &str = "public, max-age=300";

pub(super) async fn handler(State(state): State<Arc<AppState>>, uri: Uri) -> Response {
    let path = uri.path().trim_matches('/');
    let snapshot = state.snapshot.load();
    let page = [path.to_owned(), page_index(path)]
        .iter()
        .find_map(|candidate| snapshot.pages.get(candidate));
    if let Some(page) = page {
        return page_response(page, StatusCode::OK);
    }
    if let Some(response) = file_response(path) {
        return response;
    }
    snapshot.pages.get(NOT_FOUND_PAGE).map_or_else(
        || StatusCode::NOT_FOUND.into_response(),
        |page| page_response(page, StatusCode::NOT_FOUND),
    )
}

fn page_index(directory: &str) -> String {
    if directory.is_empty() {
        INDEX_FILE.to_owned()
    } else {
        format!("{directory}/{INDEX_FILE}")
    }
}

fn page_response(page: &Page, status: StatusCode) -> Response {
    let headers = [
        (CONTENT_TYPE, page.content_type),
        (CACHE_CONTROL, PAGE_CACHE),
    ];
    (status, headers, page.body.clone()).into_response()
}

fn file_response(path: &str) -> Option<Response> {
    if path.ends_with(PAGE_EXTENSION) {
        return None;
    }
    let file = Assets::get(path)?;
    let cache = if path.starts_with(HASHED_ASSETS_PREFIX) {
        HASHED_ASSET_CACHE
    } else {
        PAGE_CACHE
    };
    let body = match file.data {
        Cow::Borrowed(bytes) => Bytes::from_static(bytes),
        Cow::Owned(bytes) => Bytes::from(bytes),
    };
    let headers = [
        (CONTENT_TYPE, file.metadata.mimetype()),
        (CACHE_CONTROL, cache),
    ];
    Some((headers, body).into_response())
}
