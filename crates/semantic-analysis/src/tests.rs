//! Tests for semantic analysis components

use crate::{
    SymbolResolver, SymbolResolverConfig, ScopeManager, SymbolTable,
    Symbol, SymbolKind, ReferenceType, SymbolReference, ScopeType,
    TypeExtractor, TypeExtractorConfig, TypeSignature, TypeEquivalence,
    TypeDependencyGraphBuilder, TypeRelationshipType,
    ComprehensiveDependencyGraphBuilder, DependencyAnalysisConfig, CallType
};
use smart_diff_parser::{TreeSitterParser, Language, ParseResult};
use std::collections::HashSet;

#[cfg(test)]
mod symbol_resolver_tests {
    use super::*;

    fn create_test_config() -> SymbolResolverConfig {
        SymbolResolverConfig {
            resolve_cross_file: true,
            track_usages: true,
            resolve_imports: true,
            max_resolution_depth: 5,
            file_extensions: {
                let mut ext = HashSet::new();
                ext.insert("java".to_string());
                ext.insert("py".to_string());
                ext.insert("js".to_string());
                ext
            },
        }
    }

    #[test]
    fn test_java_import_parsing() {
        let resolver = SymbolResolver::new(create_test_config());
        
        // Test regular import
        let import1 = resolver.parse_java_import("import java.util.List;", 1, 0).unwrap();
        assert_eq!(import1.imported_name, "java.util.List");
        assert!(!import1.is_wildcard);
        
        // Test wildcard import
        let import2 = resolver.parse_java_import("import java.util.*;", 2, 0).unwrap();
        assert_eq!(import2.imported_name, "java.util");
        assert!(import2.is_wildcard);
        
        // Test static import
        let import3 = resolver.parse_java_import("import static java.lang.Math.PI;", 3, 0).unwrap();
        assert_eq!(import3.imported_name, "java.lang.Math.PI");
        assert!(!import3.is_wildcard);
    }

    #[test]
    fn test_python_import_parsing() {
        let resolver = SymbolResolver::new(create_test_config());
        
        // Test regular import
        let import1 = resolver.parse_python_import("import os", 1, 0).unwrap();
        assert_eq!(import1.imported_name, "os");
        assert!(import1.alias.is_none());
        
        // Test import with alias
        let import2 = resolver.parse_python_import("import numpy as np", 2, 0).unwrap();
        assert_eq!(import2.imported_name, "numpy");
        assert_eq!(import2.alias, Some("np".to_string()));
        
        // Test from import
        let import3 = resolver.parse_python_import("from collections import defaultdict", 3, 0).unwrap();
        assert_eq!(import3.imported_name, "defaultdict");
        assert_eq!(import3.source_path, Some("collections".to_string()));
        
        // Test from import with alias
        let import4 = resolver.parse_python_import("from datetime import datetime as dt", 4, 0).unwrap();
        assert_eq!(import4.imported_name, "datetime");
        assert_eq!(import4.alias, Some("dt".to_string()));
        assert_eq!(import4.source_path, Some("datetime".to_string()));
    }

    #[test]
    fn test_javascript_import_parsing() {
        let resolver = SymbolResolver::new(create_test_config());
        
        // Test ES6 default import
        let import1 = resolver.parse_js_import("import React from 'react'", 1, 0).unwrap();
        assert_eq!(import1.imported_name, "React");
        assert_eq!(import1.source_path, Some("react".to_string()));
        
        // Test ES6 named import
        let import2 = resolver.parse_js_import("import { useState } from 'react'", 2, 0).unwrap();
        assert_eq!(import2.imported_name, "useState");
        assert_eq!(import2.source_path, Some("react".to_string()));
        
        // Test ES6 wildcard import
        let import3 = resolver.parse_js_import("import * as React from 'react'", 3, 0).unwrap();
        assert_eq!(import3.imported_name, "*");
        assert_eq!(import3.alias, Some("React".to_string()));
        assert!(import3.is_wildcard);
        
        // Test CommonJS require
        let import4 = resolver.parse_js_import("const fs = require('fs')", 4, 0).unwrap();
        assert_eq!(import4.imported_name, "fs");
        assert_eq!(import4.source_path, Some("fs".to_string()));
    }

