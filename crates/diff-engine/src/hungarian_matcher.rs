//! Hungarian algorithm implementation for optimal function matching

use crate::similarity_scorer::{SimilarityScorer, ComprehensiveSimilarityScore};
use smart_diff_parser::{ASTNode, Language};
use smart_diff_semantic::EnhancedFunctionSignature;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use anyhow::Result;

/// Configuration for Hungarian algorithm matching
#[derive(Debug, Clone)]
pub struct HungarianMatcherConfig {
    /// Minimum similarity threshold for matches (default: 0.7)
    pub min_similarity_threshold: f64,
    /// Maximum cost for assignment (1.0 - min_similarity)
    pub max_assignment_cost: f64,
    /// Enable many-to-many matching for split/merge detection
    pub enable_many_to_many: bool,
    /// Maximum number of candidates to consider per function
    pub max_candidates_per_function: usize,
    /// Enable cross-file matching
    pub enable_cross_file_matching: bool,
    /// Penalty for cross-file matches (0.0 = no penalty, 1.0 = maximum penalty)
    pub cross_file_penalty: f64,
}

impl Default for HungarianMatcherConfig {
    fn default() -> Self {
        Self {
            min_similarity_threshold: 0.7,
            max_assignment_cost: 0.3, // 1.0 - 0.7
            enable_many_to_many: true,
            max_candidates_per_function: 10,
            enable_cross_file_matching: true,
            cross_file_penalty: 0.1,
        }
    }
}

/// Hungarian algorithm matcher for optimal function assignment
pub struct HungarianMatcher {
    config: HungarianMatcherConfig,
    similarity_scorer: SimilarityScorer,
}

/// Result of Hungarian algorithm matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HungarianMatchResult {
    /// Optimal one-to-one assignments
    pub assignments: Vec<FunctionAssignment>,
    /// Unmatched source functions (deletions)
    pub unmatched_source: Vec<usize>,
    /// Unmatched target functions (additions)
    pub unmatched_target: Vec<usize>,
    /// Many-to-many mappings (splits/merges)
    pub many_to_many_mappings: Vec<ManyToManyMapping>,
    /// Total assignment cost
    pub total_cost: f64,
    /// Average similarity of assignments
    pub average_similarity: f64,
    /// Assignment statistics
    pub statistics: MatchingStatistics,
}

/// Individual function assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionAssignment {
    /// Source function index
    pub source_index: usize,
    /// Target function index
    pub target_index: usize,
    /// Similarity score details
    pub similarity: ComprehensiveSimilarityScore,
    /// Assignment cost (1.0 - similarity)
    pub cost: f64,
    /// Assignment confidence
    pub confidence: f64,
}

/// Many-to-many mapping for split/merge detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManyToManyMapping {
    /// Source function indices
    pub source_indices: Vec<usize>,
    /// Target function indices
    pub target_indices: Vec<usize>,
    /// Mapping type
    pub mapping_type: MappingType,
    /// Combined similarity score
    pub combined_similarity: f64,
    /// Confidence in the mapping
    pub confidence: f64,
}

/// Type of many-to-many mapping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MappingType {
    /// One function split into multiple functions
    Split,
    /// Multiple functions merged into one
    Merge,
    /// Complex many-to-many transformation
    Complex,
}

/// Statistics about the matching process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchingStatistics {
    /// Total source functions
    pub total_source_functions: usize,
    /// Total target functions
    pub total_target_functions: usize,
    /// Number of one-to-one assignments
    pub one_to_one_assignments: usize,
    /// Number of unmatched source functions
    pub unmatched_source_count: usize,
    /// Number of unmatched target functions
    pub unmatched_target_count: usize,
    /// Number of many-to-many mappings
    pub many_to_many_count: usize,
    /// Percentage of functions matched
    pub match_percentage: f64,
    /// Average assignment cost
    pub average_cost: f64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

impl HungarianMatcher {
    pub fn new(language: Language, config: HungarianMatcherConfig) -> Self {
        let similarity_scorer = SimilarityScorer::with_defaults(language);
        
        Self {
            config,
            similarity_scorer,
        }
    }
    
    pub fn with_defaults(language: Language) -> Self {
        Self::new(language, HungarianMatcherConfig::default())
    }
    
