//! Class hierarchy tracking for enhanced refactoring detection
//!
//! This module provides advanced tracking of class hierarchies, inheritance relationships,
//! and OOP patterns across files to detect complex refactoring operations:
//! - Class moves with inheritance preservation
//! - Method migrations between classes in a hierarchy
//! - Interface/trait/protocol implementations across files
//! - Mixin/trait composition changes

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Configuration for class hierarchy tracking
#[derive(Debug, Clone)]
pub struct ClassHierarchyTrackerConfig {
    /// Track inheritance relationships
    pub track_inheritance: bool,
    /// Track interface/protocol implementations
    pub track_interfaces: bool,
    /// Track trait/mixin compositions (Ruby, PHP)
    pub track_traits: bool,
    /// Minimum similarity for class matching (0.0 to 1.0)
    pub min_class_similarity: f64,
    /// Minimum similarity for method matching (0.0 to 1.0)
    pub min_method_similarity: f64,
    /// Enable cross-file hierarchy analysis
    pub cross_file_analysis: bool,
    /// Maximum hierarchy depth to analyze
    pub max_hierarchy_depth: usize,
}

impl Default for ClassHierarchyTrackerConfig {
    fn default() -> Self {
        Self {
            track_inheritance: true,
            track_interfaces: true,
            track_traits: true,
            min_class_similarity: 0.7,
            min_method_similarity: 0.6,
            cross_file_analysis: true,
            max_hierarchy_depth: 10,
        }
    }
}

/// Class hierarchy tracker
pub struct ClassHierarchyTracker {
    config: ClassHierarchyTrackerConfig,
}

/// Complete class hierarchy information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassHierarchy {
    /// Root classes (no parent)
    pub root_classes: Vec<ClassNode>,
    /// All classes indexed by qualified name
    pub classes: HashMap<String, ClassNode>,
    /// Inheritance relationships (child -> parent)
    pub inheritance_map: HashMap<String, String>,
    /// Interface implementations (class -> interfaces)
    pub interface_map: HashMap<String, Vec<String>>,
    /// Trait/mixin compositions (class -> traits)
    pub trait_map: HashMap<String, Vec<String>>,
    /// File locations (class -> file path)
    pub file_map: HashMap<String, String>,
}

/// Node representing a class in the hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassNode {
    /// Fully qualified class name
    pub qualified_name: String,
    /// Simple class name
    pub name: String,
    /// Parent class (if any)
    pub parent: Option<String>,
    /// Implemented interfaces/protocols
    pub interfaces: Vec<String>,
    /// Used traits/mixins
    pub traits: Vec<String>,
    /// Methods defined in this class
    pub methods: Vec<MethodInfo>,
    /// Fields/properties
    pub fields: Vec<FieldInfo>,
    /// File path
    pub file_path: String,
    /// Line number
    pub line: usize,
    /// Whether this is abstract
    pub is_abstract: bool,
    /// Whether this is an interface/protocol
    pub is_interface: bool,
}

/// Method information for hierarchy tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodInfo {
    /// Method name
    pub name: String,
    /// Method signature (for matching)
    pub signature: String,
    /// Visibility
    pub visibility: Visibility,
    /// Whether it's static
    pub is_static: bool,
    /// Whether it's abstract
    pub is_abstract: bool,
    /// Whether it overrides a parent method
    pub is_override: bool,
    /// Line number
    pub line: usize,
}

/// Field information for hierarchy tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Visibility
    pub visibility: Visibility,
    /// Whether it's static
    pub is_static: bool,
    /// Line number
    pub line: usize,
}

/// Visibility levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Package,
    Internal,
}

/// Result of class hierarchy analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassHierarchyAnalysisResult {
    /// Detected class moves
    pub class_moves: Vec<ClassMove>,
    /// Detected method migrations
    pub method_migrations: Vec<MethodMigration>,
    /// Detected hierarchy changes
    pub hierarchy_changes: Vec<HierarchyChange>,
    /// Detected interface/trait changes
    pub interface_changes: Vec<InterfaceChange>,
    /// Overall statistics
    pub statistics: HierarchyStatistics,
}

/// Class move detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMove {
    /// Class name
    pub class_name: String,
    /// Source file
    pub source_file: String,
    /// Target file
    pub target_file: String,
    /// Whether inheritance was preserved
    pub inheritance_preserved: bool,
    /// Whether interfaces were preserved
    pub interfaces_preserved: bool,
    /// Methods that moved with the class
    pub moved_methods: Vec<String>,
    /// Confidence score
    pub confidence: f64,
}

