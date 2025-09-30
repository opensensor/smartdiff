//! Smart function matching with practical heuristics
//!
//! This module implements a practical function matching algorithm that prioritizes
//! common real-world scenarios over theoretical optimality. It uses smart rules to
//! handle same-named functions, simple functions, and cross-file moves.

use smart_diff_parser::{Function, MatchResult, Change, ChangeType, CodeElement};
use std::collections::{HashMap, HashSet};

/// Configuration for smart matching
#[derive(Debug, Clone)]
pub struct SmartMatcherConfig {
    /// Minimum similarity threshold for matching functions
    pub similarity_threshold: f64,
    /// Whether to enable cross-file matching
    pub enable_cross_file_matching: bool,
    /// Penalty factor for cross-file matches (0.0 to 1.0)
    pub cross_file_penalty: f64,
}

impl Default for SmartMatcherConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.7,
            enable_cross_file_matching: true,
            cross_file_penalty: 0.5,
        }
    }
}

/// Smart function matcher using practical heuristics
pub struct SmartMatcher {
    config: SmartMatcherConfig,
}

impl SmartMatcher {
    pub fn new(config: SmartMatcherConfig) -> Self {
        Self { config }
    }

    /// Match functions between two sets using smart heuristics
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

        // Track which target functions have been matched
        let mut matched_targets = HashSet::new();

        // First pass: Match functions greedily by best similarity
        for source_func in source_functions {
            let mut best_match: Option<(usize, f64)> = None;

            for (target_idx, target_func) in target_functions.iter().enumerate() {
                if matched_targets.contains(&target_idx) {
                    continue;
                }

                let similarity = self.calculate_function_similarity(source_func, target_func);

                // Apply cross-file penalty to matching threshold, not to similarity score
                let same_file = source_func.location.file_path == target_func.location.file_path;
                let matching_threshold = if !same_file && self.config.enable_cross_file_matching {
                    // Require higher similarity for cross-file matches
                    self.config.similarity_threshold.max(0.85)
                } else {
                    self.config.similarity_threshold
                };

                if similarity >= matching_threshold {
                    if let Some((_, best_sim)) = best_match {
                        if similarity > best_sim {
                            best_match = Some((target_idx, similarity));
                        }
                    } else {
                        best_match = Some((target_idx, similarity));
                    }
                }
            }

            if let Some((target_idx, similarity)) = best_match {
                let target_func = &target_functions[target_idx];
                matched_targets.insert(target_idx);

                // Add to mapping
                result.mapping.insert(source_func.hash.clone(), target_func.hash.clone());

                // Create change record if not identical
                if similarity < 1.0 {
                    let source_element = CodeElement::from_function(source_func);
                    let target_element = CodeElement::from_function(target_func);

                    let change_type = self.classify_change_type(source_func, target_func, similarity);
                    let description = self.change_type_description(&change_type);

                    let mut change = Change::new(
                        change_type,
                        format!(
                            "Function '{}' {} (similarity: {:.2})",
                            source_func.signature.name,
                            description,
                            similarity
                        ),
                    );
                    change.source = Some(source_element);
                    change.target = Some(target_element);
                    change.confidence = similarity;
                    change.details.similarity_score = Some(similarity);

                    result.changes.push(change);
                }
            } else {
                // Function was deleted
                result.unmatched_source.push(source_func.hash.clone());

                let source_element = CodeElement::from_function(source_func);
                let mut change = Change::new(
                    ChangeType::Delete,
                    format!("Function '{}' deleted", source_func.signature.name),
                );
                change.source = Some(source_element);
                change.target = None;
                change.confidence = 1.0;

                result.changes.push(change);
            }
        }

        // Second pass: Find added functions
        for (target_idx, target_func) in target_functions.iter().enumerate() {
            if !matched_targets.contains(&target_idx) {
                result.unmatched_target.push(target_func.hash.clone());

                let target_element = CodeElement::from_function(target_func);
                let mut change = Change::new(
                    ChangeType::Add,
                    format!("Function '{}' added", target_func.signature.name),
                );
                change.source = None;
                change.target = Some(target_element);
                change.confidence = 1.0;

                result.changes.push(change);
            }
        }