    /// Perform optimal matching between source and target functions
    pub fn match_functions(
        &mut self,
        source_functions: &[(EnhancedFunctionSignature, ASTNode)],
        target_functions: &[(EnhancedFunctionSignature, ASTNode)],
    ) -> Result<HungarianMatchResult> {
        let start_time = std::time::Instant::now();
        
        // Handle empty cases
        if source_functions.is_empty() && target_functions.is_empty() {
            return Ok(self.create_empty_result(0));
        }
        
        if source_functions.is_empty() {
            return Ok(self.create_all_additions_result(target_functions.len()));
        }
        
        if target_functions.is_empty() {
            return Ok(self.create_all_deletions_result(source_functions.len()));
        }
        
        // Calculate similarity matrix
        let similarity_matrix = self.calculate_similarity_matrix(source_functions, target_functions)?;
        
        // Convert similarity to cost matrix (Hungarian algorithm minimizes cost)
        let cost_matrix = self.similarity_to_cost_matrix(&similarity_matrix);
        
        // Apply Hungarian algorithm
        let assignments = self.solve_hungarian_assignment(&cost_matrix)?;
        
        // Process assignments and create result
        let mut result = self.process_assignments(
            source_functions,
            target_functions,
            &assignments,
            &similarity_matrix,
        )?;
        
        // Detect many-to-many mappings if enabled
        if self.config.enable_many_to_many {
            let many_to_many = self.detect_many_to_many_mappings(
                source_functions,
                target_functions,
                &result.assignments,
                &result.unmatched_source,
                &result.unmatched_target,
            )?;
            result.many_to_many_mappings = many_to_many;
        }
        
        // Calculate statistics
        let execution_time = start_time.elapsed().as_millis() as u64;
        result.statistics = self.calculate_statistics(
            source_functions.len(),
            target_functions.len(),
            &result,
            execution_time,
        );
        
        Ok(result)
    }
    
    /// Calculate similarity matrix between all source and target function pairs
    fn calculate_similarity_matrix(
        &mut self,
        source_functions: &[(EnhancedFunctionSignature, ASTNode)],
        target_functions: &[(EnhancedFunctionSignature, ASTNode)],
    ) -> Result<Vec<Vec<ComprehensiveSimilarityScore>>> {
        let mut matrix = Vec::with_capacity(source_functions.len());
        
        for (source_sig, source_ast) in source_functions {
            let mut row = Vec::with_capacity(target_functions.len());
            
            for (target_sig, target_ast) in target_functions {
                let mut similarity = self.similarity_scorer.calculate_comprehensive_similarity(
                    source_sig, source_ast,
                    target_sig, target_ast,
                )?;
                
                // Apply cross-file penalty if enabled
                if self.config.enable_cross_file_matching && 
                   source_sig.file_path != target_sig.file_path {
                    similarity.overall_similarity *= (1.0 - self.config.cross_file_penalty);
                }
                
                row.push(similarity);
            }
            
            matrix.push(row);
        }
        
        Ok(matrix)
    }
    
    /// Convert similarity matrix to cost matrix for Hungarian algorithm
    fn similarity_to_cost_matrix(&self, similarity_matrix: &[Vec<ComprehensiveSimilarityScore>]) -> Vec<Vec<f64>> {
        similarity_matrix.iter()
            .map(|row| {
                row.iter()
                    .map(|similarity| {
                        // Cost = 1.0 - similarity, clamped to max_assignment_cost
                        let cost = 1.0 - similarity.overall_similarity;
                        if cost > self.config.max_assignment_cost {
                            f64::INFINITY // Exclude assignments below threshold
                        } else {
                            cost
                        }
                    })
                    .collect()
            })
            .collect()
    }
    
    /// Solve the assignment problem using Hungarian algorithm
    fn solve_hungarian_assignment(&self, cost_matrix: &[Vec<f64>]) -> Result<Vec<(usize, usize)>> {
        use hungarian::minimize;
        
        // Convert to the format expected by the hungarian crate
        let matrix: Vec<Vec<i32>> = cost_matrix.iter()
            .map(|row| {
                row.iter()
                    .map(|&cost| {
                        if cost.is_infinite() {
                            i32::MAX
                        } else {
                            (cost * 1000.0) as i32 // Scale for integer representation
                        }
                    })
                    .collect()
            })
            .collect();
        
        // Solve the assignment problem
        let (assignment_cost, assignments) = minimize(&matrix);
        
        // Convert back to our format, filtering out invalid assignments
        let valid_assignments: Vec<(usize, usize)> = assignments.into_iter()
            .enumerate()
            .filter_map(|(source_idx, target_idx)| {
                if target_idx < cost_matrix[source_idx].len() && 
                   !cost_matrix[source_idx][target_idx].is_infinite() {
                    Some((source_idx, target_idx))
                } else {
                    None
                }
            })
            .collect();
        
        Ok(valid_assignments)
    }
    
