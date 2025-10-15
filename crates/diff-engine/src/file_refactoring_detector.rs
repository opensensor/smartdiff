//! File-level refactoring detection for tracking file renames, splits, and merges
//!
//! This module provides advanced detection of file-level refactoring operations:
//! - File renames: Detecting when a file is renamed but content remains similar
//! - File splits: Detecting when a file is split into multiple files
//! - File merges: Detecting when multiple files are merged into one
//! - File moves: Detecting when files are moved to different directories

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Configuration for file refactoring detection
#[derive(Debug, Clone)]
pub struct FileRefactoringDetectorConfig {
    /// Minimum content similarity for file rename detection (0.0 to 1.0)
    pub min_rename_similarity: f64,
    /// Minimum content similarity for file split detection (0.0 to 1.0)
    pub min_split_similarity: f64,
    /// Minimum content similarity for file merge detection (0.0 to 1.0)
    pub min_merge_similarity: f64,
    /// Enable path similarity analysis
    pub use_path_similarity: bool,
    /// Enable content fingerprinting
    pub use_content_fingerprinting: bool,
    /// Enable symbol migration tracking
    pub use_symbol_migration: bool,
    /// Maximum number of files to consider for split/merge detection
    pub max_split_merge_candidates: usize,
}

impl Default for FileRefactoringDetectorConfig {
    fn default() -> Self {
        Self {
            min_rename_similarity: 0.7,
            min_split_similarity: 0.5,
            min_merge_similarity: 0.5,
            use_path_similarity: true,
            use_content_fingerprinting: true,
            use_symbol_migration: true,
            max_split_merge_candidates: 10,
        }
    }
}

/// File refactoring detector
pub struct FileRefactoringDetector {
    config: FileRefactoringDetectorConfig,
}

/// Result of file refactoring detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRefactoringResult {
    /// Detected file renames
    pub file_renames: Vec<FileRename>,
    /// Detected file splits
    pub file_splits: Vec<FileSplit>,
    /// Detected file merges
    pub file_merges: Vec<FileMerge>,
    /// Detected file moves
    pub file_moves: Vec<FileMove>,
    /// Overall statistics
    pub statistics: FileRefactoringStats,
}

/// File rename detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRename {
    /// Original file path
    pub source_path: String,
    /// New file path
    pub target_path: String,
    /// Content similarity score
    pub content_similarity: f64,
    /// Path similarity score
    pub path_similarity: f64,
    /// Symbol migration score (percentage of symbols that moved)
    pub symbol_migration_score: f64,
    /// Overall confidence in the rename detection
    pub confidence: f64,
}

/// File split detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSplit {
    /// Original file path
    pub source_path: String,
    /// Target files with their similarity scores
    pub target_files: Vec<(String, f64)>,
    /// Combined similarity score
    pub combined_similarity: f64,
    /// Confidence in the split detection
    pub confidence: f64,
}

/// File merge detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMerge {
    /// Source files with their similarity scores
    pub source_files: Vec<(String, f64)>,
    /// Merged file path
    pub target_path: String,
    /// Combined similarity score
    pub combined_similarity: f64,
    /// Confidence in the merge detection
    pub confidence: f64,
}

/// File move detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMove {
    /// Original file path
    pub source_path: String,
    /// New file path
    pub target_path: String,
    /// Whether the file was also renamed
    pub was_renamed: bool,
    /// Confidence in the move detection
    pub confidence: f64,
}

/// Statistics about file refactoring detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRefactoringStats {
    /// Total source files analyzed
    pub total_source_files: usize,
    /// Total target files analyzed
    pub total_target_files: usize,
    /// Number of file renames detected
    pub rename_count: usize,
    /// Number of file splits detected
    pub split_count: usize,
    /// Number of file merges detected
    pub merge_count: usize,
    /// Number of file moves detected
    pub move_count: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Content fingerprint for a file
#[derive(Debug, Clone)]
pub struct ContentFingerprint {
    /// Hash of the file content
    pub content_hash: String,
    /// Hash of normalized content (whitespace removed)
    pub normalized_hash: String,
    /// Set of unique identifiers (function names, class names, etc.)
    pub identifier_set: HashSet<String>,
    /// Total number of lines
    pub line_count: usize,
    /// Total number of non-empty lines
    pub non_empty_line_count: usize,
}

