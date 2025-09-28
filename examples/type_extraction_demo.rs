//! Demonstration of comprehensive type information extraction capabilities

use smart_diff_parser::{TreeSitterParser, Language};
use smart_diff_semantic::{
    TypeExtractor, TypeExtractorConfig, TypeSignature, TypeEquivalence,
    TypeDependencyGraphBuilder, TypeRelationshipType
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Smart Code Diff - Type Information Extraction Demo");
    println!("=================================================");
    
    // Demo type signature parsing
    demo_type_signature_parsing()?;
    
    // Demo type equivalence checking
    demo_type_equivalence()?;
    
    // Demo type extraction from code
    demo_type_extraction()?;
    
    // Demo type dependency analysis
    demo_type_dependency_analysis()?;
    
    // Demo cross-language type handling
    demo_cross_language_types()?;
    
    Ok(())
}

fn demo_type_signature_parsing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Type Signature Parsing ---");
    
    let test_types = vec![
        "String",
        "List<String>",
        "Map<String, Integer>",
        "List<Map<String, List<Integer>>>",
        "String[]",
        "int[][]",
        "List<String>[]",
        "Optional<String>?",
        "Map<K, V>",
    ];
    
    println!("Parsing various type signatures:");
    
    for type_str in test_types {
        match TypeSignature::parse(type_str) {
            Ok(signature) => {
                println!("  {} -> {}", type_str, signature.to_string());
                println!("    Base type: {}", signature.base_type);
                println!("    Generic params: {}", signature.generic_params.len());
                println!("    Array dimensions: {}", signature.array_dimensions);
                println!("    Nullable: {}", signature.is_nullable);
                if !signature.modifiers.is_empty() {
                    println!("    Modifiers: {:?}", signature.modifiers);
                }
                println!();
            }
            Err(e) => {
                println!("  {} -> ERROR: {}", type_str, e);
            }
        }
    }
    
    Ok(())
}

fn demo_type_equivalence() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Type Equivalence Checking ---");
    
    let equivalence_tests = vec![
        ("String", "String", true),
        ("int", "i32", true),
        ("String", "string", true),
        ("bool", "Boolean", true),
        ("long", "i64", true),
        ("float", "f32", true),
        ("double", "f64", true),
        ("String", "Integer", false),
        ("int", "float", false),
        ("List", "ArrayList", false),
    ];
    
    println!("Type equivalence tests:");
    
    for (type1, type2, expected) in equivalence_tests {
        let result = TypeEquivalence::are_equivalent(type1, type2);
        let status = if result == expected { "✓" } else { "✗" };
        println!("  {} {} ≡ {} -> {} (expected: {})", 
                 status, type1, type2, result, expected);
    }
    
    // Test complex type equivalence
    println!("\nComplex type equivalence tests:");
    
    let complex_tests = vec![
        ("List<String>", "List<String>", true),
        ("List<String>", "List<Integer>", false),
        ("Map<String, Integer>", "Map<String, Integer>", true),
        ("Map<String, Integer>", "Map<Integer, String>", false),
        ("String[]", "String[]", true),
        ("String[]", "String[][]", false),
    ];
    
    for (type1_str, type2_str, expected) in complex_tests {
        let type1 = TypeSignature::parse(type1_str)?;
        let type2 = TypeSignature::parse(type2_str)?;
        let result = TypeEquivalence::are_complex_types_equivalent(&type1, &type2);
        let status = if result == expected { "✓" } else { "✗" };
        println!("  {} {} ≡ {} -> {} (expected: {})", 
                 status, type1_str, type2_str, result, expected);
    }
    
    // Test type similarity
    println!("\nType similarity scores:");
    
    let similarity_tests = vec![
        ("List<String>", "List<String>"),
        ("List<String>", "List<Integer>"),
        ("List<String>", "ArrayList<String>"),
        ("String", "Integer"),
        ("int", "long"),
        ("List<String>", "String"),
    ];
    
    for (type1_str, type2_str) in similarity_tests {
        let type1 = TypeSignature::parse(type1_str)?;
        let type2 = TypeSignature::parse(type2_str)?;
        let similarity = TypeEquivalence::calculate_type_similarity(&type1, &type2);
        println!("  {} ~ {} -> {:.3}", type1_str, type2_str, similarity);
    }
    
    Ok(())
}

