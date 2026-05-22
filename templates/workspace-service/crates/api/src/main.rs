use anyhow::{Context, Result};
use axum::Router;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/health", axum::routing::get(health))
        .nest("/api/v1", routes::routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!(addr = %addr, "starting server");

    let listener = tokio::net::TcpListener::bind(addr).await
        .context("failed to bind")?;

    axum::serve(listener, app).await
        .context("server error")?;

    Ok(())
}

async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}