//! Comprehensive Change Classification and Analysis System
//!
//! This module implements an advanced change classification system that categorizes
//! code changes with detailed analysis, confidence scoring, and integration with
//! tree edit distance and similarity scoring algorithms.

use crate::tree_edit::{TreeEditDistance, ZhangShashaConfig};
use crate::similarity_scorer::{SimilarityScorer, SimilarityScoringConfig, ComprehensiveSimilarityScore};
use smart_diff_parser::{ChangeType, CodeElement, ASTNode, Language};
use smart_diff_semantic::EnhancedFunctionSignature;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;

/// Configuration for change classification
#[derive(Debug, Clone)]
pub struct ChangeClassificationConfig {
    /// Minimum similarity threshold for considering elements as modified (vs replaced)
    pub modification_threshold: f64,
    /// Minimum similarity threshold for rename detection
    pub rename_threshold: f64,
    /// Minimum similarity threshold for move detection
    pub move_threshold: f64,
    /// Enable detailed AST-based analysis
    pub enable_ast_analysis: bool,
    /// Enable semantic analysis integration
    pub enable_semantic_analysis: bool,
    /// Enable confidence scoring
    pub enable_confidence_scoring: bool,
    /// Maximum depth for AST analysis
    pub max_ast_depth: usize,
    /// Enable change impact analysis
    pub enable_impact_analysis: bool,
}

impl Default for ChangeClassificationConfig {
    fn default() -> Self {
        Self {
            modification_threshold: 0.7,
            rename_threshold: 0.8,
            move_threshold: 0.9,
            enable_ast_analysis: true,
            enable_semantic_analysis: true,
            enable_confidence_scoring: true,
            max_ast_depth: 20,
            enable_impact_analysis: true,
        }
    }
}

/// Comprehensive change classifier with advanced analysis capabilities
pub struct ChangeClassifier {
    config: ChangeClassificationConfig,
    tree_edit_distance: TreeEditDistance,
    similarity_scorer: Option<SimilarityScorer>,
    language: Language,
}

/// Detailed change classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedChangeClassification {
    /// Primary change type
    pub change_type: ChangeType,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Detailed analysis of the change
    pub analysis: ChangeAnalysis,
    /// Secondary change types (for complex changes)
    pub secondary_types: Vec<ChangeType>,
    /// Change impact assessment
    pub impact: ChangeImpact,
    /// Similarity metrics if applicable
    pub similarity_metrics: Option<ComprehensiveSimilarityScore>,
}

/// Detailed analysis of a change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeAnalysis {
    /// Human-readable description
    pub description: String,
    /// Specific change characteristics
    pub characteristics: Vec<ChangeCharacteristic>,
    /// Evidence supporting the classification
    pub evidence: Vec<ClassificationEvidence>,
    /// Potential alternative classifications
    pub alternatives: Vec<AlternativeClassification>,
    /// Change complexity score
    pub complexity_score: f64,
}

/// Specific characteristic of a change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeCharacteristic {
    /// Type of characteristic
    pub characteristic_type: CharacteristicType,
    /// Value or description
    pub value: String,
    /// Confidence in this characteristic
    pub confidence: f64,
}

/// Types of change characteristics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CharacteristicType {
    /// Name similarity
    NameSimilarity,
    /// Structural similarity
    StructuralSimilarity,
    /// Content similarity
    ContentSimilarity,
    /// Location change
    LocationChange,
    /// Size change
    SizeChange,
    /// Complexity change
    ComplexityChange,
    /// Dependency change
    DependencyChange,
    /// Signature change
    SignatureChange,
}

/// Evidence supporting a classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationEvidence {
    /// Type of evidence
    pub evidence_type: EvidenceType,
    /// Description of the evidence
    pub description: String,
    /// Strength of evidence (0.0 to 1.0)
    pub strength: f64,
    /// Supporting data
    pub data: HashMap<String, String>,
}

/// Types of classification evidence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EvidenceType {
    /// Name matching evidence
    NameMatch,
    /// Signature matching evidence
    SignatureMatch,
    /// AST structure matching evidence
    ASTMatch,
    /// Content similarity evidence
    ContentSimilarity,
    /// Location evidence
    LocationEvidence,
    /// Dependency evidence
    DependencyEvidence,
    /// Temporal evidence (based on change patterns)
    TemporalEvidence,
}

/// Alternative classification possibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeClassification {
    /// Alternative change type
    pub change_type: ChangeType,
    /// Confidence in this alternative
    pub confidence: f64,
    /// Reason for considering this alternative
    pub reason: String,
}

/// Change impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeImpact {
    /// Overall impact level
    pub impact_level: ImpactLevel,
    /// Affected components
    pub affected_components: Vec<String>,
    /// Estimated effort to implement
    pub implementation_effort: EffortLevel,
    /// Risk assessment
    pub risk_level: RiskLevel,
    /// Breaking change indicator
    pub is_breaking_change: bool,
}

/// Levels of change impact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImpactLevel {
    /// Minimal impact
    Minimal,
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
    /// Critical impact
    Critical,
}

/// Levels of implementation effort
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EffortLevel {
    /// Trivial effort
    Trivial,
    /// Low effort
    Low,
    /// Medium effort
    Medium,
    /// High effort
    High,
    /// Very high effort
    VeryHigh,
}

/// Levels of risk
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    /// Very low risk
    VeryLow,
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Very high risk
    VeryHigh,
}

impl ChangeClassifier {
    /// Create a new change classifier with default configuration
    pub fn new(language: Language) -> Self {
        Self {
            config: ChangeClassificationConfig::default(),
            tree_edit_distance: TreeEditDistance::with_defaults(),
            similarity_scorer: Some(SimilarityScorer::new(language, SimilarityScoringConfig::default())),
            language,
        }
    }

