//! AST builder for converting tree-sitter parse trees to normalized AST representation

use crate::ast::{ASTNode, NodeType, NodeMetadata};
use crate::language::Language;
use crate::language_config::{LanguageConfig, LANGUAGE_CONFIGS};
use std::collections::HashMap;
use tree_sitter::{Node, Tree};

/// Configuration for AST building process
#[derive(Debug, Clone)]
pub struct ASTBuilderConfig {
    /// Whether to include comment nodes in the AST
    pub include_comments: bool,
    /// Whether to include whitespace-only nodes
    pub include_whitespace: bool,
    /// Maximum text length to store in node metadata
    pub max_text_length: usize,
    /// Whether to extract detailed function signatures
    pub extract_signatures: bool,
    /// Whether to build symbol tables during AST construction
    pub build_symbol_table: bool,
}

impl Default for ASTBuilderConfig {
    fn default() -> Self {
        Self {
            include_comments: true,
            include_whitespace: false,
            max_text_length: 200,
            extract_signatures: true,
            build_symbol_table: true,
        }
    }
}

/// Statistics collected during AST building
#[derive(Debug, Default)]
pub struct ASTBuildStats {
    pub total_nodes: usize,
    pub function_nodes: usize,
    pub class_nodes: usize,
    pub comment_nodes: usize,
    pub skipped_nodes: usize,
    pub max_depth: usize,
}

/// Enhanced AST builder with configurable processing
pub struct ASTBuilder {
    config: ASTBuilderConfig,
    language: Language,
    language_config: Option<&'static LanguageConfig>,
    stats: ASTBuildStats,
}

impl ASTBuilder {
    /// Create a new AST builder for the specified language
    pub fn new(language: Language, config: ASTBuilderConfig) -> Self {
        let language_config = LANGUAGE_CONFIGS.get(&language);
        
        Self {
            config,
            language,
            language_config,
            stats: ASTBuildStats::default(),
        }
    }
    
    /// Create AST builder with default configuration
    pub fn with_defaults(language: Language) -> Self {
        Self::new(language, ASTBuilderConfig::default())
    }
    
    /// Build AST from tree-sitter tree
    pub fn build_ast(&mut self, tree: &Tree, source: &str) -> ASTNode {
        let root_node = tree.root_node();
        self.stats = ASTBuildStats::default();
        
        let ast = self.convert_node(&root_node, source, 0);
        self.stats.max_depth = ast.depth();
        
        ast
    }
    
    /// Get build statistics
    pub fn get_stats(&self) -> &ASTBuildStats {
        &self.stats
    }
    
    /// Convert a tree-sitter node to AST node
    fn convert_node(&mut self, node: &Node, source: &str, depth: usize) -> ASTNode {
        self.stats.total_nodes += 1;
        
        let node_kind = node.kind();
        let node_type = self.map_node_type(node_kind);
        
        // Extract text content
        let text = node.utf8_text(source.as_bytes()).unwrap_or("");
        let trimmed_text = text.trim();
        
        // Skip whitespace-only nodes if configured
        if !self.config.include_whitespace && trimmed_text.is_empty() && node.child_count() == 0 {
            self.stats.skipped_nodes += 1;
            // Return a placeholder node that will be filtered out
            return self.create_placeholder_node();
        }
        
        // Skip comment nodes if configured
        if !self.config.include_comments && self.is_comment_node(node_kind) {
            self.stats.skipped_nodes += 1;
            return self.create_placeholder_node();
        }
        
        // Count specific node types
        match node_type {
            NodeType::Function | NodeType::Method | NodeType::Constructor => {
                self.stats.function_nodes += 1;
            }
            NodeType::Class | NodeType::Interface => {
                self.stats.class_nodes += 1;
            }
            NodeType::Comment => {
                self.stats.comment_nodes += 1;
            }
            _ => {}
        }
        
        // Create metadata
        let metadata = self.create_node_metadata(node, source, node_kind, text);
        
        // Create AST node
        let mut ast_node = ASTNode::new(node_type, metadata);
        
        // Process children
        self.process_children(&mut ast_node, node, source, depth + 1);
        
        ast_node
    }
    
