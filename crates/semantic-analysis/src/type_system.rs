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
    
    fn normalize_type(type_name: &str) -> String {
        match type_name {
            "int" | "Int32" | "i32" | "integer" => "int".to_string(),
            "long" | "Int64" | "i64" => "long".to_string(),
            "float" | "Float32" | "f32" => "float".to_string(),
            "double" | "Float64" | "f64" => "double".to_string(),
            "str" | "String" | "string" => "string".to_string(),
            "bool" | "Boolean" | "boolean" => "bool".to_string(),
            _ => type_name.to_string(),
        }
    }
}
