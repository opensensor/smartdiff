//! Type information extraction from AST nodes

use crate::type_system::ParameterInfo;
use crate::{
    FieldInfo, MethodInfo, Symbol, SymbolKind, SymbolTable, TypeInfo, TypeKind, TypeResolver,
    TypeSignature, Visibility,
};
use anyhow::{anyhow, Result};
use smart_diff_parser::{ASTNode, Language, NodeType, ParseResult};
use std::collections::{HashMap, HashSet};

/// Configuration for type extraction
#[derive(Debug, Clone)]
pub struct TypeExtractorConfig {
    /// Whether to extract method signatures
    pub extract_methods: bool,
    /// Whether to extract field information
    pub extract_fields: bool,
    /// Whether to resolve inheritance relationships
    pub resolve_inheritance: bool,
    /// Whether to extract generic type parameters
    pub extract_generics: bool,
    /// Maximum depth for type resolution
    pub max_resolution_depth: usize,
}

impl Default for TypeExtractorConfig {
    fn default() -> Self {
        Self {
            extract_methods: true,
            extract_fields: true,
            resolve_inheritance: true,
            extract_generics: true,
            max_resolution_depth: 10,
        }
    }
}

/// Type information extractor
pub struct TypeExtractor {
    config: TypeExtractorConfig,
    type_resolver: TypeResolver,
    language: Language,
    current_file: String,
}

/// Extracted type information with relationships
#[derive(Debug, Clone)]
pub struct ExtractedTypeInfo {
    pub type_info: TypeInfo,
    pub inheritance: Vec<String>,
    pub implementations: Vec<String>,
    pub dependencies: HashSet<String>,
    pub generic_constraints: HashMap<String, Vec<String>>,
}

/// Type extraction result
#[derive(Debug)]
pub struct TypeExtractionResult {
    pub types: Vec<ExtractedTypeInfo>,
    pub type_aliases: HashMap<String, String>,
    pub primitive_usage: HashMap<String, usize>,
    pub generic_usage: HashMap<String, usize>,
}

impl TypeExtractor {
    pub fn new(language: Language, config: TypeExtractorConfig) -> Self {
        Self {
            config,
            type_resolver: TypeResolver::new(),
            language,
            current_file: String::new(),
        }
    }

    pub fn with_defaults(language: Language) -> Self {
        Self::new(language, TypeExtractorConfig::default())
    }

    /// Extract type information from a parse result
    pub fn extract_types(
        &mut self,
        file_path: &str,
        parse_result: &ParseResult,
    ) -> Result<TypeExtractionResult> {
        self.current_file = file_path.to_string();

        let mut result = TypeExtractionResult {
            types: Vec::new(),
            type_aliases: HashMap::new(),
            primitive_usage: HashMap::new(),
            generic_usage: HashMap::new(),
        };

        self.extract_from_node(&parse_result.ast, &mut result)?;

        Ok(result)
    }

    /// Extract type information from multiple files
    pub fn extract_types_from_files(
        &mut self,
        files: Vec<(String, ParseResult)>,
    ) -> Result<TypeExtractionResult> {
        let mut combined_result = TypeExtractionResult {
            types: Vec::new(),
            type_aliases: HashMap::new(),
            primitive_usage: HashMap::new(),
            generic_usage: HashMap::new(),
        };

        for (file_path, parse_result) in files {
            let file_result = self.extract_types(&file_path, &parse_result)?;
            self.merge_results(&mut combined_result, file_result);
        }

        Ok(combined_result)
    }

    /// Extract type information from AST node
    fn extract_from_node(
        &mut self,
        node: &ASTNode,
        result: &mut TypeExtractionResult,
    ) -> Result<()> {
        match node.node_type {
            NodeType::Class => {
                if let Some(type_info) = self.extract_class_info(node)? {
                    result.types.push(type_info);
                }
            }
            NodeType::Interface => {
                if let Some(type_info) = self.extract_interface_info(node)? {
                    result.types.push(type_info);
                }
            }
            NodeType::Enum => {
                if let Some(type_info) = self.extract_enum_info(node)? {
                    result.types.push(type_info);
                }
            }
            NodeType::Struct => {
                if let Some(type_info) = self.extract_struct_info(node)? {
                    result.types.push(type_info);
                }
            }
            NodeType::TypeAlias => {
                self.extract_type_alias(node, result)?;
            }
            _ => {}
        }

        // Recursively process children
        for child in &node.children {
            self.extract_from_node(child, result)?;
        }

        Ok(())
    }

