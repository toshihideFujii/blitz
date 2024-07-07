#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use hlo::hlo_value::HloValue;



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
  is_phi: bool,
  users: Vec<Node>,
  operands: Vec<Node>,
  value_id: i64,
  mark_as_dead: bool
}

impl Node {
  pub fn new() -> Self {
    Node {
      is_phi: false,
      users: Vec::new(),
      operands: Vec::new(),
      value_id: 0,
      mark_as_dead: false
    }
  }
}

// Phi graph is a graph that contains and connects phi nodes build on top of
// HloValues with explicit edges, as well as non-phi nodes that are direct
// inputs to the phi nodes.
pub struct PhiGraph {
  node_to_value_id: HashMap<Node, Vec<i64>>,
  value_id_to_node: HashMap<i64, Node>,
  node_strage: Vec<Node>
}

impl PhiGraph {
  pub fn new() -> Self {
    PhiGraph {
      node_to_value_id: HashMap::new(),
      value_id_to_node: HashMap::new(),
      node_strage: Vec::new()
    }
  }

  // Register an hlo value into the phi node.
  pub fn register_phi(&mut self, value: &HloValue) -> &mut Node {
    let node = self.create_or_reuse_node(value);
    debug_assert!(value.is_phi());
    node.is_phi = true;
    node.operands.clear();
    //for input in inputs {
      //let input_node = self.create_or_reuse_node(input);
      //node.operands.push(input_node.clone());
    //}
    node
  }

  // Splitted from reigster_phi to avoid compile error.
  pub fn register_phi_operands(&mut self, node: &mut Node, inputs: &Vec<HloValue>) {
    for input in inputs {
      let input_node = self.create_or_reuse_node(input);
      node.operands.push(input_node.clone());
    }
  }

  pub fn get_optimized_id(&self, value: &HloValue) -> i64 {
    let node = self.value_id_to_node.get(&value.id());
    debug_assert!(node.is_some());
    debug_assert!(!node.as_ref().unwrap().mark_as_dead);
    node.as_ref().unwrap().value_id
  }

  // Returns true if the input to a hlo value is the same as 'inputs'.
  pub fn inputs_equal_to(&self, value: &HloValue, inputs: &Vec<HloValue>) -> bool {
    let node = self.value_id_to_node.get(&value.id());
    debug_assert!(node.is_some());

    let mut existing_set = HashSet::new();
    for operand in &node.as_ref().unwrap().operands {
      existing_set.insert(operand.value_id);
    }

    let mut new_set = HashSet::new();
    for input in inputs {
      new_set.insert(input.id());
    }

    existing_set == new_set
  }

  // Given 'id', returns the new id that 'id' shpuld be replaced with.
  pub fn find_optimized_value(&self, id: i64) -> i64 {
    let opt_id = self.value_id_to_node.get(&id);
    debug_assert!(opt_id.is_some());
    debug_assert!(!opt_id.as_ref().unwrap().mark_as_dead);
    opt_id.as_ref().unwrap().value_id
  }

  // Optimize the entire graph.
  pub fn optimize(&mut self) {}

  pub fn to_string(&self) -> String {
    let mut out = "PhiGraph: \n".to_string();
    for node in &self.node_strage {
      out.push_str(node.value_id.to_string().as_str());
      if node.is_phi {
        out.push_str(", phi");
      }
      if node.mark_as_dead {
        out.push_str(", dead:\n"); // CHECK
      }
      for input in &node.operands {
        out.push_str("  ");
        out.push_str(input.value_id.to_string().as_str());
        out.push_str("\n");
      }
    }
    out
  }

  fn create_or_reuse_node(&mut self, value: &HloValue) -> &mut Node {
    if !self.value_id_to_node.contains_key(&value.id()) {
      let mut node = Node::new();
      node.value_id = value.id();
      self.node_strage.push(node.clone());

      let mut node_id_vec = self.node_to_value_id.get_mut(&node.clone());
      node_id_vec.as_mut().unwrap().push(value.id());

      self.value_id_to_node.insert(value.id(), node);
      self.value_id_to_node.get_mut(&value.id()).unwrap()
    } else {
      self.value_id_to_node.get_mut(&value.id()).unwrap()
    }
  }

  // Replace 'node' with 'replace'.
  fn replace_node_with(&self, node: &Node, replace: &Node) {
    // Updat users.
    debug_assert!(node.is_phi);
    if node.mark_as_dead {
      // The node has already been replaced with another.
      return;
    }
    if replace.mark_as_dead {
      // The node we are placing with has already been replaced with another node.
      let replace_node = self.value_id_to_node.get(&replace.value_id);
      debug_assert!(replace_node.is_some());
      self.replace_node_with(node, replace_node.as_ref().unwrap());
    }
  }
}