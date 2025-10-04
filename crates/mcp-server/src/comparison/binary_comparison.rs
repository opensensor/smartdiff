//! Binary comparison context and management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use smart_diff_engine::{BinaryFunctionMatch, BinaryMatchType};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a binary comparison
pub type BinaryComparisonId = Uuid;

/// Parameters for binary comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryComparisonParams {
    /// Binary A server ID (e.g., "port_9009")
    pub binary_a_id: String,

    /// Binary B server ID (e.g., "port_9010")
    pub binary_b_id: String,

    /// Binary A filename
    pub binary_a_filename: String,

    /// Binary B filename
    pub binary_b_filename: String,

    /// Use decompiled code for comparison
    pub use_decompiled_code: bool,

    /// Minimum similarity threshold
    pub similarity_threshold: f64,
}

/// Binary comparison context
#[derive(Debug, Clone)]
pub struct BinaryComparisonContext {
    /// Unique comparison ID
    pub id: BinaryComparisonId,

    /// Comparison parameters
    pub params: BinaryComparisonParams,

    /// Function matches
    pub matches: Vec<BinaryFunctionMatch>,

    /// Functions only in binary A (deleted)
    pub deleted_functions: Vec<String>,

    /// Functions only in binary B (added)
    pub added_functions: Vec<String>,

    /// Timestamp when comparison was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl BinaryComparisonContext {
    /// Create a new binary comparison context
    pub fn new(params: BinaryComparisonParams, matches: Vec<BinaryFunctionMatch>) -> Self {
        Self {
            id: Uuid::new_v4(),
            params,
            matches,
            deleted_functions: Vec::new(),
            added_functions: Vec::new(),
            created_at: chrono::Utc::now(),
        }
    }

    /// Get summary statistics
    pub fn get_summary(&self) -> BinaryComparisonSummary {
        let total_matches = self.matches.len();
        let exact_matches = self
            .matches
            .iter()
            .filter(|m| m.match_type == BinaryMatchType::ExactName)
            .count();
        let fuzzy_matches = self
            .matches
            .iter()
            .filter(|m| m.match_type == BinaryMatchType::FuzzyName)
            .count();
        let code_matches = self
            .matches
            .iter()
            .filter(|m| m.match_type == BinaryMatchType::CodeSimilarity)
            .count();
        let hybrid_matches = self
            .matches
            .iter()
            .filter(|m| m.match_type == BinaryMatchType::Hybrid)
            .count();

        let avg_similarity = if total_matches > 0 {
            self.matches.iter().map(|m| m.similarity).sum::<f64>() / total_matches as f64
        } else {
            0.0
        };

        BinaryComparisonSummary {
            comparison_id: self.id,
            binary_a_filename: self.params.binary_a_filename.clone(),
            binary_b_filename: self.params.binary_b_filename.clone(),
            total_matches,
            exact_matches,
            fuzzy_matches,
            code_matches,
            hybrid_matches,
            added_functions: self.added_functions.len(),
            deleted_functions: self.deleted_functions.len(),
            average_similarity: avg_similarity,
        }
    }

    /// Get matches sorted by similarity (ascending - most changed first)
    pub fn get_sorted_matches(&self) -> Vec<BinaryFunctionMatch> {
        let mut sorted = self.matches.clone();
        sorted.sort_by(|a, b| a.similarity.partial_cmp(&b.similarity).unwrap());
        sorted
    }

    /// Get a specific match by function name
    pub fn get_match_by_name(&self, function_name: &str) -> Option<&BinaryFunctionMatch> {
        self.matches
            .iter()
            .find(|m| m.function_a.name == function_name || m.function_b.name == function_name)
    }
}

