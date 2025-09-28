//! Demonstration of comprehensive function signature extraction capabilities

use smart_diff_parser::{TreeSitterParser, Language};
use smart_diff_semantic::{
    FunctionSignatureExtractor, FunctionSignatureConfig,
    FunctionType, GenericVariance, Visibility
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Smart Code Diff - Function Signature Extraction Demo");
    println!("===================================================");
    
    // Demo basic function signature extraction
    demo_basic_signature_extraction()?;
    
    // Demo advanced signature analysis
    demo_advanced_signature_analysis()?;
    
    // Demo function similarity comparison
    demo_function_similarity()?;
    
    // Demo overload detection
    demo_overload_detection()?;
    
    // Demo cross-language signature handling
    demo_cross_language_signatures()?;
    
    Ok(())
}

fn demo_basic_signature_extraction() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Basic Function Signature Extraction ---");
    
    let parser = TreeSitterParser::new()?;
    let mut extractor = FunctionSignatureExtractor::with_defaults(Language::Java);
    
    let java_code = r#"
package com.example.service;

import java.util.List;
import java.util.Optional;

public class UserService {
    private final UserRepository repository;
    
    public UserService(UserRepository repository) {
        this.repository = repository;
    }
    
    public User createUser(String name, String email) {
        validateInput(name, email);
        User user = new User(name, email);
        return repository.save(user);
    }
    
    public Optional<User> findUserById(Long id) {
        if (id == null || id <= 0) {
            return Optional.empty();
        }
        return repository.findById(id);
    }
    
    public List<User> findUsersByEmail(String email) {
        return repository.findByEmail(email);
    }
    
    private void validateInput(String name, String email) {
        if (name == null || name.trim().isEmpty()) {
            throw new IllegalArgumentException("Name cannot be empty");
        }
        if (email == null || !email.contains("@")) {
            throw new IllegalArgumentException("Invalid email");
        }
    }
    
    public static UserService getInstance() {
        return new UserService(new DefaultUserRepository());
    }
    
    @Override
    public String toString() {
        return "UserService{repository=" + repository + "}";
    }
}
"#;
    
    let parse_result = parser.parse(java_code, Language::Java)?;
    let extraction_result = extractor.extract_signatures("UserService.java", &parse_result)?;
    
    println!("Extracted {} function signatures:", extraction_result.signatures.len());
    
    for signature in &extraction_result.signatures {
        println!("\n  Function: {} ({})", signature.name, format!("{:?}", signature.function_type));
        println!("    Qualified name: {}", signature.qualified_name);
        println!("    Visibility: {:?}", signature.visibility);
        
        if !signature.parameters.is_empty() {
            println!("    Parameters: {}", signature.parameters.len());
            for (i, param) in signature.parameters.iter().enumerate() {
                println!("      {}. {}: {} {}", 
                         i + 1, 
                         param.name, 
                         param.param_type.to_string(),
                         if param.is_optional { "(optional)" } else { "" });
            }
        }
        
        println!("    Return type: {}", signature.return_type.to_string());
        
        if !signature.modifiers.is_empty() {
            println!("    Modifiers: {}", signature.modifiers.join(", "));
        }
        
        if !signature.annotations.is_empty() {
            println!("    Annotations: {}", signature.annotations.join(", "));
        }
        
        if let Some(metrics) = &signature.complexity_metrics {
            println!("    Complexity:");
            println!("      Cyclomatic: {}", metrics.cyclomatic_complexity);
            println!("      Cognitive: {}", metrics.cognitive_complexity);
            println!("      Lines of code: {}", metrics.lines_of_code);
            println!("      Nesting depth: {}", metrics.nesting_depth);
        }
        
        if !signature.dependencies.is_empty() {
            println!("    Dependencies: {}", signature.dependencies.join(", "));
        }
        
        println!("    Location: {}:{}-{}", signature.file_path, signature.line, signature.end_line);
    }
    
    // Show extraction statistics
    let stats = &extraction_result.extraction_stats;
    println!("\nExtraction Statistics:");
    println!("  Total functions: {}", stats.total_functions);
    println!("  Public functions: {}", stats.public_functions);
    println!("  Private functions: {}", stats.private_functions);
    println!("  Static functions: {}", stats.static_functions);
    println!("  Constructors: {}", stats.constructors);
    println!("  Overloaded functions: {}", stats.overloaded_functions);
    println!("  Generic functions: {}", stats.generic_functions);
    println!("  Complex functions: {}", stats.complex_functions);
    
    Ok(())
}

