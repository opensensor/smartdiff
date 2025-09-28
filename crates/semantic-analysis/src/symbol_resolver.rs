//! Symbol resolution engine for cross-file symbol lookup and reference tracking

use crate::symbol_table::{SymbolTable, Symbol, SymbolKind, SymbolReference, ReferenceType, ScopeId, ScopeType};
use smart_diff_parser::{ASTNode, NodeType, Language, ParseResult};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};

/// Configuration for symbol resolution
#[derive(Debug, Clone)]
pub struct SymbolResolverConfig {
    /// Whether to resolve cross-file references
    pub resolve_cross_file: bool,
    /// Whether to track all symbol usages
    pub track_usages: bool,
    /// Whether to resolve import statements
    pub resolve_imports: bool,
    /// Maximum depth for recursive symbol resolution
    pub max_resolution_depth: usize,
    /// File extensions to consider for resolution
    pub file_extensions: HashSet<String>,
}

impl Default for SymbolResolverConfig {
    fn default() -> Self {
        let mut extensions = HashSet::new();
        extensions.insert("java".to_string());
        extensions.insert("py".to_string());
        extensions.insert("js".to_string());
        extensions.insert("cpp".to_string());
        extensions.insert("c".to_string());
        extensions.insert("h".to_string());
        
        Self {
            resolve_cross_file: true,
            track_usages: true,
            resolve_imports: true,
            max_resolution_depth: 10,
            file_extensions: extensions,
        }
    }
}

/// Import information extracted from code
#[derive(Debug, Clone)]
pub struct ImportInfo {
    pub imported_name: String,
    pub source_path: Option<String>,
    pub alias: Option<String>,
    pub is_wildcard: bool,
    pub line: usize,
    pub column: usize,
}

/// Symbol resolution context for a single file
#[derive(Debug)]
pub struct FileContext {
    pub file_path: String,
    pub language: Language,
    pub imports: Vec<ImportInfo>,
    pub exports: Vec<String>,
    pub local_scope_stack: Vec<ScopeId>,
}

/// Comprehensive symbol resolver
pub struct SymbolResolver {
    config: SymbolResolverConfig,
    symbol_table: SymbolTable,
    file_contexts: HashMap<String, FileContext>,
    resolution_cache: HashMap<String, Option<Symbol>>,
    import_graph: HashMap<String, Vec<String>>, // file -> imported files
}

impl SymbolResolver {
    pub fn new(config: SymbolResolverConfig) -> Self {
        Self {
            config,
            symbol_table: SymbolTable::new(),
            file_contexts: HashMap::new(),
            resolution_cache: HashMap::new(),
            import_graph: HashMap::new(),
        }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(SymbolResolverConfig::default())
    }
    
    /// Process a single file and add its symbols to the resolver
    pub fn process_file(&mut self, file_path: &str, parse_result: &ParseResult) -> Result<()> {
        let mut file_context = FileContext {
            file_path: file_path.to_string(),
            language: parse_result.language.clone(),
            imports: Vec::new(),
            exports: Vec::new(),
            local_scope_stack: Vec::new(),
        };
        
        // Create file scope
        let file_scope_id = self.symbol_table.create_scope(
            None,
            ScopeType::File,
            file_path.to_string(),
            1,
            usize::MAX,
        );
        file_context.local_scope_stack.push(file_scope_id);
        
        // Extract imports first
        self.extract_imports(&parse_result.ast, &mut file_context)?;
        
        // Process symbols in the AST
        self.process_ast_node(&parse_result.ast, &mut file_context, file_scope_id)?;
        
        // Store file context
        self.file_contexts.insert(file_path.to_string(), file_context);
        
        Ok(())
    }
    
    /// Process multiple files in dependency order
    pub fn process_files(&mut self, files: Vec<(String, ParseResult)>) -> Result<()> {
        // First pass: extract all symbols and imports
        for (file_path, parse_result) in &files {
            self.process_file(file_path, parse_result)?;
        }
        
        // Second pass: resolve cross-file references
        if self.config.resolve_cross_file {
            self.resolve_cross_file_references()?;
        }
        
        Ok(())
    }
    
    /// Extract import statements from AST
    fn extract_imports(&mut self, node: &ASTNode, file_context: &mut FileContext) -> Result<()> {
        match node.node_type {
            NodeType::Module => {
                // Handle different import patterns based on language
                match file_context.language {
                    Language::Java => self.extract_java_imports(node, file_context)?,
                    Language::Python => self.extract_python_imports(node, file_context)?,
                    Language::JavaScript => self.extract_js_imports(node, file_context)?,
                    Language::Cpp | Language::C => self.extract_c_includes(node, file_context)?,
                    _ => {}
                }
            }
            _ => {}
        }
        
        // Recursively process children
        for child in &node.children {
            self.extract_imports(child, file_context)?;
        }
        
        Ok(())
    }
    
    /// Extract Java import statements
    fn extract_java_imports(&mut self, node: &ASTNode, file_context: &mut FileContext) -> Result<()> {
        if let Some(import_text) = node.metadata.attributes.get("text") {
            if import_text.starts_with("import") {
                let import_info = self.parse_java_import(import_text, node.metadata.line, node.metadata.column)?;
                file_context.imports.push(import_info);
            }
        }
        Ok(())
    }
    
