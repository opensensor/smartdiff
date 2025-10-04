#!/bin/bash
# Test script for Binary Ninja comparison functionality

echo "=== Testing Binary Ninja MCP Integration ==="
echo ""

# Test 1: Discover Binary Ninja servers
echo "Test 1: Discovering Binary Ninja servers..."
curl -s http://localhost:9009/health 2>/dev/null && echo "✓ Server on port 9009 is running" || echo "✗ Server on port 9009 not found"
curl -s http://localhost:9010/health 2>/dev/null && echo "✓ Server on port 9010 is running" || echo "✗ Server on port 9010 not found"
echo ""

# Test 2: Get binary info from first server
echo "Test 2: Getting binary info from port 9009..."
curl -s -X POST http://localhost:9009/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "get_binary_status_BN_MCP",
      "arguments": {}
    }
  }' | jq -r '.result.content[0].text' 2>/dev/null || echo "Failed to get binary info"
echo ""

# Test 3: Get binary info from second server
echo "Test 3: Getting binary info from port 9010..."
curl -s -X POST http://localhost:9010/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "get_binary_status_BN_MCP",
      "arguments": {}
    }
  }' | jq -r '.result.content[0].text' 2>/dev/null || echo "Failed to get binary info"
echo ""

# Test 4: List functions from first binary
echo "Test 4: Listing functions from port 9009 (first 10)..."
curl -s -X POST http://localhost:9009/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "list_entities_BN_MCP",
      "arguments": {
        "kind": "methods",
        "limit": 10
      }
    }
  }' | jq -r '.result.content[0].text' 2>/dev/null || echo "Failed to list functions"
echo ""

# Test 5: List functions from second binary
echo "Test 5: Listing functions from port 9010 (first 10)..."
curl -s -X POST http://localhost:9010/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "list_entities_BN_MCP",
      "arguments": {
        "kind": "methods",
        "limit": 10
      }
    }
  }' | jq -r '.result.content[0].text' 2>/dev/null || echo "Failed to list functions"
echo ""

echo "=== Test Complete ==="

