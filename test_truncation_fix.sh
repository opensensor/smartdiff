#!/bin/bash
# Test script to verify the truncation fix

echo "=== Testing MCP Truncation Fix ==="
echo ""

# Create test directory
TEST_DIR="/tmp/mcp-truncation-test"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/old"
mkdir -p "$TEST_DIR/new"

# Create a test file with a large function (>200 bytes)
cat > "$TEST_DIR/old/test.c" << 'EOF'
#include <stdio.h>

// This function is intentionally large to test truncation
static void large_function_test(struct work_struct *work) {
    extern struct device *global_device;
    struct device *dev = global_device;
    static int counter = 0;
    
    printf("Starting large function test\n");
    printf("This is line 1 of many lines\n");
    printf("This is line 2 of many lines\n");
    printf("This is line 3 of many lines\n");
    printf("This is line 4 of many lines\n");
    printf("This is line 5 of many lines\n");
    printf("Counter value: %d\n", counter++);
    
    if (dev != NULL) {
        printf("Device is valid\n");
    }
}
EOF

cat > "$TEST_DIR/new/test.c" << 'EOF'
#include <stdio.h>

// This function is intentionally large to test truncation
static void large_function_test(struct work_struct *work) {
    extern struct device *global_device;
    struct device *dev;
    
    /* CRITICAL: Take local reference to prevent race condition */
    dev = global_device;
    
    printf("Starting MODIFIED large function test\n");
    printf("This is line 1 of many MODIFIED lines\n");
    printf("This is line 2 of many MODIFIED lines\n");
    printf("This is line 3 of many MODIFIED lines\n");
    printf("This is line 4 of many MODIFIED lines\n");
    printf("This is line 5 of many MODIFIED lines\n");
    printf("Device status: %s\n", dev ? "valid" : "invalid");
    
    if (dev != NULL) {
        printf("Device is valid and ready\n");
    }
}
EOF

echo "1. Created test files in $TEST_DIR"
echo ""

# Test the MCP server
echo "2. Testing MCP server..."
echo ""

# Initialize
echo "   Initializing..."
INIT_RESPONSE=$(curl -s -X POST http://127.0.0.1:8011/message \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
      "protocolVersion": "2024-11-05",
      "capabilities": {},
      "clientInfo": {"name": "test", "version": "1.0"}
    }
  }')

if echo "$INIT_RESPONSE" | jq -e '.result' > /dev/null 2>&1; then
    echo "   ✓ Initialized"
else
    echo "   ✗ Initialization failed"
    echo "$INIT_RESPONSE" | jq .
    exit 1
fi

# Compare locations
echo "   Creating comparison..."
COMPARE_RESPONSE=$(curl -s -X POST http://127.0.0.1:8011/message \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 2,
    \"method\": \"tools/call\",
    \"params\": {
      \"name\": \"compare_locations\",
      \"arguments\": {
        \"source_path\": \"$TEST_DIR/old\",
        \"target_path\": \"$TEST_DIR/new\",
        \"recursive\": true
      }
    }
  }")

COMPARISON_ID=$(echo "$COMPARE_RESPONSE" | jq -r '.result.content[0].text' | grep -oP 'Comparison ID: \K[a-f0-9-]+')

if [ -z "$COMPARISON_ID" ]; then
    echo "   ✗ Failed to create comparison"
    echo "$COMPARE_RESPONSE" | jq .
    exit 1
fi

echo "   ✓ Comparison created: $COMPARISON_ID"
echo ""

# Get function diff
echo "3. Getting function diff..."
DIFF_RESPONSE=$(curl -s -X POST http://127.0.0.1:8011/message \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 3,
    \"method\": \"tools/call\",
    \"params\": {
      \"name\": \"get_function_diff\",
      \"arguments\": {
        \"comparison_id\": \"$COMPARISON_ID\",
        \"function_name\": \"large_function_test\",
        \"include_content\": true
      }
    }
  }")

# Extract the diff content
DIFF_TEXT=$(echo "$DIFF_RESPONSE" | jq -r '.result.content[0].text')

echo ""
echo "=== Function Diff Result ==="
echo "$DIFF_TEXT"
echo ""

# Check for truncation
if echo "$DIFF_TEXT" | grep -q '\.\.\.'; then
    echo "❌ FAILED: Content is still truncated (contains '...')"
    echo ""
    echo "The fix may not have been applied correctly or the server needs to be restarted."
    exit 1
else
    echo "✅ SUCCESS: No truncation detected!"
    echo ""
    echo "The function content is complete without '...' truncation."
    echo "The fix is working correctly."
fi

# Cleanup
echo ""
echo "Cleaning up test files..."
rm -rf "$TEST_DIR"
echo "Done!"

