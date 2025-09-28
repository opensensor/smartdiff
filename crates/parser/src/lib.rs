//! Smart Code Diff Parser
//! 
//! Multi-language parser engine that converts source code into normalized AST representations
//! for structural and semantic comparison.

pub mod ast;
pub mod language;
pub mod parser;
pub mod tree_sitter;

pub use ast::{ASTNode, NodeType, NodeMetadata};
pub use language::{Language, LanguageDetector};
pub use parser::{Parser, ParseResult, ParseError};

/// Re-export commonly used types
pub type Result<T> = std::result::Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Basic smoke test to ensure the crate compiles
        assert!(true);
    }
}
