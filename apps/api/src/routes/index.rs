use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::{Query, State},
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{
            ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL, CONTENT_ENCODING, CONTENT_TYPE, ETAG,
            IF_NONE_MATCH, VARY,
        },
    },
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::state::AppState;

const CACHE: &str = "public, max-age=60";

#[derive(Deserialize)]
pub(super) struct Params {
    p: Option<String>,
}

struct Listing {
    etag: String,
    body: Bytes,
    encoding: Option<&'static str>,
}

pub(super) async fn handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<Params>,
    headers: HeaderMap,
) -> Response {
    match listing(&state, params.p.as_deref(), &headers) {
        Ok(Some(listing)) => respond(&listing, &headers),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            tracing::error!(%error, "listing serialization failed");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

fn listing(
    state: &AppState,
    requested: Option<&str>,
    headers: &HeaderMap,
) -> serde_json::Result<Option<Listing>> {
    let snapshot = state.snapshot.load();
    let Some(requested) = requested else {
        let negotiated = snapshot.full.negotiate(headers);
        return Ok(Some(Listing {
            etag: format!("W/\"{:x}\"", snapshot.hash),
            body: negotiated.body,
            encoding: negotiated.encoding,
        }));
    };
    let selection = snapshot.select(&state.listing, requested)?;
    Ok(selection.map(|selection| Listing {
        etag: format!("W/\"{:x}-{:x}\"", snapshot.hash, selection.hash),
        body: selection.body.into(),
        encoding: None,
    }))
}

fn respond(listing: &Listing, request_headers: &HeaderMap) -> Response {
    let caching = [
        (ETAG, listing.etag.as_str()),
        (CACHE_CONTROL, CACHE),
        (VARY, "accept-encoding"),
        (ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
    ];
    let is_cached = request_headers
        .get(IF_NONE_MATCH)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.contains(&listing.etag));
    if is_cached {
        return (StatusCode::NOT_MODIFIED, caching).into_response();
    }

    let content_type = [(CONTENT_TYPE, "application/json")];
    let mut response = (caching, content_type, listing.body.clone()).into_response();
    if let Some(encoding) = listing.encoding {
        response
            .headers_mut()
            .insert(CONTENT_ENCODING, HeaderValue::from_static(encoding));
    }
    response
}
