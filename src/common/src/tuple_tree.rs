#![allow(dead_code)]

use std::fmt::Debug;

use crate::shape::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
  node_id: usize,
  children_start_id: i64,
  num_children: usize
}

impl Entry {
  pub fn default() -> Self {
    Entry { node_id: 0, children_start_id: -1, num_children: 0 }
  }
}

// Index table for TupleTree.
#[derive(Debug, PartialEq)]
pub struct IndexTable {
  entries: Option<Vec<Entry>>
}

impl IndexTable {
  pub fn default() -> Self {
    IndexTable { entries: Some(Vec::new()) }
  }

  pub fn new<T>(root: &Node<T>) -> Self where T: Default {
    let mut table = IndexTable::default();
    table.initialize(root);
    table
  }

  pub fn new_from_shape(shape: &Shape) -> Self {
    if !shape.is_tuple() {
      let mut instance = IndexTable::default();
      instance.entries.as_mut().unwrap().push(Entry::default());
      return instance;
    }
    let mut instance = IndexTable::default();
    let next_node_id = 0;
    let next_children_start_index = 1;
    IndexTable::initialize_index_table(
      shape,
      instance.entries.as_mut().unwrap(),
      0,
      next_node_id,
      next_children_start_index);
    instance
  }

  pub fn copy(other: &IndexTable) -> Self {
    let mut entries = vec![];
    debug_assert!(other.entries.is_some());
    entries.clone_from(other.entries.as_ref().unwrap());
    IndexTable { entries: Some(entries) }
  }

  pub fn get_entry(&self, index: &Vec<i64>) -> Result<&Entry, String> {
    if self.entries.is_none() {
      return Err("Index table not initialized".to_string());
    }
    let mut current = &self.entries.as_ref().unwrap()[0];
    for i in index {
      if *i < 0 {
        return Err("Negative index in shape_index".to_string());
      }
      if current.children_start_id == -1 {
        return Err("Cannot index into a leaf node".to_string());
      }
      if *i >= current.num_children as i64 {
        return Err("Index out of bounds".to_string());
      }
      current = &self.entries.as_ref().unwrap()[(current.children_start_id + i) as usize];
    }
    Ok(current)
  }

  pub fn entries(&self) -> &Option<Vec<Entry>> {
    &self.entries
  }

  pub fn new_from_subtree(
    original_table: &IndexTable, index: &Vec<i64>) -> Result<Self, String>
  {
    let root_entry = original_table.get_entry(index);
    if root_entry.is_err() {
      return Err(root_entry.err().unwrap());
    }
    let num_nodes = IndexTable::count_subtree_nodes(original_table,
      root_entry.as_ref().unwrap());
    if num_nodes == 0 {
      return Err("Subtree is empty".to_string());
    }

    let mut new_table = IndexTable::default();
    for _i in 0..num_nodes {
      new_table.entries.as_mut().unwrap().push(Entry::default());    
    }

    let mut next_node_id = 0;
    let mut next_children_start_index = 1;

    IndexTable::build_sub_table_impl(
      original_table,
      root_entry.as_ref().unwrap(),
      new_table.entries.as_mut().unwrap(),
      0,
      &mut next_node_id,
      &mut next_children_start_index);

    Ok(new_table)
  }

  // Checks if two subtrees rooted at the given entries are structurally
  // compatible.
  pub fn is_subtree_compatible(
    other_table: &IndexTable, other_entry: &Entry,
    this_table: &IndexTable, this_entry: &Entry) -> Result<(), String>
  {
    if other_entry.num_children != this_entry.num_children {
      let err_msg = String::from(
        "Subtree structures incompatible: different number of childre");
      return Err(err_msg);
    }
    if other_entry.children_start_id == -1 { // Leaf nodes
      return Ok(());
    }

    let other_entries = other_table.entries.as_ref().unwrap();
    let this_entries = this_table.entries.as_ref().unwrap();

    for i in 0..other_entry.num_children {
      let other_child =
        &other_entries[(other_entry.children_start_id as usize) + i];   
      let this_child =
        &this_entries[(this_entry.children_start_id as usize) + i];
      let result = IndexTable::is_subtree_compatible(
        other_table, other_child, this_table, this_child);
      if result.is_err() {
        return Err(result.err().unwrap());
      }
    }
    Ok(())
  }

  // Counts the number of nodes in the subtree rooted at root_entry.
  pub fn count_subtree_nodes(table: &IndexTable, root_entry: &Entry) -> usize {
    if table.entries.is_none() {
      return 0;
    }
    if root_entry.children_start_id == -1 {
      return 1;
    }
    let mut count = 1;
    for i in 0..root_entry.num_children {
      let child_entry =
        &table.entries.as_ref().unwrap()[(root_entry.children_start_id as usize) + i];
      count += IndexTable::count_subtree_nodes(table, child_entry);
    }
    count
  }

  // Computes the total size of all nested tuples in the given tuple shape.
  fn index_talbe_tuples_size(shape: &Shape) -> usize {
    debug_assert!(shape.is_tuple(), "Shape must be a tuple");

    let mut size = shape.tuple_shapes_size();
    for subshape in shape.tuple_shapes_vec() {
      if subshape.is_tuple() {
        size += IndexTable::index_talbe_tuples_size(subshape);
      }
    }
    size
  }

  // Initializes the index table in the given entries span. Span must point into
  // the appropriately sized entries storage.
  fn initialize_index_table(
    shape: &Shape,
    entries: &mut Vec<Entry>,
    entry_index: usize,
    mut next_node_id: usize,
    mut next_children_start_index: usize)
  {
    let entry = &mut entries[entry_index];
    entry.node_id = next_node_id;
    next_node_id += 1;

    // Stop shape traversal once reaching a leaf shape.
    if !shape.is_tuple() {
      entry.children_start_id = -1;
      entry.num_children = 0;
      return;
    }

    // The nodes are in depth-first pre-order. However, in order to efficiently
    // lookup indices, we generate the index table using breadth-first.
    entry.children_start_id = next_children_start_index as i64;
    entry.num_children = shape.tuple_shapes_size();

    // Add entry for children first, before recursing, so they are consecutive.
    next_children_start_index += shape.tuple_shapes_size();
    let entry_index = entry.children_start_id as usize;
    for i in 0..shape.tuple_shapes_size() {
      IndexTable::initialize_index_table(
        shape.tuple_shapes(i),
        entries,
        entry_index + i,
        next_node_id,
        next_children_start_index);
    }
  }

  fn count_nodes<T>(node: &Node<T>) -> usize
    where T: Default
  {
    let mut count = 1;
    if !node.is_leaf() {
      for child in node.children() {
        count += IndexTable::count_nodes(child);
      }
    }
    count
  }