    /// Create metadata for a node
    fn create_node_metadata(&self, node: &Node, source: &str, node_kind: &str, text: &str) -> NodeMetadata {
        let mut attributes = HashMap::new();
        
        // Basic node information
        attributes.insert("kind".to_string(), node_kind.to_string());
        attributes.insert("byte_range".to_string(), format!("{}..{}", node.start_byte(), node.end_byte()));
        
        // Store text if not too long
        let trimmed_text = text.trim();
        if !trimmed_text.is_empty() && trimmed_text.len() <= self.config.max_text_length {
            attributes.insert("text".to_string(), trimmed_text.to_string());
        }
        
        // Extract language-specific attributes
        self.extract_language_specific_attributes(node, source, &mut attributes);
        
        // Extract structural information
        self.extract_structural_attributes(node, &mut attributes);
        
        NodeMetadata {
            line: node.start_position().row + 1,
            column: node.start_position().column + 1,
            original_text: if text.len() <= self.config.max_text_length {
                text.to_string()
            } else {
                format!("{}...", &text[..self.config.max_text_length.min(text.len())])
            },
            attributes,
        }
    }
    
    /// Extract language-specific attributes from a node
    fn extract_language_specific_attributes(&self, node: &Node, source: &str, attributes: &mut HashMap<String, String>) {
        let node_kind = node.kind();
        
        // Extract identifier/name information
        if let Some(config) = self.language_config {
            for field_name in &config.identifier_field_names {
                if let Some(name_node) = node.child_by_field_name(field_name) {
                    if let Ok(name) = name_node.utf8_text(source.as_bytes()) {
                        attributes.insert("name".to_string(), name.to_string());
                        break;
                    }
                }
            }
        }
        
        // Node-specific attribute extraction
        match node_kind {
            "call_expression" | "function_call" | "method_invocation" => {
                self.extract_call_attributes(node, source, attributes);
            }
            "function_declaration" | "method_declaration" | "function_definition" => {
                self.extract_function_attributes(node, source, attributes);
            }
            "class_declaration" | "class_definition" | "interface_declaration" => {
                self.extract_class_attributes(node, source, attributes);
            }
            "variable_declaration" | "field_declaration" | "parameter_declaration" => {
                self.extract_declaration_attributes(node, source, attributes);
            }
            _ => {}
        }
    }
    
    /// Extract attributes for function call nodes
    fn extract_call_attributes(&self, node: &Node, source: &str, attributes: &mut HashMap<String, String>) {
        // Extract function name
        if let Some(function_node) = node.child_by_field_name("function") {
            if let Ok(name) = function_node.utf8_text(source.as_bytes()) {
                attributes.insert("function_name".to_string(), name.to_string());
            }
        }
        
        // Count arguments
        if let Some(args_node) = node.child_by_field_name("arguments") {
            let arg_count = args_node.named_child_count();
            attributes.insert("argument_count".to_string(), arg_count.to_string());
        }
    }
    
    /// Extract attributes for function declaration nodes
    fn extract_function_attributes(&self, node: &Node, source: &str, attributes: &mut HashMap<String, String>) {
        // Extract return type
        if let Some(type_node) = node.child_by_field_name("type") {
            if let Ok(return_type) = type_node.utf8_text(source.as_bytes()) {
                attributes.insert("return_type".to_string(), return_type.to_string());
            }
        }
        
        // Count parameters
        if let Some(params_node) = node.child_by_field_name("parameters") {
            let param_count = params_node.named_child_count();
            attributes.insert("parameter_count".to_string(), param_count.to_string());
        }
        
        // Extract modifiers (for languages that support them)
        self.extract_modifiers(node, source, attributes);
    }
    
    /// Extract attributes for class declaration nodes
    fn extract_class_attributes(&self, node: &Node, source: &str, attributes: &mut HashMap<String, String>) {
        // Extract superclass/extends
        if let Some(superclass_node) = node.child_by_field_name("superclass") {
            if let Ok(superclass) = superclass_node.utf8_text(source.as_bytes()) {
                attributes.insert("superclass".to_string(), superclass.to_string());
            }
        }
        
        // Extract interfaces (for languages that support them)
        if let Some(interfaces_node) = node.child_by_field_name("interfaces") {
            let interface_count = interfaces_node.named_child_count();
            attributes.insert("interface_count".to_string(), interface_count.to_string());
        }
        
        // Extract modifiers
        self.extract_modifiers(node, source, attributes);
    }
    
    /// Extract attributes for declaration nodes
    fn extract_declaration_attributes(&self, node: &Node, source: &str, attributes: &mut HashMap<String, String>) {
        // Extract type information
        if let Some(type_node) = node.child_by_field_name("type") {
            if let Ok(var_type) = type_node.utf8_text(source.as_bytes()) {
                attributes.insert("type".to_string(), var_type.to_string());
            }
        }
        
        // Extract initializer information
        if let Some(init_node) = node.child_by_field_name("value") {
            attributes.insert("has_initializer".to_string(), "true".to_string());
        }
    }
    
