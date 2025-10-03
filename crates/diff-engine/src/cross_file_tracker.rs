//! Cross-file function tracking for detecting function moves between files

use crate::hungarian_matcher::{HungarianMatcher, HungarianMatcherConfig};
use crate::similarity_scorer::{ComprehensiveSimilarityScore, SimilarityScorer};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use smart_diff_parser::{ASTNode, Language};
use smart_diff_semantic::{
    ComprehensiveDependencyGraphBuilder, EnhancedFunctionSignature, SymbolResolver,
};
use std::collections::{HashMap, HashSet};

/// Configuration for cross-file function tracking
#[derive(Debug, Clone)]
pub struct CrossFileTrackerConfig {
    /// Minimum similarity threshold for cross-file matches
    pub min_cross_file_similarity: f64,
    /// Penalty for cross-file moves (0.0 = no penalty, 1.0 = maximum penalty)
    pub cross_file_move_penalty: f64,
    /// Enable tracking of function renames across files
    pub track_renames: bool,
    /// Enable tracking of function splits/merges across files
    pub track_splits_merges: bool,
    /// Maximum number of files to consider for cross-file matching
    pub max_files_to_consider: usize,
    /// Enable global symbol table integration
    pub use_global_symbol_table: bool,
    /// Enable dependency graph analysis for move detection
    pub use_dependency_analysis: bool,
}

impl Default for CrossFileTrackerConfig {
    fn default() -> Self {
        Self {
            min_cross_file_similarity: 0.8, // Higher threshold for cross-file moves
            cross_file_move_penalty: 0.1,
            track_renames: true,
            track_splits_merges: true,
            max_files_to_consider: 50,
            use_global_symbol_table: true,
            use_dependency_analysis: true,
        }
    }
}

/// Cross-file function tracker
pub struct CrossFileTracker {
    config: CrossFileTrackerConfig,
    #[allow(dead_code)]
    language: Language,
    hungarian_matcher: HungarianMatcher,
    similarity_scorer: SimilarityScorer,
    symbol_resolver: Option<SymbolResolver>,
    dependency_builder: Option<ComprehensiveDependencyGraphBuilder>,
}

/// Result of cross-file function tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossFileTrackingResult {
    /// Functions that moved between files
    pub moved_functions: Vec<FunctionMove>,
    /// Functions that were renamed and moved
    pub renamed_and_moved: Vec<FunctionRenameMove>,
    /// Functions that were split across files
    pub cross_file_splits: Vec<CrossFileSplit>,
    /// Functions that were merged from multiple files
    pub cross_file_merges: Vec<CrossFileMerge>,
    /// File-level statistics
    pub file_statistics: HashMap<String, FileTrackingStats>,
    /// Overall tracking statistics
    pub overall_statistics: CrossFileTrackingStats,
}

/// Function move between files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMove {
    /// Function signature
    pub function_signature: EnhancedFunctionSignature,
    /// Source file path
    pub source_file: String,
    /// Target file path
    pub target_file: String,
    /// Similarity score
    pub similarity: ComprehensiveSimilarityScore,
    /// Confidence in the move detection
    pub confidence: f64,
    /// Move type classification
    pub move_type: MoveType,
}

/// Function rename and move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRenameMove {
    /// Original function name
    pub original_name: String,
    /// New function name
    pub new_name: String,
    /// Source file path
    pub source_file: String,
    /// Target file path
    pub target_file: String,
    /// Similarity score
    pub similarity: ComprehensiveSimilarityScore,
    /// Confidence in the rename+move detection
    pub confidence: f64,
}

/// Cross-file function split
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossFileSplit {
    /// Original function in source file
    pub source_function: String,
    /// Source file path
    pub source_file: String,
    /// Split functions with their target files
    pub split_functions: Vec<(String, String)>, // (function_name, target_file)
    /// Combined similarity score
    pub combined_similarity: f64,
    /// Confidence in the split detection
    pub confidence: f64,
}

/// Cross-file function merge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossFileMerge {
    /// Source functions with their files
    pub source_functions: Vec<(String, String)>, // (function_name, source_file)
    /// Merged function name
    pub merged_function: String,
    /// Target file path
    pub target_file: String,
    /// Combined similarity score
    pub combined_similarity: f64,
    /// Confidence in the merge detection
    pub confidence: f64,
}

/// Type of function move
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MoveType {
    /// Simple move without changes
    SimpleMove,
    /// Move with minor modifications
    MoveWithModification,
    /// Move as part of refactoring
    RefactoringMove,
    /// Move with significant changes
    ComplexMove,
}

