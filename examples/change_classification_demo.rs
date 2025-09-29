//! Comprehensive demonstration of Change Classification System
//! 
//! This example showcases the advanced change classification system that categorizes
//! code changes with detailed analysis, confidence scoring, and impact assessment
//! using AST-level analysis and semantic understanding.

use smart_diff_engine::{
    ChangeClassifier, ChangeClassificationConfig, DetailedChangeClassification,
    ImpactLevel, EffortLevel, RiskLevel, CharacteristicType, EvidenceType
};
use smart_diff_parser::{ASTNode, NodeType, NodeMetadata, Language, CodeElement};
use smart_diff_semantic::{
    EnhancedFunctionSignature, FunctionType, Visibility, TypeSignature,
    ParameterInfo, ComplexityMetrics
};
use std::collections::HashMap;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🔍 Change Classification System Demo");
    println!("===================================\n");

    // Demo 1: Basic change type classification
    demo_basic_change_classification()?;
    
    // Demo 2: Detailed change analysis with confidence scoring
    demo_detailed_change_analysis()?;
    
    // Demo 3: Impact assessment and risk analysis
    demo_impact_assessment()?;
    
    // Demo 4: Configuration and customization
    demo_configuration_customization()?;
    
    // Demo 5: Real-world change scenarios
    demo_real_world_scenarios()?;
    
    // Demo 6: Advanced features and integration
    demo_advanced_features()?;

    println!("\n✅ Change Classification System Demo Complete!");
    Ok(())
}

/// Demo 1: Basic change type classification
fn demo_basic_change_classification() -> Result<()> {
    println!("📊 Demo 1: Basic Change Type Classification");
    println!("-------------------------------------------");

    let classifier = ChangeClassifier::new(Language::Java);

    // Test different change types
    let test_cases = vec![
        ("Addition", None, Some(create_code_element("newFunction", "test.java", 10))),
        ("Deletion", Some(create_code_element("oldFunction", "test.java", 10)), None),
        ("Rename", Some(create_code_element("oldName", "test.java", 10)), Some(create_code_element("newName", "test.java", 10))),
        ("Move", Some(create_code_element("function", "test.java", 10)), Some(create_code_element("function", "test.java", 50))),
        ("Cross-file Move", Some(create_code_element("function", "old.java", 10)), Some(create_code_element("function", "new.java", 10))),
        ("Modification", Some(create_code_element("function", "test.java", 10)), Some(create_code_element("function", "test.java", 10))),
    ];

    for (test_name, source, target) in test_cases {
        let change_type = classifier.classify_change(source.as_ref(), target.as_ref());
        println!("🔍 {}: {:?}", test_name, change_type);
    }

    println!();
    Ok(())
}

/// Demo 2: Detailed change analysis with confidence scoring
fn demo_detailed_change_analysis() -> Result<()> {
    println!("🔬 Demo 2: Detailed Change Analysis");
    println!("-----------------------------------");

    let classifier = ChangeClassifier::new(Language::Java);

    // Test detailed analysis for a function rename
    let source_element = create_code_element("calculateSum", "Calculator.java", 15);
    let target_element = create_code_element("computeTotal", "Calculator.java", 15);
    let source_signature = create_function_signature("calculateSum", 3, Visibility::Public);
    let target_signature = create_function_signature("computeTotal", 4, Visibility::Public);

    let result = classifier.classify_change_detailed(
        Some(&source_element),
        Some(&target_element),
        None, // No AST for this demo
        None,
        Some(&source_signature),
        Some(&target_signature),
    )?;

    println!("🔍 Detailed Analysis Results:");
    println!("  • Change Type: {:?}", result.change_type);
    println!("  • Confidence: {:.3}", result.confidence);
    println!("  • Description: {}", result.analysis.description);
    println!("  • Complexity Score: {:.2}", result.analysis.complexity_score);
    
    println!("\n📋 Characteristics:");
    for (i, characteristic) in result.analysis.characteristics.iter().enumerate() {
        println!("  {}. {:?}: {} (confidence: {:.3})", 
            i + 1, characteristic.characteristic_type, characteristic.value, characteristic.confidence);
    }
    
    println!("\n🔍 Evidence:");
    for (i, evidence) in result.analysis.evidence.iter().enumerate() {
        println!("  {}. {:?}: {} (strength: {:.3})", 
            i + 1, evidence.evidence_type, evidence.description, evidence.strength);
    }
    
    println!("\n⚠️  Impact Assessment:");
    println!("  • Impact Level: {:?}", result.impact.impact_level);
    println!("  • Implementation Effort: {:?}", result.impact.implementation_effort);
    println!("  • Risk Level: {:?}", result.impact.risk_level);
    println!("  • Breaking Change: {}", result.impact.is_breaking_change);
    println!("  • Affected Components: {:?}", result.impact.affected_components);

    if !result.analysis.alternatives.is_empty() {
        println!("\n🤔 Alternative Classifications:");
        for (i, alt) in result.analysis.alternatives.iter().enumerate() {
            println!("  {}. {:?} (confidence: {:.3}) - {}", 
                i + 1, alt.change_type, alt.confidence, alt.reason);
        }
    }

    println!();
    Ok(())
}

