//! Smart Code Diff Semantic Analysis
//! 
//! Semantic analysis engine that builds symbol tables, resolves references,
//! and extracts type information from parsed ASTs.

pub mod symbol_table;
pub mod symbol_resolver;
pub mod scope_manager;
pub mod type_system;
pub mod type_extractor;
pub mod type_dependency_graph;
pub mod comprehensive_dependency_graph;
pub mod function_signature_extractor;
pub mod dependency_graph;
pub mod analyzer;

pub use symbol_table::{SymbolTable, Symbol, SymbolKind, Scope, SymbolReference, ReferenceType, ScopeType, ScopeId};
pub use symbol_resolver::{SymbolResolver, SymbolResolverConfig, ImportInfo, FileContext};
pub use scope_manager::{ScopeManager, ScopeResolution, ScopeAnalysis};
pub use type_system::{TypeInfo, TypeResolver, TypeEquivalence, TypeSignature, TypeKind, FieldInfo, MethodInfo, Visibility};
pub use type_extractor::{TypeExtractor, TypeExtractorConfig, ExtractedTypeInfo, TypeExtractionResult};
pub use type_dependency_graph::{TypeDependencyGraphBuilder, TypeRelationship, TypeRelationshipType, TypeDependencyAnalysis, TypeCouplingMetrics};
pub use comprehensive_dependency_graph::{
    ComprehensiveDependencyGraphBuilder, DependencyAnalysisConfig, FileAnalysisContext,
    FunctionInfo, ClassInfo, VariableInfo, FunctionCallInfo, CallType,
    ComprehensiveDependencyAnalysis, ComprehensiveCouplingMetrics, DependencyHotspot
};
pub use function_signature_extractor::{
    FunctionSignatureExtractor, FunctionSignatureConfig, EnhancedFunctionSignature,
    FunctionParameter, GenericParameter, GenericVariance, FunctionType,
    FunctionComplexityMetrics, FunctionSignatureExtractionResult, ExtractionStats,
    FunctionSignatureSimilarity, SimilarityBreakdown
};
pub use dependency_graph::{DependencyGraph, DependencyNode, DependencyEdge, DependencyNodeType, DependencyEdgeType};
pub use analyzer::{SemanticAnalyzer, AnalysisResult, AnalysisError};

/// Re-export commonly used types
pub type Result<T> = std::result::Result<T, AnalysisError>;

#[cfg(test)]
mod tests;
