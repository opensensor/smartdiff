# Frontend MCP Integration Fix

## Problem

The frontend was trying to get function diffs by reading files directly from the filesystem using `/api/comparison/diff`, which failed with "Not Found" errors because:

1. The file paths from the comparison were relative, not absolute
2. The Next.js server couldn't access the files
3. The frontend wasn't using the MCP server's `get_function_diff` tool

## Solution

Created an MCP-based function diff API that proxies to the MCP server.

### Files Created/Modified

#### 1. New API Route: `nextjs-frontend/src/app/api/mcp/function-diff/route.ts`

This route:
- Accepts `comparisonId`, `functionName`, and `includeContent`
- Calls the MCP server's `get_function_diff` tool
- Parses the MCP response into structured data
- Returns both parsed data and raw text

**Usage:**
```typescript
POST /api/mcp/function-diff
{
  "comparisonId": "uuid-here",
  "functionName": "my_function",
  "includeContent": true
}
```

#### 2. Updated: `nextjs-frontend/src/services/diffService.ts`

Added new method:
```typescript
async getFunctionDiffFromMCP(
  comparisonId: string,
  functionName: string,
  includeContent: boolean = true
): Promise<any>
```

#### 3. Updated: `nextjs-frontend/src/services/comparisonService.ts`

Added `comparisonId` field to `ComparisonResult` interface:
```typescript
export interface ComparisonResult {
  comparisonId?: string; // MCP comparison ID for getting function diffs
  // ... rest of fields
}
```

#### 4. Updated: `nextjs-frontend/src/components/graph/FunctionGraphViewer.tsx`

Modified `loadFunctionDiff` to:
1. Check if `data.comparisonId` is available
2. If yes, use `diffService.getFunctionDiffFromMCP()` 
3. If no, fall back to file-based diff
4. Convert MCP diff format to `FileDiff` format for display

Added helper function `parseUnifiedDiffToLines()` to convert unified diff format to line-by-line format.

## How It Works

### Flow

1. User clicks on a function node in the graph
2. `FunctionGraphViewer` calls `loadFunctionDiff(node)`
3. If `data.comparisonId` exists:
   - Calls `/api/mcp/function-diff` with comparison ID and function name
   - API route calls MCP server's `get_function_diff` tool
   - MCP server returns function diff with full content
   - Response is converted to `FileDiff` format
   - Diff is displayed in modal
4. If no comparison ID:
   - Falls back to file-based diff (old behavior)

### MCP Server Communication

```
Frontend → Next.js API Route → MCP Server
         ← Parsed Response   ← JSON-RPC Response
```

**MCP Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 123,
  "method": "tools/call",
  "params": {
    "name": "get_function_diff",
    "arguments": {
      "comparison_id": "uuid",
      "function_name": "my_function",
      "include_content": true
    }
  }
}
```

**MCP Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 123,
  "result": {
    "content": [{
      "type": "text",
      "text": "Function: my_function\nChange Type: modified\n..."
    }]
  }
}
```

## Remaining Work

### 1. Store Comparison ID When Creating Comparisons

The frontend needs to be updated to:
- Call the MCP server's `compare_locations` tool when creating comparisons
- Extract and store the comparison ID from the response
- Pass the comparison ID to the `ComparisonResult`

**Files to update:**
- Where comparisons are initiated (likely in a comparison page or service)
- Need to create an API route that calls MCP's `compare_locations`
- Store the comparison ID in the result

### 2. Create MCP Compare API Route

Create `nextjs-frontend/src/app/api/mcp/compare/route.ts`:
```typescript
POST /api/mcp/compare
{
  "sourcePath": "/path/to/source",
  "targetPath": "/path/to/target",
  "recursive": true
}

Response:
{
  "comparisonId": "uuid",
  "summary": { ... },
  "functions": [ ... ]
}
```

### 3. Update Comparison Flow

Current flow (Rust backend):
```
Frontend → Rust API → Parser → Comparison Result
```

New flow (MCP server):
```
Frontend → Next.js API → MCP Server → Comparison Result (with ID)
         → Store ID
         → Use ID for function diffs
```

## Benefits

1. ✅ **No file access issues** - MCP server already has the comparison data
2. ✅ **Complete function content** - No truncation (1MB limit)
3. ✅ **Consistent data** - Same source for comparison and diffs
4. ✅ **Better performance** - No need to re-parse files
5. ✅ **Unified diff format** - Proper diff with context

## Testing

### Test the MCP Function Diff API

```bash
# 1. Create a comparison
curl -s -X POST http://127.0.0.1:8011/message \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "compare_locations",
      "arguments": {
        "source_path": "/home/matteius/isp-was-better/driver",
        "target_path": "/home/matteius/isp-latest/driver"
      }
    }
  }' | jq -r '.result.content[0].text'

# Note the comparison ID from the output

# 2. Test the Next.js API route
curl -X POST http://localhost:3000/api/mcp/function-diff \
  -H "Content-Type: application/json" \
  -d '{
    "comparisonId": "YOUR_COMPARISON_ID",
    "functionName": "vic_framedone_irq_function",
    "includeContent": true
  }' | jq .
```

## Environment Variables

Make sure these are set in `.env.local`:

```bash
MCP_SERVER_URL=http://127.0.0.1:8011
```

## Next Steps

1. Create `/api/mcp/compare` route to initiate comparisons via MCP
2. Update comparison pages to use MCP for comparisons
3. Ensure comparison ID is passed through to graph viewer
4. Test end-to-end flow
5. Remove old file-based diff code once MCP integration is complete

## Status

- ✅ MCP function diff API created
- ✅ DiffService updated with MCP method
- ✅ FunctionGraphViewer updated to use MCP diffs
- ✅ ComparisonResult interface updated
- ⏳ Need to create MCP compare API
- ⏳ Need to update comparison initiation code
- ⏳ Need to test end-to-end flow