fn demo_type_extraction() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Type Extraction from Code ---");
    
    let parser = TreeSitterParser::new()?;
    let mut extractor = TypeExtractor::with_defaults(Language::Java);
    
    let java_code = r#"
package com.example.types;

import java.util.List;
import java.util.Map;
import java.util.Optional;

public abstract class DataProcessor<T extends Comparable<T>> {
    private static final String DEFAULT_NAME = "processor";
    private final List<T> data;
    private Map<String, Integer> counters;
    protected Optional<String> name;
    
    public DataProcessor(List<T> initialData) {
        this.data = initialData;
        this.counters = new HashMap<>();
        this.name = Optional.of(DEFAULT_NAME);
    }
    
    public abstract ProcessResult<T> process(T item, ProcessOptions options);
    
    public final List<T> getData() {
        return Collections.unmodifiableList(data);
    }
    
    public void addItem(T item) {
        if (item != null) {
            data.add(item);
            updateCounters(item);
        }
    }
    
    private void updateCounters(T item) {
        String key = item.toString();
        counters.put(key, counters.getOrDefault(key, 0) + 1);
    }
    
    protected static class ProcessOptions {
        public final boolean validateInput;
        public final int maxRetries;
        
        public ProcessOptions(boolean validateInput, int maxRetries) {
            this.validateInput = validateInput;
            this.maxRetries = maxRetries;
        }
    }
}

interface ProcessResult<T> {
    T getResult();
    boolean isSuccess();
    Optional<String> getErrorMessage();
}

enum ProcessStatus {
    PENDING,
    PROCESSING,
    COMPLETED,
    FAILED
}
"#;
    
    let parse_result = parser.parse(java_code, Language::Java)?;
    let extraction_result = extractor.extract_types("DataProcessor.java", &parse_result)?;
    
    println!("Extracted type information:");
    println!("  Total types found: {}", extraction_result.types.len());
    
    for extracted_type in &extraction_result.types {
        let type_info = &extracted_type.type_info;
        println!("\n  Type: {} ({})", type_info.name, format!("{:?}", type_info.kind));
        println!("    File: {} (line {})", type_info.file_path, type_info.line);
        
        if !type_info.generic_parameters.is_empty() {
            println!("    Generic parameters: {:?}", type_info.generic_parameters);
        }
        
        if !extracted_type.inheritance.is_empty() {
            println!("    Inherits from: {:?}", extracted_type.inheritance);
        }
        
        if !extracted_type.implementations.is_empty() {
            println!("    Implements: {:?}", extracted_type.implementations);
        }
        
        if !type_info.fields.is_empty() {
            println!("    Fields: {}", type_info.fields.len());
            for field in &type_info.fields {
                println!("      - {}: {} ({:?})", field.name, field.field_type, field.visibility);
            }
        }
        
        if !type_info.methods.is_empty() {
            println!("    Methods: {}", type_info.methods.len());
            for method in &type_info.methods {
                let params_str = method.parameters.join(", ");
                println!("      - {}({}) -> {} ({:?})", 
                         method.name, params_str, method.return_type, method.visibility);
            }
        }
        
        if !extracted_type.dependencies.is_empty() {
            println!("    Dependencies: {:?}", extracted_type.dependencies);
        }
        
        if !extracted_type.generic_constraints.is_empty() {
            println!("    Generic constraints: {:?}", extracted_type.generic_constraints);
        }
    }
    
    // Show type aliases
    if !extraction_result.type_aliases.is_empty() {
        println!("\nType aliases:");
        for (alias, target) in &extraction_result.type_aliases {
            println!("  {} -> {}", alias, target);
        }
    }
    
    Ok(())
}

fn demo_type_dependency_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Type Dependency Analysis ---");
    
    let parser = TreeSitterParser::new()?;
    let mut extractor = TypeExtractor::with_defaults(Language::Java);
    
    // Process multiple related files
    let interface_code = r#"
