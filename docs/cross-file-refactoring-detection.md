# Cross-File Refactoring Detection

## Overview

The Smart Diff Engine provides comprehensive cross-file refactoring detection capabilities that go beyond simple line-by-line comparison. This document describes the advanced features for detecting code movements, renames, splits, and merges across multiple files.

## Features

### 1. File-Level Refactoring Detection

The `FileRefactoringDetector` identifies structural changes at the file level:

#### File Renames
Detects when a file is renamed but maintains similar content:
- **Content Similarity**: Compares file content using fingerprinting
- **Path Similarity**: Analyzes file path and name similarity
- **Symbol Migration**: Tracks how symbols move between files

#### File Splits
Detects when a single file is split into multiple files:
- Identifies when content from one source file appears in multiple target files
- Tracks which symbols migrated to which files
- Provides confidence scores for each split detection

#### File Merges
Detects when multiple files are merged into one:
- Identifies when content from multiple source files appears in a single target file
- Tracks symbol consolidation
- Provides confidence scores for merge detection

#### File Moves
Detects when files are moved to different directories:
- Distinguishes between pure moves (same name, different directory)
- Detects combined move+rename operations
- Tracks directory structure changes

### 2. Symbol Migration Tracking

The `SymbolMigrationTracker` provides detailed tracking of how symbols move between files:

#### Symbol-Level Tracking
- Tracks individual functions, classes, and variables
- Detects symbol renames during migration
- Provides confidence scores for each migration

#### File Migration Analysis
- Groups symbol migrations by file pairs
- Calculates migration percentages
- Identifies patterns in refactoring

#### Cross-File Reference Analysis
- Tracks how references change when symbols move
- Identifies broken references
- Detects reference updates that follow moved symbols

### 3. Global Symbol Table Integration

Enhanced integration with the semantic analysis layer:

#### Symbol Resolution
- Uses `SymbolResolver` to track symbols across files
- Resolves cross-file references
- Builds import dependency graphs

#### Reference Tracking
- Tracks all symbol references across the codebase
- Identifies which files reference which symbols
- Detects when references need to be updated

## Usage

### Basic File Refactoring Detection

```rust
use smart_diff_engine::{FileRefactoringDetector, FileRefactoringDetectorConfig};
use std::collections::HashMap;

// Create detector with default configuration
let detector = FileRefactoringDetector::with_defaults();

// Prepare source and target file contents
let mut source_files = HashMap::new();
source_files.insert("Calculator.java".to_string(), source_content);

let mut target_files = HashMap::new();
target_files.insert("MathCalculator.java".to_string(), target_content);

// Detect refactorings
let result = detector.detect_file_refactorings(&source_files, &target_files)?;

// Analyze results
for rename in &result.file_renames {
    println!("Renamed: {} -> {}", rename.source_path, rename.target_path);
    println!("Confidence: {:.2}%", rename.confidence * 100.0);
}

for split in &result.file_splits {
    println!("Split: {} into {} files", split.source_path, split.target_files.len());
}

for merge in &result.file_merges {
    println!("Merged: {} files into {}", merge.source_files.len(), merge.target_path);
}
```

### Symbol Migration Tracking

```rust
use smart_diff_engine::{SymbolMigrationTracker, SymbolMigrationTrackerConfig};
use smart_diff_semantic::SymbolResolver;

// Create symbol resolvers for source and target
let mut source_resolver = SymbolResolver::with_defaults();
let mut target_resolver = SymbolResolver::with_defaults();

// Process files
source_resolver.process_file("Calculator.java", &source_parse_result)?;
target_resolver.process_file("MathCalculator.java", &target_parse_result)?;

// Track migrations
let tracker = SymbolMigrationTracker::with_defaults();
let migration_result = tracker.track_migrations(&source_resolver, &target_resolver)?;

// Analyze migrations
for migration in &migration_result.symbol_migrations {
    println!("Symbol {} migrated from {} to {}", 
        migration.symbol_name, 
        migration.source_file, 
        migration.target_file
    );
}
```

### Custom Configuration

```rust
use smart_diff_engine::FileRefactoringDetectorConfig;

let config = FileRefactoringDetectorConfig {
    min_rename_similarity: 0.8,  // Higher threshold for renames
    min_split_similarity: 0.6,   // Higher threshold for splits
    min_merge_similarity: 0.6,   // Higher threshold for merges
    use_path_similarity: true,
    use_content_fingerprinting: true,
    use_symbol_migration: true,
    max_split_merge_candidates: 5,  // Limit candidates
};

let detector = FileRefactoringDetector::new(config);
```

## Algorithms

### Content Fingerprinting

The system uses multiple techniques to create content fingerprints:

1. **Content Hash**: Full content hash for exact matching
2. **Normalized Hash**: Hash of content with whitespace removed
3. **Identifier Set**: Set of unique identifiers (classes, functions, variables)
4. **Line Counts**: Total lines and non-empty lines

### Similarity Calculation

File similarity is calculated using weighted combination:

```
similarity = (identifier_similarity * 0.7) + (line_similarity * 0.3)
```

