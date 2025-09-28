//! Change classification and analysis

use smart_diff_parser::{Change, ChangeType, CodeElement};
use serde::{Deserialize, Serialize};

/// Change classifier that categorizes detected changes
pub struct ChangeClassifier;

impl ChangeClassifier {
    /// Classify a change based on its characteristics
    pub fn classify_change(&self, source: Option<&CodeElement>, target: Option<&CodeElement>) -> ChangeType {
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
    
    /// Detect if a change represents a function split
    pub fn detect_split(&self, source: &CodeElement, targets: &[CodeElement]) -> bool {
        targets.len() > 1 && targets.iter().all(|t| {
            t.name.contains(&source.name) || source.name.contains(&t.name)
        })
    }
    
    /// Detect if changes represent a function merge
    pub fn detect_merge(&self, sources: &[CodeElement], target: &CodeElement) -> bool {
        sources.len() > 1 && sources.iter().all(|s| {
            s.name.contains(&target.name) || target.name.contains(&s.name)
        })
    }
}