    /// Process Hungarian algorithm assignments into structured result
    fn process_assignments(
        &self,
        source_functions: &[(EnhancedFunctionSignature, ASTNode)],
        target_functions: &[(EnhancedFunctionSignature, ASTNode)],
        assignments: &[(usize, usize)],
        similarity_matrix: &[Vec<ComprehensiveSimilarityScore>],
    ) -> Result<HungarianMatchResult> {
        let mut function_assignments = Vec::new();
        let mut matched_source = HashSet::new();
        let mut matched_target = HashSet::new();
        let mut total_cost = 0.0;
        let mut total_similarity = 0.0;
        
        // Process valid assignments
        for &(source_idx, target_idx) in assignments {
            let similarity = &similarity_matrix[source_idx][target_idx];
            let cost = 1.0 - similarity.overall_similarity;
            
            function_assignments.push(FunctionAssignment {
                source_index: source_idx,
                target_index: target_idx,
                similarity: similarity.clone(),
                cost,
                confidence: similarity.confidence,
            });
            
            matched_source.insert(source_idx);
            matched_target.insert(target_idx);
            total_cost += cost;
            total_similarity += similarity.overall_similarity;
        }
        
        // Find unmatched functions
        let unmatched_source: Vec<usize> = (0..source_functions.len())
            .filter(|&i| !matched_source.contains(&i))
            .collect();
        
        let unmatched_target: Vec<usize> = (0..target_functions.len())
            .filter(|&i| !matched_target.contains(&i))
            .collect();
        
        // Calculate averages
        let average_similarity = if function_assignments.is_empty() {
            0.0
        } else {
            total_similarity / function_assignments.len() as f64
        };
        
        Ok(HungarianMatchResult {
            assignments: function_assignments,
            unmatched_source,
            unmatched_target,
            many_to_many_mappings: Vec::new(), // Will be filled later if enabled
            total_cost,
            average_similarity,
            statistics: MatchingStatistics {
                total_source_functions: 0,
                total_target_functions: 0,
                one_to_one_assignments: 0,
                unmatched_source_count: 0,
                unmatched_target_count: 0,
                many_to_many_count: 0,
                match_percentage: 0.0,
                average_cost: 0.0,
                execution_time_ms: 0,
            }, // Will be calculated later
        })
    }

    /// Detect many-to-many mappings for split/merge functions
    fn detect_many_to_many_mappings(
        &mut self,
        source_functions: &[(EnhancedFunctionSignature, ASTNode)],
        target_functions: &[(EnhancedFunctionSignature, ASTNode)],
        assignments: &[FunctionAssignment],
        unmatched_source: &[usize],
        unmatched_target: &[usize],
    ) -> Result<Vec<ManyToManyMapping>> {
        let mut mappings = Vec::new();

        // Detect splits: one source function -> multiple target functions
        mappings.extend(self.detect_splits(
            source_functions,
            target_functions,
            assignments,
            unmatched_source,
            unmatched_target,
        )?);

        // Detect merges: multiple source functions -> one target function
        mappings.extend(self.detect_merges(
            source_functions,
            target_functions,
            assignments,
            unmatched_source,
            unmatched_target,
        )?);

        // Detect complex many-to-many mappings
        mappings.extend(self.detect_complex_mappings(
            source_functions,
            target_functions,
            unmatched_source,
            unmatched_target,
        )?);

        Ok(mappings)
    }

    /// Detect function splits (1 -> N mappings)
    fn detect_splits(
        &mut self,
        source_functions: &[(EnhancedFunctionSignature, ASTNode)],
        target_functions: &[(EnhancedFunctionSignature, ASTNode)],
        assignments: &[FunctionAssignment],
        unmatched_source: &[usize],
        unmatched_target: &[usize],
    ) -> Result<Vec<ManyToManyMapping>> {
        let mut splits = Vec::new();
        let assigned_targets: HashSet<usize> = assignments.iter()
            .map(|a| a.target_index)
            .collect();

        // For each unmatched source function, look for multiple similar target functions
        for &source_idx in unmatched_source {
            let (source_sig, source_ast) = &source_functions[source_idx];
            let mut candidates = Vec::new();

            // Check unmatched target functions for similarity
            for &target_idx in unmatched_target {
                if assigned_targets.contains(&target_idx) {
                    continue;
                }

                let (target_sig, target_ast) = &target_functions[target_idx];
                let similarity = self.similarity_scorer.calculate_comprehensive_similarity(
                    source_sig, source_ast,
                    target_sig, target_ast,
                )?;

                if similarity.overall_similarity >= self.config.min_similarity_threshold {
                    candidates.push((target_idx, similarity));
                }
            }

            // If we found multiple candidates, it might be a split
            if candidates.len() >= 2 {
                candidates.sort_by(|a, b| b.1.overall_similarity.partial_cmp(&a.1.overall_similarity).unwrap());

                let target_indices: Vec<usize> = candidates.iter()
                    .take(self.config.max_candidates_per_function)
                    .map(|(idx, _)| *idx)
                    .collect();

                let combined_similarity = candidates.iter()
                    .take(target_indices.len())
                    .map(|(_, sim)| sim.overall_similarity)
                    .sum::<f64>() / target_indices.len() as f64;

                let confidence = self.calculate_split_confidence(
                    source_sig,
                    &target_indices.iter()
                        .map(|&idx| &target_functions[idx].0)
                        .collect::<Vec<_>>(),
                );

                splits.push(ManyToManyMapping {
                    source_indices: vec![source_idx],
                    target_indices,
                    mapping_type: MappingType::Split,
                    combined_similarity,
                    confidence,
                });
            }
        }

        Ok(splits)
    }

