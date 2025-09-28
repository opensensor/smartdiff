//! Type dependency graph construction and analysis

use crate::{
    DependencyEdge, DependencyEdgeType, DependencyGraph, DependencyNode, DependencyNodeType,
    ExtractedTypeInfo, TypeExtractionResult, TypeInfo,
};
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};

/// Type dependency graph builder
pub struct TypeDependencyGraphBuilder {
    dependency_graph: DependencyGraph,
    type_nodes: HashMap<String, NodeIndex>,
    type_info_map: HashMap<String, ExtractedTypeInfo>,
}

/// Type relationship information
#[derive(Debug, Clone)]
pub struct TypeRelationship {
    pub from_type: String,
    pub to_type: String,
    pub relationship_type: TypeRelationshipType,
    pub strength: f64,
}

/// Types of relationships between types
#[derive(Debug, Clone, PartialEq)]
pub enum TypeRelationshipType {
    Inheritance,      // Class extends another class
    Implementation,   // Class implements interface
    Composition,      // Class has field of another type
    Aggregation,      // Class uses another type
    Dependency,       // Class depends on another type
    Association,      // General association
    GenericParameter, // Generic type parameter
}

/// Type dependency analysis result
#[derive(Debug)]
pub struct TypeDependencyAnalysis {
    pub total_types: usize,
    pub inheritance_chains: Vec<Vec<String>>,
    pub circular_dependencies: Vec<Vec<String>>,
    pub coupling_metrics: HashMap<String, TypeCouplingMetrics>,
    pub type_hierarchy_depth: HashMap<String, usize>,
    pub interface_implementations: HashMap<String, Vec<String>>,
}

/// Coupling metrics for a type
#[derive(Debug, Clone)]
pub struct TypeCouplingMetrics {
    pub afferent_coupling: usize, // Number of types that depend on this type
    pub efferent_coupling: usize, // Number of types this type depends on
    pub instability: f64,         // Efferent / (Afferent + Efferent)
    pub abstractness: f64,        // Abstract methods / Total methods
}

impl TypeDependencyGraphBuilder {
    pub fn new() -> Self {
        Self {
            dependency_graph: DependencyGraph::new(),
            type_nodes: HashMap::new(),
            type_info_map: HashMap::new(),
        }
    }

    /// Build type dependency graph from extraction results
    pub fn build_from_extraction_result(
        &mut self,
        result: &TypeExtractionResult,
    ) -> Result<(), String> {
        // First pass: Create nodes for all types
        for extracted_type in &result.types {
            self.add_type_node(&extracted_type.type_info)?;
            self.type_info_map.insert(
                extracted_type.type_info.name.clone(),
                extracted_type.clone(),
            );
        }

        // Second pass: Create edges for relationships
        for extracted_type in &result.types {
            self.add_type_relationships(extracted_type)?;
        }

        Ok(())
    }

    /// Add a type node to the graph
    fn add_type_node(&mut self, type_info: &TypeInfo) -> Result<NodeIndex, String> {
        let node_type = match type_info.kind {
            crate::TypeKind::Class => DependencyNodeType::Class,
            crate::TypeKind::Interface => DependencyNodeType::Class, // Treat as class for graph purposes
            crate::TypeKind::Struct => DependencyNodeType::Class,
            crate::TypeKind::Enum => DependencyNodeType::Class,
            _ => DependencyNodeType::Class,
        };

        let dependency_node = DependencyNode {
            id: type_info.name.clone(),
            name: type_info.name.clone(),
            node_type,
            file_path: type_info.file_path.clone(),
            line: type_info.line,
        };

        let node_index = self.dependency_graph.add_node(dependency_node);
        self.type_nodes.insert(type_info.name.clone(), node_index);

        Ok(node_index)
    }

