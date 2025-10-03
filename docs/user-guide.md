# Smart Code Diff User Guide

Welcome to Smart Code Diff, a next-generation code comparison tool that goes beyond traditional line-by-line diffs to provide structural and semantic analysis of your source code.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Command Line Interface](#command-line-interface)
- [Web Interface](#web-interface)
- [Configuration](#configuration)
- [Common Use Cases](#common-use-cases)
- [Troubleshooting](#troubleshooting)

## Overview

Smart Code Diff analyzes code at multiple levels:

- **Structural Analysis**: Compares Abstract Syntax Trees (AST) rather than just text
- **Semantic Understanding**: Resolves symbols and understands code context
- **Function Matching**: Intelligently matches functions across file versions
- **Refactoring Detection**: Identifies common refactoring patterns automatically
- **Cross-File Tracking**: Tracks functions moved between files

### Supported Languages

- Java
- Python
- JavaScript
- C++
- C

## Installation

### Prerequisites

- Rust 1.70+ (for building from source)
- Node.js 18+ (for web interface)

### From Source

```bash
# Clone the repository
git clone https://github.com/opensensor/smartdiff.git
cd smartdiff

# Build the CLI tool
cargo build --release -p smart-diff-cli

# Build the web server
cargo build --release -p smart-diff-web

# Install the frontend dependencies
cd frontend
npm install
npm run build
```


## Quick Start

### CLI Usage

Compare two files:
```bash
./target/release/smart-diff-cli compare file1.java file2.java
```

Compare directories:
```bash
./target/release/smart-diff-cli compare-dir src/ src-new/
```

Analyze multiple files:
```bash
./target/release/smart-diff-cli analyze src/*.java
```

### Web Interface

1. Start the web server:
```bash
./target/release/smart-diff-server
```

2. Open your browser to `http://localhost:3000`

3. Use the web interface to:
   - Upload and compare files
   - Analyze multiple files
   - Configure system settings
   - View interactive visualizations

## Command Line Interface

The CLI provides comprehensive options for code analysis and comparison.

### Basic Commands

#### File Comparison

```bash
smart-diff-cli compare [OPTIONS] <FILE1> <FILE2>
```

**Options:**
- `--threshold <FLOAT>`: Similarity threshold (0.0-1.0, default: 0.7)
- `--output <FORMAT>`: Output format (text, json, html, xml)
- `--ignore-whitespace`: Ignore whitespace changes
- `--detect-moves`: Enable cross-file move detection
- `--language <LANG>`: Force language detection

**Examples:**

```bash
# Basic comparison
smart-diff-cli compare Calculator.java Calculator_v2.java

# JSON output with custom threshold
smart-diff-cli compare --output json --threshold 0.8 old.py new.py

# Ignore whitespace changes
smart-diff-cli compare --ignore-whitespace format_old.js format_new.js

# Force language detection
smart-diff-cli compare --language cpp header.h header_new.h
```

#### Directory Comparison

```bash
smart-diff-cli compare-dir [OPTIONS] <DIR1> <DIR2>
```

**Options:**
- `--recursive`: Compare directories recursively
- `--pattern <GLOB>`: File pattern to match (e.g., "*.java")
- `--exclude <PATTERN>`: Exclude files matching pattern
- `--parallel <N>`: Number of parallel workers

**Examples:**

```bash
# Compare all Java files in directories
smart-diff-cli compare-dir --pattern "*.java" src/ src-refactored/

# Recursive comparison excluding test files
smart-diff-cli compare-dir --recursive --exclude "*test*" project-v1/ project-v2/

# Parallel processing with 4 workers
smart-diff-cli compare-dir --parallel 4 large-project-old/ large-project-new/
```

#### Multi-File Analysis

```bash
smart-diff-cli analyze [OPTIONS] <FILES>...
```

**Options:**
- `--complexity`: Include complexity analysis
- `--dependencies`: Analyze dependencies
- `--duplicates`: Detect duplicate functions
- `--output-dir <DIR>`: Output directory for reports

**Examples:**

```bash
# Analyze all Python files with complexity
smart-diff-cli analyze --complexity src/*.py

# Full analysis with dependency detection
smart-diff-cli analyze --complexity --dependencies --duplicates src/**/*.java

# Generate reports in specific directory
smart-diff-cli analyze --output-dir reports/ src/*.cpp
```

### Output Formats

#### Text Output (Default)

```
File Comparison: Calculator.java → Calculator.java
Language: java
Overall Similarity: 87.5%

Function Analysis:
├── add (100% match) - unchanged
├── multiply (100% match) - unchanged  
├── isEven → isNumberEven (75% match) - renamed
└── subtract (new) - added

Changes Detected:
├── Function renamed: isEven → isNumberEven
├── Method extracted: checkEvenness
└── Function added: subtract

Refactoring Patterns:
└── Extract Method (92% confidence)
    └── Logic extracted from isNumberEven to checkEvenness
```

#### JSON Output

```json
{
  "similarity": 0.875,
  "analysis": {
    "files": {
      "source": {
        "path": "Calculator.java",
        "lines": 25,
        "functions": 3,
        "classes": 1,
        "complexity": 4.2
      },
      "target": {
        "path": "Calculator.java", 
        "lines": 32,
        "functions": 4,
        "classes": 1,
        "complexity": 5.1
      },
      "language": "java",
      "similarity": {
        "overall": 0.875,
        "structure": 0.90,
        "content": 0.85,
        "semantic": 0.88
      }
    },
    "functions": {
      "total_functions": 4,
      "matched_functions": 3,
      "average_similarity": 0.92
    }
  }
}
```

#### HTML Output

Generates an interactive HTML report with:
- Side-by-side code comparison
- Function-level analysis
- Change visualization
- Refactoring pattern detection
- Similarity metrics

### Configuration

#### Global Configuration

Create a configuration file at `~/.smart-diff/config.toml`:

```toml
[parser]
max_file_size = 10485760  # 10MB
parse_timeout = 30
enable_error_recovery = true

[semantic]
max_resolution_depth = 10
enable_cross_file_analysis = true
symbol_cache_size = 1000

[diff_engine]
default_similarity_threshold = 0.7
enable_refactoring_detection = true
enable_cross_file_tracking = true
max_tree_depth = 20

[output]
default_format = "text"
enable_colors = true
include_timestamps = false

[ui]
theme = "light"
show_line_numbers = true
enable_syntax_highlighting = true
```

#### Project-Specific Configuration

Create `.smart-diff.toml` in your project root:

```toml
# Project-specific settings
[project]
name = "My Project"
language_priority = ["java", "python"]
exclude_patterns = ["**/test/**", "**/*.generated.*"]

[analysis]
complexity_threshold = 10
duplicate_threshold = 0.9
min_function_lines = 3

[reporting]
output_dir = "diff-reports"
include_metrics = true
generate_charts = true
```

## Web Interface

The web interface provides an intuitive way to analyze code differences with rich visualizations.

### Features

#### 1. File Comparison
- **Drag-and-drop upload**: Easy file selection
- **Side-by-side view**: Synchronized code comparison
- **Unified diff view**: Traditional diff format
- **Structure view**: AST-level comparison
- **Function-centric view**: Detailed function analysis

#### 2. Multi-File Analysis
- **Batch upload**: Analyze multiple files at once
- **Cross-file detection**: Find moved and duplicate functions
- **Dependency analysis**: Visualize code dependencies
- **Complexity metrics**: Code quality assessment

#### 3. Interactive Features
- **Zoom and navigation**: Easy code exploration
- **Change filtering**: Focus on specific change types
- **Search functionality**: Find specific functions or changes
- **Export options**: Save results in various formats

#### 4. Configuration Management
- **Real-time settings**: Update configuration on the fly
- **Component status**: Monitor system health
- **Performance metrics**: Track analysis performance

### Navigation Guide

#### Home Page
- Overview of features and capabilities
- Quick access to main functions
- Recent analysis history

#### Compare Page
1. **Upload Files**: Drag and drop or click to select files
2. **Configure Options**: Set similarity threshold and analysis options
3. **View Results**: Explore different visualization modes
4. **Export Results**: Download analysis in preferred format

#### Analyze Page
1. **Select Multiple Files**: Upload files for batch analysis
2. **Choose Analysis Type**: Select complexity, dependencies, or signatures
3. **Review Results**: Examine cross-file analysis and metrics
4. **Generate Reports**: Create comprehensive analysis reports

#### Settings Page
- **Parser Configuration**: File size limits, timeouts, error recovery
- **Semantic Analysis**: Resolution depth, cross-file analysis, caching
- **Diff Engine**: Similarity thresholds, refactoring detection
- **Output Options**: Formats, colors, timestamps
- **UI Preferences**: Theme, line numbers, syntax highlighting

## Common Use Cases

### 1. Code Review

**Scenario**: Reviewing changes in a pull request

```bash
# Compare branch changes
git diff --name-only main..feature-branch | xargs smart-diff-cli analyze

# Detailed comparison of specific files
smart-diff-cli compare --output html main:Calculator.java feature:Calculator.java
```

**Web Interface**: Upload old and new versions, use side-by-side view to review changes with function-level analysis.

### 2. Refactoring Analysis

**Scenario**: Understanding the impact of a large refactoring

```bash
# Analyze refactoring patterns
smart-diff-cli compare-dir --recursive --output json before/ after/ > refactoring-report.json

# Focus on moved functions
smart-diff-cli compare-dir --detect-moves src-old/ src-new/
```

**Web Interface**: Use function-centric view to see refactoring patterns with confidence scores.

### 3. Code Quality Assessment

**Scenario**: Assessing code quality across a project

```bash
# Comprehensive analysis with complexity metrics
smart-diff-cli analyze --complexity --dependencies --duplicates src/**/*.java

# Generate quality report
smart-diff-cli analyze --output-dir quality-report/ --complexity src/
```

**Web Interface**: Use analyze page to upload multiple files and review complexity distribution and duplicate detection.

### 4. Migration Analysis

**Scenario**: Analyzing code migration between languages or frameworks

```bash
# Compare similar functionality across languages
smart-diff-cli compare --threshold 0.6 Calculator.java Calculator.py

# Analyze architectural changes
smart-diff-cli compare-dir --pattern "*.java" old-architecture/ new-architecture/
```

### 5. Merge Conflict Resolution

**Scenario**: Understanding complex merge conflicts

```bash
# Analyze three-way merge
smart-diff-cli compare base.java ours.java
smart-diff-cli compare base.java theirs.java
smart-diff-cli compare ours.java theirs.java
```

**Web Interface**: Compare different versions to understand the nature of conflicts and make informed merge decisions.

### 6. Documentation Generation

**Scenario**: Generating change documentation

```bash
# Generate HTML documentation of changes
smart-diff-cli compare-dir --output html --output-dir docs/ v1.0/ v2.0/

# Create JSON report for further processing
smart-diff-cli compare --output json old.py new.py | jq '.analysis.changes'
```

## Troubleshooting

### Common Issues

#### 1. Parser Errors

**Problem**: "Unsupported language" or parsing failures

**Solutions**:
- Verify file extension matches content
- Use `--language` flag to force language detection
- Check if file contains syntax errors
- Increase `parse_timeout` in configuration

```bash
# Force language detection
smart-diff-cli compare --language java MyClass.txt MyClass2.txt

# Increase timeout
smart-diff-cli compare --config parser.parse_timeout=60 large_file.cpp large_file2.cpp
```

#### 2. Memory Issues

**Problem**: Out of memory errors with large files

**Solutions**:
- Reduce `max_file_size` setting
- Use `--parallel 1` to reduce memory usage
- Split large files into smaller chunks
- Increase system memory limits

```bash
# Reduce parallel processing
smart-diff-cli compare-dir --parallel 1 large-project-1/ large-project-2/

# Set memory limits
export RUST_MAX_STACK=8388608
smart-diff-cli compare huge_file1.js huge_file2.js
```

#### 3. Performance Issues

**Problem**: Slow analysis on large codebases

**Solutions**:
- Use appropriate similarity thresholds
- Enable parallel processing
- Exclude unnecessary files
- Use incremental analysis

```bash
# Optimize for speed
smart-diff-cli compare-dir --threshold 0.8 --parallel 8 --exclude "*test*" src1/ src2/

# Incremental analysis
smart-diff-cli compare-dir --pattern "*.java" --exclude "generated/**" src/ src-new/
```

#### 4. Web Interface Issues

**Problem**: Web interface not loading or slow

**Solutions**:
- Check server logs for errors
- Verify port 3000 is available
- Clear browser cache
- Check network connectivity

```bash
# Check server status
curl http://localhost:3000/api/health

# View server logs
./target/release/smart-diff-server 2>&1 | tee server.log

# Run with debug logging
RUST_LOG=debug ./target/release/smart-diff-server
```

### Getting Help

- **Documentation**: Check the [docs/](../docs/) directory
- **Examples**: See [examples/](../examples/) for usage patterns
- **Issues**: Report bugs on GitHub Issues
- **Discussions**: Join GitHub Discussions for questions

### Performance Tips

1. **Use appropriate thresholds**: Higher thresholds (0.8+) are faster
2. **Enable parallel processing**: Use `--parallel` for large datasets
3. **Filter files**: Use patterns to exclude unnecessary files
4. **Cache results**: Reuse analysis results when possible
5. **Monitor memory**: Watch memory usage with large files

### Best Practices

1. **Start small**: Test with small files before large projects
2. **Use version control**: Track configuration changes
3. **Regular updates**: Keep the tool updated for best performance
4. **Backup results**: Save important analysis results
5. **Document workflows**: Create scripts for common analysis tasks

For more detailed information, see the [API Documentation](api/integration-guide.md) and [Developer Guide](developer-guide.md).
