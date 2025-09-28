//! Comprehensive demonstration of Hungarian Algorithm Function Matching
//! 
//! This example showcases the advanced Hungarian algorithm implementation for optimal
//! function matching, including one-to-one assignments, many-to-many mappings for
//! split/merge detection, and detailed matching statistics.

use smart_diff_engine::{
    HungarianMatcher, HungarianMatcherConfig, MappingType
};
use smart_diff_parser::{ASTNode, NodeType, NodeMetadata, Language};
use smart_diff_semantic::{
    EnhancedFunctionSignature, FunctionType, Visibility, TypeSignature,
    ParameterInfo, ComplexityMetrics
};
use std::collections::HashMap;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🔍 Hungarian Algorithm Function Matching Demo");
    println!("==============================================\n");

    // Demo 1: Basic optimal matching
    demo_basic_optimal_matching()?;
    
    // Demo 2: Split detection (1 -> N mapping)
    demo_split_detection()?;
    
    // Demo 3: Merge detection (N -> 1 mapping)
    demo_merge_detection()?;
    
    // Demo 4: Complex many-to-many mappings
    demo_complex_mappings()?;
    
    // Demo 5: Cross-file matching with penalties
    demo_cross_file_matching()?;
    
    // Demo 6: Configuration and performance tuning
    demo_configuration_tuning()?;

    println!("\n✅ Hungarian Algorithm Function Matching Demo Complete!");
    Ok(())
}

/// Demo 1: Basic optimal matching between similar functions
fn demo_basic_optimal_matching() -> Result<()> {
    println!("📊 Demo 1: Basic Optimal Function Matching");
    println!("-------------------------------------------");

    let mut matcher = HungarianMatcher::with_defaults(Language::Java);

    // Create source functions (version 1)
    let source_functions = vec![
        create_function_with_complexity("calculateSum", "Calculator.java", 5),
        create_function_with_complexity("validateInput", "Validator.java", 8),
        create_function_with_complexity("processData", "DataProcessor.java", 12),
        create_function_with_complexity("formatOutput", "Formatter.java", 3),
    ];

    // Create target functions (version 2) - some renamed, some modified
    let target_functions = vec![
        create_function_with_complexity("computeSum", "Calculator.java", 6), // Renamed
        create_function_with_complexity("validateUserInput", "Validator.java", 9), // Renamed + modified
        create_function_with_complexity("processDataAdvanced", "DataProcessor.java", 15), // Enhanced
        create_function_with_complexity("formatOutput", "Formatter.java", 3), // Unchanged
        create_function_with_complexity("logResults", "Logger.java", 4), // New function
    ];

    let result = matcher.match_functions(&source_functions, &target_functions)?;

    println!("📈 Matching Results:");
    println!("  • One-to-one assignments: {}", result.assignments.len());
    println!("  • Unmatched source (deletions): {}", result.unmatched_source.len());
    println!("  • Unmatched target (additions): {}", result.unmatched_target.len());
    println!("  • Average similarity: {:.2}%", result.average_similarity * 100.0);
    println!("  • Match percentage: {:.1}%", result.statistics.match_percentage);
    println!("  • Total assignment cost: {:.3}", result.total_cost);

    // Show detailed assignments
    for assignment in &result.assignments {
        let source_name = &source_functions[assignment.source_index].0.name;
        let target_name = &target_functions[assignment.target_index].0.name;
        println!("  📝 {} → {} (similarity: {:.2}%, confidence: {:.2}%)",
            source_name, target_name,
            assignment.similarity.overall_similarity * 100.0,
            assignment.confidence * 100.0
        );
    }

    println!();
    Ok(())
}

/// Demo 2: Split detection - one function becomes multiple functions
fn demo_split_detection() -> Result<()> {
    println!("🔀 Demo 2: Function Split Detection");
    println!("-----------------------------------");

    let mut config = HungarianMatcherConfig::default();
    config.enable_many_to_many = true;
    config.min_similarity_threshold = 0.6; // Lower threshold for split detection
    
    let mut matcher = HungarianMatcher::new(Language::Java, config);

    // Source: One large function
    let source_functions = vec![
        create_function_with_complexity("processUserRequest", "RequestHandler.java", 25),
    ];

    // Target: Split into multiple smaller functions
    let target_functions = vec![
        create_function_with_complexity("validateUserRequest", "RequestHandler.java", 8),
        create_function_with_complexity("parseUserRequest", "RequestHandler.java", 6),
        create_function_with_complexity("executeUserRequest", "RequestHandler.java", 10),
        create_function_with_complexity("formatUserResponse", "RequestHandler.java", 5),
    ];

    let result = matcher.match_functions(&source_functions, &target_functions)?;

    println!("📈 Split Detection Results:");
    println!("  • One-to-one assignments: {}", result.assignments.len());
    println!("  • Many-to-many mappings: {}", result.many_to_many_mappings.len());

    for mapping in &result.many_to_many_mappings {
        if mapping.mapping_type == MappingType::Split {
            let source_name = &source_functions[mapping.source_indices[0]].0.name;
            let target_names: Vec<String> = mapping.target_indices.iter()
                .map(|&idx| target_functions[idx].0.name.clone())
                .collect();
            
            println!("  🔀 SPLIT: {} → [{}]", 
                source_name, 
                target_names.join(", ")
            );
            println!("     Combined similarity: {:.2}%, Confidence: {:.2}%",
                mapping.combined_similarity * 100.0,
                mapping.confidence * 100.0
            );
        }
    }

    println!();
    Ok(())
}

