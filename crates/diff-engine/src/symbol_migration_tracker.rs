//! Symbol migration tracking for cross-file refactoring detection
//!
//! This module tracks how symbols (functions, classes, variables) migrate between files
//! during refactoring operations, providing enhanced detection of cross-file moves.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use smart_diff_semantic::{Symbol, SymbolKind, SymbolResolver, SymbolTable};
use std::collections::HashMap;

/// Configuration for symbol migration tracking
#[derive(Debug, Clone)]
pub struct SymbolMigrationTrackerConfig {
    /// Minimum percentage of symbols that must migrate for file relationship (0.0 to 1.0)
    pub min_migration_threshold: f64,
    /// Track function migrations
    pub track_functions: bool,
    /// Track class migrations
    pub track_classes: bool,
    /// Track variable migrations
    pub track_variables: bool,
    /// Enable cross-file reference analysis
    pub analyze_cross_file_references: bool,
}

impl Default for SymbolMigrationTrackerConfig {
    fn default() -> Self {
        Self {
            min_migration_threshold: 0.3,
            track_functions: true,
            track_classes: true,
            track_variables: false,
            analyze_cross_file_references: true,
        }
    }
}

/// Symbol migration tracker
pub struct SymbolMigrationTracker {
    config: SymbolMigrationTrackerConfig,
}

/// Result of symbol migration analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolMigrationResult {
    /// Migrations detected between files
    pub file_migrations: Vec<FileMigration>,
    /// Symbol-level migrations
    pub symbol_migrations: Vec<SymbolMigration>,
    /// Cross-file reference changes
    pub reference_changes: Vec<ReferenceChange>,
    /// Overall statistics
    pub statistics: MigrationStatistics,
}

/// Migration of symbols between two files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMigration {
    /// Source file path
    pub source_file: String,
    /// Target file path
    pub target_file: String,
    /// Symbols that migrated
    pub migrated_symbols: Vec<String>,
    /// Migration percentage (0.0 to 1.0)
    pub migration_percentage: f64,
    /// Confidence in the migration detection
    pub confidence: f64,
}

/// Individual symbol migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolMigration {
    /// Symbol name
    pub symbol_name: String,
    /// Symbol kind
    pub symbol_kind: String,
    /// Source file
    pub source_file: String,
    /// Target file
    pub target_file: String,
    /// Whether the symbol was renamed
    pub was_renamed: bool,
    /// New name if renamed
    pub new_name: Option<String>,
    /// Confidence score
    pub confidence: f64,
}

/// Change in cross-file references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceChange {
    /// Symbol being referenced
    pub symbol_name: String,
    /// File containing the reference
    pub referencing_file: String,
    /// Old referenced file
    pub old_referenced_file: String,
    /// New referenced file
    pub new_referenced_file: String,
    /// Change type
    pub change_type: ReferenceChangeType,
}

/// Type of reference change
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReferenceChangeType {
    /// Reference updated to follow moved symbol
    FollowedMove,
    /// Reference broken (symbol moved but reference not updated)
    Broken,
    /// New reference created
    NewReference,
}

/// Statistics about symbol migrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStatistics {
    /// Total symbols analyzed
    pub total_symbols: usize,
    /// Total symbols migrated
    pub migrated_symbols: usize,
    /// Total file pairs with migrations
    pub file_migration_count: usize,
    /// Total reference changes
    pub reference_change_count: usize,
    /// Migration percentage
    pub overall_migration_percentage: f64,
}

impl SymbolMigrationTracker {
    pub fn new(config: SymbolMigrationTrackerConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(SymbolMigrationTrackerConfig::default())
    }

    /// Track symbol migrations between two versions
    pub fn track_migrations(
        &self,
        source_resolver: &SymbolResolver,
        target_resolver: &SymbolResolver,
    ) -> Result<SymbolMigrationResult> {
        let source_table = source_resolver.get_symbol_table();
        let target_table = target_resolver.get_symbol_table();

        let mut result = SymbolMigrationResult {
            file_migrations: Vec::new(),
            symbol_migrations: Vec::new(),
            reference_changes: Vec::new(),
            statistics: MigrationStatistics {
                total_symbols: 0,
                migrated_symbols: 0,
                file_migration_count: 0,
                reference_change_count: 0,
                overall_migration_percentage: 0.0,
            },
        };

        // Step 1: Identify symbol migrations
        let symbol_migrations = self.identify_symbol_migrations(source_table, target_table)?;
        result.symbol_migrations = symbol_migrations;

        // Step 2: Group migrations by file pairs
        let file_migrations = self.group_migrations_by_files(&result.symbol_migrations);
        result.file_migrations = file_migrations;

        // Step 3: Analyze cross-file reference changes
        if self.config.analyze_cross_file_references {
            let reference_changes =
                self.analyze_reference_changes(source_resolver, target_resolver)?;
            result.reference_changes = reference_changes;
        }

        // Step 4: Calculate statistics
        result.statistics = self.calculate_statistics(&result);

        Ok(result)
    }

