# Binary Ninja Diff MCP Integration Plan

## Executive Summary

This document outlines the plan to port the Binary Ninja (BN) Diff tooling from the `rust_diff/` repository into the smartdiff architecture via an MCP (Model Context Protocol) layer. The goal is to enable AI agents to compare binary functions using Binary Ninja's analysis capabilities while maintaining architecture compliance with smartdiff's design principles.

## Current State Analysis

### rust_diff Repository (Alternative Implementation)

**Location**: `/home/matteius/codediff/rust_diff/`

**Architecture**:
- **Rust Core** (`src/lib.rs`): Binary diffing engine with C FFI exports
- **Python Plugin** (`__init__.py`): Binary Ninja plugin integration
- **Binary Ninja Integration**: Direct BinaryView API usage for binary analysis
- **Output Formats**: JSON, CSV, SQLite, HTML reports
- **GUI**: Optional Qt-based results viewer

**Key Features**:
1. **Multi-phase Function Matching**:
   - Exact hash matching (CFG + call graph hashes)
   - Name-based matching
   - Structural similarity matching
   - Heuristic matching with parallel processing

2. **Binary-Specific Analysis**:
   - Basic block extraction and analysis
   - Instruction-level comparison
   - Control Flow Graph (CFG) hashing
   - Call graph analysis
   - Cyclomatic complexity calculation

3. **Similarity Metrics**:
   - CFG similarity (50% weight)
   - Basic block similarity (15% weight)
   - Instruction similarity (10% weight)
   - Edge similarity (25% weight)
   - Name similarity
   - Call similarity

4. **Confidence Scoring**:
   - Size-based confidence boost
   - Complexity-based confidence boost
   - Basic block count boost
   - Name match boost

### smartdiff Current MCP Implementation

**Location**: `/home/matteius/codediff/crates/mcp-server/`

**Architecture**:
- **MCP Server** (`src/server.rs`): JSON-RPC 2.0 over stdio
- **Comparison Manager** (`src/comparison/`): Stateful comparison lifecycle
- **Tool Handler** (`src/tools/`): MCP tools for code analysis
- **Resource Handler** (`src/resources/`): Structured data access
- **Source Code Focus**: Tree-sitter based AST parsing for source code

**Current MCP Tools**:
1. `compare_locations` - Compare files/directories
2. `list_changed_functions` - List functions by change magnitude
3. `get_function_diff` - Get detailed function diff
4. `get_comparison_summary` - Get comparison overview

**Supported Languages**: Rust, Python, JavaScript, Java, C/C++ (source code only)

## Gap Analysis

### What rust_diff Has That smartdiff MCP Lacks

1. **Binary Analysis Capabilities**:
   - Direct binary file parsing (via Binary Ninja)
   - Assembly instruction analysis
   - Binary-specific hashing (CFG, call graph)
   - Basic block level granularity
   - Cross-architecture support

2. **Binary Ninja Integration**:
   - BinaryView API access
   - BNDB file format support
   - Binary Ninja's advanced analysis features
   - Decompilation integration potential

3. **Binary-Specific Matching Algorithms**:
   - Hash-based exact matching for binaries
   - Structural matching optimized for compiled code
   - Instruction mnemonic hashing

### What smartdiff MCP Has That rust_diff Lacks

1. **MCP Protocol Integration**:
   - Standardized AI agent interface
   - JSON-RPC 2.0 communication
   - Resource-based data access
   - Stateful comparison management

2. **Source Code Analysis**:
   - AST-based comparison
   - Tree edit distance algorithms
   - Semantic analysis
   - Refactoring detection

3. **Multi-file/Directory Support**:
   - Recursive directory comparison
   - Cross-file tracking
   - File pattern filtering

## Integration Strategy

### Architecture-Compliant Approach

Following smartdiff's architecture principles:

1. **Rust Backend Layer** (New: `crates/binary-ninja-bridge/`)
   - Binary Ninja API integration
   - Binary function extraction
   - Binary-specific feature computation
   - Abstraction layer over BinaryView

2. **MCP Server Extension** (Extend: `crates/mcp-server/`)
   - New MCP tools for binary comparison
   - Binary-specific resources
   - Unified interface for both source and binary analysis

3. **Comparison Engine Integration** (Extend: `crates/diff-engine/`)
   - Binary function matching algorithms
   - Hybrid source/binary comparison support
   - Unified similarity scoring

