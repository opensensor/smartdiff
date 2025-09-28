//! Function and signature definitions

use crate::ast::ASTNode;
use serde::{Deserialize, Serialize};

/// Represents a function or method in the code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Function {
    pub signature: FunctionSignature,
    pub body: ASTNode,
    pub dependencies: Vec<String>,
    pub hash: String,
    pub location: FunctionLocation,
}

/// Function signature information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub modifiers: Vec<String>,
    pub generic_parameters: Vec<String>,
}

/// Function parameter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub default_value: Option<String>,
    pub is_variadic: bool,
}

/// Type information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Type {
    pub name: String,
    pub generic_args: Vec<Type>,
    pub is_nullable: bool,
    pub is_array: bool,
    pub array_dimensions: usize,
}

/// Function location in source code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionLocation {
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_column: usize,
    pub end_column: usize,
}

impl Function {
    pub fn new(signature: FunctionSignature, body: ASTNode, file_path: String) -> Self {
        let hash = Self::calculate_hash(&signature, &body);
        let location = FunctionLocation {
            file_path,
            start_line: body.metadata.line,
            end_line: body.metadata.line, // TODO: Calculate actual end line
            start_column: body.metadata.column,
            end_column: body.metadata.column, // TODO: Calculate actual end column
        };

        Self {
            signature,
            body,
            dependencies: Vec::new(),
            hash,
            location,
        }
    }

    /// Calculate a hash for the function based on signature and body structure
    fn calculate_hash(signature: &FunctionSignature, body: &ASTNode) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        signature.name.hash(&mut hasher);
        signature.parameters.len().hash(&mut hasher);
        body.structural_hash().hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }

    /// Get the complexity score of this function
    pub fn complexity_score(&self) -> usize {
        self.count_complexity_nodes(&self.body)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn count_complexity_nodes(&self, node: &ASTNode) -> usize {
        let mut complexity = match node.node_type {
            crate::ast::NodeType::IfStatement => 1,
            crate::ast::NodeType::WhileLoop => 1,
            crate::ast::NodeType::ForLoop => 1,
            _ => 0,
        };

        for child in &node.children {
            complexity += self.count_complexity_nodes(child);
        }

        complexity
    }

    /// Extract function calls from the body
    pub fn extract_function_calls(&self) -> Vec<String> {
        let mut calls = Vec::new();
        self.extract_calls_recursive(&self.body, &mut calls);
        calls
    }

    #[allow(clippy::only_used_in_recursion)]
    fn extract_calls_recursive(&self, node: &ASTNode, calls: &mut Vec<String>) {
        if let crate::ast::NodeType::CallExpression = node.node_type {
            if let Some(name) = node.metadata.attributes.get("function_name") {
                calls.push(name.clone());
            }
        }

        for child in &node.children {
            self.extract_calls_recursive(child, calls);
        }
    }
}

impl FunctionSignature {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parameters: Vec::new(),
            return_type: None,
            modifiers: Vec::new(),
            generic_parameters: Vec::new(),
        }
    }

    /// Calculate similarity score with another signature
    pub fn similarity(&self, other: &FunctionSignature) -> f64 {
        let mut score = 0.0;
        let mut total_weight = 0.0;

        // Name similarity (weight: 0.4)
        let name_weight = 0.4;
        if self.name == other.name {
            score += name_weight;
        } else {
            // Use edit distance for partial similarity
            let distance = edit_distance::edit_distance(&self.name, &other.name);
            let max_len = self.name.len().max(other.name.len());
            if max_len > 0 {
                score += name_weight * (1.0 - (distance as f64 / max_len as f64));
            }
        }
        total_weight += name_weight;

        // Parameter count similarity (weight: 0.3)
        let param_weight = 0.3;
        let param_diff = (self.parameters.len() as i32 - other.parameters.len() as i32).abs();
        let max_params = self.parameters.len().max(other.parameters.len());
        if max_params > 0 {
            score += param_weight * (1.0 - (param_diff as f64 / max_params as f64));
        } else {
            score += param_weight; // Both have no parameters
        }
        total_weight += param_weight;

        // Return type similarity (weight: 0.2)
        let return_weight = 0.2;
        match (&self.return_type, &other.return_type) {
            (Some(t1), Some(t2)) => {
                if t1.name == t2.name {
                    score += return_weight;
                }
            }
            (None, None) => score += return_weight,
            _ => {} // One has return type, other doesn't - no points
        }
        total_weight += return_weight;

        // Modifiers similarity (weight: 0.1)
        let modifier_weight = 0.1;
        let common_modifiers = self
            .modifiers
            .iter()
            .filter(|m| other.modifiers.contains(m))
            .count();
        let total_modifiers = self.modifiers.len().max(other.modifiers.len());
        if total_modifiers > 0 {
            score += modifier_weight * (common_modifiers as f64 / total_modifiers as f64);
        } else {
            score += modifier_weight; // Both have no modifiers
        }
        total_weight += modifier_weight;

        score / total_weight
    }
}

impl Type {
    pub fn new(name: String) -> Self {
        Self {
            name,
            generic_args: Vec::new(),
            is_nullable: false,
            is_array: false,
            array_dimensions: 0,
        }
    }

    /// Check if this type is equivalent to another type
    pub fn is_equivalent(&self, other: &Type) -> bool {
        // Basic name equivalence
        if self.name == other.name {
            return true;
        }

        // Check for common type aliases
        self.normalize_type_name() == other.normalize_type_name()
    }

    fn normalize_type_name(&self) -> String {
        match self.name.as_str() {
            "int" | "Int32" | "i32" => "integer".to_string(),
            "long" | "Int64" | "i64" => "long_integer".to_string(),
            "float" | "Float32" | "f32" => "float".to_string(),
            "double" | "Float64" | "f64" => "double".to_string(),
            "str" | "String" | "string" => "string".to_string(),
            "bool" | "Boolean" | "boolean" => "boolean".to_string(),
            _ => self.name.clone(),
        }
    }
}
