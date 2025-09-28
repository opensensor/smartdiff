//! Language-specific configuration for tree-sitter parsers

use crate::language::Language;
use crate::ast::NodeType;
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Language-specific configuration
#[derive(Debug, Clone)]
pub struct LanguageConfig {
    pub name: &'static str,
    pub file_extensions: Vec<&'static str>,
    pub function_node_types: Vec<&'static str>,
    pub class_node_types: Vec<&'static str>,
    pub comment_node_types: Vec<&'static str>,
    pub identifier_field_names: Vec<&'static str>,
}

/// Global language configurations
pub static LANGUAGE_CONFIGS: Lazy<HashMap<Language, LanguageConfig>> = Lazy::new(|| {
    let mut configs = HashMap::new();
    
    // Java configuration
    configs.insert(Language::Java, LanguageConfig {
        name: "java",
        file_extensions: vec!["java"],
        function_node_types: vec![
            "method_declaration",
            "constructor_declaration",
        ],
        class_node_types: vec![
            "class_declaration",
            "interface_declaration",
            "enum_declaration",
        ],
        comment_node_types: vec![
            "line_comment",
            "block_comment",
        ],
        identifier_field_names: vec!["name", "identifier"],
    });
    
    // Python configuration
    configs.insert(Language::Python, LanguageConfig {
        name: "python",
        file_extensions: vec!["py", "pyw"],
        function_node_types: vec![
            "function_definition",
        ],
        class_node_types: vec![
            "class_definition",
        ],
        comment_node_types: vec![
            "comment",
        ],
        identifier_field_names: vec!["name"],
    });
    
    // JavaScript configuration
    configs.insert(Language::JavaScript, LanguageConfig {
        name: "javascript",
        file_extensions: vec!["js", "jsx"],
        function_node_types: vec![
            "function_declaration",
            "function_expression",
            "arrow_function",
            "method_definition",
        ],
        class_node_types: vec![
            "class_declaration",
        ],
        comment_node_types: vec![
            "comment",
        ],
        identifier_field_names: vec!["name", "property"],
    });
    
    // C++ configuration
    configs.insert(Language::Cpp, LanguageConfig {
        name: "cpp",
        file_extensions: vec!["cpp", "cc", "cxx", "c++", "hpp", "hxx", "h++"],
        function_node_types: vec![
            "function_definition",
            "function_declarator",
            "method_definition",
        ],
        class_node_types: vec![
            "class_specifier",
            "struct_specifier",
            "union_specifier",
        ],
        comment_node_types: vec![
            "comment",
        ],
        identifier_field_names: vec!["declarator", "name"],
    });
    
    // C configuration
    configs.insert(Language::C, LanguageConfig {
        name: "c",
        file_extensions: vec!["c", "h"],
        function_node_types: vec![
            "function_definition",
            "function_declarator",
        ],
        class_node_types: vec![
            "struct_specifier",
            "union_specifier",
        ],
        comment_node_types: vec![
            "comment",
        ],
        identifier_field_names: vec!["declarator", "name"],
    });
    
    configs
});

