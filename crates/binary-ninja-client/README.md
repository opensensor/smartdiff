# Binary Ninja MCP Client

HTTP client library for communicating with Binary Ninja MCP servers.

## Overview

This crate provides a Rust client for the Binary Ninja MCP server (from `binary_ninja_mcp/`). It enables smartdiff to fetch binary analysis data from Binary Ninja without requiring direct Binary Ninja API dependencies.

**Key Features**:
- ✅ No Binary Ninja dependencies required
- ✅ Works with Personal License (via Binary Ninja MCP server)
- ✅ Simple HTTP/JSON API
- ✅ Multi-binary support
- ✅ Async/await with tokio
- ✅ Comprehensive error handling

## Architecture

```
smartdiff (Rust)
    ↓ (HTTP client - this crate)
Binary Ninja MCP Server (Python)
    ↓ (Python API)
Binary Ninja (GUI with Personal License)
```

## Usage

### Basic Example

```rust
use smart_diff_binary_ninja_client::BinaryNinjaClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = BinaryNinjaClient::new();
    
    // Discover available Binary Ninja servers
    let servers = client.discover_servers().await?;
    
    for server in servers {
        println!("Found binary: {} on port {}", server.filename, server.port);
        
        // List functions
        let functions = client.list_functions(&server.binary_id).await?;
        println!("  Functions: {}", functions.len());
        
        // Decompile a function
        if let Some(func) = functions.first() {
            let code = client.decompile_function(&server.binary_id, &func.name).await?;
            println!("  Decompiled {}:\n{}", func.name, code);
        }
    }
    
    Ok(())
}
```

### Custom Configuration

```rust
use smart_diff_binary_ninja_client::{BinaryNinjaClient, ClientConfig};
use std::time::Duration;

let config = ClientConfig {
    base_url: "http://localhost".to_string(),
    base_port: 9009,
    max_servers: 10,
    connect_timeout: Duration::from_secs(2),
    read_timeout: Duration::from_secs(30),
    max_retries: 2,
};

let client = BinaryNinjaClient::with_config(config);
```

## API Reference

### BinaryNinjaClient

#### `new() -> Self`
Create a new client with default configuration.

#### `with_config(config: ClientConfig) -> Self`
Create a new client with custom configuration.

#### `discover_servers() -> Result<Vec<BinaryNinjaServer>>`
Discover available Binary Ninja MCP servers by scanning ports 9009+.

Returns a list of servers with metadata about loaded binaries.

#### `get_binary_info(binary_id: &str) -> Result<BinaryInfo>`
Get information about a specific binary server.

#### `list_functions(binary_id: &str) -> Result<Vec<FunctionInfo>>`
List all functions in a binary.

Returns a list of function names. Use `get_function_info` for detailed information.

#### `search_functions(binary_id: &str, search_term: &str) -> Result<Vec<FunctionInfo>>`
Search for functions by name.

#### `decompile_function(binary_id: &str, function_name: &str) -> Result<String>`
Decompile a function and return the decompiled code.

#### `get_function_info(binary_id: &str, function_name: &str) -> Result<FunctionInfo>`
Get detailed information about a function including decompiled code.

## Data Types

### BinaryNinjaServer

Information about a Binary Ninja MCP server instance.

```rust
pub struct BinaryNinjaServer {
    pub binary_id: String,      // e.g., "port_9009"
    pub url: String,             // e.g., "http://localhost:9009"
    pub port: u16,               // e.g., 9009
    pub filename: String,        // e.g., "malware.exe"
    pub metadata: Option<BinaryMetadata>,
}
```

### FunctionInfo

Information about a function in a binary.

```rust
pub struct FunctionInfo {
    pub name: String,
    pub address: String,
    pub raw_name: Option<String>,
    pub symbol: Option<SymbolInfo>,
    pub decompiled_code: Option<String>,
}
```

### BinaryInfo

Information about a binary.

```rust
pub struct BinaryInfo {
    pub binary_id: String,
    pub filename: String,
    pub loaded: bool,
    pub metadata: Option<BinaryMetadata>,
}
```

## Error Handling

The crate uses `BinaryNinjaError` for domain-specific errors:

```rust
pub enum BinaryNinjaError {
    NoServersAvailable,
    ServerNotFound(String),
    RequestFailed(String),
    ParseError(String),
    BinaryNinjaError(String),
    FunctionNotFound(String),
    Timeout,
}
```

All public methods return `anyhow::Result<T>` for easy error propagation.

## Prerequisites

To use this client, you need:

1. **Binary Ninja** (with Personal License or higher)
2. **Binary Ninja MCP Server** running (from `binary_ninja_mcp/`)
3. **Binaries loaded** in Binary Ninja with MCP server started

### Setup

1. Start Binary Ninja and load a binary
2. Start the MCP server for that binary:
   - Plugins > MCP Server > Start Server for This Binary
   - Server will start on port 9009 (or next available port)
3. Use this client to connect and fetch data

## Testing

Run tests with:

```bash
cargo test -p smart-diff-binary-ninja-client
```

Note: Most tests are unit tests that don't require a running Binary Ninja server. Integration tests that require a server are marked with `#[ignore]`.

## Dependencies

- `reqwest` - HTTP client
- `serde` / `serde_json` - JSON serialization
- `tokio` - Async runtime
- `anyhow` / `thiserror` - Error handling
- `tracing` - Logging

## License

AGPL-3.0-only

## See Also

- [Binary Ninja MCP Server](../../binary_ninja_mcp/) - The server this client connects to
- [smartdiff MCP Server](../mcp-server/) - Uses this client for binary comparison
- [Binary Ninja](https://binary.ninja/) - The reverse engineering platform

