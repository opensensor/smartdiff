//! CLI Demonstration Example
//! 
//! This example showcases the comprehensive command-line interface for the Smart Code Diffing Tool,
//! demonstrating all major features including file comparison, code analysis, configuration management,
//! and system diagnostics with multiple output formats and advanced options.

use std::fs;
use std::path::Path;
use tempfile::TempDir;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🖥️  Smart Code Diff CLI Demonstration");
    println!("=====================================\n");

    // Create temporary test files for demonstration
    let temp_dir = create_test_files()?;
    let temp_path = temp_dir.path();

    println!("📁 Created test files in: {}\n", temp_path.display());

    // Demo 1: Basic file comparison
    demo_basic_comparison(temp_path)?;
    
    // Demo 2: Directory comparison with options
    demo_directory_comparison(temp_path)?;
    
    // Demo 3: Different output formats
    demo_output_formats(temp_path)?;
    
    // Demo 4: Code analysis
    demo_code_analysis(temp_path)?;
    
    // Demo 5: Configuration management
    demo_configuration_management()?;
    
    // Demo 6: System diagnostics
    demo_system_diagnostics()?;
    
    // Demo 7: Advanced features
    demo_advanced_features(temp_path)?;

    println!("\n✅ CLI Demonstration Complete!");
    println!("\n📖 Usage Examples:");
    print_usage_examples();

    Ok(())
}

