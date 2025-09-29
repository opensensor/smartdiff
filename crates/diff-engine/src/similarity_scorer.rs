//! Comprehensive similarity scoring algorithm for code comparison

use crate::tree_edit::{EditCost, TreeEditDistance, ZhangShashaConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use smart_diff_parser::{ASTNode, Language, NodeType};
use smart_diff_semantic::{
    EnhancedFunctionSignature, FunctionSignatureExtractor, FunctionSignatureSimilarity,
};
use std::collections::{HashMap, HashSet};

/// Configuration for similarity scoring
#[derive(Debug, Clone)]
pub struct SimilarityScoringConfig {
    /// Weight for signature similarity (default: 0.4)
    pub signature_weight: f64,
    /// Weight for AST body similarity (default: 0.4)
    pub body_weight: f64,
    /// Weight for context similarity (default: 0.2)
    pub context_weight: f64,
    /// Minimum similarity threshold for matches (default: 0.7)
    pub match_threshold: f64,
    /// Enable advanced AST comparison (default: true)
    pub enable_advanced_ast_comparison: bool,
    /// Enable semantic context analysis (default: true)
    pub enable_semantic_context: bool,
    /// Enable cross-language normalization (default: false)
    pub enable_cross_language: bool,
    /// Maximum AST depth for comparison (default: 10)
    pub max_ast_depth: usize,
    /// Edit distance costs for AST comparison
    pub edit_costs: EditCost,
}

impl Default for SimilarityScoringConfig {
    fn default() -> Self {
        Self {
            signature_weight: 0.4,
            body_weight: 0.4,
            context_weight: 0.2,
            match_threshold: 0.7,
            enable_advanced_ast_comparison: true,
            enable_semantic_context: true,
            enable_cross_language: false,
            max_ast_depth: 10,
            edit_costs: EditCost::default(),
        }
    }
}

/// Comprehensive similarity scorer
pub struct SimilarityScorer {
    config: SimilarityScoringConfig,
    #[allow(dead_code)]
    language: Language,
    signature_extractor: FunctionSignatureExtractor,
    tree_edit_calculator: TreeEditDistance,
    context_cache: HashMap<String, ContextInfo>,
}

/// Comprehensive similarity score with detailed breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveSimilarityScore {
    /// Overall weighted similarity score
    pub overall_similarity: f64,

    /// Signature similarity breakdown
    pub signature_similarity: FunctionSignatureSimilarity,

    /// AST body similarity details
    pub body_similarity: ASTSimilarityScore,

    /// Context similarity details
    pub context_similarity: ContextSimilarityScore,

    /// Additional semantic metrics
    pub semantic_metrics: SemanticSimilarityMetrics,

    /// Confidence score for the match
    pub confidence: f64,

    /// Match classification
    pub match_type: MatchType,

    /// Detailed similarity breakdown
    pub similarity_breakdown: DetailedSimilarityBreakdown,
}

/// AST similarity score with structural analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASTSimilarityScore {
    /// Overall AST similarity
    pub overall_similarity: f64,
    /// Structural similarity (node types and hierarchy)
    pub structural_similarity: f64,
    /// Content similarity (identifiers, literals)
    pub content_similarity: f64,
    /// Control flow similarity
    pub control_flow_similarity: f64,
    /// Edit distance normalized score
    pub edit_distance_score: f64,
    /// Tree depth comparison
    pub depth_similarity: f64,
    /// Node count similarity
    pub node_count_similarity: f64,
}

/// Context similarity score with semantic analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSimilarityScore {
    /// Overall context similarity
    pub overall_similarity: f64,
    /// Function call similarity
    pub function_call_similarity: f64,
    /// Variable usage similarity
    pub variable_usage_similarity: f64,
    /// Import/dependency similarity
    pub dependency_similarity: f64,
    /// Surrounding code similarity
    pub surrounding_code_similarity: f64,
    /// Class/namespace context similarity
    pub namespace_context_similarity: f64,
}

/// Semantic similarity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSimilarityMetrics {
    /// Type usage similarity
    pub type_usage_similarity: f64,
    /// API pattern similarity
    pub api_pattern_similarity: f64,
    /// Error handling similarity
    pub error_handling_similarity: f64,
    /// Resource management similarity
    pub resource_management_similarity: f64,
    /// Algorithm pattern similarity
    pub algorithm_pattern_similarity: f64,
}

/// Match type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchType {
    /// Exact match (similarity >= 0.95)
    ExactMatch,
    /// High similarity match (0.85 <= similarity < 0.95)
    HighSimilarity,
    /// Potential match (0.7 <= similarity < 0.85)
    PotentialMatch,
    /// Weak match (0.5 <= similarity < 0.7)
    WeakMatch,
    /// Potential refactoring (different signature, similar body)
    PotentialRefactoring,
    /// Potential rename (similar signature, different name)
    PotentialRename,
    /// No match (similarity < 0.5)
    NoMatch,
}

/// Detailed similarity breakdown for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedSimilarityBreakdown {
    /// Signature components breakdown
    pub signature_components: HashMap<String, f64>,
    /// AST node type distribution similarity
    pub ast_node_distribution: HashMap<String, f64>,
    /// Control flow pattern matches
    pub control_flow_patterns: Vec<String>,
    /// Common function calls
    pub common_function_calls: Vec<String>,
    /// Common variable names
    pub common_variables: Vec<String>,
    /// Similarity contributing factors
    pub contributing_factors: Vec<SimilarityFactor>,
    /// Dissimilarity factors
    pub dissimilarity_factors: Vec<SimilarityFactor>,
}

/// Factor contributing to similarity or dissimilarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityFactor {
    pub factor_type: String,
    pub description: String,
    pub impact: f64,
    pub confidence: f64,
}

/// Context information for caching
#[derive(Debug, Clone)]
struct ContextInfo {
    function_calls: HashSet<String>,
    variable_names: HashSet<String>,
    type_usage: HashSet<String>,
    #[allow(dead_code)]
    control_flow_patterns: Vec<String>,
    dependencies: HashSet<String>,
}

impl SimilarityScorer {
    pub fn new(language: Language, config: SimilarityScoringConfig) -> Self {
        let signature_extractor = FunctionSignatureExtractor::with_defaults(language);
        let zhang_shasha_config = ZhangShashaConfig {
            insert_cost: config.edit_costs.insert,
            delete_cost: config.edit_costs.delete,
            update_cost: config.edit_costs.update,
            ..Default::default()
        };
        let tree_edit_calculator = TreeEditDistance::new(zhang_shasha_config);

        Self {
            config,
            language,
            signature_extractor,
            tree_edit_calculator,
            context_cache: HashMap::new(),
        }
    }