    /// Create a new change classifier with custom configuration
    pub fn with_config(language: Language, config: ChangeClassificationConfig) -> Self {
        let tree_config = ZhangShashaConfig {
            enable_caching: true,
            enable_pruning: true,
            max_depth: config.max_ast_depth,
            ..Default::default()
        };

        Self {
            config,
            tree_edit_distance: TreeEditDistance::new(tree_config),
            similarity_scorer: Some(SimilarityScorer::new(language, SimilarityScoringConfig::default())),
            language,
        }
    }

    /// Simple change classification (backward compatibility)
    pub fn classify_change(
        &self,
        source: Option<&CodeElement>,
        target: Option<&CodeElement>,
    ) -> ChangeType {
        match (source, target) {
            (None, Some(_)) => ChangeType::Add,
            (Some(_), None) => ChangeType::Delete,
            (Some(src), Some(tgt)) => {
                if src.name != tgt.name {
                    if src.file_path != tgt.file_path {
                        ChangeType::CrossFileMove
                    } else {
                        ChangeType::Rename
                    }
                } else if src.file_path != tgt.file_path {
                    ChangeType::CrossFileMove
                } else if src.start_line != tgt.start_line {
                    ChangeType::Move
                } else {
                    ChangeType::Modify
                }
            }
            (None, None) => ChangeType::Modify, // Shouldn't happen
        }
    }

    /// Comprehensive change classification with detailed analysis
    pub fn classify_change_detailed(
        &mut self,
        source: Option<&CodeElement>,
        target: Option<&CodeElement>,
        source_ast: Option<&ASTNode>,
        target_ast: Option<&ASTNode>,
        source_signature: Option<&EnhancedFunctionSignature>,
        target_signature: Option<&EnhancedFunctionSignature>,
    ) -> Result<DetailedChangeClassification> {
        match (source, target) {
            (None, Some(target_elem)) => {
                self.classify_addition(target_elem, target_ast, target_signature)
            }
            (Some(source_elem), None) => {
                self.classify_deletion(source_elem, source_ast, source_signature)
            }
            (Some(source_elem), Some(target_elem)) => {
                self.classify_modification(
                    source_elem, target_elem,
                    source_ast, target_ast,
                    source_signature, target_signature
                )
            }
            (None, None) => {
                // This shouldn't happen in normal cases
                Ok(DetailedChangeClassification {
                    change_type: ChangeType::Modify,
                    confidence: 0.0,
                    analysis: ChangeAnalysis {
                        description: "Invalid change: both source and target are None".to_string(),
                        characteristics: Vec::new(),
                        evidence: Vec::new(),
                        alternatives: Vec::new(),
                        complexity_score: 0.0,
                    },
                    secondary_types: Vec::new(),
                    impact: ChangeImpact {
                        impact_level: ImpactLevel::Minimal,
                        affected_components: Vec::new(),
                        implementation_effort: EffortLevel::Trivial,
                        risk_level: RiskLevel::VeryLow,
                        is_breaking_change: false,
                    },
                    similarity_metrics: None,
                })
            }
        }
    }

    /// Classify an addition
    fn classify_addition(
        &self,
        target: &CodeElement,
        target_ast: Option<&ASTNode>,
        target_signature: Option<&EnhancedFunctionSignature>,
    ) -> Result<DetailedChangeClassification> {
        let mut characteristics = Vec::new();
        let mut evidence = Vec::new();

        // Analyze the added element
        if let Some(ast) = target_ast {
            let complexity = self.calculate_ast_complexity(ast);
            characteristics.push(ChangeCharacteristic {
                characteristic_type: CharacteristicType::ComplexityChange,
                value: format!("Added element complexity: {:.2}", complexity),
                confidence: 0.9,
            });
        }

        if let Some(signature) = target_signature {
            characteristics.push(ChangeCharacteristic {
                characteristic_type: CharacteristicType::SignatureChange,
                value: format!("Added function: {}", signature.name),
                confidence: 1.0,
            });

            evidence.push(ClassificationEvidence {
                evidence_type: EvidenceType::SignatureMatch,
                description: format!("New function signature: {}", signature.name),
                strength: 1.0,
                data: HashMap::new(),
            });
        }

        let impact = self.assess_addition_impact(target, target_signature);

        Ok(DetailedChangeClassification {
            change_type: ChangeType::Add,
            confidence: 1.0,
            analysis: ChangeAnalysis {
                description: format!("Added new element: {}", target.name),
                characteristics,
                evidence,
                alternatives: Vec::new(),
                complexity_score: target_signature
                    .and_then(|s| s.complexity_metrics.as_ref())
                    .map(|m| m.cyclomatic_complexity as f64)
                    .unwrap_or(1.0),
            },
            secondary_types: Vec::new(),
            impact,
            similarity_metrics: None,
        })
    }

    /// Classify a deletion
    fn classify_deletion(
        &self,
        source: &CodeElement,
        source_ast: Option<&ASTNode>,
        source_signature: Option<&EnhancedFunctionSignature>,
    ) -> Result<DetailedChangeClassification> {
        let mut characteristics = Vec::new();
        let mut evidence = Vec::new();

        // Analyze the deleted element
        if let Some(ast) = source_ast {
            let complexity = self.calculate_ast_complexity(ast);
            characteristics.push(ChangeCharacteristic {
                characteristic_type: CharacteristicType::ComplexityChange,
                value: format!("Deleted element complexity: {:.2}", complexity),
                confidence: 0.9,
            });
        }

        if let Some(signature) = source_signature {
            characteristics.push(ChangeCharacteristic {
                characteristic_type: CharacteristicType::SignatureChange,
                value: format!("Deleted function: {}", signature.name),
                confidence: 1.0,
            });

            evidence.push(ClassificationEvidence {
                evidence_type: EvidenceType::SignatureMatch,
                description: format!("Removed function signature: {}", signature.name),
                strength: 1.0,
                data: HashMap::new(),
            });
        }

        let impact = self.assess_deletion_impact(source, source_signature);

        Ok(DetailedChangeClassification {
            change_type: ChangeType::Delete,
            confidence: 1.0,
            analysis: ChangeAnalysis {
                description: format!("Deleted element: {}", source.name),
                characteristics,
                evidence,
                alternatives: Vec::new(),
                complexity_score: source_signature
                    .and_then(|s| s.complexity_metrics.as_ref())
                    .map(|m| m.cyclomatic_complexity as f64)
                    .unwrap_or(1.0),
            },
            secondary_types: Vec::new(),
            impact,
            similarity_metrics: None,
        })
    }

