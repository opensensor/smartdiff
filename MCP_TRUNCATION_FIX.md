# MCP Diff Results Truncation - Fixed

## Problem

The MCP server was returning truncated function content in `get_function_diff` responses. Function bodies were being cut off with "..." appended, making it impossible to see the full diff.

### Example of Truncated Output
```
--- Source Content ---
static void ispcore_irq_fs_work(struct work_struct *work)
{
    extern struct tx_isp_dev *ourISPdev;
    struct tx_isp_dev *isp_dev = ourISPdev;
    static int sensor_call_counter = 0;

    pr_info("*...

+++ Target Content +++
static void ispcore_irq_fs_work(struct work_struct *work)
{
    extern struct tx_isp_dev *ourISPdev;
    struct tx_isp_dev *isp_dev;

    /* CRITICAL: Take local reference to prevent race condition */...
```

## Root Cause

The issue was in the **AST parser configuration**:

1. **Location**: `crates/parser/src/ast_builder.rs` lines 172-178
2. **Default Limit**: The `ASTBuilderConfig::default()` sets `max_text_length: 200` bytes
3. **Truncation Logic**: When creating AST node metadata, if text exceeds `max_text_length`, it gets truncated with "..." appended:

```rust
original_text: if text.len() <= self.config.max_text_length {
    text.to_string()
} else {
    let truncated = self.truncate_text_safely(text, self.config.max_text_length);
    format!("{}...", truncated)
},
```

4. **MCP Server Usage**: The `ComparisonManager` was using `TreeSitterParser::new()` which uses the default config with the 200-byte limit.

## Solution

Modified `crates/mcp-server/src/comparison/manager.rs` to configure the parser with a much larger `max_text_length`:

```rust
impl ComparisonManager {
    pub fn new() -> Self {
        let config = SmartMatcherConfig {
            similarity_threshold: 0.7,
            enable_cross_file_matching: true,
            cross_file_penalty: 0.5,
        };

        // Configure parser with large max_text_length to avoid truncating function bodies
        // Default is 200 bytes which is too small for most functions
        let parser = TreeSitterParser::builder()
            .max_text_length(1_000_000) // 1MB should be enough for any reasonable function
            .include_comments(true)
            .extract_signatures(true)
            .build_symbol_table(true)
            .enable_optimization(true)
            .enable_analysis(false) // Disable analysis warnings for MCP usage
            .build()
            .expect("Failed to create parser");

        Self {
            contexts: Arc::new(RwLock::new(HashMap::new())),
            parser,
            smart_matcher: SmartMatcher::new(config),
        }
    }
}
```

### Key Changes
- **max_text_length**: Increased from 200 bytes to 1,000,000 bytes (1MB)
- **Rationale**: 1MB is more than sufficient for any reasonable function while still preventing memory issues from extremely large files

## How to Apply the Fix

### 1. Rebuild the MCP Server
The fix has already been applied and the server has been rebuilt:

```bash
cd crates/mcp-server
cargo build --release
```

The binary is located at: `target/release/smart-diff-mcp`

### 2. Restart the MCP Server

**If using stdio transport (Claude Desktop):**
- Restart Claude Desktop application
- The MCP server will be automatically restarted with the new binary

**If using SSE bridge:**
```bash
# Stop the current SSE bridge (Ctrl+C or kill the process)
# Then restart it:
cd crates/mcp-server
./start_sse_bridge.sh
```

**If running manually:**
```bash
# Stop the current server (Ctrl+C)
# Then restart:
./target/release/smart-diff-mcp
```

### 3. Verify the Fix

Test with a function that was previously truncated:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "get_function_diff",
    "arguments": {
      "comparison_id": "YOUR_COMPARISON_ID",
      "function_name": "ispcore_irq_fs_work",
      "include_content": true
    }
  }
}
```

You should now see the **complete function content** without "..." truncation.

## Impact

- ✅ **Function content**: Now returns complete function bodies
- ✅ **Unified diffs**: Now shows complete diffs without truncation
- ✅ **Memory usage**: Still reasonable (1MB limit per function)
- ✅ **Performance**: No noticeable impact
- ✅ **Backward compatibility**: No breaking changes to API

## Testing

The fix has been applied, tested, and verified successfully! ✅

### Test Results

A test was run with a function containing >200 bytes of content:

```bash
./test_truncation_fix.sh
```

**Result**: ✅ SUCCESS - No truncation detected!

The test confirmed:
- Complete function content is returned (no "..." truncation)
- Full unified diff is displayed correctly
- All lines are visible including complete statements
- Functions with >200 bytes work perfectly

### Example Output (After Fix)

```
--- Source Content ---
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

+++ Target Content +++
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
```

**Note**: All content is complete - no "..." truncation!

## Additional Notes

- The 1MB limit is configurable via the `TreeSitterParser::builder()` API
- If you encounter functions larger than 1MB, you can increase this limit further
- The default 200-byte limit remains in place for other uses of the parser to maintain backward compatibility
- Only the MCP server uses the increased limit

## Scripts Provided

1. **`restart_mcp_server.sh`** - Restarts the MCP server with the new binary
2. **`test_truncation_fix.sh`** - Tests the fix with a sample function

