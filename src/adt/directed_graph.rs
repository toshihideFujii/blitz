#![allow(dead_code)]

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Index;

use rand::{thread_rng, Rng};

use crate::adt::set_vector::SetVector;
use crate::adt::small_vector::SmallVector;

// Represent an edge in the directed graph.
// The edge contains the target node it connects to.
#[derive(Debug, Clone, PartialEq)]
struct DGEdge {
  target_node: DGNode
}

impl DGEdge {
  // Create an edge pointing to the given node n.
  pub fn new(n: &DGNode) -> Self {
    DGEdge { target_node: n.clone() }
  }

  // Retrieve the target node this edge connects to.
  pub fn get_target_node(&self) -> &DGNode {
    &self.target_node
  }

  // Set the target node this edge connects to.
  pub fn set_target_node(&mut self, n: DGNode) {
    self.target_node = n;
  }
}

impl Eq for DGEdge {
  
}

impl Hash for DGEdge {
  fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
      
  }
}

// Represent a node in the directed graph.
// The node has a (possively empty) list of outgoing edges.
#[derive(Debug, Clone, PartialEq)]
struct DGNode {
  id: u32,
  edges: SetVector<DGEdge>
}

impl DGNode {
  pub fn new() -> Self {
    let mut rhg = thread_rng();
    DGNode { id: rhg.gen(), edges: SetVector::new() }
  }

  pub fn front(&self) -> Option<&DGEdge> {
    self.edges.front()
  }

  pub fn back(&self) -> Option<&DGEdge> {
    self.edges.back()
  }

  // Collect in el, all the edges from this node to n,
  // Return true if at least one edge was found, and false otherwise.
  pub fn find_edges_to(&self, n: &DGNode) -> Vec<DGEdge> {
    let mut el = Vec::new();
    /*
    for e in self.edges.clone().into_iter() {
      if *e.get_target_node() == *n {
        el.push(e);
      }
    }
    */
    for i in 0..self.edges.size() {
      let e = self.edges.index(i);
      if *e.get_target_node() == *n {
        el.push(e.clone());
      }
    }
    el
  }

  // Add the given edge e to this node, if it doesn't exist already.
  pub fn add_edge(&mut self, e: &DGEdge) -> bool {
    self.edges.insert(e.clone())
  }

  // Remove the given edge e from this node, if it exists.
  pub fn remove_edge(&mut self, e: &DGEdge) {
    self.edges.remove(e.clone())
  }

  // Test wheter there is an edge that goes from this node to n.
  pub fn has_edge_to(&self, n: DGNode) -> bool {
    if self.find_edge_to(n) != None {
      return true;
    } else {
      return false;
    }
  }

  // Retrieve the outgoing edges for the node.
  pub fn get_edges(&self) -> SetVector<DGEdge> {
    self.edges.clone()
  }

  // Clear the outgoing edges.
  pub fn clear(&mut self) {
    self.edges.clear()
  }

  // Find an edge to n.
  fn find_edge_to(&self, n: DGNode) -> Option<DGEdge> {
    for e in self.edges.clone().into_iter() {
      if *e.get_target_node() == n {
        return Some(e);
      }
    }
    None
  }
}
/*
impl PartialEq for DGNode {
  fn eq(&self, other: &Self) -> bool {
    if self == other {
      return true;
    } else {
      return false;
    }
  }
}
*/

// The graph is represented by a table of nodes.
// Each node contains a (possively empty) list of outgoing edges.
// Each edge contains the target node it connects to.
#[derive(Debug, PartialEq)]
struct DirectedGraph {
  nodes: SmallVector<DGNode>,
  edges: SmallVector<DGEdge>
}

impl DirectedGraph {
  pub fn new() -> Self {
    DirectedGraph { nodes: SmallVector::new(), edges: SmallVector::new() }
  }

  pub fn front(&self) -> Option<&DGNode> {
    self.nodes.front()
  }

  pub fn back(&self) -> Option<&DGNode> {
    self.nodes.back()
  }

  pub fn size(&self) -> usize {
    self.nodes.size()
  }

  // Find the given node n in the table.
  pub fn find_node(&self, n: &DGNode) -> Option<usize> {
    self.nodes.find(n)
  }

  // Add the given node n to the graph if it is not already present.
  pub fn add_node(&mut self, n: &DGNode) -> bool {
    let node = n.clone();
    if self.find_node(n) != None {
      return false;
    }
    self.nodes.push_back(node);
    true
  }

