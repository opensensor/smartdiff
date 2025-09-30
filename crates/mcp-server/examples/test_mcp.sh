#!/bin/bash
# Test script for MCP server

# Build the server
echo "Building MCP server..."
cargo build --release -p smart-diff-mcp-server

# Create test directories
mkdir -p /tmp/mcp-test/old
mkdir -p /tmp/mcp-test/new

# Create test files
cat > /tmp/mcp-test/old/example.rs << 'EOF'
fn hello() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}
EOF

cat > /tmp/mcp-test/new/example.rs << 'EOF'
fn hello_world() {
    println!("Hello, world!");
    println!("Welcome!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(x: i32, y: i32) -> i32 {
    if y != 0 {
        x / y
    } else {
        0
    }
}
EOF

echo "Test files created in /tmp/mcp-test/"
echo ""
echo "To test the MCP server, run:"
echo "  ./target/release/smart-diff-mcp"
echo ""
echo "Then send JSON-RPC requests like:"
echo ""
echo "Initialize:"
cat << 'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocol_version":"2024-11-05","capabilities":{},"client_info":{"name":"test","version":"1.0"}}}
EOF
echo ""
echo ""
echo "List tools:"
cat << 'EOF'
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
EOF
echo ""
echo ""
echo "Compare locations:"
cat << 'EOF'
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"compare_locations","arguments":{"source_path":"/tmp/mcp-test/old","target_path":"/tmp/mcp-test/new","recursive":true}}}
EOF
echo ""
echo ""
echo "List changed functions (use comparison_id from previous response):"
cat << 'EOF'
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"list_changed_functions","arguments":{"comparison_id":"YOUR_COMPARISON_ID_HERE"}}}
EOF
echo ""

