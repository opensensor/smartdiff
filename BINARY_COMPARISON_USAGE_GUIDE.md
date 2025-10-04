# Binary Comparison Usage Guide

## Overview

smartdiff now supports binary comparison through integration with Binary Ninja MCP servers. This enables AI agents to:

1. **Discover binaries** loaded in Binary Ninja
2. **List functions** in binaries
3. **Decompile functions** to see high-level code
4. **Compare binaries** (coming in Phase 3)

## Architecture

```
AI Agent (Claude Desktop)
    ↓ (MCP/stdio)
smartdiff MCP Server
    ↓ (HTTP client)
Binary Ninja MCP Server(s)
    ↓ (Python API)
Binary Ninja (GUI with Personal License)
```

## Prerequisites

### 1. Binary Ninja

- Binary Ninja installed (Personal License or higher)
- Binary Ninja MCP plugin installed (from `/home/matteius/codediff/binary_ninja_mcp/`)

### 2. Binary Ninja MCP Server

Located at: `/home/matteius/codediff/binary_ninja_mcp/`

**Installation**:
```bash
cd /home/matteius/codediff/binary_ninja_mcp
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

**Plugin Installation**:
1. Copy `plugin/` directory to Binary Ninja plugins folder
2. Restart Binary Ninja
3. Plugin will appear in Plugins menu

### 3. smartdiff MCP Server

Located at: `/home/matteius/codediff/`

**Build**:
```bash
cd /home/matteius/codediff
cargo build --release -p smart-diff-mcp-server
```

## Setup Workflow

### Step 1: Load Binaries in Binary Ninja

1. **Open Binary Ninja**
2. **Load first binary** (e.g., `malware_v1.exe`)
3. **Start MCP server for this binary**:
   - Plugins > MCP Server > Start Server for This Binary
   - Server will start on port 9009
   - You'll see: "MCP Server started on port 9009"

4. **Open another Binary Ninja window** (or tab)
5. **Load second binary** (e.g., `malware_v2.exe`)
6. **Start MCP server for this binary**:
   - Plugins > MCP Server > Start Server for This Binary
   - Server will start on port 9010
   - You'll see: "MCP Server started on port 9010"

### Step 2: Start Binary Ninja MCP Bridge (Optional)

If you want to use Binary Ninja MCP directly with Claude:

```bash
cd /home/matteius/codediff/binary_ninja_mcp
source .venv/bin/activate
python bridge/bn_mcp_bridge_multi_http.py
```

This bridge connects to Claude Desktop and routes requests to Binary Ninja servers.

### Step 3: Start smartdiff MCP Server

```bash
cd /home/matteius/codediff
cargo run --release --bin smart-diff-mcp
```

The server will start and listen on stdio for MCP requests.

### Step 4: Configure Claude Desktop

Edit `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) or equivalent:

```json
{
  "mcpServers": {
    "smartdiff": {
      "command": "/path/to/codediff/target/release/smart-diff-mcp",
      "args": []
    }
  }
}
```

**Note**: You can also add the Binary Ninja MCP bridge if you want direct access:

```json
{
  "mcpServers": {
    "smartdiff": {
      "command": "/path/to/codediff/target/release/smart-diff-mcp",
      "args": []
    },
    "binary_ninja": {
      "command": "/path/to/binary_ninja_mcp/.venv/bin/python",
      "args": ["/path/to/binary_ninja_mcp/bridge/bn_mcp_bridge_multi_http.py"]
    }
  }
}
```

### Step 5: Restart Claude Desktop

Restart Claude Desktop to load the new MCP server configuration.

## Available Tools

### 1. list_binja_servers

**Description**: List all available Binary Ninja MCP servers with loaded binaries.

**Usage**:
```
User: "What binaries do I have loaded in Binary Ninja?"

Agent calls: list_binja_servers()

Response:
Found 2 Binary Ninja server(s):

Binary ID: port_9009
Filename: malware_v1.exe
Port: 9009
URL: http://localhost:9009

Binary ID: port_9010
Filename: malware_v2.exe
Port: 9010
URL: http://localhost:9010
```

### 2. list_binary_functions

**Description**: List all functions in a binary.

**Parameters**:
- `binary_id` (required): Binary server ID (e.g., "port_9009")
- `search` (optional): Search term to filter functions

**Usage**:
```
User: "Show me all functions in the first binary"

Agent calls: list_binary_functions(binary_id="port_9009")

Response:
Found 150 function(s) in port_9009:

1. main
2. process_data
3. encrypt_payload
4. decrypt_payload
5. connect_to_server
...
```

**With search**:
```
User: "Find functions related to encryption"

Agent calls: list_binary_functions(binary_id="port_9009", search="encrypt")

Response:
Found 3 function(s) matching 'encrypt' in port_9009:

1. encrypt_payload
2. encrypt_string
3. decrypt_payload
```