/// File-level tracking statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTrackingStats {
    /// Total functions in the file
    pub total_functions: usize,
    /// Functions moved out of this file
    pub functions_moved_out: usize,
    /// Functions moved into this file
    pub functions_moved_in: usize,
    /// Functions renamed within moves
    pub functions_renamed: usize,
    /// Net function change (positive = gained, negative = lost)
    pub net_function_change: i32,
}

/// Overall cross-file tracking statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossFileTrackingStats {
    /// Total files analyzed
    pub total_files: usize,
    /// Total functions tracked
    pub total_functions: usize,
    /// Total cross-file moves detected
    pub total_moves: usize,
    /// Total renames with moves
    pub total_rename_moves: usize,
    /// Total cross-file splits
    pub total_splits: usize,
    /// Total cross-file merges
    pub total_merges: usize,
    /// Percentage of functions that moved
    pub move_percentage: f64,
    /// Average confidence of detections
    pub average_confidence: f64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

impl CrossFileTracker {
    pub fn new(language: Language, config: CrossFileTrackerConfig) -> Self {
        let hungarian_config = HungarianMatcherConfig {
            enable_cross_file_matching: true,
            cross_file_penalty: config.cross_file_move_penalty,
            min_similarity_threshold: config.min_cross_file_similarity,
            ..Default::default()
        };

        let hungarian_matcher = HungarianMatcher::new(language, hungarian_config);
        let similarity_scorer = SimilarityScorer::with_defaults(language);

        Self {
            config,
            language,
            hungarian_matcher,
            similarity_scorer,
            symbol_resolver: None,
            dependency_builder: None,
        }
    }

    pub fn with_defaults(language: Language) -> Self {
        Self::new(language, CrossFileTrackerConfig::default())
    }

    /// Set symbol resolver for global symbol table integration
    pub fn set_symbol_resolver(&mut self, resolver: SymbolResolver) {
        self.symbol_resolver = Some(resolver);
    }

    /// Set dependency graph builder for dependency analysis
    pub fn set_dependency_builder(&mut self, builder: ComprehensiveDependencyGraphBuilder) {
        self.dependency_builder = Some(builder);
    }

    /// Track function moves between file versions
    pub fn track_cross_file_changes(
        &mut self,
        source_files: &HashMap<String, Vec<(EnhancedFunctionSignature, ASTNode)>>,
        target_files: &HashMap<String, Vec<(EnhancedFunctionSignature, ASTNode)>>,
    ) -> Result<CrossFileTrackingResult> {
        let start_time = std::time::Instant::now();

        let mut result = CrossFileTrackingResult {
            moved_functions: Vec::new(),
            renamed_and_moved: Vec::new(),
            cross_file_splits: Vec::new(),
            cross_file_merges: Vec::new(),
            file_statistics: HashMap::new(),
            overall_statistics: CrossFileTrackingStats {
                total_files: 0,
                total_functions: 0,
                total_moves: 0,
                total_rename_moves: 0,
                total_splits: 0,
                total_merges: 0,
                move_percentage: 0.0,
                average_confidence: 0.0,
                execution_time_ms: 0,
            },
        };

        // Step 1: Perform intra-file matching to identify unmatched functions
        let (unmatched_source, unmatched_target) =
            self.identify_unmatched_functions(source_files, target_files)?;

        // Step 2: Detect simple cross-file moves
        let moves = self.detect_cross_file_moves(&unmatched_source, &unmatched_target)?;
        result.moved_functions = moves;

        // Step 3: Detect renames with moves if enabled
        if self.config.track_renames {
            let rename_moves = self.detect_rename_moves(&unmatched_source, &unmatched_target)?;
            result.renamed_and_moved = rename_moves;
        }

        // Step 4: Detect cross-file splits and merges if enabled
        if self.config.track_splits_merges {
            let splits = self.detect_cross_file_splits(&unmatched_source, &unmatched_target)?;
            let merges = self.detect_cross_file_merges(&unmatched_source, &unmatched_target)?;
            result.cross_file_splits = splits;
            result.cross_file_merges = merges;
        }

        // Step 5: Calculate statistics
        result.file_statistics =
            self.calculate_file_statistics(source_files, target_files, &result);

        let execution_time = start_time.elapsed().as_millis() as u64;
        result.overall_statistics =
            self.calculate_overall_statistics(source_files, target_files, &result, execution_time);

        Ok(result)
    }
    #[allow(clippy::type_complexity)]

