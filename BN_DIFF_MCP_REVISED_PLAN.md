# Binary Ninja Diff MCP Integration - REVISED PLAN

## Executive Summary

**REVISED APPROACH**: Instead of building Binary Ninja integration directly into smartdiff, we will leverage the **existing Binary Ninja MCP server** at `/home/matteius/codediff/binary_ninja_mcp/` as a data source. This is a much cleaner architecture that respects the Personal License limitations and maintains proper separation of concerns.

## Why This Approach is Better

### 1. **License Compliance**
- ✅ Binary Ninja Personal License works with the existing MCP server
- ✅ No need for headless API access
- ✅ Binary Ninja runs in GUI mode with plugin
- ✅ No licensing issues

### 2. **Separation of Concerns**
- ✅ Binary Ninja MCP server handles all Binary Ninja interactions
- ✅ smartdiff MCP server handles comparison logic
- ✅ Clean API boundaries between systems
- ✅ Each system does what it does best

### 3. **Already Working**
- ✅ Binary Ninja MCP server is mature and tested
- ✅ Multi-binary support already implemented
- ✅ Decompilation, function listing, data access all working
- ✅ No need to reimplement Binary Ninja integration

### 4. **Simpler Implementation**
- ✅ smartdiff just needs to call Binary Ninja MCP tools
- ✅ No Binary Ninja API dependencies in smartdiff
- ✅ Easier to test and maintain
- ✅ Faster development

## Architecture Overview

### Current State

```
┌─────────────────────────────────────────┐
│  Binary Ninja (GUI)                     │
│  - Personal License                     │
│  - Binary Ninja MCP Plugin              │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│  Binary Ninja MCP Server                │
│  - HTTP Server (port 9009+)             │
│  - Multi-binary support                 │
│  - Tools: decompile, list_methods, etc. │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│  MCP Bridge (stdio)                     │
│  - Connects to Claude Desktop           │
│  - Routes to Binary Ninja servers       │
└─────────────────────────────────────────┘
```

### Proposed Integrated Architecture

```
┌─────────────────────────────────────────────────────────┐
│  AI Agent (Claude Desktop)                              │
└──────────────┬──────────────────────────────────────────┘
               │
         ┌─────┴─────┐
         │           │
         ▼           ▼
┌────────────────┐  ┌────────────────────────────────────┐
│  smartdiff MCP │  │  Binary Ninja MCP Bridge           │
│  Server        │  │  (existing)                        │
│  (stdio)       │  │  (stdio)                           │
└────────┬───────┘  └──────────┬─────────────────────────┘
         │                     │
         │                     ▼
         │          ┌────────────────────────────────────┐
         │          │  Binary Ninja MCP Server           │
         │          │  (HTTP, multi-binary)              │
         │          └──────────┬─────────────────────────┘
         │                     │
         │                     ▼
         │          ┌────────────────────────────────────┐
         │          │  Binary Ninja (GUI)                │
         │          │  - Binary A (port 9009)            │
         │          │  - Binary B (port 9010)            │
         │          └────────────────────────────────────┘
         │
         ▼
┌────────────────────────────────────────┐
│  smartdiff Diff Engine                 │
│  - Binary Function Matcher (NEW)       │
│  - Calls Binary Ninja MCP for data     │
│  - Performs comparison logic           │
└────────────────────────────────────────┘
```

## Integration Strategy

### Phase 1: Add Binary Ninja MCP Client to smartdiff

Create a client library in smartdiff that calls the Binary Ninja MCP server:

**New Module**: `crates/binary-ninja-client/`

**Purpose**: HTTP client to communicate with Binary Ninja MCP servers

**Key Components**:
1. `BinaryNinjaClient` - HTTP client for Binary Ninja MCP
2. `BinaryInfo` - Data structures for binary metadata
3. `FunctionInfo` - Data structures for function information
4. `BinaryLoader` - Load and manage binary connections

**No Binary Ninja dependencies required!**

### Phase 2: Extend smartdiff MCP with Binary Comparison Tools

Add new MCP tools to smartdiff that orchestrate binary comparison:

**New Tools**:
1. `compare_binaries_via_binja` - Compare two binaries using Binary Ninja MCP
2. `list_binary_function_matches` - List matched functions
3. `get_binary_function_diff` - Get detailed diff

**How it works**:
1. Agent calls smartdiff MCP tool
2. smartdiff calls Binary Ninja MCP to get function data
3. smartdiff performs comparison using diff engine
4. smartdiff returns results to agent

### Phase 3: Implement Binary Function Matching

Port the matching algorithms from rust_diff, but use Binary Ninja MCP as data source:

