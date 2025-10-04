# Binary Ninja Diff MCP - Quick Start Implementation Guide

## Overview

This guide provides step-by-step instructions to begin implementing Binary Ninja diff capabilities in smartdiff via MCP. Start here for immediate action.

## Prerequisites

### Required Software
1. **Binary Ninja** (Commercial or Personal license)
   - Download from: https://binary.ninja/
   - Install latest stable or dev build
   - Note installation path

2. **Rust Toolchain** (already installed)
   - Verify: `rustc --version`
   - Should be 1.70+

3. **Binary Ninja Rust API**
   - Available at: https://github.com/Vector35/binaryninja-api
   - Will be added as dependency

### Environment Setup

```bash
# Set Binary Ninja installation path
export BINJA_DIR="/path/to/Binary Ninja.app/Contents/MacOS"  # macOS
# or
export BINJA_DIR="/path/to/binaryninja"  # Linux
# or
set BINJA_DIR="C:\Program Files\Vector35\BinaryNinja"  # Windows

# Verify Binary Ninja is accessible
ls "$BINJA_DIR"
```

## Step 1: Create Binary Ninja Bridge Crate

### 1.1 Create Crate Structure

```bash
cd /home/matteius/codediff
mkdir -p crates/binary-ninja-bridge/src
cd crates/binary-ninja-bridge
```

### 1.2 Create Cargo.toml

```toml
[package]
name = "smart-diff-binary-ninja"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Binary Ninja integration for smart code diff"

[dependencies]
# Binary Ninja API
binaryninja = { git = "https://github.com/Vector35/binaryninja-api", branch = "dev" }

# Workspace dependencies
serde.workspace = true
serde_json.workspace = true
anyhow.workspace = true
thiserror.workspace = true
tracing.workspace = true

# Additional dependencies
sha2 = "0.10"
hex = "0.4"
parking_lot = "0.12"

[dev-dependencies]
tokio.workspace = true
```

### 1.3 Create lib.rs Skeleton

```rust
//! Binary Ninja integration for smart code diff
//!
//! This crate provides an abstraction layer over Binary Ninja's API
//! for extracting and analyzing binary functions.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

pub mod loader;
pub mod extractor;
pub mod features;
pub mod types;

pub use loader::BinaryLoader;
pub use extractor::FunctionExtractor;
pub use types::{BinaryInfo, FunctionInfo, BasicBlockInfo, InstructionInfo};

#[derive(Debug, Error)]
pub enum BinaryNinjaError {
    #[error("Failed to load binary: {0}")]
    LoadError(String),
    
    #[error("Binary Ninja not available: {0}")]
    NotAvailable(String),
    
    #[error("Analysis failed: {0}")]
    AnalysisError(String),
    
    #[error("Invalid binary format: {0}")]
    InvalidFormat(String),
}

/// Check if Binary Ninja is available
pub fn is_available() -> bool {
    // TODO: Implement Binary Ninja availability check
    false
}

/// Get Binary Ninja version
pub fn version() -> Result<String> {
    // TODO: Implement version check
    Ok("unknown".to_string())
}
```

### 1.4 Create types.rs

```rust
//! Type definitions for binary analysis

use serde::{Deserialize, Serialize};

/// Information about a loaded binary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryInfo {
    pub file_path: String,
    pub architecture: String,
    pub platform: String,
    pub entry_point: u64,
    pub function_count: usize,
    pub analysis_complete: bool,
}

/// Information about a binary function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub basic_blocks: Vec<BasicBlockInfo>,
    pub instructions: Vec<InstructionInfo>,
    pub cyclomatic_complexity: u32,
    pub call_graph_hash: String,
    pub cfg_hash: String,
    pub instruction_count: usize,
    pub call_count: usize,
}

/// Information about a basic block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlockInfo {
    pub address: u64,
    pub size: u64,
    pub instructions: Vec<InstructionInfo>,
    pub edges: Vec<u64>,
    pub mnemonic_hash: String,
    pub instruction_count: usize,
}

/// Information about an instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionInfo {
    pub address: u64,
    pub mnemonic: String,
    pub operands: Vec<String>,
    pub bytes: Vec<u8>,
    pub length: usize,
}

impl FunctionInfo {
    /// Create a new function info
    pub fn new(name: String, address: u64) -> Self {
        Self {
            name,
            address,
            size: 0,
            basic_blocks: Vec::new(),
            instructions: Vec::new(),
            cyclomatic_complexity: 0,
            call_graph_hash: String::new(),
            cfg_hash: String::new(),
            instruction_count: 0,
            call_count: 0,
        }
    }
}
```

