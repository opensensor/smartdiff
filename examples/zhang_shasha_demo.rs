//! Comprehensive demonstration of Zhang-Shasha Tree Edit Distance Algorithm
//! 
//! This example showcases the optimized Zhang-Shasha algorithm implementation with
//! heuristic pruning, caching, and advanced tree comparison capabilities for
//! precise AST-level code similarity analysis.

use smart_diff_engine::{TreeEditDistance, EditCost, ZhangShashaConfig, EditOperation};
use smart_diff_parser::{ASTNode, NodeType, NodeMetadata};
use std::collections::HashMap;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🌳 Zhang-Shasha Tree Edit Distance Demo");
    println!("=======================================\n");

    // Demo 1: Basic tree edit distance calculations
    demo_basic_tree_edit_distance()?;
    
    // Demo 2: Complex AST comparisons
    demo_complex_ast_comparisons()?;
    
    // Demo 3: Edit operations and transformations
    demo_edit_operations()?;
    
    // Demo 4: Performance optimizations (caching and pruning)
    demo_performance_optimizations()?;
    
    // Demo 5: Configuration and cost customization
    demo_configuration_customization()?;
    
    // Demo 6: Real-world code comparison scenarios
    demo_real_world_scenarios()?;

    println!("\n✅ Zhang-Shasha Tree Edit Distance Demo Complete!");
    Ok(())
}

/// Demo 1: Basic tree edit distance calculations
fn demo_basic_tree_edit_distance() -> Result<()> {
    println!("📊 Demo 1: Basic Tree Edit Distance Calculations");
    println!("------------------------------------------------");

    let ted = TreeEditDistance::with_defaults();

    // Test 1: Identical trees
    let tree1 = create_simple_function_tree("calculateSum");
    let tree2 = create_simple_function_tree("calculateSum");
    
    let distance = ted.calculate_distance(&tree1, &tree2);
    let similarity = ted.calculate_similarity(&tree1, &tree2);
    
    println!("🔍 Identical Trees:");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Similarity Score: {:.3}", similarity);
    println!("  • Expected: Distance = 0.0, Similarity = 1.0");

    // Test 2: Single node difference
    let tree3 = create_simple_function_tree("calculateSum");
    let tree4 = create_simple_function_tree("calculateProduct");
    
    let distance = ted.calculate_distance(&tree3, &tree4);
    let similarity = ted.calculate_similarity(&tree3, &tree4);
    
    println!("\n🔍 Single Node Difference:");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Similarity Score: {:.3}", similarity);
    println!("  • Trees differ only in function name");

    // Test 3: Structural differences
    let tree5 = create_if_statement_tree();
    let tree6 = create_while_statement_tree();
    
    let distance = ted.calculate_distance(&tree5, &tree6);
    let similarity = ted.calculate_similarity(&tree5, &tree6);
    
    println!("\n🔍 Structural Differences (If vs While):");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Similarity Score: {:.3}", similarity);
    println!("  • Similar structure, different control flow");

    println!();
    Ok(())
}

/// Demo 2: Complex AST comparisons
fn demo_complex_ast_comparisons() -> Result<()> {
    println!("🌲 Demo 2: Complex AST Comparisons");
    println!("----------------------------------");

    let ted = TreeEditDistance::with_defaults();

    // Compare complex function structures
    let original_function = create_complex_function_tree();
    let refactored_function = create_refactored_function_tree();
    
    let distance = ted.calculate_distance(&original_function, &refactored_function);
    let similarity = ted.calculate_similarity(&original_function, &refactored_function);
    
    println!("🔍 Original vs Refactored Function:");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Similarity Score: {:.3}", similarity);
    println!("  • Nodes in original: {}", count_nodes(&original_function));
    println!("  • Nodes in refactored: {}", count_nodes(&refactored_function));
    println!("  • Tree depth original: {}", calculate_depth(&original_function));
    println!("  • Tree depth refactored: {}", calculate_depth(&refactored_function));

    // Compare class structures
    let class1 = create_class_tree("UserService", vec!["createUser", "updateUser", "deleteUser"]);
    let class2 = create_class_tree("UserService", vec!["createUser", "updateUser", "deleteUser", "findUser"]);
    
    let distance = ted.calculate_distance(&class1, &class2);
    let similarity = ted.calculate_similarity(&class1, &class2);
    
    println!("\n🔍 Class with Added Method:");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Similarity Score: {:.3}", similarity);
    println!("  • Change: Added 'findUser' method");

    println!();
    Ok(())
}

