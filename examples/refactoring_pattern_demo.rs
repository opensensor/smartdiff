//! Comprehensive demonstration of Refactoring Pattern Detection System
//! 
//! This example showcases the advanced refactoring pattern detection system that identifies
//! common code refactoring patterns with confidence scoring, detailed analysis, and
//! comprehensive evidence gathering for intelligent refactoring recognition.

use smart_diff_engine::{
    RefactoringDetector, RefactoringDetectionConfig, RefactoringPattern,
    RefactoringImpactLevel, RefactoringComplexityLevel, RefactoringEffort,
    RefactoringCharacteristicType, RefactoringEvidenceType
};
use smart_diff_parser::{Change, ChangeType, ChangeDetail, CodeElement, Language, RefactoringType};
use std::collections::HashMap;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🔄 Refactoring Pattern Detection System Demo");
    println!("============================================\n");

    // Demo 1: Basic refactoring pattern detection
    demo_basic_pattern_detection()?;
    
    // Demo 2: Extract method pattern analysis
    demo_extract_method_detection()?;
    
    // Demo 3: Inline method pattern analysis
    demo_inline_method_detection()?;
    
    // Demo 4: Rename and move pattern detection
    demo_rename_move_detection()?;
    
    // Demo 5: Complex refactoring patterns
    demo_complex_pattern_detection()?;
    
    // Demo 6: Configuration and customization
    demo_configuration_customization()?;

    println!("\n✅ Refactoring Pattern Detection System Demo Complete!");
    Ok(())
}

/// Demo 1: Basic refactoring pattern detection
fn demo_basic_pattern_detection() -> Result<()> {
    println!("📊 Demo 1: Basic Refactoring Pattern Detection");
    println!("----------------------------------------------");

    let detector = RefactoringDetector::new(Language::Java);

    // Test different refactoring patterns
    let test_scenarios = vec![
        ("Extract Method", vec![
            create_change(ChangeType::Modify, 
                Some(("processOrder", "OrderService.java", 20)), 
                Some(("processOrder", "OrderService.java", 20)), 
                Some(0.8)),
            create_change(ChangeType::Add, 
                None, 
                Some(("validateOrder", "OrderService.java", 60)), 
                None),
        ]),
        ("Inline Method", vec![
            create_change(ChangeType::Delete, 
                Some(("helper", "Utils.java", 100)), 
                None, 
                None),
            create_change(ChangeType::Modify, 
                Some(("mainMethod", "Utils.java", 50)), 
                Some(("mainMethod", "Utils.java", 50)), 
                Some(0.7)),
        ]),
        ("Rename Method", vec![
            create_change(ChangeType::Rename, 
                Some(("calculatePrice", "PriceCalculator.java", 30)), 
                Some(("computeCost", "PriceCalculator.java", 30)), 
                Some(0.9)),
        ]),
        ("Move Method", vec![
            create_change(ChangeType::CrossFileMove, 
                Some(("utility", "OldUtils.java", 10)), 
                Some(("utility", "helpers/StringUtils.java", 15)), 
                Some(0.95)),
        ]),
    ];

    for (scenario_name, changes) in test_scenarios {
        println!("🔍 Scenario: {}", scenario_name);
        
        let patterns = detector.detect_patterns(&changes);
        
        if patterns.is_empty() {
            println!("  • No patterns detected");
        } else {
            for (i, pattern) in patterns.iter().enumerate() {
                println!("  • Pattern {}: {:?} (confidence: {:.3})", 
                    i + 1, pattern.pattern_type, pattern.confidence);
                println!("    Description: {}", pattern.description);
                println!("    Affected Elements: {:?}", pattern.affected_elements);
                println!("    Complexity: {:?}", pattern.complexity.complexity_level);
            }
        }
        println!();
    }

    Ok(())
}

