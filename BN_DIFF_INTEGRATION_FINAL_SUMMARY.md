# Binary Ninja Diff Integration - Final Summary & Recommendation

## Executive Decision

**RECOMMENDED APPROACH**: Leverage the existing Binary Ninja MCP server as a data source rather than building direct Binary Ninja integration into smartdiff.

## Why This is the Right Approach

### 1. License Compliance ✅
- **Your Personal License works perfectly** with the existing Binary Ninja MCP server
- No need for headless API access (which requires Commercial license)
- Binary Ninja runs in GUI mode with the plugin
- **Zero licensing issues**

### 2. Architecture Benefits ✅
- **Clean separation of concerns**: Binary Ninja MCP handles Binary Ninja, smartdiff handles comparison
- **Reuses proven infrastructure**: Both systems are already working
- **No tight coupling**: HTTP API boundary between systems
- **Easier to test and maintain**: Each component is independent

### 3. Development Speed ✅
- **4 weeks vs 7-8 weeks**: Much faster implementation
- **No Binary Ninja API learning curve**: Just HTTP client
- **Reuses existing diff engine**: Tree edit distance, Hungarian matching, etc.
- **Less code to write and maintain**

### 4. Flexibility ✅
- Can compare binaries from different Binary Ninja instances
- Can mix source code and binary comparison
- Easy to extend with new features
- Works with multi-binary setup out of the box

## What You Already Have

### Binary Ninja MCP Server (`/home/matteius/codediff/binary_ninja_mcp/`)

**Status**: ✅ Fully functional, tested, and documented

**Features**:
- Multi-binary support (multiple binaries on different ports)
- Function listing and searching
- Decompilation (HLIL, MLIL)
- Data inspection
- Symbol renaming
- HTTP API on ports 9009+
- MCP bridge for Claude Desktop

**Available Tools**:
- `list_binary_servers()` - Discover available binaries
- `list_entities(kind="methods", binary_id)` - List functions
- `decompile_function(name, binary_id)` - Get decompiled code
- `get_binary_status(binary_id)` - Get binary info
- `search_functions_by_name(query, binary_id)` - Search functions
- And many more...

### smartdiff MCP Server (`/home/matteius/codediff/crates/mcp-server/`)

**Status**: ✅ Fully functional, tested, and documented

**Features**:
- Source code comparison via tree-sitter
- Function-level granularity
- Tree edit distance algorithms
- Hungarian matching for optimal pairing
- Change classification
- Stateful comparisons with unique IDs
- MCP protocol compliance

**Available Tools**:
- `compare_locations` - Compare files/directories
- `list_changed_functions` - List functions by change magnitude
- `get_function_diff` - Get detailed function diff
- `get_comparison_summary` - Get comparison overview

### rust_diff (`/home/matteius/codediff/rust_diff/`)

**Status**: ✅ Reference implementation with proven algorithms

**Key Algorithms to Port**:
- Multi-phase function matching (exact hash, name, structural, heuristic)
- Binary-specific similarity metrics (CFG, basic blocks, instructions, edges)
- Confidence scoring with multiple factors
- Parallel processing with rayon

## What Needs to be Built

### 1. Binary Ninja MCP Client Library (Week 1)

**New Crate**: `crates/binary-ninja-client/`

**Purpose**: HTTP client to communicate with Binary Ninja MCP servers

**Key Components**:
```rust
// HTTP client
pub struct BinaryNinjaClient {
    base_url: String,
    client: reqwest::Client,
}

// Data structures
pub struct BinaryNinjaServer {
    pub binary_id: String,
    pub url: String,
    pub port: u16,
    pub filename: String,
}

pub struct FunctionInfo {
    pub name: String,
    pub address: String,
    pub decompiled_code: Option<String>,
}

// Client methods
impl BinaryNinjaClient {
    pub async fn discover_servers() -> Result<Vec<BinaryNinjaServer>>;
    pub async fn list_functions(binary_id: &str) -> Result<Vec<FunctionInfo>>;
    pub async fn decompile_function(binary_id: &str, name: &str) -> Result<String>;
}
```

**Dependencies**: `reqwest`, `serde_json`, `tokio` (already in workspace)

**No Binary Ninja dependencies required!**

### 2. Binary Comparison MCP Tools (Week 2)

**Extend**: `crates/mcp-server/src/tools/`

**New Tools**:

#### `list_binja_servers`
List available Binary Ninja MCP servers
```json
{
  "servers": [
    {
      "binary_id": "port_9009",
      "filename": "malware_v1.exe",
      "port": 9009,
      "function_count": 150
    },
    {
      "binary_id": "port_9010",
      "filename": "malware_v2.exe",
      "port": 9010,
      "function_count": 148
    }
  ]
}
```