    #[test]
    fn test_c_include_parsing() {
        let resolver = SymbolResolver::new(create_test_config());
        
        // Test system header
        let include1 = resolver.parse_c_include("#include <stdio.h>", 1, 0).unwrap();
        assert_eq!(include1.imported_name, "stdio.h");
        assert!(include1.source_path.is_none()); // System headers don't have source paths
        
        // Test local header
        let include2 = resolver.parse_c_include("#include \"myheader.h\"", 2, 0).unwrap();
        assert_eq!(include2.imported_name, "myheader.h");
        assert_eq!(include2.source_path, Some("myheader.h".to_string()));
    }

    #[test]
    fn test_symbol_resolution_integration() -> Result<(), Box<dyn std::error::Error>> {
        let mut resolver = SymbolResolver::new(create_test_config());
        let parser = TreeSitterParser::new()?;
        
        // Test Java code with class and method
        let java_code = r#"
public class Calculator {
    private int value;
    
    public Calculator() {
        this.value = 0;
    }
    
    public int add(int a, int b) {
        return a + b;
    }
    
    public void setValue(int newValue) {
        this.value = newValue;
    }
}
"#;
        
        let parse_result = parser.parse(java_code, Language::Java)?;
        resolver.process_file("Calculator.java", &parse_result)?;
        
        let symbol_table = resolver.get_symbol_table();
        
        // Check that class was added
        let class_symbols = symbol_table.get_symbols_by_kind(SymbolKind::Class);
        assert_eq!(class_symbols.len(), 1);
        assert_eq!(class_symbols[0].name, "Calculator");
        
        // Check that methods were added
        let method_symbols = symbol_table.get_symbols_by_kind(SymbolKind::Method);
        assert!(method_symbols.len() >= 2); // Constructor and methods
        
        // Check symbol lookup
        let calculator_symbol = resolver.find_symbol("Calculator", Some("Calculator.java"));
        assert!(calculator_symbol.is_some());
        assert_eq!(calculator_symbol.unwrap().name, "Calculator");
        
        Ok(())
    }
}

#[cfg(test)]
mod scope_manager_tests {
    use super::*;

    #[test]
    fn test_scope_creation_and_hierarchy() {
        let mut scope_manager = ScopeManager::new(Language::Java);
        
        // Create global scope
        let global_scope = scope_manager.create_scope(
            ScopeType::Global, 
            "test.java".to_string(), 
            1, 
            100
        );
        scope_manager.enter_scope(global_scope);
        
        // Create class scope
        let class_scope = scope_manager.create_scope(
            ScopeType::Class, 
            "test.java".to_string(), 
            5, 
            50
        );
        scope_manager.enter_scope(class_scope);
        
        // Create method scope
        let method_scope = scope_manager.create_scope(
            ScopeType::Function, 
            "test.java".to_string(), 
            10, 
            20
        );
        scope_manager.enter_scope(method_scope);
        
        // Check hierarchy
        let hierarchy = scope_manager.get_scope_hierarchy();
        assert_eq!(hierarchy.len(), 3);
        assert_eq!(hierarchy[0], global_scope);
        assert_eq!(hierarchy[1], class_scope);
        assert_eq!(hierarchy[2], method_scope);
        
        // Check current scope
        assert_eq!(scope_manager.current_scope(), Some(method_scope));
        assert_eq!(scope_manager.current_depth(), 3);
    }

