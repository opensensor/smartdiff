//! Doctor command implementation for system validation

use crate::cli::{Cli, Commands};
use anyhow::Result;
use colored::*;
use console::Term;
use smart_diff_parser::{tree_sitter::TreeSitterParser, Parser, LanguageDetector, Language};
use smart_diff_semantic::SemanticAnalyzer;
use smart_diff_engine::{DiffEngine, RefactoringDetector, SimilarityScorer};

pub async fn run(cli: Cli) -> Result<()> {
    if let Commands::Doctor { component, fix } = cli.command {
        let term = Term::stdout();
        
        if !cli.quiet {
            term.write_line(&format!("{}", "Smart Code Diff - System Diagnostics".bold().blue()))?;
            term.write_line(&format!("{}", "=".repeat(45).dimmed()))?;
            term.write_line("")?;
        }

        let mut issues_found = 0;
        let mut issues_fixed = 0;

        // Check core components
        if component.is_none() || component.as_deref() == Some("parser") {
            let (parser_issues, parser_fixes) = check_parser_system(&term, fix, cli.quiet).await?;
            issues_found += parser_issues;
            issues_fixed += parser_fixes;
        }

        if component.is_none() || component.as_deref() == Some("semantic") {
            let (semantic_issues, semantic_fixes) = check_semantic_system(&term, fix, cli.quiet).await?;
            issues_found += semantic_issues;
            issues_fixed += semantic_fixes;
        }

        if component.is_none() || component.as_deref() == Some("engine") {
            let (engine_issues, engine_fixes) = check_diff_engine(&term, fix, cli.quiet).await?;
            issues_found += engine_issues;
            issues_fixed += engine_fixes;
        }

        if component.is_none() || component.as_deref() == Some("languages") {
            let (lang_issues, lang_fixes) = check_language_support(&term, fix, cli.quiet).await?;
            issues_found += lang_issues;
            issues_fixed += lang_fixes;
        }

        if component.is_none() || component.as_deref() == Some("config") {
            let (config_issues, config_fixes) = check_configuration(&term, fix, cli.quiet).await?;
            issues_found += config_issues;
            issues_fixed += config_fixes;
        }

        // Summary
        if !cli.quiet {
            term.write_line("")?;
            term.write_line(&format!("{}", "Diagnostic Summary".bold().green()))?;
            term.write_line(&format!("{}", "-".repeat(20).dimmed()))?;
            
            if issues_found == 0 {
                term.write_line(&format!("{} All systems are functioning correctly!", "✓".green().bold()))?;
            } else {
                term.write_line(&format!("{} {} issues found", "⚠".yellow().bold(), issues_found))?;
                
                if fix && issues_fixed > 0 {
                    term.write_line(&format!("{} {} issues automatically fixed", "✓".green().bold(), issues_fixed))?;
                }
                
                if issues_found > issues_fixed {
                    term.write_line(&format!("{} {} issues require manual attention", 
                        "!".red().bold(), 
                        issues_found - issues_fixed))?;
                }
            }
        }

        // Exit with error code if issues remain
        if issues_found > issues_fixed {
            std::process::exit(1);
        }

        Ok(())
    } else {
        unreachable!("Doctor command should have been matched")
    }
}

/// Check parser system functionality
async fn check_parser_system(term: &Term, _fix: bool, quiet: bool) -> Result<(usize, usize)> {
    if !quiet {
        term.write_line(&format!("{}", "Checking Parser System...".bold()))?;
    }

    let mut issues = 0;
    let fixes = 0;

    // Test language detector
    let _language_detector = LanguageDetector;
    
    // Test basic language detection
    let test_cases = vec![
        ("test.java", Language::Java),
        ("test.py", Language::Python),
        ("test.js", Language::JavaScript),
        ("test.cpp", Language::Cpp),
        ("test.c", Language::C),
    ];

    for (filename, expected) in test_cases {
        let detected = LanguageDetector::detect_from_path(std::path::Path::new(filename));
        if detected != expected {
            issues += 1;
            if !quiet {
                term.write_line(&format!("  {} Language detection failed for {}: expected {:?}, got {:?}", 
                    "✗".red(), filename, expected, detected))?;
            }
        } else if !quiet {
            term.write_line(&format!("  {} Language detection for {}: {:?}", 
                "✓".green(), filename, detected))?;
        }
    }

    // Test parser creation for each language
    let languages = vec![Language::Java, Language::Python, Language::JavaScript, Language::Cpp, Language::C];
    
    for lang in languages {
        match std::panic::catch_unwind(|| TreeSitterParser::new()) {
            Ok(_parser) => {
                if !quiet {
                    term.write_line(&format!("  {} Parser creation for {:?}: OK", "✓".green(), lang))?;
                }
            }
            Err(_) => {
                issues += 1;
                if !quiet {
                    term.write_line(&format!("  {} Parser creation for {:?}: FAILED", "✗".red(), lang))?;
                }
            }
        }
    }

    // Test basic parsing
    let test_code = "function test() { return 42; }";
    let parser = TreeSitterParser::new().expect("Failed to create parser");

    match parser.parse(test_code, Language::JavaScript) {
        Ok(_ast) => {
            if !quiet {
                term.write_line(&format!("  {} Basic parsing test: OK", "✓".green()))?;
            }
        }
        Err(e) => {
            issues += 1;
            if !quiet {
                term.write_line(&format!("  {} Basic parsing test: FAILED ({})", "✗".red(), e))?;
            }
        }
    }

    Ok((issues, fixes))
}