/// Method migration between classes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodMigration {
    /// Method name
    pub method_name: String,
    /// Method signature
    pub signature: String,
    /// Source class
    pub source_class: String,
    /// Target class
    pub target_class: String,
    /// Source file
    pub source_file: String,
    /// Target file
    pub target_file: String,
    /// Migration type
    pub migration_type: MethodMigrationType,
    /// Confidence score
    pub confidence: f64,
}

/// Type of method migration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MethodMigrationType {
    /// Method moved to parent class (pull up)
    PullUp,
    /// Method moved to child class (push down)
    PushDown,
    /// Method moved to sibling class
    MovedToSibling,
    /// Method extracted to new class
    ExtractedToNewClass,
    /// Method moved to unrelated class
    MovedToUnrelated,
}

/// Change in class hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyChange {
    /// Class affected
    pub class_name: String,
    /// Type of change
    pub change_type: HierarchyChangeType,
    /// Old parent (if applicable)
    pub old_parent: Option<String>,
    /// New parent (if applicable)
    pub new_parent: Option<String>,
    /// File path
    pub file_path: String,
    /// Confidence score
    pub confidence: f64,
}

/// Type of hierarchy change
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HierarchyChangeType {
    /// Parent class changed
    ParentChanged,
    /// Inheritance added
    InheritanceAdded,
    /// Inheritance removed
    InheritanceRemoved,
    /// Class flattened (inheritance removed, methods inlined)
    ClassFlattened,
    /// Class extracted from parent
    ClassExtracted,
}

/// Change in interface/trait implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceChange {
    /// Class affected
    pub class_name: String,
    /// Type of change
    pub change_type: InterfaceChangeType,
    /// Interface/trait name
    pub interface_name: String,
    /// File path
    pub file_path: String,
    /// Confidence score
    pub confidence: f64,
}

/// Type of interface/trait change
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InterfaceChangeType {
    /// Interface/protocol added
    InterfaceAdded,
    /// Interface/protocol removed
    InterfaceRemoved,
    /// Trait/mixin added
    TraitAdded,
    /// Trait/mixin removed
    TraitRemoved,
    /// Interface extracted from class
    InterfaceExtracted,
}

/// Statistics for hierarchy analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyStatistics {
    /// Total classes analyzed
    pub total_classes: usize,
    /// Classes with inheritance
    pub classes_with_inheritance: usize,
    /// Classes implementing interfaces
    pub classes_with_interfaces: usize,
    /// Classes using traits/mixins
    pub classes_with_traits: usize,
    /// Maximum hierarchy depth
    pub max_hierarchy_depth: usize,
    /// Average methods per class
    pub avg_methods_per_class: f64,
    /// Total class moves detected
    pub total_class_moves: usize,
    /// Total method migrations detected
    pub total_method_migrations: usize,
    /// Total hierarchy changes detected
    pub total_hierarchy_changes: usize,
}

impl ClassHierarchyTracker {
    /// Create a new class hierarchy tracker
    pub fn new(config: ClassHierarchyTrackerConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(ClassHierarchyTrackerConfig::default())
    }

    /// Build class hierarchy from source code
    pub fn build_hierarchy(
        &self,
        source_classes: &HashMap<String, ClassNode>,
    ) -> Result<ClassHierarchy> {
        let mut hierarchy = ClassHierarchy {
            root_classes: Vec::new(),
            classes: source_classes.clone(),
            inheritance_map: HashMap::new(),
            interface_map: HashMap::new(),
            trait_map: HashMap::new(),
            file_map: HashMap::new(),
        };

        // Build maps
        for (name, class) in source_classes {
            // Inheritance map
            if let Some(parent) = &class.parent {
                hierarchy.inheritance_map.insert(name.clone(), parent.clone());
            } else {
                hierarchy.root_classes.push(class.clone());
            }

            // Interface map
            if !class.interfaces.is_empty() {
                hierarchy.interface_map.insert(name.clone(), class.interfaces.clone());
            }

            // Trait map
            if !class.traits.is_empty() {
                hierarchy.trait_map.insert(name.clone(), class.traits.clone());
            }

            // File map
            hierarchy.file_map.insert(name.clone(), class.file_path.clone());
        }

        Ok(hierarchy)
    }