**Data Flow**:
```
smartdiff MCP Tool
    ↓
Binary Ninja MCP Client
    ↓ (HTTP request)
Binary Ninja MCP Server
    ↓
Binary Ninja (GUI)
    ↓ (function data)
Binary Ninja MCP Server
    ↓ (HTTP response)
Binary Ninja MCP Client
    ↓
smartdiff Diff Engine
    ↓ (comparison results)
smartdiff MCP Tool
    ↓
AI Agent
```

## Detailed Implementation Plan

### Phase 1: Binary Ninja MCP Client (Week 1)

#### 1.1 Create Client Crate

```bash
cd /home/matteius/codediff
mkdir -p crates/binary-ninja-client/src
```

**Cargo.toml**:
```toml
[package]
name = "smart-diff-binary-ninja-client"
version.workspace = true
edition.workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
anyhow.workspace = true
thiserror.workspace = true
tracing.workspace = true
reqwest = { version = "0.11", features = ["json"] }
tokio.workspace = true
```

#### 1.2 Implement HTTP Client

**Key Functions**:
- `list_binary_servers()` - Discover available Binary Ninja servers
- `get_binary_info(binary_id)` - Get binary metadata
- `list_functions(binary_id)` - List all functions
- `get_function_info(binary_id, function_name)` - Get function details
- `decompile_function(binary_id, function_name)` - Get decompiled code

#### 1.3 Data Structures

Map Binary Ninja MCP responses to smartdiff types:

```rust
pub struct BinaryNinjaServer {
    pub binary_id: String,
    pub url: String,
    pub port: u16,
    pub filename: String,
}

pub struct BinaryInfo {
    pub binary_id: String,
    pub filename: String,
    pub architecture: String,
    pub platform: String,
    pub function_count: usize,
}

pub struct FunctionInfo {
    pub name: String,
    pub address: String,
    pub raw_name: Option<String>,
    pub decompiled_code: Option<String>,
}
```

### Phase 2: Binary Comparison Tools (Week 2-3)

#### 2.1 Extend MCP Server

Add to `crates/mcp-server/src/tools/`:

**binary_comparison_tools.rs**:
```rust
pub async fn compare_binaries_via_binja(
    binary_a_id: String,
    binary_b_id: String,
    options: ComparisonOptions,
) -> Result<ComparisonResult> {
    // 1. Connect to Binary Ninja MCP
    let bn_client = BinaryNinjaClient::new();
    
    // 2. Get function lists from both binaries
    let functions_a = bn_client.list_functions(&binary_a_id).await?;
    let functions_b = bn_client.list_functions(&binary_b_id).await?;
    
    // 3. Extract function details (names, addresses, decompiled code)
    let detailed_a = extract_function_details(&bn_client, &binary_a_id, &functions_a).await?;
    let detailed_b = extract_function_details(&bn_client, &binary_b_id, &functions_b).await?;
    
    // 4. Perform matching using diff engine
    let matcher = BinaryFunctionMatcher::new();
    let matches = matcher.match_functions(&detailed_a, &detailed_b)?;
    
    // 5. Store results in comparison manager
    let comparison_id = comparison_manager.store_binary_comparison(matches);
    
    Ok(ComparisonResult {
        comparison_id,
        matched_count: matches.len(),
        // ... other fields
    })
}
```

#### 2.2 MCP Tool Definitions

**Tool 1: compare_binaries_via_binja**
```json
{
  "name": "compare_binaries_via_binja",
  "description": "Compare two binaries loaded in Binary Ninja",
  "inputSchema": {
    "type": "object",
    "properties": {
      "binary_a_id": {
        "type": "string",
        "description": "Binary Ninja server ID for first binary (e.g., 'port_9009')"
      },
      "binary_b_id": {
        "type": "string",
        "description": "Binary Ninja server ID for second binary (e.g., 'port_9010')"
      },
      "options": {
        "type": "object",
        "properties": {
          "use_decompiled_code": {
            "type": "boolean",
            "description": "Compare decompiled code in addition to function names"
          },
          "similarity_threshold": {
            "type": "number",
            "description": "Minimum similarity score (0.0-1.0)"
          }
        }
      }
    },
    "required": ["binary_a_id", "binary_b_id"]
  }
}
```

**Tool 2: list_binja_servers**
```json
{
  "name": "list_binja_servers",
  "description": "List available Binary Ninja MCP servers",
  "inputSchema": {
    "type": "object",
    "properties": {}
  }
}
```

### Phase 3: Binary Function Matching (Week 3-4)

#### 3.1 Matching Strategies