    pub fn with_defaults(language: Language) -> Self {
        Self::new(language, SimilarityScoringConfig::default())
    }

    /// Calculate comprehensive similarity between two functions
    pub fn calculate_comprehensive_similarity(
        &mut self,
        func1_signature: &EnhancedFunctionSignature,
        func1_ast: &ASTNode,
        func2_signature: &EnhancedFunctionSignature,
        func2_ast: &ASTNode,
    ) -> Result<ComprehensiveSimilarityScore> {
        // Calculate signature similarity
        let signature_similarity = self
            .signature_extractor
            .calculate_similarity(func1_signature, func2_signature);

        // Calculate AST body similarity
        let body_similarity = self.calculate_ast_similarity(func1_ast, func2_ast)?;

        // Calculate context similarity
        let context_similarity = self.calculate_context_similarity(
            func1_signature,
            func1_ast,
            func2_signature,
            func2_ast,
        )?;

        // Calculate semantic metrics
        let semantic_metrics = self.calculate_semantic_metrics(func1_ast, func2_ast)?;

        // Calculate overall weighted similarity
        let overall_similarity = signature_similarity.overall_similarity
            * self.config.signature_weight
            + body_similarity.overall_similarity * self.config.body_weight
            + context_similarity.overall_similarity * self.config.context_weight;

        // Determine match type
        let match_type =
            self.classify_match_type(&signature_similarity, &body_similarity, overall_similarity);

        // Calculate confidence score
        let confidence = self.calculate_confidence_score(
            &signature_similarity,
            &body_similarity,
            &context_similarity,
        );

        // Build detailed breakdown
        let similarity_breakdown = self.build_detailed_breakdown(
            func1_signature,
            func1_ast,
            func2_signature,
            func2_ast,
            &signature_similarity,
            &body_similarity,
            &context_similarity,
        )?;

        Ok(ComprehensiveSimilarityScore {
            overall_similarity,
            signature_similarity,
            body_similarity,
            context_similarity,
            semantic_metrics,
            confidence,
            match_type,
            similarity_breakdown,
        })
    }

    /// Calculate advanced AST similarity with structural analysis
    fn calculate_ast_similarity(
        &self,
        ast1: &ASTNode,
        ast2: &ASTNode,
    ) -> Result<ASTSimilarityScore> {
        // Structural similarity (node types and hierarchy)
        let structural_similarity = self.calculate_structural_similarity(ast1, ast2, 0)?;

        // Content similarity (identifiers, literals, etc.)
        let content_similarity = self.calculate_content_similarity(ast1, ast2)?;

        // Control flow similarity
        let control_flow_similarity = self.calculate_control_flow_similarity(ast1, ast2)?;

        // Edit distance based similarity
        let edit_distance = self.tree_edit_calculator.calculate_distance(ast1, ast2);
        let max_nodes = self.count_nodes(ast1).max(self.count_nodes(ast2)) as f64;
        let edit_distance_score = if max_nodes > 0.0 {
            (1.0 - (edit_distance / max_nodes)).max(0.0)
        } else {
            1.0
        };

        // Tree depth similarity
        let depth1 = self.calculate_tree_depth(ast1);
        let depth2 = self.calculate_tree_depth(ast2);
        let depth_similarity = 1.0 - ((depth1 as i32 - depth2 as i32).abs() as f64 / 20.0).min(1.0);

        // Node count similarity
        let count1 = self.count_nodes(ast1) as f64;
        let count2 = self.count_nodes(ast2) as f64;
        let node_count_similarity = if count1.max(count2) > 0.0 {
            count1.min(count2) / count1.max(count2)
        } else {
            1.0
        };

        // Weighted overall AST similarity
        let overall_similarity = structural_similarity * 0.3
            + content_similarity * 0.25
            + control_flow_similarity * 0.2
            + edit_distance_score * 0.15
            + depth_similarity * 0.05
            + node_count_similarity * 0.05;

        Ok(ASTSimilarityScore {
            overall_similarity,
            structural_similarity,
            content_similarity,
            control_flow_similarity,
            edit_distance_score,
            depth_similarity,
            node_count_similarity,
        })
    }

    /// Calculate structural similarity based on AST node types and hierarchy
    fn calculate_structural_similarity(
        &self,
        ast1: &ASTNode,
        ast2: &ASTNode,
        depth: usize,
    ) -> Result<f64> {
        if depth > self.config.max_ast_depth {
            return Ok(0.5); // Partial similarity for deep trees
        }

        // Node type similarity
        if ast1.node_type != ast2.node_type {
            return Ok(0.0);
        }

        // Leaf nodes
        if ast1.children.is_empty() && ast2.children.is_empty() {
            return Ok(1.0);
        }

        // Different child counts
        if ast1.children.len() != ast2.children.len() {
            // Try to find best alignment for different child counts
            return self.calculate_partial_structural_similarity(ast1, ast2, depth);
        }

        // Same child count - compare recursively
        let mut total_similarity = 0.0;
        for (child1, child2) in ast1.children.iter().zip(ast2.children.iter()) {
            total_similarity += self.calculate_structural_similarity(child1, child2, depth + 1)?;
        }

        Ok(total_similarity / ast1.children.len() as f64)
    }

    /// Calculate partial structural similarity for nodes with different child counts
    fn calculate_partial_structural_similarity(
        &self,
        ast1: &ASTNode,
        ast2: &ASTNode,
        depth: usize,
    ) -> Result<f64> {
        let min_children = ast1.children.len().min(ast2.children.len());
        let max_children = ast1.children.len().max(ast2.children.len());

        if max_children == 0 {
            return Ok(1.0);
        }

        // Calculate similarity for common children
        let mut total_similarity = 0.0;
        for i in 0..min_children {
            total_similarity += self.calculate_structural_similarity(
                &ast1.children[i],
                &ast2.children[i],
                depth + 1,
            )?;
        }

        // Penalty for missing children
        let child_count_penalty = min_children as f64 / max_children as f64;
        let average_similarity = if min_children > 0 {
            total_similarity / min_children as f64
        } else {
            0.0
        };

        Ok(average_similarity * child_count_penalty)
    }