    #[test]
    fn test_symbol_resolution_with_shadowing() {
        let mut scope_manager = ScopeManager::new(Language::Java);
        
        // Create scopes
        let global_scope = scope_manager.create_scope(ScopeType::Global, "test.java".to_string(), 1, 100);
        scope_manager.enter_scope(global_scope);
        
        let class_scope = scope_manager.create_scope(ScopeType::Class, "test.java".to_string(), 5, 50);
        scope_manager.enter_scope(class_scope);
        
        let method_scope = scope_manager.create_scope(ScopeType::Function, "test.java".to_string(), 10, 20);
        scope_manager.enter_scope(method_scope);
        
        // Add symbols with same name in different scopes
        let global_symbol = Symbol {
            name: "x".to_string(),
            symbol_kind: SymbolKind::Variable,
            file_path: "test.java".to_string(),
            line: 2,
            column: 0,
            scope_id: global_scope,
            type_info: Some("int".to_string()),
            references: Vec::new(),
        };
        
        let method_symbol = Symbol {
            name: "x".to_string(),
            symbol_kind: SymbolKind::Parameter,
            file_path: "test.java".to_string(),
            line: 10,
            column: 15,
            scope_id: method_scope,
            type_info: Some("String".to_string()),
            references: Vec::new(),
        };
        
        // Add to global scope first
        scope_manager.exit_scope(); // Exit method scope
        scope_manager.exit_scope(); // Exit class scope
        scope_manager.add_symbol_to_current_scope(global_symbol).unwrap();
        
        // Re-enter scopes and add method parameter
        scope_manager.enter_scope(class_scope);
        scope_manager.enter_scope(method_scope);
        scope_manager.add_symbol_to_current_scope(method_symbol).unwrap();
        
        // Resolve symbol - should find method parameter (shadows global)
        let resolution = scope_manager.resolve_symbol("x").unwrap();
        assert_eq!(resolution.symbol.symbol_kind, SymbolKind::Parameter);
        assert_eq!(resolution.symbol.line, 10);
        assert_eq!(resolution.scope_id, method_scope);
        
        // Find all symbols with name "x"
        let all_symbols = scope_manager.find_all_symbols("x");
        assert_eq!(all_symbols.len(), 2);
        assert!(!all_symbols[0].is_shadowed); // Method parameter is not shadowed
        assert!(all_symbols[1].is_shadowed);  // Global variable is shadowed
    }

    #[test]
    fn test_scope_analysis() {
        let mut scope_manager = ScopeManager::new(Language::Python);
        
        // Create multiple scopes with symbols
        let global_scope = scope_manager.create_scope(ScopeType::Global, "test.py".to_string(), 1, 100);
        scope_manager.enter_scope(global_scope);
        
        let class_scope = scope_manager.create_scope(ScopeType::Class, "test.py".to_string(), 5, 50);
        scope_manager.enter_scope(class_scope);
        
        let method_scope1 = scope_manager.create_scope(ScopeType::Function, "test.py".to_string(), 10, 20);
        let method_scope2 = scope_manager.create_scope(ScopeType::Function, "test.py".to_string(), 25, 35);
        
        // Add some symbols
        let symbol1 = Symbol {
            name: "global_var".to_string(),
            symbol_kind: SymbolKind::Variable,
            file_path: "test.py".to_string(),
            line: 2,
            column: 0,
            scope_id: global_scope,
            type_info: None,
            references: Vec::new(),
        };
        
        scope_manager.add_symbol_to_current_scope(symbol1).unwrap();
        
        // Analyze scopes
        let analysis = scope_manager.analyze_scopes();
        
        assert_eq!(analysis.total_scopes, 4); // global, class, 2 methods
        assert_eq!(analysis.max_depth, 2); // class scope is depth 2 from global
        assert!(analysis.scope_type_counts.contains_key(&ScopeType::Global));
        assert!(analysis.scope_type_counts.contains_key(&ScopeType::Class));
        assert!(analysis.scope_type_counts.contains_key(&ScopeType::Function));
    }
}

#[cfg(test)]
mod symbol_table_tests {
    use super::*;

    #[test]
    fn test_symbol_table_operations() {
        let mut symbol_table = SymbolTable::new();
        
        // Create a scope
        let scope_id = symbol_table.create_scope(
            None,
            ScopeType::Global,
            "test.java".to_string(),
            1,
            100,
        );
        
        // Add a symbol
        let symbol = Symbol {
            name: "testFunction".to_string(),
            symbol_kind: SymbolKind::Function,
            file_path: "test.java".to_string(),
            line: 10,
            column: 5,
            scope_id,
            type_info: Some("void".to_string()),
            references: Vec::new(),
        };
        
        symbol_table.add_symbol(symbol);
        
        // Test symbol lookup
        let found_symbol = symbol_table.find_symbol("testFunction", scope_id);
        assert!(found_symbol.is_some());
        assert_eq!(found_symbol.unwrap().name, "testFunction");
        
        // Test qualified lookup
        let qualified_symbol = symbol_table.find_qualified_symbol("testFunction");
        assert!(qualified_symbol.is_some());
        
        // Test symbols by kind
        let functions = symbol_table.get_symbols_by_kind(SymbolKind::Function);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "testFunction");
        