    /// Detect function merges (N -> 1 mappings)
    fn detect_merges(
        &mut self,
        source_functions: &[(EnhancedFunctionSignature, ASTNode)],
        target_functions: &[(EnhancedFunctionSignature, ASTNode)],
        assignments: &[FunctionAssignment],
        unmatched_source: &[usize],
        unmatched_target: &[usize],
    ) -> Result<Vec<ManyToManyMapping>> {
        let mut merges = Vec::new();
        let assigned_sources: HashSet<usize> = assignments.iter()
            .map(|a| a.source_index)
            .collect();

        // For each unmatched target function, look for multiple similar source functions
        for &target_idx in unmatched_target {
            let (target_sig, target_ast) = &target_functions[target_idx];
            let mut candidates = Vec::new();

            // Check unmatched source functions for similarity
            for &source_idx in unmatched_source {
                if assigned_sources.contains(&source_idx) {
                    continue;
                }

                let (source_sig, source_ast) = &source_functions[source_idx];
                let similarity = self.similarity_scorer.calculate_comprehensive_similarity(
                    source_sig, source_ast,
                    target_sig, target_ast,
                )?;

                if similarity.overall_similarity >= self.config.min_similarity_threshold {
                    candidates.push((source_idx, similarity));
                }
            }

            // If we found multiple candidates, it might be a merge
            if candidates.len() >= 2 {
                candidates.sort_by(|a, b| b.1.overall_similarity.partial_cmp(&a.1.overall_similarity).unwrap());

                let source_indices: Vec<usize> = candidates.iter()
                    .take(self.config.max_candidates_per_function)
                    .map(|(idx, _)| *idx)
                    .collect();

                let combined_similarity = candidates.iter()
                    .take(source_indices.len())
                    .map(|(_, sim)| sim.overall_similarity)
                    .sum::<f64>() / source_indices.len() as f64;

                let confidence = self.calculate_merge_confidence(
                    &source_indices.iter()
                        .map(|&idx| &source_functions[idx].0)
                        .collect::<Vec<_>>(),
                    target_sig,
                );

                merges.push(ManyToManyMapping {
                    source_indices,
                    target_indices: vec![target_idx],
                    mapping_type: MappingType::Merge,
                    combined_similarity,
                    confidence,
                });
            }
        }

        Ok(merges)
    }

    /// Detect complex many-to-many mappings
    fn detect_complex_mappings(
        &mut self,
        source_functions: &[(EnhancedFunctionSignature, ASTNode)],
        target_functions: &[(EnhancedFunctionSignature, ASTNode)],
        unmatched_source: &[usize],
        unmatched_target: &[usize],
    ) -> Result<Vec<ManyToManyMapping>> {
        let mut complex_mappings = Vec::new();

        // For complex mappings, we look for groups of functions that have high collective similarity
        // This is a simplified heuristic - in practice, this could be much more sophisticated

        if unmatched_source.len() >= 2 && unmatched_target.len() >= 2 {
            // Try to find the best N:M mapping using a greedy approach
            let mut best_mapping = None;
            let mut best_score = 0.0;

            // Limit search space for performance
            let max_group_size = 3.min(unmatched_source.len()).min(unmatched_target.len());

            for source_group_size in 2..=max_group_size {
                for target_group_size in 2..=max_group_size {
                    if let Some(mapping) = self.find_best_group_mapping(
                        source_functions,
                        target_functions,
                        unmatched_source,
                        unmatched_target,
                        source_group_size,
                        target_group_size,
                    )? {
                        if mapping.combined_similarity > best_score {
                            best_score = mapping.combined_similarity;
                            best_mapping = Some(mapping);
                        }
                    }
                }
            }

            if let Some(mapping) = best_mapping {
                if mapping.combined_similarity >= self.config.min_similarity_threshold {
                    complex_mappings.push(mapping);
                }
            }
        }

        Ok(complex_mappings)
    }

    /// Find the best group mapping for given group sizes
    fn find_best_group_mapping(
        &mut self,
        source_functions: &[(EnhancedFunctionSignature, ASTNode)],
        target_functions: &[(EnhancedFunctionSignature, ASTNode)],
        unmatched_source: &[usize],
        unmatched_target: &[usize],
        source_group_size: usize,
        target_group_size: usize,
    ) -> Result<Option<ManyToManyMapping>> {
        // This is a simplified implementation - could be optimized with more sophisticated algorithms
        let mut best_mapping = None;
        let mut best_score = 0.0;

        // Generate combinations of source and target functions
        let source_combinations = self.generate_combinations(unmatched_source, source_group_size);
        let target_combinations = self.generate_combinations(unmatched_target, target_group_size);

        for source_group in source_combinations.iter().take(10) { // Limit for performance
            for target_group in target_combinations.iter().take(10) {
                let score = self.calculate_group_similarity(
                    source_functions,
                    target_functions,
                    source_group,
                    target_group,
                )?;

                if score > best_score {
                    best_score = score;
                    best_mapping = Some(ManyToManyMapping {
                        source_indices: source_group.clone(),
                        target_indices: target_group.clone(),
                        mapping_type: MappingType::Complex,
                        combined_similarity: score,
                        confidence: self.calculate_complex_mapping_confidence(score),
                    });
                }
            }
        }

        Ok(best_mapping)
    }

