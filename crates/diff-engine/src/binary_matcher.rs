//! Binary function matching using decompiled code similarity
//!
//! This module implements function matching for binary comparison by:
//! 1. Exact name matching (fast O(n) lookup)
//! 2. Fuzzy name matching (Levenshtein distance)
//! 3. Decompiled code similarity (reusing tree edit distance on C code)
//! 4. Hybrid scoring (combining name and code similarity)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// Configuration for binary function matching
#[derive(Debug, Clone)]
pub struct BinaryMatcherConfig {
    /// Weight for name similarity (default: 0.3)
    pub name_weight: f64,

    /// Weight for code similarity (default: 0.7)
    pub code_weight: f64,

    /// Minimum similarity threshold for matches (default: 0.7)
    pub match_threshold: f64,

    /// Maximum Levenshtein distance for fuzzy name matching (default: 3)
    pub max_name_edit_distance: usize,

    /// Enable decompiled code comparison (default: true)
    pub enable_code_comparison: bool,

    /// Enable parallel processing (default: true)
    pub enable_parallel: bool,
}

impl Default for BinaryMatcherConfig {
    fn default() -> Self {
        Self {
            name_weight: 0.3,
            code_weight: 0.7,
            match_threshold: 0.7,
            max_name_edit_distance: 3,
            enable_code_comparison: true,
            enable_parallel: true,
        }
    }
}

/// Information about a binary function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryFunctionInfo {
    /// Function name
    pub name: String,

    /// Function address (hex string)
    pub address: String,

    /// Decompiled code (if available)
    pub decompiled_code: Option<String>,

    /// Raw/mangled name (if different from name)
    pub raw_name: Option<String>,
}

impl BinaryFunctionInfo {
    pub fn new(name: String, address: String) -> Self {
        Self {
            name,
            address,
            decompiled_code: None,
            raw_name: None,
        }
    }

    pub fn with_code(mut self, code: String) -> Self {
        self.decompiled_code = Some(code);
        self
    }
}

/// Type of match between two functions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryMatchType {
    /// Exact name match
    ExactName,
    /// Fuzzy name match
    FuzzyName,
    /// Code similarity match
    CodeSimilarity,
    /// Hybrid match (name + code)
    Hybrid,
}

/// A match between two binary functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryFunctionMatch {
    /// Function from binary A
    pub function_a: BinaryFunctionInfo,

    /// Function from binary B
    pub function_b: BinaryFunctionInfo,

    /// Overall similarity score (0.0 to 1.0)
    pub similarity: f64,

    /// Name similarity score
    pub name_similarity: f64,

    /// Code similarity score (if available)
    pub code_similarity: Option<f64>,

    /// Type of match
    pub match_type: BinaryMatchType,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

/// Binary function matcher
pub struct BinaryFunctionMatcher {
    config: BinaryMatcherConfig,
}

impl BinaryFunctionMatcher {
    /// Create a new matcher with default configuration
    pub fn new() -> Self {
        Self {
            config: BinaryMatcherConfig::default(),
        }
    }

    /// Create a new matcher with custom configuration
    pub fn with_config(config: BinaryMatcherConfig) -> Self {
        Self { config }
    }

    /// Match functions between two binaries
    ///
    /// This performs multi-phase matching:
    /// 1. Exact name matching (fast)
    /// 2. Fuzzy name matching (for renamed functions)
    /// 3. Code similarity matching (for heavily modified functions)
    pub fn match_functions(
        &self,
        functions_a: &[BinaryFunctionInfo],
        functions_b: &[BinaryFunctionInfo],
    ) -> Result<Vec<BinaryFunctionMatch>> {
        info!(
            "Matching {} functions from binary A with {} functions from binary B",
            functions_a.len(),
            functions_b.len()
        );

        let mut matches = Vec::new();
        let mut matched_a = HashMap::new();
        let mut matched_b = HashMap::new();

        // Phase 1: Exact name matching
        debug!("Phase 1: Exact name matching");
        let exact_matches = self.exact_name_matching(functions_a, functions_b)?;
        for m in exact_matches {
            matched_a.insert(m.function_a.name.clone(), true);
            matched_b.insert(m.function_b.name.clone(), true);
            matches.push(m);
        }
        info!("Found {} exact name matches", matches.len());

        // Phase 2: Fuzzy name matching for unmatched functions
        debug!("Phase 2: Fuzzy name matching");
        let unmatched_a: Vec<_> = functions_a
            .iter()
            .filter(|f| !matched_a.contains_key(&f.name))
            .cloned()
            .collect();
        let unmatched_b: Vec<_> = functions_b
            .iter()
            .filter(|f| !matched_b.contains_key(&f.name))
            .cloned()
            .collect();

        let fuzzy_matches = self.fuzzy_name_matching(&unmatched_a, &unmatched_b)?;
        for m in &fuzzy_matches {
            matched_a.insert(m.function_a.name.clone(), true);
            matched_b.insert(m.function_b.name.clone(), true);
        }
        matches.extend(fuzzy_matches);
        info!("Found {} fuzzy name matches", matches.len());

        // Phase 3: Code similarity matching (if enabled and code available)
        if self.config.enable_code_comparison {
            debug!("Phase 3: Code similarity matching");
            let unmatched_a: Vec<_> = functions_a
                .iter()
                .filter(|f| !matched_a.contains_key(&f.name))
                .cloned()
                .collect();
            let unmatched_b: Vec<_> = functions_b
                .iter()
                .filter(|f| !matched_b.contains_key(&f.name))
                .cloned()
                .collect();

            let code_matches = self.code_similarity_matching(&unmatched_a, &unmatched_b)?;
            matches.extend(code_matches);
        }

        info!("Total matches found: {}", matches.len());
        Ok(matches)
    }