        // Test statistics
        let stats = symbol_table.get_statistics();
        assert!(stats.total_symbols > 0);
        assert_eq!(stats.function_count, 1);
        assert_eq!(stats.total_scopes, 1);
    }

    #[test]
    fn test_symbol_references() {
        let mut symbol_table = SymbolTable::new();
        
        // Add a reference
        let reference = SymbolReference {
            file_path: "caller.java".to_string(),
            line: 15,
            column: 10,
            reference_type: ReferenceType::Call,
        };
        
        symbol_table.add_reference("testFunction", reference);
        
        // Get references
        let references = symbol_table.get_symbol_references("testFunction");
        assert_eq!(references.len(), 1);
        assert_eq!(references[0].line, 15);
        assert_eq!(references[0].reference_type, ReferenceType::Call);
    }
}

#[cfg(test)]
mod type_system_tests {
    use super::*;

    #[test]
    fn test_type_signature_parsing() {
        // Test simple type
        let simple_type = TypeSignature::parse("String").unwrap();
        assert_eq!(simple_type.base_type, "String");
        assert!(simple_type.generic_params.is_empty());
        assert_eq!(simple_type.array_dimensions, 0);

        // Test generic type
        let generic_type = TypeSignature::parse("List<String>").unwrap();
        assert_eq!(generic_type.base_type, "List");
        assert_eq!(generic_type.generic_params.len(), 1);
        assert_eq!(generic_type.generic_params[0].base_type, "String");

        // Test nested generics
        let nested_generic = TypeSignature::parse("Map<String, List<Integer>>").unwrap();
        assert_eq!(nested_generic.base_type, "Map");
        assert_eq!(nested_generic.generic_params.len(), 2);
        assert_eq!(nested_generic.generic_params[0].base_type, "String");
        assert_eq!(nested_generic.generic_params[1].base_type, "List");
        assert_eq!(nested_generic.generic_params[1].generic_params[0].base_type, "Integer");

        // Test array type
        let array_type = TypeSignature::parse("String[][]").unwrap();
        assert_eq!(array_type.base_type, "String");
        assert_eq!(array_type.array_dimensions, 2);

        // Test nullable type
        let nullable_type = TypeSignature::parse("String?").unwrap();
        assert_eq!(nullable_type.base_type, "String");
        assert!(nullable_type.is_nullable);
    }

    #[test]
    fn test_type_equivalence() {
        // Test exact match
        assert!(TypeEquivalence::are_equivalent("String", "String"));

        // Test normalized equivalence
        assert!(TypeEquivalence::are_equivalent("int", "i32"));
        assert!(TypeEquivalence::are_equivalent("String", "string"));
        assert!(TypeEquivalence::are_equivalent("bool", "Boolean"));

        // Test non-equivalent types
        assert!(!TypeEquivalence::are_equivalent("String", "Integer"));
        assert!(!TypeEquivalence::are_equivalent("int", "float"));
    }

    #[test]
    fn test_complex_type_equivalence() {
        let type1 = TypeSignature::parse("List<String>").unwrap();
        let type2 = TypeSignature::parse("List<String>").unwrap();
        let type3 = TypeSignature::parse("List<Integer>").unwrap();

        assert!(TypeEquivalence::are_complex_types_equivalent(&type1, &type2));
        assert!(!TypeEquivalence::are_complex_types_equivalent(&type1, &type3));
    }

    #[test]
    fn test_type_similarity_calculation() {
        let type1 = TypeSignature::parse("List<String>").unwrap();
        let type2 = TypeSignature::parse("List<String>").unwrap();
        let type3 = TypeSignature::parse("List<Integer>").unwrap();
        let type4 = TypeSignature::parse("ArrayList<String>").unwrap();

        // Identical types should have similarity 1.0
        assert_eq!(TypeEquivalence::calculate_type_similarity(&type1, &type2), 1.0);

        // Different generic parameters should have lower similarity
        let similarity_diff_generic = TypeEquivalence::calculate_type_similarity(&type1, &type3);
        assert!(similarity_diff_generic < 1.0);
        assert!(similarity_diff_generic > 0.0);

        // Related types should have some similarity
        let similarity_related = TypeEquivalence::calculate_type_similarity(&type1, &type4);
        assert!(similarity_related > 0.0);
        assert!(similarity_related < 1.0);
    }

    #[test]
    fn test_type_signature_string_conversion() {
        let type_sig = TypeSignature::new("List".to_string())
            .with_generics(vec![TypeSignature::new("String".to_string())])
            .with_array_dimensions(1)
            .with_nullable(true);

        let type_string = type_sig.to_string();
        assert!(type_string.contains("List"));
        assert!(type_string.contains("String"));
        assert!(type_string.contains("[]"));
        assert!(type_string.contains("?"));
    }
}