    /// Classify a modification (most complex case)
    fn classify_modification(
        &mut self,
        source: &CodeElement,
        target: &CodeElement,
        source_ast: Option<&ASTNode>,
        target_ast: Option<&ASTNode>,
        source_signature: Option<&EnhancedFunctionSignature>,
        target_signature: Option<&EnhancedFunctionSignature>,
    ) -> Result<DetailedChangeClassification> {
        let mut characteristics = Vec::new();
        let mut evidence = Vec::new();
        let mut alternatives = Vec::new();
        let mut secondary_types = Vec::new();

        // Calculate similarity metrics if AST and signatures are available
        let similarity_metrics = if let (Some(src_ast), Some(tgt_ast), Some(src_sig), Some(tgt_sig)) =
            (source_ast, target_ast, source_signature, target_signature) {
            if let Some(ref mut scorer) = self.similarity_scorer {
                Some(scorer.calculate_comprehensive_similarity(src_sig, src_ast, tgt_sig, tgt_ast)?)
            } else {
                None
            }
        } else {
            None
        };

        // Determine primary change type based on various factors
        let (primary_type, confidence) = self.determine_primary_change_type(
            source, target, &similarity_metrics, source_signature, target_signature
        );

        // Analyze name changes
        if source.name != target.name {
            let name_similarity = self.calculate_name_similarity(&source.name, &target.name);
            characteristics.push(ChangeCharacteristic {
                characteristic_type: CharacteristicType::NameSimilarity,
                value: format!("{:.3}", name_similarity),
                confidence: 0.9,
            });

            evidence.push(ClassificationEvidence {
                evidence_type: EvidenceType::NameMatch,
                description: format!("Name changed from '{}' to '{}'", source.name, target.name),
                strength: 1.0 - name_similarity,
                data: [
                    ("old_name".to_string(), source.name.clone()),
                    ("new_name".to_string(), target.name.clone()),
                ].into_iter().collect(),
            });

            if name_similarity > self.config.rename_threshold {
                if primary_type != ChangeType::Rename {
                    alternatives.push(AlternativeClassification {
                        change_type: ChangeType::Rename,
                        confidence: name_similarity,
                        reason: "High name similarity suggests rename".to_string(),
                    });
                }
            }
        }

        // Analyze location changes
        if source.file_path != target.file_path {
            characteristics.push(ChangeCharacteristic {
                characteristic_type: CharacteristicType::LocationChange,
                value: format!("File: {} → {}", source.file_path, target.file_path),
                confidence: 1.0,
            });

            evidence.push(ClassificationEvidence {
                evidence_type: EvidenceType::LocationEvidence,
                description: "Element moved to different file".to_string(),
                strength: 1.0,
                data: [
                    ("old_file".to_string(), source.file_path.clone()),
                    ("new_file".to_string(), target.file_path.clone()),
                ].into_iter().collect(),
            });

            if primary_type != ChangeType::CrossFileMove {
                secondary_types.push(ChangeType::CrossFileMove);
            }
        } else if source.start_line != target.start_line {
            let line_distance = (target.start_line as i32 - source.start_line as i32).abs();
            characteristics.push(ChangeCharacteristic {
                characteristic_type: CharacteristicType::LocationChange,
                value: format!("Line: {} → {} (distance: {})", source.start_line, target.start_line, line_distance),
                confidence: 1.0,
            });

            evidence.push(ClassificationEvidence {
                evidence_type: EvidenceType::LocationEvidence,
                description: format!("Element moved {} lines", line_distance),
                strength: (line_distance as f64 / 100.0).min(1.0),
                data: [
                    ("old_line".to_string(), source.start_line.to_string()),
                    ("new_line".to_string(), target.start_line.to_string()),
                    ("distance".to_string(), line_distance.to_string()),
                ].into_iter().collect(),
            });

            if primary_type != ChangeType::Move {
                secondary_types.push(ChangeType::Move);
            }
        }

        // Analyze structural changes using AST
        if let Some(metrics) = &similarity_metrics {
            characteristics.push(ChangeCharacteristic {
                characteristic_type: CharacteristicType::StructuralSimilarity,
                value: format!("{:.3}", metrics.body_similarity.structural_similarity),
                confidence: 0.9,
            });

            characteristics.push(ChangeCharacteristic {
                characteristic_type: CharacteristicType::ContentSimilarity,
                value: format!("{:.3}", metrics.body_similarity.content_similarity),
                confidence: 0.9,
            });

            evidence.push(ClassificationEvidence {
                evidence_type: EvidenceType::ASTMatch,
                description: format!("AST similarity: {:.3}", metrics.overall_similarity),
                strength: metrics.overall_similarity,
                data: [
                    ("structural_similarity".to_string(), metrics.body_similarity.structural_similarity.to_string()),
                    ("content_similarity".to_string(), metrics.body_similarity.content_similarity.to_string()),
                    ("control_flow_similarity".to_string(), metrics.body_similarity.control_flow_similarity.to_string()),
                ].into_iter().collect(),
            });
        }

        // Analyze signature changes
        if let (Some(src_sig), Some(tgt_sig)) = (source_signature, target_signature) {
            let sig_similarity = self.calculate_signature_similarity(src_sig, tgt_sig);
            characteristics.push(ChangeCharacteristic {
                characteristic_type: CharacteristicType::SignatureChange,
                value: format!("Signature similarity: {:.3}", sig_similarity),
                confidence: 0.9,
            });

            evidence.push(ClassificationEvidence {
                evidence_type: EvidenceType::SignatureMatch,
                description: format!("Function signature similarity: {:.3}", sig_similarity),
                strength: sig_similarity,
                data: HashMap::new(),
            });

            // Analyze complexity changes
            let complexity_change = if let (Some(tgt_metrics), Some(src_metrics)) =
                (&tgt_sig.complexity_metrics, &src_sig.complexity_metrics) {
                tgt_metrics.cyclomatic_complexity as f64 - src_metrics.cyclomatic_complexity as f64
            } else {
                0.0
            };

            if complexity_change.abs() > 0.1 {
                characteristics.push(ChangeCharacteristic {
                    characteristic_type: CharacteristicType::ComplexityChange,
                    value: format!("Complexity change: {:+.2}", complexity_change),
                    confidence: 0.8,
                });
            }
        }

        // Calculate overall complexity score
        let complexity_score = self.calculate_change_complexity(
            source, target, &similarity_metrics, source_signature, target_signature
        );

        // Assess impact
        let impact = self.assess_modification_impact(
            source, target, &similarity_metrics, source_signature, target_signature
        );

        Ok(DetailedChangeClassification {
            change_type: primary_type.clone(),
            confidence,
            analysis: ChangeAnalysis {
                description: self.generate_change_description(
                    &primary_type, source, target, &similarity_metrics
                ),
                characteristics,
                evidence,
                alternatives,
                complexity_score,
            },
            secondary_types,
            impact,
            similarity_metrics,
        })
    }