### 1.5 Create loader.rs

```rust
//! Binary loading and management

use crate::types::BinaryInfo;
use crate::BinaryNinjaError;
use anyhow::Result;
use std::path::Path;
use tracing::{info, warn};

/// Binary loader for Binary Ninja
pub struct BinaryLoader {
    // TODO: Add Binary Ninja BinaryView handle
}

impl BinaryLoader {
    /// Create a new binary loader
    pub fn new() -> Self {
        Self {}
    }
    
    /// Load a binary file
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<BinaryInfo> {
        let path = path.as_ref();
        info!("Loading binary: {}", path.display());
        
        // TODO: Implement Binary Ninja loading
        // For now, return mock data
        warn!("Binary Ninja integration not yet implemented");
        
        Ok(BinaryInfo {
            file_path: path.to_string_lossy().to_string(),
            architecture: "x86_64".to_string(),
            platform: "linux".to_string(),
            entry_point: 0x1000,
            function_count: 0,
            analysis_complete: false,
        })
    }
    
    /// Check if a binary is loaded
    pub fn is_loaded(&self) -> bool {
        // TODO: Implement
        false
    }
    
    /// Close the loaded binary
    pub fn close(&mut self) {
        // TODO: Implement
    }
}

impl Default for BinaryLoader {
    fn default() -> Self {
        Self::new()
    }
}
```

### 1.6 Create extractor.rs

```rust
//! Function extraction from binaries

use crate::types::{FunctionInfo, BasicBlockInfo, InstructionInfo};
use crate::BinaryNinjaError;
use anyhow::Result;
use tracing::info;

/// Function extractor for Binary Ninja
pub struct FunctionExtractor {
    // TODO: Add Binary Ninja BinaryView reference
}

impl FunctionExtractor {
    /// Create a new function extractor
    pub fn new() -> Self {
        Self {}
    }
    
    /// Extract all functions from the binary
    pub fn extract_all(&self) -> Result<Vec<FunctionInfo>> {
        info!("Extracting all functions");
        
        // TODO: Implement Binary Ninja function extraction
        // For now, return empty list
        Ok(Vec::new())
    }
    
    /// Extract a specific function by address
    pub fn extract_at(&self, address: u64) -> Result<Option<FunctionInfo>> {
        info!("Extracting function at 0x{:x}", address);
        
        // TODO: Implement
        Ok(None)
    }
    
    /// Extract a specific function by name
    pub fn extract_by_name(&self, name: &str) -> Result<Option<FunctionInfo>> {
        info!("Extracting function: {}", name);
        
        // TODO: Implement
        Ok(None)
    }
}

impl Default for FunctionExtractor {
    fn default() -> Self {
        Self::new()
    }
}
```

### 1.7 Create features.rs

```rust
//! Feature computation for binary functions

use crate::types::{FunctionInfo, BasicBlockInfo};
use sha2::{Sha256, Digest};
use hex;

/// Compute CFG hash for a function
pub fn compute_cfg_hash(function: &FunctionInfo) -> String {
    let mut hasher = Sha256::new();
    
    // Hash basic block structure
    for bb in &function.basic_blocks {
        hasher.update(bb.address.to_le_bytes());
        hasher.update(bb.size.to_le_bytes());
        
        // Hash edges
        for edge in &bb.edges {
            hasher.update(edge.to_le_bytes());
        }
    }
    
    let result = hasher.finalize();
    hex::encode(result)
}

/// Compute call graph hash for a function
pub fn compute_call_graph_hash(function: &FunctionInfo) -> String {
    let mut hasher = Sha256::new();
    
    // Hash call count and pattern
    hasher.update(function.call_count.to_le_bytes());
    
    // TODO: Hash actual call targets when available
    
    let result = hasher.finalize();
    hex::encode(result)
}

/// Compute mnemonic hash for a basic block
pub fn compute_mnemonic_hash(basic_block: &BasicBlockInfo) -> String {
    let mut hasher = Sha256::new();
    
    // Hash instruction mnemonics
    for instr in &basic_block.instructions {
        hasher.update(instr.mnemonic.as_bytes());
    }
    
    let result = hasher.finalize();
    hex::encode(result)
}

/// Calculate cyclomatic complexity
pub fn calculate_complexity(function: &FunctionInfo) -> u32 {
    // McCabe's cyclomatic complexity: E - N + 2P
    // For a single function: edges - nodes + 2
    
    let nodes = function.basic_blocks.len() as u32;
    let edges: u32 = function.basic_blocks.iter()
        .map(|bb| bb.edges.len() as u32)
        .sum();
    
    if nodes == 0 {
        return 0;
    }
    
    edges.saturating_sub(nodes).saturating_add(2)
}
```

