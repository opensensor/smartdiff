//! Comprehensive demonstration of Class Hierarchy Tracking System
//!
//! This example showcases the advanced class hierarchy tracking system that detects:
//! - Class moves between files with inheritance preservation
//! - Method migrations between classes (pull up, push down, extract)
//! - Hierarchy changes (parent changes, flattening, extraction)
//! - Interface/trait/protocol implementation changes
//!
//! This is particularly useful for class-based languages like Java, C++, PHP, Swift, and Ruby.

use smart_diff_engine::{
    ClassHierarchyAnalysisResult, ClassHierarchyTracker,
    ClassHierarchyTrackerConfig, ClassNode, MethodInfo, MethodMigrationType, Visibility,
};
use std::collections::HashMap;

fn main() {
    println!("=== Class Hierarchy Tracking Demo ===\n");

    // Create tracker with default configuration
    let tracker = ClassHierarchyTracker::new(ClassHierarchyTrackerConfig::default());

    // Scenario 1: Class Move with Inheritance Preservation
    println!("📦 Scenario 1: Class Move with Inheritance Preservation");
    println!("--------------------------------------------------------");
    demo_class_move(&tracker);
    println!();

    // Scenario 2: Method Pull Up Refactoring
    println!("⬆️  Scenario 2: Method Pull Up Refactoring");
    println!("--------------------------------------------------------");
    demo_method_pull_up(&tracker);
    println!();

    // Scenario 3: Method Push Down Refactoring
    println!("⬇️  Scenario 3: Method Push Down Refactoring");
    println!("--------------------------------------------------------");
    demo_method_push_down(&tracker);
    println!();

    // Scenario 4: Class Flattening (Remove Inheritance, Inline Methods)
    println!("🔨 Scenario 4: Class Flattening");
    println!("--------------------------------------------------------");
    demo_class_flattening(&tracker);
    println!();

    // Scenario 5: Interface/Trait Changes
    println!("🔌 Scenario 5: Interface/Trait Changes");
    println!("--------------------------------------------------------");
    demo_interface_changes(&tracker);
    println!();

    // Scenario 6: Complex Hierarchy Refactoring
    println!("🌳 Scenario 6: Complex Hierarchy Refactoring");
    println!("--------------------------------------------------------");
    demo_complex_hierarchy(&tracker);
    println!();

    println!("=== Demo Complete ===");
}

/// Demo: Class moved from one file to another, preserving inheritance
fn demo_class_move(tracker: &ClassHierarchyTracker) {
    // Source: DataProcessor in old/processors/DataProcessor.java
    let source_classes = create_simple_hierarchy(
        "DataProcessor",
        Some("BaseProcessor".to_string()),
        "old/processors/DataProcessor.java",
    );

    // Target: DataProcessor moved to new/core/DataProcessor.java
    let target_classes = create_simple_hierarchy(
        "DataProcessor",
        Some("BaseProcessor".to_string()),
        "new/core/DataProcessor.java",
    );

    let source_hierarchy = tracker.build_hierarchy(&source_classes).unwrap();
    let target_hierarchy = tracker.build_hierarchy(&target_classes).unwrap();

    let result = tracker
        .analyze_hierarchy_changes(&source_hierarchy, &target_hierarchy)
        .unwrap();

    print_class_moves(&result);
}