/// Demo 2: Extract method pattern analysis
fn demo_extract_method_detection() -> Result<()> {
    println!("🔬 Demo 2: Extract Method Pattern Analysis");
    println!("------------------------------------------");

    let detector = RefactoringDetector::new(Language::Java);

    // Simulate extract method refactoring
    let changes = vec![
        create_change(ChangeType::Modify, 
            Some(("processPayment", "PaymentService.java", 25)), 
            Some(("processPayment", "PaymentService.java", 25)), 
            Some(0.75)), // Reduced similarity due to extracted code
        create_change(ChangeType::Add, 
            None, 
            Some(("validatePaymentData", "PaymentService.java", 80)), 
            None),
    ];

    let patterns = detector.detect_patterns(&changes);
    
    println!("🔍 Extract Method Analysis:");
    for pattern in &patterns {
        if pattern.pattern_type == RefactoringType::ExtractMethod {
            println!("  • Pattern: {:?}", pattern.pattern_type);
            println!("  • Confidence: {:.3}", pattern.confidence);
            println!("  • Description: {}", pattern.description);
            
            println!("\n📋 Characteristics:");
            for (i, characteristic) in pattern.analysis.characteristics.iter().enumerate() {
                println!("    {}. {:?}: {} (confidence: {:.3})", 
                    i + 1, characteristic.characteristic_type, characteristic.value, characteristic.confidence);
            }
            
            println!("\n🔍 Evidence:");
            for (i, evidence) in pattern.evidence.iter().enumerate() {
                println!("    {}. {:?}: {} (strength: {:.3})", 
                    i + 1, evidence.evidence_type, evidence.description, evidence.strength);
            }
            
            println!("\n⚠️  Impact Assessment:");
            println!("    • Impact Level: {:?}", pattern.analysis.impact.impact_level);
            println!("    • Breaking Change: {}", pattern.analysis.impact.is_breaking_change);
            println!("    • API Compatibility: {:?}", pattern.analysis.impact.api_compatibility);
            println!("    • Affected Files: {:?}", pattern.analysis.impact.affected_files);
            
            println!("\n📊 Quality Metrics:");
            println!("    • Quality Improvement: {:.2}", pattern.analysis.quality_metrics.quality_improvement);
            println!("    • Maintainability Impact: {:.2}", pattern.analysis.quality_metrics.maintainability_impact);
            println!("    • Readability Impact: {:.2}", pattern.analysis.quality_metrics.readability_impact);
            println!("    • Testability Impact: {:.2}", pattern.analysis.quality_metrics.testability_impact);
            
            println!("\n🔧 Complexity Assessment:");
            println!("    • Complexity Level: {:?}", pattern.complexity.complexity_level);
            println!("    • Elements Involved: {}", pattern.complexity.elements_involved);
            println!("    • Files Affected: {}", pattern.complexity.files_affected);
            println!("    • Estimated Effort: {:?}", pattern.complexity.estimated_effort);
        }
    }

    println!();
    Ok(())
}

/// Demo 3: Inline method pattern analysis
fn demo_inline_method_detection() -> Result<()> {
    println!("📥 Demo 3: Inline Method Pattern Analysis");
    println!("-----------------------------------------");

    let detector = RefactoringDetector::new(Language::Java);

    // Simulate inline method refactoring
    let changes = vec![
        create_change(ChangeType::Delete, 
            Some(("isValidEmail", "ValidationUtils.java", 45)), 
            None, 
            None),
        create_change(ChangeType::Modify, 
            Some(("validateUser", "UserService.java", 30)), 
            Some(("validateUser", "UserService.java", 30)), 
            Some(0.65)), // Lower similarity due to inlined code
    ];

    let patterns = detector.detect_patterns(&changes);
    
    println!("🔍 Inline Method Analysis:");
    for pattern in &patterns {
        if pattern.pattern_type == RefactoringType::InlineMethod {
            println!("  • Pattern: {:?}", pattern.pattern_type);
            println!("  • Confidence: {:.3}", pattern.confidence);
            println!("  • Description: {}", pattern.description);
            
            println!("\n📈 Quality Impact:");
            let quality = &pattern.analysis.quality_metrics;
            println!("    • Quality Improvement: {:.2} ({})", 
                quality.quality_improvement,
                if quality.quality_improvement > 0.5 { "Positive" } else { "Neutral/Negative" });
            println!("    • Maintainability: {:.2}", quality.maintainability_impact);
            println!("    • Performance: {:.2}", quality.performance_impact);
            
            println!("\n🎯 Refactoring Rationale:");
            if pattern.confidence > 0.7 {
                println!("    • High confidence inline method detected");
                println!("    • Likely removed small, single-use method");
                println!("    • May improve performance by reducing method calls");
            } else {
                println!("    • Moderate confidence inline method");
                println!("    • Consider if this reduces code readability");
            }
        }
    }

    println!();
    Ok(())
}

