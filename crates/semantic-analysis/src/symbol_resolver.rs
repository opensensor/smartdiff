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

    /// Parse Java import statement
    fn parse_java_import(&self, import_text: &str, line: usize, column: usize) -> Result<ImportInfo> {
        let trimmed = import_text.trim();
        let is_static = trimmed.contains("static");
        let is_wildcard = trimmed.ends_with("*;");

        // Extract the imported name
        let import_part = if is_static {
            trimmed.strip_prefix("import static").unwrap_or(trimmed)
        } else {
            trimmed.strip_prefix("import").unwrap_or(trimmed)
        }.trim().trim_end_matches(';');

        let imported_name = if is_wildcard {
            import_part.trim_end_matches(".*").to_string()
        } else {
            import_part.to_string()
        };

        Ok(ImportInfo {
            imported_name,
            source_path: None, // Java uses package names, not file paths
            alias: None,
            is_wildcard,
            line,
            column,
        })
    }

    /// Parse Python import statement
    fn parse_python_import(&self, import_text: &str, line: usize, column: usize) -> Result<ImportInfo> {
        let trimmed = import_text.trim();

        if trimmed.starts_with("from") {
            // from module import name [as alias]
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 4 && parts[2] == "import" {
                let module = parts[1];
                let imported = parts[3];
                let alias = if parts.len() >= 6 && parts[4] == "as" {
                    Some(parts[5].to_string())
                } else {
                    None
                };

                Ok(ImportInfo {
                    imported_name: imported.to_string(),
                    source_path: Some(module.to_string()),
                    alias,
                    is_wildcard: imported == "*",
                    line,
                    column,
                })
            } else {
                Err(anyhow!("Invalid Python from-import statement: {}", trimmed))
            }
        } else if trimmed.starts_with("import") {
            // import module [as alias]
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let module = parts[1];
                let alias = if parts.len() >= 4 && parts[2] == "as" {
                    Some(parts[3].to_string())
                } else {
                    None
                };

                Ok(ImportInfo {
                    imported_name: module.to_string(),
                    source_path: None,
                    alias,
                    is_wildcard: false,
                    line,
                    column,
                })
            } else {
                Err(anyhow!("Invalid Python import statement: {}", trimmed))
            }
        } else {
            Err(anyhow!("Unknown Python import format: {}", trimmed))
        }
    }

    /// Parse JavaScript import statement
    fn parse_js_import(&self, import_text: &str, line: usize, column: usize) -> Result<ImportInfo> {
        let trimmed = import_text.trim();

        if trimmed.starts_with("import") {
            // ES6 import: import name from 'module'
            // or: import { name } from 'module'
            // or: import * as name from 'module'

            if let Some(from_pos) = trimmed.find(" from ") {
                let import_part = &trimmed[6..from_pos].trim(); // Skip "import"
                let module_part = &trimmed[from_pos + 6..].trim().trim_matches('\'').trim_matches('"');

                let (imported_name, alias, is_wildcard) = if import_part.starts_with('*') {
                    // import * as name from 'module'
                    let as_pos = import_part.find(" as ").ok_or_else(|| anyhow!("Invalid wildcard import"))?;
                    let alias_name = import_part[as_pos + 4..].trim();
                    ("*".to_string(), Some(alias_name.to_string()), true)
                } else if import_part.starts_with('{') && import_part.ends_with('}') {
                    // import { name } from 'module'
                    let inner = &import_part[1..import_part.len()-1].trim();
                    (inner.to_string(), None, false)
                } else {
                    // import name from 'module'
                    (import_part.to_string(), None, false)
                };

                Ok(ImportInfo {
                    imported_name,
                    source_path: Some(module_part.to_string()),
                    alias,
                    is_wildcard,
                    line,
                    column,
                })
            } else {
                Err(anyhow!("Invalid ES6 import statement: {}", trimmed))
            }
        } else if trimmed.contains("require(") {
            // CommonJS require: const name = require('module')
            if let Some(start) = trimmed.find("require('") {
                let module_start = start + 9;
                if let Some(end) = trimmed[module_start..].find("')") {
                    let module_name = &trimmed[module_start..module_start + end];

                    // Try to extract variable name
                    let var_name = if let Some(eq_pos) = trimmed.find('=') {
                        trimmed[..eq_pos].trim().split_whitespace().last().unwrap_or("unknown")
                    } else {
                        "unknown"
                    };

                    Ok(ImportInfo {
                        imported_name: var_name.to_string(),
                        source_path: Some(module_name.to_string()),
                        alias: None,
                        is_wildcard: false,
                        line,
                        column,
                    })
                } else {
                    Err(anyhow!("Invalid require statement: {}", trimmed))
                }
            } else {
                Err(anyhow!("Invalid require statement: {}", trimmed))
            }
        } else {
            Err(anyhow!("Unknown JavaScript import format: {}", trimmed))
        }
    }

    /// Parse C/C++ include statement
    fn parse_c_include(&self, include_text: &str, line: usize, column: usize) -> Result<ImportInfo> {
        let trimmed = include_text.trim();

        if trimmed.starts_with("#include") {
            let include_part = trimmed[8..].trim(); // Skip "#include"

            let (header_name, is_system) = if include_part.starts_with('<') && include_part.ends_with('>') {
                // System header: #include <stdio.h>
                (&include_part[1..include_part.len()-1], true)
            } else if include_part.starts_with('"') && include_part.ends_with('"') {
                // Local header: #include "myheader.h"
                (&include_part[1..include_part.len()-1], false)
            } else {
                return Err(anyhow!("Invalid include statement: {}", trimmed));
            };

            Ok(ImportInfo {
                imported_name: header_name.to_string(),
                source_path: if is_system { None } else { Some(header_name.to_string()) },
                alias: None,
                is_wildcard: false,
                line,
                column,
            })
        } else {
            Err(anyhow!("Not an include statement: {}", trimmed))
        }
    }

    /// Resolve cross-file references
    fn resolve_cross_file_references(&mut self) -> Result<()> {
        let file_paths: Vec<String> = self.file_contexts.keys().cloned().collect();

        for file_path in file_paths {
            self.resolve_file_references(&file_path)?;
        }

        Ok(())
    }

    /// Resolve references for a specific file
    fn resolve_file_references(&mut self, file_path: &str) -> Result<()> {
        let file_context = self.file_contexts.get(file_path).cloned();
        if let Some(context) = file_context {
            // Build import map for this file
            let mut import_map = HashMap::new();

            for import in &context.imports {
                if let Some(source_path) = &import.source_path {
                    // Try to resolve the source path to an actual file
                    if let Some(resolved_path) = self.resolve_import_path(source_path, file_path)? {
                        import_map.insert(import.imported_name.clone(), resolved_path);
                    }
                }
            }

            // Update import graph
            let imported_files: Vec<String> = import_map.values().cloned().collect();
            self.import_graph.insert(file_path.to_string(), imported_files);
        }

        Ok(())
    }

    /// Resolve import path to actual file path
    fn resolve_import_path(&self, import_path: &str, current_file: &str) -> Result<Option<String>> {
        let current_dir = Path::new(current_file).parent().unwrap_or(Path::new("."));

        // Try different resolution strategies
        let candidates = vec![
            // Relative to current file
            current_dir.join(import_path),
            current_dir.join(format!("{}.java", import_path)),
            current_dir.join(format!("{}.py", import_path)),
            current_dir.join(format!("{}.js", import_path)),
            current_dir.join(format!("{}.cpp", import_path)),
            current_dir.join(format!("{}.c", import_path)),
            current_dir.join(format!("{}.h", import_path)),

            // Direct path
            PathBuf::from(import_path),
        ];

        for candidate in candidates {
            if candidate.exists() {
                if let Some(path_str) = candidate.to_str() {
                    return Ok(Some(path_str.to_string()));
                }
            }
        }

        Ok(None)
    }

    /// Get symbol table
    pub fn get_symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    /// Get symbol table (mutable)
    pub fn get_symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    /// Find symbol across all files
    pub fn find_symbol(&self, name: &str, context_file: Option<&str>) -> Option<&Symbol> {
        // First try local file context if provided
        if let Some(file_path) = context_file {
            if let Some(file_symbols) = self.symbol_table.get_file_symbols(file_path) {
                if let Some(symbol) = file_symbols.get(name) {
                    return Some(symbol);
                }
            }
        }

        // Try qualified lookup
        if let Some(symbol) = self.symbol_table.find_qualified_symbol(name) {
            return Some(symbol);
        }

        // Try global symbols
        self.symbol_table.global_symbols.get(name)
    }

    /// Get import graph
    pub fn get_import_graph(&self) -> &HashMap<String, Vec<String>> {
        &self.import_graph
    }

    /// Get file context
    pub fn get_file_context(&self, file_path: &str) -> Option<&FileContext> {
        self.file_contexts.get(file_path)
    }

    /// Clear resolution cache
    pub fn clear_cache(&mut self) {
        self.resolution_cache.clear();
    }
}