impl FileRefactoringDetector {
    pub fn new(config: FileRefactoringDetectorConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(FileRefactoringDetectorConfig::default())
    }

    /// Detect file-level refactorings between two sets of files
    pub fn detect_file_refactorings(
        &self,
        source_files: &HashMap<String, String>, // file_path -> content
        target_files: &HashMap<String, String>, // file_path -> content
    ) -> Result<FileRefactoringResult> {
        let start_time = std::time::Instant::now();

        let mut result = FileRefactoringResult {
            file_renames: Vec::new(),
            file_splits: Vec::new(),
            file_merges: Vec::new(),
            file_moves: Vec::new(),
            statistics: FileRefactoringStats {
                total_source_files: source_files.len(),
                total_target_files: target_files.len(),
                rename_count: 0,
                split_count: 0,
                merge_count: 0,
                move_count: 0,
                execution_time_ms: 0,
            },
        };

        // Step 1: Create content fingerprints for all files
        let source_fingerprints = self.create_fingerprints(source_files)?;
        let target_fingerprints = self.create_fingerprints(target_files)?;

        // Step 2: Detect file renames and moves
        let (renames, moves) = self.detect_renames_and_moves(
            source_files,
            target_files,
            &source_fingerprints,
            &target_fingerprints,
        )?;
        result.file_renames = renames;
        result.file_moves = moves;

        // Step 3: Identify unmatched files for split/merge detection
        let matched_sources: HashSet<String> = result
            .file_renames
            .iter()
            .map(|r| r.source_path.clone())
            .chain(result.file_moves.iter().map(|m| m.source_path.clone()))
            .collect();

        let matched_targets: HashSet<String> = result
            .file_renames
            .iter()
            .map(|r| r.target_path.clone())
            .chain(result.file_moves.iter().map(|m| m.target_path.clone()))
            .collect();

        let unmatched_sources: HashMap<String, String> = source_files
            .iter()
            .filter(|(path, _)| !matched_sources.contains(*path))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let unmatched_targets: HashMap<String, String> = target_files
            .iter()
            .filter(|(path, _)| !matched_targets.contains(*path))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Step 4: Detect file splits
        let splits = self.detect_file_splits(&unmatched_sources, &unmatched_targets)?;
        result.file_splits = splits;

        // Step 5: Detect file merges
        let merges = self.detect_file_merges(&unmatched_sources, &unmatched_targets)?;
        result.file_merges = merges;

        // Update statistics
        result.statistics.rename_count = result.file_renames.len();
        result.statistics.split_count = result.file_splits.len();
        result.statistics.merge_count = result.file_merges.len();
        result.statistics.move_count = result.file_moves.len();
        result.statistics.execution_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(result)
    }

    /// Create content fingerprints for all files
    fn create_fingerprints(
        &self,
        files: &HashMap<String, String>,
    ) -> Result<HashMap<String, ContentFingerprint>> {
        let mut fingerprints = HashMap::new();

        for (path, content) in files {
            let fingerprint = self.create_fingerprint(content)?;
            fingerprints.insert(path.clone(), fingerprint);
        }

        Ok(fingerprints)
    }

    /// Create a content fingerprint for a single file
    fn create_fingerprint(&self, content: &str) -> Result<ContentFingerprint> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Calculate content hash
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let content_hash = format!("{:x}", hasher.finish());

        // Calculate normalized hash (without whitespace)
        let normalized = content
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let mut hasher = DefaultHasher::new();
        normalized.hash(&mut hasher);
        let normalized_hash = format!("{:x}", hasher.finish());

        // Extract identifiers (simple regex-based extraction)
        let identifier_set = self.extract_identifiers(content);

        // Count lines
        let line_count = content.lines().count();
        let non_empty_line_count = content.lines().filter(|line| !line.trim().is_empty()).count();

