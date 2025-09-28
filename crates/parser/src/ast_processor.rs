//! AST post-processing utilities for analysis and optimization

use crate::ast::{ASTNode, NodeType};
use crate::language::Language;
use std::collections::HashMap;

/// Statistics about the AST structure
#[derive(Debug, Default, Clone)]
pub struct ASTAnalysis {
    pub total_nodes: usize,
    pub node_type_counts: HashMap<NodeType, usize>,
    pub max_depth: usize,
    pub avg_depth: f64,
    pub function_count: usize,
    pub class_count: usize,
    pub complexity_score: f64,
    pub cyclomatic_complexity: usize,
}

/// AST processor for analysis and optimization
pub struct ASTProcessor {
    #[allow(dead_code)]
    language: Language,
}

impl ASTProcessor {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    /// Perform comprehensive analysis of the AST
    pub fn analyze(&self, ast: &ASTNode) -> ASTAnalysis {
        let mut analysis = ASTAnalysis::default();
        let mut depths = Vec::new();

        self.analyze_node(ast, 0, &mut analysis, &mut depths);

        // Calculate average depth
        if !depths.is_empty() {
            analysis.avg_depth = depths.iter().sum::<usize>() as f64 / depths.len() as f64;
        }

        // Calculate complexity score based on various factors
        analysis.complexity_score = self.calculate_complexity_score(&analysis);

        analysis
    }

    /// Recursively analyze a node and its children
    #[allow(clippy::only_used_in_recursion)]
    fn analyze_node(
        &self,
        node: &ASTNode,
        depth: usize,
        analysis: &mut ASTAnalysis,
        depths: &mut Vec<usize>,
    ) {
        analysis.total_nodes += 1;
        depths.push(depth);

        if depth > analysis.max_depth {
            analysis.max_depth = depth;
        }

        // Count node types
        *analysis.node_type_counts.entry(node.node_type).or_insert(0) += 1;

        // Count specific constructs
        match node.node_type {
            NodeType::Function | NodeType::Method | NodeType::Constructor => {
                analysis.function_count += 1;
            }
            NodeType::Class | NodeType::Interface => {
                analysis.class_count += 1;
            }
            NodeType::IfStatement | NodeType::WhileLoop | NodeType::ForLoop => {
                analysis.cyclomatic_complexity += 1;
            }
            _ => {}
        }

        // Recursively analyze children
        for child in &node.children {
            self.analyze_node(child, depth + 1, analysis, depths);
        }
    }

    /// Calculate a complexity score for the AST
    fn calculate_complexity_score(&self, analysis: &ASTAnalysis) -> f64 {
        let mut score = 0.0;

        // Base complexity from node count
        score += analysis.total_nodes as f64 * 0.1;

        // Depth penalty
        score += analysis.max_depth as f64 * 2.0;

        // Cyclomatic complexity penalty
        score += analysis.cyclomatic_complexity as f64 * 5.0;

        // Function complexity
        score += analysis.function_count as f64 * 3.0;

        // Class complexity
        score += analysis.class_count as f64 * 2.0;

        score
    }

    /// Optimize the AST by removing redundant nodes and simplifying structure
    pub fn optimize(&self, ast: &mut ASTNode) -> OptimizationResult {
        let mut result = OptimizationResult::default();

        self.optimize_node(ast, &mut result);

        result
    }

    /// Recursively optimize a node and its children
    fn optimize_node(&self, node: &mut ASTNode, result: &mut OptimizationResult) {
        // Remove empty or redundant children
        let original_child_count = node.children.len();
        node.children.retain(|child| !self.is_redundant_node(child));
        result.nodes_removed += original_child_count - node.children.len();

        // Flatten single-child wrapper nodes
        self.flatten_wrapper_nodes(node, result);

        // Recursively optimize children
        for child in &mut node.children {
            self.optimize_node(child, result);
        }

        // Merge consecutive literal nodes
        self.merge_consecutive_literals(node, result);
    }

    /// Check if a node is redundant and can be removed
    fn is_redundant_node(&self, node: &ASTNode) -> bool {
        match node.node_type {
            NodeType::Unknown => {
                // Remove unknown nodes with no children and no meaningful attributes
                node.children.is_empty()
                    && !node.metadata.attributes.contains_key("name")
                    && node.metadata.original_text.trim().is_empty()
            }
            _ => false,
        }
    }