    /// Calculate content similarity based on identifiers, literals, and values
    fn calculate_content_similarity(&self, ast1: &ASTNode, ast2: &ASTNode) -> Result<f64> {
        let content1 = self.extract_content_features(ast1);
        let content2 = self.extract_content_features(ast2);

        // Calculate Jaccard similarity for content features
        let intersection = content1.intersection(&content2).count();
        let union = content1.union(&content2).count();

        if union == 0 {
            Ok(1.0) // Both empty
        } else {
            Ok(intersection as f64 / union as f64)
        }
    }

    /// Extract content features from AST (identifiers, literals, etc.)
    fn extract_content_features(&self, ast: &ASTNode) -> HashSet<String> {
        let mut features = HashSet::new();
        self.extract_content_features_recursive(ast, &mut features);
        features
    }

    /// Recursively extract content features
    #[allow(clippy::only_used_in_recursion)]
    fn extract_content_features_recursive(&self, ast: &ASTNode, features: &mut HashSet<String>) {
        // Extract identifiers
        if let Some(identifier) = ast.metadata.attributes.get("identifier") {
            features.insert(format!("id:{}", identifier));
        }

        // Extract literals
        if let Some(literal) = ast.metadata.attributes.get("literal") {
            features.insert(format!("lit:{}", literal));
        }

        // Extract operators
        if let Some(operator) = ast.metadata.attributes.get("operator") {
            features.insert(format!("op:{}", operator));
        }

        // Extract type names
        if let Some(type_name) = ast.metadata.attributes.get("type") {
            features.insert(format!("type:{}", type_name));
        }

        // Process children
        for child in &ast.children {
            self.extract_content_features_recursive(child, features);
        }
    }

    /// Calculate control flow similarity
    fn calculate_control_flow_similarity(&self, ast1: &ASTNode, ast2: &ASTNode) -> Result<f64> {
        let patterns1 = self.extract_control_flow_patterns(ast1);
        let patterns2 = self.extract_control_flow_patterns(ast2);

        if patterns1.is_empty() && patterns2.is_empty() {
            return Ok(1.0);
        }

        // Calculate pattern similarity
        let mut total_similarity = 0.0;
        let mut pattern_count = 0;

        for pattern1 in &patterns1 {
            let mut best_match: f64 = 0.0;
            for pattern2 in &patterns2 {
                let similarity = self.calculate_pattern_similarity(pattern1, pattern2);
                best_match = best_match.max(similarity);
            }
            total_similarity += best_match;
            pattern_count += 1;
        }

        if pattern_count > 0 {
            Ok(total_similarity / pattern_count as f64)
        } else {
            Ok(1.0)
        }
    }

    /// Extract control flow patterns from AST
    fn extract_control_flow_patterns(&self, ast: &ASTNode) -> Vec<String> {
        let mut patterns = Vec::new();
        self.extract_control_flow_patterns_recursive(ast, &mut patterns, Vec::new());
        patterns
    }

    /// Recursively extract control flow patterns
    #[allow(clippy::only_used_in_recursion)]
    fn extract_control_flow_patterns_recursive(
        &self,
        ast: &ASTNode,
        patterns: &mut Vec<String>,
        mut path: Vec<String>,
    ) {
        match ast.node_type {
            NodeType::IfStatement => {
                path.push("if".to_string());
                patterns.push(path.join("->"));
            }
            NodeType::WhileLoop => {
                path.push("while".to_string());
                patterns.push(path.join("->"));
            }
            NodeType::ForLoop => {
                path.push("for".to_string());
                patterns.push(path.join("->"));
            }
            NodeType::SwitchStatement => {
                path.push("switch".to_string());
                patterns.push(path.join("->"));
            }
            NodeType::TryStatement => {
                path.push("try".to_string());
                patterns.push(path.join("->"));
            }
            _ => {}
        }

        for child in &ast.children {
            self.extract_control_flow_patterns_recursive(child, patterns, path.clone());
        }
    }

    /// Calculate similarity between control flow patterns
    fn calculate_pattern_similarity(&self, pattern1: &str, pattern2: &str) -> f64 {
        if pattern1 == pattern2 {
            return 1.0;
        }

        let parts1: Vec<&str> = pattern1.split("->").collect();
        let parts2: Vec<&str> = pattern2.split("->").collect();

        let common_parts = parts1.iter().filter(|part| parts2.contains(part)).count();

        let total_parts = parts1.len().max(parts2.len());

        if total_parts > 0 {
            common_parts as f64 / total_parts as f64
        } else {
            1.0
        }
    }

    /// Count total nodes in AST
    #[allow(clippy::only_used_in_recursion)]
    fn count_nodes(&self, ast: &ASTNode) -> usize {
        1 + ast
            .children
            .iter()
            .map(|child| self.count_nodes(child))
            .sum::<usize>()
    }

    /// Calculate tree depth
    #[allow(clippy::only_used_in_recursion)]
    fn calculate_tree_depth(&self, ast: &ASTNode) -> usize {
        if ast.children.is_empty() {
            1
        } else {
            1 + ast
                .children
                .iter()
                .map(|child| self.calculate_tree_depth(child))
                .max()
                .unwrap_or(0)
        }
    }

    /// Calculate context similarity between two functions
    fn calculate_context_similarity(
        &mut self,
        func1_signature: &EnhancedFunctionSignature,
        func1_ast: &ASTNode,
        func2_signature: &EnhancedFunctionSignature,
        func2_ast: &ASTNode,
    ) -> Result<ContextSimilarityScore> {
        // Get or calculate context info
        let context1 =
            self.get_or_calculate_context_info(&func1_signature.qualified_name, func1_ast);
        let context2 =
            self.get_or_calculate_context_info(&func2_signature.qualified_name, func2_ast);

        // Function call similarity
        let function_call_similarity =
            self.calculate_set_similarity(&context1.function_calls, &context2.function_calls);

        // Variable usage similarity
        let variable_usage_similarity =
            self.calculate_set_similarity(&context1.variable_names, &context2.variable_names);

        // Dependency similarity
        let dependency_similarity =
            self.calculate_set_similarity(&context1.dependencies, &context2.dependencies);

        // Type usage similarity
        let _type_usage_similarity =
            self.calculate_set_similarity(&context1.type_usage, &context2.type_usage);

        // Surrounding code similarity (based on class/namespace context)
        let surrounding_code_similarity =
            self.calculate_surrounding_code_similarity(func1_signature, func2_signature);

        // Namespace context similarity
        let namespace_context_similarity =
            self.calculate_namespace_context_similarity(func1_signature, func2_signature);

        // Weighted overall context similarity
        let overall_similarity = function_call_similarity * 0.3
            + variable_usage_similarity * 0.2
            + dependency_similarity * 0.2
            + surrounding_code_similarity * 0.15
            + namespace_context_similarity * 0.15;

        Ok(ContextSimilarityScore {
            overall_similarity,
            function_call_similarity,
            variable_usage_similarity,
            dependency_similarity,
            surrounding_code_similarity,
            namespace_context_similarity,
        })
    }