/// Demo: Method pulled up from child to parent class
fn demo_method_pull_up(tracker: &ClassHierarchyTracker) {
    // Source: validate() method in Child class
    let mut source_classes = HashMap::new();

    let parent_methods = vec![create_method("process", "void process()")];
    let child_methods = vec![
        create_method("process", "void process()"),
        create_method("validate", "boolean validate()"), // Will be pulled up
    ];

    source_classes.insert(
        "Parent".to_string(),
        create_class("Parent", None, parent_methods, "Parent.java"),
    );
    source_classes.insert(
        "Child".to_string(),
        create_class(
            "Child",
            Some("Parent".to_string()),
            child_methods,
            "Child.java",
        ),
    );

    // Target: validate() method moved to Parent class
    let mut target_classes = HashMap::new();

    let parent_methods = vec![
        create_method("process", "void process()"),
        create_method("validate", "boolean validate()"), // Pulled up
    ];
    let child_methods = vec![create_method("process", "void process()")];

    target_classes.insert(
        "Parent".to_string(),
        create_class("Parent", None, parent_methods, "Parent.java"),
    );
    target_classes.insert(
        "Child".to_string(),
        create_class(
            "Child",
            Some("Parent".to_string()),
            child_methods,
            "Child.java",
        ),
    );

    let source_hierarchy = tracker.build_hierarchy(&source_classes).unwrap();
    let target_hierarchy = tracker.build_hierarchy(&target_classes).unwrap();

    let result = tracker
        .analyze_hierarchy_changes(&source_hierarchy, &target_hierarchy)
        .unwrap();

    print_method_migrations(&result);
}

/// Demo: Method pushed down from parent to child class
fn demo_method_push_down(tracker: &ClassHierarchyTracker) {
    // Source: specialized() method in Parent class
    let mut source_classes = HashMap::new();

    let parent_methods = vec![
        create_method("process", "void process()"),
        create_method("specialized", "void specialized()"), // Will be pushed down
    ];
    let child_methods = vec![create_method("process", "void process()")];

    source_classes.insert(
        "Parent".to_string(),
        create_class("Parent", None, parent_methods, "Parent.java"),
    );
    source_classes.insert(
        "Child".to_string(),
        create_class(
            "Child",
            Some("Parent".to_string()),
            child_methods,
            "Child.java",
        ),
    );

    // Target: specialized() method moved to Child class
    let mut target_classes = HashMap::new();

    let parent_methods = vec![create_method("process", "void process()")];
    let child_methods = vec![
        create_method("process", "void process()"),
        create_method("specialized", "void specialized()"), // Pushed down
    ];

    target_classes.insert(
        "Parent".to_string(),
        create_class("Parent", None, parent_methods, "Parent.java"),
    );
    target_classes.insert(
        "Child".to_string(),
        create_class(
            "Child",
            Some("Parent".to_string()),
            child_methods,
            "Child.java",
        ),
    );

    let source_hierarchy = tracker.build_hierarchy(&source_classes).unwrap();
    let target_hierarchy = tracker.build_hierarchy(&target_classes).unwrap();

    let result = tracker
        .analyze_hierarchy_changes(&source_hierarchy, &target_hierarchy)
        .unwrap();

    print_method_migrations(&result);
}

/// Demo: Class flattening - inheritance removed, parent methods inlined
fn demo_class_flattening(tracker: &ClassHierarchyTracker) {
    // Source: Child extends Parent
    let mut source_classes = HashMap::new();

    let parent_methods = vec![create_method("parentMethod", "void parentMethod()")];
    let child_methods = vec![create_method("childMethod", "void childMethod()")];

    source_classes.insert(
        "Parent".to_string(),
        create_class("Parent", None, parent_methods, "Parent.java"),
    );
    source_classes.insert(
        "Child".to_string(),
        create_class(
            "Child",
            Some("Parent".to_string()),
            child_methods,
            "Child.java",
        ),
    );

    // Target: Child no longer extends Parent, parent methods inlined
    let mut target_classes = HashMap::new();

    let parent_methods = vec![create_method("parentMethod", "void parentMethod()")];
    let child_methods = vec![
        create_method("childMethod", "void childMethod()"),
        create_method("parentMethod", "void parentMethod()"), // Inlined
    ];

    target_classes.insert(
        "Parent".to_string(),
        create_class("Parent", None, parent_methods, "Parent.java"),
    );
    target_classes.insert(
        "Child".to_string(),
        create_class("Child", None, child_methods, "Child.java"), // No parent
    );

    let source_hierarchy = tracker.build_hierarchy(&source_classes).unwrap();
    let target_hierarchy = tracker.build_hierarchy(&target_classes).unwrap();

    let result = tracker
        .analyze_hierarchy_changes(&source_hierarchy, &target_hierarchy)
        .unwrap();

    print_hierarchy_changes(&result);
}

