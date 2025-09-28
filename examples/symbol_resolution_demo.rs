//! Demonstration of comprehensive symbol resolution capabilities

use smart_diff_parser::{Language, TreeSitterParser};
use smart_diff_semantic::{
    ReferenceType, ScopeManager, ScopeType, SymbolKind, SymbolResolver, SymbolResolverConfig,
};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Smart Code Diff - Symbol Resolution Demo");
    println!("=======================================");

    // Demo basic symbol resolution
    demo_basic_symbol_resolution()?;

    // Demo cross-file resolution
    demo_cross_file_resolution()?;

    // Demo import resolution
    demo_import_resolution()?;

    // Demo scope management
    demo_scope_management()?;

    // Demo symbol statistics
    demo_symbol_statistics()?;

    Ok(())
}

fn demo_basic_symbol_resolution() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Basic Symbol Resolution ---");

    let mut resolver = SymbolResolver::with_defaults();
    let parser = TreeSitterParser::new()?;

    let java_code = r#"
package com.example;

import java.util.List;
import java.util.ArrayList;

public class DataProcessor {
    private List<String> data;
    private int processedCount;
    
    public DataProcessor() {
        this.data = new ArrayList<>();
        this.processedCount = 0;
    }
    
    public void addData(String item) {
        data.add(item);
    }
    
    public List<String> processData() {
        List<String> result = new ArrayList<>();
        
        for (String item : data) {
            if (isValid(item)) {
                result.add(transform(item));
                processedCount++;
            }
        }
        
        return result;
    }
    
    private boolean isValid(String item) {
        return item != null && !item.isEmpty();
    }
    
    private String transform(String item) {
        return item.toUpperCase().trim();
    }
    
    public int getProcessedCount() {
        return processedCount;
    }
}
"#;

    let parse_result = parser.parse(java_code, Language::Java)?;
    resolver.process_file("DataProcessor.java", &parse_result)?;

    let symbol_table = resolver.get_symbol_table();

    // Display discovered symbols
    println!("Discovered symbols:");

    let classes = symbol_table.get_symbols_by_kind(SymbolKind::Class);
    println!("  Classes: {}", classes.len());
    for class in &classes {
        println!("    - {} at line {}", class.name, class.line);
    }

    let methods = symbol_table.get_symbols_by_kind(SymbolKind::Method);
    println!("  Methods: {}", methods.len());
    for method in &methods {
        println!(
            "    - {} at line {} ({})",
            method.name,
            method.line,
            method.type_info.as_deref().unwrap_or("void")
        );
    }

    let fields = symbol_table.get_symbols_by_kind(SymbolKind::Field);
    println!("  Fields: {}", fields.len());
    for field in &fields {
        println!(
            "    - {} at line {} ({})",
            field.name,
            field.line,
            field.type_info.as_deref().unwrap_or("unknown")
        );
    }

    // Test symbol lookup
    println!("\nSymbol lookup tests:");
    if let Some(class_symbol) = resolver.find_symbol("DataProcessor", Some("DataProcessor.java")) {
        println!(
            "  Found class: {} (kind: {:?})",
            class_symbol.name, class_symbol.symbol_kind
        );
    }

    if let Some(method_symbol) = resolver.find_symbol("processData", Some("DataProcessor.java")) {
        println!(
            "  Found method: {} (kind: {:?})",
            method_symbol.name, method_symbol.symbol_kind
        );
    }

    // Display import information
    if let Some(file_context) = resolver.get_file_context("DataProcessor.java") {
        println!("\nImports found: {}", file_context.imports.len());
        for import in &file_context.imports {
            println!("  - {} from {:?}", import.imported_name, import.source_path);
        }
    }

    Ok(())
}

fn demo_cross_file_resolution() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Cross-File Resolution ---");

    let mut config = SymbolResolverConfig::default();
    config.resolve_cross_file = true;
    let mut resolver = SymbolResolver::new(config);
    let parser = TreeSitterParser::new()?;

    // First file: Interface definition
    let interface_code = r#"