    /// Get or calculate context information for a function
    fn get_or_calculate_context_info(
        &mut self,
        qualified_name: &str,
        ast: &ASTNode,
    ) -> ContextInfo {
        if let Some(cached_info) = self.context_cache.get(qualified_name) {
            return cached_info.clone();
        }

        let context_info = self.extract_context_info(ast);
        self.context_cache
            .insert(qualified_name.to_string(), context_info.clone());
        context_info
    }

    /// Extract context information from AST
    fn extract_context_info(&self, ast: &ASTNode) -> ContextInfo {
        let mut function_calls = HashSet::new();
        let mut variable_names = HashSet::new();
        let mut type_usage = HashSet::new();
        let mut control_flow_patterns = Vec::new();
        let mut dependencies = HashSet::new();

        self.extract_context_info_recursive(
            ast,
            &mut function_calls,
            &mut variable_names,
            &mut type_usage,
            &mut control_flow_patterns,
            &mut dependencies,
        );

        ContextInfo {
            function_calls,
            variable_names,
            type_usage,
            control_flow_patterns,
            dependencies,
        }
    }

    /// Recursively extract context information
    #[allow(clippy::only_used_in_recursion)]
    fn extract_context_info_recursive(
        &self,
        ast: &ASTNode,
        function_calls: &mut HashSet<String>,
        variable_names: &mut HashSet<String>,
        type_usage: &mut HashSet<String>,
        control_flow_patterns: &mut Vec<String>,
        dependencies: &mut HashSet<String>,
    ) {
        match ast.node_type {
            NodeType::CallExpression => {
                if let Some(function_name) = ast.metadata.attributes.get("function_name") {
                    function_calls.insert(function_name.clone());
                }
            }
            NodeType::Identifier => {
                if let Some(name) = ast.metadata.attributes.get("name") {
                    variable_names.insert(name.clone());
                }
            }
            NodeType::TypeReference => {
                if let Some(type_name) = ast.metadata.attributes.get("type") {
                    type_usage.insert(type_name.clone());
                }
            }
            NodeType::ImportStatement => {
                if let Some(import_path) = ast.metadata.attributes.get("path") {
                    dependencies.insert(import_path.clone());
                }
            }
            NodeType::IfStatement | NodeType::WhileLoop | NodeType::ForLoop => {
                control_flow_patterns.push(format!("{:?}", ast.node_type));
            }
            _ => {}
        }

        for child in &ast.children {
            self.extract_context_info_recursive(
                child,
                function_calls,
                variable_names,
                type_usage,
                control_flow_patterns,
                dependencies,
            );
        }
    }

    /// Calculate similarity between two sets
    fn calculate_set_similarity(&self, set1: &HashSet<String>, set2: &HashSet<String>) -> f64 {
        if set1.is_empty() && set2.is_empty() {
            return 1.0;
        }

        let intersection = set1.intersection(set2).count();
        let union = set1.union(set2).count();

        if union > 0 {
            intersection as f64 / union as f64
        } else {
            1.0
        }
    }

    /// Calculate surrounding code similarity
    fn calculate_surrounding_code_similarity(
        &self,
        func1_signature: &EnhancedFunctionSignature,
        func2_signature: &EnhancedFunctionSignature,
    ) -> f64 {
        // Compare file paths
        let file_similarity = if func1_signature.file_path == func2_signature.file_path {
            1.0
        } else {
            // Calculate path similarity
            let path1_parts: Vec<&str> = func1_signature.file_path.split('/').collect();
            let path2_parts: Vec<&str> = func2_signature.file_path.split('/').collect();

            let common_parts = path1_parts
                .iter()
                .zip(path2_parts.iter())
                .take_while(|(a, b)| a == b)
                .count();

            let max_parts = path1_parts.len().max(path2_parts.len());
            if max_parts > 0 {
                common_parts as f64 / max_parts as f64
            } else {
                0.0
            }
        };

        // Compare line proximity (if same file)
        let line_proximity = if func1_signature.file_path == func2_signature.file_path {
            let line_distance = (func1_signature.line as i32 - func2_signature.line as i32).abs();
            (1.0 - (line_distance as f64 / 1000.0)).max(0.0)
        } else {
            0.0
        };

        (file_similarity * 0.7) + (line_proximity * 0.3)
    }

    /// Calculate namespace context similarity
    fn calculate_namespace_context_similarity(
        &self,
        func1_signature: &EnhancedFunctionSignature,
        func2_signature: &EnhancedFunctionSignature,
    ) -> f64 {
        let namespace1 = self.extract_namespace(&func1_signature.qualified_name);
        let namespace2 = self.extract_namespace(&func2_signature.qualified_name);

        if namespace1 == namespace2 {
            1.0
        } else {
            // Calculate namespace hierarchy similarity
            let parts1: Vec<&str> = namespace1.split('.').collect();
            let parts2: Vec<&str> = namespace2.split('.').collect();

            let common_parts = parts1
                .iter()
                .zip(parts2.iter())
                .take_while(|(a, b)| a == b)
                .count();

            let max_parts = parts1.len().max(parts2.len());
            if max_parts > 0 {
                common_parts as f64 / max_parts as f64
            } else {
                0.0
            }
        }
    }

    /// Extract namespace from qualified name
    fn extract_namespace(&self, qualified_name: &str) -> String {
        if let Some(last_dot) = qualified_name.rfind('.') {
            qualified_name[..last_dot].to_string()
        } else {
            "global".to_string()
        }
    }

