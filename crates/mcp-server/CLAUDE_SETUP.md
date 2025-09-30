# Setting Up Smart Diff MCP Server with Claude Desktop

This guide will help you set up the Smart Diff MCP server to work with Claude Desktop.

## Prerequisites

1. Claude Desktop installed
2. Rust toolchain installed
3. This repository cloned

## Step 1: Build the MCP Server

```bash
cd /path/to/codediff
cargo build --release -p smart-diff-mcp-server
```

The binary will be at: `target/release/smart-diff-mcp`

## Step 2: Find Your Claude Desktop Config File

The location depends on your operating system:

- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **Linux**: `~/.config/Claude/claude_desktop_config.json`

If the file doesn't exist, create it.

## Step 3: Add the MCP Server Configuration

Edit the config file and add the smart-diff server:

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

**Important**: Replace `/absolute/path/to/codediff` with the actual absolute path to your repository.

### Example (macOS/Linux):

```json
{
  "mcpServers": {
    "smart-diff": {
      "command": "/home/username/projects/codediff/target/release/smart-diff-mcp",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Example (Windows):

```json
{
  "mcpServers": {
    "smart-diff": {
      "command": "C:\\Users\\username\\projects\\codediff\\target\\release\\smart-diff-mcp.exe",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

## Step 4: Restart Claude Desktop

Close and reopen Claude Desktop to load the new MCP server.

## Step 5: Verify the Setup

In Claude Desktop, you can verify the server is loaded by asking:

```
Can you list the available MCP tools?
```

You should see the smart-diff tools listed:
- `compare_locations`
- `list_changed_functions`
- `get_function_diff`
- `get_comparison_summary`

## Step 6: Test the Server

Try a simple comparison:

```
Please use the smart-diff MCP server to compare these two directories:
- Source: /path/to/old/version
- Target: /path/to/new/version

Show me what functions changed.
```

Claude will use the MCP server to perform the comparison and report the results.

## Troubleshooting

### Server Not Appearing

1. **Check the config file path**: Make sure you edited the correct file for your OS
2. **Verify the binary path**: Ensure the path in the config is absolute and correct
3. **Check file permissions**: Make sure the binary is executable:
   ```bash
   chmod +x target/release/smart-diff-mcp
   ```
4. **View logs**: Check Claude's logs for errors:
   - macOS: `~/Library/Logs/Claude/mcp*.log`
   - Windows: `%APPDATA%\Claude\Logs\mcp*.log`
   - Linux: `~/.config/Claude/logs/mcp*.log`

### Server Crashes or Errors

1. **Check RUST_LOG**: Set to `debug` for more detailed logs:
   ```json
   "env": {
     "RUST_LOG": "debug"
   }
   ```

2. **Test manually**: Run the server directly to see errors:
   ```bash
   ./target/release/smart-diff-mcp
   ```
   Then type a test message:
   ```json
   {"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocol_version":"2024-11-05","capabilities":{},"client_info":{"name":"test","version":"1.0"}}}
   ```

3. **Rebuild**: Try a clean rebuild:
   ```bash
   cargo clean
   cargo build --release -p smart-diff-mcp-server
   ```

### Comparison Fails

1. **Check paths**: Ensure the paths you're comparing exist and are readable
2. **Supported languages**: The server supports Rust, Python, JavaScript, Java, C/C++
3. **File permissions**: Make sure the server can read the files

## Example Usage

Once set up, you can ask Claude things like:

### Basic Comparison
```
Compare /old/src with /new/src and tell me what changed
```

### Focused Analysis
```
Compare these directories and show me only the functions that changed significantly (more than 50%)
```

### Detailed Review
```
Compare the old and new versions, list the top 5 most changed functions, and explain what changed in each
```

### Refactoring Detection
```
Analyze the code changes and identify any refactoring patterns like function renames or moves
```

## Advanced Configuration

### Adjust Log Level

For production use, you might want less verbose logging:

```json
"env": {
  "RUST_LOG": "warn"
}
```

For debugging:

```json
"env": {
  "RUST_LOG": "trace"
}
```

### Multiple MCP Servers

You can have multiple MCP servers configured:

```json
{
  "mcpServers": {
    "smart-diff": {
      "command": "/path/to/smart-diff-mcp",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    },
    "other-server": {
      "command": "/path/to/other-mcp-server",
      "args": []
    }
  }
}
```

## Next Steps

- Read [MCP_USAGE.md](MCP_USAGE.md) for detailed tool documentation
- Check [README.md](README.md) for feature overview
- See [MCP_IMPLEMENTATION_SUMMARY.md](../../../MCP_IMPLEMENTATION_SUMMARY.md) for architecture details

## Support

If you encounter issues:

1. Check the troubleshooting section above
2. Review the logs in Claude's log directory
3. Test the server manually with JSON-RPC messages
4. Ensure you're using the latest version of Claude Desktop

