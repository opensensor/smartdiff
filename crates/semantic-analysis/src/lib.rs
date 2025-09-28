//! Smart Code Diff Semantic Analysis
//! 
//! Semantic analysis engine that builds symbol tables, resolves references,
//! and extracts type information from parsed ASTs.

pub mod symbol_table;
pub mod symbol_resolver;
pub mod scope_manager;
pub mod type_system;
pub mod dependency_graph;
pub mod analyzer;

pub use symbol_table::{SymbolTable, Symbol, SymbolKind, Scope, SymbolReference, ReferenceType, ScopeType, ScopeId};
pub use symbol_resolver::{SymbolResolver, SymbolResolverConfig, ImportInfo, FileContext};
pub use scope_manager::{ScopeManager, ScopeResolution, ScopeAnalysis};
pub use type_system::{TypeInfo, TypeResolver, TypeEquivalence};
pub use dependency_graph::{DependencyGraph, DependencyNode, DependencyEdge};
pub use analyzer::{SemanticAnalyzer, AnalysisResult, AnalysisError};

/// Re-export commonly used types
pub type Result<T> = std::result::Result<T, AnalysisError>;

#[cfg(test)]
mod tests;