    /// Generate combinations of indices
    fn generate_combinations(&self, indices: &[usize], size: usize) -> Vec<Vec<usize>> {
        if size == 0 || size > indices.len() {
            return Vec::new();
        }

        if size == 1 {
            return indices.iter().map(|&i| vec![i]).collect();
        }

        let mut combinations = Vec::new();
        self.generate_combinations_recursive(indices, size, 0, Vec::new(), &mut combinations);
        combinations
    }

    /// Recursive helper for generating combinations
    fn generate_combinations_recursive(
        &self,
        indices: &[usize],
        size: usize,
        start: usize,
        current: Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if current.len() == size {
            result.push(current);
            return;
        }

        for i in start..indices.len() {
            let mut new_current = current.clone();
            new_current.push(indices[i]);
            self.generate_combinations_recursive(indices, size, i + 1, new_current, result);
        }
    }

    /// Calculate similarity score for a group of functions
    fn calculate_group_similarity(
        &mut self,
        source_functions: &[(EnhancedFunctionSignature, ASTNode)],
        target_functions: &[(EnhancedFunctionSignature, ASTNode)],
        source_group: &[usize],
        target_group: &[usize],
    ) -> Result<f64> {
        let mut total_similarity = 0.0;
        let mut comparison_count = 0;

        // Calculate pairwise similarities within the groups
        for &source_idx in source_group {
            for &target_idx in target_group {
                let (source_sig, source_ast) = &source_functions[source_idx];
                let (target_sig, target_ast) = &target_functions[target_idx];

                let similarity = self.similarity_scorer.calculate_comprehensive_similarity(
                    source_sig, source_ast,
                    target_sig, target_ast,
                )?;

                total_similarity += similarity.overall_similarity;
                comparison_count += 1;
            }
        }

        if comparison_count > 0 {
            Ok(total_similarity / comparison_count as f64)
        } else {
            Ok(0.0)
        }
    }

    /// Calculate confidence for split detection
    fn calculate_split_confidence(
        &self,
        source_sig: &EnhancedFunctionSignature,
        target_sigs: &[&EnhancedFunctionSignature],
    ) -> f64 {
        let mut confidence = 0.5; // Base confidence

        // Boost confidence if target functions have similar names to source
        let source_name_lower = source_sig.name.to_lowercase();
        let similar_names = target_sigs.iter()
            .filter(|target_sig| {
                let target_name_lower = target_sig.name.to_lowercase();
                target_name_lower.contains(&source_name_lower) ||
                source_name_lower.contains(&target_name_lower)
            })
            .count();

        confidence += (similar_names as f64 / target_sigs.len() as f64) * 0.3;

        // Boost confidence if functions are in the same file
        let same_file_count = target_sigs.iter()
            .filter(|target_sig| target_sig.file_path == source_sig.file_path)
            .count();

        confidence += (same_file_count as f64 / target_sigs.len() as f64) * 0.2;

        confidence.min(1.0)
    }

    /// Calculate confidence for merge detection
    fn calculate_merge_confidence(
        &self,
        source_sigs: &[&EnhancedFunctionSignature],
        target_sig: &EnhancedFunctionSignature,
    ) -> f64 {
        let mut confidence = 0.5; // Base confidence

        // Boost confidence if source functions have similar names to target
        let target_name_lower = target_sig.name.to_lowercase();
        let similar_names = source_sigs.iter()
            .filter(|source_sig| {
                let source_name_lower = source_sig.name.to_lowercase();
                source_name_lower.contains(&target_name_lower) ||
                target_name_lower.contains(&source_name_lower)
            })
            .count();

        confidence += (similar_names as f64 / source_sigs.len() as f64) * 0.3;

        // Boost confidence if functions are in the same file
        let same_file_count = source_sigs.iter()
            .filter(|source_sig| source_sig.file_path == target_sig.file_path)
            .count();

        confidence += (same_file_count as f64 / source_sigs.len() as f64) * 0.2;

        confidence.min(1.0)
    }

    /// Calculate confidence for complex mapping
    fn calculate_complex_mapping_confidence(&self, similarity_score: f64) -> f64 {
        // Complex mappings have lower base confidence
        let base_confidence = 0.3;
        let similarity_boost = similarity_score * 0.5;

        (base_confidence + similarity_boost).min(1.0)
    }