  fn initialize<T>(&mut self, root: &Node<T>) where T: Default {
    let num_nodes = IndexTable::count_nodes(root);
    self.entries.as_mut().unwrap().resize(num_nodes, Entry::default());

    let mut next_node_id = 0;
    let mut next_children_start_index = 1;
    IndexTable::build_table(root, self.entries.as_mut().unwrap(),
      0, &mut next_node_id, &mut next_children_start_index);
  }

  fn build_table<T>(
    node: &Node<T>,
    entries: &mut Vec<Entry>,
    current_entry_idx: usize,
    next_node_id: &mut usize,
    next_children_start_index: &mut usize) where T: Default
  {
    let entry = &mut entries[current_entry_idx];
    entry.node_id = *next_node_id;
    *next_node_id += 1;

    let entry = &mut entries[current_entry_idx];
    if node.is_leaf() {
      entry.children_start_id = -1;
      entry.num_children = 0;
      return;
    }

    // !node.IsLeaf(), so it's a tuple node (possibly empty).
    let children = node.children();
    entry.num_children = children.len();
    entry.children_start_id = *next_children_start_index as i64;

    let my_children_start = *next_children_start_index;
    *next_children_start_index += entry.num_children;

    for i in 0..children.len() {
      IndexTable::build_table(
        &children[i],
        entries,
        my_children_start + i,
        next_node_id,
        next_children_start_index);
    }
  }

  // Helper function to recursively build the IndexTable for a subtree.
  fn build_sub_table_impl(
    original_table: &IndexTable,
    original_entry: &Entry,
    new_entries: &mut Vec<Entry>,
    current_new_entry_idx: usize,
    next_node_id: &mut usize,
    next_children_start_index: &mut i64)
  {
    let new_entry = &mut new_entries[current_new_entry_idx];    
    new_entry.node_id = next_node_id.clone();
    *next_node_id += 1;
    new_entry.num_children = original_entry.num_children;

    if original_entry.children_start_id == -1 {
      new_entry.children_start_id = -1;
      return;
    }

    new_entry.children_start_id = next_children_start_index.clone();
    let my_children_start = next_children_start_index.clone();
    *next_children_start_index += new_entry.num_children as i64;

    let original_entries = original_table.entries().as_ref().unwrap();
    for i in 0..new_entry.num_children {
      let original_child_entry =
        &original_entries[(original_entry.children_start_id as usize) + i];
      IndexTable::build_sub_table_impl(
        original_table,
        original_child_entry,
        new_entries,
        (my_children_start as usize) + i,
        next_node_id,
        next_children_start_index);
    }
  }
}

// Represents a node in the tree. It can be either a leaf value of type T
// or a vector of subtrees (inner tuple).
// This class is used for constructing TupleTree instances, defining the
// structure and initial values.
#[derive(Clone)]
pub struct Node<T> {
  value: Option<T>,
  children: Option<Vec<Node<T>>>
}

impl<T> Node<T> where T: Default {
  // Static factory for leaf nodes.
  pub fn leaf(value: T) -> Self {
    Node::new(Some(value), None)
  }

  // Static factories for tuple nodes.
  pub fn tuple_default() -> Self {
    Node::new(Some(T::default()), Some(Vec::new()))
  }

  pub fn tuple_value(value: T) -> Self {
    Node::new(Some(value), Some(Vec::new()))
  }

  pub fn tuple_children(children: Vec<Node<T>>) -> Self {
    Node::new(Some(T::default()), Some(children))
  }

  pub fn tuple_value_children(value: T, children: Vec<Node<T>>) -> Self {
    Node::new(Some(value), Some(children))
  }

  // Default constructor creates an empty tuple.
  pub fn default() -> Self {
    Node { value: None, children: Some(Vec::new()) }
  }

  pub fn is_leaf(&self) -> bool {
    self.children.is_none()
  }

  pub fn value(&self) -> &T {
    debug_assert!(self.value.is_some());
    self.value.as_ref().unwrap()
  }

  pub fn mutable_value(&mut self) -> &mut T {
    debug_assert!(self.value.is_some());
    self.value.as_mut().unwrap()
  }

  pub fn children(&self) -> &Vec<Node<T>> {
    debug_assert!(self.children.is_some());
    self.children.as_ref().unwrap()
  }

  pub fn mutable_children(&mut self) -> &mut Vec<Node<T>> {
    debug_assert!(self.children.is_some());
    self.children.as_mut().unwrap()
  }

  fn new(value: Option<T>, children: Option<Vec<Node<T>>>) -> Self {
    Node {
      value: value,
      children: children
    }
  }
}

// A TupleTree<T> is a tree data structure where each node, whether an
// internal node (tuple) or a leaf node, holds a value of type T. The structure
// is defined by the nesting of tuples.
//
// Key Characteristics:
// - Each node in the tree has an associated value of type T.
// - Nodes can be either internal nodes (tuples with children) or leaf nodes
//   (no children).
// - The structure is independent of XLA Shapes.
// - Internal nodes' values are distinct from their children's values.
//
// Non-obvious Behaviors:
// - Constructor from Span of Pairs: When constructing from a span of
//   {ShapeIndex, T} pairs, only the nodes at the specified indices are
//   initialized with the given values. Any necessary ancestor tuple nodes are
//   implicitly created and their values are constructed from the provided
//   arguments.
// - `element()` and `mutable_element()`: These methods can access the value
//   of *any* node in the tree, not just leaves, using its ShapeIndex.
// - `Map` and `MapWithStatus`: These functions apply the given function to the
//   values of *all* nodes in the tree, including internal tuple nodes.
// - Iterators:
//     - `begin()`/`end()` (and `nodes()`): Iterate over all nodes in the tree
//       in pre-order.
//     - `leaf_begin()`/`leaf_end()` (and `leaves()`): Iterate only over nodes
//       for which `IsLeaf()` is true (i.e., nodes with no children).
#[derive(Debug, PartialEq)]
pub struct TupleTree<T> {
  pub nodes: Vec<(Vec<i64>, T)>,
  index_table: IndexTable
}

impl<T> TupleTree<T> where T: Debug + Default + Clone {
  // Constructor for an empty tuple.
  pub fn default() -> Self {
    let mut instance = TupleTree {
      nodes: Vec::new(),
      index_table: IndexTable::default()
    };
    instance.initialize(&mut Node::tuple_default());
    instance
  }

  // Constructor for a single leaf node.
  pub fn new_from_leaf_value(leaf_value: T) -> Self {
    let mut instance = TupleTree {
      nodes: Vec::new(),
      index_table: IndexTable::default()
    };
    instance.initialize(&mut Node::leaf(leaf_value));
    instance
  }

  pub fn new_from_items(items: &Vec<T>) -> Self {
    let mut children = vec![];
    for item in items {
      children.push(Node::leaf(item.clone()));
    }
    let mut instance = TupleTree {
      nodes: Vec::new(),
      index_table: IndexTable::default()
    };
    instance.initialize(&mut Node::tuple_children(children));
    instance
  }

