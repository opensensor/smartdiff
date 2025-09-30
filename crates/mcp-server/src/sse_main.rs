//! MCP Server with SSE Transport for Smart Code Diff
//!
//! This server implements the Model Context Protocol (MCP) using
//! Server-Sent Events (SSE) transport instead of stdio.
//!
//! SSE transport is better for long-running operations as it doesn't
//! have the timeout limitations of stdio transport.

use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

mod comparison;
mod mcp;
mod resources;
mod server;
mod tools;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tracing::info!("Starting Smart Diff MCP Server with SSE transport");

    // Create the MCP server in SSE mode
    let server = Arc::new(server::McpServer::new_sse());

    // Create SSE router
    let app = mcp::sse_transport::create_sse_router(server);

    // Add CORS middleware
    let app = app.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );

    // Bind to address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8011));
    tracing::info!("SSE server listening on http://{}", addr);
    tracing::info!("SSE endpoint: http://{}/sse", addr);
    tracing::info!("Message endpoint: http://{}/message", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

