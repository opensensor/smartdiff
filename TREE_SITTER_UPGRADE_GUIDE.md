# Tree-Sitter Upgrade Guide: 0.20 → 0.22.6

## Overview

This guide provides step-by-step instructions for upgrading the Smart Diff project from tree-sitter 0.20 to 0.22.6. This upgrade is **required** to support PHP and Swift language parsers.

## Why This Upgrade is Needed

**Problem**: Version conflict between tree-sitter dependencies
- Current project uses: `tree-sitter = "0.20"`
- `tree-sitter-php = "0.23"` requires: `tree-sitter = "0.22.6"`
- `tree-sitter-swift = "0.6"` requires: `tree-sitter = "0.22.6"`

**Impact**: Rust treats `tree_sitter::Language` from version 0.20 and 0.22.6 as incompatible types, causing compilation errors.

**Solution**: Upgrade all tree-sitter dependencies to compatible versions.

## Pre-Upgrade Checklist

- [ ] Commit all current changes
- [ ] Create a new branch: `git checkout -b upgrade/tree-sitter-0.22.6`
- [ ] Run existing tests to establish baseline: `cargo test`
- [ ] Document current test results
- [ ] Backup `Cargo.lock` file

## Step 1: Update Workspace Dependencies

**File**: `Cargo.toml` (workspace root)

### Current Dependencies
```toml
[workspace.dependencies]
tree-sitter = "0.20"
tree-sitter-java = "0.20"
tree-sitter-python = "0.20"
tree-sitter-javascript = "0.20"
tree-sitter-cpp = "0.21"
tree-sitter-c = "0.20"
tree-sitter-go = "0.20"
tree-sitter-ruby = "0.20"
tree-sitter-php = "0.23"
tree-sitter-swift = "0.6"
```

### Updated Dependencies
```toml
[workspace.dependencies]
tree-sitter = "0.22.6"
tree-sitter-java = "0.23"
tree-sitter-python = "0.23"
tree-sitter-javascript = "0.23"
tree-sitter-cpp = "0.23"
tree-sitter-c = "0.23"
tree-sitter-go = "0.25"
tree-sitter-ruby = "0.23"
tree-sitter-php = "0.24"
tree-sitter-swift = "0.7"
```

### Commands
```bash
# Update workspace Cargo.toml
sed -i 's/tree-sitter = "0.20"/tree-sitter = "0.22.6"/' Cargo.toml
sed -i 's/tree-sitter-java = "0.20"/tree-sitter-java = "0.23"/' Cargo.toml
sed -i 's/tree-sitter-python = "0.20"/tree-sitter-python = "0.23"/' Cargo.toml
sed -i 's/tree-sitter-javascript = "0.20"/tree-sitter-javascript = "0.23"/' Cargo.toml
sed -i 's/tree-sitter-cpp = "0.21"/tree-sitter-cpp = "0.23"/' Cargo.toml
sed -i 's/tree-sitter-c = "0.20"/tree-sitter-c = "0.23"/' Cargo.toml
sed -i 's/tree-sitter-go = "0.20"/tree-sitter-go = "0.25"/' Cargo.toml
sed -i 's/tree-sitter-ruby = "0.20"/tree-sitter-ruby = "0.23"/' Cargo.toml
sed -i 's/tree-sitter-php = "0.23"/tree-sitter-php = "0.24"/' Cargo.toml
sed -i 's/tree-sitter-swift = "0.6"/tree-sitter-swift = "0.7"/' Cargo.toml

# Update Cargo.lock
cargo update
```

## Step 2: Fix PHP and Swift Parser Integration

**File**: `crates/parser/src/tree_sitter.rs`

### Current Code (Broken)
```rust
configs.insert(
    Language::PHP,
    || tree_sitter_php::language_php(),
);
configs.insert(
    Language::Swift,
    || unsafe { tree_sitter::Language::from_raw(tree_sitter_swift::LANGUAGE.into_raw()) },
);
```

### Updated Code
```rust
configs.insert(
    Language::PHP,
    tree_sitter_php::language as fn() -> tree_sitter::Language,
);
configs.insert(
    Language::Swift,
    tree_sitter_swift::language as fn() -> tree_sitter::Language,
);
```

**Note**: After the upgrade, PHP and Swift should use the same pattern as other languages.

## Step 3: Check for API Changes

### Known API Changes in tree-sitter 0.22.6

1. **Language struct**: No breaking changes expected
2. **Parser API**: Minor improvements, backward compatible
3. **Query API**: Enhanced but backward compatible
4. **Node API**: No breaking changes

### Files to Review

