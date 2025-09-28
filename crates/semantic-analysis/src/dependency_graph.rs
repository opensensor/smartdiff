//! Dependency graph construction and analysis

use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::{Directed, Graph};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dependency graph representing relationships between code elements
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    graph: Graph<DependencyNode, DependencyEdge, Directed>,
    node_map: HashMap<String, NodeIndex>,
}

/// Node in the dependency graph
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct DependencyNode {
    pub id: String,
    pub name: String,
    pub node_type: DependencyNodeType,
    pub file_path: String,
    pub line: usize,
}

/// Edge in the dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub edge_type: DependencyEdgeType,
    pub strength: f64, // 0.0 to 1.0
}

/// Types of dependency nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DependencyNodeType {
    Function,
    Class,
    Module,
    Variable,
    File,
}

/// Types of dependency edges
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DependencyEdgeType {
    Calls,
    Inherits,
    Implements,
    Uses,
    Imports,
    Contains,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_map: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: DependencyNode) -> NodeIndex {
        let node_id = node.id.clone();
        let index = self.graph.add_node(node);
        self.node_map.insert(node_id, index);
        index
    }

    /// Add an edge between two nodes
    pub fn add_edge(
        &mut self,
        from_id: &str,
        to_id: &str,
        edge: DependencyEdge,
    ) -> Option<EdgeIndex> {
        let from_index = *self.node_map.get(from_id)?;
        let to_index = *self.node_map.get(to_id)?;
        Some(self.graph.add_edge(from_index, to_index, edge))
    }

    /// Get dependencies of a node
    pub fn get_dependencies(&self, node_id: &str) -> Vec<&DependencyNode> {
        if let Some(&node_index) = self.node_map.get(node_id) {
            self.graph
                .neighbors(node_index)
                .map(|neighbor_index| &self.graph[neighbor_index])
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get dependents of a node (reverse dependencies)
    pub fn get_dependents(&self, node_id: &str) -> Vec<&DependencyNode> {
        if let Some(&node_index) = self.node_map.get(node_id) {
            self.graph
                .neighbors_directed(node_index, petgraph::Direction::Incoming)
                .map(|neighbor_index| &self.graph[neighbor_index])
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Find strongly connected components
    pub fn find_cycles(&self) -> Vec<Vec<String>> {
        use petgraph::algo::kosaraju_scc;

        let sccs = kosaraju_scc(&self.graph);
        sccs.into_iter()
            .filter(|scc| scc.len() > 1) // Only return actual cycles
            .map(|scc| {
                scc.into_iter()
                    .map(|node_index| self.graph[node_index].id.clone())
                    .collect()
            })
            .collect()
    }

    /// Calculate coupling metrics
    pub fn calculate_coupling(&self, node_id: &str) -> CouplingMetrics {
        let dependencies = self.get_dependencies(node_id).len();
        let dependents = self.get_dependents(node_id).len();

        CouplingMetrics {
            afferent_coupling: dependents,
            efferent_coupling: dependencies,
            instability: if dependencies + dependents > 0 {
                dependencies as f64 / (dependencies + dependents) as f64
            } else {
                0.0
            },
        }
    }

    /// Get the number of nodes in the graph
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of edges in the graph
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Get all edge weights
    pub fn edge_weights(&self) -> impl Iterator<Item = &DependencyEdge> {
        self.graph.edge_weights()
    }

    /// Get a node by its index
    pub fn get_node(&self, node_index: NodeIndex) -> Option<&DependencyNode> {
        self.graph.node_weight(node_index)
    }
}

/// Coupling metrics for a node
#[derive(Debug, Clone)]
pub struct CouplingMetrics {
    pub afferent_coupling: usize, // Number of classes that depend on this class
    pub efferent_coupling: usize, // Number of classes this class depends on
    pub instability: f64,         // Efferent / (Afferent + Efferent)
}