    /// Identify individual symbol migrations
    fn identify_symbol_migrations(
        &self,
        source_table: &SymbolTable,
        target_table: &SymbolTable,
    ) -> Result<Vec<SymbolMigration>> {
        let mut migrations = Vec::new();

        // Get all symbols from source
        for (source_file, source_symbols) in &source_table.file_symbols {
            for (symbol_name, source_symbol) in source_symbols {
                // Check if we should track this symbol kind
                if !self.should_track_symbol(&source_symbol.symbol_kind) {
                    continue;
                }

                // Try to find the symbol in target
                let target_symbol = self.find_matching_symbol(
                    symbol_name,
                    source_symbol,
                    target_table,
                );

                if let Some((target_file, _target_sym, was_renamed, new_name)) = target_symbol {
                    // Check if it migrated to a different file
                    if source_file != &target_file {
                        migrations.push(SymbolMigration {
                            symbol_name: symbol_name.clone(),
                            symbol_kind: format!("{:?}", source_symbol.symbol_kind),
                            source_file: source_file.clone(),
                            target_file,
                            was_renamed,
                            new_name,
                            confidence: 0.9, // High confidence for exact matches
                        });
                    }
                }
            }
        }

        Ok(migrations)
    }

    /// Check if we should track this symbol kind
    fn should_track_symbol(&self, kind: &SymbolKind) -> bool {
        match kind {
            SymbolKind::Function | SymbolKind::Method => self.config.track_functions,
            SymbolKind::Class | SymbolKind::Interface => self.config.track_classes,
            SymbolKind::Variable | SymbolKind::Constant | SymbolKind::Field => {
                self.config.track_variables
            }
            _ => false,
        }
    }

    /// Find matching symbol in target table
    fn find_matching_symbol(
        &self,
        symbol_name: &str,
        source_symbol: &Symbol,
        target_table: &SymbolTable,
    ) -> Option<(String, Symbol, bool, Option<String>)> {
        // First try exact name match
        for (target_file, target_symbols) in &target_table.file_symbols {
            if let Some(target_symbol) = target_symbols.get(symbol_name) {
                // Check if it's the same kind of symbol
                if source_symbol.symbol_kind == target_symbol.symbol_kind {
                    return Some((
                        target_file.clone(),
                        target_symbol.clone(),
                        false,
                        None,
                    ));
                }
            }
        }

        // Try to find renamed symbols (same kind, similar location, different name)
        // This is a simplified heuristic - could be enhanced with more sophisticated matching
        for (target_file, target_symbols) in &target_table.file_symbols {
            for (target_name, target_symbol) in target_symbols {
                if target_name != symbol_name
                    && source_symbol.symbol_kind == target_symbol.symbol_kind
                {
                    // Check if line numbers are similar (within 10 lines)
                    let line_diff = (source_symbol.line as i32 - target_symbol.line as i32).abs();
                    if line_diff <= 10 {
                        return Some((
                            target_file.clone(),
                            target_symbol.clone(),
                            true,
                            Some(target_name.clone()),
                        ));
                    }
                }
            }
        }

        None
    }

    /// Group symbol migrations by file pairs
    fn group_migrations_by_files(
        &self,
        symbol_migrations: &[SymbolMigration],
    ) -> Vec<FileMigration> {
        let mut file_pairs: HashMap<(String, String), Vec<String>> = HashMap::new();

        // Group migrations by (source_file, target_file) pairs
        for migration in symbol_migrations {
            let key = (migration.source_file.clone(), migration.target_file.clone());
            file_pairs
                .entry(key)
                .or_default()
                .push(migration.symbol_name.clone());
        }

        // Convert to FileMigration structs
        file_pairs
            .into_iter()
            .map(|((source_file, target_file), migrated_symbols)| {
                let migration_count = migrated_symbols.len();
                let migration_percentage = migration_count as f64 / 100.0; // Simplified
                let confidence = if migration_percentage >= self.config.min_migration_threshold {
                    0.8
                } else {
                    0.5
                };

                FileMigration {
                    source_file,
                    target_file,
                    migrated_symbols,
                    migration_percentage,
                    confidence,
                }
            })
            .collect()
    }

    /// Analyze cross-file reference changes
    fn analyze_reference_changes(
        &self,
        _source_resolver: &SymbolResolver,
        _target_resolver: &SymbolResolver,
    ) -> Result<Vec<ReferenceChange>> {
        // Placeholder implementation
        // In a full implementation, this would:
        // 1. Track all cross-file references in source
        // 2. Track all cross-file references in target
        // 3. Identify which references changed due to symbol migrations
        Ok(Vec::new())
    }

    /// Calculate migration statistics
    fn calculate_statistics(&self, result: &SymbolMigrationResult) -> MigrationStatistics {
        let total_symbols = result.symbol_migrations.len();
        let migrated_symbols = result.symbol_migrations.len();
        let file_migration_count = result.file_migrations.len();
        let reference_change_count = result.reference_changes.len();

        let overall_migration_percentage = if total_symbols > 0 {
            migrated_symbols as f64 / total_symbols as f64
        } else {
            0.0
        };

        MigrationStatistics {
            total_symbols,
            migrated_symbols,
            file_migration_count,
            reference_change_count,
            overall_migration_percentage,
        }
    }

    /// Get configuration
    pub fn get_config(&self) -> &SymbolMigrationTrackerConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: SymbolMigrationTrackerConfig) {
        self.config = config;
    }
}

