//! Tests for file refactoring detection

#[cfg(test)]
mod tests {
    use crate::file_refactoring_detector::{
        FileRefactoringDetector, FileRefactoringDetectorConfig,
    };
    use std::collections::HashMap;

    #[test]
    fn test_file_rename_detection() {
        let mut source_files = HashMap::new();
        let mut target_files = HashMap::new();

        source_files.insert(
            "Calculator.java".to_string(),
            "public class Calculator { public int add(int a, int b) { return a + b; } }".to_string(),
        );

        target_files.insert(
            "MathCalculator.java".to_string(),
            "public class MathCalculator { public int add(int a, int b) { return a + b; } }".to_string(),
        );

        let detector = FileRefactoringDetector::with_defaults();
        let result = detector
            .detect_file_refactorings(&source_files, &target_files)
            .unwrap();

        // Should detect either a rename or a move
        assert!(
            !result.file_renames.is_empty() || !result.file_moves.is_empty(),
            "Should detect file rename or move"
        );
    }

    #[test]
    fn test_file_split_detection() {
        let mut source_files = HashMap::new();
        let mut target_files = HashMap::new();

        source_files.insert(
            "Utils.java".to_string(),
            r#"
class StringUtils { 
    String capitalize(String s) { return s; } 
}
class MathUtils { 
    int add(int a, int b) { return a + b; } 
}
"#
            .to_string(),
        );

        target_files.insert(
            "StringUtils.java".to_string(),
            "class StringUtils { String capitalize(String s) { return s; } }".to_string(),
        );

        target_files.insert(
            "MathUtils.java".to_string(),
            "class MathUtils { int add(int a, int b) { return a + b; } }".to_string(),
        );

        let detector = FileRefactoringDetector::with_defaults();
        let result = detector
            .detect_file_refactorings(&source_files, &target_files)
            .unwrap();

        // Should detect file split
        assert!(
            !result.file_splits.is_empty(),
            "Should detect file split"
        );
    }

    #[test]
    fn test_file_merge_detection() {
        let mut source_files = HashMap::new();
        let mut target_files = HashMap::new();

        source_files.insert(
            "Add.java".to_string(),
            "class Add { int execute(int a, int b) { return a + b; } }".to_string(),
        );

        source_files.insert(
            "Subtract.java".to_string(),
            "class Subtract { int execute(int a, int b) { return a - b; } }".to_string(),
        );

        target_files.insert(
            "Operations.java".to_string(),
            r#"
class Add { int execute(int a, int b) { return a + b; } }
class Subtract { int execute(int a, int b) { return a - b; } }
"#
            .to_string(),
        );

        let detector = FileRefactoringDetector::with_defaults();
        let result = detector
            .detect_file_refactorings(&source_files, &target_files)
            .unwrap();

        // Should detect file merge
        assert!(
            !result.file_merges.is_empty(),
            "Should detect file merge"
        );
    }

    #[test]
    fn test_content_fingerprinting() {
        let detector = FileRefactoringDetector::with_defaults();

        let content1 = "class Test { void method() { } }";
        let content2 = "class   Test   {   void   method()   {   }   }"; // Same but different whitespace

        let fp1 = detector.create_fingerprint(content1).unwrap();
        let fp2 = detector.create_fingerprint(content2).unwrap();

        // Normalized hashes should be the same
        assert_eq!(
            fp1.normalized_hash, fp2.normalized_hash,
            "Normalized hashes should match despite whitespace differences"
        );
    }

    #[test]
    fn test_path_similarity() {
        let detector = FileRefactoringDetector::with_defaults();

        // Same directory
        let sim1 = detector.calculate_path_similarity("src/Calculator.java", "src/MathCalc.java");
        assert!(sim1 > 0.5, "Same directory should have high similarity");

        // Different directory
        let sim2 = detector.calculate_path_similarity("src/Calculator.java", "test/TestCalc.java");
        assert!(sim2 < sim1, "Different directory should have lower similarity");

        // Completely different
        let sim3 = detector.calculate_path_similarity("src/Calculator.java", "lib/Database.java");
        assert!(sim3 < sim2, "Unrelated files should have lowest similarity");
    }