fn demo_advanced_signature_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Advanced Function Signature Analysis ---");
    
    let parser = TreeSitterParser::new()?;
    let mut extractor = FunctionSignatureExtractor::with_defaults(Language::Java);
    
    let complex_code = r#"
public abstract class DataProcessor<T extends Comparable<T>, R> {
    
    @SafeVarargs
    public static <E> List<E> createList(E... elements) {
        List<E> list = new ArrayList<>();
        for (E element : elements) {
            if (element != null) {
                list.add(element);
            }
        }
        return list;
    }
    
    public abstract <U extends T> ProcessResult<R> process(
        @NotNull U input,
        @Nullable ProcessOptions options,
        Consumer<String> progressCallback
    ) throws ProcessingException;
    
    protected final synchronized R processWithRetry(
        T input, 
        int maxRetries,
        Duration timeout
    ) {
        for (int attempt = 1; attempt <= maxRetries; attempt++) {
            try {
                ProcessResult<R> result = process(input, null, null);
                if (result.isSuccess()) {
                    return result.getData();
                }
                
                if (attempt < maxRetries) {
                    Thread.sleep(timeout.toMillis() * attempt);
                }
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                throw new ProcessingException("Processing interrupted", e);
            } catch (Exception e) {
                if (attempt == maxRetries) {
                    throw new ProcessingException("Max retries exceeded", e);
                }
            }
        }
        
        throw new ProcessingException("Processing failed after " + maxRetries + " attempts");
    }
    
    public Optional<R> processOptional(T input) {
        try {
            R result = processWithRetry(input, 3, Duration.ofSeconds(1));
            return Optional.ofNullable(result);
        } catch (ProcessingException e) {
            return Optional.empty();
        }
    }
    
    // Overloaded methods
    public void configure(String config) {
        configure(config, true);
    }
    
    public void configure(String config, boolean validate) {
        configure(config, validate, Duration.ofMinutes(5));
    }
    
    public void configure(String config, boolean validate, Duration timeout) {
        // Implementation details...
    }
}
"#;
    
    let parse_result = parser.parse(complex_code, Language::Java)?;
    let extraction_result = extractor.extract_signatures("DataProcessor.java", &parse_result)?;
    
    println!("Advanced signature analysis results:");
    
    for signature in &extraction_result.signatures {
        println!("\n  {} ({})", signature.name, format!("{:?}", signature.function_type));
        
        // Show generic parameters
        if !signature.generic_parameters.is_empty() {
            println!("    Generic parameters:");
            for generic in &signature.generic_parameters {
                println!("      {}: {:?} with {} bounds", 
                         generic.name, 
                         generic.variance,
                         generic.bounds.len());
                for bound in &generic.bounds {
                    println!("        extends {}", bound.to_string());
                }
            }
        }
        
        // Show parameter details
        if !signature.parameters.is_empty() {
            println!("    Parameters:");
            for param in &signature.parameters {
                let flags = vec![
                    if param.is_optional { "optional" } else { "" },
                    if param.is_varargs { "varargs" } else { "" },
                ].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(", ");
                
                println!("      {}: {} {}", 
                         param.name, 
                         param.param_type.to_string(),
                         if flags.is_empty() { "" } else { &format!("({})", flags) });
                
                if !param.annotations.is_empty() {
                    println!("        Annotations: {}", param.annotations.join(", "));
                }
            }
        }
        
        // Show complexity metrics
        if let Some(metrics) = &signature.complexity_metrics {
            println!("    Complexity Analysis:");
            println!("      Cyclomatic complexity: {}", metrics.cyclomatic_complexity);
            println!("      Cognitive complexity: {}", metrics.cognitive_complexity);
            println!("      Lines of code: {}", metrics.lines_of_code);
            println!("      Nesting depth: {}", metrics.nesting_depth);
            println!("      Branch count: {}", metrics.branch_count);
            println!("      Loop count: {}", metrics.loop_count);
            println!("      Function calls: {}", metrics.call_count);
            
            // Complexity assessment
            let complexity_level = if metrics.cyclomatic_complexity <= 5 {
                "Low"
            } else if metrics.cyclomatic_complexity <= 10 {
                "Medium"
            } else {
                "High"
            };
            println!("      Complexity level: {}", complexity_level);
        }
        
        // Show signature hashes
        println!("    Signature hash: {}", &signature.signature_hash[..8]);
        println!("    Normalized hash: {}", &signature.normalized_hash[..8]);
    }
    
    // Show overloaded functions
    if !extraction_result.overloaded_functions.is_empty() {
        println!("\nOverloaded Functions:");
        for (name, overloads) in &extraction_result.overloaded_functions {
            if overloads.len() > 1 {
                println!("  {}: {} overloads", name, overloads.len());
                for (i, overload) in overloads.iter().enumerate() {
                    println!("    {}. {} parameters", i + 1, overload.parameters.len());
                }
            }
        }
    }
    
    Ok(())
}

