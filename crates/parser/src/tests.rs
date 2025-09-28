//! Tests for the parser module

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_sitter::TreeSitterParser;
    use crate::language::Language;
    use crate::parser::Parser;

    #[test]
    fn test_language_detection_from_path() {
        use crate::language::LanguageDetector;

        // Test file extension detection
        assert_eq!(LanguageDetector::detect_from_path("test.java"), Language::Java);
        assert_eq!(LanguageDetector::detect_from_path("test.py"), Language::Python);
        assert_eq!(LanguageDetector::detect_from_path("test.pyw"), Language::Python);
        assert_eq!(LanguageDetector::detect_from_path("test.js"), Language::JavaScript);
        assert_eq!(LanguageDetector::detect_from_path("test.jsx"), Language::JavaScript);
        assert_eq!(LanguageDetector::detect_from_path("test.cpp"), Language::Cpp);
        assert_eq!(LanguageDetector::detect_from_path("test.cc"), Language::Cpp);
        assert_eq!(LanguageDetector::detect_from_path("test.cxx"), Language::Cpp);
        assert_eq!(LanguageDetector::detect_from_path("test.hpp"), Language::Cpp);
        assert_eq!(LanguageDetector::detect_from_path("test.c"), Language::C);
        assert_eq!(LanguageDetector::detect_from_path("test.h"), Language::C);
        assert_eq!(LanguageDetector::detect_from_path("test.unknown"), Language::Unknown);
    }

    #[test]
    fn test_language_detection_from_content() {
        use crate::language::LanguageDetector;

        // Test Java detection
        let java_code = r#"
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
"#;
        assert_eq!(LanguageDetector::detect_from_content(java_code), Language::Java);

        // Test Python detection
        let python_code = r#"
def hello_world():
    print("Hello, World!")

class MyClass:
    def __init__(self):
        self.value = None

if __name__ == "__main__":
    hello_world()
"#;
        assert_eq!(LanguageDetector::detect_from_content(python_code), Language::Python);

        // Test JavaScript detection
        let js_code = r#"
function greet(name) {
    console.log(`Hello, ${name}!`);
}

const person = {
    name: "World",
    greet: () => {
        console.log("Hello!");
    }
};
"#;
        assert_eq!(LanguageDetector::detect_from_content(js_code), Language::JavaScript);

        // Test C++ detection
        let cpp_code = r#"
#include <iostream>
#include <vector>

class Calculator {
public:
    int add(int a, int b) {
        return a + b;
    }
};

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
"#;
        assert_eq!(LanguageDetector::detect_from_content(cpp_code), Language::Cpp);

        // Test C detection
        let c_code = r#"
#include <stdio.h>
#include <stdlib.h>

struct Point {
    int x;
    int y;
};

int main() {
    printf("Hello, World!\n");
    return 0;
}
"#;
        assert_eq!(LanguageDetector::detect_from_content(c_code), Language::C);
    }

    #[test]
    fn test_combined_language_detection() {
        use crate::language::LanguageDetector;

        // Test that file extension takes precedence when content is ambiguous
        let ambiguous_code = r#"
// This could be C or C++
int main() {
    return 0;
}
"#;

        // Should detect as C++ based on file extension
        assert_eq!(LanguageDetector::detect("test.cpp", ambiguous_code), Language::Cpp);

        // Should detect as C based on file extension
        assert_eq!(LanguageDetector::detect("test.c", ambiguous_code), Language::C);

        // Test content overrides when very confident
        let clear_python = r#"
def hello():
    print("Hello, World!")

if __name__ == "__main__":
    hello()
"#;

        // Even with wrong extension, should detect Python due to strong content signals
        assert_eq!(LanguageDetector::detect("test.txt", clear_python), Language::Python);
    }

    #[test]
    fn test_tree_sitter_parser_creation() {
        let parser = TreeSitterParser::new();
        assert!(parser.is_ok(), "TreeSitterParser should be created successfully");
        
        let parser = parser.unwrap();
        let supported = parser.supported_languages();
        assert!(supported.contains(&Language::Java));
        assert!(supported.contains(&Language::Python));
        assert!(supported.contains(&Language::JavaScript));
        assert!(supported.contains(&Language::Cpp));
        assert!(supported.contains(&Language::C));
    }

    #[test]
    fn test_java_parsing() {
        let parser = TreeSitterParser::new().expect("Failed to create parser");
        
        let java_code = r#"
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
    
    private int calculate(int a, int b) {
        return a + b;
    }
}
"#;
        
        let result = parser.parse(java_code, Language::Java);
        assert!(result.is_ok(), "Java parsing should succeed");
        
        let parse_result = result.unwrap();
        assert_eq!(parse_result.language, Language::Java);
        assert_eq!(parse_result.ast.node_type, crate::ast::NodeType::Program);
        
        // Check that we found class and method nodes
        let class_nodes = parse_result.ast.find_by_type(&crate::ast::NodeType::Class);
        assert!(!class_nodes.is_empty(), "Should find class nodes");
        
        let function_nodes = parse_result.ast.find_by_type(&crate::ast::NodeType::Function);
        assert!(!function_nodes.is_empty(), "Should find function nodes");
    }

    #[test]
    fn test_python_parsing() {
        let parser = TreeSitterParser::new().expect("Failed to create parser");
        
        let python_code = r#"
def hello_world():
    print("Hello, World!")

class Calculator:
    def __init__(self):
        self.value = 0
    
    def add(self, x, y):
        return x + y

if __name__ == "__main__":
    hello_world()
"#;
        
        let result = parser.parse(python_code, Language::Python);
        assert!(result.is_ok(), "Python parsing should succeed");
        
        let parse_result = result.unwrap();
        assert_eq!(parse_result.language, Language::Python);
        
        // Check for function and class nodes
        let function_nodes = parse_result.ast.find_by_type(&crate::ast::NodeType::Function);
        assert!(!function_nodes.is_empty(), "Should find function nodes");
        
        let class_nodes = parse_result.ast.find_by_type(&crate::ast::NodeType::Class);
        assert!(!class_nodes.is_empty(), "Should find class nodes");
    }

    #[test]
    fn test_javascript_parsing() {
        let parser = TreeSitterParser::new().expect("Failed to create parser");
        
        let js_code = r#"
function greet(name) {
    console.log(`Hello, ${name}!`);
}

class Person {
    constructor(name) {
        this.name = name;
    }
    
    sayHello() {
        greet(this.name);
    }
}

const person = new Person("World");
person.sayHello();
"#;
        
        let result = parser.parse(js_code, Language::JavaScript);
        assert!(result.is_ok(), "JavaScript parsing should succeed");
        
        let parse_result = result.unwrap();
        assert_eq!(parse_result.language, Language::JavaScript);
        
        // Check for function and class nodes
        let function_nodes = parse_result.ast.find_by_type(&crate::ast::NodeType::Function);
        assert!(!function_nodes.is_empty(), "Should find function nodes");
        
        let class_nodes = parse_result.ast.find_by_type(&crate::ast::NodeType::Class);
        assert!(!class_nodes.is_empty(), "Should find class nodes");
    }

    #[test]
    fn test_c_parsing() {
        let parser = TreeSitterParser::new().expect("Failed to create parser");
        
        let c_code = r#"
#include <stdio.h>

struct Point {
    int x;
    int y;
};

int add(int a, int b) {
    return a + b;
}

int main() {
    printf("Hello, World!\n");
    int result = add(5, 3);
    return 0;
}
"#;
        
        let result = parser.parse(c_code, Language::C);
        assert!(result.is_ok(), "C parsing should succeed");
        
        let parse_result = result.unwrap();
        assert_eq!(parse_result.language, Language::C);
        
        // Check for function nodes
        let function_nodes = parse_result.ast.find_by_type(&crate::ast::NodeType::Function);
        assert!(!function_nodes.is_empty(), "Should find function nodes");
        
        // Check for struct (mapped to class)
        let class_nodes = parse_result.ast.find_by_type(&crate::ast::NodeType::Class);
        assert!(!class_nodes.is_empty(), "Should find struct nodes mapped to class");
    }

    #[test]
    fn test_cpp_parsing() {
        let parser = TreeSitterParser::new().expect("Failed to create parser");
        
        let cpp_code = r#"
#include <iostream>
#include <string>

class Calculator {
private:
    int value;

public:
    Calculator() : value(0) {}
    
    int add(int a, int b) {
        return a + b;
    }
    
    void display() {
        std::cout << "Value: " << value << std::endl;
    }
};

int main() {
    Calculator calc;
    int result = calc.add(10, 20);
    calc.display();
    return 0;
}
"#;
        
        let result = parser.parse(cpp_code, Language::Cpp);
        assert!(result.is_ok(), "C++ parsing should succeed");
        
        let parse_result = result.unwrap();
        assert_eq!(parse_result.language, Language::Cpp);
        
        // Check for function and class nodes
        let function_nodes = parse_result.ast.find_by_type(&crate::ast::NodeType::Function);
        assert!(!function_nodes.is_empty(), "Should find function nodes");
        
        let class_nodes = parse_result.ast.find_by_type(&crate::ast::NodeType::Class);
        assert!(!class_nodes.is_empty(), "Should find class nodes");
    }

    #[test]
    fn test_ast_node_attributes() {
        let parser = TreeSitterParser::new().expect("Failed to create parser");
        
        let java_code = r#"
public class Test {
    public void testMethod(int param) {
        System.out.println("test");
    }
}
"#;
        
        let result = parser.parse(java_code, Language::Java).expect("Parsing should succeed");
        
        // Find function nodes and check attributes
        let function_nodes = result.ast.find_by_type(&crate::ast::NodeType::Function);
        assert!(!function_nodes.is_empty());
        
        // Check that function nodes have name attributes
        for func_node in function_nodes {
            if let Some(name) = func_node.metadata.attributes.get("name") {
                assert!(!name.is_empty(), "Function should have a name");
            }
        }
    }

    #[test]
    fn test_error_handling() {
        let parser = TreeSitterParser::new().expect("Failed to create parser");

        // Test with invalid syntax
        let invalid_java = r#"
public class Invalid {
    public void method( {
        // Missing closing parenthesis
    }
}
"#;

        let result = parser.parse(invalid_java, Language::Java);
        assert!(result.is_ok(), "Parser should handle invalid syntax gracefully");

        let parse_result = result.unwrap();
        assert!(!parse_result.errors.is_empty(), "Should report parse errors");
    }

    #[test]
    fn test_ast_builder_configuration() {
        use crate::ast_builder::{ASTBuilderBuilder};

        // Test builder pattern
        let builder = ASTBuilderBuilder::new()
            .include_comments(false)
            .include_whitespace(false)
            .max_text_length(50)
            .extract_signatures(true)
            .build_symbol_table(true)
            .build(Language::Java);

        assert!(builder.get_stats().total_nodes >= 0);
    }

    #[test]
    fn test_ast_analysis() {
        use crate::ast_processor::ASTProcessor;

        let parser = TreeSitterParser::new().expect("Failed to create parser");

        let java_code = r#"
public class Calculator {
    private int value;

    public Calculator() {
        this.value = 0;
    }

    public int add(int a, int b) {
        if (a > 0 && b > 0) {
            return a + b;
        }
        return 0;
    }

    public void complexMethod() {
        for (int i = 0; i < 10; i++) {
            if (i % 2 == 0) {
                while (value < i) {
                    value++;
                }
            }
        }
    }
}
"#;

        let result = parser.parse(java_code, Language::Java).expect("Parsing should succeed");

        let processor = ASTProcessor::new(Language::Java);
        let analysis = processor.analyze(&result.ast);

        assert!(analysis.total_nodes > 0, "Should have nodes in AST");
        assert!(analysis.function_count >= 3, "Should find at least 3 functions (constructor + 2 methods)");
        assert!(analysis.class_count >= 1, "Should find at least 1 class");
        assert!(analysis.cyclomatic_complexity > 0, "Should have some cyclomatic complexity");
        assert!(analysis.max_depth > 1, "Should have nested structure");
    }

    #[test]
    fn test_function_signature_extraction() {
        use crate::ast_processor::ASTProcessor;

        let parser = TreeSitterParser::new().expect("Failed to create parser");

        let java_code = r#"
public class MathUtils {
    public static int add(int a, int b) {
        return a + b;
    }

    private String formatResult(double value) {
        return String.valueOf(value);
    }

    protected void initialize() {
        // initialization code
    }
}
"#;

        let result = parser.parse(java_code, Language::Java).expect("Parsing should succeed");

        let processor = ASTProcessor::new(Language::Java);
        let signatures = processor.extract_function_signatures(&result.ast);

        assert!(signatures.len() >= 3, "Should extract at least 3 function signatures");

        // Check that we found the expected functions
        let function_names: Vec<&String> = signatures.iter().map(|s| &s.name).collect();
        assert!(function_names.iter().any(|&name| name == "add"));
        assert!(function_names.iter().any(|&name| name == "formatResult"));
        assert!(function_names.iter().any(|&name| name == "initialize"));
    }

    #[test]
    fn test_symbol_table_construction() {
        use crate::ast_processor::ASTProcessor;

        let parser = TreeSitterParser::new().expect("Failed to create parser");

        let python_code = r#"
class Calculator:
    def __init__(self):
        self.value = 0

    def add(self, a, b):
        result = a + b
        return result

    def get_value(self):
        return self.value

def standalone_function():
    local_var = 42
    return local_var
"#;

        let result = parser.parse(python_code, Language::Python).expect("Parsing should succeed");

        let processor = ASTProcessor::new(Language::Python);
        let symbol_table = processor.build_symbol_table(&result.ast);

        let all_symbols = symbol_table.all_symbols();
        assert!(!all_symbols.is_empty(), "Should have symbols in symbol table");

        // Check for class symbol
        let class_symbols: Vec<_> = all_symbols.iter()
            .filter(|s| s.symbol_type == crate::ast_processor::SymbolType::Class)
            .collect();
        assert!(!class_symbols.is_empty(), "Should find class symbols");

        // Check for function symbols
        let function_symbols: Vec<_> = all_symbols.iter()
            .filter(|s| s.symbol_type == crate::ast_processor::SymbolType::Function)
            .collect();
        assert!(!function_symbols.is_empty(), "Should find function symbols");
    }

    #[test]
    fn test_ast_optimization() {
        use crate::ast_processor::ASTProcessor;

        let parser = TreeSitterParser::new().expect("Failed to create parser");

        let js_code = r#"
function test() {
    {
        console.log("nested block");
    }

    let a = "string1";
    let b = "string2";

    return a + b;
}
"#;

        let result = parser.parse(js_code, Language::JavaScript).expect("Parsing should succeed");
        let mut ast = result.ast;

        let processor = ASTProcessor::new(Language::JavaScript);
        let optimization_result = processor.optimize(&mut ast);

        // Should have performed some optimizations
        assert!(
            optimization_result.nodes_removed > 0 ||
            optimization_result.nodes_flattened > 0 ||
            optimization_result.nodes_merged > 0,
            "Should have performed some optimizations"
        );
    }

    #[test]
    fn test_parser_builder_pattern() {
        use crate::tree_sitter::TreeSitterParser;

        let parser = TreeSitterParser::builder()
            .include_comments(false)
            .include_whitespace(false)
            .max_text_length(100)
            .enable_optimization(true)
            .enable_analysis(true)
            .build()
            .expect("Should build parser successfully");

        let simple_code = "int main() { return 0; }";
        let result = parser.parse(simple_code, Language::C);
        assert!(result.is_ok(), "Should parse simple C code");
    }
}