    /// Identify functions that are unmatched within their original files
    fn identify_unmatched_functions(
        &mut self,
        source_files: &HashMap<String, Vec<(EnhancedFunctionSignature, ASTNode)>>,
        target_files: &HashMap<String, Vec<(EnhancedFunctionSignature, ASTNode)>>,
    ) -> Result<(
        HashMap<String, Vec<(usize, EnhancedFunctionSignature, ASTNode)>>, // source file -> unmatched functions
        HashMap<String, Vec<(usize, EnhancedFunctionSignature, ASTNode)>>, // target file -> unmatched functions
    )> {
        let mut unmatched_source = HashMap::new();
        let mut unmatched_target = HashMap::new();

        // For each file, perform intra-file matching to find unmatched functions
        for (file_path, source_functions) in source_files {
            if let Some(target_functions) = target_files.get(file_path) {
                // Match functions within the same file
                let match_result = self
                    .hungarian_matcher
                    .match_functions(source_functions, target_functions)?;

                // Collect unmatched source functions
                let unmatched: Vec<_> = match_result
                    .unmatched_source
                    .into_iter()
                    .map(|idx| {
                        (
                            idx,
                            source_functions[idx].0.clone(),
                            source_functions[idx].1.clone(),
                        )
                    })
                    .collect();

                if !unmatched.is_empty() {
                    unmatched_source.insert(file_path.clone(), unmatched);
                }

                // Collect unmatched target functions
                let unmatched: Vec<_> = match_result
                    .unmatched_target
                    .into_iter()
                    .map(|idx| {
                        (
                            idx,
                            target_functions[idx].0.clone(),
                            target_functions[idx].1.clone(),
                        )
                    })
                    .collect();

                if !unmatched.is_empty() {
                    unmatched_target.insert(file_path.clone(), unmatched);
                }
            } else {
                // File was deleted - all functions are unmatched
                let unmatched: Vec<_> = source_functions
                    .iter()
                    .enumerate()
                    .map(|(idx, (sig, ast))| (idx, sig.clone(), ast.clone()))
                    .collect();
                unmatched_source.insert(file_path.clone(), unmatched);
            }
        }

        // Handle new files - all functions are unmatched
        for (file_path, target_functions) in target_files {
            if !source_files.contains_key(file_path) {
                let unmatched: Vec<_> = target_functions
                    .iter()
                    .enumerate()
                    .map(|(idx, (sig, ast))| (idx, sig.clone(), ast.clone()))
                    .collect();
                unmatched_target.insert(file_path.clone(), unmatched);
            }
        }

        Ok((unmatched_source, unmatched_target))
    }

    /// Detect simple cross-file moves
    fn detect_cross_file_moves(
        &mut self,
        unmatched_source: &HashMap<String, Vec<(usize, EnhancedFunctionSignature, ASTNode)>>,
        unmatched_target: &HashMap<String, Vec<(usize, EnhancedFunctionSignature, ASTNode)>>,
    ) -> Result<Vec<FunctionMove>> {
        let mut moves = Vec::new();

        // For each unmatched source function, find the best match in target files
        for (source_file, source_functions) in unmatched_source {
            for (_, source_sig, source_ast) in source_functions {
                let mut best_match = None;
                let mut best_similarity = 0.0;

                // Search across all target files
                for (target_file, target_functions) in unmatched_target {
                    if source_file == target_file {
                        continue; // Skip same file (already handled in intra-file matching)
                    }

                    for (_, target_sig, target_ast) in target_functions {
                        // Check if names match (for simple moves)
                        if source_sig.name == target_sig.name {
                            let similarity =
                                self.similarity_scorer.calculate_comprehensive_similarity(
                                    source_sig, source_ast, target_sig, target_ast,
                                )?;

                            if similarity.overall_similarity
                                >= self.config.min_cross_file_similarity
                                && similarity.overall_similarity > best_similarity
                            {
                                best_similarity = similarity.overall_similarity;
                                best_match =
                                    Some((target_file.clone(), target_sig.clone(), similarity));
                            }
                        }
                    }
                }

                // If we found a good match, record the move
                if let Some((target_file, target_sig, similarity)) = best_match {
                    let move_type = self.classify_move_type(&similarity);
                    let confidence = self.calculate_move_confidence(
                        source_sig,
                        &target_sig,
                        source_file,
                        &target_file,
                        &similarity,
                    );

                    moves.push(FunctionMove {
                        function_signature: source_sig.clone(),
                        source_file: source_file.clone(),
                        target_file,
                        similarity,
                        confidence,
                        move_type,
                    });
                }
            }
        }

        Ok(moves)
    }