/// Demo: Interface and trait implementation changes
fn demo_interface_changes(tracker: &ClassHierarchyTracker) {
    // Source: Class implements Serializable
    let mut source_classes = HashMap::new();
    let mut source_class = create_class(
        "DataModel",
        None,
        vec![create_method("getData", "Object getData()")],
        "DataModel.java",
    );
    source_class.interfaces = vec!["Serializable".to_string()];
    source_classes.insert("DataModel".to_string(), source_class);

    // Target: Class implements Serializable, Comparable, and uses Loggable trait
    let mut target_classes = HashMap::new();
    let mut target_class = create_class(
        "DataModel",
        None,
        vec![create_method("getData", "Object getData()")],
        "DataModel.java",
    );
    target_class.interfaces = vec!["Serializable".to_string(), "Comparable".to_string()];
    target_class.traits = vec!["Loggable".to_string()];
    target_classes.insert("DataModel".to_string(), target_class);

    let source_hierarchy = tracker.build_hierarchy(&source_classes).unwrap();
    let target_hierarchy = tracker.build_hierarchy(&target_classes).unwrap();

    let result = tracker
        .analyze_hierarchy_changes(&source_hierarchy, &target_hierarchy)
        .unwrap();

    print_interface_changes(&result);
}

/// Demo: Complex hierarchy with multiple refactorings
fn demo_complex_hierarchy(tracker: &ClassHierarchyTracker) {
    println!("Creating a complex 3-level hierarchy with multiple changes...");
    println!("Source: GrandParent -> Parent -> Child");
    println!("Target: GrandParent -> Parent -> Child (with method migrations and moves)");
    println!();

    // This would be a more complex scenario - simplified for demo
    let source_classes = create_simple_hierarchy("Child", Some("Parent".to_string()), "old/Child.java");
    let target_classes = create_simple_hierarchy("Child", Some("Parent".to_string()), "new/Child.java");

    let source_hierarchy = tracker.build_hierarchy(&source_classes).unwrap();
    let target_hierarchy = tracker.build_hierarchy(&target_classes).unwrap();

    let result = tracker
        .analyze_hierarchy_changes(&source_hierarchy, &target_hierarchy)
        .unwrap();

    print_statistics(&result);
}

// Helper functions

fn create_method(name: &str, signature: &str) -> MethodInfo {
    MethodInfo {
        name: name.to_string(),
        signature: signature.to_string(),
        visibility: Visibility::Public,
        is_static: false,
        is_abstract: false,
        is_override: false,
        line: 1,
    }
}

fn create_class(
    name: &str,
    parent: Option<String>,
    methods: Vec<MethodInfo>,
    file_path: &str,
) -> ClassNode {
    ClassNode {
        qualified_name: name.to_string(),
        name: name.to_string(),
        parent,
        interfaces: Vec::new(),
        traits: Vec::new(),
        methods,
        fields: Vec::new(),
        file_path: file_path.to_string(),
        line: 1,
        is_abstract: false,
        is_interface: false,
    }
}

// Print functions

fn print_class_moves(result: &ClassHierarchyAnalysisResult) {
    if result.class_moves.is_empty() {
        println!("  No class moves detected");
        return;
    }

    for class_move in &result.class_moves {
        println!("  ✓ Class '{}' moved:", class_move.class_name);
        println!("    From: {}", class_move.source_file);
        println!("    To:   {}", class_move.target_file);
        println!(
            "    Inheritance preserved: {}",
            if class_move.inheritance_preserved {
                "✓"
            } else {
                "✗"
            }
        );
        println!(
            "    Interfaces preserved: {}",
            if class_move.interfaces_preserved {
                "✓"
            } else {
                "✗"
            }
        );
        println!("    Methods moved: {}", class_move.moved_methods.len());
        println!("    Confidence: {:.1}%", class_move.confidence * 100.0);
    }
}

