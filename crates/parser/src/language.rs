//! Language detection and configuration

use std::path::Path;
use regex::Regex;
use once_cell::sync::Lazy;

/// Supported programming languages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    Java,
    Python,
    JavaScript,
    TypeScript,
    Cpp,
    C,
    Rust,
    Go,
    Unknown,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "java" => Language::Java,
            "py" | "pyw" => Language::Python,
            "js" | "jsx" => Language::JavaScript,
            "ts" | "tsx" => Language::TypeScript,
            "cpp" | "cc" | "cxx" | "c++" | "hpp" => Language::Cpp,
            "c" | "h" => Language::C,
            "rs" => Language::Rust,
            "go" => Language::Go,
            _ => Language::Unknown,
        }
    }
    
    pub fn tree_sitter_name(&self) -> Option<&'static str> {
        match self {
            Language::Java => Some("java"),
            Language::Python => Some("python"),
            Language::JavaScript => Some("javascript"),
            Language::TypeScript => Some("typescript"),
            Language::Cpp => Some("cpp"),
            Language::C => Some("c"),
            Language::Rust => Some("rust"),
            Language::Go => Some("go"),
            Language::Unknown => None,
        }
    }
}

/// Language detector that identifies programming language from file path and content
pub struct LanguageDetector;

impl LanguageDetector {
    pub fn detect_from_path<P: AsRef<Path>>(path: P) -> Language {
        if let Some(ext) = path.as_ref().extension() {
            if let Some(ext_str) = ext.to_str() {
                return Language::from_extension(ext_str);
            }
        }
        Language::Unknown
    }
    
    pub fn detect_from_content(content: &str) -> Language {
        // Simple heuristic-based detection
        static JAVA_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?m)^(public|private|protected)\s+(class|interface)").unwrap()
        });
        
        static PYTHON_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?m)^(def|class|import|from)\s+").unwrap()
        });
        
        static JS_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?m)(function|const|let|var)\s+").unwrap()
        });
        
        if JAVA_PATTERN.is_match(content) {
            Language::Java
        } else if PYTHON_PATTERN.is_match(content) {
            Language::Python
        } else if JS_PATTERN.is_match(content) {
            Language::JavaScript
        } else {
            Language::Unknown
        }
    }
    
    pub fn detect<P: AsRef<Path>>(path: P, content: &str) -> Language {
        let path_lang = Self::detect_from_path(&path);
        if path_lang != Language::Unknown {
            path_lang
        } else {
            Self::detect_from_content(content)
        }
    }
}