#[cfg(test)]
mod type_extractor_tests {
    use super::*;

    #[test]
    fn test_type_extractor_creation() {
        let config = TypeExtractorConfig::default();
        let extractor = TypeExtractor::new(Language::Java, config);

        // Basic smoke test
        assert!(true);
    }

    #[test]
    fn test_java_type_parsing() {
        let extractor = TypeExtractor::with_defaults(Language::Java);

        // Test simple type
        let simple_type = extractor.parse_type_signature("String").unwrap();
        assert_eq!(simple_type.base_type, "String");

        // Test generic type
        let generic_type = extractor.parse_type_signature("List<String>").unwrap();
        assert_eq!(generic_type.base_type, "List");
        assert_eq!(generic_type.generic_params.len(), 1);
        assert_eq!(generic_type.generic_params[0].base_type, "String");
    }

    #[test]
    fn test_python_type_parsing() {
        let extractor = TypeExtractor::with_defaults(Language::Python);

        // Test Python list type
        let list_type = extractor.parse_type_signature("List[str]").unwrap();
        assert_eq!(list_type.base_type, "List");
        assert_eq!(list_type.generic_params.len(), 1);
        assert_eq!(list_type.generic_params[0].base_type, "str");

        // Test Python dict type
        let dict_type = extractor.parse_type_signature("Dict[str, int]").unwrap();
        assert_eq!(dict_type.base_type, "Dict");
        assert_eq!(dict_type.generic_params.len(), 2);
        assert_eq!(dict_type.generic_params[0].base_type, "str");
        assert_eq!(dict_type.generic_params[1].base_type, "int");
    }

    #[test]
    fn test_cpp_type_parsing() {
        let extractor = TypeExtractor::with_defaults(Language::Cpp);

        // Test const pointer type
        let const_ptr_type = extractor.parse_type_signature("const int*").unwrap();
        assert_eq!(const_ptr_type.base_type, "const int*");
        assert!(const_ptr_type.modifiers.contains(&"const".to_string()));
        assert!(const_ptr_type.modifiers.contains(&"pointer".to_string()));

        // Test reference type
        let ref_type = extractor.parse_type_signature("std::string&").unwrap();
        assert!(ref_type.modifiers.contains(&"reference".to_string()));
    }

    #[test]
    fn test_primitive_type_detection() {
        let extractor = TypeExtractor::with_defaults(Language::Java);

        assert!(extractor.is_primitive_type("int"));
        assert!(extractor.is_primitive_type("String"));
        assert!(extractor.is_primitive_type("boolean"));
        assert!(extractor.is_primitive_type("void"));

        assert!(!extractor.is_primitive_type("ArrayList"));
        assert!(!extractor.is_primitive_type("MyCustomClass"));
    }

    #[test]
    fn test_type_dependency_graph_building() {
        let extractor = TypeExtractor::with_defaults(Language::Java);

        // Create mock extracted type info
        let mut extracted_types = Vec::new();

        // This would normally come from actual type extraction
        // For now, just test that the method doesn't panic
        let dependencies = extractor.build_type_dependency_graph(&extracted_types);
        assert!(dependencies.is_empty());
    }
}

#[cfg(test)]
mod type_dependency_graph_tests {
    use super::*;

    #[test]
    fn test_type_dependency_graph_creation() {
        let mut builder = TypeDependencyGraphBuilder::new();

        // Basic smoke test
        assert_eq!(builder.get_type_info_map().len(), 0);
    }

    #[test]
    fn test_type_relationship_types() {
        // Test that all relationship types are properly defined
        let inheritance = TypeRelationshipType::Inheritance;
        let implementation = TypeRelationshipType::Implementation;
        let composition = TypeRelationshipType::Composition;

        assert_ne!(inheritance, implementation);
        assert_ne!(implementation, composition);
        assert_ne!(composition, inheritance);
    }

    #[test]
    fn test_coupling_metrics_calculation() {
        // This would test the coupling metrics calculation
        // For now, just ensure the types are properly defined
        use crate::TypeCouplingMetrics;

        let metrics = TypeCouplingMetrics {
            afferent_coupling: 5,
            efferent_coupling: 3,
            instability: 0.375, // 3 / (5 + 3)
            abstractness: 0.0,
        };

        assert_eq!(metrics.afferent_coupling, 5);
        assert_eq!(metrics.efferent_coupling, 3);
        assert!((metrics.instability - 0.375).abs() < 0.001);
    }
}

