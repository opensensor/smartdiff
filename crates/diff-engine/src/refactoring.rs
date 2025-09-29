//! Comprehensive Refactoring Pattern Detection System
//!
//! This module implements an advanced refactoring pattern detection system that identifies
//! common code refactoring patterns with confidence scoring, detailed analysis, and
//! integration with change classification and similarity analysis.

use crate::changes::{ChangeClassifier, DetailedChangeClassification};
use crate::similarity_scorer::{SimilarityScorer, SimilarityScoringConfig, ComprehensiveSimilarityScore};
use smart_diff_parser::{Change, RefactoringType, ChangeType, CodeElement, ASTNode, Language};
use smart_diff_semantic::EnhancedFunctionSignature;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use anyhow::Result;

/// Configuration for refactoring pattern detection
#[derive(Debug, Clone)]
pub struct RefactoringDetectionConfig {
    /// Minimum confidence threshold for pattern detection
    pub min_confidence_threshold: f64,
    /// Enable extract method detection
    pub enable_extract_method: bool,
    /// Enable inline method detection
    pub enable_inline_method: bool,
    /// Enable rename detection
    pub enable_rename_detection: bool,
    /// Enable move detection
    pub enable_move_detection: bool,
    /// Enable extract class detection
    pub enable_extract_class: bool,
    /// Enable inline class detection
    pub enable_inline_class: bool,
    /// Enable change signature detection
    pub enable_change_signature: bool,
    /// Maximum distance for related changes
    pub max_related_distance: usize,
    /// Enable complex pattern detection
    pub enable_complex_patterns: bool,
}

impl Default for RefactoringDetectionConfig {
    fn default() -> Self {
        Self {
            min_confidence_threshold: 0.7,
            enable_extract_method: true,
            enable_inline_method: true,
            enable_rename_detection: true,
            enable_move_detection: true,
            enable_extract_class: true,
            enable_inline_class: true,
            enable_change_signature: true,
            max_related_distance: 50,
            enable_complex_patterns: true,
        }
    }
}

/// Comprehensive refactoring pattern detector
pub struct RefactoringDetector {
    config: RefactoringDetectionConfig,
    change_classifier: Option<ChangeClassifier>,
    similarity_scorer: Option<SimilarityScorer>,
    language: Language,
}

/// Detected refactoring pattern with detailed analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringPattern {
    /// Type of refactoring pattern
    pub pattern_type: RefactoringType,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Human-readable description
    pub description: String,
    /// Elements affected by this refactoring
    pub affected_elements: Vec<String>,
    /// Detailed analysis of the refactoring
    pub analysis: RefactoringAnalysis,
    /// Supporting evidence for the pattern
    pub evidence: Vec<RefactoringEvidence>,
    /// Related changes that support this pattern
    pub related_changes: Vec<String>,
    /// Complexity assessment
    pub complexity: RefactoringComplexity,
}

/// Detailed analysis of a refactoring pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringAnalysis {
    /// Specific characteristics of this refactoring
    pub characteristics: Vec<RefactoringCharacteristic>,
    /// Before and after comparison
    pub before_after: Option<BeforeAfterComparison>,
    /// Impact assessment
    pub impact: RefactoringImpact,
    /// Quality metrics
    pub quality_metrics: RefactoringQualityMetrics,
}

/// Characteristic of a refactoring pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringCharacteristic {
    /// Type of characteristic
    pub characteristic_type: RefactoringCharacteristicType,
    /// Value or description
    pub value: String,
    /// Confidence in this characteristic
    pub confidence: f64,
}

/// Types of refactoring characteristics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefactoringCharacteristicType {
    /// Code extraction pattern
    CodeExtraction,
    /// Code inlining pattern
    CodeInlining,
    /// Name change pattern
    NameChange,
    /// Location change pattern
    LocationChange,
    /// Signature change pattern
    SignatureChange,
    /// Complexity change pattern
    ComplexityChange,
    /// Dependency change pattern
    DependencyChange,
    /// Structure change pattern
    StructureChange,
}

/// Evidence supporting a refactoring pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringEvidence {
    /// Type of evidence
    pub evidence_type: RefactoringEvidenceType,
    /// Description of the evidence
    pub description: String,
    /// Strength of evidence (0.0 to 1.0)
    pub strength: f64,
    /// Supporting data
    pub data: HashMap<String, String>,
}

/// Types of refactoring evidence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefactoringEvidenceType {
    /// Code similarity evidence
    CodeSimilarity,
    /// Name pattern evidence
    NamePattern,
    /// Structure pattern evidence
    StructurePattern,
    /// Timing evidence (related changes)
    TimingEvidence,
    /// Location evidence
    LocationEvidence,
    /// Dependency evidence
    DependencyEvidence,
}

/// Before and after comparison for refactoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeforeAfterComparison {
    /// Elements before refactoring
    pub before_elements: Vec<String>,
    /// Elements after refactoring
    pub after_elements: Vec<String>,
    /// Similarity metrics
    pub similarity_metrics: Option<ComprehensiveSimilarityScore>,
    /// Size comparison
    pub size_comparison: SizeComparison,
}

/// Size comparison metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeComparison {
    /// Lines of code before
    pub lines_before: usize,
    /// Lines of code after
    pub lines_after: usize,
    /// Functions before
    pub functions_before: usize,
    /// Functions after
    pub functions_after: usize,
    /// Complexity before
    pub complexity_before: f64,
    /// Complexity after
    pub complexity_after: f64,
}

/// Impact assessment for refactoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringImpact {
    /// Overall impact level
    pub impact_level: RefactoringImpactLevel,
    /// Affected files
    pub affected_files: Vec<String>,
    /// Affected functions
    pub affected_functions: Vec<String>,
    /// Breaking change indicator
    pub is_breaking_change: bool,
    /// API compatibility impact
    pub api_compatibility: ApiCompatibilityImpact,
}

/// Levels of refactoring impact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefactoringImpactLevel {
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

/// API compatibility impact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApiCompatibilityImpact {
    /// No API impact
    None,
    /// Backward compatible
    BackwardCompatible,
    /// Potentially breaking
    PotentiallyBreaking,
    /// Breaking change
    Breaking,
}

/// Quality metrics for refactoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringQualityMetrics {
    /// Code quality improvement score
    pub quality_improvement: f64,
    /// Maintainability impact
    pub maintainability_impact: f64,
    /// Readability impact
    pub readability_impact: f64,
    /// Testability impact
    pub testability_impact: f64,
    /// Performance impact
    pub performance_impact: f64,
}

/// Complexity assessment for refactoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringComplexity {
    /// Overall complexity level
    pub complexity_level: RefactoringComplexityLevel,
    /// Number of elements involved
    pub elements_involved: usize,
    /// Number of files affected
    pub files_affected: usize,
    /// Estimated effort
    pub estimated_effort: RefactoringEffort,
}

/// Levels of refactoring complexity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefactoringComplexityLevel {
    /// Simple refactoring
    Simple,
    /// Moderate complexity
    Moderate,
    /// Complex refactoring
    Complex,
    /// Very complex refactoring
    VeryComplex,
}

/// Estimated effort for refactoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefactoringEffort {
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

impl RefactoringDetector {
    /// Create a new refactoring detector with default configuration
    pub fn new(language: Language) -> Self {
        Self {
            config: RefactoringDetectionConfig::default(),
            change_classifier: Some(ChangeClassifier::new(language)),
            similarity_scorer: Some(SimilarityScorer::new(language, SimilarityScoringConfig::default())),
            language,
        }
    }

    /// Create a new refactoring detector with custom configuration
    pub fn with_config(language: Language, config: RefactoringDetectionConfig) -> Self {
        Self {
            config,
            change_classifier: Some(ChangeClassifier::new(language)),
            similarity_scorer: Some(SimilarityScorer::new(language, SimilarityScoringConfig::default())),
            language,
        }
    }

    /// Create a minimal refactoring detector without advanced analysis
    pub fn minimal(language: Language) -> Self {
        Self {
            config: RefactoringDetectionConfig::default(),
            change_classifier: None,
            similarity_scorer: None,
            language,
        }
    }