#### `compare_binaries_via_binja`
Compare two binaries loaded in Binary Ninja
```json
{
  "comparison_id": "uuid",
  "binary_a": "malware_v1.exe",
  "binary_b": "malware_v2.exe",
  "matched_functions": 142,
  "similarity_score": 0.87,
  "analysis_time": 3.2
}
```

#### `list_binary_function_matches`
List matched functions sorted by similarity
```json
{
  "matches": [
    {
      "function_a": "process_data",
      "function_b": "process_data_v2",
      "similarity": 0.82,
      "match_type": "name_and_code"
    }
  ],
  "total": 142
}
```

#### `get_binary_function_diff`
Get detailed diff for a specific function match
```json
{
  "function_a": {
    "name": "process_data",
    "address": "0x1800",
    "decompiled_code": "..."
  },
  "function_b": {
    "name": "process_data_v2",
    "address": "0x1850",
    "decompiled_code": "..."
  },
  "diff": {
    "similarity": 0.82,
    "changes": [...]
  }
}
```

### 3. Binary Function Matching (Week 3)

**Extend**: `crates/diff-engine/src/`

**New Module**: `binary_matcher.rs`

**Matching Strategies**:

1. **Exact Name Matching**
   - Fast O(n) lookup via HashMap
   - High confidence matches

2. **Fuzzy Name Matching**
   - Levenshtein distance
   - Demangled name comparison
   - Medium confidence

3. **Decompiled Code Similarity**
   - **Reuse existing tree edit distance!**
   - Parse decompiled C code with tree-sitter
   - Apply AST diff algorithms
   - This is the key insight: treat decompiled code as source code

4. **Hybrid Scoring**
   - Combine name similarity and code similarity
   - Weighted scoring (name: 30%, code: 70%)
   - Confidence calculation

**Implementation**:
```rust
pub struct BinaryFunctionMatcher {
    name_weight: f64,
    code_weight: f64,
    parser: TreeSitterParser, // Reuse existing parser!
}

impl BinaryFunctionMatcher {
    pub async fn match_functions(
        &self,
        bn_client: &BinaryNinjaClient,
        binary_a_id: &str,
        binary_b_id: &str,
    ) -> Result<Vec<FunctionMatch>> {
        // 1. Get function lists from Binary Ninja MCP
        let functions_a = bn_client.list_functions(binary_a_id).await?;
        let functions_b = bn_client.list_functions(binary_b_id).await?;
        
        // 2. Exact name matching
        let mut matches = self.exact_name_matching(&functions_a, &functions_b)?;
        
        // 3. For unmatched functions, try code similarity
        let unmatched_a = self.get_unmatched(&functions_a, &matches);
        let unmatched_b = self.get_unmatched(&functions_b, &matches);
        
        for func_a in unmatched_a {
            // Get decompiled code
            let code_a = bn_client.decompile_function(binary_a_id, &func_a.name).await?;
            
            for func_b in &unmatched_b {
                let code_b = bn_client.decompile_function(binary_b_id, &func_b.name).await?;
                
                // Use existing tree edit distance!
                let similarity = self.compare_code(&code_a, &code_b)?;
                
                if similarity > threshold {
                    matches.push(FunctionMatch {
                        function_a: func_a.clone(),
                        function_b: func_b.clone(),
                        similarity,
                        match_type: MatchType::CodeSimilarity,
                    });
                }
            }
        }
        
        Ok(matches)
    }
    
    fn compare_code(&self, code_a: &str, code_b: &str) -> Result<f64> {
        // Parse decompiled C code with tree-sitter
        let ast_a = self.parser.parse(code_a, Language::C)?;
        let ast_b = self.parser.parse(code_b, Language::C)?;
        
        // Use existing tree edit distance algorithm!
        let distance = self.tree_edit_distance.compute(&ast_a, &ast_b)?;
        
        // Convert distance to similarity score
        let similarity = 1.0 - (distance as f64 / max_distance);
        
        Ok(similarity)
    }
}
```

### 4. Testing & Documentation (Week 4)

**Integration Tests**:
- Test with real binaries
- Verify Binary Ninja MCP connectivity
- Test matching accuracy
- Performance benchmarking

**Documentation**:
- User guide for binary comparison
- API documentation
- Example workflows
- Troubleshooting guide

## Complete Workflow Example

### Setup

1. **Load binaries in Binary Ninja**:
   ```
   Binary Ninja Window 1:
   - Open malware_v1.exe
   - Plugins > MCP Server > Start Server for This Binary
   - Server starts on port 9009
   
   Binary Ninja Window 2:
   - Open malware_v2.exe
   - Plugins > MCP Server > Start Server for This Binary
   - Server starts on port 9010
   ```