  // Basic constructor taking the root node.
  pub fn new_from_node(root: &mut Node<T>) -> Self {
    let mut instance = TupleTree {
      nodes: Vec::new(),
      index_table: IndexTable::default()
    };
    instance.initialize(root);
    instance
  }

  pub fn new_from_nodes(children: Vec<Node<T>>) -> Self {
    let mut instance = TupleTree {
      nodes: Vec::new(),
      index_table: IndexTable::default()
    };
    instance.initialize(&mut Node::tuple_children(children));
    instance
  }

  pub fn new_from_pairs(pairs: Vec<(Vec<i64>, T)>) -> Self {
    let mut instance: TupleTree<T> = TupleTree {
      nodes: Vec::new(),
      index_table: IndexTable::default()
    };
    let mut root_node = Node::tuple_default();
    for pair in &pairs {
      let node = TupleTree::get_or_create_node(
        &mut root_node, &pair.0, false);
      *node = Node::leaf(pair.1.clone());
    }
    instance.initialize(&mut root_node);
    instance
  }

  pub fn new_from_pairs_with_default_value(
    pairs: Vec<(Vec<i64>, T)>, default_value: T) -> Self
  {
    let mut instance: TupleTree<T> = TupleTree {
      nodes: Vec::new(),
      index_table: IndexTable::default()
    };
    let mut root_node = Node::tuple_value(default_value);
    for pair in &pairs {
      let node = TupleTree::get_or_create_node(
        &mut root_node, &pair.0, false);
      *node = Node::leaf(pair.1.clone());
    }
    instance.initialize(&mut root_node);
    instance
  }

  // Returns the data element at the given index. This works for any valid
  // index, whether it's an internal node or a leaf node.
  // CHECK-fails if the index is invalid.
  pub fn element(&self, index: &Vec<i64>) -> &T {
    let entry = self.index_table.get_entry(index);
    debug_assert!(entry.is_ok());
    &self.nodes[entry.unwrap().node_id].1
  }

  // Returns a pointer to the data element at the given index. This works for
  // any valid index, whether it's an internal node or a leaf node.
  // CHECK-fails if the index is invalid.
  pub fn mutable_element(&mut self, index: &Vec<i64>) -> &mut T {
    let entry = self.index_table.get_entry(index);
    debug_assert!(entry.is_ok());
    &mut self.nodes[entry.unwrap().node_id].1
  }

  // Returns true if the node at the given index is a leaf node (has no
  // children).
  pub fn is_leaf(&self, index: &Vec<i64>) -> bool {
    let entry = self.index_table.get_entry(index);
    if entry.is_err() { return false; }
    entry.unwrap().children_start_id == -1
  }

  // Checks if the structure of this TupleTree is compatible with the given
  // shape.
  pub fn is_structurally_compatible(&self, shape: &Shape) -> bool {
    let shape_table = IndexTable::new_from_shape(shape);
    let shape_root_or = shape_table.get_entry(&vec![]);
    let tree_root_or = self.index_table.get_entry(&vec![]);
    if shape_root_or.is_err() || tree_root_or.is_err() {
      return false;
    }
    IndexTable::is_subtree_compatible(
      &shape_table, 
      shape_root_or.as_ref().unwrap(), 
      &self.index_table,
      tree_root_or.as_ref().unwrap()).is_ok()
  }

  pub fn is_tuple(&self) -> bool {
    self.nodes.len() > 1
  }

  pub fn copy_compatible_subtree_from(
    &mut self,
    other: &TupleTree<T>,
    src_index: &Vec<i64>,
    dst_index: &Vec<i64>) -> Result<(), String>
  {
    let src_entry = other.index_table.get_entry(src_index);
    if src_entry.is_err() {
      return Err(src_entry.err().unwrap());
    }
    let dst_entry = self.index_table.get_entry(dst_index);
    if dst_entry.is_err() {
      return Err(dst_entry.err().unwrap());
    }
    let result = IndexTable::is_subtree_compatible(
      &other.index_table,
      src_entry.as_ref().unwrap(),
      &self.index_table,
      dst_entry.as_ref().unwrap());
    if result.is_err() {
      return Err(result.err().unwrap());
    }
    let num_subtree_nodes = IndexTable::count_subtree_nodes(&other.index_table,
      src_entry.as_ref().unwrap());
    
    for i in 0..num_subtree_nodes {
      let src_pair =
        &other.nodes[src_entry.as_ref().unwrap().node_id + i];
      let dst_pair =
        &mut self.nodes[dst_entry.as_ref().unwrap().node_id + i];
      dst_pair.1 = src_pair.1.clone();
    }
    Ok(())
  }

  pub fn copy_subtree_from(
    &mut self,
    other: &TupleTree<T>,
    src_index: &mut Vec<i64>,
    dst_index: &Vec<i64>)
  {
    let src_node_or = other.to_node(src_index);
    debug_assert!(src_node_or.is_ok());
    let mut src_node = src_node_or.unwrap();

    if dst_index.is_empty() {
      self.initialize(&mut src_node);
      return;
    }

    let root_node_or = self.to_node(&mut vec![]);
    debug_assert!(root_node_or.is_ok());
    let mut root_node = root_node_or.unwrap();

    //#[allow(unused_variables)]
    let target_node = TupleTree::get_or_create_node(
      &mut root_node, dst_index, true);
    *target_node = src_node;
    
    self.initialize(&mut root_node);
  }

  pub fn subtree(&self, index: &Vec<i64>) -> Result<Self, String> {
    let root_entry = self.index_table.get_entry(index);
    if root_entry.is_err() {
      return Err(root_entry.err().unwrap());
    }

    let root_node_id = root_entry.as_ref().unwrap().node_id;
    let subtree_index_table =
      IndexTable::new_from_subtree(&self.index_table, index);
    if subtree_index_table.is_err() {
      return Err(subtree_index_table.err().unwrap());
    }
    if subtree_index_table.as_ref().unwrap().entries.is_none() {
      return Err("Subtree index table creation failed".to_string());
    }

    let num_subtree_nodes = subtree_index_table.as_ref().unwrap().
      entries.as_ref().unwrap().len();
    let mut subtree_nodes = vec![];
    subtree_nodes.reserve(num_subtree_nodes);

    for i in 0..num_subtree_nodes {
      let original_pair = &self.nodes[root_node_id + i];
      let mut new_index = vec![];
      for j in index.len()..original_pair.0.len() { // TODO
        new_index.push(original_pair.0[j].clone());
      }
      subtree_nodes.push((new_index, original_pair.1.clone()));
    }
    Ok(TupleTree::new_from_table_and_nodes(
      subtree_index_table.as_ref().unwrap(), subtree_nodes))
  }