/// Demo 3: Impact assessment and risk analysis
fn demo_impact_assessment() -> Result<()> {
    println!("⚡ Demo 3: Impact Assessment and Risk Analysis");
    println!("---------------------------------------------");

    let classifier = ChangeClassifier::new(Language::Java);

    // Test different impact scenarios
    let scenarios = vec![
        ("Low Impact Addition", 
         None, 
         Some((create_code_element("helper", "Utils.java", 100), 
               create_function_signature("helper", 2, Visibility::Private)))),
        
        ("High Impact Deletion", 
         Some((create_code_element("publicAPI", "API.java", 10), 
               create_function_signature("publicAPI", 15, Visibility::Public))), 
         None),
        
        ("Breaking Change", 
         Some((create_code_element("process", "Service.java", 20), 
               create_function_signature_with_params("process", 8, Visibility::Public, 2))), 
         Some((create_code_element("process", "Service.java", 20), 
               create_function_signature_with_params("process", 8, Visibility::Public, 3)))),
    ];

    for (scenario_name, source_data, target_data) in scenarios {
        let (source_element, source_signature) = match source_data {
            Some((elem, sig)) => (Some(elem), Some(sig)),
            None => (None, None),
        };
        
        let (target_element, target_signature) = match target_data {
            Some((elem, sig)) => (Some(elem), Some(sig)),
            None => (None, None),
        };

        let result = classifier.classify_change_detailed(
            source_element.as_ref(),
            target_element.as_ref(),
            None,
            None,
            source_signature.as_ref(),
            target_signature.as_ref(),
        )?;

        println!("🔍 Scenario: {}", scenario_name);
        println!("  • Change Type: {:?}", result.change_type);
        println!("  • Impact Level: {:?}", result.impact.impact_level);
        println!("  • Risk Level: {:?}", result.impact.risk_level);
        println!("  • Implementation Effort: {:?}", result.impact.implementation_effort);
        println!("  • Breaking Change: {}", result.impact.is_breaking_change);
        
        // Risk assessment
        let risk_description = match result.impact.risk_level {
            RiskLevel::VeryLow => "Minimal risk - safe to implement",
            RiskLevel::Low => "Low risk - standard review process",
            RiskLevel::Medium => "Medium risk - careful testing required",
            RiskLevel::High => "High risk - extensive testing and review needed",
            RiskLevel::VeryHigh => "Very high risk - consider alternative approaches",
        };
        println!("  • Risk Assessment: {}", risk_description);
        
        // Effort estimation
        let effort_description = match result.impact.implementation_effort {
            EffortLevel::Trivial => "< 1 hour",
            EffortLevel::Low => "1-4 hours",
            EffortLevel::Medium => "1-2 days",
            EffortLevel::High => "3-5 days",
            EffortLevel::VeryHigh => "> 1 week",
        };
        println!("  • Estimated Effort: {}", effort_description);
        println!();
    }

    Ok(())
}

