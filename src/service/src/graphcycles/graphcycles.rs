#![allow(dead_code)]

struct Node {
  rank: i64,
  visited: bool,
}

impl Node {
  pub fn new() -> Self {
    Node {
      rank: 0,
      visited: false
    }
  }
}

struct NodeIO {

}

struct Rep {
  nodes: Vec<Node>,
  node_io: Vec<NodeIO>,
  free_nodes: Vec<i64>,
  deltaf: Vec<i64>,
  deltab: Vec<i64>,
  list: Vec<i64>,
  merged: Vec<i64>,
  stack: Vec<i64>
}

impl Rep {
  pub fn new() -> Self {
    Rep {
      nodes: Vec::new(),
      node_io: Vec::new(),
      free_nodes: Vec::new(),
      deltaf: Vec::new(),
      deltab: Vec::new(),
      list: Vec::new(),
      merged: Vec::new(),
      stack: Vec::new()
    }
  }
}

pub struct GraphCycles {
  rep: Rep
}

impl GraphCycles {
  pub fn new() {}

  pub fn new_node(&mut self) -> i64 {
    if self.rep.free_nodes.is_empty() {
      let mut n = Node::new();
      n.rank = self.rep.nodes.len() as i64;
      // TODO
      n.rank
    } else {
      let r = self.rep.free_nodes.pop().unwrap();
      // rep.node_data[r] = None
      r
    }
  }

  pub fn remove_node() {}
  pub fn insert_edge() {}
  pub fn remove_edge() {}
  pub fn has_edge() {}
  pub fn contract_edge() {}
  pub fn can_contract_edge() {}
  pub fn is_reachable() {}
  pub fn is_reachable_non_const() {}
  pub fn get_node_data() {}
  pub fn set_node_data() {}
  pub fn find_path() {}
  pub fn check_invariants() {}
  pub fn successors() {}
  pub fn predecessors() {}
  pub fn successors_copy() {}
  pub fn predecessors_copy() {}
  pub fn all_nodes_in_post_order() {}
  pub fn debug_string() {}
}