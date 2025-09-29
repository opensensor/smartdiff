# Smart Code Diff Developer Guide

This guide provides comprehensive information for developers who want to contribute to, extend, or integrate with the Smart Code Diff project.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Core Components](#core-components)
- [Contributing Guidelines](#contributing-guidelines)
- [Extension Development](#extension-development)
- [Testing Strategy](#testing-strategy)
- [Performance Considerations](#performance-considerations)

## Architecture Overview

Smart Code Diff follows a modular architecture with clear separation of concerns:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web UI        │    │      CLI        │    │   REST API      │
│  (React/TS)     │    │    (Rust)       │    │    (Axum)       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
         ┌───────────────────────────────────────────────┐
         │              Core Engine                      │
         └───────────────────────────────────────────────┘
                                 │
    ┌────────────┬────────────────┼────────────────┬────────────┐
    │            │                │                │            │
┌───▼───┐   ┌───▼───┐       ┌───▼───┐       ┌───▼───┐   ┌───▼───┐
│Parser │   │Semantic│       │ Diff  │       │Function│   │Change │
│Engine │   │Analysis│       │Engine │       │Matcher │   │Classifier│
└───────┘   └───────┘       └───────┘       └───────┘   └───────┘
     │           │               │               │           │
┌────▼────┐ ┌───▼────┐     ┌───▼────┐     ┌───▼────┐ ┌───▼────┐
│Tree-    │ │Symbol  │     │Zhang-  │     │Hungarian│ │Refactor│
│sitter   │ │Resolver│     │Shasha  │     │Algorithm│ │Detector│
└─────────┘ └────────┘     └────────┘     └────────┘ └────────┘
```

### Key Design Principles

1. **Modularity**: Each component has a single responsibility
2. **Extensibility**: Easy to add new languages and analysis types
3. **Performance**: Optimized for large codebases with caching and parallelization
4. **Accuracy**: Multiple analysis layers for comprehensive comparison
5. **Usability**: Multiple interfaces (CLI, Web, API) for different use cases

## Development Setup

### Prerequisites

- **Rust**: 1.70 or later
- **Node.js**: 18 or later
- **Git**: For version control
- **Docker**: Optional, for containerized development

### Environment Setup

1. **Clone the repository**:
```bash
git clone https://github.com/smart-code-diff/smart-code-diff.git
cd smart-code-diff
```

2. **Install Rust dependencies**:
```bash
cargo build
```

3. **Install frontend dependencies**:
```bash
cd frontend
npm install
cd ..
```

4. **Install development tools**:
```bash
# Code formatting
rustup component add rustfmt

# Linting
rustup component add clippy

# Documentation
cargo install cargo-doc

# Testing tools
cargo install cargo-tarpaulin  # Code coverage
cargo install cargo-audit      # Security auditing
```

5. **Setup pre-commit hooks**:
```bash
# Install pre-commit
pip install pre-commit

# Install hooks
pre-commit install
```

### Development Workflow

1. **Create a feature branch**:
```bash
git checkout -b feature/your-feature-name
```

2. **Make changes and test**:
```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linting
cargo clippy -- -D warnings

# Test frontend
cd frontend && npm test
```

3. **Commit and push**:
```bash
git add .
git commit -m "feat: add your feature description"
git push origin feature/your-feature-name
```

4. **Create pull request** with detailed description

## Project Structure

```
smart-code-diff/
├── Cargo.toml                 # Workspace configuration
├── Cargo.lock                 # Dependency lock file
├── README.md                  # Project overview
├── LICENSE                    # MIT license
├── .github/                   # GitHub workflows and templates
│   ├── workflows/             # CI/CD pipelines
│   └── ISSUE_TEMPLATE/        # Issue templates
├── crates/                    # Rust crates
│   ├── parser/                # Parser engine
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs         # Public API
│   │   │   ├── language_detector.rs
│   │   │   ├── parser_engine.rs
│   │   │   ├── ast_builder.rs
│   │   │   └── tree_sitter/   # Tree-sitter integration
│   │   └── tests/             # Integration tests
│   ├── semantic-analysis/     # Semantic analysis engine
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── symbol_resolver.rs
│   │   │   ├── type_extractor.rs
│   │   │   └── dependency_graph.rs
│   │   └── tests/
│   ├── diff-engine/           # Diff computation engine
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── tree_edit.rs   # Zhang-Shasha algorithm
│   │   │   ├── function_matcher.rs
│   │   │   ├── similarity_scorer.rs
│   │   │   ├── changes.rs     # Change classification
│   │   │   └── refactoring.rs # Refactoring detection
│   │   └── tests/
│   ├── cli/                   # Command-line interface
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── cli.rs         # Argument parsing
│   │   │   ├── commands/      # CLI commands
│   │   │   └── output.rs      # Output formatting
│   │   └── tests/
│   └── web-ui/                # REST API server
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs
│       │   ├── handlers.rs    # HTTP handlers
│       │   ├── models.rs      # Data models
│       │   └── api.rs         # API utilities
│       └── tests/
├── frontend/                  # React TypeScript frontend
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   ├── tailwind.config.js
│   ├── src/
│   │   ├── App.tsx
│   │   ├── components/        # React components
│   │   ├── pages/             # Page components
│   │   └── utils/             # Utility functions
│   └── public/                # Static assets
├── docs/                      # Documentation
│   ├── api/                   # API documentation
│   ├── user-guide.md          # User documentation
│   ├── developer-guide.md     # This file
│   └── architecture.md        # Architecture details
├── examples/                  # Usage examples
│   ├── cli/                   # CLI examples
│   ├── api/                   # API examples
│   └── sample-code/           # Sample code files
└── tests/                     # End-to-end tests
    ├── integration/           # Integration tests
    └── performance/           # Performance benchmarks
```

## Core Components

### 1. Parser Engine (`crates/parser`)

**Purpose**: Convert source code into normalized AST representation

**Key Files**:
- `language_detector.rs`: Detect programming language from file content
- `parser_engine.rs`: Main parsing interface
- `ast_builder.rs`: Build normalized AST from tree-sitter parse trees
- `tree_sitter/`: Language-specific tree-sitter parsers

**Key Traits**:
```rust
pub trait LanguageParser {
    fn parse(&self, content: &str) -> Result<AST, ParseError>;
    fn language(&self) -> Language;
    fn file_extensions(&self) -> &[&str];
}

pub trait ASTNode {
    fn node_type(&self) -> NodeType;
    fn children(&self) -> &[Box<dyn ASTNode>];
    fn metadata(&self) -> &HashMap<String, Value>;
}
```

**Extension Points**:
- Add new language support by implementing `LanguageParser`
- Extend AST node types by implementing `ASTNode`
- Add custom metadata extractors

### 2. Semantic Analysis (`crates/semantic-analysis`)

**Purpose**: Extract semantic information from AST

**Key Files**:
- `symbol_resolver.rs`: Resolve symbols and build symbol tables
- `type_extractor.rs`: Extract type information
- `dependency_graph.rs`: Build dependency relationships

**Key Traits**:
```rust
pub trait SemanticAnalyzer {
    fn analyze(&self, ast: &AST) -> Result<SemanticInfo, AnalysisError>;
}

pub trait SymbolResolver {
    fn resolve_symbols(&self, ast: &AST) -> Result<SymbolTable, ResolverError>;
}
```

### 3. Diff Engine (`crates/diff-engine`)

**Purpose**: Compare ASTs and detect changes

**Key Files**:
- `tree_edit.rs`: Zhang-Shasha tree edit distance algorithm
- `function_matcher.rs`: Match functions between file versions
- `similarity_scorer.rs`: Calculate similarity scores
- `changes.rs`: Classify and categorize changes
- `refactoring.rs`: Detect refactoring patterns

**Key Algorithms**:
- **Zhang-Shasha**: Tree edit distance with optimizations
- **Hungarian Algorithm**: Optimal bipartite matching
- **Similarity Scoring**: Multi-dimensional similarity calculation

### 4. CLI Interface (`crates/cli`)

**Purpose**: Command-line interface for the tool

**Key Features**:
- File and directory comparison
- Multiple output formats
- Configuration management
- Batch processing

### 5. Web Interface (`crates/web-ui` + `frontend/`)

**Purpose**: Web-based interface with REST API

**Backend** (Rust/Axum):
- REST API endpoints
- Request/response handling
- Integration with core components

**Frontend** (React/TypeScript):
- Interactive code visualization
- Multiple view modes
- Configuration management
- Real-time analysis

## Contributing Guidelines

### Code Style

**Rust Code**:
- Follow standard Rust formatting (`cargo fmt`)
- Use `clippy` for linting (`cargo clippy`)
- Write comprehensive documentation
- Include unit tests for all public functions

**TypeScript/React Code**:
- Use Prettier for formatting
- Follow ESLint rules
- Use TypeScript strict mode
- Write component tests

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test additions/changes
- `chore`: Maintenance tasks

**Examples**:
```
feat(parser): add support for TypeScript parsing
fix(diff-engine): correct similarity calculation for empty functions
docs(api): update OpenAPI specification
test(semantic): add integration tests for symbol resolution
```

### Pull Request Process

1. **Create descriptive PR title** following conventional commits
2. **Fill out PR template** with:
   - Description of changes
   - Testing performed
   - Breaking changes (if any)
   - Related issues
3. **Ensure CI passes**:
   - All tests pass
   - Code coverage maintained
   - Linting passes
   - Documentation builds
4. **Request review** from maintainers
5. **Address feedback** promptly
6. **Squash and merge** when approved

### Issue Reporting

**Bug Reports**:
- Use bug report template
- Include reproduction steps
- Provide sample code files
- Include system information

**Feature Requests**:
- Use feature request template
- Describe use case and motivation
- Provide examples if possible
- Consider implementation complexity

## Extension Development

### Adding Language Support

1. **Create language parser**:
```rust
// crates/parser/src/languages/your_language.rs
use tree_sitter_your_language;

pub struct YourLanguageParser;

impl LanguageParser for YourLanguageParser {
    fn parse(&self, content: &str) -> Result<AST, ParseError> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_your_language::language())?;
        
        let tree = parser.parse(content, None)
            .ok_or(ParseError::ParseFailed)?;
        
        ASTBuilder::new().build_ast(tree.root_node(), content)
    }
    
    fn language(&self) -> Language {
        Language::YourLanguage
    }
    
    fn file_extensions(&self) -> &[&str] {
        &[".your_ext"]
    }
}
```

2. **Register language**:
```rust
// crates/parser/src/language_detector.rs
impl LanguageDetector {
    pub fn new() -> Self {
        let mut parsers = HashMap::new();
        // ... existing parsers
        parsers.insert(Language::YourLanguage, Box::new(YourLanguageParser));
        
        Self { parsers }
    }
}
```

3. **Add tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_your_language_parsing() {
        let parser = YourLanguageParser;
        let code = "your sample code here";
        let ast = parser.parse(code).unwrap();
        
        assert_eq!(ast.root().node_type(), NodeType::Program);
        // Add more assertions
    }
}
```

### Adding Analysis Features

1. **Extend semantic analysis**:
```rust
// crates/semantic-analysis/src/analyzers/your_analyzer.rs
pub struct YourAnalyzer;

impl SemanticAnalyzer for YourAnalyzer {
    fn analyze(&self, ast: &AST) -> Result<SemanticInfo, AnalysisError> {
        // Your analysis logic
        Ok(SemanticInfo::new())
    }
}
```

2. **Add to analysis pipeline**:
```rust
// crates/semantic-analysis/src/lib.rs
impl SemanticEngine {
    pub fn analyze(&self, ast: &AST) -> Result<SemanticInfo, AnalysisError> {
        let mut info = SemanticInfo::new();
        
        // Existing analyzers
        info.merge(self.symbol_resolver.analyze(ast)?);
        info.merge(self.type_extractor.analyze(ast)?);
        
        // Your analyzer
        info.merge(YourAnalyzer.analyze(ast)?);
        
        Ok(info)
    }
}
```

### Adding Output Formats

1. **Implement formatter**:
```rust
// crates/cli/src/output/your_format.rs
pub struct YourFormatFormatter;

impl OutputFormatter for YourFormatFormatter {
    fn format(&self, result: &ComparisonResult) -> Result<String, FormatError> {
        // Your formatting logic
        Ok(formatted_output)
    }
    
    fn file_extension(&self) -> &str {
        ".your_ext"
    }
}
```

2. **Register formatter**:
```rust
// crates/cli/src/output/mod.rs
pub fn get_formatter(format: &str) -> Result<Box<dyn OutputFormatter>, FormatError> {
    match format {
        "text" => Ok(Box::new(TextFormatter)),
        "json" => Ok(Box::new(JsonFormatter)),
        "html" => Ok(Box::new(HtmlFormatter)),
        "xml" => Ok(Box::new(XmlFormatter)),
        "your_format" => Ok(Box::new(YourFormatFormatter)),
        _ => Err(FormatError::UnsupportedFormat(format.to_string())),
    }
}
```

## Testing Strategy

### Unit Tests

- **Location**: `src/` directories alongside source code
- **Naming**: `#[cfg(test)] mod tests`
- **Coverage**: Aim for >90% code coverage
- **Focus**: Individual function behavior

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_output);
    }
}
```

### Integration Tests

- **Location**: `tests/` directories in each crate
- **Focus**: Component interaction and end-to-end workflows

```rust
// crates/parser/tests/integration_test.rs
use smart_diff_parser::*;