/// Demo 3: Merge detection - multiple functions become one function
fn demo_merge_detection() -> Result<()> {
    println!("🔗 Demo 3: Function Merge Detection");
    println!("-----------------------------------");

    let mut config = HungarianMatcherConfig::default();
    config.enable_many_to_many = true;
    config.min_similarity_threshold = 0.6;
    
    let mut matcher = HungarianMatcher::new(Language::Java, config);

    // Source: Multiple small functions
    let source_functions = vec![
        create_function_with_complexity("readConfigFile", "ConfigManager.java", 4),
        create_function_with_complexity("parseConfigData", "ConfigManager.java", 6),
        create_function_with_complexity("validateConfigData", "ConfigManager.java", 5),
        create_function_with_complexity("applyConfigSettings", "ConfigManager.java", 7),
    ];

    // Target: Merged into one comprehensive function
    let target_functions = vec![
        create_function_with_complexity("loadAndApplyConfiguration", "ConfigManager.java", 20),
    ];

    let result = matcher.match_functions(&source_functions, &target_functions)?;

    println!("📈 Merge Detection Results:");
    println!("  • One-to-one assignments: {}", result.assignments.len());
    println!("  • Many-to-many mappings: {}", result.many_to_many_mappings.len());

    for mapping in &result.many_to_many_mappings {
        if mapping.mapping_type == MappingType::Merge {
            let source_names: Vec<String> = mapping.source_indices.iter()
                .map(|&idx| source_functions[idx].0.name.clone())
                .collect();
            let target_name = &target_functions[mapping.target_indices[0]].0.name;
            
            println!("  🔗 MERGE: [{}] → {}", 
                source_names.join(", "), 
                target_name
            );
            println!("     Combined similarity: {:.2}%, Confidence: {:.2}%",
                mapping.combined_similarity * 100.0,
                mapping.confidence * 100.0
            );
        }
    }

    println!();
    Ok(())
}

/// Demo 4: Complex many-to-many mappings
fn demo_complex_mappings() -> Result<()> {
    println!("🌐 Demo 4: Complex Many-to-Many Mappings");
    println!("----------------------------------------");

    let mut config = HungarianMatcherConfig::default();
    config.enable_many_to_many = true;
    config.min_similarity_threshold = 0.5;
    
    let mut matcher = HungarianMatcher::new(Language::Java, config);

    // Source: Authentication and authorization functions
    let source_functions = vec![
        create_function_with_complexity("authenticateUser", "AuthService.java", 8),
        create_function_with_complexity("authorizeAction", "AuthService.java", 6),
        create_function_with_complexity("logSecurityEvent", "SecurityLogger.java", 4),
    ];

    // Target: Refactored security system
    let target_functions = vec![
        create_function_with_complexity("validateCredentials", "SecurityService.java", 7),
        create_function_with_complexity("checkPermissions", "SecurityService.java", 5),
        create_function_with_complexity("auditSecurityAction", "AuditService.java", 6),
    ];

    let result = matcher.match_functions(&source_functions, &target_functions)?;

    println!("📈 Complex Mapping Results:");
    println!("  • One-to-one assignments: {}", result.assignments.len());
    println!("  • Many-to-many mappings: {}", result.many_to_many_mappings.len());
    println!("  • Execution time: {}ms", result.statistics.execution_time_ms);

    for mapping in &result.many_to_many_mappings {
        if mapping.mapping_type == MappingType::Complex {
            let source_names: Vec<String> = mapping.source_indices.iter()
                .map(|&idx| source_functions[idx].0.name.clone())
                .collect();
            let target_names: Vec<String> = mapping.target_indices.iter()
                .map(|&idx| target_functions[idx].0.name.clone())
                .collect();
            
            println!("  🌐 COMPLEX: [{}] ↔ [{}]", 
                source_names.join(", "), 
                target_names.join(", ")
            );
            println!("     Combined similarity: {:.2}%, Confidence: {:.2}%",
                mapping.combined_similarity * 100.0,
                mapping.confidence * 100.0
            );
        }
    }

    println!();
    Ok(())
}

