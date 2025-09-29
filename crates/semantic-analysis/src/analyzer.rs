//! Main semantic analyzer

use crate::dependency_graph::DependencyGraph;
use crate::symbol_table::SymbolTable;
use crate::type_system::TypeResolver;
use smart_diff_parser::{ASTNode, ParseResult};
use thiserror::Error;

/// Semantic analyzer that processes ASTs and builds semantic information
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    type_resolver: TypeResolver,
    dependency_graph: DependencyGraph,
}

/// Result of semantic analysis
#[derive(Debug)]
pub struct AnalysisResult {
    pub symbol_table: SymbolTable,
    pub type_resolver: TypeResolver,
    pub dependency_graph: DependencyGraph,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Semantic analysis errors
#[derive(Error, Debug)]
pub enum AnalysisError {
    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Scope error: {0}")]
    ScopeError(String),

    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            type_resolver: TypeResolver::new(),
            dependency_graph: DependencyGraph::new(),
        }
    }

    /// Analyze a parsed file
    pub fn analyze(&mut self, parse_result: &ParseResult) -> Result<AnalysisResult, AnalysisError> {
        let mut errors = Vec::new();
        let warnings = Vec::new();

        // First pass: collect symbols and types
        if let Err(e) = self.collect_symbols(&parse_result.ast, &parse_result.language.to_string())
        {
            errors.push(format!("Symbol collection failed: {}", e));
        }

        // Second pass: resolve references and build dependency graph
        if let Err(e) = self.resolve_references(&parse_result.ast) {
            errors.push(format!("Reference resolution failed: {}", e));
        }

        // Third pass: type checking
        if let Err(e) = self.check_types(&parse_result.ast) {
            errors.push(format!("Type checking failed: {}", e));
        }

        Ok(AnalysisResult {
            symbol_table: self.symbol_table.clone(),
            type_resolver: self.type_resolver.clone(),
            dependency_graph: self.dependency_graph.clone(),
            errors,
            warnings,
        })
    }

    fn collect_symbols(&mut self, ast: &ASTNode, file_path: &str) -> Result<(), AnalysisError> {
        // Traverse the AST and collect symbols
        self.collect_symbols_recursive(ast, file_path, &mut Vec::new())?;
        Ok(())
    }

    fn collect_symbols_recursive(&mut self, node: &ASTNode, file_path: &str, _scope_path: &mut Vec<String>) -> Result<(), AnalysisError> {
        use crate::symbol_table::{Symbol, SymbolKind};
        use smart_diff_parser::ast::NodeType;

        match node.node_type {
            NodeType::Function | NodeType::Method | NodeType::Constructor => {
                if let Some(name) = node.metadata.attributes.get("name") {

                    let symbol = Symbol {
                        name: name.clone(),
                        symbol_kind: SymbolKind::Function,
                        scope_id: 0, // Use global scope for now
                        line: node.metadata.line,
                        column: node.metadata.column,
                        file_path: file_path.to_string(),
                        type_info: node.metadata.attributes.get("return_type").cloned(),
                        references: Vec::new(),
                    };

                    self.symbol_table.add_symbol(symbol);
                }
            }
            NodeType::VariableDeclaration | NodeType::FieldDeclaration => {
                if let Some(name) = node.metadata.attributes.get("name") {

                    let symbol = Symbol {
                        name: name.clone(),
                        symbol_kind: SymbolKind::Variable,
                        scope_id: 0, // Use global scope for now
                        line: node.metadata.line,
                        column: node.metadata.column,
                        file_path: file_path.to_string(),
                        type_info: node.metadata.attributes.get("type").cloned(),
                        references: Vec::new(),
                    };

                    self.symbol_table.add_symbol(symbol);
                }
            }
            NodeType::Class | NodeType::Interface => {
                if let Some(name) = node.metadata.attributes.get("name") {
                    let symbol = Symbol {
                        name: name.clone(),
                        symbol_kind: SymbolKind::Class,
                        scope_id: 0, // Use global scope for now
                        line: node.metadata.line,
                        column: node.metadata.column,
                        file_path: file_path.to_string(),
                        type_info: Some("class".to_string()),
                        references: Vec::new(),
                    };

                    self.symbol_table.add_symbol(symbol);
                }
            }
            _ => {}
        }

        // Recursively process children
        for child in &node.children {
            self.collect_symbols_recursive(child, file_path, _scope_path)?;
        }

        Ok(())
    }

    fn resolve_references(&mut self, _ast: &ASTNode) -> Result<(), AnalysisError> {
        // TODO: Implement reference resolution
        // This would find all symbol references and link them to declarations
        Ok(())
    }

    fn check_types(&mut self, _ast: &ASTNode) -> Result<(), AnalysisError> {
        // TODO: Implement type checking
        // This would verify type compatibility and catch type errors
        Ok(())
    }
}
