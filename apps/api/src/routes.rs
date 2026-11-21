mod assets;
mod index;

use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::compression::CompressionLayer;

use crate::state::AppState;

pub(crate) fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/index.json", get(index::handler))
        .fallback(assets::handler)
        .layer(CompressionLayer::new())
        .with_state(state)
}