    /// Calculate semantic similarity metrics
    fn calculate_semantic_metrics(
        &self,
        ast1: &ASTNode,
        ast2: &ASTNode,
    ) -> Result<SemanticSimilarityMetrics> {
        let type_usage_similarity = self.calculate_type_usage_similarity(ast1, ast2)?;
        let api_pattern_similarity = self.calculate_api_pattern_similarity(ast1, ast2)?;
        let error_handling_similarity = self.calculate_error_handling_similarity(ast1, ast2)?;
        let resource_management_similarity =
            self.calculate_resource_management_similarity(ast1, ast2)?;
        let algorithm_pattern_similarity =
            self.calculate_algorithm_pattern_similarity(ast1, ast2)?;

        Ok(SemanticSimilarityMetrics {
            type_usage_similarity,
            api_pattern_similarity,
            error_handling_similarity,
            resource_management_similarity,
            algorithm_pattern_similarity,
        })
    }

    /// Calculate type usage similarity
    fn calculate_type_usage_similarity(&self, ast1: &ASTNode, ast2: &ASTNode) -> Result<f64> {
        let types1 = self.extract_type_usage(ast1);
        let types2 = self.extract_type_usage(ast2);

        Ok(self.calculate_set_similarity(&types1, &types2))
    }

    /// Extract type usage from AST
    fn extract_type_usage(&self, ast: &ASTNode) -> HashSet<String> {
        let mut types = HashSet::new();
        self.extract_type_usage_recursive(ast, &mut types);
        types
    }

    /// Recursively extract type usage
    #[allow(clippy::only_used_in_recursion)]
    fn extract_type_usage_recursive(&self, ast: &ASTNode, types: &mut HashSet<String>) {
        if let Some(type_name) = ast.metadata.attributes.get("type") {
            types.insert(type_name.clone());
        }

        for child in &ast.children {
            self.extract_type_usage_recursive(child, types);
        }
    }

    /// Calculate API pattern similarity
    fn calculate_api_pattern_similarity(&self, ast1: &ASTNode, ast2: &ASTNode) -> Result<f64> {
        let patterns1 = self.extract_api_patterns(ast1);
        let patterns2 = self.extract_api_patterns(ast2);

        Ok(self.calculate_set_similarity(&patterns1, &patterns2))
    }

    /// Extract API patterns from AST
    fn extract_api_patterns(&self, ast: &ASTNode) -> HashSet<String> {
        let mut patterns = HashSet::new();
        self.extract_api_patterns_recursive(ast, &mut patterns);
        patterns
    }

    /// Recursively extract API patterns
    #[allow(clippy::only_used_in_recursion)]
    fn extract_api_patterns_recursive(&self, ast: &ASTNode, patterns: &mut HashSet<String>) {
        if ast.node_type == NodeType::CallExpression {
            if let Some(function_name) = ast.metadata.attributes.get("function_name") {
                // Extract common API patterns
                if function_name.contains("get") || function_name.contains("set") {
                    patterns.insert("accessor_pattern".to_string());
                }
                if function_name.contains("create") || function_name.contains("new") {
                    patterns.insert("factory_pattern".to_string());
                }
                if function_name.contains("validate") || function_name.contains("check") {
                    patterns.insert("validation_pattern".to_string());
                }
                if function_name.contains("process") || function_name.contains("transform") {
                    patterns.insert("processing_pattern".to_string());
                }
            }
        }

        for child in &ast.children {
            self.extract_api_patterns_recursive(child, patterns);
        }
    }

    /// Calculate error handling similarity
    fn calculate_error_handling_similarity(&self, ast1: &ASTNode, ast2: &ASTNode) -> Result<f64> {
        let error_patterns1 = self.extract_error_handling_patterns(ast1);
        let error_patterns2 = self.extract_error_handling_patterns(ast2);

        Ok(self.calculate_set_similarity(&error_patterns1, &error_patterns2))
    }

    /// Extract error handling patterns
    fn extract_error_handling_patterns(&self, ast: &ASTNode) -> HashSet<String> {
        let mut patterns = HashSet::new();
        self.extract_error_handling_patterns_recursive(ast, &mut patterns);
        patterns
    }

    /// Recursively extract error handling patterns
    #[allow(clippy::only_used_in_recursion)]
    fn extract_error_handling_patterns_recursive(
        &self,
        ast: &ASTNode,
        patterns: &mut HashSet<String>,
    ) {
        match ast.node_type {
            NodeType::TryStatement => {
                patterns.insert("try_catch".to_string());
            }
            NodeType::ThrowStatement => {
                patterns.insert("throw_exception".to_string());
            }
            NodeType::IfStatement => {
                // Check for error condition patterns
                if let Some(condition) = ast.metadata.attributes.get("condition") {
                    if condition.contains("null")
                        || condition.contains("error")
                        || condition.contains("exception")
                    {
                        patterns.insert("error_check".to_string());
                    }
                }
            }
            _ => {}
        }

        for child in &ast.children {
            self.extract_error_handling_patterns_recursive(child, patterns);
        }
    }

    /// Calculate resource management similarity
    fn calculate_resource_management_similarity(
        &self,
        ast1: &ASTNode,
        ast2: &ASTNode,
    ) -> Result<f64> {
        let resource_patterns1 = self.extract_resource_management_patterns(ast1);
        let resource_patterns2 = self.extract_resource_management_patterns(ast2);

        Ok(self.calculate_set_similarity(&resource_patterns1, &resource_patterns2))
    }

    /// Extract resource management patterns
    fn extract_resource_management_patterns(&self, ast: &ASTNode) -> HashSet<String> {
        let mut patterns = HashSet::new();
        self.extract_resource_management_patterns_recursive(ast, &mut patterns);
        patterns
    }

    /// Recursively extract resource management patterns
    #[allow(clippy::only_used_in_recursion)]
    fn extract_resource_management_patterns_recursive(
        &self,
        ast: &ASTNode,
        patterns: &mut HashSet<String>,
    ) {
        match ast.node_type {
            NodeType::CallExpression => {
                if let Some(function_name) = ast.metadata.attributes.get("function_name") {
                    if function_name.contains("close") || function_name.contains("dispose") {
                        patterns.insert("resource_cleanup".to_string());
                    }
                    if function_name.contains("open") || function_name.contains("connect") {
                        patterns.insert("resource_acquisition".to_string());
                    }
                }
            }
            NodeType::TryStatement => {
                // Check for try-with-resources pattern
                patterns.insert("try_with_resources".to_string());
            }
            _ => {}
        }

        for child in &ast.children {
            self.extract_resource_management_patterns_recursive(child, patterns);
        }
    }