    /// Detect function renames with moves
    fn detect_rename_moves(
        &mut self,
        unmatched_source: &HashMap<String, Vec<(usize, EnhancedFunctionSignature, ASTNode)>>,
        unmatched_target: &HashMap<String, Vec<(usize, EnhancedFunctionSignature, ASTNode)>>,
    ) -> Result<Vec<FunctionRenameMove>> {
        let mut rename_moves = Vec::new();

        // For each unmatched source function, find potential renames in other files
        for (source_file, source_functions) in unmatched_source {
            for (_, source_sig, source_ast) in source_functions {
                let mut best_match = None;
                let mut best_similarity = 0.0;

                // Search across all target files
                for (target_file, target_functions) in unmatched_target {
                    if source_file == target_file {
                        continue; // Skip same file
                    }

                    for (_, target_sig, target_ast) in target_functions {
                        // Skip if names are the same (handled in simple moves)
                        if source_sig.name == target_sig.name {
                            continue;
                        }

                        let similarity =
                            self.similarity_scorer.calculate_comprehensive_similarity(
                                source_sig, source_ast, target_sig, target_ast,
                            )?;

                        // For renames, we need high body similarity even if signature differs
                        if similarity.body_similarity.overall_similarity >= 0.8
                            && similarity.overall_similarity
                                >= self.config.min_cross_file_similarity
                            && similarity.overall_similarity > best_similarity
                        {
                            best_similarity = similarity.overall_similarity;
                            best_match =
                                Some((target_file.clone(), target_sig.clone(), similarity));
                        }
                    }
                }

                // If we found a good match, record the rename+move
                if let Some((target_file, target_sig, similarity)) = best_match {
                    let confidence = self.calculate_rename_move_confidence(
                        source_sig,
                        &target_sig,
                        source_file,
                        &target_file,
                        &similarity,
                    );

                    rename_moves.push(FunctionRenameMove {
                        original_name: source_sig.name.clone(),
                        new_name: target_sig.name.clone(),
                        source_file: source_file.clone(),
                        target_file,
                        similarity,
                        confidence,
                    });
                }
            }
        }

        Ok(rename_moves)
    }

    /// Detect cross-file function splits
    fn detect_cross_file_splits(
        &mut self,
        unmatched_source: &HashMap<String, Vec<(usize, EnhancedFunctionSignature, ASTNode)>>,
        unmatched_target: &HashMap<String, Vec<(usize, EnhancedFunctionSignature, ASTNode)>>,
    ) -> Result<Vec<CrossFileSplit>> {
        let mut splits = Vec::new();

        // For each unmatched source function, look for multiple similar functions across files
        for (source_file, source_functions) in unmatched_source {
            for (_, source_sig, source_ast) in source_functions {
                let mut candidates = Vec::new();

                // Search for similar functions across all target files
                for (target_file, target_functions) in unmatched_target {
                    for (_, target_sig, target_ast) in target_functions {
                        // Look for functions with similar names or high similarity
                        let similarity =
                            self.similarity_scorer.calculate_comprehensive_similarity(
                                source_sig, source_ast, target_sig, target_ast,
                            )?;

                        if similarity.overall_similarity >= 0.6 || // Lower threshold for splits
                           self.is_potential_split_function(&source_sig.name, &target_sig.name)
                        {
                            candidates.push((
                                target_file.clone(),
                                target_sig.name.clone(),
                                similarity.overall_similarity,
                            ));
                        }
                    }
                }

                // If we found multiple candidates, it might be a split
                if candidates.len() >= 2 {
                    candidates.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

                    let split_functions: Vec<(String, String)> = candidates
                        .into_iter()
                        .take(5) // Limit to top 5 candidates
                        .map(|(file, name, _)| (name, file))
                        .collect();

                    let combined_similarity = split_functions
                        .iter()
                        .map(|(_, _)| 0.7) // Placeholder - would calculate actual combined similarity
                        .sum::<f64>()
                        / split_functions.len() as f64;

                    let confidence = self.calculate_split_confidence(
                        source_sig,
                        &split_functions,
                        combined_similarity,
                    );

                    splits.push(CrossFileSplit {
                        source_function: source_sig.name.clone(),
                        source_file: source_file.clone(),
                        split_functions,
                        combined_similarity,
                        confidence,
                    });
                }
            }
        }

        Ok(splits)
    }

