//! Smart Code Diff Parser
//!
//! Multi-language parser engine that converts source code into normalized AST representations
//! for structural and semantic comparison.

pub mod ast;
pub mod ast_builder;
pub mod ast_processor;
pub mod function;
pub mod language;
pub mod language_config;
pub mod matching;
pub mod parser;
pub mod tree_sitter;

pub use ast::{ASTNode, NodeMetadata, NodeType};
pub use ast_builder::{ASTBuilder, ASTBuilderBuilder, ASTBuilderConfig};
pub use ast_processor::{
    ASTAnalysis, ASTProcessor, FunctionSignatureInfo, Symbol, SymbolTable, SymbolType,
};
pub use function::{Function, FunctionLocation, FunctionSignature, Parameter, Type};
pub use language::{Language, LanguageDetector};
pub use matching::{
    Change, ChangeDetail, ChangeType, CodeElement, ElementType, MatchResult, RefactoringType,
};
pub use parser::{ParseError, ParseResult, Parser};

/// Re-export commonly used types
pub type Result<T> = std::result::Result<T, ParseError>;

#[cfg(test)]
mod tests;
