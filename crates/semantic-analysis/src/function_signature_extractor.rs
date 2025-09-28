//! Comprehensive function signature extraction and analysis

use crate::{
    MethodInfo, Symbol, SymbolKind, SymbolTable, TypeEquivalence, TypeSignature, Visibility,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use smart_diff_parser::{ASTNode, Language, NodeType, ParseResult};
use std::collections::{HashMap, HashSet};

/// Configuration for function signature extraction
#[derive(Debug, Clone)]
pub struct FunctionSignatureConfig {
    /// Include private functions
    pub include_private: bool,
    /// Include static functions
    pub include_static: bool,
    /// Include abstract functions
    pub include_abstract: bool,
    /// Include constructor functions
    pub include_constructors: bool,
    /// Include getter/setter methods
    pub include_accessors: bool,
    /// Normalize parameter names for comparison
    pub normalize_parameter_names: bool,
    /// Extract function body complexity metrics
    pub extract_complexity_metrics: bool,
    /// Maximum parameter count to consider
    pub max_parameter_count: usize,
}

impl Default for FunctionSignatureConfig {
    fn default() -> Self {
        Self {
            include_private: true,
            include_static: true,
            include_abstract: true,
            include_constructors: true,
            include_accessors: true,
            normalize_parameter_names: false,
            extract_complexity_metrics: true,
            max_parameter_count: 20,
        }
    }
}

/// Comprehensive function signature extractor
pub struct FunctionSignatureExtractor {
    config: FunctionSignatureConfig,
    language: Language,
    current_file: String,
}

/// Enhanced function signature with detailed information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnhancedFunctionSignature {
    /// Basic signature information
    pub name: String,
    pub qualified_name: String,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: TypeSignature,
    pub generic_parameters: Vec<GenericParameter>,

    /// Modifiers and attributes
    pub visibility: Visibility,
    pub modifiers: Vec<String>,
    pub annotations: Vec<String>,

    /// Location information
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,

    /// Function characteristics
    pub function_type: FunctionType,
    pub complexity_metrics: Option<FunctionComplexityMetrics>,
    pub dependencies: Vec<String>,

    /// Signature hash for quick comparison
    pub signature_hash: String,
    pub normalized_hash: String,
}

/// Function parameter with detailed type information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: TypeSignature,
    pub default_value: Option<String>,
    pub is_optional: bool,
    pub is_varargs: bool,
    pub annotations: Vec<String>,
    pub position: usize,
}

/// Generic parameter information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenericParameter {
    pub name: String,
    pub bounds: Vec<TypeSignature>,
    pub variance: GenericVariance,
}

/// Generic parameter variance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GenericVariance {
    Invariant,
    Covariant,
    Contravariant,
}

/// Function type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FunctionType {
    Function,     // Regular function
    Method,       // Instance method
    StaticMethod, // Static method
    Constructor,  // Constructor
    Destructor,   // Destructor
    Getter,       // Property getter
    Setter,       // Property setter
    Operator,     // Operator overload
    Lambda,       // Lambda/anonymous function
    Callback,     // Callback function
}

/// Function complexity metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionComplexityMetrics {
    pub cyclomatic_complexity: usize,
    pub cognitive_complexity: usize,
    pub lines_of_code: usize,
    pub parameter_count: usize,
    pub nesting_depth: usize,
    pub branch_count: usize,
    pub loop_count: usize,
    pub call_count: usize,
}

/// Function signature extraction result
#[derive(Debug)]
pub struct FunctionSignatureExtractionResult {
    pub signatures: Vec<EnhancedFunctionSignature>,
    pub signature_map: HashMap<String, EnhancedFunctionSignature>,
    pub overloaded_functions: HashMap<String, Vec<EnhancedFunctionSignature>>,
    pub function_hierarchy: HashMap<String, Vec<String>>, // parent -> children
    pub extraction_stats: ExtractionStats,
}

/// Statistics about the extraction process
#[derive(Debug, Clone)]
pub struct ExtractionStats {
    pub total_functions: usize,
    pub public_functions: usize,
    pub private_functions: usize,
    pub static_functions: usize,
    pub abstract_functions: usize,
    pub constructors: usize,
    pub overloaded_functions: usize,
    pub generic_functions: usize,
    pub complex_functions: usize, // High complexity
}

