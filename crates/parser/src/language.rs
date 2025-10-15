//! Language detection and configuration

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

/// Supported programming languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Language {
    Java,
    Python,
    JavaScript,
    TypeScript,
    Cpp,
    C,
    Rust,
    Go,
    Ruby,
    PHP,
    Swift,
    Unknown,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Java => write!(f, "Java"),
            Language::Python => write!(f, "Python"),
            Language::JavaScript => write!(f, "JavaScript"),
            Language::TypeScript => write!(f, "TypeScript"),
            Language::Cpp => write!(f, "C++"),
            Language::C => write!(f, "C"),
            Language::Rust => write!(f, "Rust"),
            Language::Go => write!(f, "Go"),
            Language::Ruby => write!(f, "Ruby"),
            Language::PHP => write!(f, "PHP"),
            Language::Swift => write!(f, "Swift"),
            Language::Unknown => write!(f, "Unknown"),
        }
    }
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
            "rb" | "rake" | "gemspec" => Language::Ruby,
            "php" | "phtml" | "php3" | "php4" | "php5" | "phps" => Language::PHP,
            "swift" => Language::Swift,
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
            Language::Ruby => Some("ruby"),
            Language::PHP => Some("php"),
            Language::Swift => Some("swift"),
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
        // Use sophisticated pattern matching with scoring
        let mut scores = HashMap::new();

        // Initialize scores for all languages
        for &lang in &[
            Language::Java,
            Language::Python,
            Language::JavaScript,
            Language::Cpp,
            Language::C,
            Language::Go,
            Language::Ruby,
            Language::PHP,
            Language::Swift,
        ] {
            scores.insert(lang, 0.0);
        }

        // Apply detection patterns
        Self::apply_java_patterns(content, &mut scores);
        Self::apply_python_patterns(content, &mut scores);
        Self::apply_javascript_patterns(content, &mut scores);
        Self::apply_cpp_patterns(content, &mut scores);
        Self::apply_c_patterns(content, &mut scores);
        Self::apply_go_patterns(content, &mut scores);
        Self::apply_ruby_patterns(content, &mut scores);
        Self::apply_php_patterns(content, &mut scores);
        Self::apply_swift_patterns(content, &mut scores);

        // Find the language with the highest score
        let mut best_language = Language::Unknown;
        let mut best_score = 0.0;

        for (&language, &score) in &scores {
            if score > best_score && score > 0.3 {
                // Minimum confidence threshold
                best_score = score;
                best_language = language;
            }
        }

        best_language
    }

    pub fn detect<P: AsRef<Path>>(path: P, content: &str) -> Language {
        let path_lang = Self::detect_from_path(&path);
        if path_lang != Language::Unknown {
            path_lang
        } else {
            Self::detect_from_content(content)
        }
    }

    /// Apply Java-specific detection patterns
    fn apply_java_patterns(content: &str, scores: &mut HashMap<Language, f64>) {
        static JAVA_PATTERNS: Lazy<Vec<(Regex, f64)>> = Lazy::new(|| {
            vec![
                // Strong indicators
                (
                    Regex::new(r"(?m)^(public|private|protected)\s+(class|interface|enum)")
                        .unwrap(),
                    0.8,
                ),
                (
                    Regex::new(
                        r"(?m)^(public|private|protected)\s+(static\s+)?(void|int|String|boolean)",
                    )
                    .unwrap(),
                    0.7,
                ),
                (Regex::new(r"\bSystem\.out\.print(ln)?\s*\(").unwrap(), 0.9),
                (
                    Regex::new(r"\bpublic\s+static\s+void\s+main\s*\(\s*String\[\]\s+\w+\s*\)")
                        .unwrap(),
                    1.0,
                ),
                // Medium indicators
                (Regex::new(r"\b(import\s+java\.|package\s+)").unwrap(), 0.6),
                (
                    Regex::new(r"\b(ArrayList|HashMap|List|Map|Set)\s*<").unwrap(),
                    0.5,
                ),
                (
                    Regex::new(r"\b@(Override|Deprecated|SuppressWarnings)").unwrap(),
                    0.6,
                ),
                (Regex::new(r"\bnew\s+\w+\s*\(").unwrap(), 0.3),
                // Weak indicators
                (
                    Regex::new(r"\b(final|static|abstract|synchronized)\s+").unwrap(),
                    0.2,
                ),
                (Regex::new(r"\.length\b").unwrap(), 0.1),
            ]
        });

        let mut score = 0.0;
        for (pattern, weight) in JAVA_PATTERNS.iter() {
            if pattern.is_match(content) {
                score += weight;
            }
        }

        // Penalty for non-Java patterns
        if content.contains("def ")
            || content.contains("import ") && !content.contains("import java")
        {
            score -= 0.3;
        }
        if content.contains("function ") || content.contains("const ") || content.contains("let ") {
            score -= 0.3;
        }

        scores.insert(Language::Java, score.max(0.0));
    }

    /// Apply Python-specific detection patterns
    fn apply_python_patterns(content: &str, scores: &mut HashMap<Language, f64>) {
        static PYTHON_PATTERNS: Lazy<Vec<(Regex, f64)>> = Lazy::new(|| {
            vec![
                // Strong indicators
                (Regex::new(r"(?m)^def\s+\w+\s*\(.*\)\s*:").unwrap(), 0.9),
                (
                    Regex::new(r"(?m)^class\s+\w+(\([^)]*\))?\s*:").unwrap(),
                    0.8,
                ),
                (
                    Regex::new(r#"\bif\s+__name__\s*==\s*['"]__main__['"]"#).unwrap(),
                    1.0,
                ),
                (Regex::new(r"\bprint\s*\(").unwrap(), 0.7),
                // Medium indicators
                (
                    Regex::new(r"(?m)^(import\s+\w+|from\s+\w+\s+import)").unwrap(),
                    0.6,
                ),
                (Regex::new(r"\bself\.\w+").unwrap(), 0.6),
                (Regex::new(r"\b(True|False|None)\b").unwrap(), 0.5),
                (Regex::new(r"(?m)^\s*#.*$").unwrap(), 0.2),
                // Python-specific syntax
                (Regex::new(r"\blen\s*\(").unwrap(), 0.3),
                (Regex::new(r"\brange\s*\(").unwrap(), 0.4),
                (Regex::new(r"\b(list|dict|tuple|set)\s*\(").unwrap(), 0.3),
                (Regex::new(r"(?m)^\s*elif\s+").unwrap(), 0.5),
                (Regex::new(r"\bwith\s+\w+.*:").unwrap(), 0.4),
                (Regex::new(r"\btry\s*:").unwrap(), 0.3),
                (Regex::new(r"\bexcept\s+\w*:").unwrap(), 0.4),
            ]
        });

        let mut score = 0.0;
        for (pattern, weight) in PYTHON_PATTERNS.iter() {
            if pattern.is_match(content) {
                score += weight;
            }
        }

        // Check for Python-specific indentation patterns
        let lines: Vec<&str> = content.lines().collect();
        let mut indent_score = 0.0;
        for line in &lines {
            if line.starts_with("    ") || line.starts_with("\t") {
                indent_score += 0.1;
            }
        }
        score += (indent_score / lines.len() as f64).min(0.3);

        // Penalty for non-Python patterns
        if content.contains("public class") || content.contains("private ") {
            score -= 0.4;
        }
        if content.contains("function ") || content.contains("var ") {
            score -= 0.3;
        }

        scores.insert(Language::Python, score.max(0.0));
    }

    /// Apply JavaScript-specific detection patterns
    fn apply_javascript_patterns(content: &str, scores: &mut HashMap<Language, f64>) {
        static JS_PATTERNS: Lazy<Vec<(Regex, f64)>> = Lazy::new(|| {
            vec![
                // Strong indicators
                (Regex::new(r"\bfunction\s+\w+\s*\(").unwrap(), 0.8),
                (Regex::new(r"\b(const|let|var)\s+\w+\s*=").unwrap(), 0.7),
                (
                    Regex::new(r"\bconsole\.(log|error|warn)\s*\(").unwrap(),
                    0.9,
                ),
                (Regex::new(r"=>\s*\{").unwrap(), 0.8), // Arrow functions
                (Regex::new(r"\bclass\s+\w+\s*\{").unwrap(), 0.6),
                // Medium indicators
                (Regex::new(r"\b(require|import)\s*\(").unwrap(), 0.6),
                (Regex::new(r"\bmodule\.exports\s*=").unwrap(), 0.8),
                (
                    Regex::new(r"\bexport\s+(default\s+)?(function|class|const)").unwrap(),
                    0.7,
                ),
                (Regex::new(r"\b(true|false|null|undefined)\b").unwrap(), 0.4),
                (Regex::new(r"\bnew\s+\w+\s*\(").unwrap(), 0.3),
                // JavaScript-specific syntax
                (Regex::new(r"\bthis\.\w+").unwrap(), 0.3),
                (Regex::new(r"\b(async|await)\b").unwrap(), 0.6),
                (
                    Regex::new(r"\b(Promise|setTimeout|setInterval)\b").unwrap(),
                    0.5,
                ),
                (Regex::new(r"\.then\s*\(").unwrap(), 0.4),
                (Regex::new(r"\$\{.*\}").unwrap(), 0.5), // Template literals
                (Regex::new(r"//.*$").unwrap(), 0.1),    // Single-line comments
            ]
        });

        let mut score = 0.0;
        for (pattern, weight) in JS_PATTERNS.iter() {
            if pattern.is_match(content) {
                score += weight;
            }
        }

        // Penalty for non-JavaScript patterns
        if content.contains("public class") || content.contains("def ") {
            score -= 0.4;
        }
        if content.contains("#include") || content.contains("std::") {
            score -= 0.5;
        }

        scores.insert(Language::JavaScript, score.max(0.0));
    }

    /// Apply C++-specific detection patterns
    fn apply_cpp_patterns(content: &str, scores: &mut HashMap<Language, f64>) {
        static CPP_PATTERNS: Lazy<Vec<(Regex, f64)>> = Lazy::new(|| {
            vec![
                // Strong indicators
                (
                    Regex::new(r"#include\s*<(iostream|vector|string|map|algorithm)>").unwrap(),
                    0.9,
                ),
                (
                    Regex::new(r"\bstd::(cout|cin|endl|vector|string|map)").unwrap(),
                    0.8,
                ),
                (Regex::new(r"\bclass\s+\w+\s*\{").unwrap(), 0.6),
                (
                    Regex::new(r"\b(public|private|protected)\s*:").unwrap(),
                    0.7,
                ),
                // Medium indicators
                (Regex::new(r"\b(template\s*<|typename\s+)").unwrap(), 0.8),
                (Regex::new(r"\bnamespace\s+\w+").unwrap(), 0.7),
                (Regex::new(r"\b(virtual|override|final)\s+").unwrap(), 0.6),
                (Regex::new(r"\bnew\s+\w+(\[\]|\(\))").unwrap(), 0.4),
                (Regex::new(r"\bdelete\s+").unwrap(), 0.5),
                // C++ specific syntax
                (Regex::new(r"::").unwrap(), 0.4),
                (Regex::new(r"\b(auto|decltype)\s+").unwrap(), 0.5),
                (Regex::new(r"\b(nullptr|constexpr)\b").unwrap(), 0.6),
                (Regex::new(r"->").unwrap(), 0.2),
                (Regex::new(r"<<|>>").unwrap(), 0.2),
            ]
        });

        let mut score = 0.0;
        for (pattern, weight) in CPP_PATTERNS.iter() {
            if pattern.is_match(content) {
                score += weight;
            }
        }

        // Bonus for C++ over C patterns
        if content.contains("std::") || content.contains("class ") {
            score += 0.3;
        }

        // Penalty for non-C++ patterns
        if content.contains("def ") || content.contains("function ") {
            score -= 0.4;
        }
        if content.contains("public class") && content.contains("System.out") {
            score -= 0.5;
        }

        scores.insert(Language::Cpp, score.max(0.0));
    }

    /// Apply C-specific detection patterns
    fn apply_c_patterns(content: &str, scores: &mut HashMap<Language, f64>) {
        static C_PATTERNS: Lazy<Vec<(Regex, f64)>> = Lazy::new(|| {
            vec![
                // Strong indicators
                (
                    Regex::new(r"#include\s*<(stdio\.h|stdlib\.h|string\.h|math\.h)>").unwrap(),
                    0.9,
                ),
                (Regex::new(r"\bprintf\s*\(").unwrap(), 0.8),
                (Regex::new(r"\bscanf\s*\(").unwrap(), 0.7),
                (
                    Regex::new(r"\bmain\s*\(\s*(void|int\s+argc.*char.*argv)\s*\)").unwrap(),
                    0.8,
                ),
                // Medium indicators
                (Regex::new(r"\b(struct|union|enum)\s+\w+").unwrap(), 0.6),
                (
                    Regex::new(r"\b(malloc|calloc|realloc|free)\s*\(").unwrap(),
                    0.7,
                ),
                (
                    Regex::new(r"\b(int|char|float|double|void)\s+\*?\w+").unwrap(),
                    0.4,
                ),
                (Regex::new(r"\btypedef\s+").unwrap(), 0.5),
                // C-specific syntax
                (Regex::new(r"->").unwrap(), 0.3),
                (Regex::new(r"\*\w+").unwrap(), 0.2), // Pointer dereference
                (Regex::new(r"&\w+").unwrap(), 0.2),  // Address-of operator
                (
                    Regex::new(r"\b(NULL|EXIT_SUCCESS|EXIT_FAILURE)\b").unwrap(),
                    0.4,
                ),
            ]
        });

        let mut score = 0.0;
        for (pattern, weight) in C_PATTERNS.iter() {
            if pattern.is_match(content) {
                score += weight;
            }
        }

        // Penalty for C++ specific features
        if content.contains("std::") || content.contains("class ") || content.contains("namespace ")
        {
            score -= 0.5;
        }

        // Penalty for other languages
        if content.contains("def ") || content.contains("function ") {
            score -= 0.4;
        }
        if content.contains("public class") {
            score -= 0.5;
        }

        scores.insert(Language::C, score.max(0.0));
    }

    /// Apply Go-specific detection patterns
    fn apply_go_patterns(content: &str, scores: &mut HashMap<Language, f64>) {
        let mut score: f64 = 0.0;

        // Strong indicators
        if content.contains("package main") {
            score += 1.0;
        }
        if content.contains("func main()") {
            score += 1.0;
        }
        if content.contains("func ") {
            score += 0.8;
        }
        if content.contains("import (") {
            score += 0.7;
        }

        // Medium indicators
        if content.contains("go func") {
            score += 0.9;
        }
        if content.contains("chan ") {
            score += 0.8;
        }
        if content.contains("defer ") {
            score += 0.7;
        }
        if content.contains("struct {") {
            score += 0.7;
        }
        if content.contains("interface {") {
            score += 0.7;
        }
        if content.contains(":=") {
            score += 0.6;
        }
        if content.contains("fmt.Print") {
            score += 0.7;
        }
        if content.contains("err != nil") {
            score += 0.6;
        }

        // Weak indicators
        if content.contains("var ") {
            score += 0.3;
        }
        if content.contains("const ") {
            score += 0.3;
        }
        if content.contains("range ") {
            score += 0.5;
        }
        if content.contains("make(") {
            score += 0.5;
        }

        // Penalty for other languages
        if content.contains("public class") || content.contains("def ") {
            score -= 0.5;
        }

        scores.insert(Language::Go, score.max(0.0));
    }

    /// Apply Ruby-specific detection patterns
    fn apply_ruby_patterns(content: &str, scores: &mut HashMap<Language, f64>) {
        let mut score: f64 = 0.0;

        // Strong indicators
        if content.contains("def ") {
            score += 0.8;
        }
        if content.contains("class ") && content.contains(" < ") {
            score += 0.9;
        }
        if content.contains("module ") {
            score += 0.8;
        }
        if content.contains("require ") {
            score += 0.7;
        }
        if content.contains("require_relative ") {
            score += 0.8;
        }

        // Medium indicators
        if content.contains("end") {
            score += 0.6;
        }
        if content.contains("attr_accessor") {
            score += 0.9;
        }
        if content.contains("attr_reader") {
            score += 0.9;
        }
        if content.contains("attr_writer") {
            score += 0.9;
        }
        if content.contains("puts ") {
            score += 0.7;
        }
        if content.contains(".each do") {
            score += 0.7;
        }
        if content.contains(".map do") {
            score += 0.7;
        }
        if content.contains("@") {
            score += 0.4;
        }

        // Weak indicators
        if content.contains("unless ") {
            score += 0.5;
        }
        if content.contains("elsif ") {
            score += 0.6;
        }
        if content.contains("=>") {
            score += 0.4;
        }

        // Penalty for other languages
        if content.contains("public class") || content.contains("function ") {
            score -= 0.5;
        }

        scores.insert(Language::Ruby, score.max(0.0));
    }

    /// Apply PHP-specific detection patterns
    fn apply_php_patterns(content: &str, scores: &mut HashMap<Language, f64>) {
        let mut score: f64 = 0.0;

        // Strong indicators
        if content.contains("<?php") {
            score += 1.0;
        }
        if content.contains("$") && content.contains("=") {
            score += 0.6;
        }
        if content.contains("function ") {
            score += 0.5;
        }
        if content.contains("class ") {
            score += 0.6;
        }
        if content.contains("namespace ") {
            score += 0.8;
        }
        if content.contains("use ") {
            score += 0.5;
        }

        // Medium indicators
        if content.contains("public function") {
            score += 0.8;
        }
        if content.contains("private function") {
            score += 0.8;
        }
        if content.contains("protected function") {
            score += 0.8;
        }
        if content.contains("echo ") {
            score += 0.7;
        }
        if content.contains("print ") {
            score += 0.5;
        }
        if content.contains("$this->") {
            score += 0.8;
        }
        if content.contains("self::") {
            score += 0.7;
        }
        if content.contains("parent::") {
            score += 0.7;
        }
        if content.contains("->") {
            score += 0.4;
        }

        // Weak indicators
        if content.contains("require ") {
            score += 0.4;
        }
        if content.contains("require_once ") {
            score += 0.5;
        }
        if content.contains("include ") {
            score += 0.4;
        }
        if content.contains("include_once ") {
            score += 0.5;
        }
        if content.contains("$_GET") {
            score += 0.6;
        }
        if content.contains("$_POST") {
            score += 0.6;
        }
        if content.contains("$_SESSION") {
            score += 0.6;
        }

        // Penalty for other languages
        if content.contains("def ") || content.contains("func main") {
            score -= 0.4;
        }

        scores.insert(Language::PHP, score.max(0.0));
    }

    /// Apply Swift-specific detection patterns
    fn apply_swift_patterns(content: &str, scores: &mut HashMap<Language, f64>) {
        let mut score: f64 = 0.0;

        // Strong indicators
        if content.contains("func ") {
            score += 0.6;
        }
        if content.contains("var ") && content.contains(":") {
            score += 0.6;
        }
        if content.contains("let ") && content.contains(":") {
            score += 0.6;
        }
        if content.contains("let ") && content.contains("=") {
            score += 0.5;
        }
        if content.contains("import Foundation") {
            score += 0.9;
        }
        if content.contains("import UIKit") {
            score += 0.9;
        }
        if content.contains("import SwiftUI") {
            score += 0.9;
        }

        // Medium indicators
        if content.contains("class ") && content.contains(":") {
            score += 0.7;
        }
        if content.contains("struct ") {
            score += 0.6;
        }
        if content.contains("enum ") {
            score += 0.6;
        }
        if content.contains("protocol ") {
            score += 0.8;
        }
        if content.contains("extension ") {
            score += 0.8;
        }
        if content.contains("guard ") {
            score += 0.8;
        }
        if content.contains("if let ") {
            score += 0.7;
        }
        if content.contains("guard let ") {
            score += 0.8;
        }
        if content.contains("->") {
            score += 0.4;
        }
        if content.contains("?.") {
            score += 0.6;
        }

        // Weak indicators
        if content.contains("override func") {
            score += 0.7;
        }
        if content.contains("private ") {
            score += 0.2;
        }
        if content.contains("public ") {
            score += 0.2;
        }
        if content.contains("internal ") {
            score += 0.3;
        }
        if content.contains("fileprivate ") {
            score += 0.6;
        }
        if content.contains("print(") {
            score += 0.3;
        }

        // Penalty for other languages
        if content.contains("public class") && !content.contains("import Foundation") {
            score -= 0.4;
        }
        if content.contains("def ") {
            score -= 0.5;
        }

        scores.insert(Language::Swift, score.max(0.0));
    }
}