    /// Extract class type information
    fn extract_class_info(&mut self, node: &ASTNode) -> Result<Option<ExtractedTypeInfo>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .ok_or_else(|| anyhow!("Class node missing name"))?;

        let mut type_info = TypeInfo {
            name: name.clone(),
            kind: TypeKind::Class,
            generic_parameters: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            file_path: self.current_file.clone(),
            line: node.metadata.line,
        };

        let mut extracted = ExtractedTypeInfo {
            type_info,
            inheritance: Vec::new(),
            implementations: Vec::new(),
            dependencies: HashSet::new(),
            generic_constraints: HashMap::new(),
        };

        // Extract generic parameters
        if self.config.extract_generics {
            self.extract_generic_parameters(node, &mut extracted)?;
        }

        // Extract inheritance information
        if self.config.resolve_inheritance {
            self.extract_inheritance_info(node, &mut extracted)?;
        }

        // Extract fields
        if self.config.extract_fields {
            self.extract_class_fields(node, &mut extracted)?;
        }

        // Extract methods
        if self.config.extract_methods {
            self.extract_class_methods(node, &mut extracted)?;
        }

        Ok(Some(extracted))
    }

    /// Extract interface type information
    fn extract_interface_info(&mut self, node: &ASTNode) -> Result<Option<ExtractedTypeInfo>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .ok_or_else(|| anyhow!("Interface node missing name"))?;

        let mut type_info = TypeInfo {
            name: name.clone(),
            kind: TypeKind::Interface,
            generic_parameters: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            file_path: self.current_file.clone(),
            line: node.metadata.line,
        };

        let mut extracted = ExtractedTypeInfo {
            type_info,
            inheritance: Vec::new(),
            implementations: Vec::new(),
            dependencies: HashSet::new(),
            generic_constraints: HashMap::new(),
        };

        // Extract generic parameters
        if self.config.extract_generics {
            self.extract_generic_parameters(node, &mut extracted)?;
        }

        // Extract interface methods
        if self.config.extract_methods {
            self.extract_interface_methods(node, &mut extracted)?;
        }

        Ok(Some(extracted))
    }

    /// Extract enum type information
    fn extract_enum_info(&mut self, node: &ASTNode) -> Result<Option<ExtractedTypeInfo>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .ok_or_else(|| anyhow!("Enum node missing name"))?;

        let type_info = TypeInfo {
            name: name.clone(),
            kind: TypeKind::Enum,
            generic_parameters: Vec::new(),
            fields: self.extract_enum_values(node)?,
            methods: Vec::new(),
            file_path: self.current_file.clone(),
            line: node.metadata.line,
        };

        let extracted = ExtractedTypeInfo {
            type_info,
            inheritance: Vec::new(),
            implementations: Vec::new(),
            dependencies: HashSet::new(),
            generic_constraints: HashMap::new(),
        };

        Ok(Some(extracted))
    }

    /// Extract struct type information
    fn extract_struct_info(&mut self, node: &ASTNode) -> Result<Option<ExtractedTypeInfo>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .ok_or_else(|| anyhow!("Struct node missing name"))?;

        let mut type_info = TypeInfo {
            name: name.clone(),
            kind: TypeKind::Struct,
            generic_parameters: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            file_path: self.current_file.clone(),
            line: node.metadata.line,
        };

        let mut extracted = ExtractedTypeInfo {
            type_info,
            inheritance: Vec::new(),
            implementations: Vec::new(),
            dependencies: HashSet::new(),
            generic_constraints: HashMap::new(),
        };

        // Extract fields
        if self.config.extract_fields {
            self.extract_struct_fields(node, &mut extracted)?;
        }

        Ok(Some(extracted))
    }

    /// Extract generic parameters from a type node
    fn extract_generic_parameters(
        &mut self,
        node: &ASTNode,
        extracted: &mut ExtractedTypeInfo,
    ) -> Result<()> {
        // Look for generic parameter declarations
        for child in &node.children {
            if child.node_type == NodeType::GenericParameter {
                if let Some(param_name) = child.metadata.attributes.get("name") {
                    extracted
                        .type_info
                        .generic_parameters
                        .push(param_name.clone());

                    // Extract constraints if available
                    if let Some(constraints) = child.metadata.attributes.get("constraints") {
                        let constraint_list: Vec<String> = constraints
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                        extracted
                            .generic_constraints
                            .insert(param_name.clone(), constraint_list);
                    }
                }
            }
        }

        Ok(())
    }

    /// Extract inheritance information
    fn extract_inheritance_info(
        &mut self,
        node: &ASTNode,
        extracted: &mut ExtractedTypeInfo,
    ) -> Result<()> {
        // Look for inheritance/extends clauses
        if let Some(extends) = node.metadata.attributes.get("extends") {
            extracted.inheritance.push(extends.clone());
            extracted.dependencies.insert(extends.clone());
        }

        // Look for interface implementations
        if let Some(implements) = node.metadata.attributes.get("implements") {
            let interfaces: Vec<String> = implements
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            for interface in interfaces {
                extracted.implementations.push(interface.clone());
                extracted.dependencies.insert(interface);
            }
        }

        Ok(())
    }

    /// Extract class fields
    fn extract_class_fields(
        &mut self,
        node: &ASTNode,
        extracted: &mut ExtractedTypeInfo,
    ) -> Result<()> {
        for child in &node.children {
            if child.node_type == NodeType::FieldDeclaration {
                if let Some(field_info) = self.extract_field_info(child)? {
                    extracted.type_info.fields.push(field_info);
                }
            }
        }

        Ok(())
    }

    /// Extract struct fields
    fn extract_struct_fields(
        &mut self,
        node: &ASTNode,
        extracted: &mut ExtractedTypeInfo,
    ) -> Result<()> {
        // Similar to class fields but with different visibility rules
        self.extract_class_fields(node, extracted)
    }

    /// Extract enum values as fields
    fn extract_enum_values(&mut self, node: &ASTNode) -> Result<Vec<FieldInfo>> {
        let mut values = Vec::new();

        for child in &node.children {
            if child.node_type == NodeType::EnumValue {
                if let Some(name) = child.metadata.attributes.get("name") {
                    let field_info = FieldInfo {
                        name: name.clone(),
                        type_name: "enum_value".to_string(), // Enum value type
                        visibility: Visibility::Public,
                        is_static: true,
                        is_final: true,
                    };
                    values.push(field_info);
                }
            }
        }

        Ok(values)
    }

    /// Extract field information from a field declaration node
    fn extract_field_info(&mut self, node: &ASTNode) -> Result<Option<FieldInfo>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .ok_or_else(|| anyhow!("Field node missing name"))?;

        let field_type = node
            .metadata
            .attributes
            .get("type")
            .unwrap_or(&"unknown".to_string())
            .clone();

        let visibility = self.extract_visibility(node);
        let is_static = node.metadata.attributes.get("static").is_some();
        let is_final = node.metadata.attributes.get("final").is_some()
            || node.metadata.attributes.get("const").is_some();

        let field_info = FieldInfo {
            name: name.clone(),
            type_name: field_type,
            visibility,
            is_static,
            is_final,
        };

        Ok(Some(field_info))
    }

    /// Extract class methods
    fn extract_class_methods(
        &mut self,
        node: &ASTNode,
        extracted: &mut ExtractedTypeInfo,
    ) -> Result<()> {
        for child in &node.children {
            if matches!(child.node_type, NodeType::Method | NodeType::Constructor) {
                if let Some(method_info) = self.extract_method_info(child)? {
                    extracted.type_info.methods.push(method_info);
                }
            }
        }

        Ok(())
    }

    /// Extract interface methods
    fn extract_interface_methods(
        &mut self,
        node: &ASTNode,
        extracted: &mut ExtractedTypeInfo,
    ) -> Result<()> {
        for child in &node.children {
            if child.node_type == NodeType::Method {
                if let Some(method_info) = self.extract_method_info(child)? {
                    extracted.type_info.methods.push(method_info);
                }
            }
        }

        Ok(())
    }

    /// Extract method information from a method declaration node
    fn extract_method_info(&mut self, node: &ASTNode) -> Result<Option<MethodInfo>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .ok_or_else(|| anyhow!("Method node missing name"))?;

        let return_type = node
            .metadata
            .attributes
            .get("return_type")
            .unwrap_or(&"void".to_string())
            .clone();

        let visibility = self.extract_visibility(node);
        let is_static = node.metadata.attributes.get("static").is_some();
        let is_abstract = node.metadata.attributes.get("abstract").is_some();
        let is_final = node.metadata.attributes.get("final").is_some();

        // Extract parameters
        let parameters = self.extract_method_parameters(node)?;

        let method_info = MethodInfo {
            name: name.clone(),
            return_type,
            parameters,
            visibility,
            is_static,
            is_abstract,
        };

        Ok(Some(method_info))
    }

    /// Extract method parameters
    fn extract_method_parameters(&mut self, node: &ASTNode) -> Result<Vec<ParameterInfo>> {
        let mut parameters = Vec::new();

        for child in &node.children {
            if child.node_type == NodeType::ParameterDeclaration {
                if let Some(param_type) = child.metadata.attributes.get("type") {
                    let param_name = child
                        .metadata
                        .attributes
                        .get("name")
                        .map(|s| s.as_str())
                        .unwrap_or("param");

                    let param_info = ParameterInfo {
                        name: param_name.to_string(),
                        type_name: param_type.clone(),
                        is_optional: false, // Default value, could be extracted from attributes
                    };
                    parameters.push(param_info);
                }
            }
        }

        Ok(parameters)
    }

    /// Extract visibility from node attributes
    fn extract_visibility(&self, node: &ASTNode) -> Visibility {
        if node.metadata.attributes.contains_key("public") {
            Visibility::Public
        } else if node.metadata.attributes.contains_key("private") {
            Visibility::Private
        } else if node.metadata.attributes.contains_key("protected") {
            Visibility::Protected
        } else {
            match self.language {
                Language::Java => Visibility::Package,
                Language::Cpp | Language::C => Visibility::Public,
                _ => Visibility::Public,
            }
        }
    }

    /// Extract type alias information
    fn extract_type_alias(
        &mut self,
        node: &ASTNode,
        result: &mut TypeExtractionResult,
    ) -> Result<()> {
        if let Some(alias_name) = node.metadata.attributes.get("name") {
            if let Some(target_type) = node.metadata.attributes.get("target_type") {
                result
                    .type_aliases
                    .insert(alias_name.clone(), target_type.clone());
            }
        }

        Ok(())
    }

    /// Merge two type extraction results
    fn merge_results(&mut self, target: &mut TypeExtractionResult, source: TypeExtractionResult) {
        target.types.extend(source.types);
        target.type_aliases.extend(source.type_aliases);

        // Merge primitive usage counts
        for (primitive, count) in source.primitive_usage {
            *target.primitive_usage.entry(primitive).or_insert(0) += count;
        }

        // Merge generic usage counts
        for (generic, count) in source.generic_usage {
            *target.generic_usage.entry(generic).or_insert(0) += count;
        }
    }

    /// Get type resolver
    pub fn get_type_resolver(&self) -> &TypeResolver {
        &self.type_resolver
    }

    /// Get type resolver (mutable)
    pub fn get_type_resolver_mut(&mut self) -> &mut TypeResolver {
        &mut self.type_resolver
    }

    /// Parse type signature from string with language-specific handling
    pub fn parse_type_signature(&self, type_str: &str) -> Result<TypeSignature> {
        match self.language {
            Language::Java => self.parse_java_type(type_str),
            Language::Python => self.parse_python_type(type_str),
            Language::JavaScript => self.parse_javascript_type(type_str),
            Language::Cpp => self.parse_cpp_type(type_str),
            Language::C => self.parse_c_type(type_str),
            _ => TypeSignature::parse(type_str).map_err(|e| anyhow!(e)),
        }
    }

    /// Parse Java type signature
    fn parse_java_type(&self, type_str: &str) -> Result<TypeSignature> {
        // Handle Java-specific patterns like List<String>, Map<K,V>, etc.
        TypeSignature::parse(type_str).map_err(|e| anyhow!(e))
    }

    /// Parse Python type signature
    fn parse_python_type(&self, type_str: &str) -> Result<TypeSignature> {
        // Handle Python type hints like List[str], Dict[str, int], etc.
        let normalized = type_str.replace('[', "<").replace(']', ">");
        TypeSignature::parse(&normalized).map_err(|e| anyhow!(e))
    }

    /// Parse JavaScript type signature
    fn parse_javascript_type(&self, type_str: &str) -> Result<TypeSignature> {
        // Handle TypeScript/JSDoc types
        TypeSignature::parse(type_str).map_err(|e| anyhow!(e))
    }

    /// Parse C++ type signature
    fn parse_cpp_type(&self, type_str: &str) -> Result<TypeSignature> {
        // Handle C++ templates, pointers, references
        let mut signature = TypeSignature::parse(type_str).map_err(|e| anyhow!(e))?;

        // Handle C++ specific modifiers
        if type_str.contains("const") {
            signature.modifiers.push("const".to_string());
        }
        if type_str.contains("volatile") {
            signature.modifiers.push("volatile".to_string());
        }
        if type_str.contains('*') {
            signature.modifiers.push("pointer".to_string());
        }
        if type_str.contains('&') {
            signature.modifiers.push("reference".to_string());
        }

        Ok(signature)
    }

    /// Parse C type signature
    fn parse_c_type(&self, type_str: &str) -> Result<TypeSignature> {
        // Handle C types with pointers and arrays
        let mut signature = TypeSignature::new(type_str.to_string());

        // Count pointer levels
        let pointer_count = type_str.matches('*').count();
        if pointer_count > 0 {
            signature
                .modifiers
                .push(format!("pointer_{}", pointer_count));
        }

        // Handle const
        if type_str.contains("const") {
            signature.modifiers.push("const".to_string());
        }

        Ok(signature)
    }

    /// Build type dependency graph from extracted types
    pub fn build_type_dependency_graph(
        &self,
        types: &[ExtractedTypeInfo],
    ) -> HashMap<String, Vec<String>> {
        let mut dependencies = HashMap::new();

        for type_info in types {
            let type_name = &type_info.type_info.name;
            let mut type_deps = Vec::new();

            // Add inheritance dependencies
            type_deps.extend(type_info.inheritance.iter().cloned());

            // Add implementation dependencies
            type_deps.extend(type_info.implementations.iter().cloned());

            // Add field type dependencies
            for field in &type_info.type_info.fields {
                if !self.is_primitive_type(&field.type_name) {
                    type_deps.push(field.type_name.clone());
                }
            }

            // Add method parameter and return type dependencies
            for method in &type_info.type_info.methods {
                if !self.is_primitive_type(&method.return_type) {
                    type_deps.push(method.return_type.clone());
                }

                for param in &method.parameters {
                    if !self.is_primitive_type(&param.type_name) {
                        type_deps.push(param.type_name.clone());
                    }
                }
            }

            // Remove duplicates and self-references
            type_deps.sort();
            type_deps.dedup();
            type_deps.retain(|dep| dep != type_name);

            dependencies.insert(type_name.clone(), type_deps);
        }

        dependencies
    }

    /// Check if a type is primitive
    fn is_primitive_type(&self, type_name: &str) -> bool {
        matches!(
            type_name,
            "int"
                | "long"
                | "float"
                | "double"
                | "bool"
                | "char"
                | "byte"
                | "short"
                | "string"
                | "String"
                | "void"
                | "boolean"
                | "Boolean"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "f32"
                | "f64"
                | "str"
                | "None"
                | "null"
                | "undefined"
        )
    }
}
