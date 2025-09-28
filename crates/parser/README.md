# Smart Diff Parser

Multi-language parser engine for the Smart Code Diff tool, built on top of tree-sitter for robust and accurate parsing of source code.

## Features

- **Multi-language Support**: Java, Python, JavaScript, C++, C
- **Tree-sitter Integration**: Leverages tree-sitter parsers for accurate syntax analysis
- **Normalized AST**: Converts language-specific parse trees to a unified AST representation
- **Language Detection**: Automatic detection of programming language from file extensions and content
- **Error Handling**: Graceful handling of syntax errors with detailed error reporting
- **Extensible Architecture**: Easy to add support for new programming languages

## Supported Languages

| Language   | File Extensions | Status |
|------------|----------------|--------|
| Java       | .java          | ✅     |
| Python     | .py, .pyw      | ✅     |
| JavaScript | .js, .jsx      | ✅     |
| C++        | .cpp, .cc, .cxx, .c++, .hpp, .hxx, .h++ | ✅ |
| C          | .c, .h         | ✅     |

## Usage

### Basic Parsing

```rust
use smart_diff_parser::{
    tree_sitter::TreeSitterParser,
    language::Language,
    parser::Parser,
};

// Create a parser
let parser = TreeSitterParser::new()?;

// Parse Java code
let java_code = r#"
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
"#;

let result = parser.parse(java_code, Language::Java)?;
println!("Parsed {} with {} functions", 
         result.language, 
         result.ast.find_by_type(&NodeType::Function).len());
```

### File Parsing with Auto-detection

```rust
use smart_diff_parser::{
    tree_sitter::TreeSitterParser,
    parser::Parser,
};

let parser = TreeSitterParser::new()?;

// Language is automatically detected from file extension
let result = parser.parse_file("src/main.java")?;
println!("Detected language: {:?}", result.language);
```

### Working with AST

```rust
use smart_diff_parser::ast::NodeType;

// Find all functions in the AST
let functions = result.ast.find_by_type(&NodeType::Function);
for func in functions {
    if let Some(name) = func.metadata.attributes.get("name") {
        println!("Found function: {} at line {}", name, func.metadata.line);
    }
}

// Find all classes
let classes = result.ast.find_by_type(&NodeType::Class);
for class in classes {
    if let Some(name) = class.metadata.attributes.get("name") {
        println!("Found class: {} at line {}", name, class.metadata.line);
    }
}
```

### Language Detection

The language detector uses sophisticated pattern matching with confidence scoring to accurately identify programming languages from both file extensions and content analysis.

#### Basic Detection

```rust
use smart_diff_parser::language::LanguageDetector;

// Detect from file path
let lang = LanguageDetector::detect_from_path("Calculator.java");
assert_eq!(lang, Language::Java);

// Detect from content using pattern analysis
let java_content = r#"
public class Calculator {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
"#;
let lang = LanguageDetector::detect_from_content(java_content);
assert_eq!(lang, Language::Java);

// Combined detection (path + content) - most accurate
let lang = LanguageDetector::detect("Calculator.java", java_content);
assert_eq!(lang, Language::Java);
```

#### Advanced Content Detection

The content-based detection uses weighted pattern matching with language-specific indicators:

**Java Detection Patterns:**
- Strong indicators: `public class`, `System.out.println`, `public static void main`
- Medium indicators: `import java.*`, `@Override`, generics syntax
- Weak indicators: `final`, `static`, `.length`

**Python Detection Patterns:**
- Strong indicators: `def function():`, `class Name:`, `if __name__ == "__main__"`
- Medium indicators: `import`, `self.`, `True/False/None`
- Indentation analysis for Python-style code blocks

**JavaScript Detection Patterns:**
- Strong indicators: `function`, `const/let/var`, `console.log`, arrow functions `=>`
- Medium indicators: `require()`, `module.exports`, `async/await`
- Template literals `${}`

**C++ Detection Patterns:**
- Strong indicators: `#include <iostream>`, `std::cout`, `class` with access specifiers
- Medium indicators: `template<>`, `namespace`, `virtual/override`
- C++ specific syntax: `::`, `nullptr`, `auto`

**C Detection Patterns:**
- Strong indicators: `#include <stdio.h>`, `printf()`, `malloc/free`
- Medium indicators: `struct`, `typedef`, pointer syntax
- C-specific headers and functions

#### Confidence Scoring

Each pattern has a weight, and the language with the highest total score (above 0.3 threshold) is selected:

