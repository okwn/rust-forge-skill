//! Router type alias and route construction.

use axum::{routing::get, Router};
use serde_json::json;

use crate::state::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/version", get(version))
}

async fn healthz() -> &'static str {
    "ok"
}

async fn readyz() -> &'static str {
    "ready"
}

async fn version() -> axum::Json<serde_json::Value> {
    axum::Json(json!({
        "version": env!("CARGO_PKG_VERSION")
    }))
}