package com.example;

public interface Calculator {
    int add(int a, int b);
    int subtract(int a, int b);
    double divide(double a, double b);
}
"#;

    // Second file: Implementation
    let implementation_code = r#"
package com.example;

public class BasicCalculator implements Calculator {
    @Override
    public int add(int a, int b) {
        return a + b;
    }
    
    @Override
    public int subtract(int a, int b) {
        return a - b;
    }
    
    @Override
    public double divide(double a, double b) {
        if (b == 0) {
            throw new IllegalArgumentException("Division by zero");
        }
        return a / b;
    }
    
    public int multiply(int a, int b) {
        return a * b;
    }
}
"#;

    // Process both files
    let interface_result = parser.parse(interface_code, Language::Java)?;
    let implementation_result = parser.parse(implementation_code, Language::Java)?;

    let files = vec![
        ("Calculator.java".to_string(), interface_result),
        ("BasicCalculator.java".to_string(), implementation_result),
    ];

    resolver.process_files(files)?;

    let symbol_table = resolver.get_symbol_table();

    // Display cross-file analysis
    println!("Cross-file symbol analysis:");

    let interfaces = symbol_table.get_symbols_by_kind(SymbolKind::Interface);
    let classes = symbol_table.get_symbols_by_kind(SymbolKind::Class);

    println!("  Interfaces: {}", interfaces.len());
    for interface in &interfaces {
        println!("    - {} in {}", interface.name, interface.file_path);
    }

    println!("  Classes: {}", classes.len());
    for class in &classes {
        println!("    - {} in {}", class.name, class.file_path);
    }

    // Check import graph
    let import_graph = resolver.get_import_graph();
    println!("\nImport relationships:");
    for (file, imports) in import_graph {
        if !imports.is_empty() {
            println!("  {} imports: {:?}", file, imports);
        }
    }

    // Test qualified symbol lookup
    if let Some(calculator_symbol) = resolver.find_symbol("Calculator", None) {
        println!(
            "\nFound Calculator interface: {} ({})",
            calculator_symbol.name, calculator_symbol.file_path
        );
    }

    if let Some(basic_calc_symbol) = resolver.find_symbol("BasicCalculator", None) {
        println!(
            "Found BasicCalculator class: {} ({})",
            basic_calc_symbol.name, basic_calc_symbol.file_path
        );
    }

    Ok(())
}

fn demo_import_resolution() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Import Resolution ---");

    let resolver = SymbolResolver::with_defaults();

    // Test different import formats
    let import_examples = vec![
        ("Java", "import java.util.List;"),
        ("Java", "import java.util.*;"),
        ("Java", "import static java.lang.Math.PI;"),
        ("Python", "import os"),
        ("Python", "import numpy as np"),
        ("Python", "from collections import defaultdict"),
        ("Python", "from datetime import datetime as dt"),
        ("JavaScript", "import React from 'react'"),
        ("JavaScript", "import { useState } from 'react'"),
        ("JavaScript", "import * as React from 'react'"),
        ("JavaScript", "const fs = require('fs')"),
        ("C", "#include <stdio.h>"),
        ("C", "#include \"myheader.h\""),
    ];

    println!("Import parsing examples:");

    for (lang, import_stmt) in import_examples {
        let result = match lang {
            "Java" => resolver.parse_java_import(import_stmt, 1, 0),
            "Python" => resolver.parse_python_import(import_stmt, 1, 0),
            "JavaScript" => resolver.parse_js_import(import_stmt, 1, 0),
            "C" => resolver.parse_c_include(import_stmt, 1, 0),
            _ => continue,
        };

        match result {
            Ok(import_info) => {
                println!(
                    "  {} - {}: {} (wildcard: {}, alias: {:?})",
                    lang,
                    import_stmt,
                    import_info.imported_name,
                    import_info.is_wildcard,
                    import_info.alias
                );
            }
            Err(e) => {
                println!("  {} - {}: ERROR - {}", lang, import_stmt, e);
            }
        }
    }

    Ok(())
}