**Strategy 1: Name-Based Matching**
- Exact name match
- Fuzzy name match (Levenshtein distance)
- Demangled name comparison

**Strategy 2: Decompiled Code Similarity**
- Use existing tree edit distance on decompiled code
- Treat decompiled code as pseudo-source
- Apply AST diff algorithms

**Strategy 3: Hybrid Matching**
- Combine name similarity and code similarity
- Weighted scoring
- Confidence calculation

#### 3.2 Implementation

**crates/diff-engine/src/binary_matcher.rs**:
```rust
pub struct BinaryFunctionMatcher {
    name_weight: f64,
    code_weight: f64,
}

impl BinaryFunctionMatcher {
    pub fn match_functions(
        &self,
        functions_a: &[FunctionInfo],
        functions_b: &[FunctionInfo],
    ) -> Result<Vec<FunctionMatch>> {
        let mut matches = Vec::new();
        
        // Phase 1: Exact name matching
        let exact_matches = self.exact_name_matching(functions_a, functions_b)?;
        matches.extend(exact_matches);
        
        // Phase 2: Fuzzy name matching
        let fuzzy_matches = self.fuzzy_name_matching(functions_a, functions_b)?;
        matches.extend(fuzzy_matches);
        
        // Phase 3: Code similarity matching (if decompiled code available)
        if self.has_decompiled_code(functions_a) && self.has_decompiled_code(functions_b) {
            let code_matches = self.code_similarity_matching(functions_a, functions_b)?;
            matches.extend(code_matches);
        }
        
        Ok(matches)
    }
    
    fn code_similarity_matching(
        &self,
        functions_a: &[FunctionInfo],
        functions_b: &[FunctionInfo],
    ) -> Result<Vec<FunctionMatch>> {
        // Use existing tree edit distance on decompiled code
        // Treat decompiled C code as source code
        // Parse with tree-sitter C parser
        // Apply AST diff algorithms
        
        // This reuses all the existing smartdiff infrastructure!
        todo!()
    }
}
```

## Usage Example

### Setup

1. **Start Binary Ninja with binaries**:
   ```
   - Open Binary Ninja
   - Load binary_v1.exe
   - Plugins > MCP Server > Start Server for This Binary (port 9009)
   - Open another Binary Ninja window
   - Load binary_v2.exe
   - Plugins > MCP Server > Start Server for This Binary (port 9010)
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
   cargo run --bin smart-diff-mcp
   ```

4. **Configure Claude Desktop**:
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

### Agent Workflow

```
User: "Compare the two binaries I have loaded in Binary Ninja"

Agent:
1. Calls list_binja_servers() to see available binaries
2. Identifies binary_a_id="port_9009" and binary_b_id="port_9010"
3. Calls compare_binaries_via_binja(binary_a_id, binary_b_id)
4. smartdiff fetches function data from Binary Ninja MCP
5. smartdiff performs comparison
6. Returns results to agent
7. Agent presents findings to user

User: "Show me functions that changed significantly"

Agent:
1. Calls list_binary_function_matches(comparison_id, sort_by="similarity_asc")
2. Filters for low similarity scores
3. Presents list to user

User: "Show me the diff for function process_data"

Agent:
1. Calls get_binary_function_diff(comparison_id, "process_data")
2. Gets decompiled code from both versions
3. Shows side-by-side comparison
```

## Advantages of This Approach

1. **No Binary Ninja Dependencies in smartdiff**
   - Clean separation
   - Easier to build and test
   - No licensing complications

2. **Reuses Existing Infrastructure**
   - Binary Ninja MCP server is mature
   - smartdiff diff engine is proven
   - Both systems do what they do best

3. **Flexible**
   - Can compare binaries from different Binary Ninja instances
   - Can mix source and binary comparison
   - Easy to extend

4. **Maintainable**
   - Clear API boundaries
   - Each component is independently testable
   - Easier to debug

5. **Personal License Compatible**
   - Binary Ninja runs in GUI mode
   - No headless API needed
   - Fully supported configuration

## Implementation Timeline

- **Week 1**: Binary Ninja MCP client library
- **Week 2**: MCP tools for binary comparison
- **Week 3**: Binary function matching algorithms
- **Week 4**: Testing and documentation

**Total**: 4 weeks (vs 7-8 weeks for direct integration)

## Next Steps

1. Create `crates/binary-ninja-client/` skeleton
2. Implement HTTP client for Binary Ninja MCP
3. Add binary comparison tools to smartdiff MCP
4. Implement matching algorithms
5. Test with real binaries
6. Document usage

This approach is **simpler, faster, and more maintainable** than direct Binary Ninja integration!

