use smart_diff_parser::language::{Language, LanguageDetector};
use smart_diff_parser::tree_sitter::TreeSitterParser;
use smart_diff_parser::parser::Parser;

#[test]
fn test_go_language_detection() {
    let go_code = r#"
package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
"#;
    assert_eq!(LanguageDetector::detect_from_content(go_code), Language::Go);
}

#[test]
fn test_go_parsing() {
    let go_code = r#"
package main

func add(a int, b int) int {
    return a + b
}
"#;
    let parser = TreeSitterParser::new().expect("Failed to create parser");
    let result = parser.parse(go_code, Language::Go);
    assert!(result.is_ok(), "Failed to parse Go code: {:?}", result.err());

    let parse_result = result.unwrap();
    assert_eq!(parse_result.language, Language::Go);
    // Just verify parsing succeeded - detailed AST structure tests can be added later
    assert!(parse_result.ast.children.len() > 0);
}

#[test]
fn test_ruby_language_detection() {
    let ruby_code = r#"
class Calculator
  def add(a, b)
    a + b
  end
end
"#;
    assert_eq!(LanguageDetector::detect_from_content(ruby_code), Language::Ruby);
}

#[test]
fn test_ruby_parsing() {
    let ruby_code = r#"
def greet(name)
  puts "Hello, #{name}!"
end
"#;
    let parser = TreeSitterParser::new().expect("Failed to create parser");
    let result = parser.parse(ruby_code, Language::Ruby);
    assert!(result.is_ok(), "Failed to parse Ruby code: {:?}", result.err());

    let parse_result = result.unwrap();
    assert_eq!(parse_result.language, Language::Ruby);
    assert!(parse_result.ast.children.len() > 0);
}

#[test]
fn test_php_language_detection() {
    let php_code = r#"
<?php
class Calculator {
    public function add($a, $b) {
        return $a + $b;
    }
}
"#;
    assert_eq!(LanguageDetector::detect_from_content(php_code), Language::PHP);
}

#[test]
fn test_php_parsing() {
    let php_code = r#"
<?php
function add($a, $b) {
    return $a + $b;
}
"#;
    let parser = TreeSitterParser::new().expect("Failed to create parser");
    let result = parser.parse(php_code, Language::PHP);
    assert!(result.is_ok(), "Failed to parse PHP code: {:?}", result.err());

    let parse_result = result.unwrap();
    assert_eq!(parse_result.language, Language::PHP);
    assert!(parse_result.ast.children.len() > 0);
}

#[test]
fn test_swift_language_detection() {
    let swift_code = r#"
import Foundation

class Calculator {
    func add(_ a: Int, _ b: Int) -> Int {
        return a + b
    }
}
"#;
    assert_eq!(LanguageDetector::detect_from_content(swift_code), Language::Swift);
}

#[test]
fn test_swift_parsing() {
    let swift_code = r#"
func add(_ a: Int, _ b: Int) -> Int {
    return a + b
}
"#;
    let parser = TreeSitterParser::new().expect("Failed to create parser");
    let result = parser.parse(swift_code, Language::Swift);
    assert!(result.is_ok(), "Failed to parse Swift code: {:?}", result.err());

    let parse_result = result.unwrap();
    assert_eq!(parse_result.language, Language::Swift);
    assert!(parse_result.ast.children.len() > 0);
}

#[test]
fn test_go_file_extension() {
    assert_eq!(LanguageDetector::detect_from_path("test.go"), Language::Go);
}

#[test]
fn test_ruby_file_extensions() {
    assert_eq!(LanguageDetector::detect_from_path("test.rb"), Language::Ruby);
    assert_eq!(LanguageDetector::detect_from_path("test.rake"), Language::Ruby);
    assert_eq!(LanguageDetector::detect_from_path("test.gemspec"), Language::Ruby);
}

#[test]
fn test_php_file_extensions() {
    assert_eq!(LanguageDetector::detect_from_path("test.php"), Language::PHP);
    assert_eq!(LanguageDetector::detect_from_path("test.phtml"), Language::PHP);
}

#[test]
fn test_swift_file_extension() {
    assert_eq!(LanguageDetector::detect_from_path("test.swift"), Language::Swift);
}

