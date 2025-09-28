//! Smart Code Diff Web Server
//!
//! Web server providing REST API and web interface for smart code diffing.

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tracing_subscriber;

mod api;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/api/compare", post(handlers::compare))
        .route("/api/health", get(handlers::health))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Smart Diff Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
