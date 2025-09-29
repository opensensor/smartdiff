//! Smart Code Diff Web Server
//!
//! Web server providing REST API and web interface for smart code diffing.

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing_subscriber;

mod api;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // Core API endpoints
        .route("/api/health", get(handlers::health))
        .route("/api/compare", post(handlers::compare))
        .route("/api/analyze", post(handlers::analyze))
        .route("/api/configure", post(handlers::configure))
        // File system API endpoints
        .route("/api/filesystem/browse", post(handlers::browse_directory))
        .route("/api/filesystem/read", post(handlers::read_file))
        .route("/api/filesystem/read-multiple", post(handlers::read_multiple_files))
        .route("/api/filesystem/search", post(handlers::search_files))
        // Static files and SPA fallback
        .nest_service("/", ServeDir::new("static"))
        .fallback(handlers::spa_fallback)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Smart Diff Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
