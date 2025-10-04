//! Binary Ninja MCP Client
//!
//! HTTP client library for communicating with Binary Ninja MCP servers.
//! This enables smartdiff to fetch binary analysis data from Binary Ninja
//! without requiring direct Binary Ninja API dependencies.
//!
//! # Architecture
//!
//! This client communicates with the Binary Ninja MCP server (from binary_ninja_mcp/)
//! which runs as an HTTP server on ports 9009+. Each loaded binary gets its own
//! server instance on a unique port.
//!
//! # Example
//!
//! ```no_run
//! use smart_diff_binary_ninja_client::BinaryNinjaClient;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = BinaryNinjaClient::new();
//!     
//!     // Discover available Binary Ninja servers
//!     let servers = client.discover_servers().await?;
//!     
//!     for server in servers {
//!         println!("Found binary: {} on port {}", server.filename, server.port);
//!         
//!         // List functions
//!         let functions = client.list_functions(&server.binary_id).await?;
//!         println!("  Functions: {}", functions.len());
//!     }
//!     
//!     Ok(())
//! }
//! ```

use std::time::Duration;
use thiserror::Error;

pub mod client;
pub mod types;

pub use client::BinaryNinjaClient;
pub use types::{BinaryInfo, BinaryNinjaServer, FunctionInfo};

/// Errors that can occur when communicating with Binary Ninja MCP servers
#[derive(Debug, Error)]
pub enum BinaryNinjaError {
    #[error("No Binary Ninja servers found")]
    NoServersAvailable,

    #[error("Binary server '{0}' not found")]
    ServerNotFound(String),

    #[error("HTTP request failed: {0}")]
    RequestFailed(String),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("Binary Ninja returned error: {0}")]
    BinaryNinjaError(String),

    #[error("Function '{0}' not found")]
    FunctionNotFound(String),

    #[error("Timeout waiting for response")]
    Timeout,
}

/// Configuration for Binary Ninja MCP client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for Binary Ninja MCP servers (default: http://localhost)
    pub base_url: String,

    /// Starting port for server discovery (default: 9009)
    pub base_port: u16,

    /// Maximum number of servers to discover (default: 10)
    pub max_servers: usize,

    /// Connection timeout (default: 2 seconds)
    pub connect_timeout: Duration,

    /// Read timeout (default: 30 seconds)
    pub read_timeout: Duration,

    /// Maximum retries for failed requests (default: 2)
    pub max_retries: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost".to_string(),
            base_port: 9009,
            max_servers: 10,
            connect_timeout: Duration::from_secs(2),
            read_timeout: Duration::from_secs(30),
            max_retries: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClientConfig::default();
        assert_eq!(config.base_url, "http://localhost");
        assert_eq!(config.base_port, 9009);
        assert_eq!(config.max_servers, 10);
    }
}