    /// Analyze changes between two hierarchies
    pub fn analyze_hierarchy_changes(
        &self,
        source_hierarchy: &ClassHierarchy,
        target_hierarchy: &ClassHierarchy,
    ) -> Result<ClassHierarchyAnalysisResult> {
        let mut class_moves = Vec::new();
        let mut method_migrations = Vec::new();
        let mut hierarchy_changes = Vec::new();
        let mut interface_changes = Vec::new();

        // Detect class moves
        class_moves.extend(self.detect_class_moves(source_hierarchy, target_hierarchy)?);

        // Detect method migrations
        method_migrations.extend(self.detect_method_migrations(source_hierarchy, target_hierarchy)?);

        // Detect hierarchy changes
        hierarchy_changes.extend(self.detect_hierarchy_changes(source_hierarchy, target_hierarchy)?);

        // Detect interface/trait changes
        interface_changes.extend(self.detect_interface_changes(source_hierarchy, target_hierarchy)?);

        // Calculate statistics
        let statistics = self.calculate_statistics(
            source_hierarchy,
            target_hierarchy,
            &class_moves,
            &method_migrations,
            &hierarchy_changes,
        );

        Ok(ClassHierarchyAnalysisResult {
            class_moves,
            method_migrations,
            hierarchy_changes,
            interface_changes,
            statistics,
        })
    }

    /// Detect class moves between files
    fn detect_class_moves(
        &self,
        source_hierarchy: &ClassHierarchy,
        target_hierarchy: &ClassHierarchy,
    ) -> Result<Vec<ClassMove>> {
        let mut moves = Vec::new();

        for (class_name, source_class) in &source_hierarchy.classes {
            if let Some(target_class) = target_hierarchy.classes.get(class_name) {
                // Check if file changed
                if source_class.file_path != target_class.file_path {
                    let inheritance_preserved = source_class.parent == target_class.parent;
                    let interfaces_preserved =
                        source_class.interfaces == target_class.interfaces;

                    let moved_methods = self.find_moved_methods(source_class, target_class);

                    let confidence = self.calculate_class_move_confidence(
                        source_class,
                        target_class,
                        inheritance_preserved,
                        interfaces_preserved,
                    );

                    if confidence >= self.config.min_class_similarity {
                        moves.push(ClassMove {
                            class_name: class_name.clone(),
                            source_file: source_class.file_path.clone(),
                            target_file: target_class.file_path.clone(),
                            inheritance_preserved,
                            interfaces_preserved,
                            moved_methods,
                            confidence,
                        });
                    }
                }
            }
        }

        Ok(moves)
    }

    /// Find methods that moved with a class
    fn find_moved_methods(
        &self,
        source_class: &ClassNode,
        target_class: &ClassNode,
    ) -> Vec<String> {
        let mut moved = Vec::new();

        for source_method in &source_class.methods {
            if target_class.methods.iter().any(|m| {
                m.name == source_method.name &&
                self.methods_similar(&source_method.signature, &m.signature)
            }) {
                moved.push(source_method.name.clone());
            }
        }

        moved
    }

    /// Detect method migrations between classes
    fn detect_method_migrations(
        &self,
        source_hierarchy: &ClassHierarchy,
        target_hierarchy: &ClassHierarchy,
    ) -> Result<Vec<MethodMigration>> {
        let mut migrations = Vec::new();

        // For each class in source
        for (source_class_name, source_class) in &source_hierarchy.classes {
            // For each method in source class
            for source_method in &source_class.methods {
                // Check if method exists in target class
                let exists_in_same_class = target_hierarchy
                    .classes
                    .get(source_class_name)
                    .map(|tc| tc.methods.iter().any(|m| m.name == source_method.name))
                    .unwrap_or(false);

                if !exists_in_same_class {
                    // Method might have migrated - search other classes
                    if let Some(migration) = self.find_method_migration(
                        source_method,
                        source_class_name,
                        &source_class.file_path,
                        source_hierarchy,
                        target_hierarchy,
                    )? {
                        migrations.push(migration);
                    }
                }
            }
        }

        Ok(migrations)
    }