    /// Extract Python import statements
    fn extract_python_imports(&mut self, node: &ASTNode, file_context: &mut FileContext) -> Result<()> {
        if let Some(import_text) = node.metadata.attributes.get("text") {
            if import_text.starts_with("import") || import_text.starts_with("from") {
                let import_info = self.parse_python_import(import_text, node.metadata.line, node.metadata.column)?;
                file_context.imports.push(import_info);
            }
        }
        Ok(())
    }
    
    /// Extract JavaScript import statements
    fn extract_js_imports(&mut self, node: &ASTNode, file_context: &mut FileContext) -> Result<()> {
        if let Some(import_text) = node.metadata.attributes.get("text") {
            if import_text.starts_with("import") || import_text.contains("require(") {
                let import_info = self.parse_js_import(import_text, node.metadata.line, node.metadata.column)?;
                file_context.imports.push(import_info);
            }
        }
        Ok(())
    }
    
    /// Extract C/C++ include statements
    fn extract_c_includes(&mut self, node: &ASTNode, file_context: &mut FileContext) -> Result<()> {
        if let Some(include_text) = node.metadata.attributes.get("text") {
            if include_text.starts_with("#include") {
                let import_info = self.parse_c_include(include_text, node.metadata.line, node.metadata.column)?;
                file_context.imports.push(import_info);
            }
        }
        Ok(())
    }
    
    /// Process AST node and extract symbols
    fn process_ast_node(&mut self, node: &ASTNode, file_context: &mut FileContext, current_scope: ScopeId) -> Result<()> {
        let mut node_scope = current_scope;
        
        // Create new scope for certain node types
        match node.node_type {
            NodeType::Class | NodeType::Interface => {
                if let Some(name) = node.metadata.attributes.get("name") {
                    node_scope = self.symbol_table.create_scope(
                        Some(current_scope),
                        ScopeType::Class,
                        file_context.file_path.clone(),
                        node.metadata.line,
                        node.metadata.line + 100, // Estimate end line
                    );
                    
                    // Add class symbol
                    let symbol = Symbol {
                        name: name.clone(),
                        symbol_kind: if node.node_type == NodeType::Class { SymbolKind::Class } else { SymbolKind::Interface },
                        file_path: file_context.file_path.clone(),
                        line: node.metadata.line,
                        column: node.metadata.column,
                        scope_id: current_scope,
                        type_info: node.metadata.attributes.get("type").cloned(),
                        references: Vec::new(),
                    };
                    
                    self.symbol_table.add_symbol(symbol);
                }
            }
            NodeType::Function | NodeType::Method | NodeType::Constructor => {
                if let Some(name) = node.metadata.attributes.get("name") {
                    node_scope = self.symbol_table.create_scope(
                        Some(current_scope),
                        ScopeType::Function,
                        file_context.file_path.clone(),
                        node.metadata.line,
                        node.metadata.line + 50, // Estimate end line
                    );
                    
                    // Add function symbol
                    let symbol_kind = match node.node_type {
                        NodeType::Function => SymbolKind::Function,
                        NodeType::Method => SymbolKind::Method,
                        NodeType::Constructor => SymbolKind::Method,
                        _ => SymbolKind::Function,
                    };
                    
                    let symbol = Symbol {
                        name: name.clone(),
                        symbol_kind,
                        file_path: file_context.file_path.clone(),
                        line: node.metadata.line,
                        column: node.metadata.column,
                        scope_id: current_scope,
                        type_info: node.metadata.attributes.get("return_type").cloned(),
                        references: Vec::new(),
                    };
                    
                    self.symbol_table.add_symbol(symbol);
                }
            }
            NodeType::VariableDeclaration | NodeType::FieldDeclaration | NodeType::ParameterDeclaration => {
                if let Some(name) = node.metadata.attributes.get("name") {
                    let symbol_kind = match node.node_type {
                        NodeType::VariableDeclaration => SymbolKind::Variable,
                        NodeType::FieldDeclaration => SymbolKind::Field,
                        NodeType::ParameterDeclaration => SymbolKind::Parameter,
                        _ => SymbolKind::Variable,
                    };
                    
                    let symbol = Symbol {
                        name: name.clone(),
                        symbol_kind,
                        file_path: file_context.file_path.clone(),
                        line: node.metadata.line,
                        column: node.metadata.column,
                        scope_id: current_scope,
                        type_info: node.metadata.attributes.get("type").cloned(),
                        references: Vec::new(),
                    };
                    
                    self.symbol_table.add_symbol(symbol);
                }
            }
            NodeType::CallExpression => {
                // Track function calls as references
                if let Some(function_name) = node.metadata.attributes.get("function_name") {
                    let reference = SymbolReference {
                        file_path: file_context.file_path.clone(),
                        line: node.metadata.line,
                        column: node.metadata.column,
                        reference_type: ReferenceType::Call,
                    };
                    
                    self.symbol_table.add_reference(function_name, reference);
                }
            }
            NodeType::Identifier => {
                // Track identifier usage
                if self.config.track_usages {
                    if let Some(name) = node.metadata.attributes.get("name") {
                        let reference = SymbolReference {
                            file_path: file_context.file_path.clone(),
                            line: node.metadata.line,
                            column: node.metadata.column,
                            reference_type: ReferenceType::Usage,
                        };
                        
                        self.symbol_table.add_reference(name, reference);
                    }
                }
            }
            _ => {}
        }
        
        // Process children
        for child in &node.children {
            self.process_ast_node(child, file_context, node_scope)?;
        }
        
        Ok(())
    }
}
