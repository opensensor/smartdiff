//! Comprehensive demonstration of Cross-File Function Tracking
//! 
//! This example showcases the advanced cross-file function tracking system that detects
//! function moves, renames, splits, and merges across different files using global
//! symbol table integration and dependency analysis.

use smart_diff_engine::{
    CrossFileTracker, CrossFileTrackerConfig, MoveType
};
use smart_diff_parser::{ASTNode, NodeType, NodeMetadata, Language};
use smart_diff_semantic::{
    EnhancedFunctionSignature, FunctionType, Visibility, TypeSignature,
    ParameterInfo, ComplexityMetrics
};
use std::collections::HashMap;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🔄 Cross-File Function Tracking Demo");
    println!("====================================\n");

    // Demo 1: Simple function moves between files
    demo_simple_function_moves()?;
    
    // Demo 2: Function renames with moves
    demo_function_renames_with_moves()?;
    
    // Demo 3: Cross-file function splits
    demo_cross_file_splits()?;
    
    // Demo 4: Cross-file function merges
    demo_cross_file_merges()?;
    
    // Demo 5: Complex refactoring scenarios
    demo_complex_refactoring_scenarios()?;
    
    // Demo 6: Configuration and performance tuning
    demo_configuration_and_performance()?;

    println!("\n✅ Cross-File Function Tracking Demo Complete!");
    Ok(())
}

/// Demo 1: Simple function moves between files
fn demo_simple_function_moves() -> Result<()> {
    println!("📁 Demo 1: Simple Function Moves Between Files");
    println!("----------------------------------------------");

    let mut tracker = CrossFileTracker::with_defaults(Language::Java);

    // Source files (version 1)
    let mut source_files = HashMap::new();
    source_files.insert("Calculator.java".to_string(), vec![
        create_function_with_complexity("add", "Calculator.java", 3),
        create_function_with_complexity("subtract", "Calculator.java", 3),
        create_function_with_complexity("multiply", "Calculator.java", 4),
    ]);
    source_files.insert("MathUtils.java".to_string(), vec![
        create_function_with_complexity("abs", "MathUtils.java", 2),
        create_function_with_complexity("max", "MathUtils.java", 3),
    ]);

    // Target files (version 2) - some functions moved
    let mut target_files = HashMap::new();
    target_files.insert("Calculator.java".to_string(), vec![
        create_function_with_complexity("add", "Calculator.java", 3),
        create_function_with_complexity("subtract", "Calculator.java", 3),
    ]);
    target_files.insert("MathUtils.java".to_string(), vec![
        create_function_with_complexity("abs", "MathUtils.java", 2),
        create_function_with_complexity("max", "MathUtils.java", 3),
        create_function_with_complexity("multiply", "MathUtils.java", 4), // Moved from Calculator
    ]);
    target_files.insert("AdvancedMath.java".to_string(), vec![
        create_function_with_complexity("power", "AdvancedMath.java", 6), // New function
    ]);

    let result = tracker.track_cross_file_changes(&source_files, &target_files)?;

    println!("📈 Cross-File Move Results:");
    println!("  • Total moves detected: {}", result.moved_functions.len());
    println!("  • Total files analyzed: {}", result.overall_statistics.total_files);
    println!("  • Move percentage: {:.1}%", result.overall_statistics.move_percentage);
    println!("  • Average confidence: {:.2}%", result.overall_statistics.average_confidence * 100.0);

    for move_item in &result.moved_functions {
        println!("  🔄 MOVE: {} → {}", 
            move_item.source_file, 
            move_item.target_file
        );
        println!("     Function: {}", move_item.function_signature.name);
        println!("     Type: {:?}", move_item.move_type);
        println!("     Similarity: {:.2}%, Confidence: {:.2}%",
            move_item.similarity.overall_similarity * 100.0,
            move_item.confidence * 100.0
        );
    }

    println!();
    Ok(())
}

