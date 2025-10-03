//! Graph-based function matching algorithm
//!
//! This module implements an advanced graph-based algorithm that can match functions
//! across files regardless of their position, detect moved/renamed functions, and
//! create a dependency graph for intelligent comparison.

use crate::similarity_scorer::{
    ComprehensiveSimilarityScore, SimilarityScorer, SimilarityScoringConfig,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use smart_diff_parser::{ASTNode, Function, FunctionSignature, Language};
use smart_diff_semantic::{
    DependencyEdgeType, DependencyGraph, EnhancedFunctionSignature,
    FunctionParameter as SemanticFunctionParameter, FunctionType, GenericParameter,
    GenericVariance, TypeSignature as SemanticTypeSignature, Visibility,
};
use std::collections::HashSet;

/// Configuration for graph-based matching
#[derive(Debug, Clone)]
pub struct GraphMatcherConfig {
    /// Minimum similarity threshold for function matching
    pub similarity_threshold: f64,
    /// Weight for signature similarity in overall score
    pub signature_weight: f64,
    /// Weight for structural similarity in overall score
    pub structure_weight: f64,
    /// Weight for dependency context similarity in overall score
    pub context_weight: f64,
    /// Maximum depth for dependency context analysis
    pub max_context_depth: usize,
    /// Enable cross-file function tracking
    pub enable_cross_file_tracking: bool,
    /// Enable function move detection
    pub enable_move_detection: bool,
    /// Enable function rename detection
    pub enable_rename_detection: bool,
}

impl Default for GraphMatcherConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.7,
            signature_weight: 0.4,
            structure_weight: 0.4,
            context_weight: 0.2,
            max_context_depth: 3,
            enable_cross_file_tracking: true,
            enable_move_detection: true,
            enable_rename_detection: true,
        }
    }
}

/// Graph-based function matcher
pub struct GraphMatcher {
    config: GraphMatcherConfig,
    similarity_scorer: SimilarityScorer,
    language: Language,
}

/// Function node in the dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionNode {
    pub id: String,
    pub signature: FunctionSignature,
    pub ast: ASTNode,
    pub file_path: String,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub context_hash: String,
}

/// Graph-based match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMatchResult {
    pub matches: Vec<FunctionMatch>,
    pub moves: Vec<FunctionMove>,
    pub renames: Vec<FunctionRename>,
    pub additions: Vec<String>,
    pub deletions: Vec<String>,
    pub overall_similarity: f64,
    pub dependency_changes: Vec<DependencyChange>,
}

/// Individual function match with graph context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMatch {
    pub source_id: String,
    pub target_id: String,
    pub similarity: ComprehensiveSimilarityScore,
    pub context_similarity: f64,
    pub confidence: f64,
    pub match_type: MatchType,
}

/// Function move detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMove {
    pub function_id: String,
    pub source_file: String,
    pub target_file: String,
    pub similarity: f64,
    pub confidence: f64,
}

/// Function rename detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRename {
    pub old_name: String,
    pub new_name: String,
    pub function_id: String,
    pub similarity: f64,
    pub confidence: f64,
}

/// Dependency change in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyChange {
    pub change_type: DependencyChangeType,
    pub source_function: String,
    pub target_function: String,
    pub edge_type: DependencyEdgeType,
    pub strength_change: f64,
}

/// Types of dependency changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyChangeType {
    Added,
    Removed,
    Modified,
    Strengthened,
    Weakened,
}

/// Types of function matches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchType {
    Exact,
    Similar,
    Renamed,
    Moved,
    MovedAndRenamed,
    Refactored,
}

impl GraphMatcher {
    pub fn new(language: Language, config: GraphMatcherConfig) -> Self {
        let scoring_config = SimilarityScoringConfig::default();
        let similarity_scorer = SimilarityScorer::new(language, scoring_config);

        Self {
            config,
            similarity_scorer,
            language,
        }
    }