public interface Repository<T, ID> {
    Optional<T> findById(ID id);
    List<T> findAll();
    T save(T entity);
    void deleteById(ID id);
}
"#;
    
    let implementation_code = r#"
public class UserRepository implements Repository<User, Long> {
    private final DatabaseConnection connection;
    
    public UserRepository(DatabaseConnection connection) {
        this.connection = connection;
    }
    
    @Override
    public Optional<User> findById(Long id) {
        return connection.query("SELECT * FROM users WHERE id = ?", id)
            .map(User::fromResultSet);
    }
    
    @Override
    public List<User> findAll() {
        return connection.queryList("SELECT * FROM users")
            .stream()
            .map(User::fromResultSet)
            .collect(Collectors.toList());
    }
    
    @Override
    public User save(User entity) {
        if (entity.getId() == null) {
            return insert(entity);
        } else {
            return update(entity);
        }
    }
    
    @Override
    public void deleteById(Long id) {
        connection.execute("DELETE FROM users WHERE id = ?", id);
    }
    
    private User insert(User user) {
        // Implementation details...
        return user;
    }
    
    private User update(User user) {
        // Implementation details...
        return user;
    }
}
"#;
    
    let files = vec![
        ("Repository.java".to_string(), parser.parse(interface_code, Language::Java)?),
        ("UserRepository.java".to_string(), parser.parse(implementation_code, Language::Java)?),
    ];
    
    let extraction_result = extractor.extract_types_from_files(files)?;
    
    // Build dependency graph
    let mut dependency_builder = TypeDependencyGraphBuilder::new();
    dependency_builder.build_from_extraction_result(&extraction_result)?;
    
    let analysis = dependency_builder.analyze_dependencies();
    
    println!("Type dependency analysis results:");
    println!("  Total types: {}", analysis.total_types);
    
    if !analysis.inheritance_chains.is_empty() {
        println!("  Inheritance chains: {}", analysis.inheritance_chains.len());
        for chain in &analysis.inheritance_chains {
            println!("    {:?}", chain);
        }
    }
    
    if !analysis.circular_dependencies.is_empty() {
        println!("  Circular dependencies: {}", analysis.circular_dependencies.len());
        for cycle in &analysis.circular_dependencies {
            println!("    {:?}", cycle);
        }
    }
    
    println!("  Coupling metrics:");
    for (type_name, metrics) in &analysis.coupling_metrics {
        println!("    {}: AC={}, EC={}, I={:.3}, A={:.3}", 
                 type_name, 
                 metrics.afferent_coupling, 
                 metrics.efferent_coupling,
                 metrics.instability,
                 metrics.abstractness);
    }
    
    if !analysis.type_hierarchy_depth.is_empty() {
        println!("  Type hierarchy depths:");
        for (type_name, depth) in &analysis.type_hierarchy_depth {
            println!("    {}: {}", type_name, depth);
        }
    }
    
    if !analysis.interface_implementations.is_empty() {
        println!("  Interface implementations:");
        for (interface, implementations) in &analysis.interface_implementations {
            println!("    {} -> {:?}", interface, implementations);
        }
    }
    
    Ok(())
}

fn demo_cross_language_types() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Cross-Language Type Handling ---");
    
    let languages = vec![
        (Language::Java, "List<String>"),
        (Language::Python, "List[str]"),
        (Language::JavaScript, "Array<string>"),
        (Language::Cpp, "std::vector<std::string>"),
        (Language::C, "char*[]"),
    ];
    
    println!("Parsing equivalent types across languages:");
    
    for (language, type_str) in languages {
        let extractor = TypeExtractor::with_defaults(language);
        match extractor.parse_type_signature(type_str) {
            Ok(signature) => {
                println!("  {:?}: {} -> {}", language, type_str, signature.to_string());
                println!("    Base: {}, Generics: {}, Arrays: {}", 
                         signature.base_type, 
                         signature.generic_params.len(),
                         signature.array_dimensions);
                if !signature.modifiers.is_empty() {
                    println!("    Modifiers: {:?}", signature.modifiers);
                }
            }
            Err(e) => {
                println!("  {:?}: {} -> ERROR: {}", language, type_str, e);
            }
        }
    }
    
    Ok(())
}