/// Demo 2: Function renames with moves
fn demo_function_renames_with_moves() -> Result<()> {
    println!("🏷️  Demo 2: Function Renames with Moves");
    println!("--------------------------------------");

    let mut config = CrossFileTrackerConfig::default();
    config.track_renames = true;
    config.min_cross_file_similarity = 0.75; // Lower threshold for rename detection
    
    let mut tracker = CrossFileTracker::new(Language::Java, config);

    // Source files
    let mut source_files = HashMap::new();
    source_files.insert("UserService.java".to_string(), vec![
        create_function_with_complexity("validateUser", "UserService.java", 8),
        create_function_with_complexity("authenticateUser", "UserService.java", 10),
    ]);
    source_files.insert("DataProcessor.java".to_string(), vec![
        create_function_with_complexity("processUserData", "DataProcessor.java", 12),
    ]);

    // Target files - functions renamed and moved
    let mut target_files = HashMap::new();
    target_files.insert("AuthenticationService.java".to_string(), vec![
        create_function_with_complexity("validateUserCredentials", "AuthenticationService.java", 8), // Renamed + moved
        create_function_with_complexity("performUserAuthentication", "AuthenticationService.java", 10), // Renamed + moved
    ]);
    target_files.insert("UserDataProcessor.java".to_string(), vec![
        create_function_with_complexity("handleUserDataProcessing", "UserDataProcessor.java", 12), // Renamed + moved
    ]);

    let result = tracker.track_cross_file_changes(&source_files, &target_files)?;

    println!("📈 Rename+Move Results:");
    println!("  • Rename+moves detected: {}", result.renamed_and_moved.len());
    println!("  • Execution time: {}ms", result.overall_statistics.execution_time_ms);

    for rename_move in &result.renamed_and_moved {
        println!("  🏷️  RENAME+MOVE: {} → {}", 
            rename_move.source_file, 
            rename_move.target_file
        );
        println!("     {} → {}", 
            rename_move.original_name, 
            rename_move.new_name
        );
        println!("     Similarity: {:.2}%, Confidence: {:.2}%",
            rename_move.similarity.overall_similarity * 100.0,
            rename_move.confidence * 100.0
        );
    }

    println!();
    Ok(())
}

/// Demo 3: Cross-file function splits
fn demo_cross_file_splits() -> Result<()> {
    println!("🔀 Demo 3: Cross-File Function Splits");
    println!("------------------------------------");

    let mut config = CrossFileTrackerConfig::default();
    config.track_splits_merges = true;
    
    let mut tracker = CrossFileTracker::new(Language::Java, config);

    // Source files - one large function
    let mut source_files = HashMap::new();
    source_files.insert("OrderProcessor.java".to_string(), vec![
        create_function_with_complexity("processCompleteOrder", "OrderProcessor.java", 25),
    ]);

    // Target files - function split across multiple files
    let mut target_files = HashMap::new();
    target_files.insert("OrderValidator.java".to_string(), vec![
        create_function_with_complexity("validateOrderData", "OrderValidator.java", 8),
    ]);
    target_files.insert("PaymentProcessor.java".to_string(), vec![
        create_function_with_complexity("processOrderPayment", "PaymentProcessor.java", 10),
    ]);
    target_files.insert("InventoryManager.java".to_string(), vec![
        create_function_with_complexity("updateOrderInventory", "InventoryManager.java", 7),
    ]);

    let result = tracker.track_cross_file_changes(&source_files, &target_files)?;

    println!("📈 Cross-File Split Results:");
    println!("  • Splits detected: {}", result.cross_file_splits.len());

    for split in &result.cross_file_splits {
        println!("  🔀 SPLIT: {} ({})", 
            split.source_function, 
            split.source_file
        );
        println!("     Split into:");
        for (func_name, target_file) in &split.split_functions {
            println!("       • {} in {}", func_name, target_file);
        }
        println!("     Combined similarity: {:.2}%, Confidence: {:.2}%",
            split.combined_similarity * 100.0,
            split.confidence * 100.0
        );
    }

    println!();
    Ok(())
}