/// Function signature similarity result
#[derive(Debug, Clone)]
pub struct FunctionSignatureSimilarity {
    pub overall_similarity: f64,
    pub name_similarity: f64,
    pub parameter_similarity: f64,
    pub return_type_similarity: f64,
    pub modifier_similarity: f64,
    pub complexity_similarity: f64,
    pub is_potential_match: bool,
    pub similarity_breakdown: SimilarityBreakdown,
}

/// Detailed similarity breakdown
#[derive(Debug, Clone)]
pub struct SimilarityBreakdown {
    pub exact_name_match: bool,
    pub parameter_count_match: bool,
    pub parameter_types_match: Vec<bool>,
    pub return_type_match: bool,
    pub visibility_match: bool,
    pub static_match: bool,
    pub generic_parameters_match: bool,
}

impl FunctionSignatureExtractor {
    pub fn new(language: Language, config: FunctionSignatureConfig) -> Self {
        Self {
            config,
            language,
            current_file: String::new(),
        }
    }

    pub fn with_defaults(language: Language) -> Self {
        Self::new(language, FunctionSignatureConfig::default())
    }

    /// Extract function signatures from a parsed file
    pub fn extract_signatures(
        &mut self,
        file_path: &str,
        parse_result: &ParseResult,
    ) -> Result<FunctionSignatureExtractionResult> {
        self.current_file = file_path.to_string();

        let mut signatures = Vec::new();
        let mut signature_map = HashMap::new();
        let mut overloaded_functions: HashMap<String, Vec<EnhancedFunctionSignature>> =
            HashMap::new();
        let mut function_hierarchy = HashMap::new();

        // Extract signatures from AST
        self.extract_signatures_from_node(&parse_result.ast, &mut signatures, Vec::new())?;

        // Build signature map and identify overloads
        for signature in &signatures {
            signature_map.insert(signature.qualified_name.clone(), signature.clone());

            // Group overloaded functions
            overloaded_functions
                .entry(signature.name.clone())
                .or_insert_with(Vec::new)
                .push(signature.clone());
        }

        // Build function hierarchy (for inheritance analysis)
        self.build_function_hierarchy(&signatures, &mut function_hierarchy);

        // Calculate extraction statistics
        let extraction_stats = self.calculate_extraction_stats(&signatures, &overloaded_functions);

        Ok(FunctionSignatureExtractionResult {
            signatures,
            signature_map,
            overloaded_functions,
            function_hierarchy,
            extraction_stats,
        })
    }

    /// Extract signatures from multiple files
    pub fn extract_signatures_from_files(
        &mut self,
        files: Vec<(String, ParseResult)>,
    ) -> Result<FunctionSignatureExtractionResult> {
        let mut all_signatures = Vec::new();
        let mut signature_map = HashMap::new();
        let mut overloaded_functions: HashMap<String, Vec<EnhancedFunctionSignature>> =
            HashMap::new();
        let mut function_hierarchy = HashMap::new();

        for (file_path, parse_result) in files {
            let file_result = self.extract_signatures(&file_path, &parse_result)?;

            all_signatures.extend(file_result.signatures);
            signature_map.extend(file_result.signature_map);

            // Merge overloaded functions
            for (name, overloads) in file_result.overloaded_functions {
                overloaded_functions
                    .entry(name)
                    .or_insert_with(Vec::new)
                    .extend(overloads);
            }

            // Merge function hierarchy
            function_hierarchy.extend(file_result.function_hierarchy);
        }

        let extraction_stats =
            self.calculate_extraction_stats(&all_signatures, &overloaded_functions);

        Ok(FunctionSignatureExtractionResult {
            signatures: all_signatures,
            signature_map,
            overloaded_functions,
            function_hierarchy,
            extraction_stats,
        })
    }

    /// Extract signatures from AST node recursively
    fn extract_signatures_from_node(
        &self,
        node: &ASTNode,
        signatures: &mut Vec<EnhancedFunctionSignature>,
        scope_path: Vec<String>,
    ) -> Result<()> {
        match node.node_type {
            NodeType::Function | NodeType::Method | NodeType::Constructor => {
                if let Some(signature) = self.extract_function_signature(node, &scope_path)? {
                    if self.should_include_function(&signature) {
                        signatures.push(signature);
                    }
                }
            }
            NodeType::Class | NodeType::Interface => {
                // Update scope path for nested functions
                let mut new_scope_path = scope_path;
                if let Some(class_name) = node.metadata.attributes.get("name") {
                    new_scope_path.push(class_name.clone());
                }

                // Process children with updated scope
                for child in &node.children {
                    self.extract_signatures_from_node(child, signatures, new_scope_path.clone())?;
                }
                return Ok(());
            }
            _ => {}
        }

        // Process children with current scope
        for child in &node.children {
            self.extract_signatures_from_node(child, signatures, scope_path.clone())?;
        }

        Ok(())
    }