/// Demo 5: Cross-file matching with penalties
fn demo_cross_file_matching() -> Result<()> {
    println!("📁 Demo 5: Cross-File Matching with Penalties");
    println!("---------------------------------------------");

    let mut config = HungarianMatcherConfig::default();
    config.enable_cross_file_matching = true;
    config.cross_file_penalty = 0.15; // 15% penalty for cross-file moves
    
    let mut matcher = HungarianMatcher::new(Language::Java, config);

    // Source functions in different files
    let source_functions = vec![
        create_function_with_complexity("calculateTax", "TaxCalculator.java", 10),
        create_function_with_complexity("formatCurrency", "CurrencyFormatter.java", 5),
        create_function_with_complexity("validateAmount", "AmountValidator.java", 6),
    ];

    // Target functions - some moved to different files
    let target_functions = vec![
        create_function_with_complexity("calculateTax", "FinancialCalculator.java", 10), // Moved file
        create_function_with_complexity("formatCurrency", "CurrencyFormatter.java", 5), // Same file
        create_function_with_complexity("validateAmount", "InputValidator.java", 6), // Moved file
    ];

    let result = matcher.match_functions(&source_functions, &target_functions)?;

    println!("📈 Cross-File Matching Results:");
    println!("  • Total assignments: {}", result.assignments.len());
    println!("  • Average similarity: {:.2}%", result.average_similarity * 100.0);

    for assignment in &result.assignments {
        let source_func = &source_functions[assignment.source_index].0;
        let target_func = &target_functions[assignment.target_index].0;
        let is_cross_file = source_func.file_path != target_func.file_path;
        
        println!("  📝 {} → {} {}",
            source_func.name, target_func.name,
            if is_cross_file { "(CROSS-FILE)" } else { "(SAME-FILE)" }
        );
        println!("     Similarity: {:.2}%, Cost: {:.3}",
            assignment.similarity.overall_similarity * 100.0,
            assignment.cost
        );
    }

    println!();
    Ok(())
}

/// Demo 6: Configuration and performance tuning
fn demo_configuration_tuning() -> Result<()> {
    println!("⚙️  Demo 6: Configuration and Performance Tuning");
    println!("------------------------------------------------");

    // Test different configurations
    let configs = vec![
        ("High Precision", HungarianMatcherConfig {
            min_similarity_threshold: 0.8,
            max_assignment_cost: 0.2,
            enable_many_to_many: true,
            max_candidates_per_function: 15,
            enable_cross_file_matching: true,
            cross_file_penalty: 0.05,
        }),
        ("Balanced", HungarianMatcherConfig::default()),
        ("Fast Matching", HungarianMatcherConfig {
            min_similarity_threshold: 0.6,
            max_assignment_cost: 0.4,
            enable_many_to_many: false,
            max_candidates_per_function: 5,
            enable_cross_file_matching: false,
            cross_file_penalty: 0.0,
        }),
    ];

    // Create test functions
    let source_functions = vec![
        create_function_with_complexity("processOrder", "OrderService.java", 12),
        create_function_with_complexity("validatePayment", "PaymentService.java", 8),
        create_function_with_complexity("updateInventory", "InventoryService.java", 10),
    ];

    let target_functions = vec![
        create_function_with_complexity("processCustomerOrder", "OrderService.java", 14),
        create_function_with_complexity("validatePaymentMethod", "PaymentService.java", 9),
        create_function_with_complexity("updateProductInventory", "InventoryService.java", 11),
        create_function_with_complexity("sendNotification", "NotificationService.java", 5),
    ];

    for (config_name, config) in configs {
        println!("🔧 Testing {} Configuration:", config_name);
        
        let mut matcher = HungarianMatcher::new(Language::Java, config);
        let result = matcher.match_functions(&source_functions, &target_functions)?;
        
        println!("  • Assignments: {}", result.assignments.len());
        println!("  • Average similarity: {:.2}%", result.average_similarity * 100.0);
        println!("  • Match percentage: {:.1}%", result.statistics.match_percentage);
        println!("  • Execution time: {}ms", result.statistics.execution_time_ms);
        println!("  • Many-to-many mappings: {}", result.many_to_many_mappings.len());
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