  pub fn num_leaves(&self) -> usize {
    let mut num_leaves= 0;
    for node in &self.nodes {
      num_leaves += node.0.len();
    }
    num_leaves
  }

  // Returns an iterator pointing to the node at the given ShapeIndex.
  // Returns end() if the index is not found.
  pub fn find(&self, index: &Vec<i64>) -> Option<T> {
    let entry_or = self.index_table.get_entry(index);
    if entry_or.is_err() {
      return None;
    }
    for node in &self.nodes {
      for node_id in &node.0 {
        if *node_id == entry_or.as_ref().unwrap().node_id as i64 {
          return Some(node.1.clone());
        }
      }
    }
    None
  }

  // Traversal functions for all elements. These iterate over ALL nodes.
  pub fn for_each_element(&self, func: &mut dyn FnMut(&Vec<i64>, &T)) {
    for node in &self.nodes {
      func(&node.0, &node.1);
    }
  }

  pub fn for_each_mutable_element(&mut self, func: &mut dyn FnMut(&Vec<i64>, &mut T)) {
    for node in &mut self.nodes {
      func(&node.0, &mut node.1);
    }
  }

  pub fn for_each_element_with_status(
    &self,
    func: &dyn Fn(&Vec<i64>, &T)->Result<(), String>) -> Result<(), String>
  {
    for node in &self.nodes {
      let result = func(&node.0, &node.1);
      if result.is_err() { return Err(result.err().unwrap()); }
    }
    Ok(())
  }

  pub fn for_each_mutable_element_with_status(
    &mut self,
    func: &mut dyn FnMut(&Vec<i64>, &T)->Result<(), String>) -> Result<(), String>
  {
    for node in &self.nodes {
      let result = func(&node.0, &node.1);
      if result.is_err() { return Err(result.err().unwrap()); }
    }
    Ok(())
  }

  // Returns a const range to iterate over all nodes in pre-order.
  pub fn nodes(&self) -> &Vec<(Vec<i64>, T)> {
    &self.nodes
  }

  pub fn mutable_nodes(&mut self) -> &mut Vec<(Vec<i64>, T)> {
    &mut self.nodes
  }

  pub fn leaves(&self) -> Vec<&(Vec<i64>, T)> {
    let mut leaves = vec![];
    for node in &self.nodes {
      if self.is_leaf(&node.0) {
        leaves.push(node);
      }
    }
    leaves
  }

  pub fn mutable_leaves(&mut self) -> Vec<&mut (Vec<i64>, T)> {
    let mut leaves = vec![];
    for node in &mut self.nodes {
      let entry = self.index_table.get_entry(&node.0);
      if entry.is_err() { continue; }
      if entry.unwrap().children_start_id == -1 {
        leaves.push(node);
      }
    }
    leaves
  }

  // Maps each node's value to generate a new tree with the same structure.
  // The function `func` is applied to the value of *every* node.
  pub fn map<U>(&self, func: &dyn Fn(&T)->U) -> TupleTree<U>
    where U: Debug + Default + Clone
  {
    let mut result_nodes = vec![];
    result_nodes.reserve(self.nodes.len());
    for node in &self.nodes {
      let mut index = vec![];
      index.clone_from(&node.0);
      let value = func(&node.1);
      result_nodes.push((index, value));
    }
    TupleTree::new_from_table_and_nodes(&self.index_table, result_nodes)
  }

  // Maps each node's value to generate a new tree with the same structure,
  // allowing the mapping function to return a StatusOr.
  // The function `func` is applied to the value of *every* node.
  pub fn map_with_status<U>(
    &self,
    func: &dyn Fn(&T)->Result<U, String>) -> Result<TupleTree<U>, String>
    where U: Debug + Default + Clone
  {
    let mut result_nodes = vec![];
    result_nodes.reserve(self.nodes.len());
    for node in &self.nodes {
      let mut index = vec![];
      index.clone_from(&node.0);
      let value = func(&node.1);
      if value.is_err() {
        return Err(value.err().unwrap());
      }
      result_nodes.push((index, value.ok().unwrap()));
    }
    Ok(TupleTree::new_from_table_and_nodes(&self.index_table, result_nodes))
  }

  pub fn to_node(&self, index: &Vec<i64>) -> Result<Node<T>, String> {
    if self.index_table.entries.is_none(){
      return Ok(Node::tuple_value(T::default()));
    }
    self.to_node_impl(index)
  }

  fn to_node_impl(&self, index: &Vec<i64>) -> Result<Node<T>, String> {
    let entry = self.index_table.get_entry(index);
    if entry.is_err() {
      return Err(entry.err().unwrap());
    }
    let value = &self.nodes[entry.as_ref().unwrap().node_id].1;
    if entry.as_ref().unwrap().children_start_id == -1 {
      return Ok(Node::leaf(value.clone()));
    }
    // Is an internal tuple node
    let mut children = vec![];
    children.reserve(entry.as_ref().unwrap().num_children);
    let mut child_index = vec![];
    child_index.clone_from(index);
    child_index.push(0);
    for i in 0..entry.as_ref().unwrap().num_children {
      let last = child_index.last_mut();
       *last.unwrap() = i as i64;
      let child_node = self.to_node_impl(&child_index);
      if child_node.is_err() { return Err(child_node.err().unwrap()); }
      children.push(child_node.unwrap());
    }
    Ok(Node::tuple_value_children(value.clone(), children))
  }

  // Private constructor for internal use (e.g., Map).
  fn new_from_table_and_nodes(table: &IndexTable, nodes: Vec<(Vec<i64>, T)>) -> Self {
    let index_table = IndexTable::copy(table);
    TupleTree { nodes: nodes, index_table: index_table }
  }

  fn initialize(&mut self, root: &mut Node<T>) {
    // First, build the IndexTable from the structure.
    self.index_table = IndexTable::new(root);

    // Then, build the nodes_ vector, moving values from root.
    self.nodes.clear();
    if self.index_table.entries().is_some() {
      self.nodes.reserve(self.index_table.entries().as_ref().unwrap().len());
    }
    let mut current_index = vec![];
    self.build_nodes_vector(root, &mut current_index);
  }

  fn build_nodes_vector(&mut self, node: &mut Node<T>, current_index: &mut Vec<i64>) {
    let mut index = vec![];
    index.clone_from(current_index);
    self.nodes.push((index, node.value().clone()));
    if !node.is_leaf() {
      let children = node.mutable_children();
      for i in 0..children.len() {
        current_index.push(i as i64);
        self.build_nodes_vector(&mut children[i], current_index);
        current_index.pop();
      }
    }
  }