```rust
// Example: Mixed content detection
let mixed_content = r#"
#include <iostream>  // C++ indicator (0.9)
int main() {         // Weak indicator (0.2)
    std::cout << "Hello" << std::endl;  // Strong C++ (0.8)
    return 0;
}
"#;

// Total C++ score: 0.9 + 0.2 + 0.8 = 1.9
// Result: Language::Cpp
let detected = LanguageDetector::detect_from_content(mixed_content);
```

#### Handling Edge Cases

- **File extension priority**: When content is ambiguous, file extension provides the hint
- **Minimum confidence**: Requires score > 0.3 to avoid false positives
- **Penalty system**: Reduces score when conflicting language patterns are found
- **Fallback**: Returns `Language::Unknown` when confidence is too low

## AST Structure

The parser converts language-specific parse trees into a normalized AST with the following node types:

### Program Structure
- `Program`: Root node of the AST
- `Module`: Package/import declarations
- `Class`: Class definitions
- `Interface`: Interface definitions

### Functions and Methods
- `Function`: Function declarations/definitions
- `Method`: Class method declarations
- `Constructor`: Constructor methods

### Statements
- `Block`: Code blocks
- `IfStatement`: Conditional statements
- `WhileLoop`: While loops
- `ForLoop`: For loops
- `ReturnStatement`: Return statements
- `ExpressionStatement`: Expression statements

### Expressions
- `BinaryExpression`: Binary operations (a + b)
- `UnaryExpression`: Unary operations (-a)
- `CallExpression`: Function/method calls
- `AssignmentExpression`: Variable assignments
- `Identifier`: Variable/function names
- `Literal`: String, number, boolean literals

### Declarations
- `VariableDeclaration`: Variable declarations
- `ParameterDeclaration`: Function parameters
- `FieldDeclaration`: Class fields

## Node Metadata

Each AST node includes metadata:

```rust
pub struct NodeMetadata {
    pub line: usize,           // Line number (1-based)
    pub column: usize,         // Column number (1-based)
    pub original_text: String, // Original source text
    pub attributes: HashMap<String, String>, // Node-specific attributes
}
```

Common attributes:
- `name`: Identifier name for functions, classes, variables
- `function_name`: Function name for call expressions
- `param_count`: Number of parameters for functions
- `return_type`: Return type for functions (when available)
- `type`: Variable type for declarations

## Error Handling

The parser provides detailed error information:

```rust
let result = parser.parse(invalid_code, Language::Java)?;

if !result.errors.is_empty() {
    println!("Parse errors found:");
    for error in &result.errors {
        println!("  - {}", error);
    }
}
```

## Examples

### Parsing Demo

Run the comprehensive parsing demo:

```bash
cargo run --example parse_demo
```

This demonstrates parsing of all supported languages with sample code, showing:
- AST generation and node extraction
- Function and class detection
- Error handling for invalid syntax
- Node attribute extraction

### Language Detection Demo

Run the language detection demo:

```bash
cargo run --example language_detection_demo
```

This demonstrates the sophisticated language detection capabilities:
- File extension-based detection
- Content-based pattern matching with confidence scoring
- Combined detection strategies
- Edge case handling (mixed content, ambiguous code, etc.)

## Testing

Run the test suite:

```bash
cargo test
```

The tests cover:
- Language detection
- Parser creation and initialization
- Parsing of sample code in all supported languages
- AST node extraction and attribute checking
- Error handling for invalid syntax

## Architecture

The parser is built with a modular architecture:

- `language.rs`: Language detection and configuration
- `language_config.rs`: Language-specific parsing configurations
- `tree_sitter.rs`: Tree-sitter integration and AST conversion
- `ast.rs`: AST node definitions and utilities
- `parser.rs`: Main parser interface and error types

## Adding New Languages

To add support for a new language:

1. Add the language to the `Language` enum in `language.rs`
2. Add tree-sitter dependency to `Cargo.toml`
3. Update `LANGUAGE_CONFIGS` in `language_config.rs`
4. Add language-specific node mappings
5. Add tests for the new language

## Dependencies

- `tree-sitter`: Core parsing engine
- `tree-sitter-java`: Java grammar
- `tree-sitter-python`: Python grammar
- `tree-sitter-javascript`: JavaScript grammar
- `tree-sitter-cpp`: C++ grammar
- `tree-sitter-c`: C grammar
- `serde`: Serialization support
- `uuid`: Unique node identifiers
- `regex`: Pattern matching for language detection