/// Demo 4: Rename and move pattern detection
fn demo_rename_move_detection() -> Result<()> {
    println!("🏷️  Demo 4: Rename and Move Pattern Detection");
    println!("---------------------------------------------");

    let detector = RefactoringDetector::new(Language::Java);

    // Test rename patterns
    let rename_changes = vec![
        create_change(ChangeType::Rename, 
            Some(("getUserData", "UserController.java", 40)), 
            Some(("fetchUserProfile", "UserController.java", 40)), 
            Some(0.85)),
    ];

    // Test move patterns
    let move_changes = vec![
        create_change(ChangeType::CrossFileMove, 
            Some(("formatDate", "Utils.java", 20)), 
            Some(("formatDate", "helpers/DateUtils.java", 10)), 
            Some(0.98)),
    ];

    println!("🔍 Rename Pattern Analysis:");
    let rename_patterns = detector.detect_patterns(&rename_changes);
    for pattern in &rename_patterns {
        println!("  • Pattern: {:?} (confidence: {:.3})", pattern.pattern_type, pattern.confidence);
        println!("  • Description: {}", pattern.description);
        println!("  • Breaking Change: {}", pattern.analysis.impact.is_breaking_change);
        
        // Analyze name change quality
        if pattern.confidence > 0.8 {
            println!("  • Assessment: High-quality rename with good semantic similarity");
        } else if pattern.confidence > 0.6 {
            println!("  • Assessment: Moderate rename, verify semantic correctness");
        } else {
            println!("  • Assessment: Low confidence rename, may be unrelated change");
        }
    }

    println!("\n🔍 Move Pattern Analysis:");
    let move_patterns = detector.detect_patterns(&move_changes);
    for pattern in &move_patterns {
        println!("  • Pattern: {:?} (confidence: {:.3})", pattern.pattern_type, pattern.confidence);
        println!("  • Description: {}", pattern.description);
        println!("  • Complexity: {:?}", pattern.complexity.complexity_level);
        
        // Analyze move impact
        match pattern.complexity.complexity_level {
            RefactoringComplexityLevel::Simple => {
                println!("  • Assessment: Simple move, low risk");
            }
            RefactoringComplexityLevel::Moderate => {
                println!("  • Assessment: Moderate complexity, check dependencies");
            }
            RefactoringComplexityLevel::Complex => {
                println!("  • Assessment: Complex move, extensive testing recommended");
            }
            RefactoringComplexityLevel::VeryComplex => {
                println!("  • Assessment: Very complex move, consider phased approach");
            }
        }
    }

    println!();
    Ok(())
}

/// Demo 5: Complex refactoring patterns
fn demo_complex_pattern_detection() -> Result<()> {
    println!("🌟 Demo 5: Complex Refactoring Pattern Detection");
    println!("------------------------------------------------");

    let detector = RefactoringDetector::new(Language::Java);

    // Simulate extract class refactoring
    let extract_class_changes = vec![
        create_change(ChangeType::Add, 
            None, 
            Some(("validateOrder", "OrderValidator.java", 10)), 
            None),
        create_change(ChangeType::Add, 
            None, 
            Some(("calculateDiscount", "OrderValidator.java", 30)), 
            None),
        create_change(ChangeType::Add, 
            None, 
            Some(("checkInventory", "OrderValidator.java", 50)), 
            None),
        create_change(ChangeType::Modify, 
            Some(("processOrder", "OrderService.java", 100)), 
            Some(("processOrder", "OrderService.java", 100)), 
            Some(0.6)),
    ];

    println!("🔍 Extract Class Pattern Analysis:");
    let patterns = detector.detect_patterns(&extract_class_changes);
    
    for pattern in &patterns {
        if pattern.pattern_type == RefactoringType::ExtractClass {
            println!("  • Pattern: {:?}", pattern.pattern_type);
            println!("  • Confidence: {:.3}", pattern.confidence);
            println!("  • Description: {}", pattern.description);
            println!("  • Affected Elements: {} methods", pattern.affected_elements.len());
            
            println!("\n📊 Complexity Analysis:");
            println!("    • Complexity Level: {:?}", pattern.complexity.complexity_level);
            println!("    • Elements Involved: {}", pattern.complexity.elements_involved);
            println!("    • Files Affected: {}", pattern.complexity.files_affected);
            
            let effort_description = match pattern.complexity.estimated_effort {
                RefactoringEffort::Trivial => "< 1 hour",
                RefactoringEffort::Low => "1-4 hours",
                RefactoringEffort::Medium => "1-2 days",
                RefactoringEffort::High => "3-5 days",
                RefactoringEffort::VeryHigh => "> 1 week",
            };
            println!("    • Estimated Effort: {} ({})", 
                format!("{:?}", pattern.complexity.estimated_effort), effort_description);
            
            println!("\n🎯 Refactoring Benefits:");
            let quality = &pattern.analysis.quality_metrics;
            if quality.maintainability_impact > 0.7 {
                println!("    • Significant maintainability improvement expected");
            }
            if quality.readability_impact > 0.6 {
                println!("    • Good readability improvement through separation of concerns");
            }
            if quality.testability_impact > 0.7 {
                println!("    • Enhanced testability through focused class responsibility");
            }
        }
    }

    println!();
    Ok(())
}

