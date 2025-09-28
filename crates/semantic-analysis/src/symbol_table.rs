//! Symbol table implementation for tracking declarations and references

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use indexmap::IndexMap;

/// Symbol table that tracks all symbols in a codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolTable {
    /// Global symbols accessible from anywhere
    pub global_symbols: HashMap<String, Symbol>,
    /// File-scoped symbols organized by file path
    pub file_symbols: HashMap<String, HashMap<String, Symbol>>,
    /// Scoped symbols organized by scope hierarchy
    pub scoped_symbols: IndexMap<ScopeId, Scope>,
    /// Next available scope ID
    next_scope_id: usize,
}

/// Represents a symbol (variable, function, class, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub symbol_kind: SymbolKind,
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub scope_id: ScopeId,
    pub type_info: Option<String>,
    pub references: Vec<SymbolReference>,
}

/// Types of symbols
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SymbolKind {
    Function,
    Method,
    Class,
    Interface,
    Variable,
    Constant,
    Parameter,
    Field,
    Module,
    Namespace,
}

/// Represents a scope in the code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    pub id: ScopeId,
    pub parent_id: Option<ScopeId>,
    pub scope_type: ScopeType,
    pub symbols: HashMap<String, Symbol>,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
}

/// Types of scopes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScopeType {
    Global,
    File,
    Class,
    Function,
    Block,
    Module,
}

/// Unique identifier for a scope
pub type ScopeId = usize;

/// Reference to a symbol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SymbolReference {
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub reference_type: ReferenceType,
}

/// Types of symbol references
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReferenceType {
    Declaration,
    Definition,
    Usage,
    Call,
    Assignment,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            global_symbols: HashMap::new(),
            file_symbols: HashMap::new(),
            scoped_symbols: IndexMap::new(),
            next_scope_id: 0,
        }
    }
    
    /// Create a new scope
    pub fn create_scope(&mut self, parent_id: Option<ScopeId>, scope_type: ScopeType, 
                       file_path: String, start_line: usize, end_line: usize) -> ScopeId {
        let scope_id = self.next_scope_id;
        self.next_scope_id += 1;
        
        let scope = Scope {
            id: scope_id,
            parent_id,
            scope_type,
            symbols: HashMap::new(),
            file_path,
            start_line,
            end_line,
        };
        
        self.scoped_symbols.insert(scope_id, scope);
        scope_id
    }
    
    /// Add a symbol to the table
    pub fn add_symbol(&mut self, symbol: Symbol) {
        let name = symbol.name.clone();
        let scope_id = symbol.scope_id;
        
        // Add to appropriate scope
        if let Some(scope) = self.scoped_symbols.get_mut(&scope_id) {
            scope.symbols.insert(name.clone(), symbol.clone());
        }
        
        // Add to file symbols
        self.file_symbols
            .entry(symbol.file_path.clone())
            .or_insert_with(HashMap::new)
            .insert(name.clone(), symbol.clone());
        
        // Add to global symbols if it's a global symbol
        if matches!(symbol.symbol_kind, SymbolKind::Function | SymbolKind::Class | SymbolKind::Module) {
            self.global_symbols.insert(name, symbol);
        }
    }
    
    /// Find a symbol by name in the given scope
    pub fn find_symbol(&self, name: &str, scope_id: ScopeId) -> Option<&Symbol> {
        // Search in current scope and parent scopes
        let mut current_scope_id = Some(scope_id);
        
        while let Some(id) = current_scope_id {
            if let Some(scope) = self.scoped_symbols.get(&id) {
                if let Some(symbol) = scope.symbols.get(name) {
                    return Some(symbol);
                }
                current_scope_id = scope.parent_id;
            } else {
                break;
            }
        }
        
        // Search in global symbols
        self.global_symbols.get(name)
    }
    
    /// Get all symbols in a file
    pub fn get_file_symbols(&self, file_path: &str) -> Option<&HashMap<String, Symbol>> {
        self.file_symbols.get(file_path)
    }
    
    /// Add a reference to a symbol
    pub fn add_reference(&mut self, symbol_name: &str, reference: SymbolReference) {
        // Find the symbol and add the reference
        for symbols in self.file_symbols.values_mut() {
            if let Some(symbol) = symbols.get_mut(symbol_name) {
                symbol.references.push(reference.clone());
            }
        }
        
        if let Some(symbol) = self.global_symbols.get_mut(symbol_name) {
            symbol.references.push(reference);
        }
    }
    
    /// Get all references to a symbol
    pub fn get_references(&self, symbol_name: &str) -> Vec<&SymbolReference> {
        let mut references = Vec::new();
        
        // Check global symbols
        if let Some(symbol) = self.global_symbols.get(symbol_name) {
            references.extend(&symbol.references);
        }
        
        // Check file symbols
        for symbols in self.file_symbols.values() {
            if let Some(symbol) = symbols.get(symbol_name) {
                references.extend(&symbol.references);
            }
        }
        
        references
    }
    
    /// Get symbols by kind
    pub fn get_symbols_by_kind(&self, kind: SymbolKind) -> Vec<&Symbol> {
        let mut result = Vec::new();
        
        for symbols in self.file_symbols.values() {
            for symbol in symbols.values() {
                if symbol.symbol_kind == kind {
                    result.push(symbol);
                }
            }
        }
        
        result
    }
}

impl Symbol {
    pub fn new(name: String, symbol_kind: SymbolKind, file_path: String, 
               line: usize, column: usize, scope_id: ScopeId) -> Self {
        Self {
            name,
            symbol_kind,
            file_path,
            line,
            column,
            scope_id,
            type_info: None,
            references: Vec::new(),
        }
    }
    
    pub fn with_type_info(mut self, type_info: String) -> Self {
        self.type_info = Some(type_info);
        self
    }
    
    /// Check if this symbol is accessible from the given scope
    pub fn is_accessible_from(&self, scope_id: ScopeId, symbol_table: &SymbolTable) -> bool {
        // Global symbols are always accessible
        if matches!(self.symbol_kind, SymbolKind::Function | SymbolKind::Class | SymbolKind::Module) {
            return true;
        }
        
        // Check if the target scope is the same or a child of this symbol's scope
        let mut current_scope_id = Some(scope_id);
        
        while let Some(id) = current_scope_id {
            if id == self.scope_id {
                return true;
            }
            
            if let Some(scope) = symbol_table.scoped_symbols.get(&id) {
                current_scope_id = scope.parent_id;
            } else {
                break;
            }
        }
        
        false
    }
}