/// Demo 4: Cross-file function merges
fn demo_cross_file_merges() -> Result<()> {
    println!("🔗 Demo 4: Cross-File Function Merges");
    println!("------------------------------------");

    let mut config = CrossFileTrackerConfig::default();
    config.track_splits_merges = true;
    
    let mut tracker = CrossFileTracker::new(Language::Java, config);

    // Source files - multiple small functions across files
    let mut source_files = HashMap::new();
    source_files.insert("EmailValidator.java".to_string(), vec![
        create_function_with_complexity("validateEmailFormat", "EmailValidator.java", 5),
    ]);
    source_files.insert("PhoneValidator.java".to_string(), vec![
        create_function_with_complexity("validatePhoneNumber", "PhoneValidator.java", 6),
    ]);
    source_files.insert("AddressValidator.java".to_string(), vec![
        create_function_with_complexity("validateAddress", "AddressValidator.java", 7),
    ]);

    // Target files - functions merged into one comprehensive validator
    let mut target_files = HashMap::new();
    target_files.insert("ComprehensiveValidator.java".to_string(), vec![
        create_function_with_complexity("validateAllContactInfo", "ComprehensiveValidator.java", 18),
    ]);

    let result = tracker.track_cross_file_changes(&source_files, &target_files)?;

    println!("📈 Cross-File Merge Results:");
    println!("  • Merges detected: {}", result.cross_file_merges.len());

    for merge in &result.cross_file_merges {
        println!("  🔗 MERGE: {} ({})", 
            merge.merged_function, 
            merge.target_file
        );
        println!("     Merged from:");
        for (func_name, source_file) in &merge.source_functions {
            println!("       • {} from {}", func_name, source_file);
        }
        println!("     Combined similarity: {:.2}%, Confidence: {:.2}%",
            merge.combined_similarity * 100.0,
            merge.confidence * 100.0
        );
    }

    println!();
    Ok(())
}

/// Demo 5: Complex refactoring scenarios
fn demo_complex_refactoring_scenarios() -> Result<()> {
    println!("🌐 Demo 5: Complex Refactoring Scenarios");
    println!("---------------------------------------");

    let mut config = CrossFileTrackerConfig::default();
    config.track_renames = true;
    config.track_splits_merges = true;
    config.min_cross_file_similarity = 0.7;
    
    let mut tracker = CrossFileTracker::new(Language::Java, config);

    // Source files - legacy structure
    let mut source_files = HashMap::new();
    source_files.insert("LegacyUserManager.java".to_string(), vec![
        create_function_with_complexity("createUser", "LegacyUserManager.java", 8),
        create_function_with_complexity("updateUser", "LegacyUserManager.java", 10),
        create_function_with_complexity("deleteUser", "LegacyUserManager.java", 6),
        create_function_with_complexity("validateUserData", "LegacyUserManager.java", 12),
    ]);

    // Target files - modern architecture with separation of concerns
    let mut target_files = HashMap::new();
    target_files.insert("UserRepository.java".to_string(), vec![
        create_function_with_complexity("saveUser", "UserRepository.java", 5), // Renamed from createUser
        create_function_with_complexity("updateUserRecord", "UserRepository.java", 7), // Renamed from updateUser
        create_function_with_complexity("removeUser", "UserRepository.java", 4), // Renamed from deleteUser
    ]);
    target_files.insert("UserValidationService.java".to_string(), vec![
        create_function_with_complexity("performUserValidation", "UserValidationService.java", 12), // Moved + renamed
    ]);
    target_files.insert("UserAuditLogger.java".to_string(), vec![
        create_function_with_complexity("logUserOperation", "UserAuditLogger.java", 4), // New functionality
    ]);

    let result = tracker.track_cross_file_changes(&source_files, &target_files)?;

    println!("📈 Complex Refactoring Results:");
    println!("  • Simple moves: {}", result.moved_functions.len());
    println!("  • Rename+moves: {}", result.renamed_and_moved.len());
    println!("  • Splits: {}", result.cross_file_splits.len());
    println!("  • Merges: {}", result.cross_file_merges.len());
    println!("  • Overall move percentage: {:.1}%", result.overall_statistics.move_percentage);

    // Show file-level statistics
    println!("\n📊 File-Level Statistics:");
    for (file_path, stats) in &result.file_statistics {
        println!("  📄 {}:", file_path);
        println!("     Functions moved out: {}", stats.functions_moved_out);
        println!("     Functions moved in: {}", stats.functions_moved_in);
        println!("     Net change: {}", stats.net_function_change);
    }

    println!();
    Ok(())
}

