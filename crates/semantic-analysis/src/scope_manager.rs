//! Scope management for hierarchical symbol resolution

use crate::symbol_table::{Scope, ScopeId, ScopeType, Symbol, SymbolKind};
use smart_diff_parser::Language;
use std::collections::HashMap;

/// Manages scope hierarchy and symbol visibility
pub struct ScopeManager {
    scopes: HashMap<ScopeId, Scope>,
    scope_stack: Vec<ScopeId>,
    next_scope_id: ScopeId,
    #[allow(dead_code)]
    language: Language,
}

/// Scope resolution result
#[derive(Debug, Clone)]
pub struct ScopeResolution {
    pub symbol: Symbol,
    pub scope_id: ScopeId,
    pub resolution_path: Vec<ScopeId>,
    pub is_shadowed: bool,
}

/// Scope analysis metrics
#[derive(Debug, Default)]
pub struct ScopeAnalysis {
    pub total_scopes: usize,
    pub max_depth: usize,
    pub avg_symbols_per_scope: f64,
    pub scope_type_counts: HashMap<ScopeType, usize>,
    pub shadowed_symbols: Vec<String>,
}

impl ScopeManager {
    pub fn new(language: Language) -> Self {
        Self {
            scopes: HashMap::new(),
            scope_stack: Vec::new(),
            next_scope_id: 0,
            language,
        }
    }

    /// Create a new scope
    pub fn create_scope(
        &mut self,
        scope_type: ScopeType,
        file_path: String,
        start_line: usize,
        end_line: usize,
    ) -> ScopeId {
        let scope_id = self.next_scope_id;
        self.next_scope_id += 1;

        let parent_id = self.scope_stack.last().copied();

        let scope = Scope {
            id: scope_id,
            parent_id,
            scope_type,
            symbols: HashMap::new(),
            file_path,
            start_line,
            end_line,
        };

        self.scopes.insert(scope_id, scope);
        scope_id
    }

    /// Enter a scope (push to stack)
    pub fn enter_scope(&mut self, scope_id: ScopeId) {
        self.scope_stack.push(scope_id);
    }

    /// Exit current scope (pop from stack)
    pub fn exit_scope(&mut self) -> Option<ScopeId> {
        self.scope_stack.pop()
    }

    /// Get current scope ID
    pub fn current_scope(&self) -> Option<ScopeId> {
        self.scope_stack.last().copied()
    }

    /// Add symbol to current scope
    pub fn add_symbol_to_current_scope(&mut self, symbol: Symbol) -> Result<(), String> {
        if let Some(scope_id) = self.current_scope() {
            self.add_symbol_to_scope(scope_id, symbol)
        } else {
            Err("No current scope".to_string())
        }
    }

    /// Add symbol to specific scope
    pub fn add_symbol_to_scope(&mut self, scope_id: ScopeId, symbol: Symbol) -> Result<(), String> {
        if let Some(scope) = self.scopes.get_mut(&scope_id) {
            // Check for shadowing
            if scope.symbols.contains_key(&symbol.name) {
                // Symbol already exists in this scope - this is redefinition
                return Err(format!("Symbol '{}' already defined in scope", symbol.name));
            }

            scope.symbols.insert(symbol.name.clone(), symbol);
            Ok(())
        } else {
            Err(format!("Scope {} not found", scope_id))
        }
    }

    /// Resolve symbol in current scope chain
    pub fn resolve_symbol(&self, name: &str) -> Option<ScopeResolution> {
        if let Some(current_scope_id) = self.current_scope() {
            self.resolve_symbol_from_scope(name, current_scope_id)
        } else {
            None
        }
    }

    /// Resolve symbol starting from specific scope
    pub fn resolve_symbol_from_scope(
        &self,
        name: &str,
        scope_id: ScopeId,
    ) -> Option<ScopeResolution> {
        let mut resolution_path = Vec::new();
        let mut current_scope_id = Some(scope_id);
        let mut is_shadowed = false;
        let mut found_count = 0;

        while let Some(id) = current_scope_id {
            resolution_path.push(id);

            if let Some(scope) = self.scopes.get(&id) {
                if let Some(symbol) = scope.symbols.get(name) {
                    found_count += 1;

                    // If we found more than one, the first one shadows the others
                    if found_count > 1 {
                        is_shadowed = true;
                    }

                    return Some(ScopeResolution {
                        symbol: symbol.clone(),
                        scope_id: id,
                        resolution_path,
                        is_shadowed,
                    });
                }

                current_scope_id = scope.parent_id;
            } else {
                break;
            }
        }

        None
    }

    /// Find all symbols with given name (including shadowed ones)
    pub fn find_all_symbols(&self, name: &str) -> Vec<ScopeResolution> {
        let mut results = Vec::new();

        if let Some(current_scope_id) = self.current_scope() {
            let mut current_scope_id = Some(current_scope_id);
            let mut resolution_path = Vec::new();

            while let Some(id) = current_scope_id {
                resolution_path.push(id);

                if let Some(scope) = self.scopes.get(&id) {
                    if let Some(symbol) = scope.symbols.get(name) {
                        results.push(ScopeResolution {
                            symbol: symbol.clone(),
                            scope_id: id,
                            resolution_path: resolution_path.clone(),
                            is_shadowed: !results.is_empty(), // First one is not shadowed
                        });
                    }

                    current_scope_id = scope.parent_id;
                } else {
                    break;
                }
            }
        }

        results
    }

