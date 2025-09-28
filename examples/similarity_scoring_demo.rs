//! Demonstration of comprehensive similarity scoring algorithm

use smart_diff_engine::{ComprehensiveSimilarityScore, SimilarityScorer};
use smart_diff_parser::{tree_sitter::TreeSitterParser, Language};
use smart_diff_semantic::FunctionSignatureExtractor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Smart Code Diff - Comprehensive Similarity Scoring Demo");
    println!("======================================================");

    // Demo basic similarity scoring
    demo_basic_similarity_scoring()?;

    // Demo advanced AST comparison
    demo_advanced_ast_comparison()?;

    // Demo semantic similarity analysis
    demo_semantic_similarity_analysis()?;

    // Demo match type classification
    demo_match_type_classification()?;

    // Demo cross-language similarity (if enabled)
    demo_cross_language_similarity()?;

    Ok(())
}

fn demo_basic_similarity_scoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Basic Similarity Scoring ---");

    let parser = TreeSitterParser::new()?;
    let mut signature_extractor = FunctionSignatureExtractor::with_defaults(Language::Java);
    let mut similarity_scorer = SimilarityScorer::with_defaults(Language::Java);

    let original_code = r#"
public class Calculator {
    public int add(int a, int b) {
        if (a < 0 || b < 0) {
            throw new IllegalArgumentException("Negative numbers not allowed");
        }
        int result = a + b;
        return result;
    }
    
    public int multiply(int x, int y) {
        int result = 0;
        for (int i = 0; i < y; i++) {
            result += x;
        }
        return result;
    }
}
"#;

    let modified_code = r#"
public class Calculator {
    public int add(int a, int b) {
        // Added input validation
        if (a < 0 || b < 0) {
            throw new IllegalArgumentException("Negative numbers not supported");
        }
        int sum = a + b;  // Renamed variable
        return sum;
    }
    
    public int multiply(int x, int y) {
        int product = 0;  // Renamed variable
        for (int i = 0; i < y; i++) {
            product += x;
        }
        return product;
    }
    
    // New method added
    public int subtract(int a, int b) {
        return a - b;
    }
}
"#;

    // Parse both versions
    let original_parse = parser.parse(original_code, Language::Java)?;
    let modified_parse = parser.parse(modified_code, Language::Java)?;

    // Extract function signatures
    let original_signatures =
        signature_extractor.extract_signatures("Calculator.java", &original_parse)?;
    let modified_signatures =
        signature_extractor.extract_signatures("Calculator.java", &modified_parse)?;

    println!("Comparing functions between original and modified versions:");

    // Compare each function in original with functions in modified
    for original_sig in &original_signatures.signatures {
        println!("\n  Original function: {}", original_sig.name);

        let mut best_match: Option<(ComprehensiveSimilarityScore, &str)> = None;

        for modified_sig in &modified_signatures.signatures {
            // Find corresponding AST nodes (simplified for demo)
            if let (Some(orig_ast), Some(mod_ast)) = (
                find_function_ast(&original_parse.ast, &original_sig.name),
                find_function_ast(&modified_parse.ast, &modified_sig.name),
            ) {
                let similarity = similarity_scorer.calculate_comprehensive_similarity(
                    original_sig,
                    orig_ast,
                    modified_sig,
                    mod_ast,
                )?;

                if let Some((ref current_best, _)) = best_match {
                    if similarity.overall_similarity > current_best.overall_similarity {
                        best_match = Some((similarity, &modified_sig.name));
                    }
                } else {
                    best_match = Some((similarity, &modified_sig.name));
                }
            }
        }

        if let Some((similarity, matched_name)) = best_match {
            println!(
                "    Best match: {} (similarity: {:.3})",
                matched_name, similarity.overall_similarity
            );
            println!("    Match type: {:?}", similarity.match_type);
            println!("    Confidence: {:.3}", similarity.confidence);

            println!("    Similarity breakdown:");
            println!(
                "      Signature: {:.3}",
                similarity.signature_similarity.overall_similarity
            );
            println!(
                "      AST Body: {:.3}",
                similarity.body_similarity.overall_similarity
            );
            println!(
                "      Context: {:.3}",
                similarity.context_similarity.overall_similarity
            );

            if !similarity
                .similarity_breakdown
                .contributing_factors
                .is_empty()
            {
                println!("    Contributing factors:");
                for factor in &similarity.similarity_breakdown.contributing_factors {
                    println!(
                        "      - {}: {} (impact: {:.3})",
                        factor.factor_type, factor.description, factor.impact
                    );
                }
            }

            if !similarity
                .similarity_breakdown
                .dissimilarity_factors
                .is_empty()
            {
                println!("    Dissimilarity factors:");
                for factor in &similarity.similarity_breakdown.dissimilarity_factors {
                    println!(
                        "      - {}: {} (impact: {:.3})",
                        factor.factor_type, factor.description, factor.impact
                    );
                }
            }
        } else {
            println!("    No suitable match found");
        }
    }

    Ok(())
}