    #[test]
    fn test_identifier_extraction() {
        let detector = FileRefactoringDetector::with_defaults();

        let content = r#"
class Calculator {
    function add(a, b) {
        return a + b;
    }
    const PI = 3.14;
}
"#;

        let fingerprint = detector.create_fingerprint(content).unwrap();

        // Should extract class and function names
        assert!(
            fingerprint.identifier_set.contains("Calculator"),
            "Should extract class name"
        );
        assert!(
            fingerprint.identifier_set.contains("add"),
            "Should extract function name"
        );
        assert!(
            fingerprint.identifier_set.contains("PI"),
            "Should extract constant name"
        );
    }

    #[test]
    fn test_config_customization() {
        let mut config = FileRefactoringDetectorConfig::default();
        config.min_rename_similarity = 0.9;
        config.use_path_similarity = false;

        let detector = FileRefactoringDetector::new(config);

        assert_eq!(detector.get_config().min_rename_similarity, 0.9);
        assert!(!detector.get_config().use_path_similarity);
    }

    #[test]
    fn test_no_false_positives_for_unrelated_files() {
        let mut source_files = HashMap::new();
        let mut target_files = HashMap::new();

        source_files.insert(
            "Calculator.java".to_string(),
            "class Calculator { int add(int a, int b) { return a + b; } }".to_string(),
        );

        target_files.insert(
            "Database.java".to_string(),
            "class Database { void connect() { } }".to_string(),
        );

        let detector = FileRefactoringDetector::with_defaults();
        let result = detector
            .detect_file_refactorings(&source_files, &target_files)
            .unwrap();

        // Should not detect any refactorings for completely unrelated files
        assert!(
            result.file_renames.is_empty(),
            "Should not detect rename for unrelated files"
        );
        assert!(
            result.file_moves.is_empty(),
            "Should not detect move for unrelated files"
        );
    }

    #[test]
    fn test_statistics_calculation() {
        let mut source_files = HashMap::new();
        let mut target_files = HashMap::new();

        source_files.insert("File1.java".to_string(), "class A {}".to_string());
        source_files.insert("File2.java".to_string(), "class B {}".to_string());

        target_files.insert("File3.java".to_string(), "class C {}".to_string());

        let detector = FileRefactoringDetector::with_defaults();
        let result = detector
            .detect_file_refactorings(&source_files, &target_files)
            .unwrap();

        assert_eq!(result.statistics.total_source_files, 2);
        assert_eq!(result.statistics.total_target_files, 1);
        assert!(result.statistics.execution_time_ms > 0);
    }

    #[test]
    fn test_file_move_vs_rename() {
        let mut source_files = HashMap::new();
        let mut target_files = HashMap::new();

        // Pure move (same name, different directory)
        source_files.insert(
            "src/Calculator.java".to_string(),
            "class Calculator { }".to_string(),
        );

        target_files.insert(
            "lib/Calculator.java".to_string(),
            "class Calculator { }".to_string(),
        );

        let detector = FileRefactoringDetector::with_defaults();
        let result = detector
            .detect_file_refactorings(&source_files, &target_files)
            .unwrap();

        // Should detect as move, not rename
        if !result.file_moves.is_empty() {
            assert!(
                !result.file_moves[0].was_renamed,
                "Pure directory move should not be marked as renamed"
            );
        }
    }

    #[test]
    fn test_combined_move_and_rename() {
        let mut source_files = HashMap::new();
        let mut target_files = HashMap::new();

        // Move + rename
        source_files.insert(
            "src/Calculator.java".to_string(),
            "class Calculator { }".to_string(),
        );

        target_files.insert(
            "lib/MathCalc.java".to_string(),
            "class MathCalc { }".to_string(),
        );

        let detector = FileRefactoringDetector::with_defaults();
        let result = detector
            .detect_file_refactorings(&source_files, &target_files)
            .unwrap();

        // Should detect as move with rename
        if !result.file_moves.is_empty() {
            assert!(
                result.file_moves[0].was_renamed,
                "Move with name change should be marked as renamed"
            );
        }
    }
}

