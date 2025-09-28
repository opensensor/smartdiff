//! Matching and change detection data structures

use crate::function::Function;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of matching functions between two versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub similarity: f64,
    pub mapping: HashMap<String, String>, // source_id -> target_id
    pub changes: Vec<Change>,
    pub unmatched_source: Vec<String>,
    pub unmatched_target: Vec<String>,
}

/// Represents a change between two code versions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Change {
    pub change_type: ChangeType,
    pub source: Option<CodeElement>,
    pub target: Option<CodeElement>,
    pub details: ChangeDetail,
    pub confidence: f64,
}

/// Types of changes that can be detected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    /// Element was added
    Add,
    /// Element was deleted
    Delete,
    /// Element was modified
    Modify,
    /// Element was moved (within same file)
    Move,
    /// Element was renamed
    Rename,
    /// Element was moved to different file
    CrossFileMove,
    /// Function was split into multiple functions
    Split,
    /// Multiple functions were merged into one
    Merge,
}

/// Represents a code element (function, class, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeElement {
    pub id: String,
    pub element_type: ElementType,
    pub name: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub signature: Option<String>,
    pub hash: String,
}

/// Types of code elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementType {
    Function,
    Method,
    Class,
    Interface,
    Module,
    Variable,
    Constant,
}

/// Detailed information about a change
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangeDetail {
    pub description: String,
    pub affected_lines: Vec<usize>,
    pub similarity_score: Option<f64>,
    pub refactoring_type: Option<RefactoringType>,
    pub metadata: HashMap<String, String>,
}

/// Types of refactoring patterns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefactoringType {
    ExtractMethod,
    InlineMethod,
    RenameMethod,
    MoveMethod,
    ExtractClass,
    InlineClass,
    RenameClass,
    MoveClass,
    ExtractVariable,
    InlineVariable,
    RenameVariable,
    ChangeSignature,
}

impl Default for MatchResult {
    fn default() -> Self {
        Self::new()
    }
}

impl MatchResult {
    pub fn new() -> Self {
        Self {
            similarity: 0.0,
            mapping: HashMap::new(),
            changes: Vec::new(),
            unmatched_source: Vec::new(),
            unmatched_target: Vec::new(),
        }
    }

    /// Calculate overall similarity score
    pub fn calculate_similarity(&mut self) {
        if self.changes.is_empty() {
            self.similarity = 1.0;
            return;
        }

        let total_elements =
            self.mapping.len() + self.unmatched_source.len() + self.unmatched_target.len();
        if total_elements == 0 {
            self.similarity = 1.0;
            return;
        }

        let matched_elements = self.mapping.len();
        let base_similarity = matched_elements as f64 / total_elements as f64;

        // Adjust based on change types
        let change_penalty = self
            .changes
            .iter()
            .map(|change| match change.change_type {
                ChangeType::Add | ChangeType::Delete => 0.3,
                ChangeType::Move | ChangeType::Rename => 0.1,
                ChangeType::Modify => 0.2,
                ChangeType::CrossFileMove => 0.4,
                ChangeType::Split | ChangeType::Merge => 0.5,
            })
            .sum::<f64>()
            / self.changes.len() as f64;

        self.similarity = (base_similarity * (1.0 - change_penalty)).clamp(0.0, 1.0);
    }

    /// Get changes by type
    pub fn changes_by_type(&self, change_type: ChangeType) -> Vec<&Change> {
        self.changes
            .iter()
            .filter(|change| change.change_type == change_type)
            .collect()
    }

    /// Get high-confidence changes
    pub fn high_confidence_changes(&self, threshold: f64) -> Vec<&Change> {
        self.changes
            .iter()
            .filter(|change| change.confidence >= threshold)
            .collect()
    }
}

impl Change {
    pub fn new(change_type: ChangeType, description: String) -> Self {
        Self {
            change_type,
            source: None,
            target: None,
            details: ChangeDetail {
                description,
                affected_lines: Vec::new(),
                similarity_score: None,
                refactoring_type: None,
                metadata: HashMap::new(),
            },
            confidence: 1.0,
        }
    }

    pub fn with_elements(
        mut self,
        source: Option<CodeElement>,
        target: Option<CodeElement>,
    ) -> Self {
        self.source = source;
        self.target = target;
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    pub fn with_refactoring_type(mut self, refactoring_type: RefactoringType) -> Self {
        self.details.refactoring_type = Some(refactoring_type);
        self
    }

    /// Check if this change represents a significant modification
    pub fn is_significant(&self) -> bool {
        match self.change_type {
            ChangeType::Add | ChangeType::Delete => true,
            ChangeType::CrossFileMove | ChangeType::Split | ChangeType::Merge => true,
            ChangeType::Modify => self
                .details
                .similarity_score
                .is_none_or(|score| score < 0.8),
            ChangeType::Move | ChangeType::Rename => false,
        }
    }
}

impl CodeElement {
    pub fn from_function(function: &Function) -> Self {
        Self {
            id: format!("func_{}", function.hash),
            element_type: ElementType::Function,
            name: function.signature.name.clone(),
            file_path: function.location.file_path.clone(),
            start_line: function.location.start_line,
            end_line: function.location.end_line,
            signature: Some(format!(
                "{}({})",
                function.signature.name,
                function
                    .signature
                    .parameters
                    .iter()
                    .map(|p| format!("{}: {}", p.name, p.param_type.name))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            hash: function.hash.clone(),
        }
    }

    /// Calculate similarity with another code element
    pub fn similarity(&self, other: &CodeElement) -> f64 {
        if self.element_type != other.element_type {
            return 0.0;
        }

        let mut score = 0.0;
        let mut weight = 0.0;

        // Name similarity (40%)
        let name_weight = 0.4;
        if self.name == other.name {
            score += name_weight;
        } else {
            let distance = edit_distance::edit_distance(&self.name, &other.name);
            let max_len = self.name.len().max(other.name.len());
            if max_len > 0 {
                score += name_weight * (1.0 - (distance as f64 / max_len as f64));
            }
        }
        weight += name_weight;

        // Signature similarity (40%)
        let sig_weight = 0.4;
        match (&self.signature, &other.signature) {
            (Some(s1), Some(s2)) => {
                if s1 == s2 {
                    score += sig_weight;
                } else {
                    let distance = edit_distance::edit_distance(s1, s2);
                    let max_len = s1.len().max(s2.len());
                    if max_len > 0 {
                        score += sig_weight * (1.0 - (distance as f64 / max_len as f64));
                    }
                }
            }
            (None, None) => score += sig_weight,
            _ => {} // One has signature, other doesn't
        }
        weight += sig_weight;

        // Hash similarity (20%)
        let hash_weight = 0.2;
        if self.hash == other.hash {
            score += hash_weight;
        }
        weight += hash_weight;

        score / weight
    }
}
