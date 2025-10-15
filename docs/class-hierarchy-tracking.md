# Class Hierarchy Tracking for Enhanced Refactoring Detection

## Overview

The Class Hierarchy Tracking system provides advanced detection of refactoring operations in class-based languages (Java, C++, PHP, Swift, Ruby, etc.). It tracks class hierarchies, inheritance relationships, and OOP patterns across files to detect complex refactoring operations that would be missed by simple text-based diff tools.

## Key Features

### 1. Class Move Detection
Detects when classes are moved between files while preserving their structure:
- **Inheritance Preservation**: Tracks whether parent-child relationships are maintained
- **Interface Preservation**: Monitors interface/protocol implementations
- **Method Preservation**: Identifies which methods moved with the class
- **Confidence Scoring**: Provides confidence levels based on multiple factors

### 2. Method Migration Detection
Identifies method movements between classes with classification:
- **Pull Up**: Method moved from child to parent class (generalization)
- **Push Down**: Method moved from parent to child class (specialization)
- **Move to Sibling**: Method moved between classes with same parent
- **Extract to New Class**: Method extracted to a newly created class
- **Move to Unrelated**: Method moved to an unrelated class

### 3. Hierarchy Change Detection
Tracks changes in class inheritance structure:
- **Parent Changed**: Class changes its parent class
- **Inheritance Added**: Class gains a parent (was standalone)
- **Inheritance Removed**: Class loses its parent (becomes standalone)
- **Class Flattened**: Inheritance removed with parent methods inlined
- **Class Extracted**: New class extracted from existing hierarchy

### 4. Interface/Trait Change Detection
Monitors interface and trait implementations:
- **Interface Added/Removed**: Protocol/interface implementation changes
- **Trait Added/Removed**: Mixin/trait composition changes (Ruby, PHP, Swift)
- **Interface Extracted**: Interface extracted from class implementation

## Architecture

### Core Components

```
ClassHierarchyTracker
├── ClassHierarchy (data structure)
│   ├── root_classes: Vec<ClassNode>
│   ├── classes: HashMap<String, ClassNode>
│   ├── inheritance_map: HashMap<String, String>
│   ├── interface_map: HashMap<String, Vec<String>>
│   └── trait_map: HashMap<String, Vec<String>>
│
├── Detection Algorithms
│   ├── detect_class_moves()
│   ├── detect_method_migrations()
│   ├── detect_hierarchy_changes()
│   └── detect_interface_changes()
│
└── Analysis Utilities
    ├── is_ancestor()
    ├── are_siblings()
    ├── calculate_method_similarity()
    └── calculate_class_move_confidence()
```

### Data Structures

#### ClassNode
Represents a class in the hierarchy:
```rust
pub struct ClassNode {
    pub qualified_name: String,
    pub name: String,
    pub parent: Option<String>,
    pub interfaces: Vec<String>,
    pub traits: Vec<String>,
    pub methods: Vec<MethodInfo>,
    pub fields: Vec<FieldInfo>,
    pub file_path: String,
    pub line: usize,
    pub is_abstract: bool,
    pub is_interface: bool,
}
```

#### MethodInfo
Represents a method for tracking:
```rust
pub struct MethodInfo {
    pub name: String,
    pub signature: String,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_abstract: bool,
    pub is_override: bool,
    pub line: usize,
}
```

## Usage

### Basic Usage

```rust
use smart_diff_engine::{
    ClassHierarchyTracker, ClassHierarchyTrackerConfig,
    ClassNode, MethodInfo, Visibility
};
use std::collections::HashMap;

// Create tracker
let config = ClassHierarchyTrackerConfig::default();
let tracker = ClassHierarchyTracker::new(config);

// Build source hierarchy
let source_classes: HashMap<String, ClassNode> = /* ... */;
let source_hierarchy = tracker.build_hierarchy(&source_classes)?;

// Build target hierarchy
let target_classes: HashMap<String, ClassNode> = /* ... */;
let target_hierarchy = tracker.build_hierarchy(&target_classes)?;

// Analyze changes
let result = tracker.analyze_hierarchy_changes(
    &source_hierarchy,
    &target_hierarchy
)?;

// Process results
for class_move in &result.class_moves {
    println!("Class {} moved from {} to {}",
        class_move.class_name,
        class_move.source_file,
        class_move.target_file
    );
}

for migration in &result.method_migrations {
    println!("Method {} migrated: {:?}",
        migration.method_name,
        migration.migration_type
    );
}
```

### Configuration

```rust
let config = ClassHierarchyTrackerConfig {
    track_inheritance: true,
    track_interfaces: true,
    track_traits: true,
    min_class_similarity: 0.7,
    min_method_similarity: 0.6,
    cross_file_analysis: true,
    max_hierarchy_depth: 10,
};
```

## Detection Algorithms

### Class Move Detection

**Algorithm**:
1. For each class in source hierarchy
2. Find matching class in target hierarchy (by name)
3. Check if file path changed
4. Calculate confidence based on:
   - Name match (30%)
   - Method preservation (30%)
   - Field preservation (20%)
   - Inheritance preservation (10%)
   - Interface preservation (10%)
5. If confidence >= threshold, report as class move