    /// Flatten wrapper nodes that only contain a single child
    fn flatten_wrapper_nodes(&self, node: &mut ASTNode, result: &mut OptimizationResult) {
        let mut i = 0;
        while i < node.children.len() {
            let should_flatten = {
                let child = &node.children[i];
                self.is_wrapper_node(child) && child.children.len() == 1
            };

            if should_flatten {
                let wrapper_child = node.children.remove(i);
                if let Some(grandchild) = wrapper_child.children.into_iter().next() {
                    node.children.insert(i, grandchild);
                    result.nodes_flattened += 1;
                }
            } else {
                i += 1;
            }
        }
    }

    /// Check if a node is a wrapper that can be flattened
    fn is_wrapper_node(&self, node: &ASTNode) -> bool {
        match node.node_type {
            NodeType::Block => {
                // Flatten blocks that only contain expression statements
                node.children.len() == 1
                    && matches!(node.children[0].node_type, NodeType::ExpressionStatement)
            }
            NodeType::ExpressionStatement => {
                // Flatten expression statements that only wrap a single expression
                node.children.len() == 1
            }
            _ => false,
        }
    }

    /// Merge consecutive literal nodes
    fn merge_consecutive_literals(&self, node: &mut ASTNode, result: &mut OptimizationResult) {
        if node.children.len() < 2 {
            return;
        }

        let mut i = 0;
        while i < node.children.len() - 1 {
            let can_merge = {
                let current = &node.children[i];
                let next = &node.children[i + 1];

                matches!(current.node_type, NodeType::Literal)
                    && matches!(next.node_type, NodeType::Literal)
                    && self.are_mergeable_literals(current, next)
            };

            if can_merge {
                let next_node = node.children.remove(i + 1);
                let current_node = &mut node.children[i];

                // Merge the text content
                if let (Some(current_text), Some(next_text)) = (
                    current_node.metadata.attributes.get("text"),
                    next_node.metadata.attributes.get("text"),
                ) {
                    let merged_text = format!("{}{}", current_text, next_text);
                    current_node
                        .metadata
                        .attributes
                        .insert("text".to_string(), merged_text);
                }

                result.nodes_merged += 1;
            } else {
                i += 1;
            }
        }
    }

    /// Check if two literal nodes can be merged
    fn are_mergeable_literals(&self, node1: &ASTNode, node2: &ASTNode) -> bool {
        // Only merge string literals that are adjacent
        node1.metadata.line == node2.metadata.line || node1.metadata.line + 1 == node2.metadata.line
    }

    /// Extract function signatures from the AST
    pub fn extract_function_signatures(&self, ast: &ASTNode) -> Vec<FunctionSignatureInfo> {
        let mut signatures = Vec::new();
        self.extract_signatures_from_node(ast, &mut signatures);
        signatures
    }

    /// Recursively extract function signatures
    fn extract_signatures_from_node(
        &self,
        node: &ASTNode,
        signatures: &mut Vec<FunctionSignatureInfo>,
    ) {
        match node.node_type {
            NodeType::Function | NodeType::Method | NodeType::Constructor => {
                if let Some(signature) = self.create_function_signature(node) {
                    signatures.push(signature);
                }
            }
            _ => {}
        }

        for child in &node.children {
            self.extract_signatures_from_node(child, signatures);
        }
    }

    /// Create function signature information from a function node
    fn create_function_signature(&self, node: &ASTNode) -> Option<FunctionSignatureInfo> {
        let name = node.metadata.attributes.get("name")?.clone();
        let return_type = node.metadata.attributes.get("return_type").cloned();
        let parameter_count = node
            .metadata
            .attributes
            .get("parameter_count")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let modifiers = node
            .metadata
            .attributes
            .get("modifiers")
            .map(|s| s.split(',').map(|m| m.to_string()).collect())
            .unwrap_or_default();

        Some(FunctionSignatureInfo {
            name,
            return_type,
            parameter_count,
            modifiers,
            line: node.metadata.line,
            column: node.metadata.column,
            node_type: node.node_type,
        })
    }

