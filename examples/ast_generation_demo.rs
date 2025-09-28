//! Demonstration of the AST generation pipeline capabilities

use smart_diff_parser::{
    tree_sitter::TreeSitterParser,
    ast_processor::ASTProcessor,
    language::Language,
    parser::Parser,
    ast::NodeType,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Smart Code Diff - AST Generation Pipeline Demo");
    println!("==============================================");
    
    // Demo basic AST generation
    demo_basic_ast_generation()?;
    
    // Demo AST analysis
    demo_ast_analysis()?;
    
    // Demo function signature extraction
    demo_function_signature_extraction()?;
    
    // Demo symbol table construction
    demo_symbol_table_construction()?;
    
    // Demo AST optimization
    demo_ast_optimization()?;
    
    // Demo configurable parsing
    demo_configurable_parsing()?;
    
    Ok(())
}

fn demo_basic_ast_generation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Basic AST Generation ---");
    
    let parser = TreeSitterParser::new()?;
    
    let java_code = r#"
public class Calculator {
    private int value;
    
    public Calculator() {
        this.value = 0;
    }
    
    public int add(int a, int b) {
        return a + b;
    }
    
    public void setValue(int newValue) {
        this.value = newValue;
    }
}
"#;
    
    let result = parser.parse(java_code, Language::Java)?;
    
    println!("Language: {:?}", result.language);
    println!("Root node type: {:?}", result.ast.node_type);
    println!("Total child nodes: {}", result.ast.children.len());
    
    // Find and display functions
    let functions = result.ast.find_by_type(&NodeType::Function);
    let methods = result.ast.find_by_type(&NodeType::Method);
    let constructors = result.ast.find_by_type(&NodeType::Constructor);
    
    println!("Functions found: {}", functions.len());
    println!("Methods found: {}", methods.len());
    println!("Constructors found: {}", constructors.len());
    
    // Display function details
    for func in functions.iter().chain(methods.iter()).chain(constructors.iter()) {
        if let Some(name) = func.metadata.attributes.get("name") {
            let param_count = func.metadata.attributes.get("parameter_count").unwrap_or(&"0".to_string());
            let return_type = func.metadata.attributes.get("return_type").unwrap_or(&"void".to_string());
            println!("  - {} (params: {}, return: {}) at line {}", 
                     name, param_count, return_type, func.metadata.line);
        }
    }
    
    // Find classes
    let classes = result.ast.find_by_type(&NodeType::Class);
    println!("Classes found: {}", classes.len());
    for class in &classes {
        if let Some(name) = class.metadata.attributes.get("name") {
            println!("  - {} at line {}", name, class.metadata.line);
        }
    }
    
    Ok(())
}

fn demo_ast_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- AST Analysis ---");
    
    let parser = TreeSitterParser::new()?;
    
    let complex_java_code = r#"
public class ComplexCalculator {
    private int value;
    private String name;
    
    public ComplexCalculator(String name) {
        this.name = name;
        this.value = 0;
    }
    
    public int complexCalculation(int a, int b, int c) {
        int result = 0;
        
        if (a > 0) {
            for (int i = 0; i < a; i++) {
                if (i % 2 == 0) {
                    result += b;
                } else {
                    result += c;
                }
                
                while (result > 100) {
                    result -= 10;
                }
            }
        } else {
            switch (b) {
                case 1:
                    result = c * 2;
                    break;
                case 2:
                    result = c * 3;
                    break;
                default:
                    result = c;
            }
        }
        
        return result;
    }
    
    public void simpleMethod() {
        System.out.println("Simple method");
    }
}
"#;
    
    let result = parser.parse(complex_java_code, Language::Java)?;
    let processor = ASTProcessor::new(Language::Java);
    let analysis = processor.analyze(&result.ast);
    
    println!("AST Analysis Results:");
    println!("  Total nodes: {}", analysis.total_nodes);
    println!("  Max depth: {}", analysis.max_depth);
    println!("  Average depth: {:.2}", analysis.avg_depth);
    println!("  Function count: {}", analysis.function_count);
    println!("  Class count: {}", analysis.class_count);
    println!("  Cyclomatic complexity: {}", analysis.cyclomatic_complexity);
    println!("  Complexity score: {:.2}", analysis.complexity_score);
    
    println!("\nNode type distribution:");
    for (node_type, count) in &analysis.node_type_counts {
        if *count > 0 {
            println!("  {:?}: {}", node_type, count);
        }
    }
    
    Ok(())
}

fn demo_function_signature_extraction() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Function Signature Extraction ---");
    
    let parser = TreeSitterParser::new()?;
    
    let python_code = r#"
class DataProcessor:
    def __init__(self, name: str):
        self.name = name
        self.data = []
    
    def add_data(self, item: dict) -> bool:
        """Add an item to the data list."""
        if isinstance(item, dict):
            self.data.append(item)
            return True
        return False
    
    def process_data(self, filter_func=None):
        """Process the data with optional filtering."""
        processed = []
        for item in self.data:
            if filter_func is None or filter_func(item):
                processed.append(self._transform_item(item))
        return processed
    
    def _transform_item(self, item):
        """Private method to transform an item."""
        return {k: str(v) for k, v in item.items()}

def standalone_function(x, y, z=None):
    """A standalone function outside the class."""
    if z is None:
        return x + y
    return x + y + z