#[test]
fn test_java_parsing_integration() {
    let detector = LanguageDetector::new();
    let engine = ParserEngine::new();
    
    let java_code = include_str!("fixtures/Calculator.java");
    let language = detector.detect_language("Calculator.java", java_code).unwrap();
    let ast = engine.parse(java_code, &language).unwrap();
    
    assert_eq!(language, Language::Java);
    assert!(ast.functions().len() > 0);
}
```

### End-to-End Tests

- **Location**: `tests/` directory at project root
- **Focus**: Complete user workflows

```rust
// tests/e2e/cli_tests.rs
use std::process::Command;

#[test]
fn test_cli_file_comparison() {
    let output = Command::new("./target/debug/smart-diff-cli")
        .args(&["compare", "tests/fixtures/old.java", "tests/fixtures/new.java"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Similarity:"));
}
```

### Performance Tests

- **Location**: `tests/performance/`
- **Focus**: Performance benchmarks and regression detection

```rust
// tests/performance/benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_parsing(c: &mut Criterion) {
    let large_file = include_str!("fixtures/large_file.java");
    let parser = ParserEngine::new();
    
    c.bench_function("parse large java file", |b| {
        b.iter(|| parser.parse(black_box(large_file), &Language::Java))
    });
}

criterion_group!(benches, benchmark_parsing);
criterion_main!(benches);
```

## Performance Considerations

### Optimization Strategies

1. **Caching**:
   - AST caching for repeated analysis
   - Symbol table caching
   - Similarity score caching

2. **Parallelization**:
   - Multi-threaded file processing
   - Parallel function matching
   - Concurrent analysis pipelines

3. **Memory Management**:
   - Streaming for large files
   - Memory-mapped file access
   - Efficient data structures

4. **Algorithmic Optimizations**:
   - Heuristic pruning in tree edit distance
   - Early termination conditions
   - Incremental analysis

### Profiling and Monitoring

**CPU Profiling**:
```bash
# Install profiling tools
cargo install flamegraph

# Generate flame graph
cargo flamegraph --bin smart-diff-cli -- compare large1.java large2.java
```

**Memory Profiling**:
```bash
# Use valgrind (Linux)
valgrind --tool=massif ./target/release/smart-diff-cli compare file1.java file2.java

# Use heaptrack (Linux)
heaptrack ./target/release/smart-diff-cli compare file1.java file2.java
```

**Benchmarking**:
```bash
# Run benchmarks
cargo bench

# Compare with baseline
cargo bench -- --save-baseline main
git checkout feature-branch
cargo bench -- --baseline main
```

For more detailed information on specific components, see the individual crate documentation and the [Architecture Documentation](architecture.md).