    /// Match functions using graph-based analysis
    pub fn match_functions(
        &mut self,
        source_functions: &[Function],
        target_functions: &[Function],
        source_dependency_graph: &DependencyGraph,
        target_dependency_graph: &DependencyGraph,
    ) -> Result<GraphMatchResult> {
        // Build function nodes with dependency context
        let source_nodes = self.build_function_nodes(source_functions, source_dependency_graph)?;
        let target_nodes = self.build_function_nodes(target_functions, target_dependency_graph)?;

        // Calculate comprehensive similarity matrix including graph context
        let similarity_matrix =
            self.calculate_graph_similarity_matrix(&source_nodes, &target_nodes)?;

        // Perform optimal matching using enhanced Hungarian algorithm
        let matches =
            self.perform_graph_matching(&source_nodes, &target_nodes, &similarity_matrix)?;

        // Detect moves and renames
        let moves = if self.config.enable_move_detection {
            self.detect_function_moves(&source_nodes, &target_nodes, &matches)?
        } else {
            Vec::new()
        };

        let renames = if self.config.enable_rename_detection {
            self.detect_function_renames(&source_nodes, &target_nodes, &matches)?
        } else {
            Vec::new()
        };

        // Identify additions and deletions
        let (additions, deletions) =
            self.identify_additions_deletions(&source_nodes, &target_nodes, &matches);

        // Analyze dependency changes
        let dependency_changes = self.analyze_dependency_changes(
            source_dependency_graph,
            target_dependency_graph,
            &matches,
        )?;

        // Calculate overall similarity
        let overall_similarity =
            self.calculate_overall_similarity(&matches, &source_nodes, &target_nodes);

        Ok(GraphMatchResult {
            matches,
            moves,
            renames,
            additions,
            deletions,
            overall_similarity,
            dependency_changes,
        })
    }

    /// Build function nodes with dependency context
    fn build_function_nodes(
        &self,
        functions: &[Function],
        dependency_graph: &DependencyGraph,
    ) -> Result<Vec<FunctionNode>> {
        let mut nodes = Vec::new();

        for function in functions {
            let dependencies: Vec<String> = dependency_graph
                .get_dependencies(&function.hash)
                .iter()
                .map(|dep| dep.id.clone())
                .collect();

            let dependents: Vec<String> = dependency_graph
                .get_dependents(&function.hash)
                .iter()
                .map(|dep| dep.id.clone())
                .collect();

            let context_hash = self.calculate_context_hash(&dependencies, &dependents);

            let node = FunctionNode {
                id: function.hash.clone(),
                signature: function.signature.clone(),
                ast: function.body.clone(),
                file_path: function.location.file_path.clone(),
                dependencies,
                dependents,
                context_hash,
            };

            nodes.push(node);
        }

        Ok(nodes)
    }

    /// Calculate context hash for dependency fingerprinting
    fn calculate_context_hash(&self, dependencies: &[String], dependents: &[String]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        // Sort to ensure consistent hashing regardless of order
        let mut sorted_deps = dependencies.to_vec();
        sorted_deps.sort();
        sorted_deps.hash(&mut hasher);

        let mut sorted_dependents = dependents.to_vec();
        sorted_dependents.sort();
        sorted_dependents.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }

    /// Calculate comprehensive similarity matrix including graph context
    fn calculate_graph_similarity_matrix(
        &mut self,
        source_nodes: &[FunctionNode],
        target_nodes: &[FunctionNode],
    ) -> Result<Vec<Vec<f64>>> {
        let mut matrix = Vec::with_capacity(source_nodes.len());

        for source_node in source_nodes {
            let mut row = Vec::with_capacity(target_nodes.len());

            for target_node in target_nodes {
                let similarity =
                    self.calculate_comprehensive_similarity(source_node, target_node)?;
                row.push(similarity);
            }

            matrix.push(row);
        }

        Ok(matrix)
    }