    /// Find where a method migrated to
    fn find_method_migration(
        &self,
        source_method: &MethodInfo,
        source_class_name: &str,
        source_file: &str,
        source_hierarchy: &ClassHierarchy,
        target_hierarchy: &ClassHierarchy,
    ) -> Result<Option<MethodMigration>> {
        let mut best_match: Option<(String, String, String, f64, MethodMigrationType)> = None;

        // Search all target classes
        for (target_class_name, target_class) in &target_hierarchy.classes {
            for target_method in &target_class.methods {
                if self.methods_similar(&source_method.signature, &target_method.signature) {
                    let confidence = self.calculate_method_similarity(source_method, target_method);

                    if confidence >= self.config.min_method_similarity {
                        let migration_type = self.determine_migration_type(
                            source_class_name,
                            target_class_name,
                            source_hierarchy,
                            target_hierarchy,
                        );

                        if best_match.is_none() || confidence > best_match.as_ref().unwrap().3 {
                            best_match = Some((
                                target_class_name.clone(),
                                target_class.file_path.clone(),
                                target_method.name.clone(),
                                confidence,
                                migration_type,
                            ));
                        }
                    }
                }
            }
        }

        if let Some((target_class, target_file, method_name, confidence, migration_type)) = best_match {
            Ok(Some(MethodMigration {
                method_name,
                signature: source_method.signature.clone(),
                source_class: source_class_name.to_string(),
                target_class,
                source_file: source_file.to_string(),
                target_file,
                migration_type,
                confidence,
            }))
        } else {
            Ok(None)
        }
    }

    /// Determine the type of method migration
    fn determine_migration_type(
        &self,
        source_class: &str,
        target_class: &str,
        source_hierarchy: &ClassHierarchy,
        target_hierarchy: &ClassHierarchy,
    ) -> MethodMigrationType {
        // Check if target is parent of source (pull up)
        if self.is_ancestor(source_class, target_class, target_hierarchy) {
            return MethodMigrationType::PullUp;
        }

        // Check if target is child of source (push down)
        if self.is_ancestor(target_class, source_class, target_hierarchy) {
            return MethodMigrationType::PushDown;
        }

        // Check if they share a parent (siblings)
        if self.are_siblings(source_class, target_class, source_hierarchy, target_hierarchy) {
            return MethodMigrationType::MovedToSibling;
        }

        // Check if target class is new
        if !source_hierarchy.classes.contains_key(target_class) {
            return MethodMigrationType::ExtractedToNewClass;
        }

        MethodMigrationType::MovedToUnrelated
    }

    /// Check if class1 is an ancestor of class2
    fn is_ancestor(
        &self,
        class1: &str,
        class2: &str,
        hierarchy: &ClassHierarchy,
    ) -> bool {
        let mut current = class2;
        let mut depth = 0;

        while depth < self.config.max_hierarchy_depth {
            if let Some(parent) = hierarchy.inheritance_map.get(current) {
                if parent == class1 {
                    return true;
                }
                current = parent;
                depth += 1;
            } else {
                break;
            }
        }

        false
    }

    /// Check if two classes are siblings (share same parent)
    fn are_siblings(
        &self,
        class1: &str,
        class2: &str,
        source_hierarchy: &ClassHierarchy,
        target_hierarchy: &ClassHierarchy,
    ) -> bool {
        let parent1 = source_hierarchy.inheritance_map.get(class1)
            .or_else(|| target_hierarchy.inheritance_map.get(class1));
        let parent2 = source_hierarchy.inheritance_map.get(class2)
            .or_else(|| target_hierarchy.inheritance_map.get(class2));

        match (parent1, parent2) {
            (Some(p1), Some(p2)) => p1 == p2,
            _ => false,
        }
    }


