//! Demonstration of comprehensive dependency graph construction capabilities

use smart_diff_parser::{Language, TreeSitterParser};
use smart_diff_semantic::{
    CallType, ComprehensiveDependencyGraphBuilder, DependencyAnalysisConfig, SymbolResolver,
    SymbolResolverConfig, TypeExtractor, TypeExtractorConfig,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Smart Code Diff - Comprehensive Dependency Graph Demo");
    println!("====================================================");

    // Demo basic dependency graph construction
    demo_basic_dependency_graph()?;

    // Demo comprehensive dependency analysis
    demo_comprehensive_analysis()?;

    // Demo dependency hotspot identification
    demo_hotspot_identification()?;

    // Demo cross-file dependency tracking
    demo_cross_file_dependencies()?;

    Ok(())
}

fn demo_basic_dependency_graph() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Basic Dependency Graph Construction ---");

    let parser = TreeSitterParser::new()?;

    let java_code = r#"
package com.example.service;

import java.util.List;
import java.util.Map;
import com.example.model.User;
import com.example.repository.UserRepository;

public class UserService {
    private final UserRepository userRepository;
    private final ValidationService validationService;
    private Map<String, User> userCache;
    
    public UserService(UserRepository userRepository, ValidationService validationService) {
        this.userRepository = userRepository;
        this.validationService = validationService;
        this.userCache = new HashMap<>();
    }
    
    public User createUser(String name, String email) {
        // Validate input
        if (!validationService.isValidEmail(email)) {
            throw new IllegalArgumentException("Invalid email");
        }
        
        // Check if user exists
        User existingUser = userRepository.findByEmail(email);
        if (existingUser != null) {
            throw new IllegalStateException("User already exists");
        }
        
        // Create new user
        User newUser = new User(name, email);
        User savedUser = userRepository.save(newUser);
        
        // Cache the user
        userCache.put(email, savedUser);
        
        return savedUser;
    }
    
    public List<User> getAllUsers() {
        return userRepository.findAll();
    }
    
    public User getUserByEmail(String email) {
        // Check cache first
        User cachedUser = userCache.get(email);
        if (cachedUser != null) {
            return cachedUser;
        }
        
        // Fetch from repository
        User user = userRepository.findByEmail(email);
        if (user != null) {
            userCache.put(email, user);
        }
        
        return user;
    }
    
    public void deleteUser(String email) {
        User user = getUserByEmail(email);
        if (user != null) {
            userRepository.delete(user);
            userCache.remove(email);
        }
    }
}

class ValidationService {
    public boolean isValidEmail(String email) {
        return email != null && email.contains("@") && email.contains(".");
    }
    
    public boolean isValidName(String name) {
        return name != null && !name.trim().isEmpty();
    }
}
"#;

    // Parse the code
    let parse_result = parser.parse(java_code, Language::Java)?;

    // Create dependency graph builder
    let config = DependencyAnalysisConfig::default();
    let mut builder = ComprehensiveDependencyGraphBuilder::new(config);

    // Build dependency graph
    let files = vec![("UserService.java".to_string(), parse_result)];
    builder.build_comprehensive_graph(files)?;

    // Get analysis results
    let analysis = builder.analyze_comprehensive_dependencies();

    println!("Dependency Graph Analysis:");
    println!("  Total nodes: {}", analysis.total_nodes);
    println!("  Total edges: {}", analysis.total_edges);
    println!(
        "  Function call dependencies: {}",
        analysis.function_call_dependencies
    );
    println!("  Type dependencies: {}", analysis.type_dependencies);
    println!(
        "  Variable dependencies: {}",
        analysis.variable_dependencies
    );
    println!(
        "  Inheritance dependencies: {}",
        analysis.inheritance_dependencies
    );

    if !analysis.circular_dependencies.is_empty() {
        println!(
            "  Circular dependencies found: {}",
            analysis.circular_dependencies.len()
        );
        for cycle in &analysis.circular_dependencies {
            println!("    Cycle: {:?}", cycle);
        }
    } else {
        println!("  No circular dependencies found");
    }

    if !analysis.dependency_layers.is_empty() {
        println!("  Dependency layers: {}", analysis.dependency_layers.len());
        for (i, layer) in analysis.dependency_layers.iter().enumerate() {
            println!("    Layer {}: {} nodes", i + 1, layer.len());
            for node in layer.iter().take(3) {
                println!("      - {}", node);
            }
            if layer.len() > 3 {
                println!("      ... and {} more", layer.len() - 3);
            }
        }
    }

    Ok(())
}