  // Collect in el all edges that are coming into node n.
  pub fn find_incoming_edges_to_node(&self, n: &DGNode) -> Vec<DGEdge> {
    let mut el: Vec<DGEdge> = Vec::new();
    /*
    for node in self.nodes.clone().into_iter() {
      if node == *n {
        continue;
      }
      let mut temp_list = node.find_edges_to(n);
      el.append(&mut temp_list);
      temp_list.clear();
    }
    */
    for i in 0..self.nodes.size() {
      println!("bbbbbbbbbbbbb: i {}", i);
      let node = self.nodes.index(i);
      if node == n {
        continue;
      }
      let mut temp_list = node.find_edges_to(n);
      println!("aaaaaaaaaaa : i: {}, {}", i, temp_list.len());
      el.append(&mut temp_list);
      temp_list.clear();
    }
    el
  }

  // Remove the given node n from the graph.
  pub fn remove_node(&mut self, _n: &mut DGNode) -> bool {
    /*
    let node_index = self.find_node(n);
    if node_index == None {
      return false;
    }
    let cloned_nodes = self.nodes.clone();
    for mut node in cloned_nodes {
      if node == *n {
        continue;
      }
      let el = node.find_edges_to(n);
      for e in el {
        node.remove_edge(&e);
      }
      //el.clear();
    }
    n.clear();
    self.nodes.erase(node_index.unwrap());
    */
    true
  }

  // Assuming nodes src and dst are already in the graph, connect node
  // src to node dst using the provided edge e.
  pub fn connect(&self, src: &DGNode, dst: &DGNode, e: &DGEdge) -> bool {
    debug_assert!(self.find_node(src) != None, "src node should be present.");
    debug_assert!(self.find_node(dst) != None, "dst node should be present.");
    debug_assert!(*e.get_target_node() == *dst, "target of the given edge does not match dst.");
    let mut src_clone = src.clone();
    src_clone.add_edge(e)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_and_connect_nodes() {
    let mut dg = DirectedGraph::new();

    let n1 = DGNode::new();
    let n2 = DGNode::new();
    let n3 = DGNode::new();

    let e1 = DGEdge::new(&n1);
    let e2 = DGEdge::new(&n2);
    let e3 = DGEdge::new(&n3);

    assert_eq!(dg.add_node(&n1), true);
    assert_eq!(dg.add_node(&n2), true);
    assert_eq!(dg.add_node(&n3), true);

    assert_eq!(dg.connect(&n1, &n2, &e2), true);
    assert_eq!(dg.connect(&n2, &n3, &e3), true);
    assert_eq!(dg.connect(&n3, &n1, &e1), true);

    // The graph looks like this now:
    //
    // +---------------+
    // v               |
    // N1 -> N2 -> N3 -+

    //assert_eq!(dg.connect(&n3, &n1, &e1), false);
    assert_eq!(dg.size(), 3);
    assert_ne!(dg.find_node(&n3), None);

    let n4 = DGNode::new();
    assert_eq!(dg.find_node(&n4), None);
    //let el = dg.find_incoming_edges_to_node(&n1);
    //assert_eq!(el.len(), 1);
  }

  #[test]
  fn test_add_remove_edge() {
    let mut dg = DirectedGraph::new();

    let mut n1 = DGNode::new();
    let n2 = DGNode::new();
    let n3 = DGNode::new();

    let e1 = DGEdge::new(&n1);
    let e2 = DGEdge::new(&n2);
    let e3 = DGEdge::new(&n3);

    assert_eq!(dg.add_node(&n1), true);
    assert_eq!(dg.add_node(&n2), true);
    assert_eq!(dg.add_node(&n3), true);

    assert_eq!(dg.connect(&n1, &n2, &e2), true);
    assert_eq!(dg.connect(&n2, &n3, &e3), true);
    assert_eq!(dg.connect(&n3, &n1, &e1), true);

    // The graph looks like this now:
    //
    // +---------------+
    // v               |
    // N1 -> N2 -> N3 -+

    assert_eq!(dg.size(), 3);
    assert_eq!(*e1.get_target_node(), n1);
    assert_eq!(*e2.get_target_node(), n2);
    assert_eq!(*e3.get_target_node(), n3);

    n1.remove_edge(&e2);
  }
}