/// Demo 4: Configuration and customization
fn demo_configuration_customization() -> Result<()> {
    println!("⚙️  Demo 4: Configuration and Customization");
    println!("-------------------------------------------");

    // Test different configurations
    let configs = vec![
        ("Conservative", ChangeClassificationConfig {
            modification_threshold: 0.9,
            rename_threshold: 0.95,
            move_threshold: 0.98,
            enable_ast_analysis: true,
            enable_semantic_analysis: true,
            enable_confidence_scoring: true,
            max_ast_depth: 30,
            enable_impact_analysis: true,
        }),
        ("Balanced", ChangeClassificationConfig::default()),
        ("Aggressive", ChangeClassificationConfig {
            modification_threshold: 0.5,
            rename_threshold: 0.6,
            move_threshold: 0.7,
            enable_ast_analysis: true,
            enable_semantic_analysis: true,
            enable_confidence_scoring: true,
            max_ast_depth: 15,
            enable_impact_analysis: true,
        }),
        ("Performance Optimized", ChangeClassificationConfig {
            modification_threshold: 0.7,
            rename_threshold: 0.8,
            move_threshold: 0.9,
            enable_ast_analysis: false,
            enable_semantic_analysis: false,
            enable_confidence_scoring: false,
            max_ast_depth: 10,
            enable_impact_analysis: false,
        }),
    ];

    let source = create_code_element("calculateValue", "Math.java", 25);
    let target = create_code_element("computeResult", "Math.java", 25);

    for (config_name, config) in configs {
        let classifier = ChangeClassifier::with_config(Language::Java, config);
        let change_type = classifier.classify_change(Some(&source), Some(&target));
        
        println!("🔧 {} Configuration:", config_name);
        println!("  • Change Type: {:?}", change_type);
        println!("  • Modification Threshold: {:.2}", classifier.get_config().modification_threshold);
        println!("  • Rename Threshold: {:.2}", classifier.get_config().rename_threshold);
        println!("  • AST Analysis: {}", classifier.get_config().enable_ast_analysis);
        println!("  • Semantic Analysis: {}", classifier.get_config().enable_semantic_analysis);
        println!();
    }

    Ok(())
}

/// Demo 5: Real-world change scenarios
fn demo_real_world_scenarios() -> Result<()> {
    println!("🌍 Demo 5: Real-World Change Scenarios");
    println!("--------------------------------------");

    let classifier = ChangeClassifier::new(Language::Java);

    // Scenario 1: Refactoring - Extract Method
    println!("🔍 Scenario 1: Extract Method Refactoring");
    let original = create_code_element("processData", "DataProcessor.java", 50);
    let extracted = create_code_element("validateInput", "DataProcessor.java", 120);
    let original_sig = create_function_signature("processData", 25, Visibility::Public);
    let extracted_sig = create_function_signature("validateInput", 8, Visibility::Private);

    let result = classifier.classify_change_detailed(
        None, Some(&extracted), None, None, None, Some(&extracted_sig)
    )?;
    
    println!("  • Change: {:?} (confidence: {:.3})", result.change_type, result.confidence);
    println!("  • Impact: {:?}", result.impact.impact_level);
    println!("  • Description: {}", result.analysis.description);

    // Scenario 2: API Evolution - Parameter Addition
    println!("\n🔍 Scenario 2: API Evolution - Parameter Addition");
    let api_v1 = create_code_element("authenticate", "AuthService.java", 30);
    let api_v2 = create_code_element("authenticate", "AuthService.java", 30);
    let sig_v1 = create_function_signature_with_params("authenticate", 5, Visibility::Public, 2);
    let sig_v2 = create_function_signature_with_params("authenticate", 6, Visibility::Public, 3);

    let result = classifier.classify_change_detailed(
        Some(&api_v1), Some(&api_v2), None, None, Some(&sig_v1), Some(&sig_v2)
    )?;
    
    println!("  • Change: {:?} (confidence: {:.3})", result.change_type, result.confidence);
    println!("  • Breaking Change: {}", result.impact.is_breaking_change);
    println!("  • Risk Level: {:?}", result.impact.risk_level);

    // Scenario 3: Code Reorganization - Cross-file Move
    println!("\n🔍 Scenario 3: Code Reorganization - Cross-file Move");
    let old_location = create_code_element("utility", "Utils.java", 100);
    let new_location = create_code_element("utility", "helpers/StringUtils.java", 25);
    let utility_sig = create_function_signature("utility", 3, Visibility::Public);

    let result = classifier.classify_change_detailed(
        Some(&old_location), Some(&new_location), None, None, Some(&utility_sig), Some(&utility_sig)
    )?;
    
    println!("  • Change: {:?} (confidence: {:.3})", result.change_type, result.confidence);
    println!("  • Impact: {:?}", result.impact.impact_level);
    println!("  • Effort: {:?}", result.impact.implementation_effort);

    println!();
    Ok(())
}