For rename detection:
```
combined_score = (content_sim * 0.6) + (path_sim * 0.2) + (symbol_migration * 0.2)
```

### Move Detection Algorithm

1. **Exact Match Phase**: Find files with identical content
2. **Fingerprint Match Phase**: Find files with matching normalized content
3. **Identifier Match Phase**: Find files with high identifier overlap
4. **Path Analysis Phase**: Determine if it's a move, rename, or both

### Split/Merge Detection

1. **Identifier Overlap**: Calculate Jaccard similarity of identifier sets
2. **Candidate Selection**: Find files with significant overlap
3. **Grouping**: Group candidates by similarity scores
4. **Confidence Scoring**: Calculate confidence based on overlap and patterns

## Configuration Options

### FileRefactoringDetectorConfig

| Option | Default | Description |
|--------|---------|-------------|
| `min_rename_similarity` | 0.7 | Minimum similarity for rename detection |
| `min_split_similarity` | 0.5 | Minimum similarity for split detection |
| `min_merge_similarity` | 0.5 | Minimum similarity for merge detection |
| `use_path_similarity` | true | Enable path similarity analysis |
| `use_content_fingerprinting` | true | Enable content fingerprinting |
| `use_symbol_migration` | true | Enable symbol migration tracking |
| `max_split_merge_candidates` | 10 | Maximum candidates for split/merge |

### SymbolMigrationTrackerConfig

| Option | Default | Description |
|--------|---------|-------------|
| `min_migration_threshold` | 0.3 | Minimum migration percentage |
| `track_functions` | true | Track function migrations |
| `track_classes` | true | Track class migrations |
| `track_variables` | false | Track variable migrations |
| `analyze_cross_file_references` | true | Analyze reference changes |

## Performance Considerations

### Scalability

- **File Count**: Optimized for up to 50 files per comparison
- **File Size**: Efficient fingerprinting for files up to 10,000 lines
- **Symbol Count**: Can handle thousands of symbols per file

### Optimization Techniques

1. **Early Termination**: Stop searching when high-confidence match found
2. **Fingerprint Caching**: Cache fingerprints to avoid recomputation
3. **Parallel Processing**: Use rayon for parallel file analysis
4. **Threshold Filtering**: Skip low-similarity candidates early

## Best Practices

### 1. Adjust Thresholds Based on Language

Different languages may require different similarity thresholds:
- **Verbose languages** (Java, C#): Higher thresholds (0.8+)
- **Concise languages** (Python, Ruby): Lower thresholds (0.6+)

### 2. Enable Symbol Migration for Accuracy

Symbol migration tracking significantly improves detection accuracy:
```rust
config.use_symbol_migration = true;
```

### 3. Use Path Similarity for Organized Codebases

If your codebase has consistent naming conventions:
```rust
config.use_path_similarity = true;
```

### 4. Limit Candidates for Performance

For large codebases, limit split/merge candidates:
```rust
config.max_split_merge_candidates = 5;
```

## Examples

See the following examples for detailed usage:
- `examples/enhanced_cross_file_detection_demo.rs` - Comprehensive demo
- `examples/cross_file_tracking_demo.rs` - Function-level tracking
- `examples/symbol_resolution_demo.rs` - Symbol resolution

## Integration with Existing Tools

### With CrossFileTracker

```rust
use smart_diff_engine::{CrossFileTracker, FileRefactoringDetector};

// Detect file-level refactorings first
let file_detector = FileRefactoringDetector::with_defaults();
let file_result = file_detector.detect_file_refactorings(&source_files, &target_files)?;

// Then detect function-level moves
let mut tracker = CrossFileTracker::with_defaults(Language::Java);
let function_result = tracker.track_cross_file_changes(&source_functions, &target_functions)?;

// Combine results for comprehensive analysis
```

### With SymbolResolver

```rust
// Build symbol tables
let mut source_resolver = SymbolResolver::with_defaults();
let mut target_resolver = SymbolResolver::with_defaults();

// Process all files
for (path, parse_result) in source_files {
    source_resolver.process_file(&path, &parse_result)?;
}

// Use with cross-file tracker
let mut tracker = CrossFileTracker::with_defaults(Language::Java);
tracker.set_symbol_resolver(source_resolver);
```

## Troubleshooting

### Low Detection Accuracy

If detection accuracy is low:
1. Lower similarity thresholds
2. Enable all detection features
3. Check if files have sufficient identifiers
4. Verify language-specific parsing is working

### False Positives

If getting too many false positives:
1. Increase similarity thresholds
2. Enable path similarity filtering
3. Increase `min_migration_threshold`
4. Reduce `max_split_merge_candidates`

### Performance Issues

If performance is slow:
1. Reduce `max_split_merge_candidates`
2. Disable symbol migration for initial pass
3. Filter files by extension before processing
4. Use parallel processing

## Future Enhancements

Planned improvements:
- Machine learning-based similarity scoring
- Language-specific refactoring patterns
- IDE integration for real-time detection
- Visualization of refactoring flows
- Automatic refactoring suggestion