2. **Start Binary Ninja MCP Bridge**:
   ```bash
   cd /home/matteius/codediff/binary_ninja_mcp
   source .venv/bin/activate
   python bridge/bn_mcp_bridge_multi_http.py
   ```

3. **Start smartdiff MCP Server**:
   ```bash
   cd /home/matteius/codediff
   cargo run --release --bin smart-diff-mcp
   ```

4. **Configure Claude Desktop** (`~/Library/Application Support/Claude/claude_desktop_config.json`):
   ```json
   {
     "mcpServers": {
       "binary_ninja": {
         "command": "/path/to/binary_ninja_mcp/.venv/bin/python",
         "args": ["/path/to/binary_ninja_mcp/bridge/bn_mcp_bridge_multi_http.py"]
       },
       "smartdiff": {
         "command": "/path/to/codediff/target/release/smart-diff-mcp",
         "args": []
       }
     }
   }
   ```

### Usage

**User**: "I have two versions of a malware binary loaded in Binary Ninja. Can you compare them and tell me what changed?"

**Agent**:
1. Calls `list_binja_servers()` → Sees malware_v1.exe (port_9009) and malware_v2.exe (port_9010)
2. Calls `compare_binaries_via_binja("port_9009", "port_9010")`
3. smartdiff:
   - Fetches function lists from both binaries via Binary Ninja MCP
   - Performs matching (name + code similarity)
   - Stores results with comparison_id
4. Returns summary to agent
5. Agent: "I found 142 matched functions with 87% overall similarity. 6 functions were removed and 4 new functions were added."

**User**: "Show me the functions that changed the most"

**Agent**:
1. Calls `list_binary_function_matches(comparison_id, sort_by="similarity_asc", limit=10)`
2. Gets list of functions with lowest similarity scores
3. Presents: "Here are the 10 most changed functions: process_data (82% similar), encrypt_payload (75% similar), ..."

**User**: "Show me the diff for process_data"

**Agent**:
1. Calls `get_binary_function_diff(comparison_id, "process_data")`
2. Gets decompiled code for both versions
3. Shows side-by-side comparison with highlighted changes

## Implementation Checklist

### Week 1: Binary Ninja MCP Client
- [ ] Create `crates/binary-ninja-client/` crate
- [ ] Implement `BinaryNinjaClient` HTTP client
- [ ] Implement server discovery
- [ ] Implement function listing
- [ ] Implement decompilation fetching
- [ ] Add error handling
- [ ] Write unit tests

### Week 2: MCP Tools
- [ ] Add `list_binja_servers` tool
- [ ] Add `compare_binaries_via_binja` tool
- [ ] Add `list_binary_function_matches` tool
- [ ] Add `get_binary_function_diff` tool
- [ ] Update MCP server documentation
- [ ] Write integration tests

### Week 3: Binary Matching
- [ ] Create `binary_matcher.rs` module
- [ ] Implement exact name matching
- [ ] Implement fuzzy name matching
- [ ] Implement code similarity matching (reuse tree edit distance)
- [ ] Implement hybrid scoring
- [ ] Add confidence calculation
- [ ] Write comprehensive tests

### Week 4: Testing & Documentation
- [ ] End-to-end testing with real binaries
- [ ] Performance benchmarking
- [ ] User documentation
- [ ] API documentation
- [ ] Example workflows
- [ ] Troubleshooting guide

## Success Criteria

1. ✅ AI agents can compare binaries loaded in Binary Ninja
2. ✅ Matching accuracy ≥ 85% for similar binaries
3. ✅ Performance < 10 seconds for typical binaries (100-200 functions)
4. ✅ Works with Personal License (no licensing issues)
5. ✅ Clean architecture with proper separation of concerns
6. ✅ Comprehensive documentation
7. ✅ Integration tests pass

## Conclusion

This approach is **superior** to direct Binary Ninja integration because:

1. **License compliant**: Works with Personal License
2. **Faster to implement**: 4 weeks vs 7-8 weeks
3. **Cleaner architecture**: Proper separation of concerns
4. **Reuses existing infrastructure**: Both Binary Ninja MCP and smartdiff
5. **More maintainable**: Clear API boundaries
6. **More flexible**: Can extend easily

**Recommendation**: Proceed with this approach immediately.

## Next Steps

1. **Review and approve** this plan
2. **Create skeleton** for `crates/binary-ninja-client/`
3. **Implement HTTP client** for Binary Ninja MCP
4. **Add MCP tools** to smartdiff
5. **Implement matching** algorithms
6. **Test** with real binaries
7. **Document** usage

**Estimated Timeline**: 4 weeks to full implementation

**Priority**: High - Enables unique binary analysis capabilities for AI agents while respecting license constraints