/// Demo 3: Edit operations and transformations
fn demo_edit_operations() -> Result<()> {
    println!("✏️  Demo 3: Edit Operations and Transformations");
    println!("----------------------------------------------");

    let ted = TreeEditDistance::with_defaults();

    // Test insertions
    let tree_before = create_test_node(NodeType::Function, vec![
        create_leaf_node(NodeType::Identifier),
    ]);
    
    let tree_after = create_test_node(NodeType::Function, vec![
        create_leaf_node(NodeType::Identifier),
        create_leaf_node(NodeType::Parameter),
        create_leaf_node(NodeType::Block),
    ]);
    
    let operations = ted.calculate_operations(&tree_before, &tree_after);
    let distance = ted.calculate_distance(&tree_before, &tree_after);
    
    println!("🔍 Insertion Operations:");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Number of operations: {}", operations.len());
    for (i, op) in operations.iter().enumerate() {
        println!("  • Operation {}: {:?}", i + 1, op);
    }

    // Test deletions
    let operations = ted.calculate_operations(&tree_after, &tree_before);
    let distance = ted.calculate_distance(&tree_after, &tree_before);
    
    println!("\n🔍 Deletion Operations:");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Number of operations: {}", operations.len());
    for (i, op) in operations.iter().enumerate() {
        println!("  • Operation {}: {:?}", i + 1, op);
    }

    // Test updates
    let tree_if = create_leaf_node(NodeType::IfStatement);
    let tree_while = create_leaf_node(NodeType::WhileStatement);
    
    let operations = ted.calculate_operations(&tree_if, &tree_while);
    let distance = ted.calculate_distance(&tree_if, &tree_while);
    
    println!("\n🔍 Update Operations:");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Number of operations: {}", operations.len());
    for (i, op) in operations.iter().enumerate() {
        println!("  • Operation {}: {:?}", i + 1, op);
    }

    println!();
    Ok(())
}

/// Demo 4: Performance optimizations (caching and pruning)
fn demo_performance_optimizations() -> Result<()> {
    println!("⚡ Demo 4: Performance Optimizations");
    println!("------------------------------------");

    // Test caching
    let mut ted = TreeEditDistance::with_defaults();
    
    let tree1 = create_complex_function_tree();
    let tree2 = create_refactored_function_tree();
    
    println!("🔍 Caching Performance:");
    
    // First calculation (no cache)
    let start = std::time::Instant::now();
    let distance1 = ted.calculate_distance(&tree1, &tree2);
    let duration1 = start.elapsed();
    let (cache_size, _) = ted.get_cache_stats();
    
    println!("  • First calculation: {:.3} distance in {:?}", distance1, duration1);
    println!("  • Cache entries after: {}", cache_size);
    
    // Second calculation (with cache)
    let start = std::time::Instant::now();
    let distance2 = ted.calculate_distance(&tree1, &tree2);
    let duration2 = start.elapsed();
    
    println!("  • Second calculation: {:.3} distance in {:?}", distance2, duration2);
    println!("  • Speedup: {:.2}x", duration1.as_nanos() as f64 / duration2.as_nanos() as f64);
    
    // Test pruning
    let mut config = ZhangShashaConfig::default();
    config.enable_pruning = true;
    config.max_nodes = 10; // Small limit to trigger pruning
    
    let ted_pruned = TreeEditDistance::new(config);
    
    let large_tree1 = create_large_tree(15); // Exceeds limit
    let large_tree2 = create_large_tree(20);
    
    let start = std::time::Instant::now();
    let distance_pruned = ted_pruned.calculate_distance(&large_tree1, &large_tree2);
    let duration_pruned = start.elapsed();
    
    println!("\n🔍 Pruning Performance:");
    println!("  • Large trees comparison: {:.3} distance in {:?}", distance_pruned, duration_pruned);
    println!("  • Tree 1 nodes: {}", count_nodes(&large_tree1));
    println!("  • Tree 2 nodes: {}", count_nodes(&large_tree2));
    println!("  • Pruning triggered due to size limit");

    println!();
    Ok(())
}