    /// Phase 1: Exact name matching
    fn exact_name_matching(
        &self,
        functions_a: &[BinaryFunctionInfo],
        functions_b: &[BinaryFunctionInfo],
    ) -> Result<Vec<BinaryFunctionMatch>> {
        // Build HashMap for O(n) lookup
        let map_b: HashMap<String, &BinaryFunctionInfo> =
            functions_b.iter().map(|f| (f.name.clone(), f)).collect();

        let mut matches = Vec::new();

        for func_a in functions_a {
            if let Some(func_b) = map_b.get(&func_a.name) {
                matches.push(BinaryFunctionMatch {
                    function_a: func_a.clone(),
                    function_b: (*func_b).clone(),
                    similarity: 1.0,
                    name_similarity: 1.0,
                    code_similarity: None,
                    match_type: BinaryMatchType::ExactName,
                    confidence: 1.0,
                });
            }
        }

        Ok(matches)
    }

    /// Phase 2: Fuzzy name matching using Levenshtein distance
    fn fuzzy_name_matching(
        &self,
        functions_a: &[BinaryFunctionInfo],
        functions_b: &[BinaryFunctionInfo],
    ) -> Result<Vec<BinaryFunctionMatch>> {
        let mut matches = Vec::new();
        let mut matched_b = HashMap::new();

        for func_a in functions_a {
            let mut best_match: Option<(usize, f64)> = None;

            for (idx, func_b) in functions_b.iter().enumerate() {
                if matched_b.contains_key(&idx) {
                    continue;
                }

                let distance = levenshtein_distance(&func_a.name, &func_b.name);
                if distance <= self.config.max_name_edit_distance {
                    let max_len = func_a.name.len().max(func_b.name.len()) as f64;
                    let similarity = 1.0 - (distance as f64 / max_len);

                    if similarity >= self.config.match_threshold {
                        if let Some((_, best_sim)) = best_match {
                            if similarity > best_sim {
                                best_match = Some((idx, similarity));
                            }
                        } else {
                            best_match = Some((idx, similarity));
                        }
                    }
                }
            }

            if let Some((idx, name_sim)) = best_match {
                matched_b.insert(idx, true);
                matches.push(BinaryFunctionMatch {
                    function_a: func_a.clone(),
                    function_b: functions_b[idx].clone(),
                    similarity: name_sim,
                    name_similarity: name_sim,
                    code_similarity: None,
                    match_type: BinaryMatchType::FuzzyName,
                    confidence: name_sim * 0.8, // Lower confidence for fuzzy matches
                });
            }
        }

        Ok(matches)
    }

    /// Phase 3: Code similarity matching (placeholder - will be implemented with tree-sitter)
    fn code_similarity_matching(
        &self,
        _functions_a: &[BinaryFunctionInfo],
        _functions_b: &[BinaryFunctionInfo],
    ) -> Result<Vec<BinaryFunctionMatch>> {
        // TODO: Implement code similarity using tree-sitter C parser
        // This will parse decompiled C code and use tree edit distance
        // For now, return empty matches
        Ok(Vec::new())
    }
}

impl Default for BinaryFunctionMatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
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

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
        assert_eq!(levenshtein_distance("abc", "abd"), 1);
        assert_eq!(levenshtein_distance("abc", ""), 3);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn test_exact_name_matching() {
        let matcher = BinaryFunctionMatcher::new();

        let functions_a = vec![
            BinaryFunctionInfo::new("main".to_string(), "0x1000".to_string()),
            BinaryFunctionInfo::new("process_data".to_string(), "0x2000".to_string()),
        ];

        let functions_b = vec![
            BinaryFunctionInfo::new("main".to_string(), "0x1100".to_string()),
            BinaryFunctionInfo::new("helper".to_string(), "0x2100".to_string()),
        ];

        let matches = matcher.exact_name_matching(&functions_a, &functions_b).unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].function_a.name, "main");
        assert_eq!(matches[0].function_b.name, "main");
        assert_eq!(matches[0].similarity, 1.0);
    }

    #[test]
    fn test_fuzzy_name_matching() {
        let matcher = BinaryFunctionMatcher::new();

        let functions_a = vec![
            BinaryFunctionInfo::new("process_data".to_string(), "0x1000".to_string()),
        ];

        let functions_b = vec![
            BinaryFunctionInfo::new("process_dat".to_string(), "0x1100".to_string()),
        ];

        let matches = matcher.fuzzy_name_matching(&functions_a, &functions_b).unwrap();
        // Should match with edit distance of 1
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].match_type, BinaryMatchType::FuzzyName);
        assert!(matches[0].name_similarity > 0.9);
    }
}