    /// Calculate matching statistics
    fn calculate_statistics(
        &self,
        total_source: usize,
        total_target: usize,
        result: &HungarianMatchResult,
        execution_time_ms: u64,
    ) -> MatchingStatistics {
        let one_to_one_assignments = result.assignments.len();
        let unmatched_source_count = result.unmatched_source.len();
        let unmatched_target_count = result.unmatched_target.len();
        let many_to_many_count = result.many_to_many_mappings.len();

        let total_matched = one_to_one_assignments +
            result.many_to_many_mappings.iter()
                .map(|m| m.source_indices.len().min(m.target_indices.len()))
                .sum::<usize>();

        let match_percentage = if total_source.max(total_target) > 0 {
            (total_matched as f64 / total_source.max(total_target) as f64) * 100.0
        } else {
            100.0
        };

        let average_cost = if one_to_one_assignments > 0 {
            result.assignments.iter().map(|a| a.cost).sum::<f64>() / one_to_one_assignments as f64
        } else {
            0.0
        };

        MatchingStatistics {
            total_source_functions: total_source,
            total_target_functions: total_target,
            one_to_one_assignments,
            unmatched_source_count,
            unmatched_target_count,
            many_to_many_count,
            match_percentage,
            average_cost,
            execution_time_ms,
        }
    }

    /// Create empty result for when both function sets are empty
    fn create_empty_result(&self, execution_time_ms: u64) -> HungarianMatchResult {
        HungarianMatchResult {
            assignments: Vec::new(),
            unmatched_source: Vec::new(),
            unmatched_target: Vec::new(),
            many_to_many_mappings: Vec::new(),
            total_cost: 0.0,
            average_similarity: 1.0,
            statistics: MatchingStatistics {
                total_source_functions: 0,
                total_target_functions: 0,
                one_to_one_assignments: 0,
                unmatched_source_count: 0,
                unmatched_target_count: 0,
                many_to_many_count: 0,
                match_percentage: 100.0,
                average_cost: 0.0,
                execution_time_ms,
            },
        }
    }

    /// Create result for when all functions are additions
    fn create_all_additions_result(&self, target_count: usize) -> HungarianMatchResult {
        HungarianMatchResult {
            assignments: Vec::new(),
            unmatched_source: Vec::new(),
            unmatched_target: (0..target_count).collect(),
            many_to_many_mappings: Vec::new(),
            total_cost: target_count as f64,
            average_similarity: 0.0,
            statistics: MatchingStatistics {
                total_source_functions: 0,
                total_target_functions: target_count,
                one_to_one_assignments: 0,
                unmatched_source_count: 0,
                unmatched_target_count: target_count,
                many_to_many_count: 0,
                match_percentage: 0.0,
                average_cost: 1.0,
                execution_time_ms: 0,
            },
        }
    }

    /// Create result for when all functions are deletions
    fn create_all_deletions_result(&self, source_count: usize) -> HungarianMatchResult {
        HungarianMatchResult {
            assignments: Vec::new(),
            unmatched_source: (0..source_count).collect(),
            unmatched_target: Vec::new(),
            many_to_many_mappings: Vec::new(),
            total_cost: source_count as f64,
            average_similarity: 0.0,
            statistics: MatchingStatistics {
                total_source_functions: source_count,
                total_target_functions: 0,
                one_to_one_assignments: 0,
                unmatched_source_count: source_count,
                unmatched_target_count: 0,
                many_to_many_count: 0,
                match_percentage: 0.0,
                average_cost: 1.0,
                execution_time_ms: 0,
            },
        }
    }