/// Demo 5: Configuration and cost customization
fn demo_configuration_customization() -> Result<()> {
    println!("⚙️  Demo 5: Configuration and Cost Customization");
    println!("-----------------------------------------------");

    let tree1 = create_if_statement_tree();
    let tree2 = create_while_statement_tree();

    // Test different cost configurations
    let configs = vec![
        ("Balanced Costs", ZhangShashaConfig {
            insert_cost: 1.0,
            delete_cost: 1.0,
            update_cost: 1.0,
            ..Default::default()
        }),
        ("Expensive Insertions", ZhangShashaConfig {
            insert_cost: 2.0,
            delete_cost: 1.0,
            update_cost: 0.5,
            ..Default::default()
        }),
        ("Cheap Updates", ZhangShashaConfig {
            insert_cost: 1.0,
            delete_cost: 1.0,
            update_cost: 0.1,
            ..Default::default()
        }),
        ("High Precision", ZhangShashaConfig {
            insert_cost: 1.0,
            delete_cost: 1.0,
            update_cost: 1.0,
            enable_caching: true,
            enable_pruning: false,
            max_depth: 100,
            max_nodes: 50000,
            similarity_threshold: 0.01,
            enable_parallel: true,
        }),
    ];

    for (config_name, config) in configs {
        let ted = TreeEditDistance::new(config);
        let distance = ted.calculate_distance(&tree1, &tree2);
        let similarity = ted.calculate_similarity(&tree1, &tree2);
        
        println!("🔧 {} Configuration:", config_name);
        println!("  • Edit Distance: {:.3}", distance);
        println!("  • Similarity Score: {:.3}", similarity);
        println!("  • Insert Cost: {:.1}", ted.get_config().insert_cost);
        println!("  • Delete Cost: {:.1}", ted.get_config().delete_cost);
        println!("  • Update Cost: {:.1}", ted.get_config().update_cost);
        println!();
    }

    Ok(())
}

/// Demo 6: Real-world code comparison scenarios
fn demo_real_world_scenarios() -> Result<()> {
    println!("🌍 Demo 6: Real-World Code Comparison Scenarios");
    println!("-----------------------------------------------");

    let ted = TreeEditDistance::with_defaults();

    // Scenario 1: Method extraction refactoring
    let original_method = create_long_method_tree();
    let extracted_methods = create_extracted_methods_tree();
    
    let distance = ted.calculate_distance(&original_method, &extracted_methods);
    let similarity = ted.calculate_similarity(&original_method, &extracted_methods);
    
    println!("🔍 Method Extraction Refactoring:");
    println!("  • Original method vs extracted methods");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Similarity Score: {:.3}", similarity);
    println!("  • Refactoring detected: {}", if similarity > 0.6 { "Yes" } else { "No" });

    // Scenario 2: Loop transformation
    let for_loop = create_for_loop_tree();
    let while_loop = create_equivalent_while_loop_tree();
    
    let distance = ted.calculate_distance(&for_loop, &while_loop);
    let similarity = ted.calculate_similarity(&for_loop, &while_loop);
    
    println!("\n🔍 Loop Transformation (For → While):");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Similarity Score: {:.3}", similarity);
    println!("  • Semantic equivalence: {}", if similarity > 0.7 { "High" } else { "Low" });

    // Scenario 3: Code optimization
    let unoptimized = create_unoptimized_code_tree();
    let optimized = create_optimized_code_tree();
    
    let distance = ted.calculate_distance(&unoptimized, &optimized);
    let similarity = ted.calculate_similarity(&unoptimized, &optimized);
    
    println!("\n🔍 Code Optimization:");
    println!("  • Unoptimized vs optimized version");
    println!("  • Edit Distance: {:.3}", distance);
    println!("  • Similarity Score: {:.3}", similarity);
    println!("  • Optimization impact: {}", 
        if similarity > 0.8 { "Minor changes" } 
        else if similarity > 0.5 { "Moderate changes" } 
        else { "Major changes" }
    );

    println!();
    Ok(())
}