    /// Detect cross-file function merges
    fn detect_cross_file_merges(
        &mut self,
        unmatched_source: &HashMap<String, Vec<(usize, EnhancedFunctionSignature, ASTNode)>>,
        unmatched_target: &HashMap<String, Vec<(usize, EnhancedFunctionSignature, ASTNode)>>,
    ) -> Result<Vec<CrossFileMerge>> {
        let mut merges = Vec::new();

        // For each unmatched target function, look for multiple source functions that might have merged
        for (target_file, target_functions) in unmatched_target {
            for (_, target_sig, target_ast) in target_functions {
                let mut candidates = Vec::new();

                // Search for similar functions across all source files
                for (source_file, source_functions) in unmatched_source {
                    for (_, source_sig, source_ast) in source_functions {
                        let similarity =
                            self.similarity_scorer.calculate_comprehensive_similarity(
                                source_sig, source_ast, target_sig, target_ast,
                            )?;

                        if similarity.overall_similarity >= 0.6 || // Lower threshold for merges
                           self.is_potential_merge_function(&source_sig.name, &target_sig.name)
                        {
                            candidates.push((
                                source_file.clone(),
                                source_sig.name.clone(),
                                similarity.overall_similarity,
                            ));
                        }
                    }
                }

                // If we found multiple candidates, it might be a merge
                if candidates.len() >= 2 {
                    candidates.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

                    let source_functions: Vec<(String, String)> = candidates
                        .into_iter()
                        .take(5) // Limit to top 5 candidates
                        .map(|(file, name, _)| (name, file))
                        .collect();

                    let combined_similarity = source_functions
                        .iter()
                        .map(|(_, _)| 0.7) // Placeholder - would calculate actual combined similarity
                        .sum::<f64>()
                        / source_functions.len() as f64;

                    let confidence = self.calculate_merge_confidence(
                        &source_functions,
                        target_sig,
                        combined_similarity,
                    );

                    merges.push(CrossFileMerge {
                        source_functions,
                        merged_function: target_sig.name.clone(),
                        target_file: target_file.clone(),
                        combined_similarity,
                        confidence,
                    });
                }
            }
        }

        Ok(merges)
    }

    /// Classify the type of move based on similarity score
    fn classify_move_type(&self, similarity: &ComprehensiveSimilarityScore) -> MoveType {
        if similarity.overall_similarity >= 0.95 {
            MoveType::SimpleMove
        } else if similarity.overall_similarity >= 0.85 {
            MoveType::MoveWithModification
        } else if similarity.overall_similarity >= 0.75 {
            MoveType::RefactoringMove
        } else {
            MoveType::ComplexMove
        }
    }

    /// Calculate confidence for move detection
    fn calculate_move_confidence(
        &self,
        source_sig: &EnhancedFunctionSignature,
        target_sig: &EnhancedFunctionSignature,
        source_file: &str,
        target_file: &str,
        similarity: &ComprehensiveSimilarityScore,
    ) -> f64 {
        let mut confidence = similarity.confidence;

        // Boost confidence for exact name matches
        if source_sig.name == target_sig.name {
            confidence += 0.1;
        }

        // Boost confidence for similar file paths
        if self.are_files_related(source_file, target_file) {
            confidence += 0.05;
        }

        // Boost confidence if using global symbol table
        if self.config.use_global_symbol_table {
            if let Some(resolver) = &self.symbol_resolver {
                if self.is_symbol_referenced_across_files(resolver, &source_sig.name, target_file) {
                    confidence += 0.1;
                }
            }
        }

        confidence.min(1.0)
    }

    /// Calculate confidence for rename+move detection
    fn calculate_rename_move_confidence(
        &self,
        _source_sig: &EnhancedFunctionSignature,
        _target_sig: &EnhancedFunctionSignature,
        source_file: &str,
        target_file: &str,
        similarity: &ComprehensiveSimilarityScore,
    ) -> f64 {
        let mut confidence = similarity.confidence * 0.8; // Lower base confidence for renames

        // Boost confidence for high body similarity
        if similarity.body_similarity.overall_similarity >= 0.9 {
            confidence += 0.15;
        }

        // Boost confidence for similar parameter signatures
        if similarity.signature_similarity.parameter_similarity >= 0.9 {
            confidence += 0.1;
        }

        // Boost confidence for related file paths
        if self.are_files_related(source_file, target_file) {
            confidence += 0.05;
        }

        confidence.min(1.0)
    }

    /// Calculate confidence for split detection
    fn calculate_split_confidence(
        &self,
        source_sig: &EnhancedFunctionSignature,
        split_functions: &[(String, String)],
        combined_similarity: f64,
    ) -> f64 {
        let mut confidence = combined_similarity * 0.6; // Lower base confidence for splits

        // Boost confidence if split functions have related names
        let related_names = split_functions
            .iter()
            .filter(|(name, _)| self.is_potential_split_function(&source_sig.name, name))
            .count();

        confidence += (related_names as f64 / split_functions.len() as f64) * 0.2;

        // Boost confidence if functions are in related files
        let related_files = split_functions
            .iter()
            .filter(|(_, file)| self.are_files_related(&source_sig.file_path, file))
            .count();

        confidence += (related_files as f64 / split_functions.len() as f64) * 0.1;

        confidence.min(1.0)
    }

