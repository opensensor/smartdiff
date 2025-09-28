//! Smart Code Diff Semantic Analysis
//!
//! Semantic analysis engine that builds symbol tables, resolves references,
//! and extracts type information from parsed ASTs.

pub mod analyzer;
pub mod comprehensive_dependency_graph;
pub mod dependency_graph;
pub mod function_signature_extractor;
pub mod scope_manager;
pub mod symbol_resolver;
pub mod symbol_table;
pub mod type_dependency_graph;
pub mod type_extractor;
pub mod type_system;

pub use analyzer::{AnalysisError, AnalysisResult, SemanticAnalyzer};
pub use comprehensive_dependency_graph::{
    CallType, ClassInfo, ComprehensiveCouplingMetrics, ComprehensiveDependencyAnalysis,
    ComprehensiveDependencyGraphBuilder, DependencyAnalysisConfig, DependencyHotspot,
    FileAnalysisContext, FunctionCallInfo, FunctionInfo, VariableInfo,
};
pub use dependency_graph::{
    DependencyEdge, DependencyEdgeType, DependencyGraph, DependencyNode, DependencyNodeType,
};
pub use function_signature_extractor::{
    EnhancedFunctionSignature, ExtractionStats, FunctionComplexityMetrics, FunctionParameter,
    FunctionSignatureConfig, FunctionSignatureExtractionResult, FunctionSignatureExtractor,
    FunctionSignatureSimilarity, FunctionType, GenericParameter, GenericVariance,
    SimilarityBreakdown,
};
pub use scope_manager::{ScopeAnalysis, ScopeManager, ScopeResolution};
pub use symbol_resolver::{FileContext, ImportInfo, SymbolResolver, SymbolResolverConfig};
pub use symbol_table::{
    ReferenceType, Scope, ScopeId, ScopeType, Symbol, SymbolKind, SymbolReference, SymbolTable,
};
pub use type_dependency_graph::{
    TypeCouplingMetrics, TypeDependencyAnalysis, TypeDependencyGraphBuilder, TypeRelationship,
    TypeRelationshipType,
};
pub use type_extractor::{
    ExtractedTypeInfo, TypeExtractionResult, TypeExtractor, TypeExtractorConfig,
};
pub use type_system::{
    FieldInfo, MethodInfo, TypeEquivalence, TypeInfo, TypeKind, TypeResolver, TypeSignature,
    Visibility,
};

/// Re-export commonly used types
pub type Result<T> = std::result::Result<T, AnalysisError>;

#[cfg(test)]
mod tests;