    /// Get configuration
    pub fn get_config(&self) -> &HungarianMatcherConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: HungarianMatcherConfig) {
        self.config = config;
    }

    /// Clear similarity scorer cache
    pub fn clear_cache(&mut self) {
        self.similarity_scorer.clear_cache();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smart_diff_parser::{ASTNode, NodeType, NodeMetadata};
    use smart_diff_semantic::{EnhancedFunctionSignature, FunctionType, Visibility, TypeSignature};
    use std::collections::HashMap;

    fn create_test_ast_node(node_type: NodeType) -> ASTNode {
        ASTNode {
            node_type,
            children: Vec::new(),
            metadata: NodeMetadata {
                line: 1,
                column: 1,
                attributes: HashMap::new(),
            },
        }
    }

    fn create_test_function_signature(name: &str, file_path: &str) -> EnhancedFunctionSignature {
        EnhancedFunctionSignature {
            name: name.to_string(),
            qualified_name: format!("TestClass.{}", name),
            parameters: Vec::new(),
            return_type: TypeSignature::new("void".to_string()),
            generic_parameters: Vec::new(),
            visibility: Visibility::Public,
            modifiers: Vec::new(),
            annotations: Vec::new(),
            file_path: file_path.to_string(),
            line: 1,
            column: 1,
            end_line: 10,
            function_type: FunctionType::Method,
            complexity_metrics: None,
            dependencies: Vec::new(),
            signature_hash: format!("{}_hash", name),
            normalized_hash: format!("{}_normalized", name),
        }
    }

    #[test]
    fn test_hungarian_matcher_config_default() {
        let config = HungarianMatcherConfig::default();

        assert_eq!(config.min_similarity_threshold, 0.7);
        assert_eq!(config.max_assignment_cost, 0.3);
        assert!(config.enable_many_to_many);
        assert_eq!(config.max_candidates_per_function, 10);
        assert!(config.enable_cross_file_matching);
        assert_eq!(config.cross_file_penalty, 0.1);
    }

    #[test]
    fn test_hungarian_matcher_creation() {
        let config = HungarianMatcherConfig::default();
        let matcher = HungarianMatcher::new(Language::Java, config);

        assert_eq!(matcher.config.min_similarity_threshold, 0.7);
    }

    #[test]
    fn test_empty_function_sets() {
        let mut matcher = HungarianMatcher::with_defaults(Language::Java);

        let source_functions = Vec::new();
        let target_functions = Vec::new();

        let result = matcher.match_functions(&source_functions, &target_functions).unwrap();

        assert_eq!(result.assignments.len(), 0);
        assert_eq!(result.unmatched_source.len(), 0);
        assert_eq!(result.unmatched_target.len(), 0);
        assert_eq!(result.average_similarity, 1.0);
        assert_eq!(result.statistics.match_percentage, 100.0);
    }

    #[test]
    fn test_all_additions() {
        let mut matcher = HungarianMatcher::with_defaults(Language::Java);

        let source_functions = Vec::new();
        let target_functions = vec![
            (create_test_function_signature("func1", "test.java"), create_test_ast_node(NodeType::Function)),
            (create_test_function_signature("func2", "test.java"), create_test_ast_node(NodeType::Function)),
        ];

        let result = matcher.match_functions(&source_functions, &target_functions).unwrap();

        assert_eq!(result.assignments.len(), 0);
        assert_eq!(result.unmatched_source.len(), 0);
        assert_eq!(result.unmatched_target.len(), 2);
        assert_eq!(result.average_similarity, 0.0);
        assert_eq!(result.statistics.total_target_functions, 2);
        assert_eq!(result.statistics.match_percentage, 0.0);
    }

    #[test]
    fn test_all_deletions() {
        let mut matcher = HungarianMatcher::with_defaults(Language::Java);

        let source_functions = vec![
            (create_test_function_signature("func1", "test.java"), create_test_ast_node(NodeType::Function)),
            (create_test_function_signature("func2", "test.java"), create_test_ast_node(NodeType::Function)),
        ];
        let target_functions = Vec::new();

        let result = matcher.match_functions(&source_functions, &target_functions).unwrap();

        assert_eq!(result.assignments.len(), 0);
        assert_eq!(result.unmatched_source.len(), 2);
        assert_eq!(result.unmatched_target.len(), 0);
        assert_eq!(result.average_similarity, 0.0);
        assert_eq!(result.statistics.total_source_functions, 2);
        assert_eq!(result.statistics.match_percentage, 0.0);
    }

    #[test]
    fn test_mapping_type_variants() {
        assert_ne!(MappingType::Split, MappingType::Merge);
        assert_ne!(MappingType::Merge, MappingType::Complex);
        assert_ne!(MappingType::Complex, MappingType::Split);
    }

    #[test]
    fn test_similarity_to_cost_conversion() {
        let config = HungarianMatcherConfig::default();
        let matcher = HungarianMatcher::new(Language::Java, config);

        // Create mock similarity matrix
        let similarity_matrix = vec![
            vec![
                ComprehensiveSimilarityScore {
                    overall_similarity: 0.9,
                    signature_similarity: smart_diff_semantic::FunctionSignatureSimilarity {
                        overall_similarity: 0.9,
                        name_similarity: 0.9,
                        parameter_similarity: 0.8,
                        return_type_similarity: 0.95,
                        modifier_similarity: 0.7,
                        complexity_similarity: 0.6,
                        is_potential_match: true,
                        similarity_breakdown: smart_diff_semantic::SimilarityBreakdown {
                            exact_name_match: true,
                            parameter_count_match: true,
                            parameter_types_match: vec![true],
                            return_type_match: true,
                            visibility_match: true,
                            static_match: true,
                            generic_parameters_match: true,
                        },
                    },
                    body_similarity: crate::similarity_scorer::ASTSimilarityScore {
                        overall_similarity: 0.85,
                        structural_similarity: 0.9,
                        content_similarity: 0.8,
                        control_flow_similarity: 0.85,
                        edit_distance_score: 0.9,
                        depth_similarity: 0.95,
                        node_count_similarity: 0.9,
                    },
                    context_similarity: crate::similarity_scorer::ContextSimilarityScore {
                        overall_similarity: 0.75,
                        function_call_similarity: 0.8,
                        variable_usage_similarity: 0.7,
                        dependency_similarity: 0.6,
                        surrounding_code_similarity: 0.9,
                        namespace_context_similarity: 0.8,
                    },
                    semantic_metrics: crate::similarity_scorer::SemanticSimilarityMetrics {
                        type_usage_similarity: 0.85,
                        api_pattern_similarity: 0.9,
                        error_handling_similarity: 0.8,
                        resource_management_similarity: 0.7,
                        algorithm_pattern_similarity: 0.75,
                    },
                    confidence: 0.92,
                    match_type: crate::similarity_scorer::MatchType::HighSimilarity,
                    similarity_breakdown: crate::similarity_scorer::DetailedSimilarityBreakdown {
                        signature_components: HashMap::new(),
                        ast_node_distribution: HashMap::new(),
                        control_flow_patterns: Vec::new(),
                        common_function_calls: Vec::new(),
                        common_variables: Vec::new(),
                        contributing_factors: Vec::new(),
                        dissimilarity_factors: Vec::new(),
                    },
                }
            ]
        ];

        let cost_matrix = matcher.similarity_to_cost_matrix(&similarity_matrix);

        assert_eq!(cost_matrix.len(), 1);
        assert_eq!(cost_matrix[0].len(), 1);
        assert!((cost_matrix[0][0] - 0.1).abs() < 0.001); // 1.0 - 0.9 = 0.1
    }

    #[test]
    fn test_combination_generation() {
        let matcher = HungarianMatcher::with_defaults(Language::Java);

        let indices = vec![0, 1, 2, 3];
        let combinations = matcher.generate_combinations(&indices, 2);

        assert_eq!(combinations.len(), 6); // C(4,2) = 6
        assert!(combinations.contains(&vec![0, 1]));
        assert!(combinations.contains(&vec![0, 2]));
        assert!(combinations.contains(&vec![0, 3]));
        assert!(combinations.contains(&vec![1, 2]));
        assert!(combinations.contains(&vec![1, 3]));
        assert!(combinations.contains(&vec![2, 3]));
    }

    #[test]
    fn test_combination_generation_edge_cases() {
        let matcher = HungarianMatcher::with_defaults(Language::Java);

        let indices = vec![0, 1, 2];

        // Size 0
        let combinations = matcher.generate_combinations(&indices, 0);
        assert_eq!(combinations.len(), 0);

        // Size 1
        let combinations = matcher.generate_combinations(&indices, 1);
        assert_eq!(combinations.len(), 3);
        assert!(combinations.contains(&vec![0]));
        assert!(combinations.contains(&vec![1]));
        assert!(combinations.contains(&vec![2]));

        // Size larger than input
        let combinations = matcher.generate_combinations(&indices, 5);
        assert_eq!(combinations.len(), 0);
    }

    #[test]
    fn test_confidence_calculations() {
        let matcher = HungarianMatcher::with_defaults(Language::Java);

        // Test split confidence
        let source_sig = create_test_function_signature("processData", "test.java");
        let target_sigs = vec![
            &create_test_function_signature("processDataPart1", "test.java"),
            &create_test_function_signature("processDataPart2", "test.java"),
        ];

        let split_confidence = matcher.calculate_split_confidence(&source_sig, &target_sigs);
        assert!(split_confidence > 0.5); // Should have decent confidence due to name similarity and same file

        // Test merge confidence
        let source_sigs = vec![
            &create_test_function_signature("validateInput", "test.java"),
            &create_test_function_signature("validateOutput", "test.java"),
        ];
        let target_sig = create_test_function_signature("validate", "test.java");

        let merge_confidence = matcher.calculate_merge_confidence(&source_sigs, &target_sig);
        assert!(merge_confidence > 0.5); // Should have decent confidence due to name similarity and same file

        // Test complex mapping confidence
        let complex_confidence = matcher.calculate_complex_mapping_confidence(0.8);
        assert!(complex_confidence > 0.3); // Should be above base confidence
        assert!(complex_confidence <= 1.0);
    }

    #[test]
    fn test_config_updates() {
        let mut matcher = HungarianMatcher::with_defaults(Language::Java);

        let original_threshold = matcher.get_config().min_similarity_threshold;
        assert_eq!(original_threshold, 0.7);

        let new_config = HungarianMatcherConfig {
            min_similarity_threshold: 0.8,
            max_assignment_cost: 0.2,
            enable_many_to_many: false,
            max_candidates_per_function: 5,
            enable_cross_file_matching: false,
            cross_file_penalty: 0.2,
        };

        matcher.set_config(new_config);

        assert_eq!(matcher.get_config().min_similarity_threshold, 0.8);
        assert_eq!(matcher.get_config().max_assignment_cost, 0.2);
        assert!(!matcher.get_config().enable_many_to_many);
        assert_eq!(matcher.get_config().max_candidates_per_function, 5);
        assert!(!matcher.get_config().enable_cross_file_matching);
        assert_eq!(matcher.get_config().cross_file_penalty, 0.2);
    }
}