        Ok(ContentFingerprint {
            content_hash,
            normalized_hash,
            identifier_set,
            line_count,
            non_empty_line_count,
        })
    }

    /// Extract identifiers from content
    fn extract_identifiers(&self, content: &str) -> HashSet<String> {
        let mut identifiers = HashSet::new();

        // Simple pattern matching for common identifiers
        // This is a basic implementation - could be enhanced with proper parsing
        let patterns = [
            r"class\s+(\w+)",
            r"interface\s+(\w+)",
            r"function\s+(\w+)",
            r"def\s+(\w+)",
            r"fn\s+(\w+)",
            r"const\s+(\w+)",
            r"let\s+(\w+)",
            r"var\s+(\w+)",
        ];

        for line in content.lines() {
            for pattern in &patterns {
                if let Ok(re) = regex::Regex::new(pattern) {
                    for cap in re.captures_iter(line) {
                        if let Some(ident) = cap.get(1) {
                            identifiers.insert(ident.as_str().to_string());
                        }
                    }
                }
            }
        }

        identifiers
    }

    /// Detect file renames and moves
    fn detect_renames_and_moves(
        &self,
        source_files: &HashMap<String, String>,
        target_files: &HashMap<String, String>,
        source_fingerprints: &HashMap<String, ContentFingerprint>,
        target_fingerprints: &HashMap<String, ContentFingerprint>,
    ) -> Result<(Vec<FileRename>, Vec<FileMove>)> {
        let mut renames = Vec::new();
        let mut moves = Vec::new();
        let mut matched_targets = HashSet::new();

        for (source_path, _source_content) in source_files {
            // Skip if file exists in target with same path
            if target_files.contains_key(source_path) {
                continue;
            }

            let source_fp = source_fingerprints.get(source_path).unwrap();
            let mut best_match: Option<(String, f64, f64, f64)> = None;

            // Search for best matching target file
            for (target_path, _target_content) in target_files {
                if matched_targets.contains(target_path) {
                    continue;
                }

                let target_fp = target_fingerprints.get(target_path).unwrap();

                // Calculate content similarity
                let content_sim = self.calculate_content_similarity(source_fp, target_fp);

                // Calculate path similarity
                let path_sim = if self.config.use_path_similarity {
                    self.calculate_path_similarity(source_path, target_path)
                } else {
                    0.0
                };

                // Calculate symbol migration score
                let symbol_migration = if self.config.use_symbol_migration {
                    self.calculate_symbol_migration(source_fp, target_fp)
                } else {
                    0.0
                };

                // Combined score
                let combined_score = content_sim * 0.6 + path_sim * 0.2 + symbol_migration * 0.2;

                if combined_score >= self.config.min_rename_similarity {
                    if let Some((_, _, _, best_score)) = best_match {
                        if combined_score > best_score {
                            best_match = Some((
                                target_path.clone(),
                                content_sim,
                                path_sim,
                                combined_score,
                            ));
                        }
                    } else {
                        best_match = Some((
                            target_path.clone(),
                            content_sim,
                            path_sim,
                            combined_score,
                        ));
                    }
                }
            }

            // If we found a match, determine if it's a rename or move
            if let Some((target_path, content_sim, path_sim, combined_score)) = best_match {
                matched_targets.insert(target_path.clone());

                let source_fp = source_fingerprints.get(source_path).unwrap();
                let target_fp = target_fingerprints.get(&target_path).unwrap();
                let symbol_migration = self.calculate_symbol_migration(source_fp, target_fp);

                // Check if it's a move (directory change) or rename (filename change)
                let source_dir = Path::new(source_path).parent();
                let target_dir = Path::new(&target_path).parent();
                let source_name = Path::new(source_path).file_name();
                let target_name = Path::new(&target_path).file_name();

                if source_dir != target_dir && source_name == target_name {
                    // Pure move (same filename, different directory)
                    moves.push(FileMove {
                        source_path: source_path.clone(),
                        target_path: target_path.clone(),
                        was_renamed: false,
                        confidence: combined_score,
                    });
                } else if source_dir == target_dir && source_name != target_name {
                    // Pure rename (same directory, different filename)
                    renames.push(FileRename {
                        source_path: source_path.clone(),
                        target_path: target_path.clone(),
                        content_similarity: content_sim,
                        path_similarity: path_sim,
                        symbol_migration_score: symbol_migration,
                        confidence: combined_score,
                    });
                } else {
                    // Move + rename
                    moves.push(FileMove {
                        source_path: source_path.clone(),
                        target_path: target_path.clone(),
                        was_renamed: true,
                        confidence: combined_score,
                    });
                }
            }
        }

        Ok((renames, moves))
    }

    /// Calculate content similarity between two fingerprints
    fn calculate_content_similarity(
        &self,
        source: &ContentFingerprint,
        target: &ContentFingerprint,
    ) -> f64 {
        if !self.config.use_content_fingerprinting {
            return 0.0;
        }

        // Exact hash match
        if source.content_hash == target.content_hash {
            return 1.0;
        }

        // Normalized hash match (ignoring whitespace)
        if source.normalized_hash == target.normalized_hash {
            return 0.95;
        }

        // Identifier set similarity (Jaccard index)
        let intersection = source
            .identifier_set
            .intersection(&target.identifier_set)
            .count();
        let union = source.identifier_set.union(&target.identifier_set).count();

        let identifier_similarity = if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        };

        // Line count similarity
        let max_lines = source.line_count.max(target.line_count) as f64;
        let min_lines = source.line_count.min(target.line_count) as f64;
        let line_similarity = if max_lines > 0.0 {
            min_lines / max_lines
        } else {
            0.0
        };

        // Weighted combination
        identifier_similarity * 0.7 + line_similarity * 0.3
    }

    /// Calculate path similarity between two file paths
    fn calculate_path_similarity(&self, source_path: &str, target_path: &str) -> f64 {
        let source = Path::new(source_path);
        let target = Path::new(target_path);

        // Same directory bonus
        let same_dir = source.parent() == target.parent();
        let mut score = if same_dir { 0.5 } else { 0.0 };

        // Similar filename
        if let (Some(source_name), Some(target_name)) = (source.file_stem(), target.file_stem()) {
            if let (Some(s), Some(t)) = (source_name.to_str(), target_name.to_str()) {
                let name_sim = self.calculate_string_similarity(s, t);
                score += name_sim * 0.5;
            }
        }

        score
    }

    /// Calculate symbol migration score
    fn calculate_symbol_migration(
        &self,
        source: &ContentFingerprint,
        target: &ContentFingerprint,
    ) -> f64 {
        if source.identifier_set.is_empty() {
            return 0.0;
        }

        let migrated = source
            .identifier_set
            .intersection(&target.identifier_set)
            .count();

        migrated as f64 / source.identifier_set.len() as f64
    }

    /// Calculate string similarity using Levenshtein distance
    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f64 {
        let distance = levenshtein_distance(s1, s2);
        let max_len = s1.len().max(s2.len());

        if max_len == 0 {
            return 1.0;
        }

        1.0 - (distance as f64 / max_len as f64)
    }

    /// Detect file splits (one source file -> multiple target files)
    fn detect_file_splits(
        &self,
        source_files: &HashMap<String, String>,
        target_files: &HashMap<String, String>,
    ) -> Result<Vec<FileSplit>> {
        let mut splits = Vec::new();

        // Create fingerprints for unmatched files
        let source_fingerprints = self.create_fingerprints(source_files)?;
        let target_fingerprints = self.create_fingerprints(target_files)?;

        for (source_path, _source_content) in source_files {
            let source_fp = source_fingerprints.get(source_path).unwrap();
            let mut candidates = Vec::new();

            // Find target files that share identifiers with this source file
            for (target_path, _target_content) in target_files {
                let target_fp = target_fingerprints.get(target_path).unwrap();

                // Calculate identifier overlap
                let overlap = source_fp
                    .identifier_set
                    .intersection(&target_fp.identifier_set)
                    .count();

                if overlap > 0 {
                    let similarity = overlap as f64 / source_fp.identifier_set.len() as f64;

                    if similarity >= self.config.min_split_similarity {
                        candidates.push((target_path.clone(), similarity));
                    }
                }
            }

            // If we found multiple candidates, it might be a split
            if candidates.len() >= 2 {
                candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                candidates.truncate(self.config.max_split_merge_candidates);

                let combined_similarity = candidates.iter().map(|(_, sim)| sim).sum::<f64>()
                    / candidates.len() as f64;

                let confidence = self.calculate_split_confidence(&candidates, source_fp);

                splits.push(FileSplit {
                    source_path: source_path.clone(),
                    target_files: candidates,
                    combined_similarity,
                    confidence,
                });
            }
        }

        Ok(splits)
    }

    /// Detect file merges (multiple source files -> one target file)
    fn detect_file_merges(
        &self,
        source_files: &HashMap<String, String>,
        target_files: &HashMap<String, String>,
    ) -> Result<Vec<FileMerge>> {
        let mut merges = Vec::new();

        // Create fingerprints for unmatched files
        let source_fingerprints = self.create_fingerprints(source_files)?;
        let target_fingerprints = self.create_fingerprints(target_files)?;

        for (target_path, _target_content) in target_files {
            let target_fp = target_fingerprints.get(target_path).unwrap();
            let mut candidates = Vec::new();

            // Find source files that share identifiers with this target file
            for (source_path, _source_content) in source_files {
                let source_fp = source_fingerprints.get(source_path).unwrap();

                // Calculate identifier overlap
                let overlap = target_fp
                    .identifier_set
                    .intersection(&source_fp.identifier_set)
                    .count();

                if overlap > 0 {
                    let similarity = overlap as f64 / target_fp.identifier_set.len() as f64;

                    if similarity >= self.config.min_merge_similarity {
                        candidates.push((source_path.clone(), similarity));
                    }
                }
            }

            // If we found multiple candidates, it might be a merge
            if candidates.len() >= 2 {
                candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                candidates.truncate(self.config.max_split_merge_candidates);

                let combined_similarity = candidates.iter().map(|(_, sim)| sim).sum::<f64>()
                    / candidates.len() as f64;

                let confidence = self.calculate_merge_confidence(&candidates, target_fp);

                merges.push(FileMerge {
                    source_files: candidates,
                    target_path: target_path.clone(),
                    combined_similarity,
                    confidence,
                });
            }
        }

        Ok(merges)
    }

    /// Calculate confidence for split detection
    fn calculate_split_confidence(
        &self,
        candidates: &[(String, f64)],
        _source_fp: &ContentFingerprint,
    ) -> f64 {
        let mut confidence = 0.5; // Base confidence

        // Higher confidence if more candidates
        confidence += (candidates.len() as f64 / 10.0).min(0.2);

        // Higher confidence if average similarity is high
        let avg_similarity = candidates.iter().map(|(_, sim)| sim).sum::<f64>()
            / candidates.len() as f64;
        confidence += avg_similarity * 0.3;

        confidence.min(1.0)
    }

    /// Calculate confidence for merge detection
    fn calculate_merge_confidence(
        &self,
        candidates: &[(String, f64)],
        _target_fp: &ContentFingerprint,
    ) -> f64 {
        let mut confidence = 0.5; // Base confidence

        // Higher confidence if more candidates
        confidence += (candidates.len() as f64 / 10.0).min(0.2);

        // Higher confidence if average similarity is high
        let avg_similarity = candidates.iter().map(|(_, sim)| sim).sum::<f64>()
            / candidates.len() as f64;
        confidence += avg_similarity * 0.3;

        confidence.min(1.0)
    }

    /// Get configuration
    pub fn get_config(&self) -> &FileRefactoringDetectorConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: FileRefactoringDetectorConfig) {
        self.config = config;
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

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };

            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = FileRefactoringDetectorConfig::default();
        assert_eq!(config.min_rename_similarity, 0.7);
        assert_eq!(config.min_split_similarity, 0.5);
        assert_eq!(config.min_merge_similarity, 0.5);
        assert!(config.use_path_similarity);
        assert!(config.use_content_fingerprinting);
        assert!(config.use_symbol_migration);
    }

    #[test]
    fn test_detector_creation() {
        let detector = FileRefactoringDetector::with_defaults();
        assert_eq!(detector.get_config().min_rename_similarity, 0.7);
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
        assert_eq!(levenshtein_distance("abc", "abd"), 1);
        assert_eq!(levenshtein_distance("abc", "def"), 3);
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn test_string_similarity() {
        let detector = FileRefactoringDetector::with_defaults();

        assert_eq!(detector.calculate_string_similarity("abc", "abc"), 1.0);
        assert!(detector.calculate_string_similarity("abc", "abd") > 0.6);
        assert!(detector.calculate_string_similarity("abc", "xyz") < 0.5);
    }
}

