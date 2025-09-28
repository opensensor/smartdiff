//! Core parser interface and error types

use crate::ast::ASTNode;
use crate::language::Language;
use thiserror::Error;

/// Parser error types
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unsupported language: {0:?}")]
    UnsupportedLanguage(Language),
    
    #[error("Parse error: {0}")]
    ParseFailed(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Tree-sitter error: {0}")]
    TreeSitterError(String),
}

/// Result of parsing operation
#[derive(Debug)]
pub struct ParseResult {
    pub ast: ASTNode,
    pub language: Language,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Main parser interface
pub trait Parser {
    fn parse(&self, content: &str, language: Language) -> Result<ParseResult, ParseError>;
    fn parse_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<ParseResult, ParseError>;
    fn supported_languages(&self) -> Vec<Language>;
}

/// Default parser implementation
pub struct DefaultParser;

impl Parser for DefaultParser {
    fn parse(&self, _content: &str, language: Language) -> Result<ParseResult, ParseError> {
        // Placeholder implementation
        Err(ParseError::UnsupportedLanguage(language))
    }
    
    fn parse_file<P: AsRef<std::path::Path>>(&self, _path: P) -> Result<ParseResult, ParseError> {
        // Placeholder implementation
        Err(ParseError::UnsupportedLanguage(Language::Unknown))
    }
    
    fn supported_languages(&self) -> Vec<Language> {
        vec![
            Language::Java,
            Language::Python,
            Language::JavaScript,
            Language::Cpp,
            Language::C,
        ]
    }
}