fn demo_function_similarity() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Function Similarity Comparison ---");
    
    // This would demonstrate comparing functions from different files
    // For brevity, we'll show the concept
    
    println!("Function similarity analysis compares:");
    println!("  - Name similarity (40% weight)");
    println!("    - Exact matches: 1.0");
    println!("    - Normalized matches (case, underscores): 0.95");
    println!("    - Edit distance based: 0.0-0.9");
    
    println!("  - Parameter similarity (30% weight)");
    println!("    - Parameter count matching");
    println!("    - Parameter type equivalence");
    println!("    - Optional/varargs flag matching");
    
    println!("  - Return type similarity (20% weight)");
    println!("    - Type equivalence checking");
    println!("    - Generic parameter matching");
    
    println!("  - Modifier similarity (10% weight)");
    println!("    - Visibility matching");
    println!("    - Static/abstract/final matching");
    
    println!("\nSimilarity thresholds:");
    println!("  - Exact match: 1.0");
    println!("  - High similarity: 0.8-0.99");
    println!("  - Potential match: 0.7-0.79");
    println!("  - Low similarity: 0.3-0.69");
    println!("  - No match: 0.0-0.29");
    
    Ok(())
}

fn demo_overload_detection() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Function Overload Detection ---");
    
    println!("Overload detection identifies:");
    println!("  - Functions with the same name but different signatures");
    println!("  - Parameter count variations");
    println!("  - Parameter type variations");
    println!("  - Generic parameter variations");
    
    println!("\nOverload analysis benefits:");
    println!("  - API evolution tracking");
    println!("  - Refactoring impact assessment");
    println!("  - Code complexity measurement");
    println!("  - Documentation generation");
    
    Ok(())
}

fn demo_cross_language_signatures() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Cross-Language Function Signature Handling ---");
    
    let languages_and_examples = vec![
        (Language::Java, "public List<String> processData(String input, boolean validate)"),
        (Language::Python, "def process_data(input: str, validate: bool = True) -> List[str]"),
        (Language::JavaScript, "function processData(input, validate = true)"),
        (Language::Cpp, "std::vector<std::string> processData(const std::string& input, bool validate)"),
        (Language::C, "char** process_data(const char* input, int validate, int* result_count)"),
    ];
    
    println!("Equivalent function signatures across languages:");
    
    for (language, signature) in languages_and_examples {
        println!("  {:?}: {}", language, signature);
    }
    
    println!("\nCross-language normalization handles:");
    println!("  - Naming conventions (camelCase vs snake_case)");
    println!("  - Type system differences (List<String> vs list[str])");
    println!("  - Parameter syntax variations");
    println!("  - Default parameter handling");
    println!("  - Generic/template parameter mapping");
    
    Ok(())
}