    /// Get all symbols in current scope
    pub fn get_current_scope_symbols(&self) -> Option<&HashMap<String, Symbol>> {
        if let Some(scope_id) = self.current_scope() {
            self.scopes.get(&scope_id).map(|scope| &scope.symbols)
        } else {
            None
        }
    }

    /// Get all symbols in specific scope
    pub fn get_scope_symbols(&self, scope_id: ScopeId) -> Option<&HashMap<String, Symbol>> {
        self.scopes.get(&scope_id).map(|scope| &scope.symbols)
    }

    /// Get scope information
    pub fn get_scope(&self, scope_id: ScopeId) -> Option<&Scope> {
        self.scopes.get(&scope_id)
    }

    /// Get scope hierarchy from root to current
    pub fn get_scope_hierarchy(&self) -> Vec<ScopeId> {
        self.scope_stack.clone()
    }

    /// Check if symbol is visible from current scope
    pub fn is_symbol_visible(&self, _symbol_name: &str, symbol_scope_id: ScopeId) -> bool {
        // Symbol is visible if its scope is in the current scope chain
        if let Some(current_scope_id) = self.current_scope() {
            self.is_scope_ancestor(symbol_scope_id, current_scope_id)
                || symbol_scope_id == current_scope_id
        } else {
            false
        }
    }

    /// Check if one scope is an ancestor of another
    fn is_scope_ancestor(&self, ancestor_id: ScopeId, descendant_id: ScopeId) -> bool {
        let mut current_id = Some(descendant_id);

        while let Some(id) = current_id {
            if id == ancestor_id {
                return true;
            }

            if let Some(scope) = self.scopes.get(&id) {
                current_id = scope.parent_id;
            } else {
                break;
            }
        }

        false
    }

    /// Analyze scope structure
    pub fn analyze_scopes(&self) -> ScopeAnalysis {
        let mut analysis = ScopeAnalysis {
            total_scopes: self.scopes.len(),
            ..Default::default()
        };

        let mut total_symbols = 0;
        let mut max_depth = 0;
        let mut shadowed_symbols = Vec::new();

        for scope in self.scopes.values() {
            total_symbols += scope.symbols.len();

            // Count scope types
            *analysis
                .scope_type_counts
                .entry(scope.scope_type.clone())
                .or_insert(0) += 1;

            // Calculate depth
            let depth = self.calculate_scope_depth(scope.id);
            if depth > max_depth {
                max_depth = depth;
            }

            // Check for shadowed symbols
            for symbol_name in scope.symbols.keys() {
                if let Some(parent_id) = scope.parent_id {
                    if self.symbol_exists_in_ancestor(symbol_name, parent_id) {
                        shadowed_symbols.push(symbol_name.clone());
                    }
                }
            }
        }

        analysis.max_depth = max_depth;
        analysis.shadowed_symbols = shadowed_symbols;

        if analysis.total_scopes > 0 {
            analysis.avg_symbols_per_scope = total_symbols as f64 / analysis.total_scopes as f64;
        }

        analysis
    }

    /// Calculate depth of a scope from root
    fn calculate_scope_depth(&self, scope_id: ScopeId) -> usize {
        let mut depth = 0;
        let mut current_id = Some(scope_id);

        while let Some(id) = current_id {
            if let Some(scope) = self.scopes.get(&id) {
                current_id = scope.parent_id;
                depth += 1;
            } else {
                break;
            }
        }

        depth
    }

    /// Check if symbol exists in ancestor scopes
    fn symbol_exists_in_ancestor(&self, symbol_name: &str, ancestor_id: ScopeId) -> bool {
        let mut current_id = Some(ancestor_id);

        while let Some(id) = current_id {
            if let Some(scope) = self.scopes.get(&id) {
                if scope.symbols.contains_key(symbol_name) {
                    return true;
                }
                current_id = scope.parent_id;
            } else {
                break;
            }
        }

        false
    }

    /// Get symbols by kind in current scope chain
    pub fn get_symbols_by_kind(&self, kind: SymbolKind) -> Vec<&Symbol> {
        let mut symbols = Vec::new();

        if let Some(current_scope_id) = self.current_scope() {
            let mut current_id = Some(current_scope_id);

            while let Some(id) = current_id {
                if let Some(scope) = self.scopes.get(&id) {
                    for symbol in scope.symbols.values() {
                        if symbol.symbol_kind == kind {
                            symbols.push(symbol);
                        }
                    }
                    current_id = scope.parent_id;
                } else {
                    break;
                }
            }
        }

        symbols
    }

    /// Clear all scopes
    pub fn clear(&mut self) {
        self.scopes.clear();
        self.scope_stack.clear();
        self.next_scope_id = 0;
    }

    /// Get total number of scopes
    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

    /// Get current scope depth
    pub fn current_depth(&self) -> usize {
        self.scope_stack.len()
    }
}