fn print_method_migrations(result: &ClassHierarchyAnalysisResult) {
    if result.method_migrations.is_empty() {
        println!("  No method migrations detected");
        return;
    }

    for migration in &result.method_migrations {
        let migration_type_str = match migration.migration_type {
            MethodMigrationType::PullUp => "⬆️  Pull Up",
            MethodMigrationType::PushDown => "⬇️  Push Down",
            MethodMigrationType::MovedToSibling => "↔️  Move to Sibling",
            MethodMigrationType::ExtractedToNewClass => "📦 Extract to New Class",
            MethodMigrationType::MovedToUnrelated => "🔀 Move to Unrelated",
        };

        println!("  {} Method '{}':", migration_type_str, migration.method_name);
        println!("    From: {} ({})", migration.source_class, migration.source_file);
        println!("    To:   {} ({})", migration.target_class, migration.target_file);
        println!("    Signature: {}", migration.signature);
        println!("    Confidence: {:.1}%", migration.confidence * 100.0);
    }
}

fn print_hierarchy_changes(result: &ClassHierarchyAnalysisResult) {
    if result.hierarchy_changes.is_empty() {
        println!("  No hierarchy changes detected");
        return;
    }

    for change in &result.hierarchy_changes {
        let change_type_str = format!("{:?}", change.change_type);
        println!("  {} Class '{}':", change_type_str, change.class_name);

        if let Some(old_parent) = &change.old_parent {
            println!("    Old parent: {}", old_parent);
        }
        if let Some(new_parent) = &change.new_parent {
            println!("    New parent: {}", new_parent);
        }

        println!("    File: {}", change.file_path);
        println!("    Confidence: {:.1}%", change.confidence * 100.0);
    }
}

fn print_interface_changes(result: &ClassHierarchyAnalysisResult) {
    if result.interface_changes.is_empty() {
        println!("  No interface/trait changes detected");
        return;
    }

    for change in &result.interface_changes {
        let change_type_str = format!("{:?}", change.change_type);
        println!(
            "  {} '{}' on class '{}':",
            change_type_str, change.interface_name, change.class_name
        );
        println!("    File: {}", change.file_path);
        println!("    Confidence: {:.1}%", change.confidence * 100.0);
    }
}

fn print_statistics(result: &ClassHierarchyAnalysisResult) {
    let stats = &result.statistics;

    println!("  📊 Hierarchy Statistics:");
    println!("    Total classes: {}", stats.total_classes);
    println!(
        "    Classes with inheritance: {}",
        stats.classes_with_inheritance
    );
    println!(
        "    Classes with interfaces: {}",
        stats.classes_with_interfaces
    );
    println!("    Classes with traits: {}", stats.classes_with_traits);
    println!("    Max hierarchy depth: {}", stats.max_hierarchy_depth);
    println!(
        "    Avg methods per class: {:.1}",
        stats.avg_methods_per_class
    );
    println!();
    println!("  🔍 Detected Changes:");
    println!("    Class moves: {}", stats.total_class_moves);
    println!("    Method migrations: {}", stats.total_method_migrations);
    println!("    Hierarchy changes: {}", stats.total_hierarchy_changes);
}

fn create_simple_hierarchy(
    class_name: &str,
    parent: Option<String>,
    file_path: &str,
) -> HashMap<String, ClassNode> {
    let mut classes = HashMap::new();
    let methods = vec![
        create_method("process", "void process()"),
        create_method("validate", "boolean validate()"),
    ];
    classes.insert(
        class_name.to_string(),
        create_class(class_name, parent, methods, file_path),
    );
    classes
}