fn demo_advanced_ast_comparison() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Advanced AST Comparison ---");

    let parser = TreeSitterParser::new()?;
    let mut signature_extractor = FunctionSignatureExtractor::with_defaults(Language::Java);
    let mut similarity_scorer = SimilarityScorer::with_defaults(Language::Java);

    let function1 = r#"
public int fibonacci(int n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
"#;

    let function2 = r#"
public int fib(int num) {
    if (num <= 1) {
        return num;
    }
    return fib(num - 1) + fib(num - 2);
}
"#;

    let function3 = r#"
public int fibonacci(int n) {
    int a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        int temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}
"#;

    let functions = vec![
        ("Recursive Fibonacci", function1),
        ("Renamed Recursive Fibonacci", function2),
        ("Iterative Fibonacci", function3),
    ];

    println!("Comparing different implementations of Fibonacci:");

    for (i, (name1, code1)) in functions.iter().enumerate() {
        for (j, (name2, code2)) in functions.iter().enumerate() {
            if i >= j {
                continue;
            } // Avoid duplicate comparisons

            let parse1 = parser.parse(code1, Language::Java)?;
            let parse2 = parser.parse(code2, Language::Java)?;

            let sigs1 = signature_extractor.extract_signatures("test1.java", &parse1)?;
            let sigs2 = signature_extractor.extract_signatures("test2.java", &parse2)?;

            if let (Some(sig1), Some(sig2)) = (sigs1.signatures.first(), sigs2.signatures.first()) {
                if let (Some(ast1), Some(ast2)) = (
                    find_function_ast(&parse1.ast, &sig1.name),
                    find_function_ast(&parse2.ast, &sig2.name),
                ) {
                    let similarity = similarity_scorer
                        .calculate_comprehensive_similarity(sig1, ast1, sig2, ast2)?;

                    println!("\n  {} vs {}", name1, name2);
                    println!(
                        "    Overall similarity: {:.3}",
                        similarity.overall_similarity
                    );
                    println!("    Match type: {:?}", similarity.match_type);

                    println!("    AST Analysis:");
                    println!(
                        "      Structural similarity: {:.3}",
                        similarity.body_similarity.structural_similarity
                    );
                    println!(
                        "      Content similarity: {:.3}",
                        similarity.body_similarity.content_similarity
                    );
                    println!(
                        "      Control flow similarity: {:.3}",
                        similarity.body_similarity.control_flow_similarity
                    );
                    println!(
                        "      Edit distance score: {:.3}",
                        similarity.body_similarity.edit_distance_score
                    );

                    println!("    Semantic Analysis:");
                    println!(
                        "      Algorithm pattern similarity: {:.3}",
                        similarity.semantic_metrics.algorithm_pattern_similarity
                    );
                    println!(
                        "      API pattern similarity: {:.3}",
                        similarity.semantic_metrics.api_pattern_similarity
                    );

                    if !similarity
                        .similarity_breakdown
                        .control_flow_patterns
                        .is_empty()
                    {
                        println!(
                            "    Common control flow patterns: {:?}",
                            similarity.similarity_breakdown.control_flow_patterns
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

fn demo_semantic_similarity_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Semantic Similarity Analysis ---");

    let parser = TreeSitterParser::new()?;
    let mut signature_extractor = FunctionSignatureExtractor::with_defaults(Language::Java);
    let mut similarity_scorer = SimilarityScorer::with_defaults(Language::Java);

    let validation_function1 = r#"
public boolean validateEmail(String email) {
    if (email == null || email.isEmpty()) {
        return false;
    }
    if (!email.contains("@")) {
        return false;
    }
    return true;
}
"#;

    let validation_function2 = r#"
public boolean isValidEmail(String emailAddress) {
    if (emailAddress == null || emailAddress.trim().isEmpty()) {
        return false;
    }
    if (!emailAddress.contains("@")) {
        return false;
    }
    return emailAddress.length() > 5;
}
"#;

    let parse1 = parser.parse(validation_function1, Language::Java)?;
    let parse2 = parser.parse(validation_function2, Language::Java)?;

    let sigs1 = signature_extractor.extract_signatures("validation1.java", &parse1)?;
    let sigs2 = signature_extractor.extract_signatures("validation2.java", &parse2)?;

    if let (Some(sig1), Some(sig2)) = (sigs1.signatures.first(), sigs2.signatures.first()) {
        if let (Some(ast1), Some(ast2)) = (
            find_function_ast(&parse1.ast, &sig1.name),
            find_function_ast(&parse2.ast, &sig2.name),
        ) {
            let similarity =
                similarity_scorer.calculate_comprehensive_similarity(sig1, ast1, sig2, ast2)?;

            println!("Comparing email validation functions:");
            println!("  Overall similarity: {:.3}", similarity.overall_similarity);
            println!("  Match type: {:?}", similarity.match_type);

            println!("\n  Semantic Analysis:");
            println!(
                "    API pattern similarity: {:.3}",
                similarity.semantic_metrics.api_pattern_similarity
            );
            println!(
                "    Error handling similarity: {:.3}",
                similarity.semantic_metrics.error_handling_similarity
            );
            println!(
                "    Type usage similarity: {:.3}",
                similarity.semantic_metrics.type_usage_similarity
            );

            println!("\n  Context Analysis:");
            println!(
                "    Function call similarity: {:.3}",
                similarity.context_similarity.function_call_similarity
            );
            println!(
                "    Variable usage similarity: {:.3}",
                similarity.context_similarity.variable_usage_similarity
            );

            if !similarity.similarity_breakdown.common_variables.is_empty() {
                println!(
                    "    Common variables: {:?}",
                    similarity.similarity_breakdown.common_variables
                );
            }
        }
    }

    Ok(())
}

fn demo_match_type_classification() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Match Type Classification ---");

    println!("Match types and their criteria:");
    println!("  ExactMatch: similarity >= 0.95");
    println!("  HighSimilarity: 0.85 <= similarity < 0.95");
    println!("  PotentialMatch: 0.7 <= similarity < 0.85");
    println!("  WeakMatch: 0.5 <= similarity < 0.7");
    println!("  PotentialRefactoring: high name similarity, low body similarity");
    println!("  PotentialRename: low name similarity, high body similarity");
    println!("  NoMatch: similarity < 0.5");

    println!("\nMatch type examples:");
    println!("  ExactMatch: Identical functions");
    println!("  HighSimilarity: Minor variable renames or comment changes");
    println!("  PotentialMatch: Logic changes but same overall structure");
    println!("  WeakMatch: Significant changes but some similarities");
    println!("  PotentialRefactoring: Function extracted or inlined");
    println!("  PotentialRename: Same logic, different function name");
    println!("  NoMatch: Completely different functions");

    Ok(())
}

fn demo_cross_language_similarity() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Cross-Language Similarity (Conceptual) ---");

    println!("Cross-language similarity would compare equivalent functions across languages:");

    let examples = vec![
        ("Java", "public int add(int a, int b) { return a + b; }"),
        ("Python", "def add(a: int, b: int) -> int: return a + b"),
        ("JavaScript", "function add(a, b) { return a + b; }"),
        ("C++", "int add(int a, int b) { return a + b; }"),
    ];

    for (language, code) in examples {
        println!("  {}: {}", language, code);
    }

    println!("\nCross-language normalization would handle:");
    println!("  - Naming conventions (camelCase vs snake_case)");
    println!("  - Type system differences");
    println!("  - Syntax variations");
    println!("  - Language-specific patterns");

    Ok(())
}

// Helper function to find function AST node by name (simplified for demo)
fn find_function_ast<'a>(
    ast: &'a smart_diff_parser::ASTNode,
    function_name: &str,
) -> Option<&'a smart_diff_parser::ASTNode> {
    use smart_diff_parser::NodeType;

    if matches!(ast.node_type, NodeType::Function | NodeType::Method) {
        if let Some(name) = ast.metadata.attributes.get("name") {
            if name == function_name {
                return Some(ast);
            }
        }
    }

    for child in &ast.children {
        if let Some(found) = find_function_ast(child, function_name) {
            return Some(found);
        }
    }

    None
}
