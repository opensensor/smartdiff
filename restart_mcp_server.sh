#!/bin/bash
# Script to restart the MCP server with the updated binary

echo "=== Restarting MCP Server with Truncation Fix ==="
echo ""

# Find and kill existing processes
echo "1. Stopping existing MCP server processes..."
pkill -f "smart-diff-mcp" || echo "   No smart-diff-mcp process found"
pkill -f "sse_bridge.py" || echo "   No sse_bridge.py process found"

# Wait a moment for processes to terminate
sleep 2

# Verify processes are stopped
if pgrep -f "smart-diff-mcp" > /dev/null || pgrep -f "sse_bridge.py" > /dev/null; then
    echo "   Warning: Some processes may still be running. Forcing termination..."
    pkill -9 -f "smart-diff-mcp"
    pkill -9 -f "sse_bridge.py"
    sleep 1
fi

echo "   ✓ Processes stopped"
echo ""

# Check if binary exists
if [ ! -f "target/release/smart-diff-mcp" ]; then
    echo "Error: Binary not found at target/release/smart-diff-mcp"
    echo "Please run: cd crates/mcp-server && cargo build --release"
    exit 1
fi

echo "2. Binary found: target/release/smart-diff-mcp"
echo ""

# Start SSE bridge
echo "3. Starting SSE bridge..."
cd crates/mcp-server

# Check if Python virtual environment is activated
if [ -z "$VIRTUAL_ENV" ]; then
    echo "   Note: No Python virtual environment detected"
    echo "   If you have a venv, activate it first with: source /path/to/venv/bin/activate"
fi

# Start the SSE bridge in the background
nohup python3 sse_bridge.py --port 8011 --binary ../../target/release/smart-diff-mcp > /tmp/mcp-sse-bridge.log 2>&1 &
SSE_PID=$!

echo "   ✓ SSE bridge started (PID: $SSE_PID)"
echo "   Log file: /tmp/mcp-sse-bridge.log"
echo "   MCP server stderr: /tmp/mcp-server-stderr.log"
echo ""

# Wait a moment for startup
sleep 2

# Check if processes are running
if pgrep -f "sse_bridge.py" > /dev/null && pgrep -f "smart-diff-mcp" > /dev/null; then
    echo "4. ✓ MCP Server is running with the updated binary!"
    echo ""
    echo "   SSE endpoint: http://127.0.0.1:8011/sse"
    echo "   Message endpoint: http://127.0.0.1:8011/message"
    echo "   Health endpoint: http://127.0.0.1:8011/health"
    echo ""
    echo "   The truncation fix is now active."
    echo "   Function content will no longer be truncated at 200 bytes."
    echo ""
    echo "To view logs:"
    echo "   tail -f /tmp/mcp-sse-bridge.log"
    echo "   tail -f /tmp/mcp-server-stderr.log"
else
    echo "4. ✗ Error: MCP Server failed to start"
    echo ""
    echo "Check the logs for errors:"
    echo "   cat /tmp/mcp-sse-bridge.log"
    echo "   cat /tmp/mcp-server-stderr.log"
    exit 1
fi

