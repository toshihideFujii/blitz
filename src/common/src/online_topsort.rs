#![allow(dead_code)]

// The topological sort is an intrusive data structure. Nodes of type T that
// participate in the topological sort must have a TopologicalSortNode<T>
// embedded within them.
#[derive(Clone, PartialEq)]
pub struct TopologicalSortNode<T> {
  pub index: i64,
  pub level: i64,
  pub next: Option<T>,
  pub prev: Option<Box<TopologicalSortNode<T>>>
}

impl<T> TopologicalSortNode<T> {
  pub fn default() -> Self {
    TopologicalSortNode { index: -1, level: -1, next: None, prev: None }
  }

  pub fn clear(&mut self) {
    self.next = None;
    self.prev = None;
    self.level = -1;
    self.index = -1;
  }

  // Returns true if this node has been added to a topological order.
  // It may have temporarily been removed from a specific location in that
  // order if we are in the middle of an AddEdge() operation.
  pub fn in_topological_order(&self) -> bool {
    self.level >= 0
  }
}

#[derive(Clone)]
pub struct TopologicalSort<T> {
  node: TopologicalSortNode<T>,
  num_edges: i64,
  num_nodes: i64,
  // How many nodes to search backwards when adding an edge. This should be
  // ceil(min(m**(1/2), n**(2/3))), but we compute that bound online as we add
  // nodes and edges via UpdateDelta().
  delta: i64,
  // The next value of index_ to assign, aka "a" in the paper. Monotonically
  // decreasing as indices are assigned.
  // You might also wonder where 'b' from the paper is, but we simply don't
  // need it, since we're trying to maintain a doubly-linked list in topological
  // order, and we don't care about computing a topological numbering.
  next_index: i64,
  // The first node in each level or a higher level.
  // As is the usual convention for this data structure, this is actually the
  // TopologicalSortNode whose next_ pointer points to that node, if any.
  // Invariant: There is always at least one level. Futher, these pointers are
  // never nullptr: there's always a preceding node (node_, if nothing else).
  first_in_level: Vec<TopologicalSortNode<T>>,
  // Visited state for forwards and backwards searches which are used during
  // AddEdge(). We keep this state in the class to save repeatedly allocating
  // it. This would not be thread-safe, but neither is AddEdge().
  visited_backwards: Vec<bool>,
  visited_forwards: Vec<bool>,
  increased: Vec<bool>
}

impl<T: Clone + PartialEq> TopologicalSort<T> {
  pub fn default() {
      
  }

  // Invalidates iterators.
  pub fn add_node(&mut self, v: T) {

    // next_ and prev_ should be nullptr for a new node.
    assert!(self.node.next == None);
    assert!(self.node.prev == None);
    self.node.level = 0;
    self.node.index = self.next_index;
    self.next_index += 1;
    self.num_edges += 1;
    self.update_delta();

    // Add the node to the front of the topological ordering.
    self.node.next = Some(self.first_in_level[0].next.as_ref().unwrap().clone());
    self.node.prev = Some(Box::new(self.first_in_level[0].clone()));
    if self.node.next.is_some() {

    } else {
      self.node.prev = Some(Box::new(self.node.clone()));
    }
    self.first_in_level[0].next = Some(v);
    for level in 1..self.first_in_level.len() {
      if self.first_in_level[level] == self.node {
        self.first_in_level[level] = self.node.clone();
      }
    }
  }

  // Invalidates iterators.
  pub fn remove_node(&mut self, v: T) {
    assert!(self.node.prev.as_ref().unwrap().as_ref() == &self.node ||
      self.node.prev.as_ref().unwrap().in_topological_order());
    self.num_nodes -= 1;

    self.remove_from_order(v);
    self.node.level = -1;
    self.node.index = -1;
  }

  pub fn add_edge() {
      
  }

  pub fn log_order() {
      
  }

  pub fn clear() {
      
  }

  fn update_delta(&mut self) {
    let m = self.num_edges;
    let n = self.num_nodes;
    // delta should be ceil(min(m**(1/2), n**(2/3)))
    while self.delta * self.delta < m &&
      self.delta * self.delta * self.delta < n * n {
      self.delta += 1;   
    }
  }

  fn search_backwards() {
      
  }

  fn search_forwards() {
      
  }

  fn remove_from_order(&self, _v: T) {
      
  }

  fn update_index() {
      
  }

  fn update_max_index_in_parent() {
      
  }
}