        result.calculate_similarity();
        result
    }

    /// Calculate similarity between two functions using smart rules
    fn calculate_function_similarity(&self, func1: &Function, func2: &Function) -> f64 {
        let same_file = func1.location.file_path == func2.location.file_path;
        let same_name = func1.signature.name == func2.signature.name;

        // Rule 1: Same-named functions in same file should always match
        if same_name && same_file {
            // Base similarity on content/body
            let body_similarity = self.calculate_body_similarity(&func1.body, &func2.body);
            return 0.7 + (body_similarity * 0.3); // Minimum 70% for same name, up to 100%
        }

        // Rule 2: Don't match simple functions unless identical
        if self.is_simple_function(func1) || self.is_simple_function(func2) {
            if same_name && func1.hash == func2.hash {
                return 1.0;
            } else {
                return 0.0;
            }
        }

        // Rule 3: Regular similarity calculation for complex functions
        let mut score = 0.0;
        let mut weight = 0.0;

        // Calculate name similarity first to determine if this is a potential rename
        let name_sim = if same_name {
            1.0
        } else {
            self.string_similarity(&func1.signature.name, &func2.signature.name)
        };

        // Name similarity (30% weight)
        let name_weight = 0.3;
        if same_name {
            score += name_weight;
        } else {
            score += name_weight * name_sim * 0.5;
        }
        weight += name_weight;

        // Signature similarity (20% weight)
        let sig_weight = 0.2;
        let sig_sim = func1.signature.similarity(&func2.signature);
        score += sig_weight * sig_sim;
        weight += sig_weight;

        // Body similarity (50% weight) - highest weight
        let body_weight = 0.5;
        let body_sim = self.calculate_body_similarity(&func1.body, &func2.body);
        score += body_weight * body_sim;
        weight += body_weight;

        let mut final_score = if weight > 0.0 { score / weight } else { 0.0 };

        // Rule 3a: Stricter matching for different-named functions
        // If names are different, require high body similarity to avoid matching
        // unrelated functions with similar structure
        if !same_name {
            // Same file, different names: likely different functions, not renames
            // Require very high body similarity (95%) to match
            if same_file && body_sim < 0.95 {
                return 0.0;
            }

            // Cross-file matches with different names
            // For highly similar names (0.8-1.0), require 85% body similarity
            if name_sim >= 0.8 && body_sim < 0.85 {
                return 0.0;
            }
            // For moderately similar names (0.5-0.8), require 92% body similarity
            if name_sim >= 0.5 && name_sim < 0.8 && body_sim < 0.92 {
                return 0.0;
            }
            // For very different names (< 0.5), require 95% body similarity
            if name_sim < 0.5 && body_sim < 0.95 {
                return 0.0;
            }
        }

        // For cross-file matching, apply penalty only to the matching threshold,
        // not to the similarity score itself. The similarity score should reflect
        // actual code similarity, not matching confidence.
        if !same_file && !self.config.enable_cross_file_matching {
            0.0 // Don't match across files if disabled
        } else {
            final_score
        }
    }

    /// Check if a function is "simple" (small body, likely a getter/setter/wrapper)
    fn is_simple_function(&self, func: &Function) -> bool {
        // Count non-empty nodes in the body
        let node_count = self.count_ast_nodes(&func.body);
        // Simple functions: single statement wrappers, getters, setters
        // Typically have 10 or fewer AST nodes
        node_count <= 10
    }

    /// Count AST nodes recursively
    fn count_ast_nodes(&self, node: &smart_diff_parser::ASTNode) -> usize {
        1 + node.children.iter().map(|child| self.count_ast_nodes(child)).sum::<usize>()
    }

    /// Calculate body similarity using AST structure
    fn calculate_body_similarity(
        &self,
        body1: &smart_diff_parser::ASTNode,
        body2: &smart_diff_parser::ASTNode,
    ) -> f64 {
        // Simple structural similarity based on node count and depth
        let count1 = self.count_ast_nodes(body1);
        let count2 = self.count_ast_nodes(body2);
        
        let depth1 = self.calculate_ast_depth(body1);
        let depth2 = self.calculate_ast_depth(body2);

        // Node count similarity (60%)
        let count_sim = if count1.max(count2) == 0 {
            1.0
        } else {
            count1.min(count2) as f64 / count1.max(count2) as f64
        };

        // Depth similarity
        let depth_sim = if depth1.max(depth2) == 0 {
            1.0
        } else {
            1.0 - ((depth1 as i32 - depth2 as i32).abs() as f64 / depth1.max(depth2) as f64)
        };

        // Content similarity - compare actual text content
        let content_sim = self.string_similarity(
            &body1.metadata.original_text,
            &body2.metadata.original_text,
        );

        // Weighted combination: structure (30%) + content (70%)
        // Content is more important to avoid matching structurally similar but semantically different code
        count_sim * 0.15 + depth_sim * 0.15 + content_sim * 0.7
    }

    /// Calculate AST depth
    fn calculate_ast_depth(&self, node: &smart_diff_parser::ASTNode) -> usize {
        if node.children.is_empty() {
            1
        } else {
            1 + node.children.iter().map(|child| self.calculate_ast_depth(child)).max().unwrap_or(0)
        }
    }

    /// Simple string similarity using character overlap
    fn string_similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        let chars1: HashSet<char> = s1.chars().collect();
        let chars2: HashSet<char> = s2.chars().collect();

        let intersection = chars1.intersection(&chars2).count();
        let union = chars1.union(&chars2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Classify the type of change between two matched functions
    fn classify_change_type(&self, func1: &Function, func2: &Function, similarity: f64) -> ChangeType {
        let same_file = func1.location.file_path == func2.location.file_path;
        let same_name = func1.signature.name == func2.signature.name;

        if !same_file && same_name {
            ChangeType::CrossFileMove
        } else if !same_name && similarity > 0.9 {
            ChangeType::Rename
        } else if !same_file {
            ChangeType::Move
        } else {
            ChangeType::Modify
        }
    }

    /// Get human-readable description of change type
    fn change_type_description(&self, change_type: &ChangeType) -> &'static str {
        match change_type {
            ChangeType::Modify => "modified",
            ChangeType::Rename => "renamed",
            ChangeType::Move => "moved",
            ChangeType::CrossFileMove => "moved to different file",
            _ => "changed",
        }
    }
}