/// Demo 6: Configuration and performance tuning
fn demo_configuration_and_performance() -> Result<()> {
    println!("⚙️  Demo 6: Configuration and Performance Tuning");
    println!("-----------------------------------------------");

    // Test different configurations
    let configs = vec![
        ("High Precision", CrossFileTrackerConfig {
            min_cross_file_similarity: 0.9,
            cross_file_move_penalty: 0.05,
            track_renames: true,
            track_splits_merges: true,
            max_files_to_consider: 100,
            use_global_symbol_table: true,
            use_dependency_analysis: true,
        }),
        ("Balanced", CrossFileTrackerConfig::default()),
        ("Fast Tracking", CrossFileTrackerConfig {
            min_cross_file_similarity: 0.7,
            cross_file_move_penalty: 0.2,
            track_renames: false,
            track_splits_merges: false,
            max_files_to_consider: 20,
            use_global_symbol_table: false,
            use_dependency_analysis: false,
        }),
    ];

    // Create test scenario
    let mut source_files = HashMap::new();
    source_files.insert("ServiceA.java".to_string(), vec![
        create_function_with_complexity("methodA", "ServiceA.java", 5),
        create_function_with_complexity("methodB", "ServiceA.java", 7),
    ]);
    source_files.insert("ServiceB.java".to_string(), vec![
        create_function_with_complexity("methodC", "ServiceB.java", 6),
    ]);

    let mut target_files = HashMap::new();
    target_files.insert("ServiceA.java".to_string(), vec![
        create_function_with_complexity("methodA", "ServiceA.java", 5),
    ]);
    target_files.insert("ServiceB.java".to_string(), vec![
        create_function_with_complexity("methodC", "ServiceB.java", 6),
        create_function_with_complexity("methodB", "ServiceB.java", 7), // Moved
    ]);
    target_files.insert("ServiceC.java".to_string(), vec![
        create_function_with_complexity("methodD", "ServiceC.java", 8), // New
    ]);

    for (config_name, config) in configs {
        println!("🔧 Testing {} Configuration:", config_name);
        
        let mut tracker = CrossFileTracker::new(Language::Java, config);
        let result = tracker.track_cross_file_changes(&source_files, &target_files)?;
        
        println!("  • Moves detected: {}", result.moved_functions.len());
        println!("  • Rename+moves: {}", result.renamed_and_moved.len());
        println!("  • Splits: {}", result.cross_file_splits.len());
        println!("  • Merges: {}", result.cross_file_merges.len());
        println!("  • Average confidence: {:.2}%", result.overall_statistics.average_confidence * 100.0);
        println!("  • Execution time: {}ms", result.overall_statistics.execution_time_ms);
        println!();
    }

    Ok(())
}

/// Helper function to create a function signature with complexity metrics
fn create_function_with_complexity(name: &str, file_path: &str, complexity: u32) -> (EnhancedFunctionSignature, ASTNode) {
    let signature = EnhancedFunctionSignature {
        name: name.to_string(),
        qualified_name: format!("TestClass.{}", name),
        parameters: vec![
            ParameterInfo {
                name: "param1".to_string(),
                param_type: TypeSignature::new("String".to_string()),
                default_value: None,
                is_varargs: false,
                annotations: Vec::new(),
            }
        ],
        return_type: TypeSignature::new("void".to_string()),
        generic_parameters: Vec::new(),
        visibility: Visibility::Public,
        modifiers: Vec::new(),
        annotations: Vec::new(),
        file_path: file_path.to_string(),
        line: 1,
        column: 1,
        end_line: 20,
        function_type: FunctionType::Method,
        complexity_metrics: Some(ComplexityMetrics {
            cyclomatic_complexity: complexity,
            cognitive_complexity: complexity + 2,
            nesting_depth: 3,
            parameter_count: 1,
            return_points: 1,
            lines_of_code: complexity * 2,
        }),
        dependencies: Vec::new(),
        signature_hash: format!("{}_hash", name),
        normalized_hash: format!("{}_normalized", name),
    };

    let ast_node = ASTNode {
        node_type: NodeType::Function,
        children: Vec::new(),
        metadata: NodeMetadata {
            line: 1,
            column: 1,
            attributes: HashMap::new(),
        },
    };

    (signature, ast_node)
}