Check these files for potential API usage:
- `crates/parser/src/tree_sitter.rs` - Parser initialization
- `crates/parser/src/ast_builder.rs` - AST construction
- `crates/parser/src/parser.rs` - Main parser logic
- `crates/semantic-analysis/src/symbol_resolver.rs` - Symbol resolution

### Verification Commands
```bash
# Check for deprecated API usage
cargo check -p smart-diff-parser 2>&1 | grep -i "deprecated"

# Check for breaking changes
cargo check -p smart-diff-parser 2>&1 | grep -i "error"
```

## Step 4: Test Each Language Parser

### Test Strategy

Test each language parser individually to isolate issues:

```bash
# Test Java parser
cargo test -p smart-diff-parser test_java

# Test Python parser
cargo test -p smart-diff-parser test_python

# Test JavaScript parser
cargo test -p smart-diff-parser test_javascript

# Test C++ parser
cargo test -p smart-diff-parser test_cpp

# Test C parser
cargo test -p smart-diff-parser test_c

# Test Go parser
cargo test -p smart-diff-parser test_go

# Test Ruby parser
cargo test -p smart-diff-parser test_ruby

# Test PHP parser (NEW)
cargo test -p smart-diff-parser test_php

# Test Swift parser (NEW)
cargo test -p smart-diff-parser test_swift
```

### Expected Results

All existing tests should pass. If any fail:
1. Check for API changes in that specific parser
2. Review tree-sitter changelog for that language
3. Update code accordingly

## Step 5: Run Full Test Suite

```bash
# Run all parser tests
cargo test -p smart-diff-parser

# Run all semantic analysis tests
cargo test -p smart-diff-semantic-analysis

# Run all diff engine tests
cargo test -p smart-diff-engine

# Run all tests
cargo test --workspace
```

### Success Criteria

- [ ] All existing tests pass
- [ ] No compilation errors
- [ ] No new warnings introduced
- [ ] PHP parser compiles and works
- [ ] Swift parser compiles and works

## Step 6: Add Tests for New Languages

### Go Language Tests

**File**: `crates/parser/tests/go_parser_tests.rs`

```rust
use smart_diff_parser::{Language, Parser};

#[test]
fn test_go_function_parsing() {
    let code = r#"
package main

func add(a int, b int) int {
    return a + b
}
"#;
    let parser = Parser::new(Language::Go);
    let ast = parser.parse(code).unwrap();
    
    assert_eq!(ast.functions.len(), 1);
    assert_eq!(ast.functions[0].name, "add");
}

#[test]
fn test_go_struct_parsing() {
    let code = r#"
package main

type Person struct {
    Name string
    Age  int
}
"#;
    let parser = Parser::new(Language::Go);
    let ast = parser.parse(code).unwrap();
    
    assert_eq!(ast.classes.len(), 1);
    assert_eq!(ast.classes[0].name, "Person");
}
```

### Ruby Language Tests

**File**: `crates/parser/tests/ruby_parser_tests.rs`

```rust
use smart_diff_parser::{Language, Parser};

#[test]
fn test_ruby_method_parsing() {
    let code = r#"
def greet(name)
  puts "Hello, #{name}!"
end
"#;
    let parser = Parser::new(Language::Ruby);
    let ast = parser.parse(code).unwrap();
    
    assert_eq!(ast.functions.len(), 1);
    assert_eq!(ast.functions[0].name, "greet");
}

#[test]
fn test_ruby_class_parsing() {
    let code = r#"
class Calculator
  def add(a, b)
    a + b
  end
end
"#;
    let parser = Parser::new(Language::Ruby);
    let ast = parser.parse(code).unwrap();
    
    assert_eq!(ast.classes.len(), 1);
    assert_eq!(ast.classes[0].name, "Calculator");
}
```

### PHP Language Tests

**File**: `crates/parser/tests/php_parser_tests.rs`

```rust
use smart_diff_parser::{Language, Parser};

#[test]
fn test_php_function_parsing() {
    let code = r#"
<?php
function add($a, $b) {
    return $a + $b;
}
"#;
    let parser = Parser::new(Language::PHP);
    let ast = parser.parse(code).unwrap();
    
    assert_eq!(ast.functions.len(), 1);
    assert_eq!(ast.functions[0].name, "add");
}

#[test]
fn test_php_class_parsing() {
    let code = r#"
<?php
class Calculator {
    public function add($a, $b) {
        return $a + $b;
    }
}
"#;
    let parser = Parser::new(Language::PHP);
    let ast = parser.parse(code).unwrap();
    
    assert_eq!(ast.classes.len(), 1);
    assert_eq!(ast.classes[0].name, "Calculator");
}
```

### Swift Language Tests

**File**: `crates/parser/tests/swift_parser_tests.rs`