/// Demo 6: Advanced features and integration
fn demo_advanced_features() -> Result<()> {
    println!("🚀 Demo 6: Advanced Features and Integration");
    println!("--------------------------------------------");

    let mut classifier = ChangeClassifier::new(Language::Java);

    // Test semantic analysis toggle
    println!("🔍 Semantic Analysis Integration:");
    println!("  • Initially enabled: {}", classifier.similarity_scorer.is_some());
    
    classifier.set_semantic_analysis(false);
    println!("  • After disabling: {}", classifier.similarity_scorer.is_some());
    
    classifier.set_semantic_analysis(true);
    println!("  • After re-enabling: {}", classifier.similarity_scorer.is_some());

    // Test configuration updates
    println!("\n🔧 Dynamic Configuration Updates:");
    let original_threshold = classifier.get_config().modification_threshold;
    println!("  • Original modification threshold: {:.2}", original_threshold);
    
    let mut new_config = classifier.get_config().clone();
    new_config.modification_threshold = 0.85;
    classifier.set_config(new_config);
    
    println!("  • Updated modification threshold: {:.2}", classifier.get_config().modification_threshold);

    // Test evidence strength analysis
    println!("\n📊 Evidence Strength Analysis:");
    let source = create_code_element("oldMethod", "Service.java", 40);
    let target = create_code_element("newMethod", "Service.java", 40);
    let source_sig = create_function_signature("oldMethod", 10, Visibility::Public);
    let target_sig = create_function_signature("newMethod", 12, Visibility::Public);

    let result = classifier.classify_change_detailed(
        Some(&source), Some(&target), None, None, Some(&source_sig), Some(&target_sig)
    )?;

    println!("  • Total evidence pieces: {}", result.analysis.evidence.len());
    for evidence in &result.analysis.evidence {
        let strength_level = match evidence.strength {
            s if s >= 0.8 => "Strong",
            s if s >= 0.6 => "Moderate",
            s if s >= 0.4 => "Weak",
            _ => "Very Weak",
        };
        println!("    - {:?}: {} ({:.3} - {})", 
            evidence.evidence_type, evidence.description, evidence.strength, strength_level);
    }

    println!();
    Ok(())
}

// Helper functions for creating test data

fn create_code_element(name: &str, file_path: &str, start_line: usize) -> CodeElement {
    CodeElement {
        name: name.to_string(),
        file_path: file_path.to_string(),
        start_line,
        end_line: start_line + 10,
        element_type: "function".to_string(),
    }
}

fn create_function_signature(name: &str, complexity: u32, visibility: Visibility) -> EnhancedFunctionSignature {
    EnhancedFunctionSignature {
        name: name.to_string(),
        parameters: Vec::new(),
        return_type: TypeSignature::Simple("void".to_string()),
        visibility,
        function_type: FunctionType::Regular,
        is_async: false,
        is_static: false,
        is_abstract: false,
        generic_parameters: Vec::new(),
        throws: Vec::new(),
        annotations: Vec::new(),
        complexity_metrics: ComplexityMetrics {
            cyclomatic_complexity: complexity,
            cognitive_complexity: complexity,
            nesting_depth: 2,
            parameter_count: 0,
            return_points: 1,
            lines_of_code: complexity * 2,
        },
        dependencies: Vec::new(),
    }
}

fn create_function_signature_with_params(
    name: &str, 
    complexity: u32, 
    visibility: Visibility, 
    param_count: usize
) -> EnhancedFunctionSignature {
    let mut signature = create_function_signature(name, complexity, visibility);
    
    // Add dummy parameters
    for i in 0..param_count {
        signature.parameters.push(ParameterInfo {
            name: format!("param{}", i),
            param_type: TypeSignature::Simple("String".to_string()),
            default_value: None,
            is_varargs: false,
            annotations: Vec::new(),
        });
    }
    
    signature.complexity_metrics.parameter_count = param_count as u32;
    signature
}
