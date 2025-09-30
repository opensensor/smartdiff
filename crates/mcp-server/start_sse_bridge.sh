#!/bin/bash
# Start the SSE bridge for Smart Diff MCP Server

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$SCRIPT_DIR/../../target/release/smart-diff-mcp"

cd "$SCRIPT_DIR"

echo "Starting Smart Diff MCP SSE Bridge..."
echo "Binary: $BINARY_PATH"
echo "Port: 8011"
echo ""

PIPENV_IGNORE_VIRTUALENVS=1 pipenv run python sse_bridge.py --binary "$BINARY_PATH" --port 8011