```rust
use smart_diff_parser::{Language, Parser};

#[test]
fn test_swift_function_parsing() {
    let code = r#"
func add(_ a: Int, _ b: Int) -> Int {
    return a + b
}
"#;
    let parser = Parser::new(Language::Swift);
    let ast = parser.parse(code).unwrap();
    
    assert_eq!(ast.functions.len(), 1);
    assert_eq!(ast.functions[0].name, "add");
}

#[test]
fn test_swift_class_parsing() {
    let code = r#"
class Calculator {
    func add(_ a: Int, _ b: Int) -> Int {
        return a + b
    }
}
"#;
    let parser = Parser::new(Language::Swift);
    let ast = parser.parse(code).unwrap();
    
    assert_eq!(ast.classes.len(), 1);
    assert_eq!(ast.classes[0].name, "Calculator");
}
```

## Step 7: Performance Testing

### Benchmark Parsing Performance

```bash
# Create benchmark file
cat > benches/language_parsing.rs << 'EOF'
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smart_diff_parser::{Language, Parser};

fn bench_go_parsing(c: &mut Criterion) {
    let code = include_str!("../test_data/sample.go");
    c.bench_function("parse_go", |b| {
        b.iter(|| {
            let parser = Parser::new(Language::Go);
            parser.parse(black_box(code))
        })
    });
}

criterion_group!(benches, bench_go_parsing);
criterion_main!(benches);
EOF

# Run benchmarks
cargo bench
```

## Step 8: Update Documentation

### Files to Update

1. **README.md**: Add Go, Ruby, PHP, Swift to supported languages
2. **crates/parser/README.md**: Add examples for new languages
3. **docs/ARCHITECTURE.md**: Update parser architecture section
4. **CHANGELOG.md**: Document the upgrade

### Example README Update

```markdown
## Supported Languages

Smart Diff now supports **10 programming languages**:

- ✅ Java
- ✅ Python
- ✅ JavaScript
- ✅ TypeScript
- ✅ C++
- ✅ C
- ✅ **Go** (NEW)
- ✅ **Ruby** (NEW)
- ✅ **PHP** (NEW)
- ✅ **Swift** (NEW)
```

## Rollback Plan

If the upgrade fails:

```bash
# Restore from backup
git checkout main
git branch -D upgrade/tree-sitter-0.22.6

# Or revert specific changes
git revert <commit-hash>

# Restore Cargo.lock
git checkout HEAD -- Cargo.lock
```

## Common Issues and Solutions

### Issue 1: Compilation Errors After Upgrade

**Symptom**: `error[E0308]: mismatched types`

**Solution**: Ensure all tree-sitter dependencies use compatible versions. Run `cargo tree | grep tree-sitter` to check.

### Issue 2: Tests Fail After Upgrade

**Symptom**: Existing tests fail with parsing errors

**Solution**: Check if node type names changed in the new parser version. Update `language_config.rs` accordingly.

### Issue 3: Performance Regression

**Symptom**: Parsing is slower after upgrade

**Solution**: Tree-sitter 0.22.6 should be faster. If slower, check for inefficient API usage.

## Verification Checklist

- [ ] All dependencies updated in `Cargo.toml`
- [ ] `Cargo.lock` regenerated with `cargo update`
- [ ] PHP parser integration complete
- [ ] Swift parser integration complete
- [ ] All existing tests pass
- [ ] New language tests added and passing
- [ ] No compilation warnings
- [ ] Documentation updated
- [ ] Performance benchmarks run
- [ ] Changes committed to branch
- [ ] Pull request created

## Timeline Estimate

| Task | Estimated Time |
|------|----------------|
| Update dependencies | 15 minutes |
| Fix PHP/Swift integration | 30 minutes |
| Test existing parsers | 30 minutes |
| Add new language tests | 2 hours |
| Performance testing | 30 minutes |
| Documentation updates | 1 hour |
| **Total** | **4-5 hours** |

## Success Metrics

- ✅ Zero compilation errors
- ✅ Zero test failures
- ✅ All 10 languages parsing correctly
- ✅ No performance regression
- ✅ Documentation complete

## Next Steps After Upgrade

1. Merge upgrade branch to main
2. Implement class-based refactoring detection enhancements
3. Add integration tests for cross-file refactoring with new languages
4. Create comprehensive examples for each language
5. Performance optimization for large codebases

## References

- [Tree-sitter 0.22.6 Release Notes](https://github.com/tree-sitter/tree-sitter/releases/tag/v0.22.6)
- [Tree-sitter Rust Binding Documentation](https://docs.rs/tree-sitter/0.22.6/tree_sitter/)
- [Tree-sitter PHP Grammar](https://github.com/tree-sitter/tree-sitter-php)
- [Tree-sitter Swift Grammar](https://github.com/alex-pinkus/tree-sitter-swift)

