# Smart Code Diff Examples

This directory contains practical examples and use cases for the Smart Code Diff tool.

## Directory Structure

```
examples/
├── README.md                    # This file
├── cli/                        # Command-line interface examples
│   ├── basic-comparison.sh     # Basic file comparison
│   ├── directory-analysis.sh   # Directory comparison
│   ├── batch-processing.sh     # Batch file processing
│   └── ci-integration.sh       # CI/CD integration
├── api/                        # API integration examples
│   ├── javascript/             # Node.js examples
│   ├── python/                 # Python examples
│   ├── curl/                   # cURL examples
│   └── postman/                # Postman collection
├── web/                        # Web interface examples
│   ├── screenshots/            # UI screenshots
│   └── workflows/              # Common workflows
├── sample-code/                # Sample code files for testing
│   ├── java/                   # Java examples
│   ├── python/                 # Python examples
│   ├── javascript/             # JavaScript examples
│   ├── cpp/                    # C++ examples
│   └── c/                      # C examples
├── configurations/             # Configuration examples
│   ├── basic-config.toml       # Basic configuration
│   ├── enterprise-config.toml  # Enterprise settings
│   └── ci-config.toml          # CI/CD optimized
└── use-cases/                  # Real-world use cases
    ├── code-review/            # Code review scenarios
    ├── refactoring/            # Refactoring analysis
    ├── migration/              # Code migration
    └── quality-assessment/     # Code quality analysis
```

## Quick Start Examples

### 1. Basic File Comparison

Compare two Java files to see structural changes:

```bash
# Navigate to examples directory
cd examples/sample-code/java

# Compare original and refactored versions
smart-diff-cli compare Calculator.java Calculator_refactored.java
```

### 2. API Integration

Use the REST API to compare files programmatically:

```bash
# Start the server
smart-diff-server &

# Compare files via API
curl -X POST http://localhost:3000/api/compare \
  -H "Content-Type: application/json" \
  -d @examples/api/curl/compare-request.json
```

### 3. Web Interface

1. Start the web server: `smart-diff-server`
2. Open http://localhost:3000
3. Upload files from `examples/sample-code/`
4. Explore different visualization modes

## Example Categories

### CLI Examples

- **[basic-comparison.sh](cli/basic-comparison.sh)**: Simple file comparison
- **[directory-analysis.sh](cli/directory-analysis.sh)**: Recursive directory comparison
- **[batch-processing.sh](cli/batch-processing.sh)**: Processing multiple files
- **[ci-integration.sh](cli/ci-integration.sh)**: CI/CD pipeline integration

### API Examples

- **[JavaScript/Node.js](api/javascript/)**: Complete client implementation
- **[Python](api/python/)**: Python integration examples
- **[cURL](api/curl/)**: Raw HTTP requests
- **[Postman](api/postman/)**: Postman collection for testing

### Real-World Use Cases

- **[Code Review](use-cases/code-review/)**: Pull request analysis
- **[Refactoring](use-cases/refactoring/)**: Large-scale refactoring analysis
- **[Migration](use-cases/migration/)**: Language/framework migration
- **[Quality Assessment](use-cases/quality-assessment/)**: Code quality metrics

## Sample Code Files

The `sample-code/` directory contains example files in different languages that demonstrate various scenarios:

### Java Examples
- `Calculator.java` / `Calculator_refactored.java` - Method extraction refactoring
- `UserService.java` / `UserService_v2.java` - Interface changes
- `DataProcessor.java` / `DataProcessor_optimized.java` - Performance optimization

### Python Examples
- `math_utils.py` / `math_utils_enhanced.py` - Function additions
- `data_analyzer.py` / `data_analyzer_refactored.py` - Class restructuring
- `api_client.py` / `api_client_v2.py` - API version migration

### JavaScript Examples
- `calculator.js` / `calculator.es6.js` - ES6 migration
- `user-manager.js` / `user-manager-async.js` - Async/await conversion
- `utils.js` / `utils.modular.js` - Modularization

