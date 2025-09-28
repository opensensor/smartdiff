//! Abstract Syntax Tree definitions and utilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a node in the Abstract Syntax Tree
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ASTNode {
    pub id: String,
    pub node_type: NodeType,
    pub children: Vec<ASTNode>,
    pub metadata: NodeMetadata,
}

/// Types of AST nodes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    // Program structure
    Program,
    Module,
    Class,
    Interface,

    // Functions and methods
    Function,
    Method,
    Constructor,

    // Statements
    Block,
    IfStatement,
    WhileLoop,
    ForLoop,
    ReturnStatement,
    ExpressionStatement,

    // Expressions
    BinaryExpression,
    UnaryExpression,
    CallExpression,
    AssignmentExpression,
    Identifier,
    Literal,

    // Declarations
    VariableDeclaration,
    ParameterDeclaration,
    FieldDeclaration,

    // Other
    Comment,
    Unknown,
}

/// Metadata associated with an AST node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub line: usize,
    pub column: usize,
    pub original_text: String,
    pub attributes: HashMap<String, String>,
}

impl ASTNode {
    pub fn new(node_type: NodeType, metadata: NodeMetadata) -> Self {
        Self {
            id: format!("node_{}", uuid::Uuid::new_v4().simple()),
            node_type,
            children: Vec::new(),
            metadata,
        }
    }

    pub fn add_child(&mut self, child: ASTNode) {
        self.children.push(child);
    }

    pub fn find_by_type(&self, node_type: &NodeType) -> Vec<&ASTNode> {
        let mut result = Vec::new();
        if &self.node_type == node_type {
            result.push(self);
        }
        for child in &self.children {
            result.extend(child.find_by_type(node_type));
        }
        result
    }

    /// Calculate a hash for this node based on its structure
    pub fn structural_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.node_type.hash(&mut hasher);
        for child in &self.children {
            child.structural_hash().hash(&mut hasher);
        }
        hasher.finish()
    }

    /// Get the depth of this node in the tree
    pub fn depth(&self) -> usize {
        if self.children.is_empty() {
            1
        } else {
            1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0)
        }
    }
}
