# Smart Code Diff

A next-generation code diffing tool that performs structural and semantic comparison of source code files. Unlike traditional line-based diff tools, Smart Code Diff understands code structure at the Abstract Syntax Tree (AST) level, enabling intelligent comparison of functions, classes, and other code elements regardless of their position in files.

## Features

- **Multi-Language Support**: Support for Java, Python, JavaScript, C++, C#, and more
- **Structural Comparison**: Compare code at function/method level, ignoring order and formatting
- **Semantic Understanding**: Identify renamed identifiers, moved code blocks, and refactoring patterns
- **Cross-File Tracking**: Track code moved between files
- **Multiple Interfaces**: Command-line tool, web interface, and REST API
- **Customizable Rules**: User-defined comparison rules and similarity thresholds

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/smart-code-diff.git
cd smart-code-diff

# Build the project
cargo build --release
```

### Usage

#### Command Line

```bash
# Compare two files
smart-diff compare file1.py file2.py

# Compare directories
smart-diff compare --recursive src1/ src2/

# Output in JSON format
smart-diff compare --format json file1.js file2.js
```

#### Web Interface

```bash
# Start the web server
smart-diff-server

# Open http://localhost:3000 in your browser
```

## Architecture

The project is organized as a Rust workspace with the following crates:

- **parser**: Multi-language parser engine using tree-sitter
- **semantic-analysis**: Symbol resolution and type information extraction
- **diff-engine**: Core diff computation with tree edit distance algorithms
- **cli**: Command-line interface
- **web-ui**: Web server and REST API

## Development

### Prerequisites

- Rust 1.70+
- Node.js 18+ (for web UI development)

### Building

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