    /// Calculate algorithm pattern similarity
    fn calculate_algorithm_pattern_similarity(
        &self,
        ast1: &ASTNode,
        ast2: &ASTNode,
    ) -> Result<f64> {
        let algorithm_patterns1 = self.extract_algorithm_patterns(ast1);
        let algorithm_patterns2 = self.extract_algorithm_patterns(ast2);

        Ok(self.calculate_set_similarity(&algorithm_patterns1, &algorithm_patterns2))
    }

    /// Extract algorithm patterns
    fn extract_algorithm_patterns(&self, ast: &ASTNode) -> HashSet<String> {
        let mut patterns = HashSet::new();
        self.extract_algorithm_patterns_recursive(ast, &mut patterns, 0);
        patterns
    }

    /// Recursively extract algorithm patterns
    #[allow(clippy::only_used_in_recursion)]
    fn extract_algorithm_patterns_recursive(
        &self,
        ast: &ASTNode,
        patterns: &mut HashSet<String>,
        depth: usize,
    ) {
        match ast.node_type {
            NodeType::ForLoop => {
                patterns.insert("iteration".to_string());
                // Check for nested loops (O(n²) patterns)
                if depth > 0 {
                    patterns.insert("nested_iteration".to_string());
                }
            }
            NodeType::WhileLoop => {
                patterns.insert("while_iteration".to_string());
            }
            NodeType::IfStatement => {
                patterns.insert("conditional_logic".to_string());
            }
            NodeType::SwitchStatement => {
                patterns.insert("multi_branch".to_string());
            }
            NodeType::CallExpression => {
                if let Some(function_name) = ast.metadata.attributes.get("function_name") {
                    if function_name.contains("sort") {
                        patterns.insert("sorting_algorithm".to_string());
                    }
                    if function_name.contains("search") || function_name.contains("find") {
                        patterns.insert("search_algorithm".to_string());
                    }
                    if function_name.contains("recursive")
                        || function_name
                            == ast
                                .metadata
                                .attributes
                                .get("parent_function")
                                .unwrap_or(&String::new())
                    {
                        patterns.insert("recursion".to_string());
                    }
                }
            }
            _ => {}
        }

        let new_depth = if matches!(ast.node_type, NodeType::ForLoop | NodeType::WhileLoop) {
            depth + 1
        } else {
            depth
        };

        for child in &ast.children {
            self.extract_algorithm_patterns_recursive(child, patterns, new_depth);
        }
    }

    /// Classify match type based on similarity scores
    fn classify_match_type(
        &self,
        signature_similarity: &FunctionSignatureSimilarity,
        body_similarity: &ASTSimilarityScore,
        overall_similarity: f64,
    ) -> MatchType {
        if overall_similarity >= 0.95 {
            MatchType::ExactMatch
        } else if overall_similarity >= 0.85 {
            MatchType::HighSimilarity
        } else if overall_similarity >= 0.7 {
            MatchType::PotentialMatch
        } else if overall_similarity >= 0.5 {
            MatchType::WeakMatch
        } else if signature_similarity.name_similarity > 0.8
            && body_similarity.overall_similarity < 0.3
        {
            MatchType::PotentialRefactoring
        } else if signature_similarity.name_similarity < 0.5
            && body_similarity.overall_similarity > 0.7
        {
            MatchType::PotentialRename
        } else {
            MatchType::NoMatch
        }
    }

    /// Calculate confidence score for the match
    fn calculate_confidence_score(
        &self,
        signature_similarity: &FunctionSignatureSimilarity,
        body_similarity: &ASTSimilarityScore,
        context_similarity: &ContextSimilarityScore,
    ) -> f64 {
        // Base confidence from overall similarities
        let base_confidence = signature_similarity.overall_similarity * 0.4
            + body_similarity.overall_similarity * 0.4
            + context_similarity.overall_similarity * 0.2;

        // Boost confidence for exact matches
        let exact_match_bonus = if signature_similarity.similarity_breakdown.exact_name_match {
            0.1
        } else {
            0.0
        };

        // Boost confidence for parameter type matches
        let param_type_bonus = if signature_similarity
            .similarity_breakdown
            .parameter_types_match
            .iter()
            .all(|&m| m)
        {
            0.05
        } else {
            0.0
        };

        // Reduce confidence for structural mismatches
        let structural_penalty = if body_similarity.structural_similarity < 0.3 {
            -0.1
        } else {
            0.0
        };

        (base_confidence + exact_match_bonus + param_type_bonus + structural_penalty)
            .clamp(0.0, 1.0)
    }

