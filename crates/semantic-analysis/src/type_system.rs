//! Type system and type resolution

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type information extracted from code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
    pub generic_parameters: Vec<String>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub file_path: String,
    pub line: usize,
}

/// Types of type definitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TypeKind {
    Class,
    Interface,
    Struct,
    Enum,
    Union,
    Primitive,
    Generic,
    Array,
    Function,
}

/// Field information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_final: bool,
}

/// Method information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MethodInfo {
    pub name: String,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: String,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_abstract: bool,
}

/// Parameter information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterInfo {
    pub name: String,
    pub type_name: String,
    pub is_optional: bool,
}

/// Visibility levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Package,
}

/// Type resolver for handling type equivalence and relationships
pub struct TypeResolver {
    types: HashMap<String, TypeInfo>,
    type_aliases: HashMap<String, String>,
}

/// Type equivalence checker
pub struct TypeEquivalence;

impl TypeResolver {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            type_aliases: HashMap::new(),
        }
    }
    
    pub fn add_type(&mut self, type_info: TypeInfo) {
        self.types.insert(type_info.name.clone(), type_info);
    }
    
    pub fn resolve_type(&self, type_name: &str) -> Option<&TypeInfo> {
        // First check direct lookup
        if let Some(type_info) = self.types.get(type_name) {
            return Some(type_info);
        }
        
        // Check aliases
        if let Some(aliased_name) = self.type_aliases.get(type_name) {
            return self.types.get(aliased_name);
        }
        
        None
    }
}

impl TypeEquivalence {
    /// Check if two types are equivalent
    pub fn are_equivalent(type1: &str, type2: &str) -> bool {
        if type1 == type2 {
            return true;
        }

        // Check common type equivalences
        Self::normalize_type(type1) == Self::normalize_type(type2)
    }

    /// Check if two complex types are equivalent (handles generics, arrays, etc.)
    pub fn are_complex_types_equivalent(type1: &TypeSignature, type2: &TypeSignature) -> bool {
        if type1.base_type != type2.base_type {
            return false;
        }

        // Check generic parameters
        if type1.generic_params.len() != type2.generic_params.len() {
            return false;
        }

        for (param1, param2) in type1.generic_params.iter().zip(type2.generic_params.iter()) {
            if !Self::are_complex_types_equivalent(param1, param2) {
                return false;
            }
        }

        // Check array dimensions
        type1.array_dimensions == type2.array_dimensions
    }

    /// Calculate similarity score between two types (0.0 to 1.0)
    pub fn calculate_type_similarity(type1: &TypeSignature, type2: &TypeSignature) -> f64 {
        if Self::are_complex_types_equivalent(type1, type2) {
            return 1.0;
        }

        // Check base type similarity
        let base_similarity = if Self::are_equivalent(&type1.base_type, &type2.base_type) {
            1.0
        } else if Self::are_related_types(&type1.base_type, &type2.base_type) {
            0.7
        } else {
            0.0
        };

        // Check generic parameter similarity
        let generic_similarity = if type1.generic_params.is_empty() && type2.generic_params.is_empty() {
            1.0
        } else if type1.generic_params.len() == type2.generic_params.len() {
            let mut total_similarity = 0.0;
            for (param1, param2) in type1.generic_params.iter().zip(type2.generic_params.iter()) {
                total_similarity += Self::calculate_type_similarity(param1, param2);
            }
            total_similarity / type1.generic_params.len() as f64
        } else {
            0.3 // Partial credit for different generic arity
        };

        // Check array dimension similarity
        let array_similarity = if type1.array_dimensions == type2.array_dimensions {
            1.0
        } else if type1.array_dimensions > 0 && type2.array_dimensions > 0 {
            0.5 // Both are arrays but different dimensions
        } else {
            0.0
        };

        // Weighted average
        (base_similarity * 0.6) + (generic_similarity * 0.3) + (array_similarity * 0.1)
    }

    /// Check if two types are related (inheritance, interface implementation, etc.)
    fn are_related_types(type1: &str, type2: &str) -> bool {
        // Check for common inheritance patterns
        let numeric_types = ["int", "long", "float", "double", "byte", "short"];
        let string_types = ["string", "String", "str", "char*"];
        let collection_types = ["List", "ArrayList", "Vector", "Array"];

        (numeric_types.contains(&type1) && numeric_types.contains(&type2)) ||
        (string_types.contains(&type1) && string_types.contains(&type2)) ||
        (collection_types.contains(&type1) && collection_types.contains(&type2))
    }

