# Smart Diff MCP Server

An intelligent Model Context Protocol (MCP) server that provides AI agents with powerful code comparison and analysis capabilities.

## Overview

The Smart Diff MCP Server exposes the advanced code comparison engine through the Model Context Protocol, allowing AI agents to:

- Compare code across files, directories, or commits
- List functions sorted by change magnitude
- Query detailed diffs for individual functions
- Access comparison results through structured resources

## Features

### Tools

The server provides four main tools:

#### 1. `compare_locations`

Initiates a comparison between two code locations (files or directories).

**Parameters:**
- `source_path` (required): Path to source code location
- `target_path` (required): Path to target code location
- `recursive` (optional, default: true): Whether to scan directories recursively
- `file_patterns` (optional): File patterns to include (e.g., `["*.rs", "*.py"]`)
- `ignore_patterns` (optional): File patterns to ignore

**Returns:** A comparison ID for querying results

**Example:**
```json
{
  "name": "compare_locations",
  "arguments": {
    "source_path": "/path/to/old/code",
    "target_path": "/path/to/new/code",
    "recursive": true
  }
}
```

#### 2. `list_changed_functions`

Lists all changed functions from a comparison, sorted by change magnitude (most changed first).

**Parameters:**
- `comparison_id` (required): The comparison ID from `compare_locations`
- `limit` (optional, default: 100): Maximum number of functions to return
- `change_types` (optional): Filter by change types (`["added", "deleted", "modified", "renamed", "moved"]`)
- `min_magnitude` (optional): Minimum change magnitude (0.0 to 1.0)

**Example:**
```json
{
  "name": "list_changed_functions",
  "arguments": {
    "comparison_id": "550e8400-e29b-41d4-a716-446655440000",
    "limit": 50,
    "change_types": ["modified", "added"],
    "min_magnitude": 0.3
  }
}
```

#### 3. `get_function_diff`

Gets detailed diff information for a specific function.

**Parameters:**
- `comparison_id` (required): The comparison ID
- `function_name` (required): Name of the function
- `include_content` (optional, default: true): Whether to include full source/target content

**Example:**
```json
{
  "name": "get_function_diff",
  "arguments": {
    "comparison_id": "550e8400-e29b-41d4-a716-446655440000",
    "function_name": "process_data"
  }
}
```

#### 4. `get_comparison_summary`

Gets summary statistics for a comparison.

**Parameters:**
- `comparison_id` (required): The comparison ID

**Example:**
```json
{
  "name": "get_comparison_summary",
  "arguments": {
    "comparison_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

### Resources

The server exposes comparison results as MCP resources with the following URI scheme:

- `codediff://comparison/{id}/summary` - Comparison summary statistics (JSON)
- `codediff://comparison/{id}/functions` - List of all changed functions (JSON)
- `codediff://comparison/{id}/function/{name}` - Detailed diff for a specific function (JSON)

## Installation

Build the MCP server:

```bash
cargo build --release -p smart-diff-mcp-server
```

The binary will be available at `target/release/smart-diff-mcp`.

## Usage

### Running the Server

The MCP server communicates via stdio (standard input/output) as per the MCP specification:

```bash
./target/release/smart-diff-mcp
```

The server reads JSON-RPC messages from stdin and writes responses to stdout.

### Configuration for AI Agents

#### Claude Desktop

Add to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
**Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "smart-diff": {
      "command": "/absolute/path/to/codediff/target/release/smart-diff-mcp",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

**Important**: Use the absolute path to the binary.

#### Other MCP Clients

Any MCP-compatible client can use the server by launching it as a subprocess and communicating via stdio using JSON-RPC 2.0 messages.

### Example Workflow

1. **Start a comparison:**
   ```
   Agent: Use compare_locations to compare /old/src with /new/src
   ```

2. **List changed functions:**
   ```
   Agent: Use list_changed_functions with the comparison_id to see what changed
   ```

3. **Examine specific changes:**
   ```
   Agent: Use get_function_diff to see details for the most changed function
   ```

## Architecture

The MCP server is built on top of the Smart Diff engine and provides:

- **Comparison Manager**: Manages multiple concurrent comparisons
- **Tool Handler**: Implements MCP tools for code analysis
- **Resource Handler**: Exposes comparison results as MCP resources
- **Stdio Transport**: JSON-RPC communication over stdin/stdout

## Supported Languages

The server supports the same languages as the Smart Diff engine:

- Rust (`.rs`)
- Python (`.py`)
- JavaScript/TypeScript (`.js`, `.ts`)
- Java (`.java`)
- C/C++ (`.c`, `.cpp`, `.h`, `.hpp`)

## Logging

Set the `RUST_LOG` environment variable to control logging:

```bash
RUST_LOG=info ./target/release/smart-diff-mcp
RUST_LOG=debug ./target/release/smart-diff-mcp
```

## Development

Run in development mode:

```bash
cargo run -p smart-diff-mcp-server
```

Run tests:

```bash
cargo test -p smart-diff-mcp-server
```

## License

MIT License - see LICENSE file for details