    /// Calculate confidence for merge detection
    fn calculate_merge_confidence(
        &self,
        source_functions: &[(String, String)],
        target_sig: &EnhancedFunctionSignature,
        combined_similarity: f64,
    ) -> f64 {
        let mut confidence = combined_similarity * 0.6; // Lower base confidence for merges

        // Boost confidence if source functions have related names
        let related_names = source_functions
            .iter()
            .filter(|(name, _)| self.is_potential_merge_function(name, &target_sig.name))
            .count();

        confidence += (related_names as f64 / source_functions.len() as f64) * 0.2;

        // Boost confidence if functions are from related files
        let related_files = source_functions
            .iter()
            .filter(|(_, file)| self.are_files_related(file, &target_sig.file_path))
            .count();

        confidence += (related_files as f64 / source_functions.len() as f64) * 0.1;

        confidence.min(1.0)
    }

    /// Check if two files are related (same directory, similar names, etc.)
    fn are_files_related(&self, file1: &str, file2: &str) -> bool {
        // Extract directory paths
        let dir1 = std::path::Path::new(file1).parent();
        let dir2 = std::path::Path::new(file2).parent();

        // Same directory
        if dir1 == dir2 {
            return true;
        }

        // Similar file names
        let name1 = std::path::Path::new(file1).file_stem().unwrap_or_default();
        let name2 = std::path::Path::new(file2).file_stem().unwrap_or_default();

        if let (Some(n1), Some(n2)) = (name1.to_str(), name2.to_str()) {
            // Check if one name contains the other
            if n1.contains(n2) || n2.contains(n1) {
                return true;
            }

            // Check for common prefixes/suffixes
            if n1.len() > 3 && n2.len() > 3 {
                let prefix1 = &n1[..3];
                let prefix2 = &n2[..3];
                if prefix1 == prefix2 {
                    return true;
                }
            }
        }

        false
    }

    /// Check if a function name suggests it's a split from another function
    fn is_potential_split_function(&self, original_name: &str, split_name: &str) -> bool {
        let original_lower = original_name.to_lowercase();
        let split_lower = split_name.to_lowercase();

        // Check if split name contains original name
        if split_lower.contains(&original_lower) || original_lower.contains(&split_lower) {
            return true;
        }

        // Check for common split patterns
        let split_patterns = [
            "part", "step", "phase", "stage", "helper", "util", "validate", "process", "handle",
        ];
        for pattern in &split_patterns {
            if split_lower.contains(pattern) && original_lower.len() > 5 {
                // Check if they share a common root
                let original_root = &original_lower[..original_lower.len().min(5)];
                if split_lower.contains(original_root) {
                    return true;
                }
            }
        }

        false
    }

    /// Check if a function name suggests it's part of a merge
    fn is_potential_merge_function(&self, source_name: &str, merged_name: &str) -> bool {
        let source_lower = source_name.to_lowercase();
        let merged_lower = merged_name.to_lowercase();

        // Check if merged name contains source name
        if merged_lower.contains(&source_lower) || source_lower.contains(&merged_lower) {
            return true;
        }

        // Check for common merge patterns
        let merge_patterns = [
            "combined",
            "unified",
            "merged",
            "consolidated",
            "integrated",
        ];
        for pattern in &merge_patterns {
            if merged_lower.contains(pattern) {
                return true;
            }
        }

        false
    }

    /// Check if a symbol is referenced across files using global symbol table
    fn is_symbol_referenced_across_files(
        &self,
        _resolver: &SymbolResolver,
        _symbol_name: &str,
        _target_file: &str,
    ) -> bool {
        // This would use the symbol resolver to check cross-file references
        // For now, return false as a placeholder
        false
    }

    /// Calculate file-level statistics
    fn calculate_file_statistics(
        &self,
        source_files: &HashMap<String, Vec<(EnhancedFunctionSignature, ASTNode)>>,
        target_files: &HashMap<String, Vec<(EnhancedFunctionSignature, ASTNode)>>,
        result: &CrossFileTrackingResult,
    ) -> HashMap<String, FileTrackingStats> {
        let mut stats = HashMap::new();

        // Initialize stats for all files
        let all_files: HashSet<String> = source_files
            .keys()
            .chain(target_files.keys())
            .cloned()
            .collect();

        for file_path in all_files {
            let source_count = source_files.get(&file_path).map(|f| f.len()).unwrap_or(0);
            let target_count = target_files.get(&file_path).map(|f| f.len()).unwrap_or(0);

            let mut moved_out = 0;
            let mut moved_in = 0;
            let mut renamed = 0;

            // Count moves out of this file
            for move_item in &result.moved_functions {
                if move_item.source_file == file_path {
                    moved_out += 1;
                }
                if move_item.target_file == file_path {
                    moved_in += 1;
                }
            }

            // Count renames with moves
            for rename_move in &result.renamed_and_moved {
                if rename_move.source_file == file_path {
                    moved_out += 1;
                    renamed += 1;
                }
                if rename_move.target_file == file_path {
                    moved_in += 1;
                    renamed += 1;
                }
            }

            stats.insert(
                file_path,
                FileTrackingStats {
                    total_functions: source_count.max(target_count),
                    functions_moved_out: moved_out,
                    functions_moved_in: moved_in,
                    functions_renamed: renamed,
                    net_function_change: target_count as i32 - source_count as i32,
                },
            );
        }

        stats
    }

