#![allow(dead_code)]

use crate::adt::small_vector::SmallVector;

// Represent an edge in the directed graph.
// The edge contains the target node it connects to.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DGEdge<NodeType> {
  target_node: NodeType
}

impl<NodeType> DGEdge<NodeType> {
  // Create an edge pointing to the given node n.
  pub fn new(n: NodeType) -> Self {
    DGEdge { target_node: n }
  }

  // Retrieve the target node this edge connects to.
  pub fn get_target_node(&self) -> &NodeType {
    &self.target_node
  }

  // Set the target node this edge connects to.
  pub fn set_target_node(&mut self, n: NodeType) {
    self.target_node = n;
  }
}

// Represent a node in the directed graph.
// The node has a (possively empty) list of outgoing edges.
struct DGNode {
}

impl DGNode {
  pub fn front() {}

  pub fn back() {}

  pub fn find_edges_to() {}

  pub fn add_edge() {}

  pub fn remove_edge() {}

  pub fn has_edge_to() {}

  pub fn get_edges() {}

  // Clear the outgoing edges.
  pub fn clear() {}
}

// The graph is represented by a table of nodes.
// Each node contains a (possively empty) list of outgoing edges.
// Each edge contains the target node it connects to.
struct DirectedGraph<NodeType> {
  nodes: SmallVector<NodeType>
}

impl<NodeType> DirectedGraph<NodeType> {
  pub fn front() {}

  pub fn back() {}

  pub fn size() {}

  pub fn find_node() {}

  pub fn add_node() {}

  pub fn find_incoming_edges_to_node() {}

  pub fn remove_node() {}

  pub fn connect() {}
}