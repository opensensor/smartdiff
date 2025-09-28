//! Demonstration of the tree-sitter parser integration

use smart_diff_parser::{
    ast::NodeType, language::Language, parser::Parser, tree_sitter::TreeSitterParser,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Smart Code Diff - Tree-sitter Parser Demo");
    println!("==========================================");

    let parser = TreeSitterParser::new()?;

    // Demo Java parsing
    demo_java_parsing(&parser)?;

    // Demo Python parsing
    demo_python_parsing(&parser)?;

    // Demo JavaScript parsing
    demo_javascript_parsing(&parser)?;

    // Demo C parsing
    demo_c_parsing(&parser)?;

    // Demo C++ parsing
    demo_cpp_parsing(&parser)?;

    Ok(())
}

fn demo_java_parsing(parser: &TreeSitterParser) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Java Parsing Demo ---");

    let java_code = r#"
public class Calculator {
    private int value;
    
    public Calculator() {
        this.value = 0;
    }
    
    public int add(int a, int b) {
        return a + b;
    }
    
    public void setValue(int value) {
        this.value = value;
    }
}
"#;

    let result = parser.parse(java_code, Language::Java)?;
    print_parse_results("Java", &result);

    Ok(())
}

fn demo_python_parsing(parser: &TreeSitterParser) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Python Parsing Demo ---");

    let python_code = r#"
class Calculator:
    def __init__(self):
        self.value = 0
    
    def add(self, a, b):
        return a + b
    
    def set_value(self, value):
        self.value = value

def main():
    calc = Calculator()
    result = calc.add(10, 20)
    print(f"Result: {result}")

if __name__ == "__main__":
    main()
"#;

    let result = parser.parse(python_code, Language::Python)?;
    print_parse_results("Python", &result);

    Ok(())
}

fn demo_javascript_parsing(parser: &TreeSitterParser) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- JavaScript Parsing Demo ---");

    let js_code = r#"
class Calculator {
    constructor() {
        this.value = 0;
    }
    
    add(a, b) {
        return a + b;
    }
    
    setValue(value) {
        this.value = value;
    }
}

function main() {
    const calc = new Calculator();
    const result = calc.add(10, 20);
    console.log(`Result: ${result}`);
}

main();
"#;

    let result = parser.parse(js_code, Language::JavaScript)?;
    print_parse_results("JavaScript", &result);

    Ok(())
}

fn demo_c_parsing(parser: &TreeSitterParser) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- C Parsing Demo ---");

    let c_code = r#"
#include <stdio.h>

struct Calculator {
    int value;
};

int add(int a, int b) {
    return a + b;
}

void set_value(struct Calculator* calc, int value) {
    calc->value = value;
}

int main() {
    struct Calculator calc = {0};
    int result = add(10, 20);
    printf("Result: %d\n", result);
    return 0;
}
"#;

    let result = parser.parse(c_code, Language::C)?;
    print_parse_results("C", &result);

    Ok(())
}

fn demo_cpp_parsing(parser: &TreeSitterParser) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- C++ Parsing Demo ---");

    let cpp_code = r#"
#include <iostream>

class Calculator {
private:
    int value;

public:
    Calculator() : value(0) {}
    
    int add(int a, int b) {
        return a + b;
    }
    
    void setValue(int value) {
        this->value = value;
    }
};

int main() {
    Calculator calc;
    int result = calc.add(10, 20);
    std::cout << "Result: " << result << std::endl;
    return 0;
}
"#;

    let result = parser.parse(cpp_code, Language::Cpp)?;
    print_parse_results("C++", &result);

    Ok(())
}

fn print_parse_results(language: &str, result: &smart_diff_parser::ParseResult) {
    println!("Language: {}", language);
    println!("AST Root Type: {:?}", result.ast.node_type);

    // Count different node types
    let function_count = result.ast.find_by_type(&NodeType::Function).len();
    let method_count = result.ast.find_by_type(&NodeType::Method).len();
    let class_count = result.ast.find_by_type(&NodeType::Class).len();
    let constructor_count = result.ast.find_by_type(&NodeType::Constructor).len();

    println!("Functions found: {}", function_count);
    println!("Methods found: {}", method_count);
    println!("Classes found: {}", class_count);
    println!("Constructors found: {}", constructor_count);

    if !result.errors.is_empty() {
        println!("Parse errors: {}", result.errors.len());
        for error in &result.errors {
            println!("  - {}", error);
        }
    }

    if !result.warnings.is_empty() {
        println!("Parse warnings: {}", result.warnings.len());
        for warning in &result.warnings {
            println!("  - {}", warning);
        }
    }

    // Print some function details
    let functions = result.ast.find_by_type(&NodeType::Function);
    let methods = result.ast.find_by_type(&NodeType::Method);
    let all_functions: Vec<_> = functions.into_iter().chain(methods).collect();

    if !all_functions.is_empty() {
        println!("Function details:");
        for func in all_functions.iter().take(3) {
            // Show first 3 functions
            if let Some(name) = func.metadata.attributes.get("name") {
                println!("  - {} (line {})", name, func.metadata.line);
            }
        }
    }
}