    /// Detect refactoring patterns from a set of changes
    pub fn detect_patterns(&self, changes: &[Change]) -> Vec<RefactoringPattern> {
        let mut patterns = Vec::new();

        if changes.is_empty() {
            return patterns;
        }

        // Group related changes for pattern analysis
        let change_groups = self.group_related_changes(changes);

        // Detect different types of refactoring patterns
        if self.config.enable_extract_method {
            patterns.extend(self.detect_extract_method_patterns(&change_groups));
        }

        if self.config.enable_inline_method {
            patterns.extend(self.detect_inline_method_patterns(&change_groups));
        }

        if self.config.enable_rename_detection {
            patterns.extend(self.detect_rename_patterns(&change_groups));
        }

        if self.config.enable_move_detection {
            patterns.extend(self.detect_move_patterns(&change_groups));
        }

        if self.config.enable_extract_class {
            patterns.extend(self.detect_extract_class_patterns(&change_groups));
        }

        if self.config.enable_inline_class {
            patterns.extend(self.detect_inline_class_patterns(&change_groups));
        }

        if self.config.enable_change_signature {
            patterns.extend(self.detect_change_signature_patterns(&change_groups));
        }

        if self.config.enable_complex_patterns {
            patterns.extend(self.detect_complex_patterns(&change_groups));
        }

        // Filter patterns by confidence threshold
        patterns.retain(|p| p.confidence >= self.config.min_confidence_threshold);

        // Sort by confidence (highest first)
        patterns.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));

        patterns
    }

    /// Detect refactoring patterns with detailed analysis
    pub fn detect_patterns_detailed(
        &self,
        changes: &[Change],
        source_asts: &HashMap<String, ASTNode>,
        target_asts: &HashMap<String, ASTNode>,
        source_signatures: &HashMap<String, EnhancedFunctionSignature>,
        target_signatures: &HashMap<String, EnhancedFunctionSignature>,
    ) -> Result<Vec<RefactoringPattern>> {
        let mut patterns = Vec::new();

        if changes.is_empty() {
            return Ok(patterns);
        }

        // Enhanced pattern detection with AST and signature analysis
        let change_groups = self.group_related_changes(changes);

        // Detect patterns with detailed analysis
        if self.config.enable_extract_method {
            patterns.extend(self.detect_extract_method_detailed(
                &change_groups, source_asts, target_asts, source_signatures, target_signatures
            )?);
        }

        if self.config.enable_inline_method {
            patterns.extend(self.detect_inline_method_detailed(
                &change_groups, source_asts, target_asts, source_signatures, target_signatures
            )?);
        }

        if self.config.enable_rename_detection {
            patterns.extend(self.detect_rename_patterns_detailed(
                &change_groups, source_signatures, target_signatures
            )?);
        }

        if self.config.enable_move_detection {
            patterns.extend(self.detect_move_patterns_detailed(
                &change_groups, source_asts, target_asts, source_signatures, target_signatures
            )?);
        }

        // Filter and sort patterns
        patterns.retain(|p| p.confidence >= self.config.min_confidence_threshold);
        patterns.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));

        Ok(patterns)
    }

    /// Group related changes for pattern analysis
    fn group_related_changes(&self, changes: &[Change]) -> Vec<Vec<&Change>> {
        let mut groups = Vec::new();
        let mut processed = HashSet::new();

        for (i, change) in changes.iter().enumerate() {
            if processed.contains(&i) {
                continue;
            }

            let mut group = vec![change];
            processed.insert(i);

            // Find related changes
            for (j, other_change) in changes.iter().enumerate() {
                if i == j || processed.contains(&j) {
                    continue;
                }

                if self.are_changes_related(change, other_change) {
                    group.push(other_change);
                    processed.insert(j);
                }
            }

            groups.push(group);
        }

        groups
    }

    /// Check if two changes are related
    fn are_changes_related(&self, change1: &Change, change2: &Change) -> bool {
        // Check file proximity
        if let (Some(source1), Some(source2)) = (&change1.source, &change2.source) {
            if source1.file_path == source2.file_path {
                let line_distance = (source1.start_line as i32 - source2.start_line as i32).abs();
                if line_distance <= self.config.max_related_distance as i32 {
                    return true;
                }
            }
        }

        // Check name similarity
        if let (Some(source1), Some(source2)) = (&change1.source, &change2.source) {
            let name_similarity = self.calculate_name_similarity(&source1.name, &source2.name);
            if name_similarity > 0.7 {
                return true;
            }
        }

        // Check if one is addition and other is deletion (potential move/rename)
        match (&change1.change_type, &change2.change_type) {
            (ChangeType::Add, ChangeType::Delete) | (ChangeType::Delete, ChangeType::Add) => {
                if let (Some(source1), Some(source2)) = (&change1.source, &change2.source) {
                    let name_similarity = self.calculate_name_similarity(&source1.name, &source2.name);
                    return name_similarity > 0.5;
                }
                if let (Some(target1), Some(target2)) = (&change1.target, &change2.target) {
                    let name_similarity = self.calculate_name_similarity(&target1.name, &target2.name);
                    return name_similarity > 0.5;
                }
            }
            _ => {}
        }

        false
    }

    /// Calculate name similarity using simple edit distance
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

    /// Calculate Levenshtein distance
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

    /// Detect extract method patterns
    fn detect_extract_method_patterns(&self, change_groups: &[Vec<&Change>]) -> Vec<RefactoringPattern> {
        let mut patterns = Vec::new();

        for group in change_groups {
            if let Some(pattern) = self.analyze_extract_method_group(group) {
                patterns.push(pattern);
            }
        }

        patterns
    }

    /// Analyze a group of changes for extract method pattern
    fn analyze_extract_method_group(&self, changes: &[&Change]) -> Option<RefactoringPattern> {
        // Look for pattern: one modification (source function) + one addition (new function)
        let modifications: Vec<_> = changes.iter().filter(|c| c.change_type == ChangeType::Modify).collect();
        let additions: Vec<_> = changes.iter().filter(|c| c.change_type == ChangeType::Add).collect();

        if modifications.len() == 1 && additions.len() == 1 {
            let modified = modifications[0];
            let added = additions[0];

            // Check if the added function has a name that suggests extraction
            if let Some(target) = &added.target {
                let confidence = self.calculate_extract_method_confidence(modified, added);

                if confidence > 0.5 {
                    return Some(RefactoringPattern {
                        pattern_type: RefactoringType::ExtractMethod,
                        confidence,
                        description: format!(
                            "Extracted method '{}' from '{}'",
                            target.name,
                            modified.source.as_ref().map(|s| &s.name).unwrap_or("unknown")
                        ),
                        affected_elements: vec![
                            target.name.clone(),
                            modified.source.as_ref().map(|s| s.name.clone()).unwrap_or_default()
                        ],
                        analysis: self.create_extract_method_analysis(modified, added),
                        evidence: self.gather_extract_method_evidence(modified, added),
                        related_changes: changes.iter().map(|c| format!("{:?}", c.change_type)).collect(),
                        complexity: self.assess_extract_method_complexity(changes),
                    });
                }
            }
        }

        None
    }

    /// Calculate confidence for extract method pattern
    fn calculate_extract_method_confidence(&self, modified: &Change, added: &Change) -> f64 {
        let mut confidence = 0.0;

        // Base confidence for the pattern structure
        confidence += 0.4;

        // Check name patterns (extracted methods often have descriptive names)
        if let Some(target) = &added.target {
            if target.name.len() > 5 && (
                target.name.contains("extract") ||
                target.name.contains("helper") ||
                target.name.contains("validate") ||
                target.name.contains("calculate") ||
                target.name.contains("process")
            ) {
                confidence += 0.2;
            }
        }

        // Check if modified function became simpler (lower confidence without AST analysis)
        confidence += 0.1;

        // Check file proximity
        if let (Some(mod_source), Some(add_target)) = (&modified.source, &added.target) {
            if mod_source.file_path == add_target.file_path {
                confidence += 0.2;
            }
        }

        // Check similarity score if available
        if let Some(similarity) = modified.details.similarity_score {
            if similarity > 0.7 {
                confidence += 0.1;
            }
        }

        confidence.min(1.0)
    }

    /// Detect inline method patterns
    fn detect_inline_method_patterns(&self, change_groups: &[Vec<&Change>]) -> Vec<RefactoringPattern> {
        let mut patterns = Vec::new();

        for group in change_groups {
            if let Some(pattern) = self.analyze_inline_method_group(group) {
                patterns.push(pattern);
            }
        }

        patterns
    }

    /// Analyze a group of changes for inline method pattern
    fn analyze_inline_method_group(&self, changes: &[&Change]) -> Option<RefactoringPattern> {
        // Look for pattern: one deletion (inlined function) + one modification (target function)
        let deletions: Vec<_> = changes.iter().filter(|c| c.change_type == ChangeType::Delete).collect();
        let modifications: Vec<_> = changes.iter().filter(|c| c.change_type == ChangeType::Modify).collect();

        if deletions.len() == 1 && modifications.len() == 1 {
            let deleted = deletions[0];
            let modified = modifications[0];

            let confidence = self.calculate_inline_method_confidence(deleted, modified);

            if confidence > 0.5 {
                return Some(RefactoringPattern {
                    pattern_type: RefactoringType::InlineMethod,
                    confidence,
                    description: format!(
                        "Inlined method '{}' into '{}'",
                        deleted.source.as_ref().map(|s| &s.name).unwrap_or("unknown"),
                        modified.target.as_ref().map(|t| &t.name).unwrap_or("unknown")
                    ),
                    affected_elements: vec![
                        deleted.source.as_ref().map(|s| s.name.clone()).unwrap_or_default(),
                        modified.target.as_ref().map(|t| t.name.clone()).unwrap_or_default()
                    ],
                    analysis: self.create_inline_method_analysis(deleted, modified),
                    evidence: self.gather_inline_method_evidence(deleted, modified),
                    related_changes: changes.iter().map(|c| format!("{:?}", c.change_type)).collect(),
                    complexity: self.assess_inline_method_complexity(changes),
                });
            }
        }

        None
    }

    /// Calculate confidence for inline method pattern
    fn calculate_inline_method_confidence(&self, deleted: &Change, modified: &Change) -> f64 {
        let mut confidence = 0.0;

        // Base confidence for the pattern structure
        confidence += 0.4;

        // Check if deleted method was small (typical for inlining candidates)
        if let Some(source) = &deleted.source {
            let method_size = source.end_line - source.start_line;
            if method_size <= 10 {
                confidence += 0.2;
            } else if method_size <= 20 {
                confidence += 0.1;
            }
        }

        // Check file proximity
        if let (Some(del_source), Some(mod_target)) = (&deleted.source, &modified.target) {
            if del_source.file_path == mod_target.file_path {
                confidence += 0.2;
            }
        }

        // Check if modified function became more complex
        confidence += 0.1;

        // Check similarity score if available
        if let Some(similarity) = modified.details.similarity_score {
            if similarity > 0.6 {
                confidence += 0.1;
            }
        }

        confidence.min(1.0)
    }

    /// Detect rename patterns
    fn detect_rename_patterns(&self, change_groups: &[Vec<&Change>]) -> Vec<RefactoringPattern> {
        let mut patterns = Vec::new();

        for group in change_groups {
            if let Some(pattern) = self.analyze_rename_group(group) {
                patterns.push(pattern);
            }
        }

        patterns
    }

    /// Analyze a group of changes for rename pattern
    fn analyze_rename_group(&self, changes: &[&Change]) -> Option<RefactoringPattern> {
        // Look for rename changes or high-similarity modify changes
        for change in changes {
            if change.change_type == ChangeType::Rename {
                let confidence = 0.9; // High confidence for explicit renames

                return Some(RefactoringPattern {
                    pattern_type: RefactoringType::RenameMethod,
                    confidence,
                    description: format!(
                        "Renamed '{}' to '{}'",
                        change.source.as_ref().map(|s| &s.name).unwrap_or("unknown"),
                        change.target.as_ref().map(|t| &t.name).unwrap_or("unknown")
                    ),
                    affected_elements: vec![
                        change.source.as_ref().map(|s| s.name.clone()).unwrap_or_default(),
                        change.target.as_ref().map(|t| t.name.clone()).unwrap_or_default()
                    ],
                    analysis: self.create_rename_analysis(change),
                    evidence: self.gather_rename_evidence(change),
                    related_changes: vec![format!("{:?}", change.change_type)],
                    complexity: self.assess_rename_complexity(changes),
                });
            } else if change.change_type == ChangeType::Modify {
                // Check for implicit rename (high similarity but different names)
                if let (Some(source), Some(target)) = (&change.source, &change.target) {
                    if source.name != target.name {
                        if let Some(similarity) = change.details.similarity_score {
                            if similarity > 0.8 {
                                return Some(RefactoringPattern {
                                    pattern_type: RefactoringType::RenameMethod,
                                    confidence: similarity * 0.9,
                                    description: format!(
                                        "Renamed '{}' to '{}' (with modifications)",
                                        source.name, target.name
                                    ),
                                    affected_elements: vec![source.name.clone(), target.name.clone()],
                                    analysis: self.create_rename_analysis(change),
                                    evidence: self.gather_rename_evidence(change),
                                    related_changes: vec![format!("{:?}", change.change_type)],
                                    complexity: self.assess_rename_complexity(changes),
                                });
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Detect move patterns
    fn detect_move_patterns(&self, change_groups: &[Vec<&Change>]) -> Vec<RefactoringPattern> {
        let mut patterns = Vec::new();

        for group in change_groups {
            if let Some(pattern) = self.analyze_move_group(group) {
                patterns.push(pattern);
            }
        }

        patterns
    }

    /// Analyze a group of changes for move pattern
    fn analyze_move_group(&self, changes: &[&Change]) -> Option<RefactoringPattern> {
        for change in changes {
            match change.change_type {
                ChangeType::Move | ChangeType::CrossFileMove => {
                    let confidence = if change.change_type == ChangeType::CrossFileMove { 0.9 } else { 0.8 };

                    return Some(RefactoringPattern {
                        pattern_type: RefactoringType::MoveMethod,
                        confidence,
                        description: format!(
                            "Moved '{}' from {} to {}",
                            change.source.as_ref().map(|s| &s.name).unwrap_or("unknown"),
                            change.source.as_ref().map(|s| &s.file_path).unwrap_or("unknown"),
                            change.target.as_ref().map(|t| &t.file_path).unwrap_or("unknown")
                        ),
                        affected_elements: vec![
                            change.source.as_ref().map(|s| s.name.clone()).unwrap_or_default()
                        ],
                        analysis: self.create_move_analysis(change),
                        evidence: self.gather_move_evidence(change),
                        related_changes: vec![format!("{:?}", change.change_type)],
                        complexity: self.assess_move_complexity(changes),
                    });
                }
                _ => {}
            }
        }

        None
    }

    /// Detect extract class patterns
    fn detect_extract_class_patterns(&self, change_groups: &[Vec<&Change>]) -> Vec<RefactoringPattern> {
        let mut patterns = Vec::new();

        // Look for multiple methods being moved to a new class
        for group in change_groups {
            let additions: Vec<_> = group.iter().filter(|c| c.change_type == ChangeType::Add).collect();
            let modifications: Vec<_> = group.iter().filter(|c| c.change_type == ChangeType::Modify).collect();

            if additions.len() >= 2 && modifications.len() >= 1 {
                // Check if additions are in the same new file
                if let Some(first_addition) = additions.first() {
                    if let Some(target) = &first_addition.target {
                        let same_file = additions.iter().all(|add| {
                            add.target.as_ref().map(|t| &t.file_path) == Some(&target.file_path)
                        });

                        if same_file {
                            let confidence = 0.7 + (additions.len() as f64 * 0.05).min(0.2);

                            patterns.push(RefactoringPattern {
                                pattern_type: RefactoringType::ExtractClass,
                                confidence,
                                description: format!(
                                    "Extracted {} methods to new class in {}",
                                    additions.len(),
                                    target.file_path
                                ),
                                affected_elements: additions.iter()
                                    .filter_map(|add| add.target.as_ref().map(|t| t.name.clone()))
                                    .collect(),
                                analysis: self.create_extract_class_analysis(group),
                                evidence: self.gather_extract_class_evidence(group),
                                related_changes: group.iter().map(|c| format!("{:?}", c.change_type)).collect(),
                                complexity: self.assess_extract_class_complexity(group),
                            });
                        }
                    }
                }
            }
        }

        patterns
    }

    /// Detect inline class patterns
    fn detect_inline_class_patterns(&self, change_groups: &[Vec<&Change>]) -> Vec<RefactoringPattern> {
        let mut patterns = Vec::new();

        // Look for multiple methods being removed and one class being modified
        for group in change_groups {
            let deletions: Vec<_> = group.iter().filter(|c| c.change_type == ChangeType::Delete).collect();
            let modifications: Vec<_> = group.iter().filter(|c| c.change_type == ChangeType::Modify).collect();

            if deletions.len() >= 2 && modifications.len() >= 1 {
                // Check if deletions are from the same file
                if let Some(first_deletion) = deletions.first() {
                    if let Some(source) = &first_deletion.source {
                        let same_file = deletions.iter().all(|del| {
                            del.source.as_ref().map(|s| &s.file_path) == Some(&source.file_path)
                        });

                        if same_file {
                            let confidence = 0.6 + (deletions.len() as f64 * 0.05).min(0.2);

                            patterns.push(RefactoringPattern {
                                pattern_type: RefactoringType::InlineClass,
                                confidence,
                                description: format!(
                                    "Inlined {} methods from {} into other classes",
                                    deletions.len(),
                                    source.file_path
                                ),
                                affected_elements: deletions.iter()
                                    .filter_map(|del| del.source.as_ref().map(|s| s.name.clone()))
                                    .collect(),
                                analysis: self.create_inline_class_analysis(group),
                                evidence: self.gather_inline_class_evidence(group),
                                related_changes: group.iter().map(|c| format!("{:?}", c.change_type)).collect(),
                                complexity: self.assess_inline_class_complexity(group),
                            });
                        }
                    }
                }
            }
        }

        patterns
    }

    /// Detect change signature patterns
    fn detect_change_signature_patterns(&self, change_groups: &[Vec<&Change>]) -> Vec<RefactoringPattern> {
        let mut patterns = Vec::new();

        for group in change_groups {
            for change in group {
                if change.change_type == ChangeType::Modify {
                    // Check if this looks like a signature change
                    if let Some(refactoring_type) = &change.details.refactoring_type {
                        if *refactoring_type == RefactoringType::ChangeSignature {
                            patterns.push(RefactoringPattern {
                                pattern_type: RefactoringType::ChangeSignature,
                                confidence: 0.8,
                                description: format!(
                                    "Changed signature of '{}'",
                                    change.source.as_ref().map(|s| &s.name).unwrap_or("unknown")
                                ),
                                affected_elements: vec![
                                    change.source.as_ref().map(|s| s.name.clone()).unwrap_or_default()
                                ],
                                analysis: self.create_change_signature_analysis(change),
                                evidence: self.gather_change_signature_evidence(change),
                                related_changes: vec![format!("{:?}", change.change_type)],
                                complexity: self.assess_change_signature_complexity(group),
                            });
                        }
                    }
                }
            }
        }

        patterns
    }

    /// Detect complex patterns (combinations of simpler patterns)
    fn detect_complex_patterns(&self, change_groups: &[Vec<&Change>]) -> Vec<RefactoringPattern> {
        let mut patterns = Vec::new();

        // Look for complex refactoring patterns that combine multiple simple patterns
        for group in change_groups {
            if group.len() >= 3 {
                // Check for extract method + rename pattern
                let has_addition = group.iter().any(|c| c.change_type == ChangeType::Add);
                let has_modification = group.iter().any(|c| c.change_type == ChangeType::Modify);
                let has_rename = group.iter().any(|c| c.change_type == ChangeType::Rename);

                if has_addition && has_modification && has_rename {
                    patterns.push(RefactoringPattern {
                        pattern_type: RefactoringType::ExtractMethod,
                        confidence: 0.6,
                        description: "Complex refactoring: Extract method with rename".to_string(),
                        affected_elements: group.iter()
                            .filter_map(|c| c.target.as_ref().or(c.source.as_ref()).map(|e| e.name.clone()))
                            .collect(),
                        analysis: self.create_complex_analysis(group),
                        evidence: self.gather_complex_evidence(group),
                        related_changes: group.iter().map(|c| format!("{:?}", c.change_type)).collect(),
                        complexity: RefactoringComplexity {
                            complexity_level: RefactoringComplexityLevel::Complex,
                            elements_involved: group.len(),
                            files_affected: self.count_affected_files(group),
                            estimated_effort: RefactoringEffort::High,
                        },
                    });
                }
            }
        }

        patterns
    }

    /// Count affected files in a group of changes
    fn count_affected_files(&self, changes: &[&Change]) -> usize {
        let mut files = HashSet::new();

        for change in changes {
            if let Some(source) = &change.source {
                files.insert(&source.file_path);
            }
            if let Some(target) = &change.target {
                files.insert(&target.file_path);
            }
        }

        files.len()
    }

    // Detailed analysis methods with AST and signature information

    /// Detect extract method patterns with detailed analysis
    fn detect_extract_method_detailed(
        &self,
        change_groups: &[Vec<&Change>],
        source_asts: &HashMap<String, ASTNode>,
        target_asts: &HashMap<String, ASTNode>,
        source_signatures: &HashMap<String, EnhancedFunctionSignature>,
        target_signatures: &HashMap<String, EnhancedFunctionSignature>,
    ) -> Result<Vec<RefactoringPattern>> {
        let mut patterns = Vec::new();

        for group in change_groups {
            let modifications: Vec<_> = group.iter().filter(|c| c.change_type == ChangeType::Modify).collect();
            let additions: Vec<_> = group.iter().filter(|c| c.change_type == ChangeType::Add).collect();

            if modifications.len() == 1 && additions.len() == 1 {
                let modified = modifications[0];
                let added = additions[0];

                if let (Some(mod_source), Some(add_target)) = (&modified.source, &added.target) {
                    // Get AST and signature information
                    let source_ast = source_asts.get(&mod_source.name);
                    let target_ast = target_asts.get(&add_target.name);
                    let source_sig = source_signatures.get(&mod_source.name);
                    let target_sig = target_signatures.get(&add_target.name);

                    let confidence = self.calculate_detailed_extract_method_confidence(
                        modified, added, source_ast, target_ast, source_sig, target_sig
                    )?;

                    if confidence > self.config.min_confidence_threshold {
                        patterns.push(RefactoringPattern {
                            pattern_type: RefactoringType::ExtractMethod,
                            confidence,
                            description: format!(
                                "Extracted method '{}' from '{}' (detailed analysis)",
                                add_target.name, mod_source.name
                            ),
                            affected_elements: vec![add_target.name.clone(), mod_source.name.clone()],
                            analysis: self.create_detailed_extract_method_analysis(
                                modified, added, source_ast, target_ast, source_sig, target_sig
                            )?,
                            evidence: self.gather_detailed_extract_method_evidence(
                                modified, added, source_ast, target_ast, source_sig, target_sig
                            )?,
                            related_changes: group.iter().map(|c| format!("{:?}", c.change_type)).collect(),
                            complexity: self.assess_detailed_extract_method_complexity(
                                group, source_sig, target_sig
                            ),
                        });
                    }
                }
            }
        }

        Ok(patterns)
    }

    /// Calculate detailed confidence for extract method pattern
    fn calculate_detailed_extract_method_confidence(
        &self,
        modified: &Change,
        added: &Change,
        source_ast: Option<&ASTNode>,
        target_ast: Option<&ASTNode>,
        source_sig: Option<&EnhancedFunctionSignature>,
        target_sig: Option<&EnhancedFunctionSignature>,
    ) -> Result<f64> {
        let mut confidence = 0.0;

        // Base pattern confidence
        confidence += 0.3;

        // Signature analysis
        if let (Some(src_sig), Some(tgt_sig)) = (source_sig, target_sig) {
            // Check complexity reduction in source
            if let (Some(src_metrics), Some(tgt_metrics)) = (&src_sig.complexity_metrics, &tgt_sig.complexity_metrics) {
                if src_metrics.cyclomatic_complexity > tgt_metrics.cyclomatic_complexity {
                    confidence += 0.2;
                }

                // Check if extracted method has reasonable complexity
                if tgt_metrics.cyclomatic_complexity >= 2 &&
                   tgt_metrics.cyclomatic_complexity <= 10 {
                    confidence += 0.1;
                }
            }

            // Check parameter patterns (extracted methods often have parameters from original)
            if !tgt_sig.parameters.is_empty() {
                confidence += 0.1;
            }
        }

        // AST analysis
        if let (Some(src_ast), Some(tgt_ast)) = (source_ast, target_ast) {
            if let Some(scorer) = &self.similarity_scorer {
                let similarity = scorer.calculate_comprehensive_similarity(src_ast, tgt_ast)?;

                // Some similarity expected (shared patterns) but not too high
                if similarity.overall_similarity > 0.3 && similarity.overall_similarity < 0.8 {
                    confidence += 0.2;
                }
            }
        }

        // Name pattern analysis
        if let Some(target) = &added.target {
            if self.is_extract_method_name_pattern(&target.name) {
                confidence += 0.1;
            }
        }

        Ok(confidence.min(1.0))
    }

    /// Check if name follows extract method patterns
    fn is_extract_method_name_pattern(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        name_lower.contains("validate") ||
        name_lower.contains("calculate") ||
        name_lower.contains("process") ||
        name_lower.contains("handle") ||
        name_lower.contains("check") ||
        name_lower.contains("parse") ||
        name_lower.contains("format") ||
        name_lower.contains("helper") ||
        name_lower.starts_with("do") ||
        name_lower.starts_with("get") ||
        name_lower.starts_with("set") ||
        name_lower.starts_with("is") ||
        name_lower.starts_with("has")
    }

    /// Detect inline method patterns with detailed analysis
    fn detect_inline_method_detailed(
        &self,
        change_groups: &[Vec<&Change>],
        source_asts: &HashMap<String, ASTNode>,
        target_asts: &HashMap<String, ASTNode>,
        source_signatures: &HashMap<String, EnhancedFunctionSignature>,
        target_signatures: &HashMap<String, EnhancedFunctionSignature>,
    ) -> Result<Vec<RefactoringPattern>> {
        let mut patterns = Vec::new();

        for group in change_groups {
            let deletions: Vec<_> = group.iter().filter(|c| c.change_type == ChangeType::Delete).collect();
            let modifications: Vec<_> = group.iter().filter(|c| c.change_type == ChangeType::Modify).collect();

            if deletions.len() == 1 && modifications.len() == 1 {
                let deleted = deletions[0];
                let modified = modifications[0];

                if let (Some(del_source), Some(mod_target)) = (&deleted.source, &modified.target) {
                    let deleted_sig = source_signatures.get(&del_source.name);
                    let modified_sig = target_signatures.get(&mod_target.name);

                    let confidence = self.calculate_detailed_inline_method_confidence(
                        deleted, modified, deleted_sig, modified_sig
                    );

                    if confidence > self.config.min_confidence_threshold {
                        patterns.push(RefactoringPattern {
                            pattern_type: RefactoringType::InlineMethod,
                            confidence,
                            description: format!(
                                "Inlined method '{}' into '{}' (detailed analysis)",
                                del_source.name, mod_target.name
                            ),
                            affected_elements: vec![del_source.name.clone(), mod_target.name.clone()],
                            analysis: self.create_detailed_inline_method_analysis(
                                deleted, modified, deleted_sig, modified_sig
                            ),
                            evidence: self.gather_detailed_inline_method_evidence(
                                deleted, modified, deleted_sig, modified_sig
                            ),
                            related_changes: group.iter().map(|c| format!("{:?}", c.change_type)).collect(),
                            complexity: self.assess_detailed_inline_method_complexity(
                                group, deleted_sig, modified_sig
                            ),
                        });
                    }
                }
            }
        }

        Ok(patterns)
    }

    /// Calculate detailed confidence for inline method pattern
    fn calculate_detailed_inline_method_confidence(
        &self,
        deleted: &Change,
        modified: &Change,
        deleted_sig: Option<&EnhancedFunctionSignature>,
        modified_sig: Option<&EnhancedFunctionSignature>,
    ) -> f64 {
        let mut confidence = 0.0;

        // Base pattern confidence
        confidence += 0.3;

        // Signature analysis
        if let (Some(del_sig), Some(mod_sig)) = (deleted_sig, modified_sig) {
            // Check if deleted method was simple (good candidate for inlining)
            if let (Some(del_metrics), Some(mod_metrics)) = (&del_sig.complexity_metrics, &mod_sig.complexity_metrics) {
                if del_metrics.cyclomatic_complexity <= 5 &&
                   del_metrics.lines_of_code <= 20 {
                    confidence += 0.2;
                }

                // Check if modified method became more complex
                if mod_metrics.cyclomatic_complexity > del_metrics.cyclomatic_complexity {
                    confidence += 0.1;
                }
            }

            // Check if deleted method had few parameters (easier to inline)
            if del_sig.parameters.len() <= 3 {
                confidence += 0.1;
            }
        }

        // File proximity check
        if let (Some(del_source), Some(mod_target)) = (&deleted.source, &modified.target) {
            if del_source.file_path == mod_target.file_path {
                confidence += 0.2;
            }
        }

        // Size analysis
        if let Some(del_source) = &deleted.source {
            let method_size = del_source.end_line - del_source.start_line;
            if method_size <= 10 {
                confidence += 0.1;
            }
        }

        confidence.min(1.0)
    }

    /// Detect rename patterns with detailed analysis
    fn detect_rename_patterns_detailed(
        &self,
        change_groups: &[Vec<&Change>],
        source_signatures: &HashMap<String, EnhancedFunctionSignature>,
        target_signatures: &HashMap<String, EnhancedFunctionSignature>,
    ) -> Result<Vec<RefactoringPattern>> {
        let mut patterns = Vec::new();

        for group in change_groups {
            for change in group {
                if let (Some(source), Some(target)) = (&change.source, &change.target) {
                    if source.name != target.name {
                        let source_sig = source_signatures.get(&source.name);
                        let target_sig = target_signatures.get(&target.name);

                        let confidence = self.calculate_detailed_rename_confidence(
                            change, source_sig, target_sig
                        );

                        if confidence > self.config.min_confidence_threshold {
                            patterns.push(RefactoringPattern {
                                pattern_type: RefactoringType::RenameMethod,
                                confidence,
                                description: format!(
                                    "Renamed '{}' to '{}' (detailed analysis)",
                                    source.name, target.name
                                ),
                                affected_elements: vec![source.name.clone(), target.name.clone()],
                                analysis: self.create_detailed_rename_analysis(
                                    change, source_sig, target_sig
                                ),
                                evidence: self.gather_detailed_rename_evidence(
                                    change, source_sig, target_sig
                                ),
                                related_changes: vec![format!("{:?}", change.change_type)],
                                complexity: self.assess_detailed_rename_complexity(
                                    group, source_sig, target_sig
                                ),
                            });
                        }
                    }
                }
            }
        }

        Ok(patterns)
    }

    /// Calculate detailed confidence for rename pattern
    fn calculate_detailed_rename_confidence(
        &self,
        change: &Change,
        source_sig: Option<&EnhancedFunctionSignature>,
        target_sig: Option<&EnhancedFunctionSignature>,
    ) -> f64 {
        let mut confidence = 0.0;

        // Base confidence from change type
        confidence += match change.change_type {
            ChangeType::Rename => 0.9,
            ChangeType::Modify => 0.5,
            _ => 0.3,
        };

        // Signature similarity analysis
        if let (Some(src_sig), Some(tgt_sig)) = (source_sig, target_sig) {
            // Check parameter similarity
            if src_sig.parameters.len() == tgt_sig.parameters.len() {
                confidence += 0.1;

                // Check parameter types
                let param_types_match = src_sig.parameters.iter().zip(tgt_sig.parameters.iter())
                    .all(|(p1, p2)| p1.param_type == p2.param_type);

                if param_types_match {
                    confidence += 0.1;
                }
            }

            // Check return type similarity
            if src_sig.return_type == tgt_sig.return_type {
                confidence += 0.1;
            }

            // Check visibility similarity
            if src_sig.visibility == tgt_sig.visibility {
                confidence += 0.05;
            }

            // Check complexity similarity
            if let (Some(src_metrics), Some(tgt_metrics)) = (&src_sig.complexity_metrics, &tgt_sig.complexity_metrics) {
                let complexity_diff = (src_metrics.cyclomatic_complexity as f64
                    - tgt_metrics.cyclomatic_complexity as f64).abs();

                if complexity_diff <= 2.0 {
                    confidence += 0.1;
                }
            }
        }

        // Name similarity analysis
        if let (Some(source), Some(target)) = (&change.source, &change.target) {
            let name_similarity = self.calculate_name_similarity(&source.name, &target.name);
            confidence += name_similarity * 0.2;
        }

        confidence.min(1.0)
    }

    /// Detect move patterns with detailed analysis
    fn detect_move_patterns_detailed(
        &self,
        change_groups: &[Vec<&Change>],
        source_asts: &HashMap<String, ASTNode>,
        target_asts: &HashMap<String, ASTNode>,
        source_signatures: &HashMap<String, EnhancedFunctionSignature>,
        target_signatures: &HashMap<String, EnhancedFunctionSignature>,
    ) -> Result<Vec<RefactoringPattern>> {
        let mut patterns = Vec::new();

        for group in change_groups {
            for change in group {
                if matches!(change.change_type, ChangeType::Move | ChangeType::CrossFileMove) {
                    if let (Some(source), Some(target)) = (&change.source, &change.target) {
                        let source_sig = source_signatures.get(&source.name);
                        let target_sig = target_signatures.get(&target.name);

                        let confidence = self.calculate_detailed_move_confidence(
                            change, source_sig, target_sig
                        );

                        if confidence > self.config.min_confidence_threshold {
                            let pattern_type = if source.file_path != target.file_path {
                                RefactoringType::MoveClass
                            } else {
                                RefactoringType::MoveMethod
                            };

                            patterns.push(RefactoringPattern {
                                pattern_type,
                                confidence,
                                description: format!(
                                    "Moved '{}' from {}:{} to {}:{} (detailed analysis)",
                                    source.name, source.file_path, source.start_line,
                                    target.file_path, target.start_line
                                ),
                                affected_elements: vec![source.name.clone()],
                                analysis: self.create_detailed_move_analysis(
                                    change, source_sig, target_sig
                                ),
                                evidence: self.gather_detailed_move_evidence(
                                    change, source_sig, target_sig
                                ),
                                related_changes: vec![format!("{:?}", change.change_type)],
                                complexity: self.assess_detailed_move_complexity(
                                    group, source_sig, target_sig
                                ),
                            });
                        }
                    }
                }
            }
        }

        Ok(patterns)
    }

    /// Calculate detailed confidence for move pattern
    fn calculate_detailed_move_confidence(
        &self,
        change: &Change,
        source_sig: Option<&EnhancedFunctionSignature>,
        target_sig: Option<&EnhancedFunctionSignature>,
    ) -> f64 {
        let mut confidence = 0.0;

        // Base confidence from change type
        confidence += match change.change_type {
            ChangeType::CrossFileMove => 0.9,
            ChangeType::Move => 0.8,
            _ => 0.5,
        };

        // Signature preservation analysis
        if let (Some(src_sig), Some(tgt_sig)) = (source_sig, target_sig) {
            // Check if signature is preserved (high confidence for pure moves)
            if src_sig.name == tgt_sig.name &&
               src_sig.parameters.len() == tgt_sig.parameters.len() &&
               src_sig.return_type == tgt_sig.return_type {
                confidence += 0.1;
            }

            // Check complexity preservation
            if let (Some(src_metrics), Some(tgt_metrics)) = (&src_sig.complexity_metrics, &tgt_sig.complexity_metrics) {
                let complexity_diff = (src_metrics.cyclomatic_complexity as f64
                    - tgt_metrics.cyclomatic_complexity as f64).abs();

                if complexity_diff <= 1.0 {
                    confidence += 0.05;
                }
            }
        }

        confidence.min(1.0)
    }

    // Analysis creation methods

    /// Create extract method analysis
    fn create_extract_method_analysis(&self, modified: &Change, added: &Change) -> RefactoringAnalysis {
        let mut characteristics = Vec::new();

        characteristics.push(RefactoringCharacteristic {
            characteristic_type: RefactoringCharacteristicType::CodeExtraction,
            value: "Method extracted from existing function".to_string(),
            confidence: 0.8,
        });

        if let Some(target) = &added.target {
            characteristics.push(RefactoringCharacteristic {
                characteristic_type: RefactoringCharacteristicType::NameChange,
                value: format!("New method name: {}", target.name),
                confidence: 1.0,
            });
        }

        RefactoringAnalysis {
            characteristics,
            before_after: None,
            impact: RefactoringImpact {
                impact_level: RefactoringImpactLevel::Low,
                affected_files: vec![
                    modified.source.as_ref().map(|s| s.file_path.clone()).unwrap_or_default()
                ],
                affected_functions: vec![
                    modified.source.as_ref().map(|s| s.name.clone()).unwrap_or_default(),
                    added.target.as_ref().map(|t| t.name.clone()).unwrap_or_default()
                ],
                is_breaking_change: false,
                api_compatibility: ApiCompatibilityImpact::BackwardCompatible,
            },
            quality_metrics: RefactoringQualityMetrics {
                quality_improvement: 0.7,
                maintainability_impact: 0.8,
                readability_impact: 0.6,
                testability_impact: 0.7,
                performance_impact: 0.0,
            },
        }
    }

    /// Create inline method analysis
    fn create_inline_method_analysis(&self, deleted: &Change, modified: &Change) -> RefactoringAnalysis {
        let mut characteristics = Vec::new();

        characteristics.push(RefactoringCharacteristic {
            characteristic_type: RefactoringCharacteristicType::CodeInlining,
            value: "Method inlined into calling function".to_string(),
            confidence: 0.8,
        });

        RefactoringAnalysis {
            characteristics,
            before_after: None,
            impact: RefactoringImpact {
                impact_level: RefactoringImpactLevel::Low,
                affected_files: vec![
                    deleted.source.as_ref().map(|s| s.file_path.clone()).unwrap_or_default()
                ],
                affected_functions: vec![
                    deleted.source.as_ref().map(|s| s.name.clone()).unwrap_or_default(),
                    modified.target.as_ref().map(|t| t.name.clone()).unwrap_or_default()
                ],
                is_breaking_change: false,
                api_compatibility: ApiCompatibilityImpact::BackwardCompatible,
            },
            quality_metrics: RefactoringQualityMetrics {
                quality_improvement: 0.5,
                maintainability_impact: 0.4,
                readability_impact: 0.3,
                testability_impact: 0.2,
                performance_impact: 0.1,
            },
        }
    }

    /// Create rename analysis
    fn create_rename_analysis(&self, change: &Change) -> RefactoringAnalysis {
        let mut characteristics = Vec::new();

        if let (Some(source), Some(target)) = (&change.source, &change.target) {
            characteristics.push(RefactoringCharacteristic {
                characteristic_type: RefactoringCharacteristicType::NameChange,
                value: format!("Renamed from '{}' to '{}'", source.name, target.name),
                confidence: 0.9,
            });

            let name_similarity = self.calculate_name_similarity(&source.name, &target.name);
            characteristics.push(RefactoringCharacteristic {
                characteristic_type: RefactoringCharacteristicType::NameChange,
                value: format!("Name similarity: {:.3}", name_similarity),
                confidence: 0.8,
            });
        }

        RefactoringAnalysis {
            characteristics,
            before_after: None,
            impact: RefactoringImpact {
                impact_level: RefactoringImpactLevel::Medium,
                affected_files: vec![
                    change.source.as_ref().map(|s| s.file_path.clone()).unwrap_or_default()
                ],
                affected_functions: vec![
                    change.source.as_ref().map(|s| s.name.clone()).unwrap_or_default(),
                    change.target.as_ref().map(|t| t.name.clone()).unwrap_or_default()
                ],
                is_breaking_change: true, // Renames are potentially breaking
                api_compatibility: ApiCompatibilityImpact::PotentiallyBreaking,
            },
            quality_metrics: RefactoringQualityMetrics {
                quality_improvement: 0.6,
                maintainability_impact: 0.7,
                readability_impact: 0.8,
                testability_impact: 0.5,
                performance_impact: 0.0,
            },
        }
    }

    /// Create move analysis
    fn create_move_analysis(&self, change: &Change) -> RefactoringAnalysis {
        let mut characteristics = Vec::new();

        if let (Some(source), Some(target)) = (&change.source, &change.target) {
            characteristics.push(RefactoringCharacteristic {
                characteristic_type: RefactoringCharacteristicType::LocationChange,
                value: format!("Moved from {} to {}", source.file_path, target.file_path),
                confidence: 1.0,
            });

            if source.file_path != target.file_path {
                characteristics.push(RefactoringCharacteristic {
                    characteristic_type: RefactoringCharacteristicType::StructureChange,
                    value: "Cross-file move".to_string(),
                    confidence: 1.0,
                });
            }
        }

        RefactoringAnalysis {
            characteristics,
            before_after: None,
            impact: RefactoringImpact {
                impact_level: RefactoringImpactLevel::Medium,
                affected_files: vec![
                    change.source.as_ref().map(|s| s.file_path.clone()).unwrap_or_default(),
                    change.target.as_ref().map(|t| t.file_path.clone()).unwrap_or_default()
                ],
                affected_functions: vec![
                    change.source.as_ref().map(|s| s.name.clone()).unwrap_or_default()
                ],
                is_breaking_change: false,
                api_compatibility: ApiCompatibilityImpact::BackwardCompatible,
            },
            quality_metrics: RefactoringQualityMetrics {
                quality_improvement: 0.5,
                maintainability_impact: 0.6,
                readability_impact: 0.4,
                testability_impact: 0.3,
                performance_impact: 0.0,
            },
        }
    }

    // Evidence gathering methods

    /// Gather extract method evidence
    fn gather_extract_method_evidence(&self, modified: &Change, added: &Change) -> Vec<RefactoringEvidence> {
        let mut evidence = Vec::new();

        evidence.push(RefactoringEvidence {
            evidence_type: RefactoringEvidenceType::StructurePattern,
            description: "One modification + one addition pattern".to_string(),
            strength: 0.8,
            data: HashMap::new(),
        });

        if let (Some(mod_source), Some(add_target)) = (&modified.source, &added.target) {
            if mod_source.file_path == add_target.file_path {
                evidence.push(RefactoringEvidence {
                    evidence_type: RefactoringEvidenceType::LocationEvidence,
                    description: "Both functions in same file".to_string(),
                    strength: 0.7,
                    data: [("file".to_string(), mod_source.file_path.clone())].into_iter().collect(),
                });
            }

            if self.is_extract_method_name_pattern(&add_target.name) {
                evidence.push(RefactoringEvidence {
                    evidence_type: RefactoringEvidenceType::NamePattern,
                    description: "Extracted method follows naming patterns".to_string(),
                    strength: 0.6,
                    data: [("name".to_string(), add_target.name.clone())].into_iter().collect(),
                });
            }
        }

        evidence
    }

    /// Gather inline method evidence
    fn gather_inline_method_evidence(&self, deleted: &Change, modified: &Change) -> Vec<RefactoringEvidence> {
        let mut evidence = Vec::new();

        evidence.push(RefactoringEvidence {
            evidence_type: RefactoringEvidenceType::StructurePattern,
            description: "One deletion + one modification pattern".to_string(),
            strength: 0.8,
            data: HashMap::new(),
        });

        if let Some(del_source) = &deleted.source {
            let method_size = del_source.end_line - del_source.start_line;
            if method_size <= 10 {
                evidence.push(RefactoringEvidence {
                    evidence_type: RefactoringEvidenceType::StructurePattern,
                    description: "Deleted method was small (good inline candidate)".to_string(),
                    strength: 0.7,
                    data: [("size".to_string(), method_size.to_string())].into_iter().collect(),
                });
            }
        }

        evidence
    }

    /// Gather rename evidence
    fn gather_rename_evidence(&self, change: &Change) -> Vec<RefactoringEvidence> {
        let mut evidence = Vec::new();

        if let (Some(source), Some(target)) = (&change.source, &change.target) {
            let name_similarity = self.calculate_name_similarity(&source.name, &target.name);

            evidence.push(RefactoringEvidence {
                evidence_type: RefactoringEvidenceType::NamePattern,
                description: format!("Name similarity: {:.3}", name_similarity),
                strength: name_similarity,
                data: [
                    ("old_name".to_string(), source.name.clone()),
                    ("new_name".to_string(), target.name.clone()),
                ].into_iter().collect(),
            });

            if change.change_type == ChangeType::Rename {
                evidence.push(RefactoringEvidence {
                    evidence_type: RefactoringEvidenceType::StructurePattern,
                    description: "Explicit rename change type".to_string(),
                    strength: 1.0,
                    data: HashMap::new(),
                });
            }
        }

        evidence
    }

    /// Gather move evidence
    fn gather_move_evidence(&self, change: &Change) -> Vec<RefactoringEvidence> {
        let mut evidence = Vec::new();

        if let (Some(source), Some(target)) = (&change.source, &change.target) {
            evidence.push(RefactoringEvidence {
                evidence_type: RefactoringEvidenceType::LocationEvidence,
                description: format!("Location changed from {} to {}", source.file_path, target.file_path),
                strength: 1.0,
                data: [
                    ("old_location".to_string(), format!("{}:{}", source.file_path, source.start_line)),
                    ("new_location".to_string(), format!("{}:{}", target.file_path, target.start_line)),
                ].into_iter().collect(),
            });

            if source.file_path != target.file_path {
                evidence.push(RefactoringEvidence {
                    evidence_type: RefactoringEvidenceType::StructurePattern,
                    description: "Cross-file move detected".to_string(),
                    strength: 0.9,
                    data: HashMap::new(),
                });
            }
        }

        evidence
    }

    // Complexity assessment methods

    /// Assess extract method complexity
    fn assess_extract_method_complexity(&self, changes: &[&Change]) -> RefactoringComplexity {
        RefactoringComplexity {
            complexity_level: RefactoringComplexityLevel::Simple,
            elements_involved: changes.len(),
            files_affected: self.count_affected_files(changes),
            estimated_effort: RefactoringEffort::Low,
        }
    }

    /// Assess inline method complexity
    fn assess_inline_method_complexity(&self, changes: &[&Change]) -> RefactoringComplexity {
        RefactoringComplexity {
            complexity_level: RefactoringComplexityLevel::Simple,
            elements_involved: changes.len(),
            files_affected: self.count_affected_files(changes),
            estimated_effort: RefactoringEffort::Low,
        }
    }

    /// Assess rename complexity
    fn assess_rename_complexity(&self, changes: &[&Change]) -> RefactoringComplexity {
        let files_affected = self.count_affected_files(changes);

        RefactoringComplexity {
            complexity_level: if files_affected > 1 {
                RefactoringComplexityLevel::Moderate
            } else {
                RefactoringComplexityLevel::Simple
            },
            elements_involved: changes.len(),
            files_affected,
            estimated_effort: if files_affected > 1 {
                RefactoringEffort::Medium
            } else {
                RefactoringEffort::Low
            },
        }
    }

    /// Assess move complexity
    fn assess_move_complexity(&self, changes: &[&Change]) -> RefactoringComplexity {
        let files_affected = self.count_affected_files(changes);

        RefactoringComplexity {
            complexity_level: if files_affected > 2 {
                RefactoringComplexityLevel::Complex
            } else {
                RefactoringComplexityLevel::Moderate
            },
            elements_involved: changes.len(),
            files_affected,
            estimated_effort: if files_affected > 2 {
                RefactoringEffort::High
            } else {
                RefactoringEffort::Medium
            },
        }
    }

    // Placeholder methods for complex analysis patterns

    /// Create extract class analysis
    fn create_extract_class_analysis(&self, changes: &[&Change]) -> RefactoringAnalysis {
        RefactoringAnalysis {
            characteristics: vec![
                RefactoringCharacteristic {
                    characteristic_type: RefactoringCharacteristicType::StructureChange,
                    value: format!("Extracted {} methods to new class", changes.len()),
                    confidence: 0.7,
                }
            ],
            before_after: None,
            impact: RefactoringImpact {
                impact_level: RefactoringImpactLevel::High,
                affected_files: changes.iter()
                    .filter_map(|c| c.target.as_ref().map(|t| t.file_path.clone()))
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect(),
                affected_functions: changes.iter()
                    .filter_map(|c| c.target.as_ref().map(|t| t.name.clone()))
                    .collect(),
                is_breaking_change: false,
                api_compatibility: ApiCompatibilityImpact::BackwardCompatible,
            },
            quality_metrics: RefactoringQualityMetrics {
                quality_improvement: 0.8,
                maintainability_impact: 0.9,
                readability_impact: 0.7,
                testability_impact: 0.8,
                performance_impact: 0.0,
            },
        }
    }

    /// Gather extract class evidence
    fn gather_extract_class_evidence(&self, changes: &[&Change]) -> Vec<RefactoringEvidence> {
        vec![
            RefactoringEvidence {
                evidence_type: RefactoringEvidenceType::StructurePattern,
                description: format!("Multiple methods ({}) moved to same new file", changes.len()),
                strength: 0.8,
                data: HashMap::new(),
            }
        ]
    }

    /// Assess extract class complexity
    fn assess_extract_class_complexity(&self, changes: &[&Change]) -> RefactoringComplexity {
        RefactoringComplexity {
            complexity_level: RefactoringComplexityLevel::Complex,
            elements_involved: changes.len(),
            files_affected: self.count_affected_files(changes),
            estimated_effort: RefactoringEffort::High,
        }
    }

    // Additional placeholder methods for completeness
    fn create_inline_class_analysis(&self, changes: &[&Change]) -> RefactoringAnalysis {
        self.create_extract_class_analysis(changes) // Similar structure
    }

    fn gather_inline_class_evidence(&self, changes: &[&Change]) -> Vec<RefactoringEvidence> {
        self.gather_extract_class_evidence(changes) // Similar evidence
    }

    fn assess_inline_class_complexity(&self, changes: &[&Change]) -> RefactoringComplexity {
        self.assess_extract_class_complexity(changes) // Similar complexity
    }

    fn create_change_signature_analysis(&self, change: &Change) -> RefactoringAnalysis {
        self.create_rename_analysis(change) // Similar to rename
    }

    fn gather_change_signature_evidence(&self, change: &Change) -> Vec<RefactoringEvidence> {
        self.gather_rename_evidence(change) // Similar evidence
    }

    fn assess_change_signature_complexity(&self, changes: &[&Change]) -> RefactoringComplexity {
        self.assess_rename_complexity(changes) // Similar complexity
    }

    fn create_complex_analysis(&self, changes: &[&Change]) -> RefactoringAnalysis {
        RefactoringAnalysis {
            characteristics: vec![
                RefactoringCharacteristic {
                    characteristic_type: RefactoringCharacteristicType::StructureChange,
                    value: format!("Complex refactoring involving {} changes", changes.len()),
                    confidence: 0.6,
                }
            ],
            before_after: None,
            impact: RefactoringImpact {
                impact_level: RefactoringImpactLevel::High,
                affected_files: changes.iter()
                    .filter_map(|c| c.source.as_ref().or(c.target.as_ref()).map(|e| e.file_path.clone()))
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect(),
                affected_functions: changes.iter()
                    .filter_map(|c| c.source.as_ref().or(c.target.as_ref()).map(|e| e.name.clone()))
                    .collect(),
                is_breaking_change: true,
                api_compatibility: ApiCompatibilityImpact::PotentiallyBreaking,
            },
            quality_metrics: RefactoringQualityMetrics {
                quality_improvement: 0.7,
                maintainability_impact: 0.8,
                readability_impact: 0.6,
                testability_impact: 0.7,
                performance_impact: 0.1,
            },
        }
    }

    fn gather_complex_evidence(&self, changes: &[&Change]) -> Vec<RefactoringEvidence> {
        vec![
            RefactoringEvidence {
                evidence_type: RefactoringEvidenceType::StructurePattern,
                description: format!("Complex pattern with {} related changes", changes.len()),
                strength: 0.6,
                data: HashMap::new(),
            }
        ]
    }

    // Configuration and utility methods

    /// Get current configuration
    pub fn get_config(&self) -> &RefactoringDetectionConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: RefactoringDetectionConfig) {
        self.config = config;
    }

    /// Enable or disable change classifier
    pub fn set_change_classifier(&mut self, enabled: bool) {
        if enabled && self.change_classifier.is_none() {
            self.change_classifier = Some(ChangeClassifier::new(self.language));
        } else if !enabled {
            self.change_classifier = None;
        }
    }

    /// Enable or disable similarity scorer
    pub fn set_similarity_scorer(&mut self, enabled: bool) {
        if enabled && self.similarity_scorer.is_none() {
            self.similarity_scorer = Some(SimilarityScorer::new(self.language, SimilarityScoringConfig::default()));
        } else if !enabled {
            self.similarity_scorer = None;
        }
    }

    /// Get supported refactoring types
    pub fn get_supported_refactoring_types(&self) -> Vec<RefactoringType> {
        let mut types = Vec::new();

        if self.config.enable_extract_method {
            types.push(RefactoringType::ExtractMethod);
        }
        if self.config.enable_inline_method {
            types.push(RefactoringType::InlineMethod);
        }
        if self.config.enable_rename_detection {
            types.push(RefactoringType::RenameMethod);
        }
        if self.config.enable_move_detection {
            types.push(RefactoringType::MoveMethod);
            types.push(RefactoringType::MoveClass);
        }
        if self.config.enable_extract_class {
            types.push(RefactoringType::ExtractClass);
        }
        if self.config.enable_inline_class {
            types.push(RefactoringType::InlineClass);
        }
        if self.config.enable_change_signature {
            types.push(RefactoringType::ChangeSignature);
        }

        types
    }

    // Placeholder detailed analysis methods (would be implemented with full AST analysis)
    fn create_detailed_extract_method_analysis(
        &self, _modified: &Change, _added: &Change, _source_ast: Option<&ASTNode>,
        _target_ast: Option<&ASTNode>, _source_sig: Option<&EnhancedFunctionSignature>,
        _target_sig: Option<&EnhancedFunctionSignature>
    ) -> Result<RefactoringAnalysis> {
        Ok(self.create_extract_method_analysis(_modified, _added))
    }

    fn gather_detailed_extract_method_evidence(
        &self, modified: &Change, added: &Change, _source_ast: Option<&ASTNode>,
        _target_ast: Option<&ASTNode>, _source_sig: Option<&EnhancedFunctionSignature>,
        _target_sig: Option<&EnhancedFunctionSignature>
    ) -> Result<Vec<RefactoringEvidence>> {
        Ok(self.gather_extract_method_evidence(modified, added))
    }

    fn assess_detailed_extract_method_complexity(
        &self, changes: &[&Change], _source_sig: Option<&EnhancedFunctionSignature>,
        _target_sig: Option<&EnhancedFunctionSignature>
    ) -> RefactoringComplexity {
        self.assess_extract_method_complexity(changes)
    }

    fn create_detailed_inline_method_analysis(
        &self, deleted: &Change, modified: &Change, _deleted_sig: Option<&EnhancedFunctionSignature>,
        _modified_sig: Option<&EnhancedFunctionSignature>
    ) -> RefactoringAnalysis {
        self.create_inline_method_analysis(deleted, modified)
    }

    fn gather_detailed_inline_method_evidence(
        &self, deleted: &Change, modified: &Change, _deleted_sig: Option<&EnhancedFunctionSignature>,
        _modified_sig: Option<&EnhancedFunctionSignature>
    ) -> Vec<RefactoringEvidence> {
        self.gather_inline_method_evidence(deleted, modified)
    }

    fn assess_detailed_inline_method_complexity(
        &self, changes: &[&Change], _deleted_sig: Option<&EnhancedFunctionSignature>,
        _modified_sig: Option<&EnhancedFunctionSignature>
    ) -> RefactoringComplexity {
        self.assess_inline_method_complexity(changes)
    }

    fn create_detailed_rename_analysis(
        &self, change: &Change, _source_sig: Option<&EnhancedFunctionSignature>,
        _target_sig: Option<&EnhancedFunctionSignature>
    ) -> RefactoringAnalysis {
        self.create_rename_analysis(change)
    }

    fn gather_detailed_rename_evidence(
        &self, change: &Change, _source_sig: Option<&EnhancedFunctionSignature>,
        _target_sig: Option<&EnhancedFunctionSignature>
    ) -> Vec<RefactoringEvidence> {
        self.gather_rename_evidence(change)
    }

    fn assess_detailed_rename_complexity(
        &self, changes: &[&Change], _source_sig: Option<&EnhancedFunctionSignature>,
        _target_sig: Option<&EnhancedFunctionSignature>
    ) -> RefactoringComplexity {
        self.assess_rename_complexity(changes)
    }

    fn create_detailed_move_analysis(
        &self, change: &Change, _source_sig: Option<&EnhancedFunctionSignature>,
        _target_sig: Option<&EnhancedFunctionSignature>
    ) -> RefactoringAnalysis {
        self.create_move_analysis(change)
    }

    fn gather_detailed_move_evidence(
        &self, change: &Change, _source_sig: Option<&EnhancedFunctionSignature>,
        _target_sig: Option<&EnhancedFunctionSignature>
    ) -> Vec<RefactoringEvidence> {
        self.gather_move_evidence(change)
    }

    fn assess_detailed_move_complexity(
        &self, changes: &[&Change], _source_sig: Option<&EnhancedFunctionSignature>,
        _target_sig: Option<&EnhancedFunctionSignature>
    ) -> RefactoringComplexity {
        self.assess_move_complexity(changes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smart_diff_parser::{ChangeDetail, CodeElement};
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

    fn create_test_change(
        change_type: ChangeType,
        source: Option<CodeElement>,
        target: Option<CodeElement>,
        similarity_score: Option<f64>,
    ) -> Change {
        Change {
            change_type,
            source,
            target,
            details: ChangeDetail {
                description: "Test change".to_string(),
                affected_lines: vec![1, 2, 3],
                similarity_score,
                refactoring_type: None,
                metadata: HashMap::new(),
            },
            confidence: 0.8,
        }
    }

    #[test]
    fn test_refactoring_detection_config_default() {
        let config = RefactoringDetectionConfig::default();

        assert_eq!(config.min_confidence_threshold, 0.7);
        assert!(config.enable_extract_method);
        assert!(config.enable_inline_method);
        assert!(config.enable_rename_detection);
        assert!(config.enable_move_detection);
        assert!(config.enable_extract_class);
        assert!(config.enable_inline_class);
        assert!(config.enable_change_signature);
        assert_eq!(config.max_related_distance, 50);
        assert!(config.enable_complex_patterns);
    }

    #[test]
    fn test_refactoring_detector_creation() {
        let detector = RefactoringDetector::new(Language::Java);

        assert_eq!(detector.language, Language::Java);
        assert!(detector.change_classifier.is_some());
        assert!(detector.similarity_scorer.is_some());
    }

    #[test]
    fn test_minimal_refactoring_detector() {
        let detector = RefactoringDetector::minimal(Language::Python);

        assert_eq!(detector.language, Language::Python);
        assert!(detector.change_classifier.is_none());
        assert!(detector.similarity_scorer.is_none());
    }

    #[test]
    fn test_extract_method_pattern_detection() {
        let detector = RefactoringDetector::new(Language::Java);

        let changes = vec![
            create_test_change(
                ChangeType::Modify,
                Some(create_test_code_element("processData", "Service.java", 10)),
                Some(create_test_code_element("processData", "Service.java", 10)),
                Some(0.8),
            ),
            create_test_change(
                ChangeType::Add,
                None,
                Some(create_test_code_element("validateInput", "Service.java", 50)),
                None,
            ),
        ];

        let patterns = detector.detect_patterns(&changes);

        assert!(!patterns.is_empty());
        let extract_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == RefactoringType::ExtractMethod)
            .collect();

        assert!(!extract_patterns.is_empty());
        let pattern = &extract_patterns[0];
        assert!(pattern.confidence > 0.5);
        assert!(pattern.description.contains("Extracted method"));
        assert_eq!(pattern.affected_elements.len(), 2);
    }

    #[test]
    fn test_inline_method_pattern_detection() {
        let detector = RefactoringDetector::new(Language::Java);

        let changes = vec![
            create_test_change(
                ChangeType::Delete,
                Some(create_test_code_element("helper", "Service.java", 50)),
                None,
                None,
            ),
            create_test_change(
                ChangeType::Modify,
                Some(create_test_code_element("processData", "Service.java", 10)),
                Some(create_test_code_element("processData", "Service.java", 10)),
                Some(0.7),
            ),
        ];

        let patterns = detector.detect_patterns(&changes);

        let inline_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == RefactoringType::InlineMethod)
            .collect();

        assert!(!inline_patterns.is_empty());
        let pattern = &inline_patterns[0];
        assert!(pattern.confidence > 0.5);
        assert!(pattern.description.contains("Inlined method"));
    }

    #[test]
    fn test_rename_pattern_detection() {
        let detector = RefactoringDetector::new(Language::Java);

        let changes = vec![
            create_test_change(
                ChangeType::Rename,
                Some(create_test_code_element("oldMethod", "Service.java", 10)),
                Some(create_test_code_element("newMethod", "Service.java", 10)),
                Some(0.9),
            ),
        ];

        let patterns = detector.detect_patterns(&changes);

        let rename_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == RefactoringType::RenameMethod)
            .collect();

        assert!(!rename_patterns.is_empty());
        let pattern = &rename_patterns[0];
        assert_eq!(pattern.confidence, 0.9);
        assert!(pattern.description.contains("Renamed"));
        assert!(pattern.description.contains("oldMethod"));
        assert!(pattern.description.contains("newMethod"));
    }

    #[test]
    fn test_move_pattern_detection() {
        let detector = RefactoringDetector::new(Language::Java);

        let changes = vec![
            create_test_change(
                ChangeType::CrossFileMove,
                Some(create_test_code_element("utility", "Utils.java", 10)),
                Some(create_test_code_element("utility", "helpers/StringUtils.java", 20)),
                Some(0.95),
            ),
        ];

        let patterns = detector.detect_patterns(&changes);

        let move_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == RefactoringType::MoveMethod)
            .collect();

        assert!(!move_patterns.is_empty());
        let pattern = &move_patterns[0];
        assert_eq!(pattern.confidence, 0.9);
        assert!(pattern.description.contains("Moved"));
        assert!(pattern.description.contains("Utils.java"));
        assert!(pattern.description.contains("StringUtils.java"));
    }

    #[test]
    fn test_extract_class_pattern_detection() {
        let detector = RefactoringDetector::new(Language::Java);

        let changes = vec![
            create_test_change(
                ChangeType::Add,
                None,
                Some(create_test_code_element("method1", "NewClass.java", 10)),
                None,
            ),
            create_test_change(
                ChangeType::Add,
                None,
                Some(create_test_code_element("method2", "NewClass.java", 20)),
                None,
            ),
            create_test_change(
                ChangeType::Modify,
                Some(create_test_code_element("oldClass", "OldClass.java", 30)),
                Some(create_test_code_element("oldClass", "OldClass.java", 30)),
                Some(0.6),
            ),
        ];

        let patterns = detector.detect_patterns(&changes);

        let extract_class_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == RefactoringType::ExtractClass)
            .collect();

        assert!(!extract_class_patterns.is_empty());
        let pattern = &extract_class_patterns[0];
        assert!(pattern.confidence >= 0.7);
        assert!(pattern.description.contains("Extracted"));
        assert!(pattern.description.contains("methods"));
        assert_eq!(pattern.affected_elements.len(), 2);
    }

    #[test]
    fn test_name_similarity_calculation() {
        let detector = RefactoringDetector::new(Language::Java);

        // Identical names
        assert_eq!(detector.calculate_name_similarity("method", "method"), 1.0);

        // Completely different names
        assert_eq!(detector.calculate_name_similarity("method", "function"), 0.0);

        // Similar names
        let similarity = detector.calculate_name_similarity("calculateSum", "calculateTotal");
        assert!(similarity > 0.5 && similarity < 1.0);

        // Case differences
        let similarity = detector.calculate_name_similarity("Method", "method");
        assert!(similarity > 0.8);
    }

    #[test]
    fn test_levenshtein_distance() {
        let detector = RefactoringDetector::new(Language::Java);

        assert_eq!(detector.levenshtein_distance("", ""), 0);
        assert_eq!(detector.levenshtein_distance("abc", ""), 3);
        assert_eq!(detector.levenshtein_distance("", "abc"), 3);
        assert_eq!(detector.levenshtein_distance("abc", "abc"), 0);
        assert_eq!(detector.levenshtein_distance("abc", "ab"), 1);
        assert_eq!(detector.levenshtein_distance("abc", "axc"), 1);
        assert_eq!(detector.levenshtein_distance("abc", "def"), 3);
    }

    #[test]
    fn test_extract_method_name_patterns() {
        let detector = RefactoringDetector::new(Language::Java);

        assert!(detector.is_extract_method_name_pattern("validateInput"));
        assert!(detector.is_extract_method_name_pattern("calculateTotal"));
        assert!(detector.is_extract_method_name_pattern("processData"));
        assert!(detector.is_extract_method_name_pattern("handleRequest"));
        assert!(detector.is_extract_method_name_pattern("checkPermissions"));
        assert!(detector.is_extract_method_name_pattern("parseJson"));
        assert!(detector.is_extract_method_name_pattern("formatOutput"));
        assert!(detector.is_extract_method_name_pattern("helperMethod"));
        assert!(detector.is_extract_method_name_pattern("doSomething"));
        assert!(detector.is_extract_method_name_pattern("getValue"));
        assert!(detector.is_extract_method_name_pattern("setValue"));
        assert!(detector.is_extract_method_name_pattern("isValid"));
        assert!(detector.is_extract_method_name_pattern("hasPermission"));

        assert!(!detector.is_extract_method_name_pattern("main"));
        assert!(!detector.is_extract_method_name_pattern("run"));
        assert!(!detector.is_extract_method_name_pattern("x"));
    }

    #[test]
    fn test_changes_related_detection() {
        let detector = RefactoringDetector::new(Language::Java);

        let change1 = create_test_change(
            ChangeType::Modify,
            Some(create_test_code_element("method1", "Service.java", 10)),
            Some(create_test_code_element("method1", "Service.java", 10)),
            Some(0.8),
        );

        let change2 = create_test_change(
            ChangeType::Add,
            None,
            Some(create_test_code_element("method2", "Service.java", 15)),
            None,
        );

        // Same file, close proximity
        assert!(detector.are_changes_related(&change1, &change2));

        let change3 = create_test_change(
            ChangeType::Add,
            None,
            Some(create_test_code_element("method3", "Other.java", 100)),
            None,
        );

        // Different file, far apart
        assert!(!detector.are_changes_related(&change1, &change3));

        let change4 = create_test_change(
            ChangeType::Delete,
            Some(create_test_code_element("method1Similar", "Service.java", 50)),
            None,
            None,
        );

        // Similar names
        assert!(detector.are_changes_related(&change1, &change4));
    }

    #[test]
    fn test_confidence_threshold_filtering() {
        let mut config = RefactoringDetectionConfig::default();
        config.min_confidence_threshold = 0.8;

        let detector = RefactoringDetector::with_config(Language::Java, config);

        let changes = vec![
            create_test_change(
                ChangeType::Modify,
                Some(create_test_code_element("method", "Service.java", 10)),
                Some(create_test_code_element("method", "Service.java", 10)),
                Some(0.5), // Low similarity, should result in low confidence
            ),
            create_test_change(
                ChangeType::Add,
                None,
                Some(create_test_code_element("helper", "Service.java", 50)),
                None,
            ),
        ];

        let patterns = detector.detect_patterns(&changes);

        // Should filter out low-confidence patterns
        for pattern in &patterns {
            assert!(pattern.confidence >= 0.8);
        }
    }

    #[test]
    fn test_pattern_sorting_by_confidence() {
        let detector = RefactoringDetector::new(Language::Java);

        let changes = vec![
            create_test_change(
                ChangeType::Rename,
                Some(create_test_code_element("oldName", "Service.java", 10)),
                Some(create_test_code_element("newName", "Service.java", 10)),
                Some(0.9),
            ),
            create_test_change(
                ChangeType::Move,
                Some(create_test_code_element("method", "Old.java", 20)),
                Some(create_test_code_element("method", "New.java", 30)),
                Some(0.8),
            ),
        ];

        let patterns = detector.detect_patterns(&changes);

        // Should be sorted by confidence (highest first)
        for i in 1..patterns.len() {
            assert!(patterns[i - 1].confidence >= patterns[i].confidence);
        }
    }

    #[test]
    fn test_supported_refactoring_types() {
        let detector = RefactoringDetector::new(Language::Java);
        let types = detector.get_supported_refactoring_types();

        assert!(types.contains(&RefactoringType::ExtractMethod));
        assert!(types.contains(&RefactoringType::InlineMethod));
        assert!(types.contains(&RefactoringType::RenameMethod));
        assert!(types.contains(&RefactoringType::MoveMethod));
        assert!(types.contains(&RefactoringType::MoveClass));
        assert!(types.contains(&RefactoringType::ExtractClass));
        assert!(types.contains(&RefactoringType::InlineClass));
        assert!(types.contains(&RefactoringType::ChangeSignature));
    }

    #[test]
    fn test_configuration_updates() {
        let mut detector = RefactoringDetector::new(Language::Java);

        let original_threshold = detector.get_config().min_confidence_threshold;
        assert_eq!(original_threshold, 0.7);

        let new_config = RefactoringDetectionConfig {
            min_confidence_threshold: 0.8,
            enable_extract_method: false,
            enable_inline_method: false,
            enable_rename_detection: true,
            enable_move_detection: true,
            enable_extract_class: false,
            enable_inline_class: false,
            enable_change_signature: false,
            max_related_distance: 25,
            enable_complex_patterns: false,
        };

        detector.set_config(new_config);

        assert_eq!(detector.get_config().min_confidence_threshold, 0.8);
        assert!(!detector.get_config().enable_extract_method);
        assert!(!detector.get_config().enable_inline_method);
        assert!(detector.get_config().enable_rename_detection);
        assert!(detector.get_config().enable_move_detection);
        assert_eq!(detector.get_config().max_related_distance, 25);
        assert!(!detector.get_config().enable_complex_patterns);
    }

    #[test]
    fn test_change_classifier_toggle() {
        let mut detector = RefactoringDetector::new(Language::Java);

        // Initially enabled
        assert!(detector.change_classifier.is_some());

        // Disable change classifier
        detector.set_change_classifier(false);
        assert!(detector.change_classifier.is_none());

        // Re-enable change classifier
        detector.set_change_classifier(true);
        assert!(detector.change_classifier.is_some());
    }

    #[test]
    fn test_similarity_scorer_toggle() {
        let mut detector = RefactoringDetector::new(Language::Java);

        // Initially enabled
        assert!(detector.similarity_scorer.is_some());

        // Disable similarity scorer
        detector.set_similarity_scorer(false);
        assert!(detector.similarity_scorer.is_none());

        // Re-enable similarity scorer
        detector.set_similarity_scorer(true);
        assert!(detector.similarity_scorer.is_some());
    }

    #[test]
    fn test_empty_changes_handling() {
        let detector = RefactoringDetector::new(Language::Java);

        let patterns = detector.detect_patterns(&[]);
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_complex_pattern_detection() {
        let detector = RefactoringDetector::new(Language::Java);

        let changes = vec![
            create_test_change(
                ChangeType::Add,
                None,
                Some(create_test_code_element("newMethod", "Service.java", 10)),
                None,
            ),
            create_test_change(
                ChangeType::Modify,
                Some(create_test_code_element("oldMethod", "Service.java", 20)),
                Some(create_test_code_element("oldMethod", "Service.java", 20)),
                Some(0.7),
            ),
            create_test_change(
                ChangeType::Rename,
                Some(create_test_code_element("helper", "Service.java", 30)),
                Some(create_test_code_element("utility", "Service.java", 30)),
                Some(0.8),
            ),
        ];

        let patterns = detector.detect_patterns(&changes);

        // Should detect complex patterns when multiple changes are related
        let complex_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.description.contains("Complex") || p.complexity.complexity_level == RefactoringComplexityLevel::Complex)
            .collect();

        assert!(!complex_patterns.is_empty());
    }
}