fn demo_comprehensive_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Comprehensive Dependency Analysis ---");

    let parser = TreeSitterParser::new()?;

    // Create a more complex example with multiple classes and relationships
    let complex_code = r#"
public abstract class BaseProcessor<T> {
    protected Logger logger;
    protected MetricsCollector metrics;
    
    public BaseProcessor(Logger logger, MetricsCollector metrics) {
        this.logger = logger;
        this.metrics = metrics;
    }
    
    public abstract ProcessResult<T> process(T input);
    
    protected void logProcessing(String message) {
        logger.info("Processing: " + message);
        metrics.incrementCounter("processing.events");
    }
}

public class DataProcessor extends BaseProcessor<DataRecord> {
    private final DataValidator validator;
    private final DataTransformer transformer;
    private final DataRepository repository;
    private final CacheManager cacheManager;
    
    public DataProcessor(Logger logger, MetricsCollector metrics, 
                        DataValidator validator, DataTransformer transformer,
                        DataRepository repository, CacheManager cacheManager) {
        super(logger, metrics);
        this.validator = validator;
        this.transformer = transformer;
        this.repository = repository;
        this.cacheManager = cacheManager;
    }
    
    @Override
    public ProcessResult<DataRecord> process(DataRecord input) {
        logProcessing("Starting data processing for: " + input.getId());
        
        try {
            // Validate input
            ValidationResult validationResult = validator.validate(input);
            if (!validationResult.isValid()) {
                return ProcessResult.failure("Validation failed: " + validationResult.getErrors());
            }
            
            // Check cache
            String cacheKey = generateCacheKey(input);
            DataRecord cachedResult = cacheManager.get(cacheKey);
            if (cachedResult != null) {
                logProcessing("Cache hit for: " + input.getId());
                return ProcessResult.success(cachedResult);
            }
            
            // Transform data
            DataRecord transformedData = transformer.transform(input);
            
            // Save to repository
            DataRecord savedData = repository.save(transformedData);
            
            // Cache result
            cacheManager.put(cacheKey, savedData);
            
            logProcessing("Successfully processed: " + input.getId());
            return ProcessResult.success(savedData);
            
        } catch (Exception e) {
            logger.error("Processing failed for: " + input.getId(), e);
            metrics.incrementCounter("processing.errors");
            return ProcessResult.failure("Processing failed: " + e.getMessage());
        }
    }
    
    private String generateCacheKey(DataRecord input) {
        return "data_" + input.getId() + "_" + input.getVersion();
    }
    
    public List<DataRecord> processAll(List<DataRecord> inputs) {
        List<DataRecord> results = new ArrayList<>();
        
        for (DataRecord input : inputs) {
            ProcessResult<DataRecord> result = process(input);
            if (result.isSuccess()) {
                results.add(result.getData());
            }
        }
        
        return results;
    }
}

interface DataValidator {
    ValidationResult validate(DataRecord data);
}

interface DataTransformer {
    DataRecord transform(DataRecord input);
}

interface DataRepository {
    DataRecord save(DataRecord data);
    DataRecord findById(String id);
    List<DataRecord> findAll();
}

class ProcessResult<T> {
    private final boolean success;
    private final T data;
    private final String errorMessage;
    
    private ProcessResult(boolean success, T data, String errorMessage) {
        this.success = success;
        this.data = data;
        this.errorMessage = errorMessage;
    }
    
    public static <T> ProcessResult<T> success(T data) {
        return new ProcessResult<>(true, data, null);
    }
    
    public static <T> ProcessResult<T> failure(String errorMessage) {
        return new ProcessResult<>(false, null, errorMessage);
    }
    