### 3. decompile_binary_function

**Description**: Decompile a specific function and return the decompiled code.

**Parameters**:
- `binary_id` (required): Binary server ID
- `function_name` (required): Name of the function to decompile

**Usage**:
```
User: "Show me the decompiled code for process_data"

Agent calls: decompile_binary_function(
    binary_id="port_9009",
    function_name="process_data"
)

Response:
Decompiled code for function 'process_data':

```c
int64_t process_data(int64_t arg1, int64_t arg2)
{
    int64_t rax;
    
    if (arg1 != 0)
    {
        rax = encrypt_payload(arg1, arg2);
        send_to_server(rax);
    }
    else
    {
        rax = 0;
    }
    return rax;
}
```
```

## Example Workflows

### Workflow 1: Explore a Binary

```
User: "I have a binary loaded in Binary Ninja. Can you help me understand what it does?"

Agent:
1. Calls list_binja_servers() to see available binaries
2. Calls list_binary_functions(binary_id="port_9009") to see all functions
3. Identifies interesting functions (e.g., "main", "process_data")
4. Calls decompile_binary_function() for each interesting function
5. Analyzes the decompiled code and explains functionality
```

### Workflow 2: Find Specific Functionality

```
User: "Does this binary have any network communication functions?"

Agent:
1. Calls list_binary_functions(binary_id="port_9009", search="send")
2. Calls list_binary_functions(binary_id="port_9009", search="recv")
3. Calls list_binary_functions(binary_id="port_9009", search="connect")
4. Decompiles found functions to analyze network behavior
5. Reports findings
```

### Workflow 3: Compare Two Binaries (Manual - Phase 3 will automate)

```
User: "I have two versions of a binary. What changed between them?"

Agent:
1. Calls list_binja_servers() to see both binaries
2. Calls list_binary_functions() for both binaries
3. Compares function lists to find:
   - Added functions (in v2 but not v1)
   - Deleted functions (in v1 but not v2)
   - Common functions (in both)
4. For common functions, decompiles both versions
5. Compares decompiled code to identify changes
6. Reports significant changes
```

## Troubleshooting

### No servers found

**Problem**: `list_binja_servers()` returns "No Binary Ninja servers found"

**Solutions**:
1. Make sure Binary Ninja is running
2. Make sure you have loaded binaries
3. Make sure MCP server is started for each binary (Plugins > MCP Server > Start Server)
4. Check that servers are running on ports 9009+ (check Binary Ninja console)

### Function not found

**Problem**: `decompile_binary_function()` returns "Function not found"

**Solutions**:
1. Use `list_binary_functions()` to see exact function names
2. Function names are case-sensitive
3. Some functions may have mangled names - try searching first

### Connection timeout

**Problem**: Requests to Binary Ninja servers timeout

**Solutions**:
1. Check that Binary Ninja is still running
2. Check that the binary is still loaded
3. Restart the MCP server in Binary Ninja
4. Check firewall settings (localhost should be allowed)

## Next Steps (Phase 3)

Phase 3 will add automated binary comparison:

### New Tools (Coming Soon)

#### `compare_binaries_via_binja`

Compare two binaries and identify matching functions.

```
compare_binaries_via_binja(
    binary_a_id="port_9009",
    binary_b_id="port_9010",
    options={
        "use_decompiled_code": true,
        "similarity_threshold": 0.7
    }
)
```

#### `list_binary_function_matches`

List matched functions sorted by similarity.

```
list_binary_function_matches(
    comparison_id="uuid",
    sort_by="similarity_asc",
    limit=50
)
```

#### `get_binary_function_diff`

Get detailed diff for a specific function match.

```
get_binary_function_diff(
    comparison_id="uuid",
    function_a="process_data",
    function_b="process_data_v2"
)
```

## Technical Details

### Binary Ninja MCP Client

The client library (`crates/binary-ninja-client/`) provides:

- **No Binary Ninja dependencies**: Just HTTP/JSON
- **Async/await**: Built on tokio
- **Error handling**: Comprehensive error types
- **Multi-server support**: Discover and connect to multiple servers

### MCP Tools

The MCP server (`crates/mcp-server/`) now includes:

- **Source code comparison tools** (existing):
  - `compare_locations`
  - `list_changed_functions`
  - `get_function_diff`
  - `get_comparison_summary`

- **Binary comparison tools** (new):
  - `list_binja_servers`
  - `list_binary_functions`
  - `decompile_binary_function`

All tools are available to AI agents through the MCP protocol.

## License

AGPL-3.0-only

## See Also

- [Binary Ninja MCP Server](binary_ninja_mcp/README.md)
- [Binary Ninja Client Library](crates/binary-ninja-client/README.md)
- [smartdiff MCP Server](crates/mcp-server/README.md)
- [Binary Ninja](https://binary.ninja/)