  fn get_or_create_node<'a>(
    root: &'a mut Node<T>,
    index: &Vec<i64>,
    preserve_leaf_value: bool) -> &'a mut Node<T>
  {
    let mut node = root;
    if index.is_empty() {
      return node;
    }
    for idx in index {
      debug_assert!(*idx >= 0);
      if node.is_leaf() {
        // Transition from leaf to tuple.
        if preserve_leaf_value {
          let original_value = node.mutable_value().clone();
          //*node = Node::tuple_default(); // TODO
          *node = Node::tuple_children(vec![Node::leaf(original_value)]);
          // The original leaf value is placed at index 0.
          //node.mutable_children().push(Node::leaf(original_value));
        } else {
          *node = Node::tuple_default(); // TODO
        }
      }
      let children = node.mutable_children();
      while *idx >= children.len() as i64 {
        children.push(Node::tuple_default()); // TODO
      }
      node = &mut children[*idx as usize];
    }
    node
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_single_leaf_constructor() {
    let tree = TupleTree::new_from_leaf_value(42);
    assert!(tree.is_leaf(&vec![]));
    assert_eq!(tree.element(&vec![]), &42);
  }

  #[test]
  fn test_empty_constructor() {
    let tree: TupleTree<i64> = TupleTree::default();
    assert!(!tree.is_leaf(&vec![]));
    assert_eq!(tree.nodes(), &vec![(vec![], 0)]);
    let leaves: Vec<&(Vec<i64>, i64)> = vec![];
    assert_eq!(tree.leaves(), leaves);
    assert_eq!(tree.element(&vec![]), &0);
  }

  #[test]
  fn test_node_with_value_and_children() {
    // Node with value and empty children (empty tuple)
    let tree_1 =
      TupleTree::new_from_node(&mut Node::tuple_value(100));
    assert!(!tree_1.is_leaf(&vec![])); // It's a tuple, not a leaf
    assert_eq!(tree_1.nodes(), &vec![(vec![], 100)]);
    let leaves: Vec<&(Vec<i64>, i64)> = vec![];
    assert_eq!(tree_1.leaves(), leaves);

    // Node with value and non-empty children
    let tree_2 = TupleTree::new_from_node(
      &mut Node::tuple_value_children(
        200, vec![Node::leaf(1), Node::leaf(2)]));
    assert!(!tree_2.is_leaf(&vec![]));
    assert!(tree_2.is_leaf(&vec![0]));
    assert_eq!(tree_2.element(&vec![0]), &1);
    assert!(tree_2.is_leaf(&vec![1]));
    assert_eq!(tree_2.element(&vec![1]), &2);

    assert_eq!(tree_2.nodes(), &vec![(vec![], 200), (vec![0], 1), (vec![1], 2)]);
    assert_eq!(tree_2.leaves(), vec![&(vec![0], 1), &(vec![1], 2)])
  }

  #[test]
  fn test_node_value_and_children_constructors() {
    // Test various constructors
    let tree_1 = TupleTree::new_from_node(
      &mut Node::tuple_value_children(
        10, vec![Node::leaf(1), Node::leaf(2)]));
    assert!(!tree_1.is_leaf(&vec![]));
    assert_eq!(tree_1.element(&vec![0]), &1);
    assert_eq!(tree_1.element(&vec![1]), &2);

    let mut children = vec![];
    children.push(Node::leaf(3));
    children.push(Node::leaf(4));
    let tree_2 = TupleTree::new_from_node(
      &mut Node::tuple_value_children(20, children));
    assert!(!tree_2.is_leaf(&vec![]));
    assert_eq!(tree_2.element(&vec![0]), &3);
    assert_eq!(tree_2.element(&vec![1]), &4);

    let val = 30;
    let tree_3 = TupleTree::new_from_node(
      &mut Node::tuple_value_children(val, vec![Node::leaf(5)]));
    assert!(!tree_3.is_leaf(&vec![]));
    assert_eq!(tree_3.element(&vec![0]), &5);

    let val_2 = 40;
    let mut children_2 = vec![];
    children_2.push(Node::leaf(6));
    let tree_4 = TupleTree::new_from_node(
      &mut Node::tuple_value_children(val_2, children_2));
    assert!(!tree_4.is_leaf(&vec![]));
    assert_eq!(tree_4.element(&vec![0]), &6);
  }

  #[test]
  fn test_distinguish_empty_tuple_from_leaf() {
    // Leaf node
    let leaf_tree = TupleTree::new_from_node(&mut Node::leaf(42));
    assert!(leaf_tree.is_leaf(&vec![]));
    assert_eq!(leaf_tree.element(&vec![]), &42);
    assert_eq!(leaf_tree.nodes(), &vec![(vec![], 42)]);
    assert_eq!(leaf_tree.leaves(), vec![&(vec![], 42)]);

    // Empty tuple node (with a value)
    let empty_tuple_tree =
      TupleTree::new_from_node(&mut Node::tuple_value(100));
    assert!(!empty_tuple_tree.is_leaf(&vec![]));
    assert_eq!(empty_tuple_tree.nodes(), &vec![(vec![], 100)]);
    let empty_leaves: Vec<&(Vec<i64>, i64)> = vec![];
    assert_eq!(empty_tuple_tree.leaves(), empty_leaves);

    // Empty tuple node (without a value, default constructor)
    let default_empty_tuple_tree: TupleTree<i64> = TupleTree::default();
    assert!(!default_empty_tuple_tree.is_leaf(&vec![]));
    assert_eq!(default_empty_tuple_tree.nodes(), &vec![(vec![], 0)]);
    assert_eq!(default_empty_tuple_tree.leaves(), empty_leaves);
  }

  #[test]
  #[should_panic]
  fn test_element_access_death() {
    let tree = TupleTree::new_from_nodes(
      vec![Node::leaf(1),
      Node::tuple_children(vec![Node::leaf(2), Node::leaf(3)])]);
    
    let _ = tree.element(&vec![5]);
    let _ = tree.element(&vec![0, 0]);
  }

  #[test]
  fn test_is_leaf_invalid_index() {
    let tree = TupleTree::new_from_items(&vec![1, 2]);
    assert!(!tree.is_leaf(&vec![5])); // Out of bounds
    assert!(!tree.is_leaf(&vec![0, 0])); // Too deep
  }

  #[test]
  fn test_copy_subtree_from_overwrite_leaf_with_tuple() {
    let src_tree = TupleTree::new_from_items(&vec![10, 20]);
    let mut dst_tree = TupleTree::new_from_leaf_value(5);

    dst_tree.copy_subtree_from(
      &src_tree, &mut vec![], &vec![]);
    assert!(!dst_tree.is_leaf(&vec![]));
    assert_eq!(dst_tree.element(&vec![0]), &10);
    assert_eq!(dst_tree.element(&vec![1]), &20);
  }

  #[test]
  fn test_copy_subtree_from_overwrite_tuple_with_leaf() {
    let src_tree = TupleTree::new_from_leaf_value(10);
    let mut dst_tree = TupleTree::new_from_items(&vec![1, 2]);

    dst_tree.copy_subtree_from(
      &src_tree, &mut vec![], &vec![]);
    assert_eq!(dst_tree.is_leaf(&vec![]), true);
    assert_eq!(dst_tree.element(&vec![]), &10);
  }

  #[test]
  fn test_copy_subtree_from_create_nodes() {
    let src_tree = TupleTree::new_from_leaf_value(10);
    let mut dst_tree = TupleTree::new_from_leaf_value(5);

    // Graft src_tree at {1, 0}, creating node {1}
    dst_tree.copy_subtree_from(
      &src_tree, &mut vec![], &vec![1, 0]);
    assert_eq!(dst_tree.is_leaf(&vec![0]), true);
    assert_eq!(dst_tree.element(&vec![0]), &5);
    assert_eq!(dst_tree.is_leaf(&vec![1]), false);
    assert_eq!(dst_tree.is_leaf(&vec![1, 0]), true);
    assert_eq!(dst_tree.element(&vec![1, 0]), &10);
  }

  #[test]
  fn test_element_access_index_errors() {
    unimplemented!()
  }

  #[test]
  fn test_flat_initializer_list_constructor() {
    let tree = TupleTree::new_from_items(&vec![1, 2, 3]);
    assert_eq!(tree.is_leaf(&vec![]), false);
    assert_eq!(tree.is_leaf(&vec![0]), true);
    assert_eq!(tree.element(&vec![0]), &1);
    assert_eq!(tree.is_leaf(&vec![1]), true);
    assert_eq!(tree.element(&vec![1]), &2);
    assert_eq!(tree.is_leaf(&vec![2]), true);
    assert_eq!(tree.element(&vec![2]), &3);
  }

  #[test]
  fn test_nested_initializer_list_constructor() {
    let tree = TupleTree::new_from_nodes(vec![
      Node::leaf(1),
      Node::tuple_children(vec![Node::leaf(2), Node::leaf(3)]),
      Node::leaf(4)
    ]);
    assert_eq!(tree.is_leaf(&vec![]), false);
    assert_eq!(tree.is_leaf(&vec![0]), true);
    assert_eq!(tree.element(&vec![0]), &1);

    assert_eq!(tree.is_leaf(&vec![1]), false);
    assert_eq!(tree.is_leaf(&vec![1, 0]), true);
    assert_eq!(tree.element(&vec![1, 0]), &2);
    assert_eq!(tree.is_leaf(&vec![1, 1]), true);
    assert_eq!(tree.element(&vec![1, 1]), &3);

    assert_eq!(tree.is_leaf(&vec![2]), true);
    assert_eq!(tree.element(&vec![2]), &4);
  }

  #[test]
  fn test_indices_and_values_constructor() {
    let tree = TupleTree::new_from_pairs(
      vec![(vec![0], 10), (vec![1, 0], 20), (vec![1, 1], 30)]);
    assert_eq!(tree.is_leaf(&vec![]), false);
    assert_eq!(tree.is_leaf(&vec![0]), true);
    assert_eq!(tree.element(&vec![0]), &10);

    assert_eq!(tree.is_leaf(&vec![1]), false);
    assert_eq!(tree.is_leaf(&vec![1, 0]), true);
    assert_eq!(tree.element(&vec![1, 0]), &20);

    assert_eq!(tree.is_leaf(&vec![1, 1]), true);
    assert_eq!(tree.element(&vec![1, 1]), &30);
  }

  #[test]
  fn test_element_access() {
    let tree = TupleTree::new_from_pairs(
      vec![(vec![0], 1), (vec![1, 0], 2), (vec![1, 1], 3)]);

    assert_eq!(tree.element(&vec![0]), &1);
    assert_eq!(tree.element(&vec![1, 0]), &2);
    assert_eq!(tree.element(&vec![1, 1]), &3);
  }

  #[test]
  fn test_mutable_element_access() {
    let mut tree = TupleTree::new_from_pairs(
      vec![(vec![0], 1), (vec![1, 0], 2), (vec![1, 1], 3)]);

    *tree.mutable_element(&vec![0]) = 100;
    assert_eq!(tree.element(&vec![0]), &100);
    *tree.mutable_element(&vec![1, 1]) = 300;
    assert_eq!(tree.element(&vec![1, 1]), &300);
  }

  #[test]
  fn test_is_leaf() {
    let tree = TupleTree::new_from_pairs(
      vec![(vec![0], 1), (vec![1, 0], 2)]);

    assert_eq!(tree.is_leaf(&vec![0]), true);
    assert_eq!(tree.is_leaf(&vec![1]), false);
    assert_eq!(tree.is_leaf(&vec![1, 0]), true);
  }

  #[test]
  fn test_copy_subtree_from() {
    let src_tree = TupleTree::new_from_pairs(
      vec![(vec![0], 10), (vec![1], 20)]);
    
    let mut dst_tree = TupleTree::new_from_leaf_value(0);
    println!("bbb {:?}", dst_tree.nodes);
    // Graft the whole src_tree at {1} in dst_tree
    dst_tree.copy_subtree_from(
      &src_tree, &mut vec![], &vec![1]);
    
    println!("aaa {:?}", dst_tree.nodes);
    assert_eq!(dst_tree.is_leaf(&vec![0]), true);
    //assert_eq!(dst_tree.element(&vec![0]), &0);
  }

  #[test]
  fn test_copy_subtree_from_root() {
    let src_leaves = vec![(vec![0], 10), (vec![1], 20)];
    let src_tree = TupleTree::new_from_pairs(src_leaves);
    let mut dst_tree = TupleTree::new_from_leaf_value(0);

    // Copy a leaf subtree from src to the root of dst_tree
    dst_tree.copy_subtree_from(
      &src_tree, &mut vec![1], &vec![]);
    assert_eq!(dst_tree.is_leaf(&vec![]), true);
    assert_eq!(dst_tree.element(&vec![]), &20);

    // Copy a tuple subtree from src to the root of dst_tree
    dst_tree.copy_subtree_from(
      &src_tree, &mut vec![], &vec![]);
    assert_eq!(dst_tree.is_leaf(&vec![]), false);
    assert_eq!(dst_tree.element(&vec![0]), &10);
    assert_eq!(dst_tree.element(&vec![1]), &20);
  }

  #[test]
  fn test_copy_compatible_subtree_from_success() {
    let src_tree = TupleTree::new_from_node(
      &mut Node::tuple_value_children(100, vec![
        Node::leaf(1), Node::tuple_value_children(200, vec![Node::leaf(2)])]));
    
    let mut dst_tree = TupleTree::new_from_node(
      &mut Node::tuple_value_children(-1, vec![
        Node::leaf(-2), Node::tuple_value_children(-3, vec![Node::leaf(-4)])]));
    
    // Copy entire tree
    let result = dst_tree.copy_compatible_subtree_from(
      &src_tree, &vec![], &vec![]);
    assert!(result.is_ok());
    assert_eq!(dst_tree.element(&vec![]), &100);
    assert_eq!(dst_tree.element(&vec![0]), &1);
    assert_eq!(dst_tree.element(&vec![1]), &200);
    assert_eq!(dst_tree.element(&vec![1, 0]), &2);

    // Reset dst_tree
    dst_tree = TupleTree::new_from_node(
      &mut Node::tuple_value_children(-1, vec![
        Node::leaf(-2), Node::tuple_value_children(-3, vec![Node::leaf(-4)])]));

    // Copy subtree from {1} in src to {1} in dst
    let result = dst_tree.copy_compatible_subtree_from(
      &src_tree, &vec![1], &vec![1]);
    assert!(result.is_ok());
    assert_eq!(dst_tree.element(&vec![]), &-1); // Unchanged
    assert_eq!(dst_tree.element(&vec![0]), &-2); // Unchanged
    assert_eq!(dst_tree.element(&vec![1]), &200);
    assert_eq!(dst_tree.element(&vec![1, 0]), &2);

    // Copy leaf subtree
    let result = dst_tree.copy_compatible_subtree_from(
      &src_tree, &vec![0], &vec![0]);
    assert!(result.is_ok());
    assert_eq!(dst_tree.element(&vec![0]), &1);
  }

  #[test]
  fn test_copy_compatible_subtree_from_failure() {
    unimplemented!()
  }

  #[test]
  fn test_copy_compatible_subtree_from_index_errors() {
    unimplemented!()
  }

  #[test]
  fn test_subtree() {
    let tree = TupleTree::new_from_pairs(
      vec![(vec![0], 10), (vec![1, 0], 20), (vec![1, 1], 30)]);
    let subtree_or = tree.subtree(&vec![1]);
    assert!(subtree_or.is_ok());

    let subtree = subtree_or.unwrap();
    assert_eq!(subtree.is_leaf(&vec![]), false);
    assert_eq!(subtree.element(&vec![0]), &20);
    assert_eq!(subtree.element(&vec![1]), &30);

    let leaf_subtree_or = tree.subtree(&vec![0]);
    assert!(leaf_subtree_or.is_ok());
    let leaf_subtree = leaf_subtree_or.unwrap();
    assert_eq!(leaf_subtree.is_leaf(&vec![]), true);
    assert_eq!(leaf_subtree.element(&vec![]), &10);

    assert!(tree.subtree(&vec![2]).is_err());
  }

  #[test]
  fn test_for_each_element() {
    let tree: TupleTree<i64> = TupleTree::new_from_pairs(
      vec![(vec![0], 10), (vec![1, 0], 20), (vec![1, 1], 30)]);
    
    let mut visited = vec![];
    let mut func = |index: &Vec<i64>, value: &i64| {
      let mut index_clone = vec![];
      index_clone.clone_from(index);
      visited.push((index_clone, value.clone()));
    };
    tree.for_each_element(&mut func);

    assert_eq!(visited,
      vec![(vec![], 0), (vec![0], 10), (vec![1], 0),
      (vec![1, 0], 20), (vec![1, 1], 30)]);
  }

  #[test]
  fn test_for_each_mutable_element() {
    let mut tree: TupleTree<i64> = TupleTree::new_from_pairs(
      vec![(vec![0], 10), (vec![1, 0], 20), (vec![1, 1], 30)]);

    let mut offset = 0;
    tree.for_each_mutable_element(&mut |_index: &Vec<i64>, value: &mut i64| {
      *value += offset;
      offset += 1;
    });

    assert_eq!(tree.element(&vec![]), &0); // 0 + 0
    assert_eq!(tree.element(&vec![0]), &11); // 10 + 1
    assert_eq!(tree.element(&vec![1]), &2); // 0 + 2
    assert_eq!(tree.element(&vec![1, 0]), &23); // 20 + 3
    assert_eq!(tree.element(&vec![1, 1]), &34); // 30 + 4
  }

  #[test]
  fn test_for_each_mutable_element_single_leaf() {
    let mut tree = TupleTree::new_from_leaf_value(42);

    tree.for_each_mutable_element(&mut |index: &Vec<i64>, value: &mut i64| {
      assert!(index.is_empty());
      *value += 1;
    });

    assert_eq!(tree.element(&vec![]), &43);
  }

  #[test]
  fn test_for_each_element_with_status() {
    let tree: TupleTree<i64> = TupleTree::new_from_pairs(
      vec![(vec![0], 10), (vec![1, 0], 20)]);

    let result = tree.for_each_element_with_status(
      &mut |index: &Vec<i64>, _value: &i64| -> Result<(), String> {
      if *index == vec![1, 0] {
        return Err("Stop here".to_string());
      }
      Ok(())
    });

    assert!(result.is_err());
  }

  #[test]
  fn test_equality() {
    let tree1: TupleTree<i64> = TupleTree::new_from_pairs(
      vec![(vec![0], 10), (vec![1, 0], 20)]);

    let tree2: TupleTree<i64> = TupleTree::new_from_pairs(
      vec![(vec![0], 10), (vec![1, 0], 20)]);

    let tree3: TupleTree<i64> = TupleTree::new_from_pairs(
      vec![(vec![0], 10), (vec![1, 1], 20)]);

    let tree4 = TupleTree::new_from_leaf_value(10);

    assert_eq!(tree1, tree2);
    assert_ne!(tree1, tree3);
    assert_ne!(tree1, tree4);
  }

  #[test]
  fn test_iterators() {
    let leaves =
      vec![(vec![0], 10), (vec![1, 0], 20), (vec![1, 1], 30)];
    let mut tree = TupleTree::new_from_pairs(leaves);

    let mut visited = vec![];
    for pair in tree.mutable_nodes() {
      visited.push(pair.clone());
      pair.1 += 5; // Test mutability
    }
    assert_eq!(visited, vec![(vec![], 0), (vec![0], 10), (vec![1], 0),
      (vec![1, 0], 20), (vec![1, 1], 30)]);

    // Check leaves() as well
    let mut visited_leaves = vec![];
    for pair in tree.leaves() {
      visited_leaves.push(pair.clone());
    }
    assert_eq!(visited_leaves,
      vec![(vec![0], 15), (vec![1, 0], 25), (vec![1, 1], 35)]);
    
    assert_eq!(tree.element(&vec![0]), &15);
    assert_eq!(tree.element(&vec![1, 0]), &25);
    assert_eq!(tree.element(&vec![1, 1]), &35);
  }

  #[test]
  fn test_leaf_iterators() {
    let mut tree = TupleTree::new_from_nodes(vec![
      Node::leaf(1),
      Node::tuple_value_children(100, vec![Node::leaf(2), Node::leaf(3)]),
      Node::leaf(4),
      Node::tuple_value(200)
    ]);
    assert_eq!(tree.leaves(),
      vec![&(vec![0], 1), &(vec![1, 0], 2), &(vec![1, 1], 3), &(vec![2], 4)]);

      // Test mutability
    for node in tree.mutable_leaves() {
      node.1 *= 10;
    }
    assert_eq!(tree.leaves(),
      vec![&(vec![0], 10), &(vec![1, 0], 20), &(vec![1, 1], 30), &(vec![2], 40)]);

    assert_eq!(tree.element(&vec![0]), &10);
    assert_eq!(tree.element(&vec![1, 0]), &20);
    assert_eq!(tree.element(&vec![1, 1]), &30);
    assert_eq!(tree.element(&vec![2]), &40);

    // Non-leaf elements should be unchanged
    assert_eq!(tree.element(&vec![1]), &100);
    assert_eq!(tree.element(&vec![3]), &200);
  }

  #[test]
  fn test_leaf_iterators_single_leaf() {
    let tree = TupleTree::new_from_leaf_value(42);
    assert_eq!(tree.leaves(), vec![&(vec![], 42)]);
  }

  #[test]
  fn test_leaf_iterators_empty_tuple() {
    let tree: TupleTree<i64> = TupleTree::default();
    let empty_leaves: Vec<&(Vec<i64>, i64)> = vec![];
    assert_eq!(tree.leaves(), empty_leaves);
  }

  #[test]
  fn test_map() {
    let tree = TupleTree::new_from_nodes(
      vec![
        Node::leaf(1),
        Node::tuple_value_children(100, vec![Node::leaf(2), Node::leaf(3)]) ,
        Node::leaf(4),
        Node::tuple_value(200)
      ]
    );
    let mapped_tree = tree.map(&|val: &i64| -> i64 {
      *val * 2
    });
    assert_eq!(mapped_tree.nodes(),
      &vec![
        (vec![], 0),
        (vec![0], 2),
        (vec![1], 200), (vec![1, 0], 4), (vec![1, 1], 6),
        (vec![2], 8),
        (vec![3], 400)
      ]);
    // Check that the original tree is unchanged.
    assert_eq!(tree.nodes(),
      &vec![
        (vec![], 0),
        (vec![0], 1),
        (vec![1], 100), (vec![1, 0], 2), (vec![1, 1], 3),
        (vec![2], 4),
        (vec![3], 200)
      ]);
  }

  #[test]
  fn test_map_single_leaf() {
    let tree = TupleTree::new_from_leaf_value(42);
    let mapped_tree = tree.map::<i64>(
      &|val: &i64| -> i64 { val + 1 }
    );
    assert_eq!(mapped_tree.nodes(), &vec![(vec![], 43)]);
  }

  #[test]
  fn test_map_with_status_success() {
    let tree = TupleTree::new_from_nodes(
      vec![Node::leaf(1), Node::leaf(2)]);
    
    let mapped_tree = tree.map_with_status(
      &|val: &i64| -> Result<i64, String> { Ok(*val * 2) }
    );
    assert_eq!(mapped_tree.unwrap().nodes(),
      &vec![(vec![], 0), (vec![0], 2), (vec![1], 4)]);
  }

  #[test]
  fn test_map_with_status_failure() {
    let tree = TupleTree::new_from_nodes(
      vec![Node::leaf(1), Node::leaf(-1), Node::leaf(2)]);
    
    let result = tree.map_with_status(
      &|val: &i64| -> Result<i64, String> {
        if *val < 0 {
          return Err("Negative value".to_string());
        }
        Ok(*val * 2)
      }
    );
    assert_eq!(result.err().unwrap(), "Negative value".to_string());
  }

  #[test] // FAIL
  fn test_indices_and_values_constructor_with_default_value() {
    let leaves =
      vec![(vec![0], 10), (vec![1, 0], 20), (vec![1, 1], 30)];
    let tree = TupleTree::new_from_pairs_with_default_value(
      leaves, -1);
    
    assert_eq!(tree.is_leaf(&vec![]), false);
    assert_eq!(tree.element(&vec![]), &-1); // Root node should have the default value
    assert_eq!(tree.is_leaf(&vec![0]), true);
    assert_eq!(tree.element(&vec![0]), &10);

    assert_eq!(tree.is_leaf(&vec![1]), false);
    assert_eq!(tree.element(&vec![1]), &-1); // Internal node {1} is value-initialized
  }

  #[test]
  fn test_to_node() {
    let mut tuple = Node::tuple_value_children(100,
      vec![
        Node::leaf(1), // {0}
        Node::tuple_value_children(200, vec![Node::leaf(2), Node::leaf(3)]), // {1, 0}, {1, 1}
        Node::leaf(4)]); // {2}
    let tree = TupleTree::new_from_node(&mut tuple);

    // ToNode on root
    let root_node = tree.to_node(&vec![]);
    assert!(root_node.is_ok());
    assert_eq!(root_node.as_ref().unwrap().value(), &100);
    assert_eq!(root_node.as_ref().unwrap().is_leaf(), false);
    assert_eq!(root_node.as_ref().unwrap().children().len(), 3);

    // ToNode on a leaf node
    let leaf_node = tree.to_node(&vec![0]);
    assert!(leaf_node.is_ok());
    assert_eq!(leaf_node.as_ref().unwrap().value(), &1);
    assert_eq!(leaf_node.as_ref().unwrap().is_leaf(), true);

    // ToNode on a tuple node
    let tuple_node = tree.to_node(&vec![1]);
    assert!(tuple_node.is_ok());
    assert_eq!(tuple_node.as_ref().unwrap().value(), &200);
    assert_eq!(tuple_node.as_ref().unwrap().is_leaf(), false);
    assert_eq!(tuple_node.as_ref().unwrap().children().len(), 2);
    assert_eq!(tuple_node.as_ref().unwrap().children()[0].value(), &2);
    assert_eq!(tuple_node.as_ref().unwrap().children()[1].value(), &3);

    // ToNode on nested leaf node
    let nested_leaf_node = tree.to_node(&vec![1, 1]);
    assert!(nested_leaf_node.is_ok());
    assert_eq!(nested_leaf_node.as_ref().unwrap().value(), &3);
    assert_eq!(nested_leaf_node.as_ref().unwrap().is_leaf(), true);

    // ToNode on nested leaf node
    assert!(tree.to_node(&vec![5]).is_err());
    assert!(tree.to_node(&vec![0, 0]).is_err());
  }

  #[test]
  fn test_is_tuple() {
    let tuple_tree = TupleTree::new_from_items(&vec![5]);
    let non_tuple_tree = TupleTree::new_from_leaf_value(5);

    assert_eq!(tuple_tree.is_tuple(), true);
    assert_eq!(non_tuple_tree.is_tuple(), false);
  }
}