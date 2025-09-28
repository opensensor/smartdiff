//! Comprehensive dependency graph construction integrating symbols, types, and function calls

use crate::{
    DependencyEdge, DependencyEdgeType, DependencyGraph, DependencyNode, DependencyNodeType,
    SymbolTable, TypeExtractionResult,
};
use anyhow::{anyhow, Result};
use petgraph::graph::NodeIndex;
use smart_diff_parser::{ASTNode, Language, NodeType, ParseResult};
use std::collections::{HashMap, HashSet};

/// Configuration for comprehensive dependency analysis
#[derive(Debug, Clone)]
pub struct DependencyAnalysisConfig {
    /// Include function call dependencies
    pub include_function_calls: bool,
    /// Include type dependencies
    pub include_type_dependencies: bool,
    /// Include variable usage dependencies
    pub include_variable_usage: bool,
    /// Include import/export dependencies
    pub include_import_dependencies: bool,
    /// Include inheritance relationships
    pub include_inheritance: bool,
    /// Minimum dependency strength to include (0.0 to 1.0)
    pub min_dependency_strength: f64,
    /// Maximum depth for transitive dependency analysis
    pub max_transitive_depth: usize,
}

impl Default for DependencyAnalysisConfig {
    fn default() -> Self {
        Self {
            include_function_calls: true,
            include_type_dependencies: true,
            include_variable_usage: true,
            include_import_dependencies: true,
            include_inheritance: true,
            min_dependency_strength: 0.1,
            max_transitive_depth: 10,
        }
    }
}

/// Comprehensive dependency graph builder
pub struct ComprehensiveDependencyGraphBuilder {
    config: DependencyAnalysisConfig,
    dependency_graph: DependencyGraph,
    node_map: HashMap<String, NodeIndex>,
    symbol_table: Option<SymbolTable>,
    type_extraction_results: HashMap<String, TypeExtractionResult>,
    file_contexts: HashMap<String, FileAnalysisContext>,
}

/// Analysis context for a single file
#[derive(Debug, Clone)]
pub struct FileAnalysisContext {
    pub file_path: String,
    pub language: Language,
    pub functions: Vec<FunctionInfo>,
    pub classes: Vec<ClassInfo>,
    pub variables: Vec<VariableInfo>,
    pub function_calls: Vec<FunctionCallInfo>,
    pub imports: Vec<ImportInfo>,
    pub exports: Vec<ExportInfo>,
}

/// Function information for dependency analysis
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub qualified_name: String,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub calls: Vec<String>,
    pub accesses: Vec<String>, // Variables/fields accessed
    pub line: usize,
    pub column: usize,
}

/// Class information for dependency analysis
#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub qualified_name: String,
    pub parent_classes: Vec<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<String>,
    pub methods: Vec<String>,
    pub line: usize,
}

/// Variable information for dependency analysis
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub qualified_name: String,
    pub var_type: Option<String>,
    pub scope: String,
    pub is_global: bool,
    pub line: usize,
}

/// Function call information
#[derive(Debug, Clone)]
pub struct FunctionCallInfo {
    pub caller: String,
    pub callee: String,
    pub call_type: CallType,
    pub line: usize,
    pub column: usize,
}

/// Import information for dependency analysis
#[derive(Debug, Clone)]
pub struct ImportInfo {
    pub imported_name: String,
    pub source_module: Option<String>,
    pub alias: Option<String>,
    pub is_wildcard: bool,
}

/// Export information for dependency analysis
#[derive(Debug, Clone)]
pub struct ExportInfo {
    pub exported_name: String,
    pub internal_name: String,
    pub export_type: ExportType,
}

/// Types of function calls
#[derive(Debug, Clone, PartialEq)]
pub enum CallType {
    Direct,      // Direct function call
    Method,      // Method call on object
    Constructor, // Constructor call
    Static,      // Static method call
}

/// Types of exports
#[derive(Debug, Clone, PartialEq)]
pub enum ExportType {
    Function,
    Class,
    Variable,
    Type,
    Module,
}

/// Comprehensive dependency analysis result
#[derive(Debug)]
pub struct ComprehensiveDependencyAnalysis {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub function_call_dependencies: usize,
    pub type_dependencies: usize,
    pub variable_dependencies: usize,
    pub import_dependencies: usize,
    pub inheritance_dependencies: usize,
    pub circular_dependencies: Vec<Vec<String>>,
    pub strongly_connected_components: Vec<Vec<String>>,
    pub dependency_layers: Vec<Vec<String>>,
    pub coupling_metrics: HashMap<String, ComprehensiveCouplingMetrics>,
    pub hotspots: Vec<DependencyHotspot>,
}

