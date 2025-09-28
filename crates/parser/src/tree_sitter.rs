//! Tree-sitter integration for multi-language parsing

use crate::ast::{ASTNode, NodeType, NodeMetadata};
use crate::ast_builder::{ASTBuilder, ASTBuilderBuilder, ASTBuilderConfig};
use crate::ast_processor::{ASTProcessor, ASTAnalysis};
use crate::language::Language;
use crate::language_config::{LanguageConfig, LANGUAGE_CONFIGS};
use crate::parser::{ParseError, ParseResult, Parser};
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Tree-sitter based parser implementation
pub struct TreeSitterParser {
    parsers: HashMap<Language, tree_sitter::Parser>,
    builder_config: ASTBuilderConfig,
    enable_optimization: bool,
    enable_analysis: bool,
}

/// Global language configurations
static LANGUAGE_CONFIGS: Lazy<HashMap<Language, fn() -> tree_sitter::Language>> = Lazy::new(|| {
    let mut configs = HashMap::new();
    configs.insert(Language::Java, || tree_sitter_java::language());
    configs.insert(Language::Python, || tree_sitter_python::language());
    configs.insert(Language::JavaScript, || tree_sitter_javascript::language());
    configs.insert(Language::Cpp, || tree_sitter_cpp::language());
    configs.insert(Language::C, || tree_sitter_c::language());
    configs
});

impl TreeSitterParser {
    pub fn new() -> Result<Self, ParseError> {
        Self::with_config(ASTBuilderConfig::default())
    }

    pub fn with_config(builder_config: ASTBuilderConfig) -> Result<Self, ParseError> {
        let mut parsers = HashMap::new();

        // Initialize parsers for supported languages
        for (&language, language_fn) in LANGUAGE_CONFIGS.iter() {
            let mut parser = tree_sitter::Parser::new();
            parser.set_language(language_fn())
                .map_err(|e| ParseError::TreeSitterError(format!("Failed to set language {:?}: {}", language, e)))?;
            parsers.insert(language, parser);
        }

        Ok(Self {
            parsers,
            builder_config,
            enable_optimization: true,
            enable_analysis: true,
        })
    }

    /// Create parser with builder pattern
    pub fn builder() -> TreeSitterParserBuilder {
        TreeSitterParserBuilder::new()
    }

    /// Enable or disable AST optimization
    pub fn set_optimization_enabled(&mut self, enabled: bool) {
        self.enable_optimization = enabled;
    }

    /// Enable or disable AST analysis
    pub fn set_analysis_enabled(&mut self, enabled: bool) {
        self.enable_analysis = enabled;
    }

    /// Get available languages
    pub fn supported_languages() -> Vec<Language> {
        LANGUAGE_CONFIGS.keys().cloned().collect()
    }
    
    fn convert_tree_sitter_node(&self, node: &tree_sitter::Node, source: &str) -> ASTNode {
        let node_kind = node.kind();
        let node_type = self.map_node_type(node_kind);
        let text = node.utf8_text(source.as_bytes()).unwrap_or("");

        let mut attributes = HashMap::new();

        // Extract name/identifier information based on node type
        self.extract_node_attributes(node, source, &mut attributes);

        // Add basic node information
        attributes.insert("kind".to_string(), node_kind.to_string());
        if !text.trim().is_empty() && text.len() < 100 { // Avoid storing very long text
            attributes.insert("text".to_string(), text.trim().to_string());
        }

        let metadata = NodeMetadata {
            line: node.start_position().row + 1, // Convert to 1-based line numbers
            column: node.start_position().column + 1, // Convert to 1-based column numbers
            original_text: text.to_string(),
            attributes,
        };

        let mut ast_node = ASTNode::new(node_type, metadata);

        // Convert children, filtering out some noise nodes
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                // Skip certain noise nodes like punctuation
                if !self.should_skip_node(child.kind()) {
                    ast_node.add_child(self.convert_tree_sitter_node(&child, source));
                }
            }
        }

        ast_node
    }

    /// Check if a node should be skipped during AST conversion
    fn should_skip_node(&self, kind: &str) -> bool {
        matches!(kind,
            "(" | ")" | "{" | "}" | "[" | "]" | ";" | "," | "." |
            "whitespace" | "comment" // We handle comments separately
        )
    }

    /// Collect parse errors from the tree
    fn collect_parse_errors(&self, node: &tree_sitter::Node, source: &str, errors: &mut Vec<String>) {
        if node.is_error() {
            let text = node.utf8_text(source.as_bytes()).unwrap_or("<error>");
            errors.push(format!(
                "Parse error at line {}, column {}: {}",
                node.start_position().row + 1,
                node.start_position().column + 1,
                text
            ));
        }

        if node.is_missing() {
            errors.push(format!(
                "Missing node at line {}, column {}",
                node.start_position().row + 1,
                node.start_position().column + 1
            ));
        }

        // Recursively check children
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.collect_parse_errors(&child, source, errors);
            }
        }
    }
    
    fn map_node_type(&self, kind: &str) -> NodeType {
        use crate::language_config::NODE_TYPE_MAPPINGS;
        NODE_TYPE_MAPPINGS.get(kind)
            .copied()
            .unwrap_or(NodeType::Unknown)
    }

    /// Extract attributes from a tree-sitter node
    fn extract_node_attributes(&self, node: &tree_sitter::Node, source: &str, attributes: &mut HashMap<String, String>) {
        let node_kind = node.kind();

        // Try to extract name/identifier from common field names
        for field_name in &["name", "identifier", "declarator", "property"] {
            if let Some(name_node) = node.child_by_field_name(field_name) {
                if let Ok(name) = name_node.utf8_text(source.as_bytes()) {
                    attributes.insert("name".to_string(), name.to_string());
                    break;
                }
            }
        }

        // Special handling for different node types
        match node_kind {
            "call_expression" => {
                // Extract function name from call expression
                if let Some(function_node) = node.child_by_field_name("function") {
                    if let Ok(name) = function_node.utf8_text(source.as_bytes()) {
                        attributes.insert("function_name".to_string(), name.to_string());
                    }
                }

                // Extract arguments count
                let args_count = node.children(&mut node.walk())
                    .filter(|child| child.kind() == "arguments")
                    .map(|args_node| args_node.child_count())
                    .next()
                    .unwrap_or(0);
                attributes.insert("args_count".to_string(), args_count.to_string());
            }

            "method_declaration" | "function_declaration" | "function_definition" => {
                // Extract parameter count
                if let Some(params_node) = node.child_by_field_name("parameters") {
                    let param_count = params_node.child_count();
                    attributes.insert("param_count".to_string(), param_count.to_string());
                }

                // Extract return type if available
                if let Some(type_node) = node.child_by_field_name("type") {
                    if let Ok(return_type) = type_node.utf8_text(source.as_bytes()) {
                        attributes.insert("return_type".to_string(), return_type.to_string());
                    }
                }
            }

            "variable_declaration" | "field_declaration" => {
                // Extract variable type
                if let Some(type_node) = node.child_by_field_name("type") {
                    if let Ok(var_type) = type_node.utf8_text(source.as_bytes()) {
                        attributes.insert("type".to_string(), var_type.to_string());
                    }
                }
            }

            "class_declaration" | "class_definition" => {
                // Extract superclass if available
                if let Some(superclass_node) = node.child_by_field_name("superclass") {
                    if let Ok(superclass) = superclass_node.utf8_text(source.as_bytes()) {
                        attributes.insert("superclass".to_string(), superclass.to_string());
                    }
                }
            }

            _ => {}
        }
    }
}

