//! Tests for semantic analysis components

use crate::{
    SymbolResolver, SymbolResolverConfig, ScopeManager, SymbolTable, 
    Symbol, SymbolKind, ReferenceType, SymbolReference, ScopeType
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