### 1.8 Update Workspace Cargo.toml

Add to `/home/matteius/codediff/Cargo.toml`:

```toml
[workspace]
members = [
    "crates/parser",
    "crates/diff-engine",
    "crates/semantic-analysis",
    "crates/cli",
    "crates/web-ui",
    "crates/mcp-server",
    "crates/binary-ninja-bridge",  # ADD THIS LINE
]
```

## Step 2: Test the Skeleton

### 2.1 Build the Crate

```bash
cd /home/matteius/codediff
cargo build -p smart-diff-binary-ninja
```

### 2.2 Run Tests

```bash
cargo test -p smart-diff-binary-ninja
```

### 2.3 Check for Errors

```bash
cargo clippy -p smart-diff-binary-ninja
```

## Step 3: Implement Binary Ninja Integration

### 3.1 Study Binary Ninja Rust API

```bash
# Clone Binary Ninja API repository
git clone https://github.com/Vector35/binaryninja-api.git /tmp/binja-api
cd /tmp/binja-api

# Review Rust examples
ls rust/examples/
cat rust/examples/basic.rs
```

### 3.2 Implement BinaryLoader

Update `loader.rs` with actual Binary Ninja API calls:

```rust
use binaryninja::binaryview::{BinaryView, BinaryViewExt};
use binaryninja::headless::Session;

pub struct BinaryLoader {
    session: Option<Session>,
    view: Option<BinaryView>,
}

impl BinaryLoader {
    pub fn new() -> Self {
        // Initialize Binary Ninja headless session
        let session = Session::new().ok();
        
        Self {
            session,
            view: None,
        }
    }
    
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<BinaryInfo> {
        let path = path.as_ref();
        
        // Load binary with Binary Ninja
        let view = BinaryView::from_filename(path)
            .map_err(|e| BinaryNinjaError::LoadError(e.to_string()))?;
        
        // Wait for analysis to complete
        view.update_analysis_and_wait();
        
        let info = BinaryInfo {
            file_path: path.to_string_lossy().to_string(),
            architecture: view.default_arch().map(|a| a.name()).unwrap_or("unknown".to_string()),
            platform: view.default_platform().map(|p| p.name()).unwrap_or("unknown".to_string()),
            entry_point: view.entry_point(),
            function_count: view.functions().len(),
            analysis_complete: true,
        };
        
        self.view = Some(view);
        Ok(info)
    }
}
```

## Step 4: Next Steps

1. **Complete Binary Ninja Integration**:
   - Implement full `BinaryLoader`
   - Implement full `FunctionExtractor`
   - Add comprehensive error handling

2. **Create Binary Matcher**:
   - Port matching algorithms from rust_diff
   - Integrate with smartdiff diff-engine

3. **Add MCP Tools**:
   - Extend MCP server with binary tools
   - Add binary-specific resources

4. **Testing**:
   - Create test binaries
   - Write integration tests
   - Performance benchmarking

5. **Documentation**:
   - API documentation
   - Usage examples
   - MCP tool documentation

## Troubleshooting

### Binary Ninja Not Found

```bash
# Verify Binary Ninja installation
ls "$BINJA_DIR"

# Check for libbinaryninjacore
ls "$BINJA_DIR"/libbinaryninjacore.*

# Set library path
export LD_LIBRARY_PATH="$BINJA_DIR:$LD_LIBRARY_PATH"  # Linux
export DYLD_LIBRARY_PATH="$BINJA_DIR:$DYLD_LIBRARY_PATH"  # macOS
```

### Compilation Errors

```bash
# Clean and rebuild
cargo clean
cargo build -p smart-diff-binary-ninja

# Check Rust version
rustc --version  # Should be 1.70+

# Update dependencies
cargo update
```

### License Issues

- Ensure Binary Ninja license is activated
- Check license file location
- Verify headless mode is enabled in license

## Resources

- **Binary Ninja API Docs**: https://api.binary.ninja/
- **Rust API Examples**: https://github.com/Vector35/binaryninja-api/tree/dev/rust/examples
- **MCP Specification**: https://modelcontextprotocol.io/
- **smartdiff Docs**: `/home/matteius/codediff/docs/`

## Summary

This quick start guide sets up the foundation for Binary Ninja integration. The skeleton provides:

✅ Crate structure with proper dependencies
✅ Type definitions for binary analysis
✅ Placeholder implementations
✅ Clear TODOs for next steps
✅ Testing infrastructure

Next: Implement actual Binary Ninja API integration following the examples in the binaryninja-api repository.