    /// Build detailed similarity breakdown
    #[allow(clippy::too_many_arguments)]
    fn build_detailed_breakdown(
        &self,
        _func1_signature: &EnhancedFunctionSignature,
        func1_ast: &ASTNode,
        _func2_signature: &EnhancedFunctionSignature,
        func2_ast: &ASTNode,
        signature_similarity: &FunctionSignatureSimilarity,
        body_similarity: &ASTSimilarityScore,
        context_similarity: &ContextSimilarityScore,
    ) -> Result<DetailedSimilarityBreakdown> {
        // Signature components breakdown
        let mut signature_components = HashMap::new();
        signature_components.insert("name".to_string(), signature_similarity.name_similarity);
        signature_components.insert(
            "parameters".to_string(),
            signature_similarity.parameter_similarity,
        );
        signature_components.insert(
            "return_type".to_string(),
            signature_similarity.return_type_similarity,
        );
        signature_components.insert(
            "modifiers".to_string(),
            signature_similarity.modifier_similarity,
        );

        // AST node type distribution
        let node_dist1 = self.calculate_node_type_distribution(func1_ast);
        let node_dist2 = self.calculate_node_type_distribution(func2_ast);
        let mut ast_node_distribution = HashMap::new();

        for (node_type, count1) in &node_dist1 {
            let count2 = node_dist2.get(node_type).unwrap_or(&0);
            let similarity = if count1.max(count2) > &0 {
                *count1.min(count2) as f64 / *count1.max(count2) as f64
            } else {
                1.0
            };
            ast_node_distribution.insert(node_type.clone(), similarity);
        }

        // Control flow patterns
        let patterns1 = self.extract_control_flow_patterns(func1_ast);
        let patterns2 = self.extract_control_flow_patterns(func2_ast);
        let control_flow_patterns = patterns1
            .into_iter()
            .filter(|pattern| patterns2.contains(pattern))
            .collect();

        // Common function calls
        let context1 = self.extract_context_info(func1_ast);
        let context2 = self.extract_context_info(func2_ast);
        let common_function_calls = context1
            .function_calls
            .intersection(&context2.function_calls)
            .cloned()
            .collect();

        // Common variables
        let common_variables = context1
            .variable_names
            .intersection(&context2.variable_names)
            .cloned()
            .collect();

        // Contributing factors
        let mut contributing_factors = Vec::new();

        if signature_similarity.similarity_breakdown.exact_name_match {
            contributing_factors.push(SimilarityFactor {
                factor_type: "signature".to_string(),
                description: "Exact function name match".to_string(),
                impact: 0.4,
                confidence: 1.0,
            });
        }

        if body_similarity.structural_similarity > 0.8 {
            contributing_factors.push(SimilarityFactor {
                factor_type: "structure".to_string(),
                description: "High structural similarity".to_string(),
                impact: body_similarity.structural_similarity * 0.4,
                confidence: 0.9,
            });
        }

        if context_similarity.function_call_similarity > 0.7 {
            contributing_factors.push(SimilarityFactor {
                factor_type: "context".to_string(),
                description: "Similar function call patterns".to_string(),
                impact: context_similarity.function_call_similarity * 0.2,
                confidence: 0.8,
            });
        }

        // Dissimilarity factors
        let mut dissimilarity_factors = Vec::new();

        if signature_similarity.parameter_similarity < 0.3 {
            dissimilarity_factors.push(SimilarityFactor {
                factor_type: "signature".to_string(),
                description: "Different parameter signatures".to_string(),
                impact: -(1.0 - signature_similarity.parameter_similarity) * 0.3,
                confidence: 0.9,
            });
        }

        if body_similarity.control_flow_similarity < 0.3 {
            dissimilarity_factors.push(SimilarityFactor {
                factor_type: "control_flow".to_string(),
                description: "Different control flow patterns".to_string(),
                impact: -(1.0 - body_similarity.control_flow_similarity) * 0.2,
                confidence: 0.8,
            });
        }

        Ok(DetailedSimilarityBreakdown {
            signature_components,
            ast_node_distribution,
            control_flow_patterns,
            common_function_calls,
            common_variables,
            contributing_factors,
            dissimilarity_factors,
        })
    }

    /// Calculate node type distribution in AST
    fn calculate_node_type_distribution(&self, ast: &ASTNode) -> HashMap<String, usize> {
        let mut distribution = HashMap::new();
        self.calculate_node_type_distribution_recursive(ast, &mut distribution);
        distribution
    }

    /// Recursively calculate node type distribution
    #[allow(clippy::only_used_in_recursion)]
    fn calculate_node_type_distribution_recursive(
        &self,
        ast: &ASTNode,
        distribution: &mut HashMap<String, usize>,
    ) {
        let node_type_str = format!("{:?}", ast.node_type);
        *distribution.entry(node_type_str).or_insert(0) += 1;

        for child in &ast.children {
            self.calculate_node_type_distribution_recursive(child, distribution);
        }
    }

    /// Find best matches for a function in a collection
    pub fn find_best_matches(
        &mut self,
        target_signature: &EnhancedFunctionSignature,
        target_ast: &ASTNode,
        candidates: &[(EnhancedFunctionSignature, ASTNode)],
        max_results: usize,
    ) -> Result<Vec<(ComprehensiveSimilarityScore, usize)>> {
        let mut matches = Vec::new();

        for (i, (candidate_signature, candidate_ast)) in candidates.iter().enumerate() {
            let similarity = self.calculate_comprehensive_similarity(
                target_signature,
                target_ast,
                candidate_signature,
                candidate_ast,
            )?;

            if similarity.overall_similarity >= self.config.match_threshold {
                matches.push((similarity, i));
            }
        }

        // Sort by overall similarity (descending)
        matches.sort_by(|a, b| {
            b.0.overall_similarity
                .partial_cmp(&a.0.overall_similarity)
                .unwrap()
        });

        // Return top matches
        matches.truncate(max_results);
        Ok(matches)
    }

    /// Clear context cache
    pub fn clear_cache(&mut self) {
        self.context_cache.clear();
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.context_cache.len(), self.context_cache.capacity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smart_diff_parser::{ASTNode, NodeMetadata, NodeType};
    use smart_diff_semantic::{EnhancedFunctionSignature, FunctionType, Visibility};
    use std::collections::HashMap;

    fn create_test_ast_node(node_type: NodeType, attributes: HashMap<String, String>) -> ASTNode {
        ASTNode {
            id: "test_node_123".to_string(),
            node_type,
            children: Vec::new(),
            metadata: NodeMetadata {
                line: 1,
                column: 1,
                original_text: "test_code".to_string(),
                attributes,
            },
        }
    }

    #[allow(dead_code)]
    fn create_test_function_signature(
        name: &str,
        qualified_name: &str,
    ) -> EnhancedFunctionSignature {
        use smart_diff_semantic::TypeSignature;

        EnhancedFunctionSignature {
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            parameters: Vec::new(),
            return_type: TypeSignature::new("void".to_string()),
            generic_parameters: Vec::new(),
            visibility: Visibility::Public,
            modifiers: Vec::new(),
            annotations: Vec::new(),
            file_path: "test.java".to_string(),
            line: 1,
            column: 1,
            end_line: 10,
            function_type: FunctionType::Method,
            complexity_metrics: None,
            dependencies: Vec::new(),
            signature_hash: "test_hash".to_string(),
            normalized_hash: "test_normalized".to_string(),
        }
    }

    #[test]
    fn test_similarity_scoring_config_default() {
        let config = SimilarityScoringConfig::default();

        assert_eq!(config.signature_weight, 0.4);
        assert_eq!(config.body_weight, 0.4);
        assert_eq!(config.context_weight, 0.2);
        assert_eq!(config.match_threshold, 0.7);
        assert!(config.enable_advanced_ast_comparison);
        assert!(config.enable_semantic_context);
        assert!(!config.enable_cross_language);
        assert_eq!(config.max_ast_depth, 10);
    }

    #[test]
    fn test_similarity_scorer_creation() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        assert_eq!(scorer.language, Language::Java);
        assert!(scorer.context_cache.is_empty());
    }

