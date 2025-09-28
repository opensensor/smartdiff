# Smart Code Diff - Semantic Analysis Engine

Advanced semantic analysis engine that provides comprehensive symbol resolution, cross-file reference tracking, and scope management for multi-language codebases.

## Features

### 🔍 **Symbol Resolution**
- **Cross-file symbol lookup** with import resolution
- **Qualified name resolution** (e.g., `MyClass.myMethod`)
- **Scope-aware symbol visibility** with shadowing detection
- **Multi-language support** (Java, Python, JavaScript, C/C++)

### 📊 **Symbol Table Management**
- **Hierarchical scope tracking** with parent-child relationships
- **Symbol reference tracking** (declarations, definitions, usages, calls)
- **Symbol statistics and metrics** for codebase analysis
- **Efficient symbol lookup** with caching and optimization

### 🌐 **Import Resolution**
- **Language-specific import parsing**:
  - Java: `import`, `import static`, wildcard imports
  - Python: `import`, `from...import`, aliases
  - JavaScript: ES6 imports, CommonJS require
  - C/C++: `#include` system and local headers
- **Cross-file dependency tracking**
- **Import graph construction** for dependency analysis

### 🎯 **Scope Management**
- **Hierarchical scope resolution** (global → file → class → function → block)
- **Symbol shadowing detection** and resolution
- **Scope analysis metrics** (depth, symbol distribution, etc.)
- **Context-aware symbol lookup**

## Quick Start

### Basic Symbol Resolution

```rust
use smart_diff_semantic::{SymbolResolver, SymbolResolverConfig};
use smart_diff_parser::{TreeSitterParser, Language};

// Create resolver with default configuration
let mut resolver = SymbolResolver::with_defaults();
let parser = TreeSitterParser::new()?;

// Parse and process a file
let code = r#"
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
}
"#;

let parse_result = parser.parse(code, Language::Java)?;
resolver.process_file("Calculator.java", &parse_result)?;

// Find symbols
let symbol = resolver.find_symbol("Calculator", Some("Calculator.java"));
println!("Found: {:?}", symbol);
```

### Cross-File Resolution

```rust
// Process multiple files
let files = vec![
    ("Interface.java".to_string(), interface_parse_result),
    ("Implementation.java".to_string(), impl_parse_result),
];

resolver.process_files(files)?;

// Access import graph
let import_graph = resolver.get_import_graph();
for (file, imports) in import_graph {
    println!("{} imports: {:?}", file, imports);
}
```

### Advanced Scope Management

```rust
use smart_diff_semantic::{ScopeManager, ScopeType};

let mut scope_manager = ScopeManager::new(Language::Java);

// Create nested scopes
let global_scope = scope_manager.create_scope(
    ScopeType::Global, 
    "file.java".to_string(), 
    1, 100
);
scope_manager.enter_scope(global_scope);

let class_scope = scope_manager.create_scope(
    ScopeType::Class, 
    "file.java".to_string(), 
    5, 50
);
scope_manager.enter_scope(class_scope);

// Resolve symbols with scope awareness
let resolution = scope_manager.resolve_symbol("myVariable");
if let Some(res) = resolution {
    println!("Found {} in scope {} (shadowed: {})", 
             res.symbol.name, res.scope_id, res.is_shadowed);
}
```

### Symbol Table Statistics

```rust
let symbol_table = resolver.get_symbol_table();
let stats = symbol_table.get_statistics();

println!("Total symbols: {}", stats.total_symbols);
println!("Functions: {}", stats.function_count);
println!("Classes: {}", stats.class_count);
println!("Average references per symbol: {:.2}", stats.avg_references_per_symbol);
```

## Configuration

### SymbolResolverConfig

```rust
use smart_diff_semantic::SymbolResolverConfig;
use std::collections::HashSet;

let config = SymbolResolverConfig {
    resolve_cross_file: true,        // Enable cross-file resolution
    track_usages: true,              // Track all symbol usages
    resolve_imports: true,           // Resolve import statements
    max_resolution_depth: 10,        // Maximum recursion depth
    file_extensions: {               // Supported file extensions
        let mut ext = HashSet::new();
        ext.insert("java".to_string());
        ext.insert("py".to_string());
        ext.insert("js".to_string());
        ext.insert("cpp".to_string());
        ext.insert("c".to_string());
        ext
    },
};

let resolver = SymbolResolver::new(config);
```

## Symbol Types

The engine recognizes various symbol types:

- **`Function`** - Standalone functions
- **`Method`** - Class/object methods
- **`Class`** - Class definitions
- **`Interface`** - Interface definitions
- **`Variable`** - Local and global variables
- **`Constant`** - Constants and final variables
- **`Parameter`** - Function/method parameters
- **`Field`** - Class/struct fields
- **`Module`** - Modules and namespaces
- **`Namespace`** - Namespace declarations

## Reference Types

Symbol references are categorized by usage:

- **`Declaration`** - Symbol declaration
- **`Definition`** - Symbol definition (implementation)
- **`Usage`** - General symbol usage
- **`Call`** - Function/method calls
- **`Assignment`** - Variable assignments

## Scope Types

Hierarchical scope management supports:

- **`Global`** - Global/file-level scope
- **`File`** - File-specific scope
- **`Class`** - Class/interface scope
- **`Function`** - Function/method scope
- **`Block`** - Block-level scope (loops, conditionals)
- **`Module`** - Module/namespace scope

## Import Resolution

### Java
```java
import java.util.List;           // Regular import
import java.util.*;              // Wildcard import
import static java.lang.Math.PI; // Static import
```

### Python
```python
import os                        # Module import
import numpy as np               # Import with alias
from collections import defaultdict  # From import
from datetime import datetime as dt  # From import with alias
```

### JavaScript
```javascript
import React from 'react';              // ES6 default import
import { useState } from 'react';       // ES6 named import
import * as React from 'react';         // ES6 wildcard import
const fs = require('fs');               // CommonJS require
```

### C/C++
```c
#include <stdio.h>      // System header
#include "myheader.h"   // Local header
```

## Examples

Run the comprehensive demo:

```bash
cargo run --example symbol_resolution_demo
```

This demonstrates:
- Basic symbol resolution
- Cross-file reference tracking
- Import statement parsing
- Scope management
- Symbol statistics

## Testing

Run the test suite:

```bash
cargo test -p smart-diff-semantic
```

Tests cover:
- Symbol resolution algorithms
- Import parsing for all languages
- Scope management and shadowing
- Cross-file reference resolution
- Symbol table operations

## Performance

The semantic analysis engine is optimized for:
- **Memory efficiency** with symbol deduplication
- **Fast lookups** using hash maps and caching
- **Scalable processing** for large codebases
- **Incremental updates** for real-time analysis

## Architecture

```
SymbolResolver
├── SymbolTable (hierarchical symbol storage)
├── ScopeManager (scope hierarchy management)
├── ImportResolver (cross-file dependency tracking)
└── ReferenceTracker (usage and call tracking)
```

The engine integrates seamlessly with the parser crate to provide comprehensive semantic analysis capabilities for the Smart Code Diffing Tool.