// Helper functions for creating test trees

fn create_test_node(node_type: NodeType, children: Vec<ASTNode>) -> ASTNode {
    ASTNode {
        node_type,
        children,
        metadata: NodeMetadata {
            line: 1,
            column: 1,
            attributes: HashMap::new(),
        },
    }
}

fn create_leaf_node(node_type: NodeType) -> ASTNode {
    create_test_node(node_type, Vec::new())
}

fn create_simple_function_tree(name: &str) -> ASTNode {
    create_test_node(NodeType::Function, vec![
        create_test_node(NodeType::Identifier, vec![]), // Function name
        create_test_node(NodeType::ParameterList, vec![]),
        create_test_node(NodeType::Block, vec![
            create_test_node(NodeType::ReturnStatement, vec![
                create_leaf_node(NodeType::Literal),
            ]),
        ]),
    ])
}

fn create_if_statement_tree() -> ASTNode {
    create_test_node(NodeType::IfStatement, vec![
        create_leaf_node(NodeType::BinaryExpression), // condition
        create_test_node(NodeType::Block, vec![
            create_leaf_node(NodeType::ExpressionStatement),
        ]),
    ])
}

fn create_while_statement_tree() -> ASTNode {
    create_test_node(NodeType::WhileStatement, vec![
        create_leaf_node(NodeType::BinaryExpression), // condition
        create_test_node(NodeType::Block, vec![
            create_leaf_node(NodeType::ExpressionStatement),
        ]),
    ])
}

fn create_complex_function_tree() -> ASTNode {
    create_test_node(NodeType::Function, vec![
        create_leaf_node(NodeType::Identifier),
        create_test_node(NodeType::ParameterList, vec![
            create_leaf_node(NodeType::Parameter),
            create_leaf_node(NodeType::Parameter),
        ]),
        create_test_node(NodeType::Block, vec![
            create_test_node(NodeType::IfStatement, vec![
                create_leaf_node(NodeType::BinaryExpression),
                create_test_node(NodeType::Block, vec![
                    create_leaf_node(NodeType::ExpressionStatement),
                    create_leaf_node(NodeType::ReturnStatement),
                ]),
            ]),
            create_leaf_node(NodeType::ExpressionStatement),
        ]),
    ])
}

fn create_refactored_function_tree() -> ASTNode {
    create_test_node(NodeType::Function, vec![
        create_leaf_node(NodeType::Identifier),
        create_test_node(NodeType::ParameterList, vec![
            create_leaf_node(NodeType::Parameter),
            create_leaf_node(NodeType::Parameter),
        ]),
        create_test_node(NodeType::Block, vec![
            create_test_node(NodeType::TryStatement, vec![
                create_test_node(NodeType::Block, vec![
                    create_test_node(NodeType::IfStatement, vec![
                        create_leaf_node(NodeType::BinaryExpression),
                        create_test_node(NodeType::Block, vec![
                            create_leaf_node(NodeType::ExpressionStatement),
                            create_leaf_node(NodeType::ReturnStatement),
                        ]),
                    ]),
                ]),
                create_test_node(NodeType::CatchClause, vec![
                    create_leaf_node(NodeType::Parameter),
                    create_test_node(NodeType::Block, vec![
                        create_leaf_node(NodeType::ExpressionStatement),
                    ]),
                ]),
            ]),
            create_leaf_node(NodeType::ExpressionStatement),
        ]),
    ])
}

fn create_class_tree(class_name: &str, methods: Vec<&str>) -> ASTNode {
    let method_nodes: Vec<ASTNode> = methods.iter()
        .map(|method_name| create_simple_function_tree(method_name))
        .collect();
    
    create_test_node(NodeType::Class, vec![
        create_leaf_node(NodeType::Identifier), // class name
        create_test_node(NodeType::ClassBody, method_nodes),
    ])
}

fn create_large_tree(size: usize) -> ASTNode {
    let mut children = Vec::new();
    for i in 0..size {
        if i % 2 == 0 {
            children.push(create_leaf_node(NodeType::ExpressionStatement));
        } else {
            children.push(create_test_node(NodeType::IfStatement, vec![
                create_leaf_node(NodeType::BinaryExpression),
                create_leaf_node(NodeType::Block),
            ]));
        }
    }
    
    create_test_node(NodeType::Function, children)
}