"#;
    
    let result = parser.parse(python_code, Language::Python)?;
    let processor = ASTProcessor::new(Language::Python);
    let signatures = processor.extract_function_signatures(&result.ast);
    
    println!("Extracted {} function signatures:", signatures.len());
    
    for signature in &signatures {
        println!("  - {} ({})", signature.name, signature.node_type);
        println!("    Parameters: {}", signature.parameter_count);
        if let Some(return_type) = &signature.return_type {
            println!("    Return type: {}", return_type);
        }
        if !signature.modifiers.is_empty() {
            println!("    Modifiers: {}", signature.modifiers.join(", "));
        }
        println!("    Location: line {}, column {}", signature.line, signature.column);
        println!();
    }
    
    Ok(())
}

fn demo_symbol_table_construction() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Symbol Table Construction ---");
    
    let parser = TreeSitterParser::new()?;
    
    let js_code = r#"
class Calculator {
    constructor(name) {
        this.name = name;
        this.history = [];
    }
    
    add(a, b) {
        const result = a + b;
        this.history.push({operation: 'add', result});
        return result;
    }
    
    getHistory() {
        return this.history;
    }
}

function createCalculator(name) {
    return new Calculator(name);
}

const defaultCalculator = createCalculator('default');
let currentResult = 0;
"#;
    
    let result = parser.parse(js_code, Language::JavaScript)?;
    let processor = ASTProcessor::new(Language::JavaScript);
    let symbol_table = processor.build_symbol_table(&result.ast);
    
    println!("Symbol Table Contents:");
    
    let all_symbols = symbol_table.all_symbols();
    println!("Total symbols: {}", all_symbols.len());
    
    // Group symbols by type
    let mut by_type = std::collections::HashMap::new();
    for symbol in all_symbols {
        by_type.entry(symbol.symbol_type.clone()).or_insert_with(Vec::new).push(symbol);
    }
    
    for (symbol_type, symbols) in &by_type {
        println!("\n{:?} symbols:", symbol_type);
        for symbol in symbols {
            let scope_str = if symbol.scope_path.is_empty() {
                "global".to_string()
            } else {
                symbol.scope_path.join("::")
            };
            println!("  - {} (scope: {}) at line {}", symbol.name, scope_str, symbol.line);
        }
    }
    
    Ok(())
}

fn demo_ast_optimization() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- AST Optimization ---");
    
    let parser = TreeSitterParser::new()?;
    
    let cpp_code = r#"
#include <iostream>

class OptimizationDemo {
public:
    void demonstrateOptimization() {
        {
            // Nested block that can be flattened
            std::cout << "Hello" << std::endl;
        }
        
        // Consecutive string literals
        std::string message = "Part1" + "Part2" + "Part3";
        
        {
            {
                // Deeply nested blocks
                int x = 42;
                std::cout << x << std::endl;
            }
        }
    }
};
"#;
    
    let result = parser.parse(cpp_code, Language::Cpp)?;
    let mut ast = result.ast;
    
    let processor = ASTProcessor::new(Language::Cpp);
    
    // Analyze before optimization
    let analysis_before = processor.analyze(&ast);
    println!("Before optimization:");
    println!("  Total nodes: {}", analysis_before.total_nodes);
    println!("  Max depth: {}", analysis_before.max_depth);
    
    // Perform optimization
    let optimization_result = processor.optimize(&mut ast);
    
    // Analyze after optimization
    let analysis_after = processor.analyze(&ast);
    println!("\nAfter optimization:");
    println!("  Total nodes: {}", analysis_after.total_nodes);
    println!("  Max depth: {}", analysis_after.max_depth);
    
    println!("\nOptimization results:");
    println!("  Nodes removed: {}", optimization_result.nodes_removed);
    println!("  Nodes flattened: {}", optimization_result.nodes_flattened);
    println!("  Nodes merged: {}", optimization_result.nodes_merged);
    
    let nodes_saved = analysis_before.total_nodes - analysis_after.total_nodes;
    if nodes_saved > 0 {
        let reduction_percent = (nodes_saved as f64 / analysis_before.total_nodes as f64) * 100.0;
        println!("  Total reduction: {} nodes ({:.1}%)", nodes_saved, reduction_percent);
    }
    
    Ok(())
}

fn demo_configurable_parsing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Configurable Parsing ---");
    
    let c_code = r#"
#include <stdio.h>

/* This is a block comment */
int main() {
    // This is a line comment
    int x = 42;
    
    
    // Multiple blank lines above
    printf("Hello, World!\n");
    return 0;
}
"#;
    
    // Parse with comments included
    println!("Parsing with comments included:");
    let parser_with_comments = TreeSitterParser::builder()
        .include_comments(true)
        .include_whitespace(false)
        .build()?;
    
    let result_with_comments = parser_with_comments.parse(c_code, Language::C)?;
    let analysis_with_comments = ASTProcessor::new(Language::C).analyze(&result_with_comments.ast);
    
    println!("  Total nodes: {}", analysis_with_comments.total_nodes);
    println!("  Comment nodes: {}", analysis_with_comments.node_type_counts.get(&NodeType::Comment).unwrap_or(&0));
    
    // Parse without comments
    println!("\nParsing without comments:");
    let parser_without_comments = TreeSitterParser::builder()
        .include_comments(false)
        .include_whitespace(false)
        .build()?;
    
    let result_without_comments = parser_without_comments.parse(c_code, Language::C)?;
    let analysis_without_comments = ASTProcessor::new(Language::C).analyze(&result_without_comments.ast);
    
    println!("  Total nodes: {}", analysis_without_comments.total_nodes);
    println!("  Comment nodes: {}", analysis_without_comments.node_type_counts.get(&NodeType::Comment).unwrap_or(&0));
    
    let node_difference = analysis_with_comments.total_nodes - analysis_without_comments.total_nodes;
    println!("  Difference: {} nodes", node_difference);
    
    Ok(())
}