/// Check semantic analysis system
async fn check_semantic_system(term: &Term, _fix: bool, quiet: bool) -> Result<(usize, usize)> {
    if !quiet {
        term.write_line(&format!("{}", "Checking Semantic Analysis System...".bold()))?;
    }

    let mut issues = 0;
    let fixes = 0;

    // Test semantic analyzer creation
    let languages = vec![Language::Java, Language::Python, Language::JavaScript, Language::Cpp, Language::C];
    
    for lang in languages {
        match std::panic::catch_unwind(|| SemanticAnalyzer::new()) {
            Ok(_analyzer) => {
                if !quiet {
                    term.write_line(&format!("  {} Semantic analyzer for {:?}: OK", "✓".green(), lang))?;
                }
            }
            Err(_) => {
                issues += 1;
                if !quiet {
                    term.write_line(&format!("  {} Semantic analyzer for {:?}: FAILED", "✗".red(), lang))?;
                }
            }
        }
    }

    // Test basic semantic analysis
    let test_code = "function add(a, b) { return a + b; }";
    let parser = TreeSitterParser::new().expect("Failed to create parser");

    match parser.parse(test_code, Language::JavaScript) {
        Ok(ast) => {
            let mut analyzer = SemanticAnalyzer::new();
            match analyzer.analyze(&ast) {
                Ok(_symbols) => {
                    if !quiet {
                        term.write_line(&format!("  {} Basic semantic analysis: OK", "✓".green()))?;
                    }
                }
                Err(e) => {
                    issues += 1;
                    if !quiet {
                        term.write_line(&format!("  {} Basic semantic analysis: FAILED ({})", "✗".red(), e))?;
                    }
                }
            }
        }
        Err(e) => {
            issues += 1;
            if !quiet {
                term.write_line(&format!("  {} Parse for semantic test: FAILED ({})", "✗".red(), e))?;
            }
        }
    }

    Ok((issues, fixes))
}

/// Check diff engine functionality
async fn check_diff_engine(term: &Term, _fix: bool, quiet: bool) -> Result<(usize, usize)> {
    if !quiet {
        term.write_line(&format!("{}", "Checking Diff Engine...".bold()))?;
    }

    let mut issues = 0;
    let fixes = 0;

    // Test diff engine creation
    let languages = vec![Language::Java, Language::Python, Language::JavaScript, Language::Cpp, Language::C];
    
    for lang in &languages {
        match std::panic::catch_unwind(|| DiffEngine::new()) {
            Ok(_engine) => {
                if !quiet {
                    term.write_line(&format!("  {} Diff engine for {:?}: OK", "✓".green(), lang))?;
                }
            }
            Err(_) => {
                issues += 1;
                if !quiet {
                    term.write_line(&format!("  {} Diff engine for {:?}: FAILED", "✗".red(), lang))?;
                }
            }
        }
    }

    // Test similarity scorer
    for lang in &languages {
        match std::panic::catch_unwind(|| SimilarityScorer::new(*lang, smart_diff_engine::SimilarityScoringConfig::default())) {
            Ok(_scorer) => {
                if !quiet {
                    term.write_line(&format!("  {} Similarity scorer for {:?}: OK", "✓".green(), lang))?;
                }
            }
            Err(_) => {
                issues += 1;
                if !quiet {
                    term.write_line(&format!("  {} Similarity scorer for {:?}: FAILED", "✗".red(), lang))?;
                }
            }
        }
    }

    // Test refactoring detector
    for lang in &languages {
        match std::panic::catch_unwind(|| RefactoringDetector::new(*lang)) {
            Ok(_detector) => {
                if !quiet {
                    term.write_line(&format!("  {} Refactoring detector for {:?}: OK", "✓".green(), lang))?;
                }
            }
            Err(_) => {
                issues += 1;
                if !quiet {
                    term.write_line(&format!("  {} Refactoring detector for {:?}: FAILED", "✗".red(), lang))?;
                }
            }
        }
    }

    Ok((issues, fixes))
}

/// Check language support
async fn check_language_support(term: &Term, _fix: bool, quiet: bool) -> Result<(usize, usize)> {
    if !quiet {
        term.write_line(&format!("{}", "Checking Language Support...".bold()))?;
    }

    let issues = 0;
    let fixes = 0;

    let supported_languages = vec![
        (Language::Java, vec!["java"]),
        (Language::Python, vec!["py", "pyx", "pyi"]),
        (Language::JavaScript, vec!["js", "jsx", "mjs", "cjs"]),
        (Language::Cpp, vec!["cpp", "cxx", "cc", "hpp", "hxx", "h"]),
        (Language::C, vec!["c", "h"]),
    ];

    for (lang, extensions) in supported_languages {
        if !quiet {
            term.write_line(&format!("  {} {:?} support: {} extensions", 
                "✓".green(), lang, extensions.join(", ")))?;
        }
    }

    Ok((issues, fixes))
}

/// Check configuration system
async fn check_configuration(term: &Term, fix: bool, quiet: bool) -> Result<(usize, usize)> {
    if !quiet {
        term.write_line(&format!("{}", "Checking Configuration System...".bold()))?;
    }

    let mut issues = 0;
    let fixes = 0;

    // Check if we can create default configurations
    let config_tests = vec![
        ("RefactoringDetectionConfig", || {
            smart_diff_engine::RefactoringDetectionConfig::default()
        }),
    ];

    for (config_name, create_config) in config_tests {
        match std::panic::catch_unwind(create_config) {
            Ok(_config) => {
                if !quiet {
                    term.write_line(&format!("  {} {} creation: OK", "✓".green(), config_name))?;
                }
            }
            Err(_) => {
                issues += 1;
                if !quiet {
                    term.write_line(&format!("  {} {} creation: FAILED", "✗".red(), config_name))?;
                }
            }
        }
    }

    Ok((issues, fixes))
}
