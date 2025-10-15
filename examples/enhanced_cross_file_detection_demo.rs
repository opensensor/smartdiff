//! Enhanced Cross-File Refactoring Detection Demo
//!
//! This example demonstrates the enhanced cross-file refactoring detection capabilities:
//! - File-level refactoring detection (renames, splits, merges, moves)
//! - Symbol migration tracking
//! - Global symbol table integration
//! - Advanced move detection algorithms

use smart_diff_engine::{FileRefactoringDetector, SymbolMigrationTracker};
use smart_diff_parser::tree_sitter::TreeSitterParser;
use smart_diff_parser::{Language, Parser};
use smart_diff_semantic::SymbolResolver;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    println!("=== Enhanced Cross-File Refactoring Detection Demo ===\n");

    // Example 1: File Rename Detection
    println!("Example 1: File Rename Detection");
    println!("----------------------------------");
    demonstrate_file_rename_detection()?;

    // Example 2: File Split Detection
    println!("\nExample 2: File Split Detection");
    println!("--------------------------------");
    demonstrate_file_split_detection()?;

    // Example 3: File Merge Detection
    println!("\nExample 3: File Merge Detection");
    println!("--------------------------------");
    demonstrate_file_merge_detection()?;

    // Example 4: Symbol Migration Tracking
    println!("\nExample 4: Symbol Migration Tracking");
    println!("-------------------------------------");
    demonstrate_symbol_migration_tracking()?;

    // Example 5: Integrated Detection
    println!("\nExample 5: Integrated Cross-File Detection");
    println!("-------------------------------------------");
    demonstrate_integrated_detection()?;

    Ok(())
}

fn demonstrate_file_rename_detection() -> anyhow::Result<()> {
    let mut source_files = HashMap::new();
    let mut target_files = HashMap::new();

    // Source: Calculator.java
    source_files.insert(
        "src/Calculator.java".to_string(),
        r#"
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
    
    public int subtract(int a, int b) {
        return a - b;
    }
    
    public int multiply(int a, int b) {
        return a * b;
    }
}
"#
        .to_string(),
    );

    // Target: MathCalculator.java (renamed with minor changes)
    target_files.insert(
        "src/MathCalculator.java".to_string(),
        r#"
public class MathCalculator {
    public int add(int a, int b) {
        return a + b;
    }
    
    public int subtract(int a, int b) {
        return a - b;
    }
    
    public int multiply(int a, int b) {
        return a * b;
    }
    
    public int divide(int a, int b) {
        return a / b;
    }
}
"#
        .to_string(),
    );

    let detector = FileRefactoringDetector::with_defaults();
    let result = detector.detect_file_refactorings(&source_files, &target_files)?;

    println!("File Renames Detected: {}", result.file_renames.len());
    for rename in &result.file_renames {
        println!("  {} -> {}", rename.source_path, rename.target_path);
        println!("    Content Similarity: {:.2}%", rename.content_similarity * 100.0);
        println!("    Path Similarity: {:.2}%", rename.path_similarity * 100.0);
        println!("    Confidence: {:.2}%", rename.confidence * 100.0);
    }

    println!("\nFile Moves Detected: {}", result.file_moves.len());
    for move_op in &result.file_moves {
        println!("  {} -> {}", move_op.source_path, move_op.target_path);
        println!("    Was Renamed: {}", move_op.was_renamed);
        println!("    Confidence: {:.2}%", move_op.confidence * 100.0);
    }

    Ok(())
}

fn demonstrate_file_split_detection() -> anyhow::Result<()> {
    let mut source_files = HashMap::new();
    let mut target_files = HashMap::new();

    // Source: Large monolithic file
    source_files.insert(
        "src/Utils.java".to_string(),
        r#"
public class StringUtils {
    public String capitalize(String s) {
        return s.substring(0, 1).toUpperCase() + s.substring(1);
    }
}

public class MathUtils {
    public int factorial(int n) {
        return n <= 1 ? 1 : n * factorial(n - 1);
    }
}

public class FileUtils {
    public String readFile(String path) {
        return "content";
    }
}
"#
        .to_string(),
    );

    // Target: Split into separate files
    target_files.insert(
        "src/StringUtils.java".to_string(),
        r#"
public class StringUtils {
    public String capitalize(String s) {
        return s.substring(0, 1).toUpperCase() + s.substring(1);
    }
}
"#
        .to_string(),
    );

    target_files.insert(
        "src/MathUtils.java".to_string(),
        r#"
public class MathUtils {
    public int factorial(int n) {
        return n <= 1 ? 1 : n * factorial(n - 1);
    }
}
"#
        .to_string(),
    );

    target_files.insert(
        "src/FileUtils.java".to_string(),
        r#"
public class FileUtils {
    public String readFile(String path) {
        return "content";
    }
}
"#
        .to_string(),
    );

    let detector = FileRefactoringDetector::with_defaults();
    let result = detector.detect_file_refactorings(&source_files, &target_files)?;

    println!("File Splits Detected: {}", result.file_splits.len());
    for split in &result.file_splits {
        println!("  Source: {}", split.source_path);
        println!("  Split into {} files:", split.target_files.len());
        for (target_file, similarity) in &split.target_files {
            println!("    - {} (similarity: {:.2}%)", target_file, similarity * 100.0);
        }
        println!("    Combined Similarity: {:.2}%", split.combined_similarity * 100.0);
        println!("    Confidence: {:.2}%", split.confidence * 100.0);
    }

    Ok(())
}