    /// Calculate overall statistics
    fn calculate_overall_statistics(
        &self,
        source_files: &HashMap<String, Vec<(EnhancedFunctionSignature, ASTNode)>>,
        target_files: &HashMap<String, Vec<(EnhancedFunctionSignature, ASTNode)>>,
        result: &CrossFileTrackingResult,
        execution_time_ms: u64,
    ) -> CrossFileTrackingStats {
        let total_files = source_files.len().max(target_files.len());
        let total_source_functions: usize = source_files.values().map(|f| f.len()).sum();
        let total_target_functions: usize = target_files.values().map(|f| f.len()).sum();
        let total_functions = total_source_functions.max(total_target_functions);

        let total_moves = result.moved_functions.len();
        let total_rename_moves = result.renamed_and_moved.len();
        let total_splits = result.cross_file_splits.len();
        let total_merges = result.cross_file_merges.len();

        let move_percentage = if total_functions > 0 {
            ((total_moves + total_rename_moves) as f64 / total_functions as f64) * 100.0
        } else {
            0.0
        };

        // Calculate average confidence
        let mut total_confidence = 0.0;
        let mut confidence_count = 0;

        for move_item in &result.moved_functions {
            total_confidence += move_item.confidence;
            confidence_count += 1;
        }

        for rename_move in &result.renamed_and_moved {
            total_confidence += rename_move.confidence;
            confidence_count += 1;
        }

        for split in &result.cross_file_splits {
            total_confidence += split.confidence;
            confidence_count += 1;
        }

        for merge in &result.cross_file_merges {
            total_confidence += merge.confidence;
            confidence_count += 1;
        }

        let average_confidence = if confidence_count > 0 {
            total_confidence / confidence_count as f64
        } else {
            0.0
        };

        CrossFileTrackingStats {
            total_files,
            total_functions,
            total_moves,
            total_rename_moves,
            total_splits,
            total_merges,
            move_percentage,
            average_confidence,
            execution_time_ms,
        }
    }