    /// Determine the primary change type based on various factors
    fn determine_primary_change_type(
        &self,
        source: &CodeElement,
        target: &CodeElement,
        similarity_metrics: &Option<ComprehensiveSimilarityScore>,
        _source_signature: Option<&EnhancedFunctionSignature>,
        _target_signature: Option<&EnhancedFunctionSignature>,
    ) -> (ChangeType, f64) {
        // Check for cross-file move first
        if source.file_path != target.file_path {
            let similarity = similarity_metrics
                .as_ref()
                .map(|m| m.overall_similarity)
                .unwrap_or(0.5);

            if similarity > self.config.move_threshold {
                return (ChangeType::CrossFileMove, similarity);
            }
        }

        // Check for rename
        if source.name != target.name {
            let name_similarity = self.calculate_name_similarity(&source.name, &target.name);
            let overall_similarity = similarity_metrics
                .as_ref()
                .map(|m| m.overall_similarity)
                .unwrap_or(name_similarity);

            if overall_similarity > self.config.rename_threshold {
                return (ChangeType::Rename, overall_similarity);
            }
        }

        // Check for move within same file
        if source.start_line != target.start_line && source.file_path == target.file_path {
            let similarity = similarity_metrics
                .as_ref()
                .map(|m| m.overall_similarity)
                .unwrap_or(0.8);

            if similarity > self.config.move_threshold {
                return (ChangeType::Move, similarity);
            }
        }

        // Default to modification
        let similarity = similarity_metrics
            .as_ref()
            .map(|m| m.overall_similarity)
            .unwrap_or(0.5);

        (ChangeType::Modify, similarity)
    }

    /// Calculate name similarity using edit distance
    fn calculate_name_similarity(&self, name1: &str, name2: &str) -> f64 {
        if name1 == name2 {
            return 1.0;
        }

        let max_len = name1.len().max(name2.len());
        if max_len == 0 {
            return 1.0;
        }

        let edit_distance = self.levenshtein_distance(name1, name2);
        1.0 - (edit_distance as f64 / max_len as f64)
    }