    /// Add relationships for a type
    fn add_type_relationships(&mut self, extracted_type: &ExtractedTypeInfo) -> Result<(), String> {
        let type_name = &extracted_type.type_info.name;

        // Add inheritance relationships
        for parent_type in &extracted_type.inheritance {
            self.add_relationship(
                type_name,
                parent_type,
                TypeRelationshipType::Inheritance,
                1.0,
            )?;
        }

        // Add implementation relationships
        for interface_type in &extracted_type.implementations {
            self.add_relationship(
                type_name,
                interface_type,
                TypeRelationshipType::Implementation,
                0.9,
            )?;
        }

        // Add field type dependencies
        for field in &extracted_type.type_info.fields {
            if !self.is_primitive_type(&field.type_name) {
                let relationship_type = if field.is_static {
                    TypeRelationshipType::Dependency
                } else {
                    TypeRelationshipType::Composition
                };

                self.add_relationship(type_name, &field.type_name, relationship_type, 0.7)?;
            }
        }

        // Add method parameter and return type dependencies
        for method in &extracted_type.type_info.methods {
            // Return type dependency
            if !self.is_primitive_type(&method.return_type) && method.return_type != "void" {
                self.add_relationship(
                    type_name,
                    &method.return_type,
                    TypeRelationshipType::Dependency,
                    0.5,
                )?;
            }

            // Parameter type dependencies
            for parameter in &method.parameters {
                let param_type = &parameter.type_name;
                if !self.is_primitive_type(param_type) {
                    self.add_relationship(
                        type_name,
                        param_type,
                        TypeRelationshipType::Dependency,
                        0.4,
                    )?;
                }
            }
        }

        // Add generic parameter dependencies
        for (_param_name, constraints) in &extracted_type.generic_constraints {
            for constraint in constraints {
                self.add_relationship(
                    type_name,
                    constraint,
                    TypeRelationshipType::GenericParameter,
                    0.6,
                )?;
            }
        }

        Ok(())
    }

    /// Add a relationship between two types
    fn add_relationship(
        &mut self,
        from_type: &str,
        to_type: &str,
        relationship_type: TypeRelationshipType,
        strength: f64,
    ) -> Result<(), String> {
        // Skip self-references
        if from_type == to_type {
            return Ok(());
        }

        // Skip if target type doesn't exist in our graph
        if !self.type_nodes.contains_key(to_type) {
            return Ok(());
        }

        let edge_type = match relationship_type {
            TypeRelationshipType::Inheritance => DependencyEdgeType::Inherits,
            TypeRelationshipType::Implementation => DependencyEdgeType::Implements,
            TypeRelationshipType::Composition
            | TypeRelationshipType::Aggregation
            | TypeRelationshipType::Dependency => DependencyEdgeType::Uses,
            TypeRelationshipType::Association => DependencyEdgeType::Uses,
            TypeRelationshipType::GenericParameter => DependencyEdgeType::Uses,
        };

        let edge = DependencyEdge {
            edge_type,
            strength,
        };

        self.dependency_graph.add_edge(from_type, to_type, edge);

        Ok(())
    }

    /// Extract parameter type from parameter string
    #[allow(dead_code)]
    fn extract_parameter_type(&self, parameter: &str) -> Option<String> {
        // Handle "name: type" format
        if let Some(colon_pos) = parameter.find(':') {
            let type_part = parameter[colon_pos + 1..].trim();
            Some(self.clean_type_name(type_part))
        } else {
            None
        }
    }