/// Comprehensive coupling metrics
#[derive(Debug, Clone)]
pub struct ComprehensiveCouplingMetrics {
    pub afferent_coupling: usize,
    pub efferent_coupling: usize,
    pub instability: f64,
    pub function_call_coupling: usize,
    pub type_coupling: usize,
    pub data_coupling: usize,
    pub control_coupling: usize,
}

/// Dependency hotspot (highly coupled component)
#[derive(Debug, Clone)]
pub struct DependencyHotspot {
    pub name: String,
    pub node_type: DependencyNodeType,
    pub coupling_score: f64,
    pub incoming_dependencies: usize,
    pub outgoing_dependencies: usize,
    pub file_path: String,
}

impl ComprehensiveDependencyGraphBuilder {
    pub fn new(config: DependencyAnalysisConfig) -> Self {
        Self {
            config,
            dependency_graph: DependencyGraph::new(),
            node_map: HashMap::new(),
            symbol_table: None,
            type_extraction_results: HashMap::new(),
            file_contexts: HashMap::new(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(DependencyAnalysisConfig::default())
    }

    /// Set symbol table for analysis
    pub fn with_symbol_table(mut self, symbol_table: SymbolTable) -> Self {
        self.symbol_table = Some(symbol_table);
        self
    }

    /// Add type extraction results
    pub fn add_type_extraction_result(&mut self, file_path: String, result: TypeExtractionResult) {
        self.type_extraction_results.insert(file_path, result);
    }

    /// Build comprehensive dependency graph from multiple sources
    pub fn build_comprehensive_graph(&mut self, files: Vec<(String, ParseResult)>) -> Result<()> {
        // First pass: Analyze each file and extract dependency information
        for (file_path, parse_result) in &files {
            let context = self.analyze_file(file_path, parse_result)?;
            self.file_contexts.insert(file_path.clone(), context);
        }

        // Second pass: Create nodes for all entities
        self.create_dependency_nodes()?;

        // Third pass: Create edges for all relationships
        self.create_dependency_edges()?;

        Ok(())
    }

    /// Analyze a single file and extract dependency information
    fn analyze_file(
        &mut self,
        file_path: &str,
        parse_result: &ParseResult,
    ) -> Result<FileAnalysisContext> {
        let mut context = FileAnalysisContext {
            file_path: file_path.to_string(),
            language: parse_result.language,
            functions: Vec::new(),
            classes: Vec::new(),
            variables: Vec::new(),
            function_calls: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
        };

        // Extract information from AST
        self.extract_from_ast(&parse_result.ast, &mut context, Vec::new())?;

        Ok(context)
    }

    /// Extract dependency information from AST node
    fn extract_from_ast(
        &mut self,
        node: &ASTNode,
        context: &mut FileAnalysisContext,
        scope_path: Vec<String>,
    ) -> Result<()> {
        match node.node_type {
            NodeType::Function | NodeType::Method | NodeType::Constructor => {
                if let Some(function_info) = self.extract_function_info(node, &scope_path)? {
                    context.functions.push(function_info);
                }
            }
            NodeType::Class | NodeType::Interface => {
                if let Some(class_info) = self.extract_class_info(node, &scope_path)? {
                    context.classes.push(class_info);
                }
            }
            NodeType::VariableDeclaration | NodeType::FieldDeclaration => {
                if let Some(variable_info) = self.extract_variable_info(node, &scope_path)? {
                    context.variables.push(variable_info);
                }
            }
            NodeType::CallExpression => {
                if let Some(call_info) = self.extract_function_call_info(node, &scope_path)? {
                    context.function_calls.push(call_info);
                }
            }
            NodeType::ImportDeclaration => {
                if let Some(import_info) = self.extract_import_info(node)? {
                    context.imports.push(import_info);
                }
            }
            NodeType::ExportDeclaration => {
                if let Some(export_info) = self.extract_export_info(node)? {
                    context.exports.push(export_info);
                }
            }
            _ => {}
        }

        // Update scope path for nested processing
        let mut new_scope_path = scope_path;
        if let Some(name) = node.metadata.attributes.get("name") {
            if matches!(
                node.node_type,
                NodeType::Class | NodeType::Interface | NodeType::Function | NodeType::Method
            ) {
                new_scope_path.push(name.clone());
            }
        }

        // Recursively process children
        for child in &node.children {
            self.extract_from_ast(child, context, new_scope_path.clone())?;
        }

        Ok(())
    }

    /// Extract function information from AST node
    fn extract_function_info(
        &self,
        node: &ASTNode,
        scope_path: &[String],
    ) -> Result<Option<FunctionInfo>> {
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

        let parameters = self.extract_function_parameters(node);
        let return_type = node.metadata.attributes.get("return_type").cloned();
        let calls = self.extract_function_calls_from_body(node);
        let accesses = self.extract_variable_accesses_from_body(node);

        Ok(Some(FunctionInfo {
            name: name.clone(),
            qualified_name,
            parameters,
            return_type,
            calls,
            accesses,
            line: node.metadata.line,
            column: node.metadata.column,
        }))
    }

    /// Extract class information from AST node
    fn extract_class_info(
        &self,
        node: &ASTNode,
        scope_path: &[String],
    ) -> Result<Option<ClassInfo>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .ok_or_else(|| anyhow!("Class node missing name"))?;

        let qualified_name = if scope_path.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", scope_path.join("."), name)
        };

        let parent_classes = self.extract_parent_classes(node);
        let interfaces = self.extract_implemented_interfaces(node);
        let fields = self.extract_class_fields(node);
        let methods = self.extract_class_methods(node);

        Ok(Some(ClassInfo {
            name: name.clone(),
            qualified_name,
            parent_classes,
            interfaces,
            fields,
            methods,
            line: node.metadata.line,
        }))
    }