    public boolean isSuccess() { return success; }
    public T getData() { return data; }
    public String getErrorMessage() { return errorMessage; }
}
"#;

    let parse_result = parser.parse(complex_code, Language::Java)?;

    // Create comprehensive dependency graph
    let config = DependencyAnalysisConfig {
        include_function_calls: true,
        include_type_dependencies: true,
        include_variable_usage: true,
        include_import_dependencies: true,
        include_inheritance: true,
        min_dependency_strength: 0.2,
        max_transitive_depth: 8,
    };

    let mut builder = ComprehensiveDependencyGraphBuilder::new(config);

    // Add symbol resolution
    let mut symbol_resolver = SymbolResolver::with_defaults();
    symbol_resolver.process_file("DataProcessor.java", &parse_result)?;
    builder = builder.with_symbol_table(symbol_resolver.get_symbol_table().clone());

    // Add type extraction
    let mut type_extractor = TypeExtractor::with_defaults(Language::Java);
    let type_result = type_extractor.extract_types("DataProcessor.java", &parse_result)?;
    builder.add_type_extraction_result("DataProcessor.java".to_string(), type_result);

    // Build comprehensive graph
    let files = vec![("DataProcessor.java".to_string(), parse_result)];
    builder.build_comprehensive_graph(files)?;

    let analysis = builder.analyze_comprehensive_dependencies();

    println!("Comprehensive Analysis Results:");
    println!("  Total nodes: {}", analysis.total_nodes);
    println!("  Total edges: {}", analysis.total_edges);
    println!("  Function calls: {}", analysis.function_call_dependencies);
    println!("  Type dependencies: {}", analysis.type_dependencies);
    println!("  Variable usage: {}", analysis.variable_dependencies);
    println!("  Inheritance: {}", analysis.inheritance_dependencies);

    // Show coupling metrics for top nodes
    println!("\nTop 5 Most Coupled Components:");
    let mut coupling_pairs: Vec<_> = analysis.coupling_metrics.iter().collect();
    coupling_pairs.sort_by(|a, b| {
        let score_a = (a.1.afferent_coupling + a.1.efferent_coupling) as f64;
        let score_b = (b.1.afferent_coupling + b.1.efferent_coupling) as f64;
        score_b.partial_cmp(&score_a).unwrap()
    });

    for (i, (name, metrics)) in coupling_pairs.iter().take(5).enumerate() {
        println!("  {}. {}", i + 1, name);
        println!("     Afferent coupling: {}", metrics.afferent_coupling);
        println!("     Efferent coupling: {}", metrics.efferent_coupling);
        println!("     Instability: {:.3}", metrics.instability);
        println!(
            "     Function call coupling: {}",
            metrics.function_call_coupling
        );
        println!("     Type coupling: {}", metrics.type_coupling);
        println!();
    }

    Ok(())
}

fn demo_hotspot_identification() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Dependency Hotspot Identification ---");

    // This would use the same analysis from the previous demo
    // For brevity, we'll create a mock analysis result

    println!("Dependency hotspots are components with high coupling that may indicate:");
    println!("  - Design issues (violation of single responsibility)");
    println!("  - Maintenance difficulties");
    println!("  - Testing challenges");
    println!("  - Potential refactoring candidates");

    println!("\nHotspot identification criteria:");
    println!("  - High afferent coupling (many components depend on this)");
    println!("  - High efferent coupling (this depends on many components)");
    println!("  - High instability (ratio of efferent to total coupling)");
    println!("  - Complex interaction patterns");

    Ok(())
}

fn demo_cross_file_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Cross-File Dependency Tracking ---");

    println!("Cross-file dependency analysis tracks:");
    println!("  - Import/export relationships");
    println!("  - Module dependencies");
    println!("  - Package-level coupling");
    println!("  - Circular import detection");
    println!("  - Dependency inversion opportunities");

    println!("\nBenefits of cross-file analysis:");
    println!("  - Architectural insight");
    println!("  - Refactoring guidance");
    println!("  - Module boundary validation");
    println!("  - Build optimization opportunities");

    Ok(())
}
