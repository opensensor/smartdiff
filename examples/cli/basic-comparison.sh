#!/bin/bash
# Example: Basic File Comparison
# Purpose: Demonstrates basic file comparison functionality with different output formats
# Usage: ./basic-comparison.sh

set -e

echo "Smart Code Diff Example: Basic File Comparison"
echo "=============================================="

# Check if smart-diff-cli is available
if ! command -v smart-diff-cli &> /dev/null; then
    echo "Error: smart-diff-cli not found in PATH"
    echo "Please build the project and add target/release to your PATH"
    exit 1
fi

# Create sample directory
SAMPLE_DIR="$(dirname "$0")/../sample-code/java"
mkdir -p "$SAMPLE_DIR"

# Create sample Java files for comparison
cat > "$SAMPLE_DIR/Calculator.java" << 'EOF'
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
    
    public int multiply(int a, int b) {
        return a * b;
    }
    
    public boolean isEven(int number) {
        return number % 2 == 0;
    }
    
    public double divide(double a, double b) {
        if (b == 0) {
            throw new IllegalArgumentException("Division by zero");
        }
        return a / b;
    }
}
EOF

cat > "$SAMPLE_DIR/Calculator_refactored.java" << 'EOF'
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
    
    public int multiply(int a, int b) {
        return a * b;
    }
    
    // Renamed method with extracted logic
    public boolean isNumberEven(int number) {
        return checkEvenness(number);
    }
    
    // Extracted method
    private boolean checkEvenness(int number) {
        return number % 2 == 0;
    }
    
    public double divide(double a, double b) {
        if (b == 0) {
            throw new ArithmeticException("Cannot divide by zero");
        }
        return a / b;
    }
    
    // New method
    public int subtract(int a, int b) {
        return a - b;
    }
}
EOF

echo "Created sample Java files:"
echo "  - $SAMPLE_DIR/Calculator.java"
echo "  - $SAMPLE_DIR/Calculator_refactored.java"
echo

# Example 1: Basic comparison with default settings
echo "Example 1: Basic Comparison (Text Output)"
echo "----------------------------------------"
smart-diff-cli compare "$SAMPLE_DIR/Calculator.java" "$SAMPLE_DIR/Calculator_refactored.java"
echo

# Example 2: JSON output for programmatic processing
echo "Example 2: JSON Output"
echo "---------------------"
smart-diff-cli compare --output json "$SAMPLE_DIR/Calculator.java" "$SAMPLE_DIR/Calculator_refactored.java" | jq '.similarity'
echo

# Example 3: Custom similarity threshold
echo "Example 3: Custom Similarity Threshold (0.8)"
echo "--------------------------------------------"
smart-diff-cli compare --threshold 0.8 "$SAMPLE_DIR/Calculator.java" "$SAMPLE_DIR/Calculator_refactored.java"
echo

# Example 4: Ignore whitespace changes
echo "Example 4: Ignoring Whitespace Changes"
echo "-------------------------------------"
# Create a version with different formatting
cat > "$SAMPLE_DIR/Calculator_formatted.java" << 'EOF'
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }


    public int multiply(int a, int b) {
        return a * b;
    }


    public boolean isEven(int number) {
        return number % 2 == 0;
    }


    public double divide(double a, double b) {
        if (b == 0) {
            throw new IllegalArgumentException("Division by zero");
        }
        return a / b;
    }
}
EOF

echo "Without --ignore-whitespace:"
smart-diff-cli compare "$SAMPLE_DIR/Calculator.java" "$SAMPLE_DIR/Calculator_formatted.java" | head -5

echo
echo "With --ignore-whitespace:"
smart-diff-cli compare --ignore-whitespace "$SAMPLE_DIR/Calculator.java" "$SAMPLE_DIR/Calculator_formatted.java" | head -5
echo

# Example 5: HTML output for visual inspection
echo "Example 5: HTML Output Generation"
echo "--------------------------------"
OUTPUT_DIR="$(dirname "$0")/output"
mkdir -p "$OUTPUT_DIR"

smart-diff-cli compare --output html "$SAMPLE_DIR/Calculator.java" "$SAMPLE_DIR/Calculator_refactored.java" > "$OUTPUT_DIR/comparison.html"
echo "HTML report generated: $OUTPUT_DIR/comparison.html"
echo "Open this file in a web browser to view the interactive comparison"
echo

# Example 6: Force language detection
echo "Example 6: Force Language Detection"
echo "----------------------------------"
# Create a file with wrong extension
cp "$SAMPLE_DIR/Calculator.java" "$SAMPLE_DIR/Calculator.txt"

echo "Comparing .txt file with forced Java language detection:"
smart-diff-cli compare --language java "$SAMPLE_DIR/Calculator.txt" "$SAMPLE_DIR/Calculator_refactored.java" | head -5
echo

# Example 7: Verbose output with timing
echo "Example 7: Verbose Output with Timing"
echo "------------------------------------"
time smart-diff-cli compare --output json "$SAMPLE_DIR/Calculator.java" "$SAMPLE_DIR/Calculator_refactored.java" | jq '.execution_time_ms'
echo

# Cleanup
echo "Cleaning up temporary files..."
rm -f "$SAMPLE_DIR/Calculator_formatted.java" "$SAMPLE_DIR/Calculator.txt"

echo "Basic comparison examples completed successfully!"
echo
echo "Key takeaways:"
echo "1. Smart Code Diff detects structural changes, not just text differences"
echo "2. Function renaming and extraction are identified as refactoring patterns"
echo "3. Different output formats serve different use cases"
echo "4. Similarity thresholds can be adjusted based on requirements"
echo "5. Language detection can be forced when file extensions are misleading"
echo
echo "Next steps:"
echo "- Try the directory comparison example: ./directory-analysis.sh"
echo "- Explore the web interface at http://localhost:3000 (after starting smart-diff-server)"
echo "- Check the API examples in ../api/ directory"