fn create_long_method_tree() -> ASTNode {
    create_test_node(NodeType::Function, vec![
        create_leaf_node(NodeType::Identifier),
        create_test_node(NodeType::Block, vec![
            create_leaf_node(NodeType::ExpressionStatement),
            create_leaf_node(NodeType::ExpressionStatement),
            create_leaf_node(NodeType::ExpressionStatement),
            create_test_node(NodeType::IfStatement, vec![
                create_leaf_node(NodeType::BinaryExpression),
                create_leaf_node(NodeType::Block),
            ]),
            create_leaf_node(NodeType::ReturnStatement),
        ]),
    ])
}

fn create_extracted_methods_tree() -> ASTNode {
    create_test_node(NodeType::Function, vec![
        create_leaf_node(NodeType::Identifier),
        create_test_node(NodeType::Block, vec![
            create_test_node(NodeType::CallExpression, vec![
                create_leaf_node(NodeType::Identifier), // helper method call
            ]),
            create_test_node(NodeType::IfStatement, vec![
                create_leaf_node(NodeType::BinaryExpression),
                create_leaf_node(NodeType::Block),
            ]),
            create_leaf_node(NodeType::ReturnStatement),
        ]),
    ])
}

fn create_for_loop_tree() -> ASTNode {
    create_test_node(NodeType::ForStatement, vec![
        create_leaf_node(NodeType::VariableDeclaration), // init
        create_leaf_node(NodeType::BinaryExpression),    // condition
        create_leaf_node(NodeType::UpdateExpression),    // update
        create_test_node(NodeType::Block, vec![
            create_leaf_node(NodeType::ExpressionStatement),
        ]),
    ])
}

fn create_equivalent_while_loop_tree() -> ASTNode {
    create_test_node(NodeType::Block, vec![
        create_leaf_node(NodeType::VariableDeclaration), // init
        create_test_node(NodeType::WhileStatement, vec![
            create_leaf_node(NodeType::BinaryExpression), // condition
            create_test_node(NodeType::Block, vec![
                create_leaf_node(NodeType::ExpressionStatement),
                create_leaf_node(NodeType::UpdateExpression), // update
            ]),
        ]),
    ])
}

fn create_unoptimized_code_tree() -> ASTNode {
    create_test_node(NodeType::Function, vec![
        create_leaf_node(NodeType::Identifier),
        create_test_node(NodeType::Block, vec![
            create_leaf_node(NodeType::VariableDeclaration),
            create_leaf_node(NodeType::VariableDeclaration),
            create_test_node(NodeType::ForStatement, vec![
                create_leaf_node(NodeType::VariableDeclaration),
                create_leaf_node(NodeType::BinaryExpression),
                create_leaf_node(NodeType::UpdateExpression),
                create_test_node(NodeType::Block, vec![
                    create_leaf_node(NodeType::ExpressionStatement),
                    create_leaf_node(NodeType::ExpressionStatement),
                ]),
            ]),
        ]),
    ])
}

fn create_optimized_code_tree() -> ASTNode {
    create_test_node(NodeType::Function, vec![
        create_leaf_node(NodeType::Identifier),
        create_test_node(NodeType::Block, vec![
            create_leaf_node(NodeType::VariableDeclaration), // Combined declarations
            create_test_node(NodeType::ForStatement, vec![
                create_leaf_node(NodeType::VariableDeclaration),
                create_leaf_node(NodeType::BinaryExpression),
                create_leaf_node(NodeType::UpdateExpression),
                create_test_node(NodeType::Block, vec![
                    create_leaf_node(NodeType::ExpressionStatement), // Optimized single statement
                ]),
            ]),
        ]),
    ])
}

fn count_nodes(tree: &ASTNode) -> usize {
    1 + tree.children.iter().map(count_nodes).sum::<usize>()
}

fn calculate_depth(tree: &ASTNode) -> usize {
    if tree.children.is_empty() {
        1
    } else {
        1 + tree.children.iter().map(calculate_depth).max().unwrap_or(0)
    }
}
