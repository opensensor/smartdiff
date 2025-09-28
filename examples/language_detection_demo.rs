//! Demonstration of the enhanced language detection capabilities

use smart_diff_parser::language::{Language, LanguageDetector};

fn main() {
    println!("Smart Code Diff - Language Detection Demo");
    println!("=========================================");

    // Test file extension detection
    demo_file_extension_detection();

    // Test content-based detection
    demo_content_detection();

    // Test combined detection
    demo_combined_detection();

    // Test edge cases
    demo_edge_cases();
}

fn demo_file_extension_detection() {
    println!("\n--- File Extension Detection ---");

    let test_files = vec![
        "Calculator.java",
        "script.py",
        "app.js",
        "main.cpp",
        "utils.c",
        "data.h",
        "component.jsx",
        "module.pyw",
        "unknown.xyz",
    ];

    for file in test_files {
        let detected = LanguageDetector::detect_from_path(file);
        println!("{:<15} -> {:?}", file, detected);
    }
}

fn demo_content_detection() {
    println!("\n--- Content-Based Detection ---");

    // Java example
    let java_code = r#"
public class Calculator {
    private int value;
    
    public Calculator() {
        this.value = 0;
    }
    
    public int add(int a, int b) {
        return a + b;
    }
    
    public static void main(String[] args) {
        Calculator calc = new Calculator();
        System.out.println("Result: " + calc.add(5, 3));
    }
}
"#;

    let detected = LanguageDetector::detect_from_content(java_code);
    println!("Java code detected as: {:?}", detected);

    // Python example
    let python_code = r#"
class Calculator:
    def __init__(self):
        self.value = 0
    
    def add(self, a, b):
        return a + b
    
    def main():
        calc = Calculator()
        result = calc.add(5, 3)
        print(f"Result: {result}")

if __name__ == "__main__":
    main()
"#;

    let detected = LanguageDetector::detect_from_content(python_code);
    println!("Python code detected as: {:?}", detected);

    // JavaScript example
    let js_code = r#"
class Calculator {
    constructor() {
        this.value = 0;
    }
    
    add(a, b) {
        return a + b;
    }
}

const calc = new Calculator();
const result = calc.add(5, 3);
console.log(`Result: ${result}`);

// Arrow function example
const multiply = (a, b) => a * b;
console.log(`Multiply: ${multiply(4, 6)}`);
"#;

    let detected = LanguageDetector::detect_from_content(js_code);
    println!("JavaScript code detected as: {:?}", detected);

    // C++ example
    let cpp_code = r#"
#include <iostream>
#include <vector>

class Calculator {
private:
    int value;

public:
    Calculator() : value(0) {}
    
    int add(int a, int b) {
        return a + b;
    }
};

int main() {
    Calculator calc;
    int result = calc.add(5, 3);
    std::cout << "Result: " << result << std::endl;
    
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    for (const auto& num : numbers) {
        std::cout << num << " ";
    }
    
    return 0;
}
"#;

    let detected = LanguageDetector::detect_from_content(cpp_code);
    println!("C++ code detected as: {:?}", detected);

    // C example
    let c_code = r#"
#include <stdio.h>
#include <stdlib.h>

struct Calculator {
    int value;
};

int add(int a, int b) {
    return a + b;
}

int main() {
    struct Calculator calc = {0};
    int result = add(5, 3);
    printf("Result: %d\n", result);
    
    int* numbers = malloc(5 * sizeof(int));
    for (int i = 0; i < 5; i++) {
        numbers[i] = i + 1;
        printf("%d ", numbers[i]);
    }
    printf("\n");
    
    free(numbers);
    return 0;
}
"#;

    let detected = LanguageDetector::detect_from_content(c_code);
    println!("C code detected as: {:?}", detected);
}

fn demo_combined_detection() {
    println!("\n--- Combined Detection (Path + Content) ---");

    // Test cases where extension and content might conflict
    let test_cases = vec![
        (
            "script.py",
            r#"
def hello():
    print("Hello from Python!")

if __name__ == "__main__":
    hello()
"#,
            "Clear Python content with .py extension",
        ),
        (
            "script.txt",
            r#"
def hello():
    print("Hello from Python!")

if __name__ == "__main__":
    hello()
"#,
            "Clear Python content with .txt extension",
        ),
        (
            "main.cpp",
            r#"
int main() {
    return 0;
}
"#,
            "Ambiguous C/C++ content with .cpp extension",
        ),
        (
            "main.c",
            r#"
int main() {
    return 0;
}
"#,
            "Ambiguous C/C++ content with .c extension",
        ),
        (
            "app.js",
            r#"
function greet(name) {
    console.log("Hello, " + name + "!");
}

const person = "World";
greet(person);
"#,
            "Clear JavaScript content with .js extension",
        ),
    ];

    for (filename, content, description) in test_cases {
        let detected = LanguageDetector::detect(filename, content);
        println!("{:<50} -> {:?}", description, detected);
    }
}

fn demo_edge_cases() {
    println!("\n--- Edge Cases ---");

    // Empty content
    let detected = LanguageDetector::detect_from_content("");
    println!("Empty content detected as: {:?}", detected);

    // Very short content
    let detected = LanguageDetector::detect_from_content("int x;");
    println!("Short C-like content detected as: {:?}", detected);

    // Mixed language content (should pick the strongest signal)
    let mixed_content = r#"
// This looks like C++
#include <iostream>

// But also has Python-like comments
def some_function():
    pass

// And JavaScript
console.log("Hello");

// But the C++ is strongest
int main() {
    std::cout << "Hello World" << std::endl;
    return 0;
}
"#;

    let detected = LanguageDetector::detect_from_content(mixed_content);
    println!("Mixed language content detected as: {:?}", detected);

    // Comments only
    let comments_only = r#"
// This is a comment
/* This is also a comment */
# This is a Python comment
"#;

    let detected = LanguageDetector::detect_from_content(comments_only);
    println!("Comments-only content detected as: {:?}", detected);
}