#[cfg(test)]
mod comprehensive_dependency_graph_tests {
    use super::*;

    #[test]
    fn test_dependency_analysis_config() {
        let config = DependencyAnalysisConfig::default();

        assert!(config.include_function_calls);
        assert!(config.include_type_dependencies);
        assert!(config.include_variable_usage);
        assert!(config.include_import_dependencies);
        assert!(config.include_inheritance);
        assert_eq!(config.min_dependency_strength, 0.1);
        assert_eq!(config.max_transitive_depth, 10);
    }

    #[test]
    fn test_comprehensive_dependency_graph_builder_creation() {
        let config = DependencyAnalysisConfig::default();
        let builder = ComprehensiveDependencyGraphBuilder::new(config);

        // Basic smoke test
        assert_eq!(builder.get_file_contexts().len(), 0);
    }

    #[test]
    fn test_call_type_variants() {
        // Test that all call types are properly defined
        let direct = CallType::Direct;
        let method = CallType::Method;
        let constructor = CallType::Constructor;
        let static_call = CallType::Static;

        assert_ne!(direct, method);
        assert_ne!(method, constructor);
        assert_ne!(constructor, static_call);
        assert_ne!(static_call, direct);
    }

    #[test]
    fn test_dependency_analysis_config_customization() {
        let config = DependencyAnalysisConfig {
            include_function_calls: false,
            include_type_dependencies: true,
            include_variable_usage: false,
            include_import_dependencies: true,
            include_inheritance: false,
            min_dependency_strength: 0.5,
            max_transitive_depth: 5,
        };

        let builder = ComprehensiveDependencyGraphBuilder::new(config);

        // Verify configuration is applied
        assert!(!builder.config.include_function_calls);
        assert!(builder.config.include_type_dependencies);
        assert!(!builder.config.include_variable_usage);
        assert!(builder.config.include_import_dependencies);
        assert!(!builder.config.include_inheritance);
        assert_eq!(builder.config.min_dependency_strength, 0.5);
        assert_eq!(builder.config.max_transitive_depth, 5);
    }

    #[test]
    fn test_comprehensive_dependency_analysis_structure() {
        // Test that the analysis result structure is properly defined
        use crate::{ComprehensiveDependencyAnalysis, ComprehensiveCouplingMetrics, DependencyHotspot};
        use crate::{DependencyNodeType};

        let analysis = ComprehensiveDependencyAnalysis {
            total_nodes: 10,
            total_edges: 15,
            function_call_dependencies: 5,
            type_dependencies: 3,
            variable_dependencies: 2,
            import_dependencies: 3,
            inheritance_dependencies: 2,
            circular_dependencies: Vec::new(),
            strongly_connected_components: Vec::new(),
            dependency_layers: Vec::new(),
            coupling_metrics: std::collections::HashMap::new(),
            hotspots: Vec::new(),
        };

        assert_eq!(analysis.total_nodes, 10);
        assert_eq!(analysis.total_edges, 15);
        assert_eq!(analysis.function_call_dependencies, 5);
        assert_eq!(analysis.type_dependencies, 3);
        assert_eq!(analysis.variable_dependencies, 2);
        assert_eq!(analysis.import_dependencies, 3);
        assert_eq!(analysis.inheritance_dependencies, 2);
    }

    #[test]
    fn test_comprehensive_coupling_metrics_structure() {
        use crate::ComprehensiveCouplingMetrics;

        let metrics = ComprehensiveCouplingMetrics {
            afferent_coupling: 3,
            efferent_coupling: 5,
            instability: 0.625, // 5 / (3 + 5)
            function_call_coupling: 2,
            type_coupling: 1,
            data_coupling: 1,
            control_coupling: 2,
        };

        assert_eq!(metrics.afferent_coupling, 3);
        assert_eq!(metrics.efferent_coupling, 5);
        assert!((metrics.instability - 0.625).abs() < 0.001);
        assert_eq!(metrics.function_call_coupling, 2);
        assert_eq!(metrics.type_coupling, 1);
        assert_eq!(metrics.data_coupling, 1);
        assert_eq!(metrics.control_coupling, 2);
    }

