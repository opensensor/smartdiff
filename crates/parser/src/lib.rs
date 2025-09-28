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

pub use ast::{ASTNode, NodeType, NodeMetadata};
pub use ast_builder::{ASTBuilder, ASTBuilderBuilder, ASTBuilderConfig};
pub use ast_processor::{ASTProcessor, ASTAnalysis, FunctionSignatureInfo, SymbolTable, Symbol, SymbolType};
pub use function::{Function, FunctionSignature, Parameter, Type, FunctionLocation};
pub use language::{Language, LanguageDetector};
pub use matching::{MatchResult, Change, ChangeType, CodeElement, ElementType, ChangeDetail, RefactoringType};
pub use parser::{Parser, ParseResult, ParseError};

/// Re-export commonly used types
pub type Result<T> = std::result::Result<T, ParseError>;

#[cfg(test)]
mod tests;