    /// Build a symbol table from the AST
    pub fn build_symbol_table(&self, ast: &ASTNode) -> SymbolTable {
        let mut symbol_table = SymbolTable::new();
        self.collect_symbols(ast, &mut symbol_table, Vec::new());
        symbol_table
    }

    /// Recursively collect symbols from the AST
    #[allow(clippy::only_used_in_recursion)]
    fn collect_symbols(
        &self,
        node: &ASTNode,
        symbol_table: &mut SymbolTable,
        scope_path: Vec<String>,
    ) {
        match node.node_type {
            NodeType::Function | NodeType::Method | NodeType::Constructor => {
                if let Some(name) = node.metadata.attributes.get("name") {
                    let symbol = Symbol {
                        name: name.clone(),
                        symbol_type: SymbolType::Function,
                        scope_path: scope_path.clone(),
                        line: node.metadata.line,
                        column: node.metadata.column,
                        attributes: node.metadata.attributes.clone(),
                    };
                    symbol_table.add_symbol(symbol);
                }
            }
            NodeType::Class | NodeType::Interface => {
                if let Some(name) = node.metadata.attributes.get("name") {
                    let symbol = Symbol {
                        name: name.clone(),
                        symbol_type: SymbolType::Class,
                        scope_path: scope_path.clone(),
                        line: node.metadata.line,
                        column: node.metadata.column,
                        attributes: node.metadata.attributes.clone(),
                    };
                    symbol_table.add_symbol(symbol);

                    // Create new scope for class members
                    let mut new_scope = scope_path.clone();
                    new_scope.push(name.clone());

                    for child in &node.children {
                        self.collect_symbols(child, symbol_table, new_scope.clone());
                    }
                    return; // Don't process children again
                }
            }
            NodeType::VariableDeclaration | NodeType::FieldDeclaration => {
                if let Some(name) = node.metadata.attributes.get("name") {
                    let symbol = Symbol {
                        name: name.clone(),
                        symbol_type: SymbolType::Variable,
                        scope_path: scope_path.clone(),
                        line: node.metadata.line,
                        column: node.metadata.column,
                        attributes: node.metadata.attributes.clone(),
                    };
                    symbol_table.add_symbol(symbol);
                }
            }
            _ => {}
        }

        // Process children with current scope
        for child in &node.children {
            self.collect_symbols(child, symbol_table, scope_path.clone());
        }
    }
}

/// Result of AST optimization
#[derive(Debug, Default)]
pub struct OptimizationResult {
    pub nodes_removed: usize,
    pub nodes_flattened: usize,
    pub nodes_merged: usize,
}

/// Function signature information extracted from AST
#[derive(Debug, Clone)]
pub struct FunctionSignatureInfo {
    pub name: String,
    pub return_type: Option<String>,
    pub parameter_count: usize,
    pub modifiers: Vec<String>,
    pub line: usize,
    pub column: usize,
    pub node_type: NodeType,
}

/// Symbol table for tracking declarations
#[derive(Debug, Default)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
    scope_map: HashMap<Vec<String>, Vec<usize>>, // scope -> symbol indices
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        let index = self.symbols.len();
        let scope = symbol.scope_path.clone();

        self.symbols.push(symbol);
        self.scope_map.entry(scope).or_default().push(index);
    }

    pub fn get_symbols_in_scope(&self, scope: &[String]) -> Vec<&Symbol> {
        self.scope_map
            .get(scope)
            .map(|indices| indices.iter().map(|&i| &self.symbols[i]).collect())
            .unwrap_or_default()
    }

    pub fn find_symbol(&self, name: &str, scope: &[String]) -> Option<&Symbol> {
        // Search in current scope first, then parent scopes
        for i in (0..=scope.len()).rev() {
            let search_scope = &scope[..i];
            if let Some(symbols) = self.scope_map.get(search_scope) {
                for &index in symbols {
                    if self.symbols[index].name == name {
                        return Some(&self.symbols[index]);
                    }
                }
            }
        }
        None
    }

    pub fn all_symbols(&self) -> &[Symbol] {
        &self.symbols
    }
}

/// Symbol information
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub scope_path: Vec<String>,
    pub line: usize,
    pub column: usize,
    pub attributes: HashMap<String, String>,
}

/// Types of symbols
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Function,
    Class,
    Variable,
    Parameter,
    Field,
}