/// Language-specific node type mappings
pub static NODE_TYPE_MAPPINGS: Lazy<HashMap<&'static str, NodeType>> = Lazy::new(|| {
    let mut mappings = HashMap::new();
    
    // Program structure
    mappings.insert("program", NodeType::Program);
    mappings.insert("source_file", NodeType::Program);
    mappings.insert("module", NodeType::Module);
    mappings.insert("package_declaration", NodeType::Module);
    mappings.insert("import_declaration", NodeType::Module);
    mappings.insert("import_statement", NodeType::Module);
    
    // Classes and interfaces
    mappings.insert("class_declaration", NodeType::Class);
    mappings.insert("class_definition", NodeType::Class);
    mappings.insert("class_specifier", NodeType::Class);
    mappings.insert("struct_specifier", NodeType::Class);
    mappings.insert("union_specifier", NodeType::Class);
    mappings.insert("interface_declaration", NodeType::Interface);
    mappings.insert("interface_definition", NodeType::Interface);
    
    // Functions and methods
    mappings.insert("function_declaration", NodeType::Function);
    mappings.insert("function_definition", NodeType::Function);
    mappings.insert("function_expression", NodeType::Function);
    mappings.insert("arrow_function", NodeType::Function);
    mappings.insert("method_declaration", NodeType::Method);
    mappings.insert("method_definition", NodeType::Method);
    mappings.insert("constructor_declaration", NodeType::Constructor);
    
    // Control flow
    mappings.insert("if_statement", NodeType::IfStatement);
    mappings.insert("while_statement", NodeType::WhileLoop);
    mappings.insert("for_statement", NodeType::ForLoop);
    mappings.insert("for_in_statement", NodeType::ForLoop);
    mappings.insert("enhanced_for_statement", NodeType::ForLoop);
    
    // Blocks and statements
    mappings.insert("block", NodeType::Block);
    mappings.insert("block_statement", NodeType::Block);
    mappings.insert("compound_statement", NodeType::Block);
    mappings.insert("suite", NodeType::Block);
    mappings.insert("return_statement", NodeType::ReturnStatement);
    mappings.insert("expression_statement", NodeType::ExpressionStatement);
    
    // Expressions
    mappings.insert("binary_expression", NodeType::BinaryExpression);
    mappings.insert("unary_expression", NodeType::UnaryExpression);
    mappings.insert("call_expression", NodeType::CallExpression);
    mappings.insert("assignment_expression", NodeType::AssignmentExpression);
    
    // Identifiers and literals
    mappings.insert("identifier", NodeType::Identifier);
    mappings.insert("simple_identifier", NodeType::Identifier);
    mappings.insert("type_identifier", NodeType::Identifier);
    mappings.insert("string_literal", NodeType::Literal);
    mappings.insert("number_literal", NodeType::Literal);
    mappings.insert("integer_literal", NodeType::Literal);
    mappings.insert("float_literal", NodeType::Literal);
    mappings.insert("boolean_literal", NodeType::Literal);
    mappings.insert("character_literal", NodeType::Literal);
    mappings.insert("null_literal", NodeType::Literal);
    mappings.insert("true", NodeType::Literal);
    mappings.insert("false", NodeType::Literal);
    
    // Declarations
    mappings.insert("variable_declaration", NodeType::VariableDeclaration);
    mappings.insert("local_variable_declaration", NodeType::VariableDeclaration);
    mappings.insert("parameter_declaration", NodeType::ParameterDeclaration);
    mappings.insert("field_declaration", NodeType::FieldDeclaration);
    
    // Comments
    mappings.insert("comment", NodeType::Comment);
    mappings.insert("line_comment", NodeType::Comment);
    mappings.insert("block_comment", NodeType::Comment);
    
    mappings
});

impl LanguageConfig {
    /// Get configuration for a language
    pub fn get(language: &Language) -> Option<&'static LanguageConfig> {
        LANGUAGE_CONFIGS.get(language)
    }
    
    /// Check if a node type represents a function
    pub fn is_function_node(&self, node_type: &str) -> bool {
        self.function_node_types.contains(&node_type)
    }
    
    /// Check if a node type represents a class
    pub fn is_class_node(&self, node_type: &str) -> bool {
        self.class_node_types.contains(&node_type)
    }
    
    /// Check if a node type represents a comment
    pub fn is_comment_node(&self, node_type: &str) -> bool {
        self.comment_node_types.contains(&node_type)
    }
    
    /// Map tree-sitter node type to our NodeType
    pub fn map_node_type(&self, ts_node_type: &str) -> NodeType {
        NODE_TYPE_MAPPINGS.get(ts_node_type)
            .copied()
            .unwrap_or(NodeType::Unknown)
    }
}
