#!/bin/bash
# Test script for Binary Ninja MCP Bridge Integration

echo "=== Testing Binary Ninja MCP Bridge Integration ==="
echo ""

# Test 1: List available Binary Ninja servers
echo "Test 1: Listing available Binary Ninja servers..."
curl -s -X POST http://localhost:8010/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "list_binary_servers_BN_MCP",
      "arguments": {}
    }
  }' | jq '.'
echo ""

# Test 2: Get binary info from first server
echo "Test 2: Getting binary info from first server..."
curl -s -X POST http://localhost:8010/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 2,
    "method": "tools/call",
    "params": {
      "name": "get_binary_info_BN_MCP",
      "arguments": {
        "binary_id": "port_9009"
      }
    }
  }' | jq '.'
echo ""

# Test 3: List functions from first binary
echo "Test 3: Listing functions from first binary (limit 10)..."
curl -s -X POST http://localhost:8010/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 3,
    "method": "tools/call",
    "params": {
      "name": "list_entities_BN_MCP",
      "arguments": {
        "binary_id": "port_9009",
        "kind": "methods",
        "limit": 10
      }
    }
  }' | jq '.'
echo ""

# Test 4: List functions from second binary
echo "Test 4: Listing functions from second binary (limit 10)..."
curl -s -X POST http://localhost:8010/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 4,
    "method": "tools/call",
    "params": {
      "name": "list_entities_BN_MCP",
      "arguments": {
        "binary_id": "port_9010",
        "kind": "methods",
        "limit": 10
      }
    }
  }' | jq '.'
echo ""

# Test 5: Decompile a function from first binary
echo "Test 5: Decompiling 'main' function from first binary..."
curl -s -X POST http://localhost:8010/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 5,
    "method": "tools/call",
    "params": {
      "name": "decompile_function_BN_MCP",
      "arguments": {
        "binary_id": "port_9009",
        "name": "main"
      }
    }
  }' | jq -r '.result.content[0].text' 2>/dev/null || echo "Function not found or error"
echo ""

echo "=== Test Complete ==="