### Proposed MCP Tools for Binary Analysis

#### 1. `compare_binaries`
**Purpose**: Compare two binary files using Binary Ninja analysis

**Input**:
```json
{
  "binary_a": "/path/to/binary1.bndb",
  "binary_b": "/path/to/binary2.bndb",
  "options": {
    "similarity_threshold": 0.6,
    "confidence_threshold": 0.5,
    "match_algorithms": ["exact_hash", "name", "structural", "heuristic"],
    "include_unmatched": true
  }
}
```

**Output**:
```json
{
  "comparison_id": "uuid",
  "binary_a_name": "binary1.bndb",
  "binary_b_name": "binary2.bndb",
  "total_functions_a": 150,
  "total_functions_b": 148,
  "matched_count": 142,
  "similarity_score": 0.87,
  "analysis_time": 2.3
}
```

#### 2. `list_binary_function_matches`
**Purpose**: List matched functions sorted by similarity/change magnitude

**Input**:
```json
{
  "comparison_id": "uuid",
  "sort_by": "similarity_desc",
  "filter": {
    "min_similarity": 0.5,
    "max_similarity": 0.95,
    "match_type": ["structural", "heuristic"]
  },
  "limit": 50,
  "offset": 0
}
```

**Output**:
```json
{
  "matches": [
    {
      "function_a": {
        "name": "process_data",
        "address": "0x1800",
        "size": 300,
        "basic_blocks": 6,
        "complexity": 8
      },
      "function_b": {
        "name": "process_data_v2",
        "address": "0x1850",
        "size": 320,
        "basic_blocks": 7,
        "complexity": 9
      },
      "similarity": 0.82,
      "confidence": 0.89,
      "match_type": "structural",
      "details": {
        "cfg_similarity": 0.85,
        "bb_similarity": 0.86,
        "instruction_similarity": 0.78,
        "edge_similarity": 0.88
      }
    }
  ],
  "total": 142,
  "has_more": true
}
```

#### 3. `get_binary_function_diff`
**Purpose**: Get detailed diff for a specific binary function match

**Input**:
```json
{
  "comparison_id": "uuid",
  "function_a_address": "0x1800",
  "function_b_address": "0x1850",
  "include_disassembly": true,
  "include_cfg": true,
  "include_decompilation": false
}
```

**Output**:
```json
{
  "function_a": { /* detailed function info */ },
  "function_b": { /* detailed function info */ },
  "diff": {
    "basic_blocks_added": 1,
    "basic_blocks_removed": 0,
    "basic_blocks_modified": 3,
    "instructions_added": 12,
    "instructions_removed": 5,
    "cfg_changes": [ /* CFG edge changes */ ],
    "disassembly_diff": "...",
    "decompilation_diff": null
  },
  "similarity_breakdown": { /* detailed metrics */ }
}
```

#### 4. `load_binary_in_binja`
**Purpose**: Load a binary file in Binary Ninja for analysis

**Input**:
```json
{
  "binary_path": "/path/to/binary.exe",
  "analysis_options": {
    "auto_analyze": true,
    "load_debug_info": true,
    "architecture": "auto"
  }
}
```

**Output**:
```json
{
  "binary_id": "uuid",
  "file_path": "/path/to/binary.exe",
  "architecture": "x86_64",
  "platform": "linux",
  "function_count": 150,
  "analysis_complete": true
}
```

#### 5. `list_binary_functions`
**Purpose**: List all functions in a loaded binary

**Input**:
```json
{
  "binary_id": "uuid",
  "filter": {
    "min_size": 10,
    "name_pattern": "process_*"
  },
  "sort_by": "size_desc",
  "limit": 100,
  "offset": 0
}
```

**Output**:
```json
{
  "functions": [
    {
      "name": "process_data",
      "address": "0x1800",
      "size": 300,
      "basic_blocks": 6,
      "complexity": 8,
      "call_count": 2
    }
  ],
  "total": 150,
  "has_more": true
}
```

## Implementation Phases

### Phase 1: Binary Ninja Bridge Crate (Week 1-2)

**Goal**: Create abstraction layer over Binary Ninja API

**Tasks**:
1. Create `crates/binary-ninja-bridge/` crate
2. Implement Binary Ninja Rust API bindings
3. Create `BinaryLoader` for loading BNDB files
4. Create `BinaryFunctionExtractor` for extracting function info
5. Implement binary-specific feature extraction
6. Add comprehensive error handling
7. Write unit tests with mock binaries