fn demonstrate_file_merge_detection() -> anyhow::Result<()> {
    let mut source_files = HashMap::new();
    let mut target_files = HashMap::new();

    // Source: Multiple small files
    source_files.insert(
        "src/Add.java".to_string(),
        r#"
public class Add {
    public int execute(int a, int b) {
        return a + b;
    }
}
"#
        .to_string(),
    );

    source_files.insert(
        "src/Subtract.java".to_string(),
        r#"
public class Subtract {
    public int execute(int a, int b) {
        return a - b;
    }
}
"#
        .to_string(),
    );

    // Target: Merged into one file
    target_files.insert(
        "src/Operations.java".to_string(),
        r#"
public class Add {
    public int execute(int a, int b) {
        return a + b;
    }
}

public class Subtract {
    public int execute(int a, int b) {
        return a - b;
    }
}
"#
        .to_string(),
    );

    let detector = FileRefactoringDetector::with_defaults();
    let result = detector.detect_file_refactorings(&source_files, &target_files)?;

    println!("File Merges Detected: {}", result.file_merges.len());
    for merge in &result.file_merges {
        println!("  Target: {}", merge.target_path);
        println!("  Merged from {} files:", merge.source_files.len());
        for (source_file, similarity) in &merge.source_files {
            println!("    - {} (similarity: {:.2}%)", source_file, similarity * 100.0);
        }
        println!("    Combined Similarity: {:.2}%", merge.combined_similarity * 100.0);
        println!("    Confidence: {:.2}%", merge.confidence * 100.0);
    }

    Ok(())
}

fn demonstrate_symbol_migration_tracking() -> anyhow::Result<()> {
    // Create symbol resolvers for source and target
    let mut source_resolver = SymbolResolver::with_defaults();
    let mut target_resolver = SymbolResolver::with_defaults();

    // Parse source files
    let parser = TreeSitterParser::new()?;

    let source_code = r#"
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
}
"#;

    let target_code = r#"
public class MathOperations {
    public int add(int a, int b) {
        return a + b;
    }
}
"#;

    let source_result = parser.parse(source_code, Language::Java)?;
    let target_result = parser.parse(target_code, Language::Java)?;

    source_resolver.process_file("src/Calculator.java", &source_result)?;
    target_resolver.process_file("src/MathOperations.java", &target_result)?;

    // Track symbol migrations
    let tracker = SymbolMigrationTracker::with_defaults();
    let migration_result = tracker.track_migrations(&source_resolver, &target_resolver)?;

    println!("Symbol Migrations Detected: {}", migration_result.symbol_migrations.len());
    for migration in &migration_result.symbol_migrations {
        println!("  Symbol: {} ({})", migration.symbol_name, migration.symbol_kind);
        println!("    {} -> {}", migration.source_file, migration.target_file);
        if migration.was_renamed {
            println!("    Renamed to: {:?}", migration.new_name);
        }
        println!("    Confidence: {:.2}%", migration.confidence * 100.0);
    }

    println!("\nFile Migrations: {}", migration_result.file_migrations.len());
    for file_migration in &migration_result.file_migrations {
        println!("  {} -> {}", file_migration.source_file, file_migration.target_file);
        println!("    Migrated Symbols: {}", file_migration.migrated_symbols.len());
        println!("    Migration %: {:.2}%", file_migration.migration_percentage * 100.0);
    }

    Ok(())
}

fn demonstrate_integrated_detection() -> anyhow::Result<()> {
    println!("Demonstrating integrated cross-file refactoring detection...");
    println!("This combines:");
    println!("  - File-level refactoring detection");
    println!("  - Symbol migration tracking");
    println!("  - Cross-file reference analysis");
    println!("  - Move detection with confidence scoring");

    // In a real scenario, this would combine all the above techniques
    // to provide comprehensive refactoring detection

    Ok(())
}