**Confidence Factors**:
- **High (>90%)**: All methods, fields, and relationships preserved
- **Medium (70-90%)**: Most structure preserved, minor changes
- **Low (50-70%)**: Significant changes but still recognizable

### Method Migration Detection

**Algorithm**:
1. For each method in source classes
2. Check if method exists in same class in target
3. If not, search all target classes for similar method
4. Calculate method similarity based on:
   - Name match (40%)
   - Signature match (40%)
   - Visibility match (10%)
   - Static/abstract flags (10%)
5. Determine migration type based on class relationships
6. If similarity >= threshold, report as migration

**Migration Type Determination**:
```
if target_class is ancestor of source_class:
    → Pull Up
else if target_class is descendant of source_class:
    → Push Down
else if target_class and source_class share parent:
    → Move to Sibling
else if target_class is new:
    → Extract to New Class
else:
    → Move to Unrelated
```

### Hierarchy Change Detection

**Algorithm**:
1. For each class in both hierarchies
2. Compare parent relationships
3. Detect changes:
   - Parent changed: Different parent in target
   - Inheritance added: No parent → Has parent
   - Inheritance removed: Has parent → No parent
4. For inheritance removal, check for class flattening:
   - Count methods from parent inlined into child
   - If > 0, report as class flattening

### Interface/Trait Change Detection

**Algorithm**:
1. For each class in both hierarchies
2. Compare interface lists (set difference)
3. Report added interfaces
4. Report removed interfaces
5. Repeat for trait lists

## Language-Specific Considerations

### Java
- **Interfaces**: Full support for interface implementation tracking
- **Abstract Classes**: Tracked via `is_abstract` flag
- **Inner Classes**: Qualified names include outer class

### C++
- **Multiple Inheritance**: Supported via parent list
- **Virtual Methods**: Tracked via `is_override` flag
- **Templates**: Generic parameters in qualified name

### PHP
- **Traits**: Full support for trait composition tracking
- **Interfaces**: Standard interface tracking
- **Namespaces**: Included in qualified names

### Swift
- **Protocols**: Tracked as interfaces
- **Extensions**: Tracked separately
- **Protocol Extensions**: Special handling

### Ruby
- **Modules**: Tracked as traits/mixins
- **Mixins**: Full support for include/extend tracking
- **Duck Typing**: Best-effort type inference

## Performance Characteristics

### Time Complexity
- **Build Hierarchy**: O(n) where n = number of classes
- **Detect Class Moves**: O(n) where n = number of classes
- **Detect Method Migrations**: O(n × m × k) where:
  - n = number of source classes
  - m = average methods per class
  - k = number of target classes
- **Detect Hierarchy Changes**: O(n) where n = number of classes
- **Detect Interface Changes**: O(n × i) where:
  - n = number of classes
  - i = average interfaces per class

### Space Complexity
- **Hierarchy Storage**: O(n + e) where:
  - n = number of classes
  - e = number of inheritance edges
- **Analysis Results**: O(c + m + h + i) where:
  - c = number of class moves
  - m = number of method migrations
  - h = number of hierarchy changes
  - i = number of interface changes

### Optimization Strategies
1. **Early Termination**: Stop searching when confidence threshold met
2. **Caching**: Cache method similarity calculations
3. **Parallel Processing**: Analyze classes in parallel (future enhancement)
4. **Incremental Analysis**: Only analyze changed classes (future enhancement)

## Integration with Existing Systems

### Cross-File Tracker Integration
The Class Hierarchy Tracker complements the existing Cross-File Tracker:
- **Cross-File Tracker**: Detects file-level refactorings (renames, splits, merges)
- **Class Hierarchy Tracker**: Detects class-level refactorings within and across files
- **Combined**: Provides complete picture of structural refactorings

### Symbol Migration Tracker Integration
Works together with Symbol Migration Tracker:
- **Symbol Migration**: Tracks all symbol movements
- **Class Hierarchy**: Provides context for why symbols moved (inheritance, extraction, etc.)
- **Enhanced Detection**: Hierarchy context improves migration confidence

## Examples

See `examples/class_hierarchy_tracking_demo.rs` for comprehensive examples of:
1. Class moves with inheritance preservation
2. Method pull up refactoring
3. Method push down refactoring
4. Class flattening
5. Interface/trait changes
6. Complex hierarchy refactoring

## Testing

The module includes comprehensive unit tests:
- `test_method_similarity`: Method signature comparison
- `test_is_ancestor`: Hierarchy traversal
- `test_are_siblings`: Sibling detection
- `test_detect_class_move`: Class move detection

Run tests:
```bash
cargo test -p smart-diff-engine class_hierarchy_tracker
```

## Future Enhancements

1. **Call Graph Integration**: Use method call relationships for better migration detection
2. **Dependency Analysis**: Consider class dependencies in confidence scoring
3. **Pattern Recognition**: Detect common refactoring patterns (Strategy, Factory, etc.)
4. **Multi-Language Support**: Language-specific optimizations
5. **Performance Optimization**: Parallel processing and caching
6. **Visualization**: Generate hierarchy diff visualizations
7. **Metrics**: Calculate refactoring quality metrics

## References

- Martin Fowler, "Refactoring: Improving the Design of Existing Code"
- "Design Patterns: Elements of Reusable Object-Oriented Software"
- Tree-sitter documentation for language-specific parsing