    /// Get configuration
    pub fn get_config(&self) -> &CrossFileTrackerConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: CrossFileTrackerConfig) {
        // Update Hungarian matcher config before moving config
        let mut hungarian_config = self.hungarian_matcher.get_config().clone();
        hungarian_config.cross_file_penalty = config.cross_file_move_penalty;
        hungarian_config.min_similarity_threshold = config.min_cross_file_similarity;
        self.hungarian_matcher.set_config(hungarian_config);

        self.config = config;
    }

    /// Clear caches
    pub fn clear_caches(&mut self) {
        self.hungarian_matcher.clear_cache();
        self.similarity_scorer.clear_cache();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smart_diff_parser::NodeMetadata;
    use smart_diff_semantic::{FunctionType, TypeSignature, Visibility};
    use std::collections::HashMap;

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

    fn create_test_ast_node() -> ASTNode {
        ASTNode {
            node_type: smart_diff_parser::NodeType::Function,
            children: Vec::new(),
            metadata: NodeMetadata {
                line: 1,
                column: 1,
                attributes: HashMap::new(),
            },
        }
    }

    #[test]
    fn test_cross_file_tracker_config_default() {
        let config = CrossFileTrackerConfig::default();

        assert_eq!(config.min_cross_file_similarity, 0.8);
        assert_eq!(config.cross_file_move_penalty, 0.1);
        assert!(config.track_renames);
        assert!(config.track_splits_merges);
        assert_eq!(config.max_files_to_consider, 50);
        assert!(config.use_global_symbol_table);
        assert!(config.use_dependency_analysis);
    }

    #[test]
    fn test_cross_file_tracker_creation() {
        let config = CrossFileTrackerConfig::default();
        let tracker = CrossFileTracker::new(Language::Java, config);

        assert_eq!(tracker.config.min_cross_file_similarity, 0.8);
        assert_eq!(tracker.language, Language::Java);
    }

    #[test]
    fn test_move_type_classification() {
        let tracker = CrossFileTracker::with_defaults(Language::Java);

        // Create mock similarity scores
        let high_similarity = ComprehensiveSimilarityScore {
            overall_similarity: 0.96,
            signature_similarity: smart_diff_semantic::FunctionSignatureSimilarity {
                overall_similarity: 0.95,
                name_similarity: 1.0,
                parameter_similarity: 0.9,
                return_type_similarity: 1.0,
                modifier_similarity: 1.0,
                complexity_similarity: 0.8,
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
                overall_similarity: 0.97,
                structural_similarity: 0.98,
                content_similarity: 0.96,
                control_flow_similarity: 0.97,
                edit_distance_score: 0.95,
                depth_similarity: 0.99,
                node_count_similarity: 0.98,
            },
            context_similarity: crate::similarity_scorer::ContextSimilarityScore {
                overall_similarity: 0.95,
                function_call_similarity: 0.96,
                variable_usage_similarity: 0.94,
                dependency_similarity: 0.95,
                surrounding_code_similarity: 0.96,
                namespace_context_similarity: 0.94,
            },
            semantic_metrics: crate::similarity_scorer::SemanticSimilarityMetrics {
                type_usage_similarity: 0.95,
                api_pattern_similarity: 0.96,
                error_handling_similarity: 0.94,
                resource_management_similarity: 0.95,
                algorithm_pattern_similarity: 0.97,
            },
            confidence: 0.96,
            match_type: crate::similarity_scorer::MatchType::ExactMatch,
            similarity_breakdown: crate::similarity_scorer::DetailedSimilarityBreakdown {
                signature_components: HashMap::new(),
                ast_node_distribution: HashMap::new(),
                control_flow_patterns: Vec::new(),
                common_function_calls: Vec::new(),
                common_variables: Vec::new(),
                contributing_factors: Vec::new(),
                dissimilarity_factors: Vec::new(),
            },
        };

        assert_eq!(
            tracker.classify_move_type(&high_similarity),
            MoveType::SimpleMove
        );
    }

    #[test]
    fn test_file_relationship_detection() {
        let tracker = CrossFileTracker::with_defaults(Language::Java);

        // Same directory
        assert!(tracker.are_files_related("src/main/Calculator.java", "src/main/MathUtils.java"));

        // Similar names
        assert!(tracker.are_files_related("Calculator.java", "CalculatorUtils.java"));

        // Different directories and names
        assert!(
            !tracker.are_files_related("src/main/Calculator.java", "test/unit/DatabaseTest.java")
        );
    }

    #[test]
    fn test_split_function_detection() {
        let tracker = CrossFileTracker::with_defaults(Language::Java);

        // Clear split patterns
        assert!(tracker.is_potential_split_function("processData", "processDataPart1"));
        assert!(tracker.is_potential_split_function("calculateTotal", "calculateTotalHelper"));
        assert!(tracker.is_potential_split_function("validateInput", "validateInputStep1"));

        // Not split patterns
        assert!(!tracker.is_potential_split_function("processData", "formatOutput"));
        assert!(!tracker.is_potential_split_function("calculateTotal", "deleteRecord"));
    }

    #[test]
    fn test_merge_function_detection() {
        let tracker = CrossFileTracker::with_defaults(Language::Java);

        // Clear merge patterns
        assert!(tracker.is_potential_merge_function("validateInput", "combinedValidation"));
        assert!(tracker.is_potential_merge_function("processData", "unifiedDataProcessor"));
        assert!(tracker.is_potential_merge_function("formatOutput", "integratedFormatter"));

        // Not merge patterns
        assert!(!tracker.is_potential_merge_function("validateInput", "deleteRecord"));
        assert!(!tracker.is_potential_merge_function("processData", "calculateTotal"));
    }

    #[test]
    fn test_config_updates() {
        let mut tracker = CrossFileTracker::with_defaults(Language::Java);

        let original_threshold = tracker.get_config().min_cross_file_similarity;
        assert_eq!(original_threshold, 0.8);

        let new_config = CrossFileTrackerConfig {
            min_cross_file_similarity: 0.9,
            cross_file_move_penalty: 0.2,
            track_renames: false,
            track_splits_merges: false,
            max_files_to_consider: 25,
            use_global_symbol_table: false,
            use_dependency_analysis: false,
        };

        tracker.set_config(new_config);

        assert_eq!(tracker.get_config().min_cross_file_similarity, 0.9);
        assert_eq!(tracker.get_config().cross_file_move_penalty, 0.2);
        assert!(!tracker.get_config().track_renames);
        assert!(!tracker.get_config().track_splits_merges);
        assert_eq!(tracker.get_config().max_files_to_consider, 25);
        assert!(!tracker.get_config().use_global_symbol_table);
        assert!(!tracker.get_config().use_dependency_analysis);
    }
}
