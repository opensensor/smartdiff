//! Function matching algorithms

use serde::{Deserialize, Serialize};
use smart_diff_parser::{Function, MatchResult};
use std::collections::HashMap;

/// Function matcher that finds optimal mappings between function sets
pub struct FunctionMatcher {
    threshold: f64,
}

/// Similarity score between two functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityScore {
    pub signature_similarity: f64,
    pub body_similarity: f64,
    pub context_similarity: f64,
    pub overall_similarity: f64,
}

impl FunctionMatcher {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    /// Match functions between two sets using Hungarian algorithm
    pub fn match_functions(
        &self,
        source_functions: &[Function],
        target_functions: &[Function],
    ) -> MatchResult {
        let mut result = MatchResult::new();

        if source_functions.is_empty() && target_functions.is_empty() {
            result.similarity = 1.0;
            return result;
        }

        // Calculate similarity matrix
        let similarity_matrix =
            self.calculate_similarity_matrix(source_functions, target_functions);

        // Apply Hungarian algorithm for optimal matching
        let matches = self.hungarian_matching(&similarity_matrix);

        // Process matches and create result
        self.process_matches(source_functions, target_functions, &matches, &mut result);

        result.calculate_similarity();
        result
    }

    fn calculate_similarity_matrix(
        &self,
        source: &[Function],
        target: &[Function],
    ) -> Vec<Vec<f64>> {
        let mut matrix = Vec::new();

        for source_func in source {
            let mut row = Vec::new();
            for target_func in target {
                let similarity = self.calculate_function_similarity(source_func, target_func);
                row.push(similarity.overall_similarity);
            }
            matrix.push(row);
        }

        matrix
    }

    /// Calculate similarity between two functions
    pub fn calculate_function_similarity(
        &self,
        func1: &Function,
        func2: &Function,
    ) -> SimilarityScore {
        // Signature similarity (40% weight)
        let signature_similarity = func1.signature.similarity(&func2.signature);

        // Body similarity using AST structure (40% weight)
        let body_similarity = self.calculate_ast_similarity(&func1.body, &func2.body);

        // Context similarity (20% weight) - based on surrounding functions, calls, etc.
        let context_similarity = self.calculate_context_similarity(func1, func2);

        // Weighted overall similarity
        let overall_similarity =
            signature_similarity * 0.4 + body_similarity * 0.4 + context_similarity * 0.2;

        SimilarityScore {
            signature_similarity,
            body_similarity,
            context_similarity,
            overall_similarity,
        }
    }

    fn calculate_ast_similarity(
        &self,
        ast1: &smart_diff_parser::ASTNode,
        ast2: &smart_diff_parser::ASTNode,
    ) -> f64 {
        // Simple structural similarity based on node types and tree structure
        if ast1.node_type != ast2.node_type {
            return 0.0;
        }

        if ast1.children.is_empty() && ast2.children.is_empty() {
            return 1.0;
        }

        if ast1.children.len() != ast2.children.len() {
            return 0.5; // Partial similarity for different child counts
        }

        let mut total_similarity = 0.0;
        for (child1, child2) in ast1.children.iter().zip(ast2.children.iter()) {
            total_similarity += self.calculate_ast_similarity(child1, child2);
        }

        total_similarity / ast1.children.len() as f64
    }

    fn calculate_context_similarity(&self, func1: &Function, func2: &Function) -> f64 {
        // Compare function calls, dependencies, etc.
        let calls1 = func1.extract_function_calls();
        let calls2 = func2.extract_function_calls();

        if calls1.is_empty() && calls2.is_empty() {
            return 1.0;
        }

        let common_calls = calls1.iter().filter(|call| calls2.contains(call)).count();

        let total_calls = calls1.len().max(calls2.len());
        if total_calls > 0 {
            common_calls as f64 / total_calls as f64
        } else {
            1.0
        }
    }

    fn hungarian_matching(&self, similarity_matrix: &[Vec<f64>]) -> Vec<(usize, usize)> {
        // Placeholder implementation - in reality would use Hungarian algorithm
        let mut matches = Vec::new();

        for (i, row) in similarity_matrix.iter().enumerate() {
            if let Some((j, &similarity)) = row
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            {
                if similarity >= self.threshold {
                    matches.push((i, j));
                }
            }
        }

        matches
    }

    fn process_matches(
        &self,
        source: &[Function],
        target: &[Function],
        matches: &[(usize, usize)],
        result: &mut MatchResult,
    ) {
        let mut matched_source = std::collections::HashSet::new();
        let mut matched_target = std::collections::HashSet::new();

        for &(source_idx, target_idx) in matches {
            let source_func = &source[source_idx];
            let target_func = &target[target_idx];

            result
                .mapping
                .insert(source_func.hash.clone(), target_func.hash.clone());
            matched_source.insert(source_idx);
            matched_target.insert(target_idx);

            // Create change record if functions are different
            let similarity = self.calculate_function_similarity(source_func, target_func);
            if similarity.overall_similarity < 1.0 {
                let change = smart_diff_parser::Change::new(
                    smart_diff_parser::ChangeType::Modify,
                    format!(
                        "Function '{}' modified (similarity: {:.2})",
                        source_func.signature.name, similarity.overall_similarity
                    ),
                )
                .with_confidence(similarity.overall_similarity);

                result.changes.push(change);
            }
        }

        // Record unmatched functions
        for (i, func) in source.iter().enumerate() {
            if !matched_source.contains(&i) {
                result.unmatched_source.push(func.hash.clone());

                let change = smart_diff_parser::Change::new(
                    smart_diff_parser::ChangeType::Delete,
                    format!("Function '{}' deleted", func.signature.name),
                );
                result.changes.push(change);
            }
        }

        for (i, func) in target.iter().enumerate() {
            if !matched_target.contains(&i) {
                result.unmatched_target.push(func.hash.clone());

                let change = smart_diff_parser::Change::new(
                    smart_diff_parser::ChangeType::Add,
                    format!("Function '{}' added", func.signature.name),
                );
                result.changes.push(change);
            }
        }
    }
}
