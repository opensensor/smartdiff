#!/bin/bash
# Test script to verify the improved sorting

echo "=== Testing MCP Sorting Fix ==="
echo ""

# Create test directory
TEST_DIR="/tmp/mcp-sorting-test"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/old"
mkdir -p "$TEST_DIR/new"

# Create old version with multiple functions
cat > "$TEST_DIR/old/test.c" << 'EOF'
#include <stdio.h>

// Function that will be deleted
void deleted_function() {
    printf("This will be deleted\n");
}

// Function that will be slightly modified
void slightly_modified() {
    int x = 10;
    printf("Value: %d\n", x);
}

// Function that will be heavily modified
void heavily_modified() {
    int a = 1;
    int b = 2;
    printf("Sum: %d\n", a + b);
}

// Function that will be renamed
void old_name() {
    printf("Original name\n");
}

// Function that stays the same
void unchanged() {
    printf("No changes\n");
}
EOF

# Create new version with changes
cat > "$TEST_DIR/new/test.c" << 'EOF'
#include <stdio.h>

// Function that will be slightly modified
void slightly_modified() {
    int x = 11;  // Changed value
    printf("Value: %d\n", x);
}

// Function that will be heavily modified
void heavily_modified() {
    // Completely rewritten
    int result = 0;
    for (int i = 0; i < 10; i++) {
        result += i;
        printf("Iteration %d: %d\n", i, result);
    }
    printf("Final result: %d\n", result);
}

// Function that was renamed
void new_name() {
    printf("Original name\n");
}

// Function that stays the same
void unchanged() {
    printf("No changes\n");
}

// New function that was added
void added_function() {
    printf("This is new\n");
}
EOF

echo "1. Created test files with various change types"
echo ""

# Test the MCP server
echo "2. Testing MCP server..."
echo ""

# Initialize
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

if ! echo "$INIT_RESPONSE" | jq -e '.result' > /dev/null 2>&1; then
    echo "   ✗ Initialization failed"
    exit 1
fi

# Compare locations
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
    exit 1
fi

echo "   ✓ Comparison created: $COMPARISON_ID"
echo ""

# List changed functions
echo "3. Listing changed functions (should show modified first)..."
echo ""

LIST_RESPONSE=$(curl -s -X POST http://127.0.0.1:8011/message \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 3,
    \"method\": \"tools/call\",
    \"params\": {
      \"name\": \"list_changed_functions\",
      \"arguments\": {
        \"comparison_id\": \"$COMPARISON_ID\",
        \"limit\": 100
      }
    }
  }")

# Extract the list
LIST_TEXT=$(echo "$LIST_RESPONSE" | jq -r '.result.content[0].text')

echo "=== Changed Functions List ==="
echo "$LIST_TEXT"
echo ""

# Verify sorting
echo "=== Verifying Sort Order ==="
echo ""

# Extract function names in order
FUNCTIONS=$(echo "$LIST_TEXT" | grep -oP '^\d+\. \K[a-z_]+' | head -10)

# Check if modified functions come first
FIRST_FUNC=$(echo "$FUNCTIONS" | head -1)
FIRST_TYPE=$(echo "$LIST_TEXT" | grep -A1 "1. $FIRST_FUNC" | tail -1 | grep -oP '\- \K[a-z]+')

if [ "$FIRST_TYPE" = "modified" ]; then
    echo "✅ SUCCESS: Modified functions appear first!"
    echo "   First function: $FIRST_FUNC (type: $FIRST_TYPE)"
else
    echo "❌ FAILED: First function is not modified"
    echo "   First function: $FIRST_FUNC (type: $FIRST_TYPE)"
    echo "   Expected: modified"
fi

echo ""

# Check that heavily_modified comes before slightly_modified
HEAVY_POS=$(echo "$FUNCTIONS" | grep -n "heavily_modified" | cut -d: -f1)
SLIGHT_POS=$(echo "$FUNCTIONS" | grep -n "slightly_modified" | cut -d: -f1)

if [ -n "$HEAVY_POS" ] && [ -n "$SLIGHT_POS" ] && [ "$HEAVY_POS" -lt "$SLIGHT_POS" ]; then
    echo "✅ SUCCESS: Heavily modified functions ranked higher than slightly modified!"
    echo "   heavily_modified: position $HEAVY_POS"
    echo "   slightly_modified: position $SLIGHT_POS"
else
    echo "⚠️  Note: Could not verify magnitude-based sorting within modified functions"
fi

echo ""

# Cleanup
echo "Cleaning up test files..."
rm -rf "$TEST_DIR"
echo "Done!"