/// Create test files for demonstration
fn create_test_files() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    // Create source files
    let src_dir = base_path.join("src");
    fs::create_dir_all(&src_dir)?;

    // Original Calculator.java
    fs::write(src_dir.join("Calculator.java"), r#"
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
}
"#)?;

    // Modified Calculator.java (with refactoring)
    let modified_dir = base_path.join("modified");
    fs::create_dir_all(&modified_dir)?;
    
    fs::write(modified_dir.join("Calculator.java"), r#"
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
    
    public int multiply(int a, int b) {
        return a * b;
    }
    
    // Renamed method
    public boolean isNumberEven(int number) {
        return checkEvenness(number);
    }
    
    // Extracted method
    private boolean checkEvenness(int number) {
        return number % 2 == 0;
    }
    
    // New method
    public int subtract(int a, int b) {
        return a - b;
    }
}
"#)?;

    // Python example
    fs::write(src_dir.join("utils.py"), r#"
def calculate_area(length, width):
    return length * width

def format_name(first, last):
    return f"{first} {last}"

class DataProcessor:
    def __init__(self):
        self.data = []
    
    def add_item(self, item):
        self.data.append(item)
    
    def process_data(self):
        return [item.upper() for item in self.data]
"#)?;

    fs::write(modified_dir.join("utils.py"), r#"
def calculate_area(length, width):
    """Calculate the area of a rectangle."""
    return length * width

def calculate_perimeter(length, width):
    """Calculate the perimeter of a rectangle."""
    return 2 * (length + width)

def format_full_name(first_name, last_name):
    """Format a full name from first and last name."""
    return f"{first_name} {last_name}"

class DataProcessor:
    def __init__(self):
        self.data = []
        self.processed = False
    
    def add_item(self, item):
        if item:
            self.data.append(item)
    
    def process_data(self):
        self.processed = True
        return [self._transform_item(item) for item in self.data]
    
    def _transform_item(self, item):
        return item.upper().strip()
"#)?;

    // JavaScript example
    fs::write(src_dir.join("app.js"), r#"
function validateEmail(email) {
    const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return regex.test(email);
}

function processUser(userData) {
    if (!userData.email || !validateEmail(userData.email)) {
        throw new Error('Invalid email');
    }
    
    return {
        id: userData.id,
        name: userData.name,
        email: userData.email.toLowerCase()
    };
}
"#)?;

    fs::write(modified_dir.join("app.js"), r#"
function validateEmail(email) {
    const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return regex.test(email);
}

function validateUserData(userData) {
    if (!userData.email || !validateEmail(userData.email)) {
        throw new Error('Invalid email address');
    }
    
    if (!userData.name || userData.name.trim().length === 0) {
        throw new Error('Name is required');
    }
}

function processUser(userData) {
    validateUserData(userData);
    
    return {
        id: userData.id || generateId(),
        name: userData.name.trim(),
        email: userData.email.toLowerCase(),
        createdAt: new Date().toISOString()
    };
}

function generateId() {
    return Math.random().toString(36).substr(2, 9);
}
"#)?;

    Ok(temp_dir)
}

/// Demo 1: Basic file comparison
fn demo_basic_comparison(temp_path: &Path) -> Result<()> {
    println!("🔍 Demo 1: Basic File Comparison");
    println!("-".repeat(40));
    
    let source_file = temp_path.join("src/Calculator.java");
    let target_file = temp_path.join("modified/Calculator.java");
    
    println!("Command:");
    println!("  smart-diff compare {} {}", source_file.display(), target_file.display());
    println!();
    
    println!("Expected Output:");
    println!("  Smart Code Diff - Structural Code Comparison");
    println!("  Language: Java");
    println!("  Similarity: 75.3%");
    println!("  Changes: 4");
    println!();
    println!("  Changes Detected");
    println!("  ----------------");
    println!("  1. RENAME: Method 'isEven' renamed to 'isNumberEven'");
    println!("  2. ADD: New method 'checkEvenness' added");
    println!("  3. ADD: New method 'subtract' added");
    println!("  4. MODIFY: Method 'isNumberEven' body changed");
    println!();
    println!("  Refactoring Patterns");
    println!("  -------------------");
    println!("  1. ExtractMethod (confidence: 0.85)");
    println!("     Description: Extracted method 'checkEvenness' from 'isNumberEven'");
    println!("     Complexity: Simple");
    println!();

    Ok(())
}

/// Demo 2: Directory comparison with options
fn demo_directory_comparison(temp_path: &Path) -> Result<()> {
    println!("📂 Demo 2: Directory Comparison with Options");
    println!("-".repeat(50));
    
    let src_dir = temp_path.join("src");
    let modified_dir = temp_path.join("modified");
    
    println!("Command:");
    println!("  smart-diff compare {} {} \\", src_dir.display(), modified_dir.display());
    println!("    --recursive \\");
    println!("    --detect-refactoring \\");
    println!("    --track-moves \\");
    println!("    --show-similarity \\");
    println!("    --threshold 0.6 \\");
    println!("    --format json \\");
    println!("    --output comparison_results.json");
    println!();
    
    println!("Features Demonstrated:");
    println!("  ✓ Recursive directory comparison");
    println!("  ✓ Refactoring pattern detection");
    println!("  ✓ Cross-file move tracking");
    println!("  ✓ Function similarity scoring");
    println!("  ✓ Custom similarity threshold");
    println!("  ✓ JSON output format");
    println!("  ✓ Output to file");
    println!();

    Ok(())
}

/// Demo 3: Different output formats
fn demo_output_formats(temp_path: &Path) -> Result<()> {
    println!("📄 Demo 3: Multiple Output Formats");
    println!("-".repeat(40));
    
    let source_file = temp_path.join("src/utils.py");
    let target_file = temp_path.join("modified/utils.py");
    
    let formats = vec![
        ("text", "Human-readable colored text"),
        ("json", "Structured JSON for APIs"),
        ("html", "Web-friendly HTML with CSS"),
        ("xml", "Structured XML format"),
        ("csv", "Tabular CSV format"),
        ("markdown", "Documentation-friendly Markdown"),
    ];
    
    for (format, description) in formats {
        println!("Format: {} - {}", format, description);
        println!("  smart-diff compare {} {} --format {}", 
            source_file.display(), target_file.display(), format);
    }
    println!();

    Ok(())
}

/// Demo 4: Code analysis
fn demo_code_analysis(temp_path: &Path) -> Result<()> {
    println!("🔬 Demo 4: Code Analysis");
    println!("-".repeat(30));
    
    let src_dir = temp_path.join("src");
    
    println!("Command:");
    println!("  smart-diff analyze {} \\", src_dir.display());
    println!("    --recursive \\");
    println!("    --complexity \\");
    println!("    --dependencies \\");
    println!("    --signatures \\");
    println!("    --format markdown \\");
    println!("    --output analysis_report.md");
    println!();
    
    println!("Analysis Features:");
    println!("  ✓ Code complexity metrics");
    println!("  ✓ Dependency analysis");
    println!("  ✓ Function signature extraction");
    println!("  ✓ Multi-language support");
    println!("  ✓ Recursive directory analysis");
    println!();

    Ok(())
}

/// Demo 5: Configuration management
fn demo_configuration_management() -> Result<()> {
    println!("⚙️  Demo 5: Configuration Management");
    println!("-".repeat(40));
    
    println!("Available Commands:");
    println!("  smart-diff config show                    # Show all configuration");
    println!("  smart-diff config show --section parser  # Show parser config");
    println!("  smart-diff config list                    # List all config keys");
    println!("  smart-diff config get parser.max_file_size");
    println!("  smart-diff config set parser.max_file_size 20971520");
    println!("  smart-diff config validate               # Validate configuration");
    println!("  smart-diff config reset                  # Reset to defaults");
    println!();
    
    println!("Configuration Sections:");
    println!("  • parser      - File parsing settings");
    println!("  • semantic    - Semantic analysis settings");
    println!("  • diff_engine - Comparison engine settings");
    println!("  • output      - Output formatting settings");
    println!("  • cli         - CLI behavior settings");
    println!();

    Ok(())
}

/// Demo 6: System diagnostics
fn demo_system_diagnostics() -> Result<()> {
    println!("🏥 Demo 6: System Diagnostics");
    println!("-".repeat(35));
    
    println!("Commands:");
    println!("  smart-diff doctor                    # Full system check");
    println!("  smart-diff doctor --component parser # Check parser only");
    println!("  smart-diff doctor --fix              # Auto-fix issues");
    println!();
    
    println!("Diagnostic Components:");
    println!("  ✓ Parser system functionality");
    println!("  ✓ Semantic analysis system");
    println!("  ✓ Diff engine components");
    println!("  ✓ Language support verification");
    println!("  ✓ Configuration validation");
    println!();

    Ok(())
}

/// Demo 7: Advanced features
fn demo_advanced_features(temp_path: &Path) -> Result<()> {
    println!("🚀 Demo 7: Advanced Features");
    println!("-".repeat(35));
    
    let src_dir = temp_path.join("src");
    let modified_dir = temp_path.join("modified");
    
    println!("Advanced Comparison:");
    println!("  smart-diff compare {} {} \\", src_dir.display(), modified_dir.display());
    println!("    --recursive \\");
    println!("    --detect-refactoring \\");
    println!("    --track-moves \\");
    println!("    --show-similarity \\");
    println!("    --include-ast \\");
    println!("    --max-depth 15 \\");
    println!("    --show-stats \\");
    println!("    --include '*.java,*.py,*.js' \\");
    println!("    --exclude 'test/**,*.min.js' \\");
    println!("    --language auto \\");
    println!("    --verbose");
    println!();
    
    println!("Advanced Features:");
    println!("  ✓ AST inclusion in output");
    println!("  ✓ Configurable comparison depth");
    println!("  ✓ Performance statistics");
    println!("  ✓ File pattern filtering");
    println!("  ✓ Language override");
    println!("  ✓ Verbose logging");
    println!("  ✓ Progress indicators");
    println!("  ✓ Colored output control");
    println!();

    Ok(())
}

/// Print comprehensive usage examples
fn print_usage_examples() {
    println!("# Basic Usage");
    println!("smart-diff compare file1.java file2.java");
    println!("smart-diff compare src/ modified/ --recursive");
    println!();
    
    println!("# Output Formats");
    println!("smart-diff compare file1.py file2.py --format json");
    println!("smart-diff compare src/ dst/ --format html --output report.html");
    println!();
    
    println!("# Advanced Comparison");
    println!("smart-diff compare src/ dst/ --detect-refactoring --track-moves");
    println!("smart-diff compare file1.js file2.js --show-similarity --threshold 0.8");
    println!();
    
    println!("# Code Analysis");
    println!("smart-diff analyze src/ --recursive --complexity --dependencies");
    println!("smart-diff analyze file.java --signatures --format json");
    println!();
    
    println!("# Configuration");
    println!("smart-diff config show");
    println!("smart-diff config set diff_engine.default_similarity_threshold 0.8");
    println!();
    
    println!("# System Diagnostics");
    println!("smart-diff doctor");
    println!("smart-diff doctor --component parser --fix");
    println!();
    
    println!("# Global Options");
    println!("smart-diff --verbose compare file1.java file2.java");
    println!("smart-diff --quiet --no-color compare src/ dst/ --format json");
    println!("smart-diff --config custom-config.toml compare file1.py file2.py");
}