/// Summary statistics for a binary comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryComparisonSummary {
    pub comparison_id: BinaryComparisonId,
    pub binary_a_filename: String,
    pub binary_b_filename: String,
    pub total_matches: usize,
    pub exact_matches: usize,
    pub fuzzy_matches: usize,
    pub code_matches: usize,
    pub hybrid_matches: usize,
    pub added_functions: usize,
    pub deleted_functions: usize,
    pub average_similarity: f64,
}

/// Manager for binary comparisons
pub struct BinaryComparisonManager {
    comparisons: HashMap<BinaryComparisonId, BinaryComparisonContext>,
}

impl BinaryComparisonManager {
    /// Create a new binary comparison manager
    pub fn new() -> Self {
        Self {
            comparisons: HashMap::new(),
        }
    }

    /// Store a binary comparison
    pub fn store_comparison(&mut self, context: BinaryComparisonContext) -> BinaryComparisonId {
        let id = context.id;
        self.comparisons.insert(id, context);
        id
    }

    /// Get a binary comparison by ID
    pub fn get_comparison(&self, id: BinaryComparisonId) -> Result<&BinaryComparisonContext> {
        self.comparisons
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("Binary comparison not found: {}", id))
    }

    /// Get a mutable reference to a binary comparison
    pub fn get_comparison_mut(
        &mut self,
        id: BinaryComparisonId,
    ) -> Result<&mut BinaryComparisonContext> {
        self.comparisons
            .get_mut(&id)
            .ok_or_else(|| anyhow::anyhow!("Binary comparison not found: {}", id))
    }

    /// List all comparison IDs
    pub fn list_comparisons(&self) -> Vec<BinaryComparisonId> {
        self.comparisons.keys().copied().collect()
    }

    /// Remove a comparison
    pub fn remove_comparison(&mut self, id: BinaryComparisonId) -> Result<()> {
        self.comparisons
            .remove(&id)
            .ok_or_else(|| anyhow::anyhow!("Binary comparison not found: {}", id))?;
        Ok(())
    }

    /// Clear all comparisons
    pub fn clear(&mut self) {
        self.comparisons.clear();
    }
}

impl Default for BinaryComparisonManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smart_diff_engine::BinaryFunctionInfo;

    #[test]
    fn test_binary_comparison_context() {
        let params = BinaryComparisonParams {
            binary_a_id: "port_9009".to_string(),
            binary_b_id: "port_9010".to_string(),
            binary_a_filename: "test_v1.exe".to_string(),
            binary_b_filename: "test_v2.exe".to_string(),
            use_decompiled_code: true,
            similarity_threshold: 0.7,
        };

        let matches = vec![BinaryFunctionMatch {
            function_a: BinaryFunctionInfo::new("main".to_string(), "0x1000".to_string()),
            function_b: BinaryFunctionInfo::new("main".to_string(), "0x1100".to_string()),
            similarity: 1.0,
            name_similarity: 1.0,
            code_similarity: None,
            match_type: BinaryMatchType::ExactName,
            confidence: 1.0,
        }];

        let context = BinaryComparisonContext::new(params, matches);
        let summary = context.get_summary();

        assert_eq!(summary.total_matches, 1);
        assert_eq!(summary.exact_matches, 1);
        assert_eq!(summary.average_similarity, 1.0);
    }

    #[test]
    fn test_binary_comparison_manager() {
        let mut manager = BinaryComparisonManager::new();

        let params = BinaryComparisonParams {
            binary_a_id: "port_9009".to_string(),
            binary_b_id: "port_9010".to_string(),
            binary_a_filename: "test_v1.exe".to_string(),
            binary_b_filename: "test_v2.exe".to_string(),
            use_decompiled_code: true,
            similarity_threshold: 0.7,
        };

        let context = BinaryComparisonContext::new(params, vec![]);
        let id = manager.store_comparison(context);

        assert!(manager.get_comparison(id).is_ok());
        assert_eq!(manager.list_comparisons().len(), 1);

        manager.remove_comparison(id).unwrap();
        assert_eq!(manager.list_comparisons().len(), 0);
    }
}