    /// Extract function signature from a function node
    fn extract_function_signature(
        &self,
        node: &ASTNode,
        scope_path: &[String],
    ) -> Result<Option<EnhancedFunctionSignature>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .ok_or_else(|| anyhow!("Function node missing name"))?;

        let qualified_name = if scope_path.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", scope_path.join("."), name)
        };

        // Extract parameters
        let parameters = self.extract_function_parameters(node)?;

        // Extract return type
        let return_type = self.extract_return_type(node)?;

        // Extract generic parameters
        let generic_parameters = self.extract_generic_parameters(node)?;

        // Extract modifiers and visibility
        let visibility = self.extract_visibility(node);
        let modifiers = self.extract_modifiers(node);
        let annotations = self.extract_annotations(node);

        // Determine function type
        let function_type = self.determine_function_type(node, name);

        // Extract location information
        let end_line = node
            .metadata
            .attributes
            .get("end_line")
            .and_then(|s| s.parse().ok())
            .unwrap_or(node.metadata.line);

        // Extract complexity metrics if enabled
        let complexity_metrics = if self.config.extract_complexity_metrics {
            Some(self.calculate_complexity_metrics(node)?)
        } else {
            None
        };

        // Extract dependencies
        let dependencies = self.extract_function_dependencies(node);

        // Generate signature hashes
        let signature_hash =
            self.generate_signature_hash(name, &parameters, &return_type, &modifiers);
        let normalized_hash = self.generate_normalized_hash(name, &parameters, &return_type);

        let signature = EnhancedFunctionSignature {
            name: name.clone(),
            qualified_name,
            parameters,
            return_type,
            generic_parameters,
            visibility,
            modifiers,
            annotations,
            file_path: self.current_file.clone(),
            line: node.metadata.line,
            column: node.metadata.column,
            end_line,
            function_type,
            complexity_metrics,
            dependencies,
            signature_hash,
            normalized_hash,
        };

        Ok(Some(signature))
    }

    /// Extract function parameters from node
    fn extract_function_parameters(&self, node: &ASTNode) -> Result<Vec<FunctionParameter>> {
        let mut parameters = Vec::new();

        // Find parameter list node
        for child in &node.children {
            if matches!(
                child.node_type,
                NodeType::ParameterList | NodeType::Parameters
            ) {
                for (position, param_node) in child.children.iter().enumerate() {
                    if let Some(parameter) = self.extract_single_parameter(param_node, position)? {
                        parameters.push(parameter);
                    }
                }
                break;
            }
        }

        Ok(parameters)
    }

    /// Extract a single parameter from parameter node
    fn extract_single_parameter(
        &self,
        node: &ASTNode,
        position: usize,
    ) -> Result<Option<FunctionParameter>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .unwrap_or(&format!("param{}", position))
            .clone();

        // Extract parameter type
        let param_type = if let Some(type_str) = node.metadata.attributes.get("type") {
            self.parse_type_signature(type_str)?
        } else {
            TypeSignature::new("Object".to_string()) // Default type
        };

        // Extract default value
        let default_value = node.metadata.attributes.get("default_value").cloned();

        // Check if optional or varargs
        let is_optional =
            node.metadata.attributes.get("optional").is_some() || default_value.is_some();
        let is_varargs = node.metadata.attributes.get("varargs").is_some();

        // Extract annotations
        let annotations = self.extract_parameter_annotations(node);

        Ok(Some(FunctionParameter {
            name,
            param_type,
            default_value,
            is_optional,
            is_varargs,
            annotations,
            position,
        }))
    }

    /// Extract return type from function node
    fn extract_return_type(&self, node: &ASTNode) -> Result<TypeSignature> {
        if let Some(return_type_str) = node.metadata.attributes.get("return_type") {
            self.parse_type_signature(return_type_str)
        } else {
            // Default return type based on function type
            match node.node_type {
                NodeType::Constructor => Ok(TypeSignature::new("void".to_string())),
                _ => Ok(TypeSignature::new("void".to_string())),
            }
        }
    }

    /// Extract generic parameters from function node
    fn extract_generic_parameters(&self, node: &ASTNode) -> Result<Vec<GenericParameter>> {
        let mut generic_params = Vec::new();

        // Look for generic parameter declarations
        for child in &node.children {
            if child.node_type == NodeType::GenericParameters {
                for generic_node in &child.children {
                    if let Some(generic_param) = self.extract_generic_parameter(generic_node)? {
                        generic_params.push(generic_param);
                    }
                }
                break;
            }
        }

        Ok(generic_params)
    }

    /// Extract a single generic parameter
    fn extract_generic_parameter(&self, node: &ASTNode) -> Result<Option<GenericParameter>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .ok_or_else(|| anyhow!("Generic parameter missing name"))?;

        // Extract bounds (extends/implements clauses)
        let mut bounds = Vec::new();
        if let Some(bounds_str) = node.metadata.attributes.get("bounds") {
            for bound_str in bounds_str.split(',') {
                let bound_type = self.parse_type_signature(bound_str.trim())?;
                bounds.push(bound_type);
            }
        }

        // Determine variance (for languages that support it)
        let variance = match node.metadata.attributes.get("variance").map(|s| s.as_str()) {
            Some("covariant") => GenericVariance::Covariant,
            Some("contravariant") => GenericVariance::Contravariant,
            _ => GenericVariance::Invariant,
        };

        Ok(Some(GenericParameter {
            name: name.clone(),
            bounds,
            variance,
        }))
    }

    /// Parse type signature with language-specific handling
    fn parse_type_signature(&self, type_str: &str) -> Result<TypeSignature> {
        TypeSignature::parse(type_str).map_err(|e| anyhow!(e))
    }

    /// Extract visibility from function node
    fn extract_visibility(&self, node: &ASTNode) -> Visibility {
        match node
            .metadata
            .attributes
            .get("visibility")
            .map(|s| s.as_str())
        {
            Some("public") => Visibility::Public,
            Some("private") => Visibility::Private,
            Some("protected") => Visibility::Protected,
            Some("internal") => Visibility::Internal,
            _ => match self.language {
                Language::Java => Visibility::Package,      // Default in Java
                Language::Python => Visibility::Public,     // Default in Python
                Language::JavaScript => Visibility::Public, // Default in JS
                Language::Cpp | Language::C => Visibility::Public, // Default in C/C++
                _ => Visibility::Public,
            },
        }
    }

    /// Extract modifiers from function node
    fn extract_modifiers(&self, node: &ASTNode) -> Vec<String> {
        node.metadata
            .attributes
            .get("modifiers")
            .map(|s| s.split(',').map(|m| m.trim().to_string()).collect())
            .unwrap_or_default()
    }

    /// Extract annotations from function node
    fn extract_annotations(&self, node: &ASTNode) -> Vec<String> {
        node.metadata
            .attributes
            .get("annotations")
            .map(|s| s.split(',').map(|a| a.trim().to_string()).collect())
            .unwrap_or_default()
    }

    /// Extract annotations from parameter node
    fn extract_parameter_annotations(&self, node: &ASTNode) -> Vec<String> {
        node.metadata
            .attributes
            .get("annotations")
            .map(|s| s.split(',').map(|a| a.trim().to_string()).collect())
            .unwrap_or_default()
    }

    /// Determine function type from node and name
    fn determine_function_type(&self, node: &ASTNode, name: &str) -> FunctionType {
        match node.node_type {
            NodeType::Constructor => FunctionType::Constructor,
            NodeType::Method => {
                if node.metadata.attributes.get("static").is_some() {
                    FunctionType::StaticMethod
                } else if name.starts_with("get") && name.len() > 3 {
                    FunctionType::Getter
                } else if name.starts_with("set") && name.len() > 3 {
                    FunctionType::Setter
                } else if name.starts_with("operator") {
                    FunctionType::Operator
                } else {
                    FunctionType::Method
                }
            }
            NodeType::Function => {
                if name == "lambda" || name.starts_with("lambda") {
                    FunctionType::Lambda
                } else {
                    FunctionType::Function
                }
            }
            _ => FunctionType::Function,
        }
    }

    /// Calculate complexity metrics for a function
    fn calculate_complexity_metrics(&self, node: &ASTNode) -> Result<FunctionComplexityMetrics> {
        let mut cyclomatic_complexity = 1; // Base complexity
        let mut cognitive_complexity = 0;
        let mut nesting_depth = 0;
        let mut max_nesting_depth = 0;
        let mut branch_count = 0;
        let mut loop_count = 0;
        let mut call_count = 0;

        // Calculate metrics recursively
        self.calculate_complexity_recursive(
            node,
            &mut cyclomatic_complexity,
            &mut cognitive_complexity,
            &mut nesting_depth,
            &mut max_nesting_depth,
            &mut branch_count,
            &mut loop_count,
            &mut call_count,
        );

        // Calculate lines of code
        let lines_of_code = node
            .metadata
            .attributes
            .get("end_line")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(node.metadata.line)
            .saturating_sub(node.metadata.line)
            + 1;

        // Count parameters
        let parameter_count = self.count_parameters(node);

        Ok(FunctionComplexityMetrics {
            cyclomatic_complexity,
            cognitive_complexity,
            lines_of_code,
            parameter_count,
            nesting_depth: max_nesting_depth,
            branch_count,
            loop_count,
            call_count,
        })
    }

    /// Recursively calculate complexity metrics
    fn calculate_complexity_recursive(
        &self,
        node: &ASTNode,
        cyclomatic: &mut usize,
        cognitive: &mut usize,
        current_depth: &mut usize,
        max_depth: &mut usize,
        branches: &mut usize,
        loops: &mut usize,
        calls: &mut usize,
    ) {
        match node.node_type {
            // Control flow nodes increase complexity
            NodeType::IfStatement => {
                *cyclomatic += 1;
                *cognitive += 1 + *current_depth;
                *branches += 1;
            }
            NodeType::WhileLoop | NodeType::ForLoop | NodeType::DoWhileLoop => {
                *cyclomatic += 1;
                *cognitive += 1 + *current_depth;
                *loops += 1;
                *current_depth += 1;
            }
            NodeType::SwitchStatement => {
                *branches += 1;
                // Count case statements
                let case_count = node
                    .children
                    .iter()
                    .filter(|child| child.node_type == NodeType::CaseStatement)
                    .count();
                *cyclomatic += case_count.max(1);
                *cognitive += case_count + *current_depth;
            }
            NodeType::TryStatement => {
                *cyclomatic += 1;
                *cognitive += 1 + *current_depth;
                *current_depth += 1;
            }
            NodeType::CallExpression => {
                *calls += 1;
            }
            _ => {}
        }

        // Update max nesting depth
        *max_depth = (*max_depth).max(*current_depth);

        // Process children
        let old_depth = *current_depth;
        for child in &node.children {
            self.calculate_complexity_recursive(
                child,
                cyclomatic,
                cognitive,
                current_depth,
                max_depth,
                branches,
                loops,
                calls,
            );
        }
        *current_depth = old_depth;
    }

    /// Count parameters in a function node
    fn count_parameters(&self, node: &ASTNode) -> usize {
        for child in &node.children {
            if matches!(
                child.node_type,
                NodeType::ParameterList | NodeType::Parameters
            ) {
                return child.children.len();
            }
        }
        0
    }

    /// Extract function dependencies (called functions)
    fn extract_function_dependencies(&self, node: &ASTNode) -> Vec<String> {
        let mut dependencies = Vec::new();
        self.extract_dependencies_recursive(node, &mut dependencies);
        dependencies.sort();
        dependencies.dedup();
        dependencies
    }

    /// Recursively extract function call dependencies
    fn extract_dependencies_recursive(&self, node: &ASTNode, dependencies: &mut Vec<String>) {
        if node.node_type == NodeType::CallExpression {
            if let Some(function_name) = node.metadata.attributes.get("function_name") {
                dependencies.push(function_name.clone());
            }
        }

        for child in &node.children {
            self.extract_dependencies_recursive(child, dependencies);
        }
    }

    /// Generate signature hash for exact matching
    fn generate_signature_hash(
        &self,
        name: &str,
        parameters: &[FunctionParameter],
        return_type: &TypeSignature,
        modifiers: &[String],
    ) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);

        for param in parameters {
            param.name.hash(&mut hasher);
            param.param_type.to_string().hash(&mut hasher);
            param.is_optional.hash(&mut hasher);
            param.is_varargs.hash(&mut hasher);
        }

        return_type.to_string().hash(&mut hasher);

        for modifier in modifiers {
            modifier.hash(&mut hasher);
        }

        format!("{:x}", hasher.finish())
    }

    /// Generate normalized hash for similarity matching
    fn generate_normalized_hash(
        &self,
        name: &str,
        parameters: &[FunctionParameter],
        return_type: &TypeSignature,
    ) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        // Normalize name (lowercase, remove underscores)
        let normalized_name = name.to_lowercase().replace('_', "");
        normalized_name.hash(&mut hasher);

        // Hash parameter types only (ignore names)
        for param in parameters {
            param.param_type.base_type.to_lowercase().hash(&mut hasher);
            param.param_type.array_dimensions.hash(&mut hasher);
        }

        // Hash return type
        return_type.base_type.to_lowercase().hash(&mut hasher);
        return_type.array_dimensions.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }

    /// Check if function should be included based on configuration
    fn should_include_function(&self, signature: &EnhancedFunctionSignature) -> bool {
        // Check visibility
        if !self.config.include_private && signature.visibility == Visibility::Private {
            return false;
        }

        // Check static
        if !self.config.include_static && signature.modifiers.contains(&"static".to_string()) {
            return false;
        }

        // Check abstract
        if !self.config.include_abstract && signature.modifiers.contains(&"abstract".to_string()) {
            return false;
        }

        // Check constructors
        if !self.config.include_constructors && signature.function_type == FunctionType::Constructor
        {
            return false;
        }

        // Check accessors
        if !self.config.include_accessors
            && (signature.function_type == FunctionType::Getter
                || signature.function_type == FunctionType::Setter)
        {
            return false;
        }

        // Check parameter count
        if signature.parameters.len() > self.config.max_parameter_count {
            return false;
        }

        true
    }

    /// Build function hierarchy for inheritance analysis
    fn build_function_hierarchy(
        &self,
        signatures: &[EnhancedFunctionSignature],
        hierarchy: &mut HashMap<String, Vec<String>>,
    ) {
        // Group functions by class/namespace
        let mut class_functions: HashMap<String, Vec<&EnhancedFunctionSignature>> = HashMap::new();

        for signature in signatures {
            let class_name = if let Some(dot_pos) = signature.qualified_name.rfind('.') {
                signature.qualified_name[..dot_pos].to_string()
            } else {
                "global".to_string()
            };

            class_functions
                .entry(class_name)
                .or_insert_with(Vec::new)
                .push(signature);
        }

        // Build hierarchy relationships
        for (class_name, functions) in class_functions {
            let function_names: Vec<String> = functions.iter().map(|f| f.name.clone()).collect();
            hierarchy.insert(class_name, function_names);
        }
    }

    /// Calculate extraction statistics
    fn calculate_extraction_stats(
        &self,
        signatures: &[EnhancedFunctionSignature],
        overloaded: &HashMap<String, Vec<EnhancedFunctionSignature>>,
    ) -> ExtractionStats {
        let total_functions = signatures.len();
        let mut public_functions = 0;
        let mut private_functions = 0;
        let mut static_functions = 0;
        let mut abstract_functions = 0;
        let mut constructors = 0;
        let mut generic_functions = 0;
        let mut complex_functions = 0;

        for signature in signatures {
            match signature.visibility {
                Visibility::Public => public_functions += 1,
                Visibility::Private => private_functions += 1,
                _ => {}
            }

            if signature.modifiers.contains(&"static".to_string()) {
                static_functions += 1;
            }

            if signature.modifiers.contains(&"abstract".to_string()) {
                abstract_functions += 1;
            }

            if signature.function_type == FunctionType::Constructor {
                constructors += 1;
            }

            if !signature.generic_parameters.is_empty() {
                generic_functions += 1;
            }

            if let Some(metrics) = &signature.complexity_metrics {
                if metrics.cyclomatic_complexity > 10 || metrics.cognitive_complexity > 15 {
                    complex_functions += 1;
                }
            }
        }

        let overloaded_functions = overloaded
            .values()
            .filter(|overloads| overloads.len() > 1)
            .count();

        ExtractionStats {
            total_functions,
            public_functions,
            private_functions,
            static_functions,
            abstract_functions,
            constructors,
            overloaded_functions,
            generic_functions,
            complex_functions,
        }
    }

    /// Calculate similarity between two function signatures
    pub fn calculate_similarity(
        &self,
        sig1: &EnhancedFunctionSignature,
        sig2: &EnhancedFunctionSignature,
    ) -> FunctionSignatureSimilarity {
        // Weight factors for different aspects
        const NAME_WEIGHT: f64 = 0.4;
        const PARAMETER_WEIGHT: f64 = 0.3;
        const RETURN_TYPE_WEIGHT: f64 = 0.2;
        const MODIFIER_WEIGHT: f64 = 0.1;

        // Calculate individual similarities
        let name_similarity = self.calculate_name_similarity(&sig1.name, &sig2.name);
        let parameter_similarity =
            self.calculate_parameter_similarity(&sig1.parameters, &sig2.parameters);
        let return_type_similarity =
            self.calculate_return_type_similarity(&sig1.return_type, &sig2.return_type);
        let modifier_similarity =
            self.calculate_modifier_similarity(&sig1.modifiers, &sig2.modifiers);
        let complexity_similarity = self
            .calculate_complexity_similarity(&sig1.complexity_metrics, &sig2.complexity_metrics);

        // Calculate weighted overall similarity
        let overall_similarity = (name_similarity * NAME_WEIGHT)
            + (parameter_similarity * PARAMETER_WEIGHT)
            + (return_type_similarity * RETURN_TYPE_WEIGHT)
            + (modifier_similarity * MODIFIER_WEIGHT);

        // Determine if it's a potential match
        let is_potential_match =
            overall_similarity > 0.7 || (name_similarity > 0.8 && parameter_similarity > 0.6);

        // Build detailed breakdown
        let similarity_breakdown = self.build_similarity_breakdown(sig1, sig2);

        FunctionSignatureSimilarity {
            overall_similarity,
            name_similarity,
            parameter_similarity,
            return_type_similarity,
            modifier_similarity,
            complexity_similarity,
            is_potential_match,
            similarity_breakdown,
        }
    }

    /// Calculate name similarity using edit distance
    fn calculate_name_similarity(&self, name1: &str, name2: &str) -> f64 {
        if name1 == name2 {
            return 1.0;
        }

        // Normalize names for comparison
        let norm1 = name1.to_lowercase().replace('_', "");
        let norm2 = name2.to_lowercase().replace('_', "");

        if norm1 == norm2 {
            return 0.95; // High similarity for normalized match
        }

        // Use edit distance for partial similarity
        let distance = edit_distance::edit_distance(&norm1, &norm2);
        let max_len = norm1.len().max(norm2.len());

        if max_len == 0 {
            return 1.0;
        }

        (1.0 - (distance as f64 / max_len as f64)).max(0.0_f64)
    }

    /// Calculate parameter similarity
    fn calculate_parameter_similarity(
        &self,
        params1: &[FunctionParameter],
        params2: &[FunctionParameter],
    ) -> f64 {
        if params1.is_empty() && params2.is_empty() {
            return 1.0;
        }

        if params1.len() != params2.len() {
            // Penalize different parameter counts
            let max_len = params1.len().max(params2.len());
            let min_len = params1.len().min(params2.len());
            let count_similarity = min_len as f64 / max_len as f64;

            // Still compare available parameters
            let mut type_similarity = 0.0;
            let compare_count = min_len;

            for i in 0..compare_count {
                type_similarity += TypeEquivalence::calculate_type_similarity(
                    &params1[i].param_type,
                    &params2[i].param_type,
                );
            }

            if compare_count > 0 {
                type_similarity /= compare_count as f64;
            }

            return (count_similarity * 0.5) + (type_similarity * 0.5);
        }

        // Same parameter count - compare types
        let mut total_similarity = 0.0;

        for (param1, param2) in params1.iter().zip(params2.iter()) {
            let type_sim =
                TypeEquivalence::calculate_type_similarity(&param1.param_type, &param2.param_type);

            // Bonus for matching optional/varargs flags
            let flag_bonus = if param1.is_optional == param2.is_optional
                && param1.is_varargs == param2.is_varargs
            {
                0.1
            } else {
                0.0
            };

            total_similarity += type_sim + flag_bonus;
        }

        (total_similarity / params1.len() as f64).min(1.0)
    }

    /// Calculate return type similarity
    fn calculate_return_type_similarity(
        &self,
        return1: &TypeSignature,
        return2: &TypeSignature,
    ) -> f64 {
        TypeEquivalence::calculate_type_similarity(return1, return2)
    }

    /// Calculate modifier similarity
    fn calculate_modifier_similarity(&self, modifiers1: &[String], modifiers2: &[String]) -> f64 {
        if modifiers1.is_empty() && modifiers2.is_empty() {
            return 1.0;
        }

        let set1: HashSet<_> = modifiers1.iter().collect();
        let set2: HashSet<_> = modifiers2.iter().collect();

        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();

        if union == 0 {
            return 1.0;
        }

        intersection as f64 / union as f64
    }

    /// Calculate complexity similarity
    fn calculate_complexity_similarity(
        &self,
        metrics1: &Option<FunctionComplexityMetrics>,
        metrics2: &Option<FunctionComplexityMetrics>,
    ) -> f64 {
        match (metrics1, metrics2) {
            (Some(m1), Some(m2)) => {
                // Compare various complexity metrics
                let cyclomatic_sim = 1.0
                    - ((m1.cyclomatic_complexity as i32 - m2.cyclomatic_complexity as i32).abs()
                        as f64
                        / 20.0)
                        .min(1.0);
                let cognitive_sim = 1.0
                    - ((m1.cognitive_complexity as i32 - m2.cognitive_complexity as i32).abs()
                        as f64
                        / 30.0)
                        .min(1.0);
                let loc_sim = 1.0
                    - ((m1.lines_of_code as i32 - m2.lines_of_code as i32).abs() as f64 / 100.0)
                        .min(1.0);

                (cyclomatic_sim + cognitive_sim + loc_sim) / 3.0
            }
            (None, None) => 1.0,
            _ => 0.5, // One has metrics, other doesn't
        }
    }

    /// Build detailed similarity breakdown
    fn build_similarity_breakdown(
        &self,
        sig1: &EnhancedFunctionSignature,
        sig2: &EnhancedFunctionSignature,
    ) -> SimilarityBreakdown {
        let exact_name_match = sig1.name == sig2.name;
        let parameter_count_match = sig1.parameters.len() == sig2.parameters.len();

        let parameter_types_match = sig1
            .parameters
            .iter()
            .zip(sig2.parameters.iter())
            .map(|(p1, p2)| {
                TypeEquivalence::are_complex_types_equivalent(&p1.param_type, &p2.param_type)
            })
            .collect();

        let return_type_match =
            TypeEquivalence::are_complex_types_equivalent(&sig1.return_type, &sig2.return_type);
        let visibility_match = sig1.visibility == sig2.visibility;

        let static_match = sig1.modifiers.contains(&"static".to_string())
            == sig2.modifiers.contains(&"static".to_string());

        let generic_parameters_match =
            sig1.generic_parameters.len() == sig2.generic_parameters.len();

        SimilarityBreakdown {
            exact_name_match,
            parameter_count_match,
            parameter_types_match,
            return_type_match,
            visibility_match,
            static_match,
            generic_parameters_match,
        }
    }

    /// Find similar functions in a collection
    pub fn find_similar_functions(
        &self,
        target: &EnhancedFunctionSignature,
        candidates: &[EnhancedFunctionSignature],
        min_similarity: f64,
    ) -> Vec<(EnhancedFunctionSignature, FunctionSignatureSimilarity)> {
        let mut similar_functions = Vec::new();

        for candidate in candidates {
            if candidate.qualified_name != target.qualified_name {
                let similarity = self.calculate_similarity(target, candidate);
                if similarity.overall_similarity >= min_similarity {
                    similar_functions.push((candidate.clone(), similarity));
                }
            }
        }

        // Sort by similarity (highest first)
        similar_functions.sort_by(|a, b| {
            b.1.overall_similarity
                .partial_cmp(&a.1.overall_similarity)
                .unwrap()
        });

        similar_functions
    }

    /// Find exact matches by signature hash
    pub fn find_exact_matches(
        &self,
        target: &EnhancedFunctionSignature,
        candidates: &[EnhancedFunctionSignature],
    ) -> Vec<EnhancedFunctionSignature> {
        candidates
            .iter()
            .filter(|candidate| {
                candidate.signature_hash == target.signature_hash
                    && candidate.qualified_name != target.qualified_name
            })
            .cloned()
            .collect()
    }

    /// Find potential renames by normalized hash
    pub fn find_potential_renames(
        &self,
        target: &EnhancedFunctionSignature,
        candidates: &[EnhancedFunctionSignature],
    ) -> Vec<EnhancedFunctionSignature> {
        candidates
            .iter()
            .filter(|candidate| {
                candidate.normalized_hash == target.normalized_hash
                    && candidate.qualified_name != target.qualified_name
                    && candidate.name != target.name
            })
            .cloned()
            .collect()
    }
}