    /// Detect hierarchy changes
    fn detect_hierarchy_changes(
        &self,
        source_hierarchy: &ClassHierarchy,
        target_hierarchy: &ClassHierarchy,
    ) -> Result<Vec<HierarchyChange>> {
        let mut changes = Vec::new();

        // Check all classes that exist in both hierarchies
        for (class_name, source_class) in &source_hierarchy.classes {
            if let Some(target_class) = target_hierarchy.classes.get(class_name) {
                // Check if parent changed
                if source_class.parent != target_class.parent {
                    let change_type = match (&source_class.parent, &target_class.parent) {
                        (None, Some(_)) => HierarchyChangeType::InheritanceAdded,
                        (Some(_), None) => HierarchyChangeType::InheritanceRemoved,
                        (Some(_), Some(_)) => HierarchyChangeType::ParentChanged,
                        (None, None) => continue,
                    };

                    changes.push(HierarchyChange {
                        class_name: class_name.clone(),
                        change_type,
                        old_parent: source_class.parent.clone(),
                        new_parent: target_class.parent.clone(),
                        file_path: target_class.file_path.clone(),
                        confidence: 1.0, // Direct comparison, high confidence
                    });
                }
            }
        }

        // Detect class flattening (inheritance removed, methods inlined)
        for (class_name, source_class) in &source_hierarchy.classes {
            if let Some(target_class) = target_hierarchy.classes.get(class_name) {
                if source_class.parent.is_some() && target_class.parent.is_none() {
                    // Check if methods from parent were inlined
                    if let Some(parent_name) = &source_class.parent {
                        if let Some(parent_class) = source_hierarchy.classes.get(parent_name) {
                            let inlined_methods = self.count_inlined_methods(
                                parent_class,
                                source_class,
                                target_class,
                            );

                            if inlined_methods > 0 {
                                changes.push(HierarchyChange {
                                    class_name: class_name.clone(),
                                    change_type: HierarchyChangeType::ClassFlattened,
                                    old_parent: source_class.parent.clone(),
                                    new_parent: None,
                                    file_path: target_class.file_path.clone(),
                                    confidence: 0.8,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(changes)
    }

    /// Detect interface/trait changes
    fn detect_interface_changes(
        &self,
        source_hierarchy: &ClassHierarchy,
        target_hierarchy: &ClassHierarchy,
    ) -> Result<Vec<InterfaceChange>> {
        let mut changes = Vec::new();

        for (class_name, source_class) in &source_hierarchy.classes {
            if let Some(target_class) = target_hierarchy.classes.get(class_name) {
                // Check interface changes
                let source_interfaces: HashSet<_> = source_class.interfaces.iter().collect();
                let target_interfaces: HashSet<_> = target_class.interfaces.iter().collect();

                // Added interfaces
                for interface in target_interfaces.difference(&source_interfaces) {
                    changes.push(InterfaceChange {
                        class_name: class_name.clone(),
                        change_type: InterfaceChangeType::InterfaceAdded,
                        interface_name: (*interface).clone(),
                        file_path: target_class.file_path.clone(),
                        confidence: 1.0,
                    });
                }

                // Removed interfaces
                for interface in source_interfaces.difference(&target_interfaces) {
                    changes.push(InterfaceChange {
                        class_name: class_name.clone(),
                        change_type: InterfaceChangeType::InterfaceRemoved,
                        interface_name: (*interface).clone(),
                        file_path: target_class.file_path.clone(),
                        confidence: 1.0,
                    });
                }

                // Check trait changes
                let source_traits: HashSet<_> = source_class.traits.iter().collect();
                let target_traits: HashSet<_> = target_class.traits.iter().collect();

                // Added traits
                for trait_name in target_traits.difference(&source_traits) {
                    changes.push(InterfaceChange {
                        class_name: class_name.clone(),
                        change_type: InterfaceChangeType::TraitAdded,
                        interface_name: (*trait_name).clone(),
                        file_path: target_class.file_path.clone(),
                        confidence: 1.0,
                    });
                }

                // Removed traits
                for trait_name in source_traits.difference(&target_traits) {
                    changes.push(InterfaceChange {
                        class_name: class_name.clone(),
                        change_type: InterfaceChangeType::TraitRemoved,
                        interface_name: (*trait_name).clone(),
                        file_path: target_class.file_path.clone(),
                        confidence: 1.0,
                    });
                }
            }
        }

        Ok(changes)
    }

    /// Calculate statistics
    fn calculate_statistics(
        &self,
        _source_hierarchy: &ClassHierarchy,
        target_hierarchy: &ClassHierarchy,
        class_moves: &[ClassMove],
        method_migrations: &[MethodMigration],
        hierarchy_changes: &[HierarchyChange],
    ) -> HierarchyStatistics {
        let total_classes = target_hierarchy.classes.len();
        let classes_with_inheritance = target_hierarchy.inheritance_map.len();
        let classes_with_interfaces = target_hierarchy.interface_map.len();
        let classes_with_traits = target_hierarchy.trait_map.len();

        let max_hierarchy_depth = self.calculate_max_depth(target_hierarchy);

        let total_methods: usize = target_hierarchy
            .classes
            .values()
            .map(|c| c.methods.len())
            .sum();
        let avg_methods_per_class = if total_classes > 0 {
            total_methods as f64 / total_classes as f64
        } else {
            0.0
        };

        HierarchyStatistics {
            total_classes,
            classes_with_inheritance,
            classes_with_interfaces,
            classes_with_traits,
            max_hierarchy_depth,
            avg_methods_per_class,
            total_class_moves: class_moves.len(),
            total_method_migrations: method_migrations.len(),
            total_hierarchy_changes: hierarchy_changes.len(),
        }
    }

    /// Calculate maximum hierarchy depth
    fn calculate_max_depth(&self, hierarchy: &ClassHierarchy) -> usize {
        let mut max_depth = 0;

        for class_name in hierarchy.classes.keys() {
            let depth = self.get_class_depth(class_name, hierarchy);
            max_depth = max_depth.max(depth);
        }

        max_depth
    }

    /// Get depth of a class in the hierarchy
    fn get_class_depth(&self, class_name: &str, hierarchy: &ClassHierarchy) -> usize {
        let mut depth = 0;
        let mut current = class_name;

        while depth < self.config.max_hierarchy_depth {
            if let Some(parent) = hierarchy.inheritance_map.get(current) {
                depth += 1;
                current = parent;
            } else {
                break;
            }
        }

        depth
    }

    /// Count methods inlined from parent
    fn count_inlined_methods(
        &self,
        parent_class: &ClassNode,
        source_child: &ClassNode,
        target_child: &ClassNode,
    ) -> usize {
        let mut count = 0;

        for parent_method in &parent_class.methods {
            // Check if method exists in target child but not in source child
            let in_target = target_child.methods.iter().any(|m| {
                m.name == parent_method.name &&
                self.methods_similar(&m.signature, &parent_method.signature)
            });

            let in_source = source_child.methods.iter().any(|m| {
                m.name == parent_method.name
            });

            if in_target && !in_source {
                count += 1;
            }
        }

        count
    }

    /// Check if two method signatures are similar
    fn methods_similar(&self, sig1: &str, sig2: &str) -> bool {
        // Simple similarity check - can be enhanced
        if sig1 == sig2 {
            return true;
        }

        // Normalize signatures (remove whitespace, etc.)
        let norm1 = self.normalize_signature(sig1);
        let norm2 = self.normalize_signature(sig2);

        norm1 == norm2
    }

    /// Normalize a method signature for comparison
    fn normalize_signature(&self, signature: &str) -> String {
        signature
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_lowercase()
    }

    /// Calculate method similarity score
    fn calculate_method_similarity(&self, method1: &MethodInfo, method2: &MethodInfo) -> f64 {
        let mut score = 0.0;

        // Name match
        if method1.name == method2.name {
            score += 0.4;
        }

        // Signature match
        if self.methods_similar(&method1.signature, &method2.signature) {
            score += 0.4;
        }

        // Visibility match
        if method1.visibility == method2.visibility {
            score += 0.1;
        }

        // Static match
        if method1.is_static == method2.is_static {
            score += 0.05;
        }

        // Abstract match
        if method1.is_abstract == method2.is_abstract {
            score += 0.05;
        }

        score
    }

    /// Calculate class move confidence
    fn calculate_class_move_confidence(
        &self,
        source_class: &ClassNode,
        target_class: &ClassNode,
        inheritance_preserved: bool,
        interfaces_preserved: bool,
    ) -> f64 {
        let mut confidence = 0.0;

        // Name match (should always be true for class moves)
        if source_class.name == target_class.name {
            confidence += 0.3;
        }

        // Method preservation
        let method_preservation = self.calculate_method_preservation(source_class, target_class);
        confidence += method_preservation * 0.3;

        // Field preservation
        let field_preservation = self.calculate_field_preservation(source_class, target_class);
        confidence += field_preservation * 0.2;

        // Inheritance preservation
        if inheritance_preserved {
            confidence += 0.1;
        }

        // Interface preservation
        if interfaces_preserved {
            confidence += 0.1;
        }

        confidence
    }

    /// Calculate method preservation ratio
    fn calculate_method_preservation(&self, source: &ClassNode, target: &ClassNode) -> f64 {
        if source.methods.is_empty() {
            return 1.0;
        }

        let preserved = source.methods.iter().filter(|sm| {
            target.methods.iter().any(|tm| {
                tm.name == sm.name && self.methods_similar(&tm.signature, &sm.signature)
            })
        }).count();

        preserved as f64 / source.methods.len() as f64
    }

    /// Calculate field preservation ratio
    fn calculate_field_preservation(&self, source: &ClassNode, target: &ClassNode) -> f64 {
        if source.fields.is_empty() {
            return 1.0;
        }

        let preserved = source.fields.iter().filter(|sf| {
            target.fields.iter().any(|tf| tf.name == sf.name)
        }).count();

        preserved as f64 / source.fields.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_method(name: &str, signature: &str) -> MethodInfo {
        MethodInfo {
            name: name.to_string(),
            signature: signature.to_string(),
            visibility: Visibility::Public,
            is_static: false,
            is_abstract: false,
            is_override: false,
            line: 1,
        }
    }

    fn create_test_class(name: &str, parent: Option<String>, methods: Vec<MethodInfo>) -> ClassNode {
        ClassNode {
            qualified_name: name.to_string(),
            name: name.to_string(),
            parent,
            interfaces: Vec::new(),
            traits: Vec::new(),
            methods,
            fields: Vec::new(),
            file_path: format!("{}.java", name),
            line: 1,
            is_abstract: false,
            is_interface: false,
        }
    }

    #[test]
    fn test_method_similarity() {
        let tracker = ClassHierarchyTracker::default();

        assert!(tracker.methods_similar("void foo(int x)", "void foo(int x)"));
        assert!(tracker.methods_similar("void foo(int x)", "void  foo( int  x )"));
        assert!(!tracker.methods_similar("void foo(int x)", "void bar(int x)"));
    }

    #[test]
    fn test_is_ancestor() {
        let tracker = ClassHierarchyTracker::default();

        let mut classes = HashMap::new();
        classes.insert("Child".to_string(), create_test_class("Child", Some("Parent".to_string()), vec![]));
        classes.insert("Parent".to_string(), create_test_class("Parent", Some("GrandParent".to_string()), vec![]));
        classes.insert("GrandParent".to_string(), create_test_class("GrandParent", None, vec![]));

        let hierarchy = tracker.build_hierarchy(&classes).unwrap();

        assert!(tracker.is_ancestor("Parent", "Child", &hierarchy));
        assert!(tracker.is_ancestor("GrandParent", "Child", &hierarchy));
        assert!(!tracker.is_ancestor("Child", "Parent", &hierarchy));
    }

    #[test]
    fn test_are_siblings() {
        let tracker = ClassHierarchyTracker::default();

        let mut classes = HashMap::new();
        classes.insert("Child1".to_string(), create_test_class("Child1", Some("Parent".to_string()), vec![]));
        classes.insert("Child2".to_string(), create_test_class("Child2", Some("Parent".to_string()), vec![]));
        classes.insert("Parent".to_string(), create_test_class("Parent", None, vec![]));

        let hierarchy = tracker.build_hierarchy(&classes).unwrap();

        assert!(tracker.are_siblings("Child1", "Child2", &hierarchy, &hierarchy));
        assert!(!tracker.are_siblings("Child1", "Parent", &hierarchy, &hierarchy));
    }

    #[test]
    fn test_detect_class_move() {
        let tracker = ClassHierarchyTracker::default();

        let method1 = create_test_method("foo", "void foo()");
        let method2 = create_test_method("bar", "int bar(String s)");

        let mut source_classes = HashMap::new();
        let mut source_class = create_test_class("MyClass", None, vec![method1.clone(), method2.clone()]);
        source_class.file_path = "old/MyClass.java".to_string();
        source_classes.insert("MyClass".to_string(), source_class);

        let mut target_classes = HashMap::new();
        let mut target_class = create_test_class("MyClass", None, vec![method1, method2]);
        target_class.file_path = "new/MyClass.java".to_string();
        target_classes.insert("MyClass".to_string(), target_class);

        let source_hierarchy = tracker.build_hierarchy(&source_classes).unwrap();
        let target_hierarchy = tracker.build_hierarchy(&target_classes).unwrap();

        let moves = tracker.detect_class_moves(&source_hierarchy, &target_hierarchy).unwrap();

        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].class_name, "MyClass");
        assert_eq!(moves[0].source_file, "old/MyClass.java");
        assert_eq!(moves[0].target_file, "new/MyClass.java");
    }
}