fn demo_scope_management() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Scope Management ---");

    let mut scope_manager = ScopeManager::new(Language::Java);

    // Create nested scopes
    let global_scope =
        scope_manager.create_scope(ScopeType::Global, "ScopeDemo.java".to_string(), 1, 100);
    scope_manager.enter_scope(global_scope);
    println!(
        "Created global scope (depth: {})",
        scope_manager.current_depth()
    );

    let class_scope =
        scope_manager.create_scope(ScopeType::Class, "ScopeDemo.java".to_string(), 5, 80);
    scope_manager.enter_scope(class_scope);
    println!(
        "Created class scope (depth: {})",
        scope_manager.current_depth()
    );

    let method_scope =
        scope_manager.create_scope(ScopeType::Function, "ScopeDemo.java".to_string(), 10, 30);
    scope_manager.enter_scope(method_scope);
    println!(
        "Created method scope (depth: {})",
        scope_manager.current_depth()
    );

    let block_scope =
        scope_manager.create_scope(ScopeType::Block, "ScopeDemo.java".to_string(), 15, 25);
    scope_manager.enter_scope(block_scope);
    println!(
        "Created block scope (depth: {})",
        scope_manager.current_depth()
    );

    // Display scope hierarchy
    let hierarchy = scope_manager.get_scope_hierarchy();
    println!("\nScope hierarchy: {:?}", hierarchy);

    // Test scope analysis
    let analysis = scope_manager.analyze_scopes();
    println!("\nScope analysis:");
    println!("  Total scopes: {}", analysis.total_scopes);
    println!("  Max depth: {}", analysis.max_depth);
    println!("  Scope type counts: {:?}", analysis.scope_type_counts);

    Ok(())
}

fn demo_symbol_statistics() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Symbol Statistics ---");

    let mut resolver = SymbolResolver::with_defaults();
    let parser = TreeSitterParser::new()?;

    let complex_code = r#"
public class StatisticsDemo {
    private static final String CONSTANT = "demo";
    private int instanceVar;
    
    public StatisticsDemo(int value) {
        this.instanceVar = value;
    }
    
    public static void staticMethod() {
        System.out.println("Static method");
    }
    
    public void instanceMethod(String param) {
        int localVar = 42;
        
        if (param != null) {
            String innerVar = param.toUpperCase();
            System.out.println(innerVar);
        }
        
        for (int i = 0; i < 10; i++) {
            System.out.println(i);
        }
    }
    
    private class InnerClass {
        private String innerField;
        
        public void innerMethod() {
            innerField = "inner";
        }
    }
    
    public interface InnerInterface {
        void interfaceMethod();
    }
}
"#;

    let parse_result = parser.parse(complex_code, Language::Java)?;
    resolver.process_file("StatisticsDemo.java", &parse_result)?;

    let symbol_table = resolver.get_symbol_table();
    let stats = symbol_table.get_statistics();

    println!("Symbol table statistics:");
    println!("  Total symbols: {}", stats.total_symbols);
    println!("  Total scopes: {}", stats.total_scopes);
    println!("  Total files: {}", stats.total_files);
    println!("  Total references: {}", stats.total_references);
    println!(
        "  Avg references per symbol: {:.2}",
        stats.avg_references_per_symbol
    );

    println!("\nSymbol counts by type:");
    println!("  Functions: {}", stats.function_count);
    println!("  Methods: {}", stats.method_count);
    println!("  Classes: {}", stats.class_count);
    println!("  Interfaces: {}", stats.interface_count);
    println!("  Variables: {}", stats.variable_count);
    println!("  Constants: {}", stats.constant_count);
    println!("  Parameters: {}", stats.parameter_count);
    println!("  Fields: {}", stats.field_count);
    println!("  Modules: {}", stats.module_count);
    println!("  Namespaces: {}", stats.namespace_count);

    Ok(())
}