**Deliverables**:
- `crates/binary-ninja-bridge/src/lib.rs`
- `crates/binary-ninja-bridge/src/loader.rs`
- `crates/binary-ninja-bridge/src/extractor.rs`
- `crates/binary-ninja-bridge/src/features.rs`
- `crates/binary-ninja-bridge/tests/`

### Phase 2: Binary Matching Engine (Week 3-4)

**Goal**: Port binary matching algorithms to smartdiff

**Tasks**:
1. Extend `crates/diff-engine/` with binary matching
2. Port exact hash matching algorithm
3. Port structural matching algorithm
4. Port heuristic matching with parallelization
5. Implement binary-specific similarity scoring
6. Add confidence calculation for binary matches
7. Write comprehensive tests

**Deliverables**:
- `crates/diff-engine/src/binary_matcher.rs`
- `crates/diff-engine/src/binary_similarity.rs`
- Updated `crates/diff-engine/src/engine.rs`
- Tests for binary matching

### Phase 3: MCP Server Extension (Week 5-6)

**Goal**: Add MCP tools for binary analysis

**Tasks**:
1. Extend `crates/mcp-server/` with binary tools
2. Implement `compare_binaries` tool
3. Implement `list_binary_function_matches` tool
4. Implement `get_binary_function_diff` tool
5. Implement `load_binary_in_binja` tool
6. Implement `list_binary_functions` tool
7. Add binary-specific resources
8. Update MCP server documentation

**Deliverables**:
- `crates/mcp-server/src/tools/binary_tools.rs`
- `crates/mcp-server/src/resources/binary_resources.rs`
- Updated `crates/mcp-server/src/server.rs`
- Updated `crates/mcp-server/README.md`
- Updated `crates/mcp-server/MCP_USAGE.md`

### Phase 4: Integration & Testing (Week 7)

**Goal**: End-to-end testing and documentation

**Tasks**:
1. Integration tests with real binaries
2. Performance benchmarking
3. MCP client testing (Claude Desktop)
4. Documentation updates
5. Example workflows
6. Error handling improvements

**Deliverables**:
- Integration test suite
- Performance benchmarks
- User documentation
- Example binaries and workflows

### Phase 5: Optional Enhancements (Week 8+)

**Goal**: Advanced features and optimizations

**Tasks**:
1. Decompilation diff support
2. Cross-architecture comparison
3. Incremental analysis caching
4. Web UI integration for binary diffs
5. Export formats (JSON, CSV, HTML)
6. Advanced visualization

## Technical Considerations

### Dependencies

**New Cargo Dependencies**:
```toml
[dependencies]
# Binary Ninja API
binaryninja = { git = "https://github.com/Vector35/binaryninja-api", branch = "dev" }

# Existing smartdiff dependencies
smart-diff-parser = { path = "../parser" }
smart-diff-engine = { path = "../diff-engine" }
smart-diff-semantic = { path = "../semantic-analysis" }
```

### Binary Ninja Licensing

- Requires Binary Ninja Commercial or Personal license
- MCP server should gracefully handle missing Binary Ninja
- Provide clear error messages for licensing issues
- Document Binary Ninja installation requirements

### Performance Considerations

1. **Binary Loading**: BNDB files can be large, implement lazy loading
2. **Parallel Processing**: Use rayon for parallel function matching
3. **Caching**: Cache extracted features to avoid re-analysis
4. **Memory Management**: Stream large result sets, don't load all in memory

### Error Handling

1. **Binary Ninja Not Available**: Graceful degradation
2. **Invalid Binary Files**: Clear error messages
3. **Analysis Failures**: Partial results when possible
4. **MCP Protocol Errors**: Standard JSON-RPC error codes

## Success Criteria

1. ✅ AI agents can load and analyze binary files via MCP
2. ✅ Binary function matching achieves similar accuracy to rust_diff
3. ✅ MCP tools follow smartdiff architecture patterns
4. ✅ Performance is acceptable (< 5s for typical binary comparison)
5. ✅ Documentation is comprehensive and clear
6. ✅ Integration tests pass with real binaries
7. ✅ Works with Claude Desktop and other MCP clients

## Next Steps

1. Review and approve this plan
2. Set up development environment with Binary Ninja
3. Create `crates/binary-ninja-bridge/` skeleton
4. Begin Phase 1 implementation
5. Regular progress reviews and adjustments