### C++ Examples
- `matrix.cpp` / `matrix_optimized.cpp` - Algorithm optimization
- `string_utils.cpp` / `string_utils_modern.cpp` - Modern C++ features
- `network.cpp` / `network_refactored.cpp` - Architecture changes

### C Examples
- `linked_list.c` / `linked_list_improved.c` - Memory management improvements
- `parser.c` / `parser_enhanced.c` - Feature additions
- `crypto.c` / `crypto_secure.c` - Security improvements

## Configuration Examples

### Basic Configuration (`configurations/basic-config.toml`)

```toml
[parser]
max_file_size = 5242880  # 5MB
parse_timeout = 15
enable_error_recovery = true

[semantic]
max_resolution_depth = 8
enable_cross_file_analysis = false
symbol_cache_size = 500

[diff_engine]
default_similarity_threshold = 0.7
enable_refactoring_detection = true
enable_cross_file_tracking = false
max_tree_depth = 15

[output]
default_format = "text"
enable_colors = true
include_timestamps = false
```

### Enterprise Configuration (`configurations/enterprise-config.toml`)

```toml
[parser]
max_file_size = 52428800  # 50MB
parse_timeout = 120
enable_error_recovery = true

[semantic]
max_resolution_depth = 15
enable_cross_file_analysis = true
symbol_cache_size = 5000

[diff_engine]
default_similarity_threshold = 0.6
enable_refactoring_detection = true
enable_cross_file_tracking = true
max_tree_depth = 25

[output]
default_format = "json"
enable_colors = false
include_timestamps = true

[performance]
parallel_workers = 8
memory_limit_mb = 4096
cache_size_mb = 1024
```

## Running Examples

### Prerequisites

1. Build the Smart Code Diff tool:
```bash
cargo build --release
```

2. Ensure the binaries are in your PATH or use full paths:
```bash
export PATH=$PATH:./target/release
```

### CLI Examples

```bash
# Run basic comparison example
./examples/cli/basic-comparison.sh

# Run directory analysis
./examples/cli/directory-analysis.sh

# Run batch processing
./examples/cli/batch-processing.sh
```

### API Examples

```bash
# Start the server
smart-diff-server &

# Run JavaScript examples
cd examples/api/javascript
npm install
node basic-comparison.js

# Run Python examples
cd examples/api/python
pip install -r requirements.txt
python basic_comparison.py
```

### Web Interface Examples

1. Start the web server:
```bash
smart-diff-server
```

2. Open http://localhost:3000

3. Follow the workflows in `examples/web/workflows/`

## Contributing Examples

We welcome contributions of new examples! Please follow these guidelines:

1. **Create clear, focused examples** that demonstrate specific features
2. **Include documentation** explaining the example's purpose
3. **Provide sample data** when needed
4. **Test examples** before submitting
5. **Follow naming conventions** used in existing examples

### Example Template

When creating new examples, use this template:

```bash
#!/bin/bash
# Example: [Brief Description]
# Purpose: [Detailed explanation of what this example demonstrates]
# Usage: ./example-name.sh

set -e

echo "Smart Code Diff Example: [Example Name]"
echo "======================================="

# Setup
echo "Setting up example..."
# Setup code here

# Main example
echo "Running analysis..."
# Main example code here

# Results
echo "Results:"
# Display or process results

echo "Example completed successfully!"
```

## Troubleshooting Examples

If you encounter issues running examples:

1. **Check prerequisites**: Ensure all tools are installed and in PATH
2. **Verify file paths**: Make sure sample files exist
3. **Check permissions**: Ensure scripts are executable
4. **Review logs**: Check output for error messages
5. **Update examples**: Pull latest changes from repository

## Additional Resources

- [User Guide](../docs/user-guide.md) - Comprehensive usage documentation
- [API Documentation](../docs/api/) - REST API reference
- [Developer Guide](../docs/developer-guide.md) - Development and contribution guide
- [Configuration Reference](../docs/configuration.md) - Detailed configuration options

For questions or issues with examples, please:
- Check existing GitHub Issues
- Create a new issue with the "examples" label
- Join our GitHub Discussions for community support
