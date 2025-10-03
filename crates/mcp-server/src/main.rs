//! MCP Server for Smart Code Diff
//!
//! This server implements the Model Context Protocol (MCP) to provide
//! intelligent code comparison capabilities to AI agents.
//!
//! The server uses stdio transport as per MCP specification.

use anyhow::Result;
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

    tracing::info!("Starting Smart Diff MCP Server");

    // Create and run the MCP server with stdio transport
    let server = server::McpServer::new();
    server.run().await?;

    Ok(())
}