    /// Calculate Levenshtein distance between two strings
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        // Initialize first row and column
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };

                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[len1][len2]
    }

    /// Calculate signature similarity
    fn calculate_signature_similarity(
        &self,
        sig1: &EnhancedFunctionSignature,
        sig2: &EnhancedFunctionSignature,
    ) -> f64 {
        let mut total_score = 0.0;
        let mut weight_sum = 0.0;

        // Name similarity (weight: 0.3)
        let name_sim = self.calculate_name_similarity(&sig1.name, &sig2.name);
        total_score += name_sim * 0.3;
        weight_sum += 0.3;

        // Parameter count similarity (weight: 0.2)
        let param_count_sim = if sig1.parameters.len() == sig2.parameters.len() {
            1.0
        } else {
            let max_params = sig1.parameters.len().max(sig2.parameters.len()) as f64;
            let min_params = sig1.parameters.len().min(sig2.parameters.len()) as f64;
            if max_params == 0.0 { 1.0 } else { min_params / max_params }
        };
        total_score += param_count_sim * 0.2;
        weight_sum += 0.2;

        // Return type similarity (weight: 0.2)
        let return_type_sim = if sig1.return_type == sig2.return_type {
            1.0
        } else {
            0.0
        };
        total_score += return_type_sim * 0.2;
        weight_sum += 0.2;

        // Visibility similarity (weight: 0.1)
        let visibility_sim = if sig1.visibility == sig2.visibility {
            1.0
        } else {
            0.5
        };
        total_score += visibility_sim * 0.1;
        weight_sum += 0.1;

        // Complexity similarity (weight: 0.2)
        let complexity_sim = match (&sig1.complexity_metrics, &sig2.complexity_metrics) {
            (Some(m1), Some(m2)) => {
                let c1 = m1.cyclomatic_complexity as f64;
                let c2 = m2.cyclomatic_complexity as f64;
                let max_complexity = c1.max(c2);
                if max_complexity == 0.0 {
                    1.0
                } else {
                    1.0 - (c1 - c2).abs() / max_complexity
                }
            }
            (None, None) => 1.0,
            _ => 0.5, // One has metrics, other doesn't
        };
        total_score += complexity_sim * 0.2;
        weight_sum += 0.2;

        if weight_sum == 0.0 {
            0.0
        } else {
            total_score / weight_sum
        }
    }

    /// Calculate AST complexity
    fn calculate_ast_complexity(&self, ast: &ASTNode) -> f64 {
        let node_count = self.count_ast_nodes(ast) as f64;
        let depth = self.calculate_ast_depth(ast) as f64;

        // Simple complexity metric based on size and depth
        (node_count * 0.7) + (depth * 0.3)
    }

    /// Count nodes in AST
    fn count_ast_nodes(&self, ast: &ASTNode) -> usize {
        1 + ast.children.iter().map(|child| self.count_ast_nodes(child)).sum::<usize>()
    }

    /// Calculate AST depth
    fn calculate_ast_depth(&self, ast: &ASTNode) -> usize {
        if ast.children.is_empty() {
            1
        } else {
            1 + ast.children.iter().map(|child| self.calculate_ast_depth(child)).max().unwrap_or(0)
        }
    }

    /// Calculate change complexity
    fn calculate_change_complexity(
        &self,
        source: &CodeElement,
        target: &CodeElement,
        similarity_metrics: &Option<ComprehensiveSimilarityScore>,
        source_signature: Option<&EnhancedFunctionSignature>,
        target_signature: Option<&EnhancedFunctionSignature>,
    ) -> f64 {
        let mut complexity = 0.0;

        // Base complexity from similarity (inverse relationship)
        if let Some(metrics) = similarity_metrics {
            complexity += (1.0 - metrics.overall_similarity) * 2.0;
        } else {
            complexity += 1.0;
        }

        // Add complexity for name changes
        if source.name != target.name {
            complexity += 0.5;
        }

        // Add complexity for location changes
        if source.file_path != target.file_path {
            complexity += 1.0;
        } else if source.start_line != target.start_line {
            complexity += 0.3;
        }

        // Add complexity from signature changes
        if let (Some(src_sig), Some(tgt_sig)) = (source_signature, target_signature) {
            if let (Some(src_metrics), Some(tgt_metrics)) = (&src_sig.complexity_metrics, &tgt_sig.complexity_metrics) {
                let sig_complexity_change = (tgt_metrics.cyclomatic_complexity as f64
                    - src_metrics.cyclomatic_complexity as f64).abs();
                complexity += sig_complexity_change * 0.1;
            }
        }

        complexity
    }

    /// Assess impact of an addition
    fn assess_addition_impact(
        &self,
        target: &CodeElement,
        target_signature: Option<&EnhancedFunctionSignature>,
    ) -> ChangeImpact {
        let mut impact_level = ImpactLevel::Low;
        let mut effort_level = EffortLevel::Low;
        let mut risk_level = RiskLevel::Low;
        let mut affected_components = Vec::new();

        // Assess based on element type and complexity
        if let Some(signature) = target_signature {
            if let Some(metrics) = &signature.complexity_metrics {
                let complexity = metrics.cyclomatic_complexity;

                if complexity > 10 {
                    impact_level = ImpactLevel::Medium;
                    effort_level = EffortLevel::Medium;
                    risk_level = RiskLevel::Medium;
                }

                if complexity > 20 {
                    impact_level = ImpactLevel::High;
                    effort_level = EffortLevel::High;
                    risk_level = RiskLevel::High;
                }
            }

            // Public functions have higher impact
            if signature.visibility == smart_diff_semantic::Visibility::Public {
                impact_level = match impact_level {
                    ImpactLevel::Low => ImpactLevel::Medium,
                    ImpactLevel::Medium => ImpactLevel::High,
                    other => other,
                };
            }

            affected_components.push(target.name.clone());
        }

        ChangeImpact {
            impact_level,
            affected_components,
            implementation_effort: effort_level,
            risk_level,
            is_breaking_change: false, // Additions are generally not breaking
        }
    }

    /// Assess impact of a deletion
    fn assess_deletion_impact(
        &self,
        source: &CodeElement,
        source_signature: Option<&EnhancedFunctionSignature>,
    ) -> ChangeImpact {
        let mut impact_level = ImpactLevel::Medium; // Deletions generally have higher impact
        let mut effort_level = EffortLevel::Low;
        let mut risk_level = RiskLevel::Medium;
        let mut affected_components = Vec::new();
        let mut is_breaking_change = false;

        if let Some(signature) = source_signature {
            // Public functions being deleted are breaking changes
            if signature.visibility == smart_diff_semantic::Visibility::Public {
                impact_level = ImpactLevel::High;
                risk_level = RiskLevel::High;
                is_breaking_change = true;
            }

            // Complex functions have higher impact when deleted
            if let Some(metrics) = &signature.complexity_metrics {
                let complexity = metrics.cyclomatic_complexity;
                if complexity > 10 {
                    impact_level = ImpactLevel::High;
                    effort_level = EffortLevel::Medium;
                }
            }

            affected_components.push(source.name.clone());
        }

        ChangeImpact {
            impact_level,
            affected_components,
            implementation_effort: effort_level,
            risk_level,
            is_breaking_change,
        }
    }

    /// Assess impact of a modification
    fn assess_modification_impact(
        &self,
        source: &CodeElement,
        target: &CodeElement,
        similarity_metrics: &Option<ComprehensiveSimilarityScore>,
        source_signature: Option<&EnhancedFunctionSignature>,
        target_signature: Option<&EnhancedFunctionSignature>,
    ) -> ChangeImpact {
        let mut impact_level = ImpactLevel::Low;
        let mut effort_level = EffortLevel::Low;
        let mut risk_level = RiskLevel::Low;
        let mut affected_components = Vec::new();
        let mut is_breaking_change = false;

        // Assess based on similarity - lower similarity means higher impact
        if let Some(metrics) = similarity_metrics {
            if metrics.overall_similarity < 0.3 {
                impact_level = ImpactLevel::High;
                effort_level = EffortLevel::High;
                risk_level = RiskLevel::High;
            } else if metrics.overall_similarity < 0.6 {
                impact_level = ImpactLevel::Medium;
                effort_level = EffortLevel::Medium;
                risk_level = RiskLevel::Medium;
            }
        }

        // Cross-file moves have higher impact
        if source.file_path != target.file_path {
            impact_level = match impact_level {
                ImpactLevel::Low => ImpactLevel::Medium,
                ImpactLevel::Medium => ImpactLevel::High,
                other => other,
            };
            effort_level = EffortLevel::Medium;
        }

        // Signature changes assessment
        if let (Some(src_sig), Some(tgt_sig)) = (source_signature, target_signature) {
            // Parameter changes are potentially breaking
            if src_sig.parameters.len() != tgt_sig.parameters.len() {
                is_breaking_change = true;
                impact_level = ImpactLevel::High;
                risk_level = RiskLevel::High;
            }

            // Return type changes are potentially breaking
            if src_sig.return_type != tgt_sig.return_type {
                is_breaking_change = true;
                impact_level = ImpactLevel::High;
            }

            // Visibility changes
            if src_sig.visibility != tgt_sig.visibility {
                match (&src_sig.visibility, &tgt_sig.visibility) {
                    (smart_diff_semantic::Visibility::Public, _) => {
                        // Making public function less visible is breaking
                        is_breaking_change = true;
                        impact_level = ImpactLevel::High;
                    }
                    (_, smart_diff_semantic::Visibility::Public) => {
                        // Making function public increases impact
                        impact_level = match impact_level {
                            ImpactLevel::Low => ImpactLevel::Medium,
                            other => other,
                        };
                    }
                    _ => {}
                }
            }

            affected_components.push(source.name.clone());
            if source.name != target.name {
                affected_components.push(target.name.clone());
            }
        }

        ChangeImpact {
            impact_level,
            affected_components,
            implementation_effort: effort_level,
            risk_level,
            is_breaking_change,
        }
    }

    /// Generate human-readable change description
    fn generate_change_description(
        &self,
        change_type: &ChangeType,
        source: &CodeElement,
        target: &CodeElement,
        similarity_metrics: &Option<ComprehensiveSimilarityScore>,
    ) -> String {
        match change_type {
            ChangeType::Add => format!("Added new element '{}'", target.name),
            ChangeType::Delete => format!("Deleted element '{}'", source.name),
            ChangeType::Rename => {
                let similarity = similarity_metrics
                    .as_ref()
                    .map(|m| m.overall_similarity)
                    .unwrap_or(0.0);
                format!(
                    "Renamed '{}' to '{}' (similarity: {:.1}%)",
                    source.name, target.name, similarity * 100.0
                )
            }
            ChangeType::Move => {
                if source.file_path != target.file_path {
                    format!(
                        "Moved '{}' from {} to {}",
                        source.name, source.file_path, target.file_path
                    )
                } else {
                    format!(
                        "Moved '{}' from line {} to line {}",
                        source.name, source.start_line, target.start_line
                    )
                }
            }
            ChangeType::CrossFileMove => {
                format!(
                    "Moved '{}' from {}:{} to {}:{}",
                    source.name, source.file_path, source.start_line,
                    target.file_path, target.start_line
                )
            }
            ChangeType::Modify => {
                let similarity = similarity_metrics
                    .as_ref()
                    .map(|m| m.overall_similarity)
                    .unwrap_or(0.0);
                format!(
                    "Modified '{}' (similarity: {:.1}%)",
                    source.name, similarity * 100.0
                )
            }
            ChangeType::Split => {
                format!("Split '{}' into multiple elements", source.name)
            }
            ChangeType::Merge => {
                format!("Merged multiple elements into '{}'", target.name)
            }
        }
    }

    /// Get current configuration
    pub fn get_config(&self) -> &ChangeClassificationConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: ChangeClassificationConfig) {
        // Update tree edit distance configuration before moving config
        let tree_config = ZhangShashaConfig {
            enable_caching: true,
            enable_pruning: true,
            max_depth: config.max_ast_depth,
            ..Default::default()
        };
        self.tree_edit_distance.set_config(tree_config);

        self.config = config;
    }

    /// Enable or disable semantic analysis
    pub fn set_semantic_analysis(&mut self, enabled: bool) {
        if enabled && self.similarity_scorer.is_none() {
            self.similarity_scorer = Some(SimilarityScorer::new(self.language, SimilarityScoringConfig::default()));
        } else if !enabled {
            self.similarity_scorer = None;
        }
    }

    /// Detect if a change represents a function split
    pub fn detect_split(&self, source: &CodeElement, targets: &[CodeElement]) -> bool {
        targets.len() > 1
            && targets
                .iter()
                .all(|t| t.name.contains(&source.name) || source.name.contains(&t.name))
    }

    /// Detect if changes represent a function merge
    pub fn detect_merge(&self, sources: &[CodeElement], target: &CodeElement) -> bool {
        sources.len() > 1
            && sources
                .iter()
                .all(|s| s.name.contains(&target.name) || target.name.contains(&s.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smart_diff_parser::{NodeMetadata};
    use smart_diff_semantic::{
        EnhancedFunctionSignature, FunctionType, Visibility, TypeSignature,
        ParameterInfo, ComplexityMetrics
    };
    use std::collections::HashMap;

    fn create_test_code_element(name: &str, file_path: &str, start_line: usize) -> CodeElement {
        CodeElement {
            name: name.to_string(),
            file_path: file_path.to_string(),
            start_line,
            end_line: start_line + 10,
            element_type: "function".to_string(),
        }
    }

    fn create_test_ast_node(node_type: smart_diff_parser::NodeType, children: Vec<ASTNode>) -> ASTNode {
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

    fn create_test_signature(name: &str, complexity: u32) -> EnhancedFunctionSignature {
        EnhancedFunctionSignature {
            name: name.to_string(),
            parameters: Vec::new(),
            return_type: TypeSignature::Simple("void".to_string()),
            visibility: Visibility::Public,
            function_type: FunctionType::Regular,
            is_async: false,
            is_static: false,
            is_abstract: false,
            generic_parameters: Vec::new(),
            throws: Vec::new(),
            annotations: Vec::new(),
            complexity_metrics: ComplexityMetrics {
                cyclomatic_complexity: complexity,
                cognitive_complexity: complexity,
                nesting_depth: 2,
                parameter_count: 0,
                return_points: 1,
                lines_of_code: 10,
            },
            dependencies: Vec::new(),
        }
    }

    #[test]
    fn test_change_classification_config_default() {
        let config = ChangeClassificationConfig::default();

        assert_eq!(config.modification_threshold, 0.7);
        assert_eq!(config.rename_threshold, 0.8);
        assert_eq!(config.move_threshold, 0.9);
        assert!(config.enable_ast_analysis);
        assert!(config.enable_semantic_analysis);
        assert!(config.enable_confidence_scoring);
        assert_eq!(config.max_ast_depth, 20);
        assert!(config.enable_impact_analysis);
    }

    #[test]
    fn test_change_classifier_creation() {
        let classifier = ChangeClassifier::new(Language::Java);

        assert_eq!(classifier.language, Language::Java);
        assert!(classifier.similarity_scorer.is_some());
    }

    #[test]
    fn test_simple_addition_classification() {
        let classifier = ChangeClassifier::new(Language::Java);
        let target = create_test_code_element("newFunction", "test.java", 10);

        let change_type = classifier.classify_change(None, Some(&target));
        assert_eq!(change_type, ChangeType::Add);
    }

    #[test]
    fn test_simple_deletion_classification() {
        let classifier = ChangeClassifier::new(Language::Java);
        let source = create_test_code_element("oldFunction", "test.java", 10);

        let change_type = classifier.classify_change(Some(&source), None);
        assert_eq!(change_type, ChangeType::Delete);
    }

    #[test]
    fn test_simple_rename_classification() {
        let classifier = ChangeClassifier::new(Language::Java);
        let source = create_test_code_element("oldFunction", "test.java", 10);
        let target = create_test_code_element("newFunction", "test.java", 10);

        let change_type = classifier.classify_change(Some(&source), Some(&target));
        assert_eq!(change_type, ChangeType::Rename);
    }

    #[test]
    fn test_simple_move_classification() {
        let classifier = ChangeClassifier::new(Language::Java);
        let source = create_test_code_element("function", "test.java", 10);
        let target = create_test_code_element("function", "test.java", 20);

        let change_type = classifier.classify_change(Some(&source), Some(&target));
        assert_eq!(change_type, ChangeType::Move);
    }

    #[test]
    fn test_cross_file_move_classification() {
        let classifier = ChangeClassifier::new(Language::Java);
        let source = create_test_code_element("function", "old.java", 10);
        let target = create_test_code_element("function", "new.java", 10);

        let change_type = classifier.classify_change(Some(&source), Some(&target));
        assert_eq!(change_type, ChangeType::CrossFileMove);
    }

    #[test]
    fn test_modification_classification() {
        let classifier = ChangeClassifier::new(Language::Java);
        let source = create_test_code_element("function", "test.java", 10);
        let target = create_test_code_element("function", "test.java", 10);

        let change_type = classifier.classify_change(Some(&source), Some(&target));
        assert_eq!(change_type, ChangeType::Modify);
    }

    #[test]
    fn test_detailed_addition_classification() -> Result<()> {
        let classifier = ChangeClassifier::new(Language::Java);
        let target = create_test_code_element("newFunction", "test.java", 10);
        let target_signature = create_test_signature("newFunction", 5);

        let result = classifier.classify_addition(&target, None, Some(&target_signature))?;

        assert_eq!(result.change_type, ChangeType::Add);
        assert_eq!(result.confidence, 1.0);
        assert!(!result.impact.is_breaking_change);
        assert_eq!(result.impact.impact_level, ImpactLevel::Low);

        Ok(())
    }

    #[test]
    fn test_detailed_deletion_classification() -> Result<()> {
        let classifier = ChangeClassifier::new(Language::Java);
        let source = create_test_code_element("oldFunction", "test.java", 10);
        let source_signature = create_test_signature("oldFunction", 5);

        let result = classifier.classify_deletion(&source, None, Some(&source_signature))?;

        assert_eq!(result.change_type, ChangeType::Delete);
        assert_eq!(result.confidence, 1.0);
        assert!(result.impact.is_breaking_change); // Public function deletion is breaking
        assert_eq!(result.impact.impact_level, ImpactLevel::High);

        Ok(())
    }

    #[test]
    fn test_name_similarity_calculation() {
        let classifier = ChangeClassifier::new(Language::Java);

        // Identical names
        assert_eq!(classifier.calculate_name_similarity("function", "function"), 1.0);

        // Completely different names
        assert_eq!(classifier.calculate_name_similarity("function", "method"), 0.0);

        // Similar names
        let similarity = classifier.calculate_name_similarity("calculateSum", "calculateTotal");
        assert!(similarity > 0.5 && similarity < 1.0);

        // Case differences
        let similarity = classifier.calculate_name_similarity("Function", "function");
        assert!(similarity > 0.8);
    }

    #[test]
    fn test_levenshtein_distance() {
        let classifier = ChangeClassifier::new(Language::Java);

        assert_eq!(classifier.levenshtein_distance("", ""), 0);
        assert_eq!(classifier.levenshtein_distance("abc", ""), 3);
        assert_eq!(classifier.levenshtein_distance("", "abc"), 3);
        assert_eq!(classifier.levenshtein_distance("abc", "abc"), 0);
        assert_eq!(classifier.levenshtein_distance("abc", "ab"), 1);
        assert_eq!(classifier.levenshtein_distance("abc", "axc"), 1);
        assert_eq!(classifier.levenshtein_distance("abc", "def"), 3);
    }

    #[test]
    fn test_signature_similarity_calculation() {
        let classifier = ChangeClassifier::new(Language::Java);

        let sig1 = create_test_signature("function", 5);
        let sig2 = create_test_signature("function", 5);

        // Identical signatures
        let similarity = classifier.calculate_signature_similarity(&sig1, &sig2);
        assert_eq!(similarity, 1.0);

        // Different names
        let sig3 = create_test_signature("method", 5);
        let similarity = classifier.calculate_signature_similarity(&sig1, &sig3);
        assert!(similarity < 1.0);

        // Different complexity
        let sig4 = create_test_signature("function", 15);
        let similarity = classifier.calculate_signature_similarity(&sig1, &sig4);
        assert!(similarity < 1.0);
    }

    #[test]
    fn test_ast_complexity_calculation() {
        let classifier = ChangeClassifier::new(Language::Java);

        // Simple leaf node
        let simple_ast = create_test_ast_node(smart_diff_parser::NodeType::Literal, Vec::new());
        let complexity = classifier.calculate_ast_complexity(&simple_ast);
        assert_eq!(complexity, 1.0); // 1 node * 0.7 + 1 depth * 0.3

        // Complex nested structure
        let complex_ast = create_test_ast_node(
            smart_diff_parser::NodeType::Function,
            vec![
                create_test_ast_node(smart_diff_parser::NodeType::Block, vec![
                    create_test_ast_node(smart_diff_parser::NodeType::IfStatement, Vec::new()),
                    create_test_ast_node(smart_diff_parser::NodeType::WhileStatement, Vec::new()),
                ]),
            ],
        );
        let complexity = classifier.calculate_ast_complexity(&complex_ast);
        assert!(complexity > 1.0);
    }

    #[test]
    fn test_impact_assessment_levels() {
        let classifier = ChangeClassifier::new(Language::Java);

        // Low complexity function
        let low_complexity_sig = create_test_signature("simpleFunction", 2);
        let target = create_test_code_element("simpleFunction", "test.java", 10);
        let impact = classifier.assess_addition_impact(&target, Some(&low_complexity_sig));
        assert_eq!(impact.impact_level, ImpactLevel::Low);

        // High complexity function
        let high_complexity_sig = create_test_signature("complexFunction", 25);
        let target = create_test_code_element("complexFunction", "test.java", 10);
        let impact = classifier.assess_addition_impact(&target, Some(&high_complexity_sig));
        assert_eq!(impact.impact_level, ImpactLevel::High);
    }

    #[test]
    fn test_change_description_generation() {
        let classifier = ChangeClassifier::new(Language::Java);

        let source = create_test_code_element("oldFunction", "old.java", 10);
        let target = create_test_code_element("newFunction", "new.java", 20);

        // Test rename description
        let description = classifier.generate_change_description(
            &ChangeType::Rename, &source, &target, &None
        );
        assert!(description.contains("Renamed"));
        assert!(description.contains("oldFunction"));
        assert!(description.contains("newFunction"));

        // Test move description
        let description = classifier.generate_change_description(
            &ChangeType::CrossFileMove, &source, &target, &None
        );
        assert!(description.contains("Moved"));
        assert!(description.contains("old.java"));
        assert!(description.contains("new.java"));
    }

    #[test]
    fn test_configuration_updates() {
        let mut classifier = ChangeClassifier::new(Language::Java);

        let original_threshold = classifier.get_config().modification_threshold;
        assert_eq!(original_threshold, 0.7);

        let new_config = ChangeClassificationConfig {
            modification_threshold: 0.8,
            rename_threshold: 0.9,
            move_threshold: 0.95,
            enable_ast_analysis: false,
            enable_semantic_analysis: false,
            enable_confidence_scoring: false,
            max_ast_depth: 10,
            enable_impact_analysis: false,
        };

        classifier.set_config(new_config);

        assert_eq!(classifier.get_config().modification_threshold, 0.8);
        assert_eq!(classifier.get_config().rename_threshold, 0.9);
        assert_eq!(classifier.get_config().move_threshold, 0.95);
        assert!(!classifier.get_config().enable_ast_analysis);
        assert!(!classifier.get_config().enable_semantic_analysis);
    }

    #[test]
    fn test_semantic_analysis_toggle() {
        let mut classifier = ChangeClassifier::new(Language::Java);

        // Initially enabled
        assert!(classifier.similarity_scorer.is_some());

        // Disable semantic analysis
        classifier.set_semantic_analysis(false);
        assert!(classifier.similarity_scorer.is_none());

        // Re-enable semantic analysis
        classifier.set_semantic_analysis(true);
        assert!(classifier.similarity_scorer.is_some());
    }
}