    /// Clean type name by removing generic parameters and array brackets
    #[allow(dead_code)]
    fn clean_type_name(&self, type_name: &str) -> String {
        let mut clean_name = type_name.to_string();

        // Remove generic parameters
        if let Some(generic_start) = clean_name.find('<') {
            clean_name = clean_name[..generic_start].to_string();
        }

        // Remove array brackets
        clean_name = clean_name.replace("[]", "");

        // Remove nullable indicators
        clean_name = clean_name.replace('?', "");

        clean_name.trim().to_string()
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
                | "Object"
        )
    }

    /// Analyze type dependencies
    pub fn analyze_dependencies(&self) -> TypeDependencyAnalysis {
        let mut analysis = TypeDependencyAnalysis {
            total_types: self.type_nodes.len(),
            inheritance_chains: Vec::new(),
            circular_dependencies: Vec::new(),
            coupling_metrics: HashMap::new(),
            type_hierarchy_depth: HashMap::new(),
            interface_implementations: HashMap::new(),
        };

        // Find inheritance chains
        analysis.inheritance_chains = self.find_inheritance_chains();

        // Find circular dependencies
        analysis.circular_dependencies = self.dependency_graph.find_cycles();

        // Calculate coupling metrics
        for type_name in self.type_nodes.keys() {
            let metrics = self.calculate_type_coupling_metrics(type_name);
            analysis.coupling_metrics.insert(type_name.clone(), metrics);
        }

        // Calculate hierarchy depths
        analysis.type_hierarchy_depth = self.calculate_hierarchy_depths();

        // Find interface implementations
        analysis.interface_implementations = self.find_interface_implementations();

        analysis
    }

    /// Find inheritance chains in the type hierarchy
    fn find_inheritance_chains(&self) -> Vec<Vec<String>> {
        let mut chains = Vec::new();
        let mut visited = HashSet::new();

        for type_name in self.type_nodes.keys() {
            if !visited.contains(type_name) {
                let chain = self.build_inheritance_chain(type_name, &mut visited);
                if chain.len() > 1 {
                    chains.push(chain);
                }
            }
        }

        chains
    }

    /// Build inheritance chain starting from a type
    fn build_inheritance_chain(
        &self,
        start_type: &str,
        visited: &mut HashSet<String>,
    ) -> Vec<String> {
        let mut chain = vec![start_type.to_string()];
        visited.insert(start_type.to_string());

        if let Some(extracted_type) = self.type_info_map.get(start_type) {
            for parent_type in &extracted_type.inheritance {
                if !visited.contains(parent_type) {
                    let parent_chain = self.build_inheritance_chain(parent_type, visited);
                    chain.extend(parent_chain);
                }
            }
        }

        chain
    }

    /// Calculate coupling metrics for a type
    fn calculate_type_coupling_metrics(&self, type_name: &str) -> TypeCouplingMetrics {
        let dependencies = self.dependency_graph.get_dependencies(type_name);
        let dependents = self.dependency_graph.get_dependents(type_name);

        let afferent_coupling = dependents.len();
        let efferent_coupling = dependencies.len();

        let instability = if afferent_coupling + efferent_coupling > 0 {
            efferent_coupling as f64 / (afferent_coupling + efferent_coupling) as f64
        } else {
            0.0
        };

        let abstractness = self.calculate_abstractness(type_name);

        TypeCouplingMetrics {
            afferent_coupling,
            efferent_coupling,
            instability,
            abstractness,
        }
    }

    /// Calculate abstractness of a type
    fn calculate_abstractness(&self, type_name: &str) -> f64 {
        if let Some(extracted_type) = self.type_info_map.get(type_name) {
            let total_methods = extracted_type.type_info.methods.len();
            if total_methods == 0 {
                return 0.0;
            }

            let abstract_methods = extracted_type
                .type_info
                .methods
                .iter()
                .filter(|method| method.is_abstract)
                .count();

            abstract_methods as f64 / total_methods as f64
        } else {
            0.0
        }
    }

    /// Calculate hierarchy depths for all types
    fn calculate_hierarchy_depths(&self) -> HashMap<String, usize> {
        let mut depths = HashMap::new();

        for type_name in self.type_nodes.keys() {
            let depth = self.calculate_type_depth(type_name, &mut HashSet::new());
            depths.insert(type_name.clone(), depth);
        }

        depths
    }

    /// Calculate depth of a type in the inheritance hierarchy
    fn calculate_type_depth(&self, type_name: &str, visited: &mut HashSet<String>) -> usize {
        if visited.contains(type_name) {
            return 0; // Circular reference
        }

        visited.insert(type_name.to_string());

        let mut max_depth = 0;
        if let Some(extracted_type) = self.type_info_map.get(type_name) {
            for parent_type in &extracted_type.inheritance {
                let parent_depth = self.calculate_type_depth(parent_type, visited);
                max_depth = max_depth.max(parent_depth + 1);
            }
        }

        visited.remove(type_name);
        max_depth
    }

    /// Find interface implementations
    fn find_interface_implementations(&self) -> HashMap<String, Vec<String>> {
        let mut implementations = HashMap::new();

        for (type_name, extracted_type) in &self.type_info_map {
            for interface_name in &extracted_type.implementations {
                implementations
                    .entry(interface_name.clone())
                    .or_insert_with(Vec::new)
                    .push(type_name.clone());
            }
        }

        implementations
    }

    /// Get the underlying dependency graph
    pub fn get_dependency_graph(&self) -> &DependencyGraph {
        &self.dependency_graph
    }

    /// Get type information map
    pub fn get_type_info_map(&self) -> &HashMap<String, ExtractedTypeInfo> {
        &self.type_info_map
    }
}