    /// Calculate comprehensive similarity between two function nodes
    fn calculate_comprehensive_similarity(
        &mut self,
        source: &FunctionNode,
        target: &FunctionNode,
    ) -> Result<f64> {
        // Signature similarity
        let signature_similarity =
            self.calculate_signature_similarity(&source.signature, &target.signature);

        // Structural similarity using AST comparison
        let source_enhanced = self.convert_to_enhanced_signature(&source.signature);
        let target_enhanced = self.convert_to_enhanced_signature(&target.signature);
        let structure_similarity = self.similarity_scorer.calculate_comprehensive_similarity(
            &source_enhanced,
            &source.ast,
            &target_enhanced,
            &target.ast,
        )?;

        // Context similarity based on dependency patterns
        let context_similarity = self.calculate_context_similarity(source, target);

        // Weighted combination
        let overall_similarity = signature_similarity * self.config.signature_weight
            + structure_similarity.overall_similarity * self.config.structure_weight
            + context_similarity * self.config.context_weight;

        Ok(overall_similarity)
    }

    /// Calculate signature similarity
    fn calculate_signature_similarity(
        &self,
        source: &FunctionSignature,
        target: &FunctionSignature,
    ) -> f64 {
        let mut similarity = 0.0;
        let mut total_weight = 0.0;

        // Name similarity (40% weight)
        let name_weight = 0.4;
        let name_similarity = self.calculate_string_similarity(&source.name, &target.name);
        similarity += name_similarity * name_weight;
        total_weight += name_weight;

        // Parameter similarity (35% weight)
        let param_weight = 0.35;
        let param_names: Vec<String> = source.parameters.iter().map(|p| p.name.clone()).collect();
        let target_param_names: Vec<String> =
            target.parameters.iter().map(|p| p.name.clone()).collect();
        let param_similarity =
            self.calculate_parameter_similarity(&param_names, &target_param_names);
        similarity += param_similarity * param_weight;
        total_weight += param_weight;

        // Return type similarity (15% weight)
        let return_weight = 0.15;
        let source_return_type = source.return_type.as_ref().map(|t| t.name.clone());
        let target_return_type = target.return_type.as_ref().map(|t| t.name.clone());
        let return_similarity =
            self.calculate_type_similarity(&source_return_type, &target_return_type);
        similarity += return_similarity * return_weight;
        total_weight += return_weight;

        // Modifier similarity (10% weight)
        let modifier_weight = 0.1;
        let modifier_similarity =
            self.calculate_modifier_similarity(&source.modifiers, &target.modifiers);
        similarity += modifier_similarity * modifier_weight;
        total_weight += modifier_weight;

        if total_weight > 0.0 {
            similarity / total_weight
        } else {
            0.0
        }
    }

    /// Calculate context similarity based on dependency patterns
    fn calculate_context_similarity(&self, source: &FunctionNode, target: &FunctionNode) -> f64 {
        // Quick check: if context hashes are identical, return 1.0
        if source.context_hash == target.context_hash {
            return 1.0;
        }

        let mut similarity = 0.0;
        let mut total_weight = 0.0;

        // Dependency similarity (50% weight)
        let dep_weight = 0.5;
        let dep_similarity =
            self.calculate_dependency_similarity(&source.dependencies, &target.dependencies);
        similarity += dep_similarity * dep_weight;
        total_weight += dep_weight;

        // Dependent similarity (50% weight)
        let dependent_weight = 0.5;
        let dependent_similarity =
            self.calculate_dependency_similarity(&source.dependents, &target.dependents);
        similarity += dependent_similarity * dependent_weight;
        total_weight += dependent_weight;

        if total_weight > 0.0 {
            similarity / total_weight
        } else {
            0.0
        }
    }

