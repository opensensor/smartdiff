# Debugging Smart Diff MCP Server with Augment Code

## Current Issue

The MCP server shows a red dot (disconnected) in Augment Code's MCP interface.

## Debug Steps

### Step 1: Use the Debug Wrapper

I've created a debug wrapper that logs all communication: `target/release/smart-diff-mcp-debug`

**Update your Augment MCP configuration to use:**
```
/home/matteius/codediff/target/release/smart-diff-mcp-debug
```

Instead of:
```
/home/matteius/codediff/target/release/smart-diff-mcp
```

### Step 2: Try Connecting

1. In Augment, toggle the smart-diff MCP server off and back on
2. Or restart Augment Code

### Step 3: Check the Debug Log

```bash
cat /tmp/smart-diff-mcp-debug.log
```

This will show:
- What messages Augment is sending
- What the server is responding with
- Any errors

### Step 4: Common Issues

#### Issue 1: Binary Not Executable
```bash
chmod +x /home/matteius/codediff/target/release/smart-diff-mcp
```

#### Issue 2: Wrong Path
Make sure the path in Augment's config is absolute:
```
/home/matteius/codediff/target/release/smart-diff-mcp
```

#### Issue 3: Protocol Version Mismatch

The server expects MCP protocol version `2024-11-05`. If Augment uses a different version, we may need to update the server.

#### Issue 4: Field Name Casing

MCP uses snake_case for JSON fields:
- ✅ `protocol_version`
- ❌ `protocolVersion`

If Augment sends camelCase, we need to add serde rename attributes.

### Step 5: Manual Test

Test the server manually to verify it works:

```bash
cd /home/matteius/codediff

# Test initialize
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocol_version":"2024-11-05","capabilities":{},"client_info":{"name":"test","version":"1.0"}}}' | ./target/release/smart-diff-mcp
```

Expected response:
```json
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"tools":{},"resources":{}},"serverInfo":{"name":"smart-diff-mcp","version":"0.1.0"}}}
```

### Step 6: Check Server Logs

The server logs to stderr. To see them:

```bash
# Run with debug logging
RUST_LOG=debug ./target/release/smart-diff-mcp
```

Then send a test message in another terminal.

## Quick Diagnostic Script

Run this to check everything:

```bash
#!/bin/bash
cd /home/matteius/codediff

echo "=== Checking Binary ==="
ls -lh target/release/smart-diff-mcp
file target/release/smart-diff-mcp

echo ""
echo "=== Testing Binary ==="
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocol_version":"2024-11-05","capabilities":{},"client_info":{"name":"test","version":"1.0"}}}' | timeout 2 ./target/release/smart-diff-mcp 2>&1

echo ""
echo "=== Checking Debug Log ==="
if [ -f /tmp/smart-diff-mcp-debug.log ]; then
    echo "Last 50 lines of debug log:"
    tail -50 /tmp/smart-diff-mcp-debug.log
else
    echo "No debug log found. Server hasn't been run with debug wrapper yet."
fi
```

## Likely Issues with Augment

### 1. Protocol Version

Augment might use a different MCP protocol version. Check the debug log to see what version it sends.

### 2. Field Naming Convention

Some MCP implementations use camelCase instead of snake_case. We may need to add:

```rust
#[serde(rename = "protocolVersion")]
pub protocol_version: String,
```

### 3. SSE vs Stdio

You mentioned "might need to be an SSE" - Augment might expect HTTP/SSE transport instead of stdio.

If that's the case, we need to:
1. Add back the HTTP/SSE transport
2. Run the server in HTTP mode
3. Configure Augment to connect via HTTP URL instead of command path

### 4. Different Message Format

Augment might use a slightly different message format. The debug log will show this.

## Next Steps

1. **Use the debug wrapper** and check `/tmp/smart-diff-mcp-debug.log`
2. **Share the log contents** so we can see what Augment is actually sending
3. **Check if Augment expects HTTP/SSE** instead of stdio
4. **Verify the protocol version** Augment uses

## If Augment Uses HTTP/SSE

If Augment expects an HTTP endpoint, we need to:

1. Rebuild with HTTP support (I can add this back)
2. Run the server in HTTP mode:
   ```bash
   ./target/release/smart-diff-mcp --mode http --port 3100
   ```
3. Configure Augment with the URL:
   ```
   http://127.0.0.1:3100
   ```

Let me know what you find in the debug log and I can help fix the specific issue!

