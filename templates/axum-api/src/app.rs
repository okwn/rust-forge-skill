//! Application composition and middleware configuration.

use axum::Router;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::http::routes::create_routes;
use crate::state::AppState;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", create_routes())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}