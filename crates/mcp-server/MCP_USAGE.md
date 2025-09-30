# MCP Server Usage Guide

## Quick Start

### 1. Build the Server

```bash
cargo build --release -p smart-diff-mcp-server
```

### 2. Configure Your MCP Client

For **Claude Desktop**, add to your configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "smart-diff": {
      "command": "/absolute/path/to/codediff/target/release/smart-diff-mcp",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### 3. Restart Claude Desktop

After adding the configuration, restart Claude Desktop to load the MCP server.

## Using the MCP Server with AI Agents

Once configured, you can ask Claude (or other MCP-compatible AI agents) to use the code comparison tools:

### Example Prompts

**Compare two directories:**
```
Please use the smart-diff MCP server to compare the code in /path/to/old/version 
with /path/to/new/version and tell me what changed.
```

**List changed functions:**
```
Show me all the functions that changed, sorted by how much they changed.
```

**Get details on a specific function:**
```
Can you show me the detailed diff for the process_data function?
```

**Analyze refactoring:**
```
Compare the old and new versions and identify any refactoring patterns 
like function renames or moves.
```

## Tool Reference

### compare_locations

Initiates a comparison between two code locations.

**Example request:**
```json
{
  "name": "compare_locations",
  "arguments": {
    "source_path": "/path/to/old/code",
    "target_path": "/path/to/new/code",
    "recursive": true,
    "file_patterns": ["*.rs", "*.py"],
    "ignore_patterns": ["target/", "*.pyc"]
  }
}
```

**Returns:**
- Comparison ID (UUID)
- Summary statistics (added, deleted, modified, renamed, moved functions)

### list_changed_functions

Lists all changed functions sorted by change magnitude.

**Example request:**
```json
{
  "name": "list_changed_functions",
  "arguments": {
    "comparison_id": "550e8400-e29b-41d4-a716-446655440000",
    "limit": 50,
    "change_types": ["modified", "added"],
    "min_magnitude": 0.3
  }
}
```

**Returns:**
- List of functions with:
  - Function name
  - Change type (added/deleted/modified/renamed/moved)
  - Change magnitude (0.0 to 1.0)
  - Similarity score
  - File locations and line numbers
  - Summary description

### get_function_diff

Gets detailed diff for a specific function.

**Example request:**
```json
{
  "name": "get_function_diff",
  "arguments": {
    "comparison_id": "550e8400-e29b-41d4-a716-446655440000",
    "function_name": "process_data",
    "include_content": true
  }
}
```

**Returns:**
- Function name
- Change type and magnitude
- Source and target file paths
- Line numbers
- Signatures
- Detailed change summary

### get_comparison_summary

Gets summary statistics for a comparison.

**Example request:**
```json
{
  "name": "get_comparison_summary",
  "arguments": {
    "comparison_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

**Returns:**
- Total functions analyzed
- Counts by change type
- Source and target paths
- Timestamp

## Resource URIs

The server also exposes comparison results as MCP resources:

- `codediff://comparison/{id}/summary` - JSON summary
- `codediff://comparison/{id}/functions` - JSON list of all changes
- `codediff://comparison/{id}/function/{name}` - JSON diff for specific function

## Workflow Examples

### Example 1: Code Review

```
Agent: I'll help you review the changes between these two versions.

1. First, let me compare the directories...
   [Uses compare_locations]

2. I found 47 functions changed. Here are the top 10 most significant changes:
   [Uses list_changed_functions with limit=10]

3. Let me examine the process_data function in detail since it has the 
   highest change magnitude...
   [Uses get_function_diff]

4. This function was significantly refactored. The changes include:
   - Added error handling
   - Extracted validation logic
   - Improved performance with caching
```

### Example 2: Refactoring Analysis

```
Agent: Let me analyze the refactoring patterns in your codebase.

1. Comparing old and new versions...
   [Uses compare_locations]

2. I found several refactoring patterns:
   - 5 functions were renamed
   - 3 functions were moved to different files
   - 2 functions were split into smaller functions
   [Uses list_changed_functions with different filters]

3. Here's a detailed breakdown of each refactoring...
   [Uses get_function_diff for each significant change]
```

### Example 3: Migration Impact Analysis

```
Agent: I'll analyze the impact of migrating from the old API to the new one.

1. Comparing the two codebases...
   [Uses compare_locations]

2. Summary of changes:
   - 23 functions modified (API signature changes)
   - 8 functions added (new features)
   - 5 functions deleted (deprecated)
   [Uses get_comparison_summary]

3. Functions requiring immediate attention (high change magnitude):
   [Uses list_changed_functions with min_magnitude=0.7]
```

## Troubleshooting

### Server not appearing in Claude

1. Check the configuration file path is correct
2. Ensure the binary path is absolute
3. Check logs: `tail -f ~/Library/Logs/Claude/mcp*.log` (macOS)
4. Verify the binary is executable: `chmod +x target/release/smart-diff-mcp`

### Comparison fails

1. Check paths exist and are readable
2. Ensure supported file types (`.rs`, `.py`, `.js`, `.java`, `.c`, `.cpp`)
3. Check RUST_LOG output for detailed errors

### Performance issues

1. Use `file_patterns` to limit scope
2. Add `ignore_patterns` for build artifacts
3. Set `recursive: false` for single files
4. Reduce `limit` in list_changed_functions

## Advanced Usage

### Custom File Patterns

```json
{
  "file_patterns": ["src/**/*.rs", "lib/**/*.rs"],
  "ignore_patterns": ["**/tests/**", "**/target/**"]
}
```

### Filtering by Change Type

```json
{
  "change_types": ["modified"],
  "min_magnitude": 0.5
}
```

This shows only significantly modified functions, excluding additions/deletions.

### Incremental Analysis

1. Run initial comparison
2. Save comparison_id
3. Query different aspects without re-parsing:
   - List all changes
   - Filter by type
   - Get individual diffs
   - Access via resources

## Integration with Other Tools

The MCP server can be used alongside:

- **Git**: Compare commits or branches
- **CI/CD**: Automated code review
- **IDEs**: Via MCP client plugins
- **Documentation**: Generate change logs

## Performance Characteristics

- **Parsing**: ~1000 LOC/second
- **Comparison**: ~100 functions/second
- **Memory**: ~10MB per 1000 functions
- **Concurrent comparisons**: Supported via unique IDs

## Security Considerations

- Server only reads files (no write access)
- Paths are validated before access
- No network access required
- Runs in user context (same permissions as client)