impl Parser for TreeSitterParser {
    fn parse(&self, content: &str, language: Language) -> Result<ParseResult, ParseError> {
        let parser = self.parsers.get(&language)
            .ok_or_else(|| ParseError::UnsupportedLanguage(language.clone()))?;

        // Parse the content
        let tree = parser.parse(content, None)
            .ok_or_else(|| ParseError::ParseFailed("Failed to parse content".to_string()))?;

        let root_node = tree.root_node();

        // Build AST using the enhanced AST builder
        let mut ast_builder = ASTBuilder::new(language.clone(), self.builder_config.clone());
        let mut ast = ast_builder.build_ast(&tree, content);

        // Optimize AST if enabled
        if self.enable_optimization {
            let processor = ASTProcessor::new(language.clone());
            let _optimization_result = processor.optimize(&mut ast);
        }

        // Collect any parse errors
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        if root_node.has_error() {
            self.collect_parse_errors(&root_node, content, &mut errors);
        }

        // Add build statistics as warnings if analysis is enabled
        if self.enable_analysis {
            let stats = ast_builder.get_stats();
            if stats.skipped_nodes > 0 {
                warnings.push(format!("Skipped {} noise nodes during AST construction", stats.skipped_nodes));
            }
            if stats.total_nodes > 1000 {
                warnings.push(format!("Large AST with {} nodes may impact performance", stats.total_nodes));
            }
        }

        Ok(ParseResult {
            ast,
            language,
            errors,
            warnings,
        })
    }

    fn parse_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<ParseResult, ParseError> {
        let content = std::fs::read_to_string(&path)?;
        let language = crate::language::LanguageDetector::detect(&path, &content);
        self.parse(&content, language)
    }

    fn supported_languages(&self) -> Vec<Language> {
        Self::supported_languages()
    }
}

/// Builder for TreeSitterParser with fluent configuration
pub struct TreeSitterParserBuilder {
    builder_config: ASTBuilderConfig,
    enable_optimization: bool,
    enable_analysis: bool,
}

impl TreeSitterParserBuilder {
    pub fn new() -> Self {
        Self {
            builder_config: ASTBuilderConfig::default(),
            enable_optimization: true,
            enable_analysis: true,
        }
    }

    pub fn include_comments(mut self, include: bool) -> Self {
        self.builder_config.include_comments = include;
        self
    }

    pub fn include_whitespace(mut self, include: bool) -> Self {
        self.builder_config.include_whitespace = include;
        self
    }

    pub fn max_text_length(mut self, length: usize) -> Self {
        self.builder_config.max_text_length = length;
        self
    }

    pub fn extract_signatures(mut self, extract: bool) -> Self {
        self.builder_config.extract_signatures = extract;
        self
    }

    pub fn build_symbol_table(mut self, build: bool) -> Self {
        self.builder_config.build_symbol_table = build;
        self
    }

    pub fn enable_optimization(mut self, enable: bool) -> Self {
        self.enable_optimization = enable;
        self
    }

    pub fn enable_analysis(mut self, enable: bool) -> Self {
        self.enable_analysis = enable;
        self
    }

    pub fn build(self) -> Result<TreeSitterParser, ParseError> {
        let mut parser = TreeSitterParser::with_config(self.builder_config)?;
        parser.enable_optimization = self.enable_optimization;
        parser.enable_analysis = self.enable_analysis;
        Ok(parser)
    }
}

impl Default for TreeSitterParserBuilder {
    fn default() -> Self {
        Self::new()
    }
}