    /// Extract modifier information (public, private, static, etc.)
    fn extract_modifiers(&self, node: &Node, source: &str, attributes: &mut HashMap<String, String>) {
        let mut modifiers = Vec::new();
        
        // Look for modifier nodes
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                match child.kind() {
                    "public" | "private" | "protected" | "static" | "final" | 
                    "abstract" | "virtual" | "override" | "async" | "const" => {
                        if let Ok(modifier) = child.utf8_text(source.as_bytes()) {
                            modifiers.push(modifier.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        
        if !modifiers.is_empty() {
            attributes.insert("modifiers".to_string(), modifiers.join(","));
        }
    }
    
    /// Extract structural attributes (depth, sibling count, etc.)
    fn extract_structural_attributes(&self, node: &Node, attributes: &mut HashMap<String, String>) {
        attributes.insert("child_count".to_string(), node.child_count().to_string());
        attributes.insert("named_child_count".to_string(), node.named_child_count().to_string());
        
        if node.is_named() {
            attributes.insert("is_named".to_string(), "true".to_string());
        }
        
        if node.is_missing() {
            attributes.insert("is_missing".to_string(), "true".to_string());
        }
        
        if node.has_error() {
            attributes.insert("has_error".to_string(), "true".to_string());
        }
    }

    /// Process children of a node, filtering and converting them
    fn process_children(&mut self, ast_node: &mut ASTNode, node: &Node, source: &str, depth: usize) {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                // Skip certain noise nodes
                if self.should_skip_node(child.kind()) {
                    self.stats.skipped_nodes += 1;
                    continue;
                }

                let child_ast = self.convert_node(&child, source, depth);

                // Only add non-placeholder nodes
                if !self.is_placeholder_node(&child_ast) {
                    ast_node.add_child(child_ast);
                }
            }
        }
    }

    /// Check if a node should be skipped during AST construction
    fn should_skip_node(&self, kind: &str) -> bool {
        match kind {
            // Skip punctuation and delimiters
            "(" | ")" | "{" | "}" | "[" | "]" | ";" | "," | "." => true,

            // Skip operators that don't add semantic value
            "=" | "+=" | "-=" | "*=" | "/=" => false, // Keep assignment operators

            // Language-specific noise nodes
            "ERROR" => true, // Skip error nodes

            // Whitespace and formatting
            kind if kind.starts_with("_") => true, // Tree-sitter convention for hidden nodes

            _ => false,
        }
    }

    /// Check if a node is a comment node
    fn is_comment_node(&self, kind: &str) -> bool {
        if let Some(config) = self.language_config {
            config.comment_node_types.contains(&kind)
        } else {
            matches!(kind, "comment" | "line_comment" | "block_comment")
        }
    }

    /// Map tree-sitter node type to our NodeType
    fn map_node_type(&self, kind: &str) -> NodeType {
        use crate::language_config::NODE_TYPE_MAPPINGS;
        NODE_TYPE_MAPPINGS.get(kind)
            .copied()
            .unwrap_or(NodeType::Unknown)
    }

    /// Create a placeholder node for filtered content
    fn create_placeholder_node(&self) -> ASTNode {
        let metadata = NodeMetadata {
            line: 0,
            column: 0,
            original_text: String::new(),
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("placeholder".to_string(), "true".to_string());
                attrs
            },
        };
        ASTNode::new(NodeType::Unknown, metadata)
    }

    /// Check if a node is a placeholder
    fn is_placeholder_node(&self, node: &ASTNode) -> bool {
        node.metadata.attributes.get("placeholder").map_or(false, |v| v == "true")
    }
}

/// Builder pattern for AST construction with fluent interface
pub struct ASTBuilderBuilder {
    config: ASTBuilderConfig,
}

impl ASTBuilderBuilder {
    pub fn new() -> Self {
        Self {
            config: ASTBuilderConfig::default(),
        }
    }

    pub fn include_comments(mut self, include: bool) -> Self {
        self.config.include_comments = include;
        self
    }

    pub fn include_whitespace(mut self, include: bool) -> Self {
        self.config.include_whitespace = include;
        self
    }

    pub fn max_text_length(mut self, length: usize) -> Self {
        self.config.max_text_length = length;
        self
    }

    pub fn extract_signatures(mut self, extract: bool) -> Self {
        self.config.extract_signatures = extract;
        self
    }

    pub fn build_symbol_table(mut self, build: bool) -> Self {
        self.config.build_symbol_table = build;
        self
    }

    pub fn build(self, language: Language) -> ASTBuilder {
        ASTBuilder::new(language, self.config)
    }
}

impl Default for ASTBuilderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