/// Demo 6: Configuration and customization
fn demo_configuration_customization() -> Result<()> {
    println!("⚙️  Demo 6: Configuration and Customization");
    println!("-------------------------------------------");

    // Test different configurations
    let configs = vec![
        ("Conservative", RefactoringDetectionConfig {
            min_confidence_threshold: 0.9,
            enable_extract_method: true,
            enable_inline_method: false, // Disabled for conservative approach
            enable_rename_detection: true,
            enable_move_detection: true,
            enable_extract_class: true,
            enable_inline_class: false, // Disabled for conservative approach
            enable_change_signature: false, // Disabled for conservative approach
            max_related_distance: 25,
            enable_complex_patterns: false,
        }),
        ("Balanced", RefactoringDetectionConfig::default()),
        ("Aggressive", RefactoringDetectionConfig {
            min_confidence_threshold: 0.5,
            enable_extract_method: true,
            enable_inline_method: true,
            enable_rename_detection: true,
            enable_move_detection: true,
            enable_extract_class: true,
            enable_inline_class: true,
            enable_change_signature: true,
            max_related_distance: 100,
            enable_complex_patterns: true,
        }),
    ];

    let test_changes = vec![
        create_change(ChangeType::Modify, 
            Some(("method", "Service.java", 10)), 
            Some(("method", "Service.java", 10)), 
            Some(0.6)), // Moderate similarity
        create_change(ChangeType::Add, 
            None, 
            Some(("helper", "Service.java", 50)), 
            None),
    ];

    for (config_name, config) in configs {
        let detector = RefactoringDetector::with_config(Language::Java, config);
        let patterns = detector.detect_patterns(&test_changes);
        
        println!("🔧 {} Configuration:", config_name);
        println!("  • Confidence Threshold: {:.2}", detector.get_config().min_confidence_threshold);
        println!("  • Patterns Detected: {}", patterns.len());
        
        for pattern in &patterns {
            println!("    - {:?} (confidence: {:.3})", pattern.pattern_type, pattern.confidence);
        }
        
        let supported_types = detector.get_supported_refactoring_types();
        println!("  • Supported Types: {} patterns", supported_types.len());
        println!();
    }

    // Demonstrate feature toggling
    println!("🔄 Feature Toggling Demo:");
    let mut detector = RefactoringDetector::new(Language::Java);
    
    println!("  • Initial state:");
    println!("    - Change Classifier: {}", detector.change_classifier.is_some());
    println!("    - Similarity Scorer: {}", detector.similarity_scorer.is_some());
    
    detector.set_change_classifier(false);
    detector.set_similarity_scorer(false);
    
    println!("  • After disabling advanced features:");
    println!("    - Change Classifier: {}", detector.change_classifier.is_some());
    println!("    - Similarity Scorer: {}", detector.similarity_scorer.is_some());
    
    detector.set_change_classifier(true);
    detector.set_similarity_scorer(true);
    
    println!("  • After re-enabling:");
    println!("    - Change Classifier: {}", detector.change_classifier.is_some());
    println!("    - Similarity Scorer: {}", detector.similarity_scorer.is_some());

    println!();
    Ok(())
}

// Helper functions for creating test data

fn create_change(
    change_type: ChangeType,
    source: Option<(&str, &str, usize)>,
    target: Option<(&str, &str, usize)>,
    similarity_score: Option<f64>,
) -> Change {
    Change {
        change_type,
        source: source.map(|(name, file, line)| CodeElement {
            name: name.to_string(),
            file_path: file.to_string(),
            start_line: line,
            end_line: line + 10,
            element_type: "function".to_string(),
        }),
        target: target.map(|(name, file, line)| CodeElement {
            name: name.to_string(),
            file_path: file.to_string(),
            start_line: line,
            end_line: line + 10,
            element_type: "function".to_string(),
        }),
        details: ChangeDetail {
            description: "Test change".to_string(),
            affected_lines: vec![1, 2, 3],
            similarity_score,
            refactoring_type: None,
            metadata: HashMap::new(),
        },
        confidence: 0.8,
    }
}