    #[test]
    fn test_match_type_classification() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        // Create mock similarity scores
        let signature_sim = smart_diff_semantic::FunctionSignatureSimilarity {
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
                parameter_types_match: vec![true, true],
                return_type_match: true,
                visibility_match: true,
                static_match: true,
                generic_parameters_match: true,
            },
        };

        let body_sim = ASTSimilarityScore {
            overall_similarity: 0.85,
            structural_similarity: 0.9,
            content_similarity: 0.8,
            control_flow_similarity: 0.85,
            edit_distance_score: 0.9,
            depth_similarity: 0.95,
            node_count_similarity: 0.9,
        };

        // Test exact match
        let match_type = scorer.classify_match_type(&signature_sim, &body_sim, 0.96);
        assert_eq!(match_type, MatchType::ExactMatch);

        // Test high similarity
        let match_type = scorer.classify_match_type(&signature_sim, &body_sim, 0.88);
        assert_eq!(match_type, MatchType::HighSimilarity);

        // Test potential match
        let match_type = scorer.classify_match_type(&signature_sim, &body_sim, 0.75);
        assert_eq!(match_type, MatchType::PotentialMatch);

        // Test weak match
        let match_type = scorer.classify_match_type(&signature_sim, &body_sim, 0.55);
        assert_eq!(match_type, MatchType::WeakMatch);

        // Test no match
        let match_type = scorer.classify_match_type(&signature_sim, &body_sim, 0.3);
        assert_eq!(match_type, MatchType::NoMatch);
    }

    #[test]
    fn test_structural_similarity_identical_nodes() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        let node1 = create_test_ast_node(NodeType::Function, HashMap::new());
        let node2 = create_test_ast_node(NodeType::Function, HashMap::new());

        let similarity = scorer
            .calculate_structural_similarity(&node1, &node2, 0)
            .unwrap();
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_structural_similarity_different_node_types() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        let node1 = create_test_ast_node(NodeType::Function, HashMap::new());
        let node2 = create_test_ast_node(NodeType::Method, HashMap::new());

        let similarity = scorer
            .calculate_structural_similarity(&node1, &node2, 0)
            .unwrap();
        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_content_similarity_extraction() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        let mut attributes1 = HashMap::new();
        attributes1.insert("identifier".to_string(), "variable1".to_string());
        attributes1.insert("literal".to_string(), "42".to_string());

        let mut attributes2 = HashMap::new();
        attributes2.insert("identifier".to_string(), "variable1".to_string());
        attributes2.insert("literal".to_string(), "42".to_string());

        let node1 = create_test_ast_node(NodeType::Identifier, attributes1);
        let node2 = create_test_ast_node(NodeType::Identifier, attributes2);

        let features1 = scorer.extract_content_features(&node1);
        let features2 = scorer.extract_content_features(&node2);

        assert!(features1.contains("id:variable1"));
        assert!(features1.contains("lit:42"));
        assert_eq!(features1, features2);
    }

    #[test]
    fn test_control_flow_pattern_extraction() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        let if_node = create_test_ast_node(NodeType::IfStatement, HashMap::new());
        let while_node = create_test_ast_node(NodeType::WhileLoop, HashMap::new());

        let mut root = create_test_ast_node(NodeType::Function, HashMap::new());
        root.children.push(if_node);
        root.children.push(while_node);

        let patterns = scorer.extract_control_flow_patterns(&root);

        assert!(patterns.contains(&"if".to_string()));
        assert!(patterns.contains(&"while".to_string()));
    }

    #[test]
    fn test_set_similarity_calculation() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        let mut set1 = HashSet::new();
        set1.insert("a".to_string());
        set1.insert("b".to_string());
        set1.insert("c".to_string());

        let mut set2 = HashSet::new();
        set2.insert("b".to_string());
        set2.insert("c".to_string());
        set2.insert("d".to_string());

        let similarity = scorer.calculate_set_similarity(&set1, &set2);

        // Intersection: {b, c} = 2 elements
        // Union: {a, b, c, d} = 4 elements
        // Similarity: 2/4 = 0.5
        assert_eq!(similarity, 0.5);
    }

    #[test]
    fn test_empty_sets_similarity() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        let set1 = HashSet::new();
        let set2 = HashSet::new();

        let similarity = scorer.calculate_set_similarity(&set1, &set2);
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_node_type_distribution() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        let mut root = create_test_ast_node(NodeType::Function, HashMap::new());
        root.children
            .push(create_test_ast_node(NodeType::IfStatement, HashMap::new()));
        root.children
            .push(create_test_ast_node(NodeType::IfStatement, HashMap::new()));
        root.children
            .push(create_test_ast_node(NodeType::WhileLoop, HashMap::new()));

        let distribution = scorer.calculate_node_type_distribution(&root);

        assert_eq!(distribution.get("Function"), Some(&1));
        assert_eq!(distribution.get("IfStatement"), Some(&2));
        assert_eq!(distribution.get("WhileLoop"), Some(&1));
    }

    #[test]
    fn test_tree_depth_calculation() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        let mut root = create_test_ast_node(NodeType::Function, HashMap::new());
        let mut child1 = create_test_ast_node(NodeType::IfStatement, HashMap::new());
        let child2 = create_test_ast_node(NodeType::WhileLoop, HashMap::new());

        child1.children.push(child2);
        root.children.push(child1);

        let depth = scorer.calculate_tree_depth(&root);
        assert_eq!(depth, 3); // root -> if -> while
    }

    #[test]
    fn test_node_count() {
        let config = SimilarityScoringConfig::default();
        let scorer = SimilarityScorer::new(Language::Java, config);

        let mut root = create_test_ast_node(NodeType::Function, HashMap::new());
        root.children
            .push(create_test_ast_node(NodeType::IfStatement, HashMap::new()));
        root.children
            .push(create_test_ast_node(NodeType::WhileLoop, HashMap::new()));

        let count = scorer.count_nodes(&root);
        assert_eq!(count, 3); // root + 2 children
    }

    #[test]
    fn test_cache_operations() {
        let config = SimilarityScoringConfig::default();
        let mut scorer = SimilarityScorer::new(Language::Java, config);

        // Initially empty
        let (size, _) = scorer.get_cache_stats();
        assert_eq!(size, 0);

        // Add some context info
        let ast = create_test_ast_node(NodeType::Function, HashMap::new());
        let _context = scorer.get_or_calculate_context_info("test.function", &ast);

        // Should have one entry
        let (size, _) = scorer.get_cache_stats();
        assert_eq!(size, 1);

        // Clear cache
        scorer.clear_cache();
        let (size, _) = scorer.get_cache_stats();
        assert_eq!(size, 0);
    }
}