    /// Calculate similarity between two dependency lists
    fn calculate_dependency_similarity(&self, deps1: &[String], deps2: &[String]) -> f64 {
        if deps1.is_empty() && deps2.is_empty() {
            return 1.0;
        }

        if deps1.is_empty() || deps2.is_empty() {
            return 0.0;
        }

        let set1: HashSet<_> = deps1.iter().collect();
        let set2: HashSet<_> = deps2.iter().collect();

        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();

        if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        }
    }

    /// Calculate string similarity using edit distance
    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }

        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        let distance = edit_distance::edit_distance(s1, s2);
        let max_len = s1.len().max(s2.len());

        if max_len > 0 {
            1.0 - (distance as f64 / max_len as f64)
        } else {
            0.0
        }
    }

    /// Calculate parameter similarity
    fn calculate_parameter_similarity(&self, params1: &[String], params2: &[String]) -> f64 {
        if params1.len() != params2.len() {
            // Different parameter counts reduce similarity significantly
            let len_diff = (params1.len() as i32 - params2.len() as i32).abs() as f64;
            let max_len = params1.len().max(params2.len()) as f64;
            let len_penalty = if max_len > 0.0 {
                len_diff / max_len
            } else {
                0.0
            };

            // Still compare common parameters
            let common_len = params1.len().min(params2.len());
            let mut param_similarity = 0.0;

            for i in 0..common_len {
                param_similarity += self.calculate_string_similarity(&params1[i], &params2[i]);
            }

            if common_len > 0 {
                param_similarity /= common_len as f64;
            }

            // Apply length penalty
            param_similarity * (1.0 - len_penalty * 0.5)
        } else {
            // Same parameter count, compare each parameter
            let mut total_similarity = 0.0;

            for (p1, p2) in params1.iter().zip(params2.iter()) {
                total_similarity += self.calculate_string_similarity(p1, p2);
            }

            if !params1.is_empty() {
                total_similarity / params1.len() as f64
            } else {
                1.0 // Both empty
            }
        }
    }

    /// Calculate type similarity
    fn calculate_type_similarity(&self, type1: &Option<String>, type2: &Option<String>) -> f64 {
        match (type1, type2) {
            (Some(t1), Some(t2)) => self.calculate_string_similarity(t1, t2),
            (None, None) => 1.0,
            _ => 0.0,
        }
    }

    /// Calculate modifier similarity
    fn calculate_modifier_similarity(&self, mods1: &[String], mods2: &[String]) -> f64 {
        let set1: HashSet<_> = mods1.iter().collect();
        let set2: HashSet<_> = mods2.iter().collect();

        if set1.is_empty() && set2.is_empty() {
            return 1.0;
        }

        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();

        if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        }
    }

    /// Perform optimal matching using enhanced Hungarian algorithm
    fn perform_graph_matching(
        &mut self,
        source_nodes: &[FunctionNode],
        target_nodes: &[FunctionNode],
        similarity_matrix: &[Vec<f64>],
    ) -> Result<Vec<FunctionMatch>> {
        use hungarian::minimize;

        if source_nodes.is_empty() || target_nodes.is_empty() {
            return Ok(Vec::new());
        }

        // Convert similarity to cost matrix
        let flat_cost_matrix: Vec<i32> = similarity_matrix
            .iter()
            .flat_map(|row| {
                row.iter().map(|&similarity| {
                    let cost = 1.0 - similarity;
                    if similarity < self.config.similarity_threshold {
                        i32::MAX // Exclude assignments below threshold
                    } else {
                        (cost * 10000.0) as i32 // Scale for integer representation
                    }
                })
            })
            .collect();

        let height = similarity_matrix.len();
        let width = similarity_matrix[0].len();

        // Solve the assignment problem
        let assignments = minimize(&flat_cost_matrix, height, width);

        // Convert to function matches
        let mut matches = Vec::new();

        for (source_idx, target_idx_opt) in assignments.into_iter().enumerate() {
            if let Some(target_idx) = target_idx_opt {
                if target_idx < width
                    && similarity_matrix[source_idx][target_idx] >= self.config.similarity_threshold
                {
                    let source_node = &source_nodes[source_idx];
                    let target_node = &target_nodes[target_idx];
                    let similarity = similarity_matrix[source_idx][target_idx];

                    // Calculate detailed similarity scores
                    let source_enhanced =
                        self.convert_to_enhanced_signature(&source_node.signature);
                    let target_enhanced =
                        self.convert_to_enhanced_signature(&target_node.signature);
                    let comprehensive_similarity =
                        self.similarity_scorer.calculate_comprehensive_similarity(
                            &source_enhanced,
                            &source_node.ast,
                            &target_enhanced,
                            &target_node.ast,
                        )?;
                    let context_similarity =
                        self.calculate_context_similarity(source_node, target_node);

                    // Determine match type
                    let match_type =
                        self.determine_match_type(source_node, target_node, similarity);

                    let function_match = FunctionMatch {
                        source_id: source_node.id.clone(),
                        target_id: target_node.id.clone(),
                        similarity: comprehensive_similarity,
                        context_similarity,
                        confidence: similarity,
                        match_type,
                    };

                    matches.push(function_match);
                }
            }
        }

        Ok(matches)
    }

    /// Determine the type of match based on similarity characteristics
    fn determine_match_type(
        &self,
        source: &FunctionNode,
        target: &FunctionNode,
        similarity: f64,
    ) -> MatchType {
        if similarity >= 0.99 {
            MatchType::Exact
        } else if source.signature.name != target.signature.name
            && source.file_path != target.file_path
        {
            MatchType::MovedAndRenamed
        } else if source.signature.name != target.signature.name {
            MatchType::Renamed
        } else if source.file_path != target.file_path {
            MatchType::Moved
        } else if similarity >= 0.9 {
            MatchType::Similar
        } else {
            MatchType::Refactored
        }
    }

    /// Detect function moves across files
    fn detect_function_moves(
        &self,
        source_nodes: &[FunctionNode],
        target_nodes: &[FunctionNode],
        matches: &[FunctionMatch],
    ) -> Result<Vec<FunctionMove>> {
        let mut moves = Vec::new();

        for function_match in matches {
            let source_node = source_nodes
                .iter()
                .find(|n| n.id == function_match.source_id);
            let target_node = target_nodes
                .iter()
                .find(|n| n.id == function_match.target_id);

            if let (Some(source), Some(target)) = (source_node, target_node) {
                if source.file_path != target.file_path && function_match.confidence >= 0.8 {
                    let function_move = FunctionMove {
                        function_id: source.id.clone(),
                        source_file: source.file_path.clone(),
                        target_file: target.file_path.clone(),
                        similarity: function_match.confidence,
                        confidence: function_match.confidence,
                    };

                    moves.push(function_move);
                }
            }
        }

        Ok(moves)
    }

    /// Detect function renames
    fn detect_function_renames(
        &self,
        source_nodes: &[FunctionNode],
        target_nodes: &[FunctionNode],
        matches: &[FunctionMatch],
    ) -> Result<Vec<FunctionRename>> {
        let mut renames = Vec::new();

        for function_match in matches {
            let source_node = source_nodes
                .iter()
                .find(|n| n.id == function_match.source_id);
            let target_node = target_nodes
                .iter()
                .find(|n| n.id == function_match.target_id);

            if let (Some(source), Some(target)) = (source_node, target_node) {
                if source.signature.name != target.signature.name
                    && function_match.confidence >= 0.7
                {
                    let function_rename = FunctionRename {
                        old_name: source.signature.name.clone(),
                        new_name: target.signature.name.clone(),
                        function_id: source.id.clone(),
                        similarity: function_match.confidence,
                        confidence: function_match.confidence,
                    };

                    renames.push(function_rename);
                }
            }
        }

        Ok(renames)
    }

    /// Identify additions and deletions
    fn identify_additions_deletions(
        &self,
        source_nodes: &[FunctionNode],
        target_nodes: &[FunctionNode],
        matches: &[FunctionMatch],
    ) -> (Vec<String>, Vec<String>) {
        let matched_source: HashSet<_> = matches.iter().map(|m| &m.source_id).collect();
        let matched_target: HashSet<_> = matches.iter().map(|m| &m.target_id).collect();

        let deletions = source_nodes
            .iter()
            .filter(|node| !matched_source.contains(&node.id))
            .map(|node| node.id.clone())
            .collect();

        let additions = target_nodes
            .iter()
            .filter(|node| !matched_target.contains(&node.id))
            .map(|node| node.id.clone())
            .collect();

        (additions, deletions)
    }

    /// Analyze dependency changes between graphs
    fn analyze_dependency_changes(
        &self,
        source_graph: &DependencyGraph,
        target_graph: &DependencyGraph,
        matches: &[FunctionMatch],
    ) -> Result<Vec<DependencyChange>> {
        let mut changes = Vec::new();

        // For each matched function, compare its dependencies
        for function_match in matches {
            let source_deps = source_graph.get_dependencies(&function_match.source_id);
            let target_deps = target_graph.get_dependencies(&function_match.target_id);

            // Find added dependencies
            for target_dep in &target_deps {
                if !source_deps.iter().any(|sd| sd.id == target_dep.id) {
                    changes.push(DependencyChange {
                        change_type: DependencyChangeType::Added,
                        source_function: function_match.source_id.clone(),
                        target_function: target_dep.id.clone(),
                        edge_type: DependencyEdgeType::Uses, // Simplified
                        strength_change: 1.0,
                    });
                }
            }

            // Find removed dependencies
            for source_dep in &source_deps {
                if !target_deps.iter().any(|td| td.id == source_dep.id) {
                    changes.push(DependencyChange {
                        change_type: DependencyChangeType::Removed,
                        source_function: function_match.source_id.clone(),
                        target_function: source_dep.id.clone(),
                        edge_type: DependencyEdgeType::Uses, // Simplified
                        strength_change: -1.0,
                    });
                }
            }
        }

        Ok(changes)
    }

    /// Calculate overall similarity score
    fn calculate_overall_similarity(
        &self,
        matches: &[FunctionMatch],
        source_nodes: &[FunctionNode],
        target_nodes: &[FunctionNode],
    ) -> f64 {
        if source_nodes.is_empty() && target_nodes.is_empty() {
            return 1.0;
        }

        if matches.is_empty() {
            return 0.0;
        }

        let total_similarity: f64 = matches.iter().map(|m| m.confidence).sum();
        let max_functions = source_nodes.len().max(target_nodes.len()) as f64;

        if max_functions > 0.0 {
            total_similarity / max_functions
        } else {
            0.0
        }
    }

    /// Convert FunctionSignature to EnhancedFunctionSignature for compatibility
    fn convert_to_enhanced_signature(
        &self,
        signature: &FunctionSignature,
    ) -> EnhancedFunctionSignature {
        let semantic_params: Vec<SemanticFunctionParameter> = signature
            .parameters
            .iter()
            .enumerate()
            .map(|(i, p)| SemanticFunctionParameter {
                name: p.name.clone(),
                param_type: SemanticTypeSignature::new(p.param_type.name.clone()),
                default_value: p.default_value.clone(),
                is_optional: p.default_value.is_some(),
                is_varargs: p.is_variadic,
                annotations: Vec::new(),
                position: i,
            })
            .collect();

        EnhancedFunctionSignature {
            name: signature.name.clone(),
            qualified_name: signature.name.clone(),
            parameters: semantic_params,
            return_type: signature
                .return_type
                .as_ref()
                .map(|t| SemanticTypeSignature::new(t.name.clone()))
                .unwrap_or_else(|| SemanticTypeSignature::new("void".to_string())),
            generic_parameters: signature
                .generic_parameters
                .iter()
                .map(|g| GenericParameter {
                    name: g.clone(),
                    bounds: Vec::new(),
                    variance: GenericVariance::Invariant,
                })
                .collect(),
            visibility: Visibility::Public,
            modifiers: signature.modifiers.clone(),
            annotations: Vec::new(),
            file_path: "unknown".to_string(),
            line: 0,
            column: 0,
            end_line: 0,
            function_type: FunctionType::Function,
            complexity_metrics: None,
            dependencies: Vec::new(),
            signature_hash: format!("{}_hash", signature.name),
            normalized_hash: format!("{}_normalized", signature.name),
        }
    }
}