    fn normalize_type(type_name: &str) -> String {
        match type_name {
            "int" | "Int32" | "i32" | "integer" | "Integer" => "int".to_string(),
            "long" | "Int64" | "i64" | "Long" => "long".to_string(),
            "float" | "Float32" | "f32" | "Float" => "float".to_string(),
            "double" | "Float64" | "f64" | "Double" => "double".to_string(),
            "str" | "String" | "string" | "std::string" => "string".to_string(),
            "bool" | "Boolean" | "boolean" => "bool".to_string(),
            "char" | "Character" => "char".to_string(),
            "byte" | "Byte" | "u8" | "i8" => "byte".to_string(),
            "short" | "Short" | "i16" | "u16" => "short".to_string(),
            "void" | "None" | "null" | "undefined" => "void".to_string(),
            _ => type_name.to_string(),
        }
    }
}

/// Complex type signature with generics and array support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeSignature {
    pub base_type: String,
    pub generic_params: Vec<TypeSignature>,
    pub array_dimensions: usize,
    pub is_nullable: bool,
    pub modifiers: Vec<String>, // const, volatile, etc.
}

impl TypeSignature {
    pub fn new(base_type: String) -> Self {
        Self {
            base_type,
            generic_params: Vec::new(),
            array_dimensions: 0,
            is_nullable: false,
            modifiers: Vec::new(),
        }
    }

    pub fn with_generics(mut self, params: Vec<TypeSignature>) -> Self {
        self.generic_params = params;
        self
    }

    pub fn with_array_dimensions(mut self, dimensions: usize) -> Self {
        self.array_dimensions = dimensions;
        self
    }

    pub fn with_nullable(mut self, nullable: bool) -> Self {
        self.is_nullable = nullable;
        self
    }

    pub fn with_modifiers(mut self, modifiers: Vec<String>) -> Self {
        self.modifiers = modifiers;
        self
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        let mut result = self.base_type.clone();

        // Add generic parameters
        if !self.generic_params.is_empty() {
            let params: Vec<String> = self.generic_params.iter()
                .map(|p| p.to_string())
                .collect();
            result.push_str(&format!("<{}>", params.join(", ")));
        }

        // Add array dimensions
        for _ in 0..self.array_dimensions {
            result.push_str("[]");
        }

        // Add nullable indicator
        if self.is_nullable {
            result.push('?');
        }

        result
    }

    /// Parse type signature from string
    pub fn parse(type_str: &str) -> Result<Self, String> {
        let mut signature = TypeSignature::new(String::new());
        let mut chars = type_str.chars().peekable();
        let mut current_token = String::new();

        // Parse base type
        while let Some(&ch) = chars.peek() {
            if ch == '<' || ch == '[' || ch == '?' {
                break;
            }
            current_token.push(chars.next().unwrap());
        }

        signature.base_type = current_token.trim().to_string();

        // Parse generic parameters
        if chars.peek() == Some(&'<') {
            chars.next(); // consume '<'
            signature.generic_params = Self::parse_generic_params(&mut chars)?;
        }

        // Parse array dimensions
        while chars.peek() == Some(&'[') {
            chars.next(); // consume '['
            if chars.next() != Some(']') {
                return Err("Expected ']' after '['".to_string());
            }
            signature.array_dimensions += 1;
        }

        // Parse nullable
        if chars.peek() == Some(&'?') {
            chars.next();
            signature.is_nullable = true;
        }

        Ok(signature)
    }

    fn parse_generic_params(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Vec<TypeSignature>, String> {
        let mut params = Vec::new();
        let mut current_param = String::new();
        let mut depth = 0;

        while let Some(ch) = chars.next() {
            match ch {
                '>' if depth == 0 => break,
                '<' => {
                    depth += 1;
                    current_param.push(ch);
                }
                '>' => {
                    depth -= 1;
                    current_param.push(ch);
                }
                ',' if depth == 0 => {
                    if !current_param.trim().is_empty() {
                        params.push(TypeSignature::parse(current_param.trim())?);
                        current_param.clear();
                    }
                }
                _ => current_param.push(ch),
            }
        }

        if !current_param.trim().is_empty() {
            params.push(TypeSignature::parse(current_param.trim())?);
        }

        Ok(params)
    }
}