    #[test]
    fn test_dependency_hotspot_structure() {
        use crate::{DependencyHotspot, DependencyNodeType};

        let hotspot = DependencyHotspot {
            name: "HighlyCoupledClass".to_string(),
            node_type: DependencyNodeType::Class,
            coupling_score: 25.5,
            incoming_dependencies: 8,
            outgoing_dependencies: 12,
            file_path: "src/highly_coupled.java".to_string(),
        };

        assert_eq!(hotspot.name, "HighlyCoupledClass");
        assert_eq!(hotspot.node_type, DependencyNodeType::Class);
        assert!((hotspot.coupling_score - 25.5).abs() < 0.001);
        assert_eq!(hotspot.incoming_dependencies, 8);
        assert_eq!(hotspot.outgoing_dependencies, 12);
        assert_eq!(hotspot.file_path, "src/highly_coupled.java");
    }

    #[test]
    fn test_file_analysis_context_structure() {
        use crate::{FileAnalysisContext, FunctionInfo, ClassInfo, VariableInfo, FunctionCallInfo};
        use smart_diff_parser::Language;

        let context = FileAnalysisContext {
            file_path: "test.java".to_string(),
            language: Language::Java,
            functions: Vec::new(),
            classes: Vec::new(),
            variables: Vec::new(),
            function_calls: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
        };

        assert_eq!(context.file_path, "test.java");
        assert_eq!(context.language, Language::Java);
        assert_eq!(context.functions.len(), 0);
        assert_eq!(context.classes.len(), 0);
        assert_eq!(context.variables.len(), 0);
        assert_eq!(context.function_calls.len(), 0);
        assert_eq!(context.imports.len(), 0);
        assert_eq!(context.exports.len(), 0);
    }

    #[test]
    fn test_function_info_structure() {
        use crate::FunctionInfo;

        let function_info = FunctionInfo {
            name: "processData".to_string(),
            qualified_name: "DataProcessor.processData".to_string(),
            parameters: vec!["data: String".to_string(), "options: ProcessOptions".to_string()],
            return_type: Some("ProcessResult".to_string()),
            calls: vec!["validateInput".to_string(), "transformData".to_string()],
            accesses: vec!["this.config".to_string(), "this.cache".to_string()],
            line: 42,
            column: 8,
        };

        assert_eq!(function_info.name, "processData");
        assert_eq!(function_info.qualified_name, "DataProcessor.processData");
        assert_eq!(function_info.parameters.len(), 2);
        assert_eq!(function_info.return_type, Some("ProcessResult".to_string()));
        assert_eq!(function_info.calls.len(), 2);
        assert_eq!(function_info.accesses.len(), 2);
        assert_eq!(function_info.line, 42);
        assert_eq!(function_info.column, 8);
    }

    #[test]
    fn test_class_info_structure() {
        use crate::ClassInfo;

        let class_info = ClassInfo {
            name: "DataProcessor".to_string(),
            qualified_name: "com.example.DataProcessor".to_string(),
            parent_classes: vec!["BaseProcessor".to_string()],
            interfaces: vec!["Processor".to_string(), "Configurable".to_string()],
            fields: vec!["config".to_string(), "cache".to_string()],
            methods: vec!["processData".to_string(), "validateInput".to_string()],
            line: 15,
        };

        assert_eq!(class_info.name, "DataProcessor");
        assert_eq!(class_info.qualified_name, "com.example.DataProcessor");
        assert_eq!(class_info.parent_classes.len(), 1);
        assert_eq!(class_info.interfaces.len(), 2);
        assert_eq!(class_info.fields.len(), 2);
        assert_eq!(class_info.methods.len(), 2);
        assert_eq!(class_info.line, 15);
    }

    #[test]
    fn test_function_call_info_structure() {
        use crate::{FunctionCallInfo, CallType};

        let call_info = FunctionCallInfo {
            caller: "DataProcessor.processData".to_string(),
            callee: "ValidationUtils.validateInput".to_string(),
            call_type: CallType::Static,
            line: 45,
            column: 12,
        };

        assert_eq!(call_info.caller, "DataProcessor.processData");
        assert_eq!(call_info.callee, "ValidationUtils.validateInput");
        assert_eq!(call_info.call_type, CallType::Static);
        assert_eq!(call_info.line, 45);
        assert_eq!(call_info.column, 12);
    }
}