    /// Extract variable information from AST node
    fn extract_variable_info(
        &self,
        node: &ASTNode,
        scope_path: &[String],
    ) -> Result<Option<VariableInfo>> {
        let name = node
            .metadata
            .attributes
            .get("name")
            .ok_or_else(|| anyhow!("Variable node missing name"))?;

        let qualified_name = if scope_path.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", scope_path.join("."), name)
        };

        let var_type = node.metadata.attributes.get("type").cloned();
        let scope = scope_path.join(".");
        let is_global = scope_path.is_empty() || scope_path.len() == 1;

        Ok(Some(VariableInfo {
            name: name.clone(),
            qualified_name,
            var_type,
            scope,
            is_global,
            line: node.metadata.line,
        }))
    }

    /// Extract function call information from AST node
    fn extract_function_call_info(
        &self,
        node: &ASTNode,
        scope_path: &[String],
    ) -> Result<Option<FunctionCallInfo>> {
        let callee = node
            .metadata
            .attributes
            .get("function_name")
            .ok_or_else(|| anyhow!("Call expression missing function name"))?;

        let caller = if scope_path.is_empty() {
            "global".to_string()
        } else {
            scope_path.join(".")
        };

        let call_type = self.determine_call_type(node);

        Ok(Some(FunctionCallInfo {
            caller,
            callee: callee.clone(),
            call_type,
            line: node.metadata.line,
            column: node.metadata.column,
        }))
    }

    /// Extract import information from AST node
    fn extract_import_info(&self, node: &ASTNode) -> Result<Option<ImportInfo>> {
        let imported_name = node
            .metadata
            .attributes
            .get("imported_name")
            .ok_or_else(|| anyhow!("Import node missing imported name"))?;

        let source_module = node.metadata.attributes.get("source").cloned();
        let alias = node.metadata.attributes.get("alias").cloned();
        let is_wildcard = node
            .metadata
            .attributes
            .get("wildcard")
            .map(|v| v == "true")
            .unwrap_or(false);

        Ok(Some(ImportInfo {
            imported_name: imported_name.clone(),
            source_module,
            alias,
            is_wildcard,
        }))
    }

    /// Extract export information from AST node
    fn extract_export_info(&self, node: &ASTNode) -> Result<Option<ExportInfo>> {
        let exported_name = node
            .metadata
            .attributes
            .get("exported_name")
            .ok_or_else(|| anyhow!("Export node missing exported name"))?;

        let internal_name = node
            .metadata
            .attributes
            .get("internal_name")
            .unwrap_or(exported_name);

        let export_type = match node
            .metadata
            .attributes
            .get("export_type")
            .map(|s| s.as_str())
        {
            Some("function") => ExportType::Function,
            Some("class") => ExportType::Class,
            Some("variable") => ExportType::Variable,
            Some("type") => ExportType::Type,
            Some("module") => ExportType::Module,
            _ => ExportType::Function, // Default
        };

        Ok(Some(ExportInfo {
            exported_name: exported_name.clone(),
            internal_name: internal_name.clone(),
            export_type,
        }))
    }

    /// Helper methods for extracting specific information
    fn extract_function_parameters(&self, node: &ASTNode) -> Vec<String> {
        node.children
            .iter()
            .filter(|child| child.node_type == NodeType::Parameter)
            .filter_map(|param| param.metadata.attributes.get("name"))
            .cloned()
            .collect()
    }

    fn extract_function_calls_from_body(&self, node: &ASTNode) -> Vec<String> {
        let mut calls = Vec::new();
        self.collect_function_calls_recursive(node, &mut calls);
        calls
    }

    #[allow(clippy::only_used_in_recursion)]
    fn collect_function_calls_recursive(&self, node: &ASTNode, calls: &mut Vec<String>) {
        if node.node_type == NodeType::CallExpression {
            if let Some(function_name) = node.metadata.attributes.get("function_name") {
                calls.push(function_name.clone());
            }
        }

        for child in &node.children {
            self.collect_function_calls_recursive(child, calls);
        }
    }

    fn extract_variable_accesses_from_body(&self, node: &ASTNode) -> Vec<String> {
        let mut accesses = Vec::new();
        self.collect_variable_accesses_recursive(node, &mut accesses);
        accesses
    }

    #[allow(clippy::only_used_in_recursion)]
    fn collect_variable_accesses_recursive(&self, node: &ASTNode, accesses: &mut Vec<String>) {
        if matches!(
            node.node_type,
            NodeType::Identifier | NodeType::MemberExpression
        ) {
            if let Some(name) = node.metadata.attributes.get("name") {
                accesses.push(name.clone());
            }
        }

        for child in &node.children {
            self.collect_variable_accesses_recursive(child, accesses);
        }
    }

    fn extract_parent_classes(&self, node: &ASTNode) -> Vec<String> {
        node.children
            .iter()
            .filter(|child| child.node_type == NodeType::Inheritance)
            .filter_map(|inherit| inherit.metadata.attributes.get("parent_class"))
            .cloned()
            .collect()
    }

    fn extract_implemented_interfaces(&self, node: &ASTNode) -> Vec<String> {
        node.children
            .iter()
            .filter(|child| child.node_type == NodeType::Implementation)
            .filter_map(|impl_node| impl_node.metadata.attributes.get("interface"))
            .cloned()
            .collect()
    }

    fn extract_class_fields(&self, node: &ASTNode) -> Vec<String> {
        node.children
            .iter()
            .filter(|child| child.node_type == NodeType::FieldDeclaration)
            .filter_map(|field| field.metadata.attributes.get("name"))
            .cloned()
            .collect()
    }

    fn extract_class_methods(&self, node: &ASTNode) -> Vec<String> {
        node.children
            .iter()
            .filter(|child| matches!(child.node_type, NodeType::Method | NodeType::Constructor))
            .filter_map(|method| method.metadata.attributes.get("name"))
            .cloned()
            .collect()
    }

    fn determine_call_type(&self, node: &ASTNode) -> CallType {
        if let Some(call_type) = node.metadata.attributes.get("call_type") {
            match call_type.as_str() {
                "method" => CallType::Method,
                "constructor" => CallType::Constructor,
                "static" => CallType::Static,
                _ => CallType::Direct,
            }
        } else {
            CallType::Direct
        }
    }

    /// Create dependency nodes for all entities
    fn create_dependency_nodes(&mut self) -> Result<()> {
        for context in self.file_contexts.values() {
            // Create nodes for functions
            for function in &context.functions {
                let node = DependencyNode {
                    id: function.qualified_name.clone(),
                    name: function.name.clone(),
                    node_type: DependencyNodeType::Function,
                    file_path: context.file_path.clone(),
                    line: function.line,
                };
                let index = self.dependency_graph.add_node(node);
                self.node_map.insert(function.qualified_name.clone(), index);
            }

            // Create nodes for classes
            for class in &context.classes {
                let node = DependencyNode {
                    id: class.qualified_name.clone(),
                    name: class.name.clone(),
                    node_type: DependencyNodeType::Class,
                    file_path: context.file_path.clone(),
                    line: class.line,
                };
                let index = self.dependency_graph.add_node(node);
                self.node_map.insert(class.qualified_name.clone(), index);
            }

            // Create nodes for global variables
            for variable in &context.variables {
                if variable.is_global {
                    let node = DependencyNode {
                        id: variable.qualified_name.clone(),
                        name: variable.name.clone(),
                        node_type: DependencyNodeType::Variable,
                        file_path: context.file_path.clone(),
                        line: variable.line,
                    };
                    let index = self.dependency_graph.add_node(node);
                    self.node_map.insert(variable.qualified_name.clone(), index);
                }
            }
        }

        Ok(())
    }

    /// Create dependency edges for all relationships
    fn create_dependency_edges(&mut self) -> Result<()> {
        if self.config.include_function_calls {
            self.create_function_call_edges()?;
        }

        if self.config.include_type_dependencies {
            self.create_type_dependency_edges()?;
        }

        if self.config.include_variable_usage {
            self.create_variable_usage_edges()?;
        }

        if self.config.include_import_dependencies {
            self.create_import_dependency_edges()?;
        }

        if self.config.include_inheritance {
            self.create_inheritance_edges()?;
        }

        Ok(())
    }

    /// Create function call dependency edges
    fn create_function_call_edges(&mut self) -> Result<()> {
        for context in self.file_contexts.values() {
            for call in &context.function_calls {
                if self.node_map.contains_key(&call.caller)
                    && self.node_map.contains_key(&call.callee)
                {
                    let strength = match call.call_type {
                        CallType::Direct => 0.9,
                        CallType::Method => 0.8,
                        CallType::Constructor => 0.7,
                        CallType::Static => 0.6,
                    };

                    if strength >= self.config.min_dependency_strength {
                        let edge = DependencyEdge {
                            edge_type: DependencyEdgeType::Calls,
                            strength,
                        };
                        self.dependency_graph
                            .add_edge(&call.caller, &call.callee, edge);
                    }
                }
            }
        }
        Ok(())
    }

    /// Create type dependency edges
    fn create_type_dependency_edges(&mut self) -> Result<()> {
        for type_result in self.type_extraction_results.values() {
            for extracted_type in &type_result.types {
                let type_name = &extracted_type.type_info.name;

                // Inheritance dependencies
                for parent in &extracted_type.inheritance {
                    if self.node_map.contains_key(type_name) && self.node_map.contains_key(parent) {
                        let edge = DependencyEdge {
                            edge_type: DependencyEdgeType::Inherits,
                            strength: 1.0,
                        };
                        self.dependency_graph.add_edge(type_name, parent, edge);
                    }
                }

                // Implementation dependencies
                for interface in &extracted_type.implementations {
                    if self.node_map.contains_key(type_name)
                        && self.node_map.contains_key(interface)
                    {
                        let edge = DependencyEdge {
                            edge_type: DependencyEdgeType::Implements,
                            strength: 0.9,
                        };
                        self.dependency_graph.add_edge(type_name, interface, edge);
                    }
                }

                // Field type dependencies
                for field in &extracted_type.type_info.fields {
                    if !self.is_primitive_type(&field.type_name)
                        && self.node_map.contains_key(type_name)
                        && self.node_map.contains_key(&field.type_name)
                    {
                        let strength = if field.is_static { 0.6 } else { 0.8 };
                        let edge = DependencyEdge {
                            edge_type: DependencyEdgeType::Uses,
                            strength,
                        };
                        self.dependency_graph
                            .add_edge(type_name, &field.type_name, edge);
                    }
                }
            }
        }
        Ok(())
    }

    /// Create variable usage dependency edges
    fn create_variable_usage_edges(&mut self) -> Result<()> {
        for context in self.file_contexts.values() {
            for function in &context.functions {
                for access in &function.accesses {
                    if self.node_map.contains_key(&function.qualified_name)
                        && self.node_map.contains_key(access)
                    {
                        let edge = DependencyEdge {
                            edge_type: DependencyEdgeType::Uses,
                            strength: 0.4,
                        };
                        self.dependency_graph
                            .add_edge(&function.qualified_name, access, edge);
                    }
                }
            }
        }
        Ok(())
    }

    /// Create import dependency edges
    fn create_import_dependency_edges(&mut self) -> Result<()> {
        for context in self.file_contexts.values() {
            for import in &context.imports {
                if let Some(source_module) = &import.source_module {
                    // Create dependency from current file to imported module
                    let _current_file_node = format!("file:{}", context.file_path);
                    let _imported_module_node = format!("module:{}", source_module);

                    // Note: This would require creating file/module nodes
                    // For now, we'll skip this or create them dynamically
                }
            }
        }
        Ok(())
    }

    /// Create inheritance dependency edges
    fn create_inheritance_edges(&mut self) -> Result<()> {
        for context in self.file_contexts.values() {
            for class in &context.classes {
                for parent in &class.parent_classes {
                    if self.node_map.contains_key(&class.qualified_name)
                        && self.node_map.contains_key(parent)
                    {
                        let edge = DependencyEdge {
                            edge_type: DependencyEdgeType::Inherits,
                            strength: 1.0,
                        };
                        self.dependency_graph
                            .add_edge(&class.qualified_name, parent, edge);
                    }
                }

                for interface in &class.interfaces {
                    if self.node_map.contains_key(&class.qualified_name)
                        && self.node_map.contains_key(interface)
                    {
                        let edge = DependencyEdge {
                            edge_type: DependencyEdgeType::Implements,
                            strength: 0.9,
                        };
                        self.dependency_graph
                            .add_edge(&class.qualified_name, interface, edge);
                    }
                }
            }
        }
        Ok(())
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

    /// Perform comprehensive dependency analysis
    pub fn analyze_comprehensive_dependencies(&self) -> ComprehensiveDependencyAnalysis {
        let total_nodes = self.dependency_graph.node_count();
        let total_edges = self.dependency_graph.edge_count();

        // Count different types of dependencies
        let mut function_call_dependencies = 0;
        let mut type_dependencies = 0;
        let mut variable_dependencies = 0;
        let import_dependencies = 0;
        let mut inheritance_dependencies = 0;

        for edge in self.dependency_graph.edge_weights() {
            match edge.edge_type {
                DependencyEdgeType::Calls => function_call_dependencies += 1,
                DependencyEdgeType::Uses => {
                    if edge.strength > 0.6 {
                        type_dependencies += 1;
                    } else {
                        variable_dependencies += 1;
                    }
                }
                DependencyEdgeType::Inherits | DependencyEdgeType::Implements => {
                    inheritance_dependencies += 1;
                }
                DependencyEdgeType::Imports | DependencyEdgeType::Contains => {
                    // Module/file level dependencies
                    type_dependencies += 1;
                }
            }
        }

        // Find circular dependencies
        let circular_dependencies = self.dependency_graph.find_cycles();

        // Find strongly connected components
        let strongly_connected_components = self.find_strongly_connected_components();

        // Calculate dependency layers (topological ordering)
        let dependency_layers = self.calculate_dependency_layers();

        // Calculate comprehensive coupling metrics
        let coupling_metrics = self.calculate_comprehensive_coupling_metrics();

        // Identify dependency hotspots
        let hotspots = self.identify_dependency_hotspots(&coupling_metrics);

        ComprehensiveDependencyAnalysis {
            total_nodes,
            total_edges,
            function_call_dependencies,
            type_dependencies,
            variable_dependencies,
            import_dependencies,
            inheritance_dependencies,
            circular_dependencies,
            strongly_connected_components,
            dependency_layers,
            coupling_metrics,
            hotspots,
        }
    }

    /// Find strongly connected components
    fn find_strongly_connected_components(&self) -> Vec<Vec<String>> {
        // This would use the existing find_cycles method or implement Tarjan's algorithm
        self.dependency_graph.find_cycles()
    }

    /// Calculate dependency layers using topological sorting
    fn calculate_dependency_layers(&self) -> Vec<Vec<String>> {
        let mut layers = Vec::new();
        let mut remaining_nodes: HashSet<String> = self.node_map.keys().cloned().collect();

        while !remaining_nodes.is_empty() {
            let mut current_layer = Vec::new();

            // Find nodes with no incoming dependencies from remaining nodes
            for node_id in &remaining_nodes {
                let dependencies = self.dependency_graph.get_dependencies(node_id);
                let has_remaining_deps = dependencies
                    .iter()
                    .any(|dep| remaining_nodes.contains(&dep.id));

                if !has_remaining_deps {
                    current_layer.push(node_id.clone());
                }
            }

            if current_layer.is_empty() {
                // Circular dependency - add all remaining nodes
                current_layer.extend(remaining_nodes.iter().cloned());
                remaining_nodes.clear();
            } else {
                for node in &current_layer {
                    remaining_nodes.remove(node);
                }
            }

            layers.push(current_layer);
        }

        layers
    }

    /// Calculate comprehensive coupling metrics for all nodes
    fn calculate_comprehensive_coupling_metrics(
        &self,
    ) -> HashMap<String, ComprehensiveCouplingMetrics> {
        let mut metrics = HashMap::new();

        for node_id in self.node_map.keys() {
            let basic_metrics = self.dependency_graph.calculate_coupling(node_id);

            // Calculate detailed coupling types
            let function_call_coupling =
                self.count_coupling_by_type(node_id, DependencyEdgeType::Calls);
            let type_coupling = self.count_coupling_by_type(node_id, DependencyEdgeType::Uses);
            let data_coupling = self.count_data_coupling(node_id);
            let control_coupling = self.count_control_coupling(node_id);

            let comprehensive_metrics = ComprehensiveCouplingMetrics {
                afferent_coupling: basic_metrics.afferent_coupling,
                efferent_coupling: basic_metrics.efferent_coupling,
                instability: basic_metrics.instability,
                function_call_coupling,
                type_coupling,
                data_coupling,
                control_coupling,
            };

            metrics.insert(node_id.clone(), comprehensive_metrics);
        }

        metrics
    }

    /// Count coupling by edge type
    fn count_coupling_by_type(&self, node_id: &str, _edge_type: DependencyEdgeType) -> usize {
        let dependencies = self.dependency_graph.get_dependencies(node_id);
        let dependents = self.dependency_graph.get_dependents(node_id);

        // This is a simplified implementation
        // In a real implementation, we'd check the actual edge types
        dependencies.len() + dependents.len()
    }

    /// Count data coupling (shared data access)
    fn count_data_coupling(&self, node_id: &str) -> usize {
        // Count dependencies on variables and fields
        let dependencies = self.dependency_graph.get_dependencies(node_id);
        dependencies
            .iter()
            .filter(|dep| {
                // Check if dependency is a variable or field
                matches!(
                    self.file_contexts.values().find(|ctx| {
                        ctx.variables.iter().any(|var| var.qualified_name == dep.id)
                    }),
                    Some(_context)
                )
            })
            .count()
    }

    /// Count control coupling (control flow dependencies)
    fn count_control_coupling(&self, node_id: &str) -> usize {
        // Count function call dependencies
        self.count_coupling_by_type(node_id, DependencyEdgeType::Calls)
    }

    /// Identify dependency hotspots
    fn identify_dependency_hotspots(
        &self,
        coupling_metrics: &HashMap<String, ComprehensiveCouplingMetrics>,
    ) -> Vec<DependencyHotspot> {
        let mut hotspots = Vec::new();

        for (node_id, metrics) in coupling_metrics {
            let coupling_score = (metrics.afferent_coupling + metrics.efferent_coupling) as f64
                * (1.0 + metrics.instability);

            // Consider nodes with high coupling as hotspots
            if coupling_score > 10.0 {
                if let Some(node_index) = self.node_map.get(node_id) {
                    if let Some(node) = self.dependency_graph.get_node(*node_index) {
                        let hotspot = DependencyHotspot {
                            name: node.name.clone(),
                            node_type: node.node_type.clone(),
                            coupling_score,
                            incoming_dependencies: metrics.afferent_coupling,
                            outgoing_dependencies: metrics.efferent_coupling,
                            file_path: node.file_path.clone(),
                        };
                        hotspots.push(hotspot);
                    }
                }
            }
        }

        // Sort by coupling score (highest first)
        hotspots.sort_by(|a, b| b.coupling_score.partial_cmp(&a.coupling_score).unwrap());

        hotspots
    }

    /// Get the underlying dependency graph
    pub fn get_dependency_graph(&self) -> &DependencyGraph {
        &self.dependency_graph
    }

    /// Get file analysis contexts
    pub fn get_file_contexts(&self) -> &HashMap<String, FileAnalysisContext> {
        &self.file_contexts
    }
}
