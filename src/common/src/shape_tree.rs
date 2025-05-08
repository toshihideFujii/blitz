//#![allow(dead_code)]

use std::fmt::Debug;

use crate::{shape::Shape, shape_util::ShapeUtil};

// Use indices, rather than pointers, so index table can be copied between
// ShapeTrees.
#[derive(Debug, Clone, PartialEq)]
struct Entry {
  node_id: usize,
  children_start_id: i64
}

impl Entry {
  pub fn new(node_id: usize, children_start_id: i64) -> Self {
    Entry {
      node_id: node_id,
      children_start_id: children_start_id
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
struct IndexTable {
  pub entries: Vec<Entry>
}

impl IndexTable {
  pub fn default() -> Self {
    IndexTable { entries: Vec::new() }
  }

  pub fn new(shape: &Shape) -> Self {
    let mut instance = IndexTable { entries: Vec::new() };
    let first_entry = Entry::new(0, -1);
    instance.entries.push(first_entry);
    instance.create_entry(0, shape, &mut 0);
    instance
  }

  #[allow(dead_code)]
  pub fn empty(&self) -> bool {
    self.entries.is_empty()
  }

  pub fn get(&self, index: &Vec<i64>) -> &Entry {
    let mut result = &self.entries[0];
    //println!("bbbbb {:?}", self.entries);
    for i in index {
      assert!(result.children_start_id >= 0);
      result = &self.entries[(result.children_start_id + *i) as usize];
    }
    result
  }

  fn create_entry(
    &mut self,
    entry_index: usize,
    shape: &Shape,
    next_node_id: &mut usize)
  {
    self.entries[entry_index].node_id = *next_node_id;
    *next_node_id += 1;
    if !shape.is_tuple() { return; }

    // The nodes are in depth-first pre-order. However, in order to efficiently
    // lookup indices, we generate the index table using breadth-first.
    let children_start_id = self.entries.len();
    self.entries[entry_index].children_start_id = children_start_id as i64;

    // Add entry for children first, before recursing, so they are consecutive.
    let new_len = self.entries.len() + shape.tuple_shapes_vec().len();
    let value = Entry::new(0, -1);
    self.entries.resize(new_len, value);
    for i in 0..shape.tuple_shapes_size() {
      self.create_entry(children_start_id + i, shape.tuple_shapes(i), next_node_id);
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
  index: Vec<i64>,
  value: T
}

// A ShapeTree<T> is a recursive data structure which mirrors the structure of a
// Blitz shape and holds a value of type T for each subshape (i.e. tuple or array)
// in the shape. For array shapes, a ShapeTree trivially holds a single value of
// type T.
//
// For tuple shapes which can be an arbitrary tree with arrays at the leaves, a
// ShapeTree is an identically structured tree with data elements of type T at
// every node. I.e. the root is a tuple by definition, all interior nodes are
// also tuples, and all leaves are arrays.
//
// Like the Shape data structure, this is a tree and tuple elements cannot be
// duplicated. That is, every distinct ShapeIndex in the Shape has a unique T
// object.
//
// Normally a ShapeTree owns its Shape, but for efficiency reasons, sometimes
// it's helpful not to copy a Shape just to make a ShapeTree.  In these cases,
// you can pass a Shape* instead of a Shape to the ShapeTree constructor.  It's
// then up to you to ensure that the pointed-to Shape isn't freed, moved or
// modified before its ShapeTree goes away.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeTree<T> {
  nodes: Vec<Node<T>>,
  index_table: IndexTable,
  shape_storage: Option<Shape>,
  shape: Shape
}

impl<T> ShapeTree<T> where T: Debug + Default + Clone + PartialEq {
  // Default constructor creates a tree with a nil shape (i.e. an empty tuple).
  pub fn default() -> Self {
    ShapeTree {
      nodes: Vec::new(),
      index_table: IndexTable::default(),
      shape_storage: None,
      shape: ShapeUtil::make_nil()
    }
  }

  // Create ShapeTree with the given shape, and default-constructed T values for
  // all nodes.
  //
  // The version that takes a pointer may be cheaper because it doesn't require
  // any Shape copies, but then it's up to you to ensure that the pointer stays
  // alive longer than this ShapeTree.
  pub fn new(shape: &mut Shape) -> Self {
    let mut instance = ShapeTree {
      nodes: Vec::new(),
      index_table: IndexTable::new(&shape),
      shape_storage: None,
      shape: shape.clone()
    };
    instance.nodes = ShapeTree::create_nodes_default(shape);
    instance
  }

  // Create ShapeTree with the given shape, and init_value for all nodes.
  pub fn new_with_value(shape: &mut Shape, init_value: T) -> Self {
    let mut instance = ShapeTree {
      nodes: Vec::new(),
      index_table: IndexTable::new(&shape),
      shape_storage: None,
      shape: shape.clone()
    };
    instance.nodes = ShapeTree::create_nodes(shape, init_value);
    instance
  }

  pub fn new_from_nodes(shape: &Shape, nodes: Vec<Node<T>>) -> Self {
    let instance = ShapeTree {
      nodes: nodes,
      index_table: IndexTable::new(shape),
      shape_storage: None,
      shape: shape.clone()
    };
    assert_eq!(instance.nodes.len(), ShapeUtil::subshape_count(shape));
    instance
  }

  // Returns the data element associated with the array in the shape at the
  // given index.
  pub fn element(&self, index: &Vec<i64>) -> &T {
    &self.find(index).value
  }

  pub fn mutable_element(&mut self, index: &Vec<i64>) -> &mut T {
    &mut self.find_mut(index).value
  }

  pub fn set_element_value(&mut self, index: &Vec<i64>, value: T) {
    let element = self.find_mut(index);
    element.value = value;
  }

  // Return the shape represented with this ShapeTree.
  pub fn shape(&self) -> &Shape {
    &self.shape
  }

  // A ShapeTree object can own the underlying Shape pointer (via the
  // shape_storage_ member), or can point to a Shape object owned by the caller.
  // This API replaces the underlying Shape object to the one supplied by the
  // caller, whom must ensure the object remain valid for the whole lifetime of
  // this ShapeTree object, and also that the Shape is consistent with it.
  pub fn replace_shape_ptr(&mut self, shape: Shape) {
    if self.shape_storage.is_some() {
      assert_eq!(shape, *self.shape_storage.as_ref().unwrap());
      self.shape_storage = None;
    }
    self.shape = shape;
  }

  // Returns true if the node at the given index is a leaf node (an array shape).
  pub fn is_leaf(&self, index: &Vec<i64>) -> bool {
    self.index_table.get(index).children_start_id == -1
  }

  pub fn leaves(&self) -> Vec<&Node<T>> {
    let mut leaves = vec![];
    for node in self.nodes() {
      if self.is_leaf(&node.index) { leaves.push(node); }
    }
    leaves
  }

  // Returns an iterator pointing to the given ShapeIndex.
  // REQUIRES: index must exist in the ShapeTree.
  pub fn find(&self, index: &Vec<i64>) -> &Node<T> {
    let node_id = self.index_table.get(index).node_id;
    &self.nodes[node_id]
  }

  pub fn find_mut(&mut self, index: &Vec<i64>) -> &mut Node<T> {
    let node_id = self.index_table.get(index).node_id;
    &mut self.nodes[node_id]
  }

  pub fn find_at(&self, index: &Vec<i64>) -> usize {
    let node_id = self.index_table.get(index).node_id;
    let mut result = 0;
    for node in &self.nodes {
      if node == &self.nodes[node_id] { return result; }
      result += 1;
    }
    unreachable!();
  }

  // Returns the number of leaf nodes in the tree.
  pub fn leaf_count(&self) -> usize {
    let mut leaf_count = 0;
    for entry in &self.index_table.entries {
      if entry.children_start_id == -1 { leaf_count += 1; }
    }
    leaf_count
  }

  // Recursively traverses the shape and calls the given function at each element.
  pub fn for_each_element<F>(&self, func: &F)
    where F: Fn(&Vec<i64>, &T)
  {
    for node in &self.nodes {
      func(&node.index, &node.value);
    }
  }

  pub fn for_each_mutable_element<F>(&mut self, func: &mut F)
    where F: FnMut(&Vec<i64>, &mut T)
  {
    for node in &mut self.nodes {
      func(&node.index, &mut node.value);
    }
  }

  // Like ForEach(Mutable)Element, but the callable returns a absl::Status
  // instead of void.  The first non-OK return value is returned by the ForEach*
  // function.
  pub fn for_each_element_with_status<F>(&self, func: &F) -> Result<(), String>
    where F: Fn(&Vec<i64>, &T) -> Result<(), String>
  {
    for node in &self.nodes {
      let result = func(&node.index, &node.value);
      if result.is_err() { return result; }
    }
    Ok(())
  }

  pub fn for_each_mutable_element_with_status<F>(&mut self, func: &mut F) -> Result<(), String>
    where F: FnMut(&Vec<i64>, &T) -> Result<(), String>
  {
    for node in &self.nodes {
      let result = func(&node.index, &node.value);
      if result.is_err() { return result; }
    }
    Ok(())
  }

  // Like the above, but traverses in post-order.  Note children are visited in
  // right-to-left order.
  pub fn for_each_element_post_order<F>(&self, func: &F)
    where F: Fn(&Vec<i64>, &T)
  {
    for i in (0..self.nodes.len()).rev() {
      func(&self.nodes[i].index, &self.nodes[i].value);
    }
  }

  pub fn for_each_mutable_element_post_order<F>(&self, func: &mut F)
    where F: FnMut(&Vec<i64>, &T)
  {
    for i in (0..self.nodes.len()).rev() {
      func(&self.nodes[i].index, &self.nodes[i].value);
    }
  }

  pub fn for_each_element_post_order_with_status<F>(&self, func: &F) -> Result<(), String>
    where F: Fn(&Vec<i64>, &T) -> Result<(), String>
  {
    for i in (0..self.nodes.len()).rev() {
      let result = func(&self.nodes[i].index, &self.nodes[i].value);
      if result.is_err() { return result; }
    }
    Ok(())
  }

  pub fn for_each_mutable_element_post_order_with_status<F>(&self, func: &mut F) -> Result<(), String>
    where F: FnMut(&Vec<i64>, &T) -> Result<(), String>
  {
    for i in (0..self.nodes.len()).rev() {
      let result = func(&self.nodes[i].index, &self.nodes[i].value);
      if result.is_err() { return result; }
    }
    Ok(())
  }

  // Maps each element to generate a new tree with the same shape.
  pub fn map<U, F>(&self, func: F) -> ShapeTree<U>
    where F: Fn(&T) -> U, U: Default + Clone
  {
    let mut result_nodes = vec![];
    for node in &self.nodes {
      let mut index = vec![];
      index.clone_from(&node.index);
      let new_node = Node {
        index: index,
        value: func(&node.value)
      };
      result_nodes.push(new_node);
    }

    let result = ShapeTree {
      nodes: result_nodes,
      index_table: self.index_table.clone(),
      shape_storage: self.shape_storage.clone(),
      shape: self.shape.clone()
    };
    result
  }

  // Copy the subtree of values from 'other' rooted at ShapeIndex 'src_index'
  // into the subtree of value in this ShapeTree rooted at 'dst_index'.
  //
  // Precondition: The subshape of other.shape() at index src_index must be
  // compatible with the subshape of shape() at index dst_index.
  pub fn copy_subtree_from(
    &mut self,
    other: &ShapeTree<T>,
    src_index: &Vec<i64>,
    dst_index: &Vec<i64>)
  {
    let src_shape =
      ShapeUtil::get_subshape(other.shape(), src_index);
    let dst_shape =
      ShapeUtil::get_subshape(self.shape(), dst_index);
    assert!(ShapeUtil::compatible(&src_shape, &dst_shape));

    // Replace the prefix `src_index` with `dst_index`.
    let replace_shape_index_prefix
      = |index: &Vec<i64>| -> Vec<i64>
    {
      let (without_prefix, _prefix) =
        index.split_at(src_index.len());
      let mut result = vec![];
      for i in dst_index {
        result.push(*i);
      }
      for i in without_prefix {
        result.push(*i);
      }
      result
    };

    let first = other.find_at(src_index);
    let last = first + ShapeUtil::subshape_count(&src_shape);
    let func = |node: &Node<T>| -> Node<T> {
      Node {
        index: replace_shape_index_prefix(&node.index),
        value: node.value.clone()
      }
    };

    let mut target_index = self.find_at(dst_index);
    for i in first..last {
      self.nodes[target_index] = func(&other.nodes[i]);
      target_index += 1;
    }
  }

  pub fn sub_shape_tree(&self, index: &Vec<i64>) -> Result<ShapeTree<T>, String> {
    let sub_shape =
      ShapeUtil::try_get_subshape(self.shape(), index);
    if sub_shape.is_err() { return Err(sub_shape.err().unwrap()); }

    let count = ShapeUtil::subshape_count(sub_shape.as_ref().unwrap());
    let first_node = self.find(index);
    let mut sub_tree_nodes = vec![];
    let mut first_node_found = false;
    let mut num = 0;
    for node in &self.nodes {
      if *node != *first_node && first_node_found == false {
        continue;
      }
      if *node == *first_node && first_node_found == false {
        first_node_found = true;
      }
      if num == count {
        break;
      }
      // For each shape index, remove the prefix `index`.
      let (_prefix, without_prefix) = node.index.split_at(index.len());
      let new_node = Node {
        index: without_prefix.to_vec(),
        value: node.value.clone()
      };
      sub_tree_nodes.push(new_node);
      num += 1;
    }

    Ok(ShapeTree::new_from_nodes(sub_shape.as_ref().unwrap(), sub_tree_nodes))
  }

  pub fn nodes(&self) -> &Vec<Node<T>> {
    &self.nodes
  }

  pub fn mutable_nodes(&mut self) -> &mut Vec<Node<T>> {
    &mut self.nodes
  }

  fn create_nodes_default(shape: &mut Shape) -> Vec<Node<T>> {
    let mut nodes = Vec::new();
    let mut func = |_subshape: &mut Shape, index: &Vec<i64>| {
      let mut index_vec = vec![];
      index_vec.clone_from(index);
      let node = Node { index: index_vec, value: T::default() };
      nodes.push(node);
    };
    ShapeUtil::for_each_mutable_subshape(shape, &mut func);
    nodes
  }

  fn create_nodes(shape: &mut Shape, value: T) -> Vec<Node<T>> {
    let mut nodes = Vec::new();
    let mut func = |_subshape: &mut Shape, index: &Vec<i64>| {
      let mut index_vec = vec![];
      index_vec.clone_from(index);
      let node = Node { index: index_vec, value: value.clone() };
      nodes.push(node);
    };
    ShapeUtil::for_each_mutable_subshape(shape, &mut func);
    nodes
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::blitz_data::PrimitiveType;

  fn create_array_shape() -> Shape {
    ShapeUtil::make_shape(&PrimitiveType::F32, vec![42, 42, 123])
  }

  fn create_tuple_shape() -> Shape {
    ShapeUtil::make_tuple_shape(
      vec![create_array_shape(), create_array_shape(), create_array_shape()])
  }

  fn create_nested_tuple_shape() -> Shape {
    let t1 = ShapeUtil::make_tuple_shape(
      vec![create_array_shape(), create_array_shape()]);
    let t2_1 = ShapeUtil::make_tuple_shape(
      vec![create_array_shape(), create_array_shape()]);
    let t2 = ShapeUtil::make_tuple_shape(vec![t2_1, create_array_shape()]);

    ShapeUtil::make_tuple_shape(
      vec![create_array_shape(), t1, t2])
  }

  fn shape_constructor_testing(shape: &mut Shape, expected_num_nodes: i64) {
    let mut int_tree: ShapeTree<i64> = ShapeTree::new(shape);
    let mut num_nodes = 0;
    let mut func = |_index: &Vec<i64>, data: &mut i64| {
      assert_eq!(*data, 0);
      num_nodes += 1;
    };
    int_tree.for_each_mutable_element(&mut func);
    assert_eq!(expected_num_nodes, num_nodes);

    let mut bool_tree: ShapeTree<bool> = ShapeTree::new(shape);
    let mut num_nodes = 0;
    let mut func = |_index: &Vec<i64>, data: &mut bool| {
      assert_eq!(*data, false);
      num_nodes += 1;
    };
    bool_tree.for_each_mutable_element(&mut func);
    assert_eq!(expected_num_nodes, num_nodes);
  }

  fn init_value_constructor_testing(shape: &mut Shape, expected_num_nodes: i64) {
    let mut tree: ShapeTree<i64> = ShapeTree::new_with_value(shape, 42);
    let mut num_nodes = 0;
    let mut func = |_index: &Vec<i64>, data: &mut i64| {
      assert_eq!(*data, 42);
      num_nodes += 1;
    };
    tree.for_each_mutable_element(&mut func);
    assert_eq!(expected_num_nodes, num_nodes);

    num_nodes = 0;
    let mut func2 = |_index: &Vec<i64>, data: &mut i64| {
      assert_eq!(*data, 42);
      *data = num_nodes;
      num_nodes += 1;
    };
    tree.for_each_mutable_element(&mut func2);

    num_nodes = 0;
    let mut func3 = |_index: &Vec<i64>, data: &mut i64| {
      assert_eq!(num_nodes, *data);
      num_nodes += 1;
    };
    tree.for_each_mutable_element(&mut func3);
    assert_eq!(expected_num_nodes, num_nodes);
  }

  #[test]
  fn test_default_constructor() {
    let int_tree: ShapeTree<i64> = ShapeTree::default();
    assert_eq!(ShapeUtil::is_empty_tuple(int_tree.shape()), true);

    let bool_tree: ShapeTree<bool> = ShapeTree::default();
    assert_eq!(ShapeUtil::is_empty_tuple(bool_tree.shape()), true);
  }

  #[test]
  fn test_shape_constructor() {
    shape_constructor_testing(&mut create_array_shape(), 1);
    shape_constructor_testing(&mut create_tuple_shape(), 4);
    shape_constructor_testing(&mut create_nested_tuple_shape(), 10);
  }

  #[test]
  fn test_init_value_constructor() {
    init_value_constructor_testing(&mut create_array_shape(), 1);
    init_value_constructor_testing(&mut create_tuple_shape(), 4);
    init_value_constructor_testing(&mut create_nested_tuple_shape(), 10);
  }

  #[test]
  fn test_empty_tuple_must_have_no_leaves() {
    let shape_tree: ShapeTree<i64> =
      ShapeTree::new(&mut ShapeUtil::make_tuple_shape(vec![]));
    assert_eq!(shape_tree.leaf_count(), 0);
  }

  #[test]
  fn test_nested_empty_tuple() {
    let mut shape = ShapeUtil::make_tuple_shape(
      vec![ShapeUtil::make_tuple_shape(vec![]), create_array_shape()]);

    let shape_tree: ShapeTree<i64> = ShapeTree::new(&mut shape);
    assert_eq!(shape_tree.leaf_count(), ShapeUtil::get_leaf_count(&shape));
  }

  #[test]
  fn test_array_shape() {
    let mut shape_tree: ShapeTree<i64> =
      ShapeTree::new(&mut create_array_shape());
    *shape_tree.mutable_element(&vec![]) = 42;
    assert_eq!(shape_tree.element(&vec![]), &42);
    *shape_tree.mutable_element(&vec![]) = 123;
    assert_eq!(shape_tree.element(&vec![]), &123);
    assert_eq!(ShapeUtil::compatible(&create_array_shape(), shape_tree.shape()), true);

    // Test the copy constructor.
    let mut copy = shape_tree.clone();
    assert_eq!(copy.element(&vec![]), &123);

    // Mutate the copy, and ensure the original doesn't change.
    *copy.mutable_element(&vec![]) = 99;
    assert_eq!(copy.element(&vec![]), &99);
    assert_eq!(shape_tree.element(&vec![]), &123);

    // Test the assignment operator.
    copy = shape_tree;
    assert_eq!(copy.element(&vec![]), &123);
  }

  #[test]
  fn test_tuple_shape() {
    let mut shape_tree: ShapeTree<i64> =
      ShapeTree::new(&mut create_tuple_shape());
    *shape_tree.mutable_element(&vec![]) = 1;
    *shape_tree.mutable_element(&vec![0]) = 42;
    *shape_tree.mutable_element(&vec![1]) = 123;
    *shape_tree.mutable_element(&vec![2]) = -100;
    assert_eq!(shape_tree.element(&vec![]), &1);
    assert_eq!(shape_tree.element(&vec![0]), &42);
    assert_eq!(shape_tree.element(&vec![1]), &123);
    assert_eq!(shape_tree.element(&vec![2]), &-100);
    assert_eq!(ShapeUtil::compatible(&create_tuple_shape(), shape_tree.shape()), true);

    // Sum all elements in the shape.
    let mut sum = 0;
    let mut func = |_index: &Vec<i64>, data: &mut i64| {
      sum += *data;
    };
    shape_tree.for_each_mutable_element(&mut func);
    assert_eq!(sum, 66);

    // Test the copy constructor.
    let mut copy = shape_tree.clone();
    assert_eq!(copy.element(&vec![]), &1);
    assert_eq!(copy.element(&vec![0]), &42);
    assert_eq!(copy.element(&vec![1]), &123);
    assert_eq!(copy.element(&vec![2]), &-100);

    // Write zero to all elements.
    let mut func2 = |_index: &Vec<i64>, data: &mut i64| {
      *data = 0;
    };
    shape_tree.for_each_mutable_element(&mut func2);
    assert_eq!(shape_tree.element(&vec![]), &0);
    assert_eq!(shape_tree.element(&vec![0]), &0);
    assert_eq!(shape_tree.element(&vec![1]), &0);
    assert_eq!(shape_tree.element(&vec![2]), &0);
    assert_eq!(copy.element(&vec![]), &1);
    assert_eq!(copy.element(&vec![0]), &42);
    assert_eq!(copy.element(&vec![1]), &123);
    assert_eq!(copy.element(&vec![2]), &-100);

    // Test the assignment operator.
    copy = shape_tree;
    assert_eq!(copy.element(&vec![]), &0);
    assert_eq!(copy.element(&vec![0]), &0);
    assert_eq!(copy.element(&vec![1]), &0);
    assert_eq!(copy.element(&vec![2]), &0);
  }

  #[test]
  fn test_nested_tuple_shape() {
    let mut nested_tuple_shape = create_nested_tuple_shape();
    let mut shape_tree: ShapeTree<i64> = ShapeTree::new(&mut nested_tuple_shape);
    *shape_tree.mutable_element(&vec![0]) = 42;
    *shape_tree.mutable_element(&vec![1, 1]) = 123;
    *shape_tree.mutable_element(&vec![2, 0, 1]) = -100;
    assert_eq!(shape_tree.element(&vec![0]), &42);
    assert_eq!(shape_tree.element(&vec![1,1]), &123);
    assert_eq!(shape_tree.element(&vec![2, 0, 1]), &-100);
    assert_eq!(ShapeUtil::compatible(&nested_tuple_shape, shape_tree.shape()), true);

    // Test the copy constructor.
    let mut copy = shape_tree.clone();
    assert_eq!(copy.element(&vec![0]), &42);
    assert_eq!(copy.element(&vec![1, 1]), &123);
    assert_eq!(copy.element(&vec![2, 0, 1]), &-100);

    // Mutate the copy, and ensure the original doesn't change.
    *copy.mutable_element(&vec![0]) = 1;
    *copy.mutable_element(&vec![1, 1]) = 2;
    *copy.mutable_element(&vec![2, 0, 1]) = 3;
    assert_eq!(copy.element(&vec![0]), &1);
    assert_eq!(copy.element(&vec![1, 1]), &2);
    assert_eq!(copy.element(&vec![2, 0, 1]), &3);
    assert_eq!(shape_tree.element(&vec![0]), &42);
    assert_eq!(shape_tree.element(&vec![1,1]), &123);
    assert_eq!(shape_tree.element(&vec![2, 0, 1]), &-100);

    // Test the assignment operator.
    copy = shape_tree;
    assert_eq!(copy.element(&vec![0]), &42);
    assert_eq!(copy.element(&vec![1, 1]), &123);
    assert_eq!(copy.element(&vec![2, 0, 1]), &-100);
  }

  #[test]
  fn test_copy_subtree_from_array_shape() {
    let mut source: ShapeTree<i64> = ShapeTree::new(&mut create_array_shape());
    *source.mutable_element(&vec![]) = 42;

    let mut destination: ShapeTree<i64> =
      ShapeTree::new_with_value(&mut create_array_shape(), 123);

    assert_eq!(destination.element(&vec![]), &123);
    destination.copy_subtree_from(&source, &vec![], &vec![]);
    assert_eq!(destination.element(&vec![]), &42);
  }

  #[test]
  fn test_full_copy_subtree_from_tuple_shape() {
    let mut source = ShapeTree::new(&mut create_tuple_shape());
    *source.mutable_element(&vec![]) = 10;
    *source.mutable_element(&vec![0]) = 11;
    *source.mutable_element(&vec![1]) = 12;
    *source.mutable_element(&vec![2]) = 13;

    let mut destination =
      ShapeTree::new_with_value(&mut create_tuple_shape(), 0);
    destination.copy_subtree_from(&source, &vec![], &vec![]);

    assert_eq!(destination.element(&vec![]), &10);
    assert_eq!(destination.element(&vec![0]), &11);
    assert_eq!(destination.element(&vec![1]), &12);
    assert_eq!(destination.element(&vec![2]), &13);
  }

  #[test]
  fn test_single_element_copy_subtree_from_tuple_shape() {
    let mut source = ShapeTree::new(&mut create_tuple_shape());
    *source.mutable_element(&vec![]) = 10;
    *source.mutable_element(&vec![0]) = 11;
    *source.mutable_element(&vec![1]) = 12;
    *source.mutable_element(&vec![2]) = 13;

    let mut destination =
      ShapeTree::new_with_value(&mut create_tuple_shape(), 0);
    destination.copy_subtree_from(&source, &vec![0], &vec![1]);

    assert_eq!(destination.element(&vec![]), &0);
    assert_eq!(destination.element(&vec![0]), &0);
    assert_eq!(destination.element(&vec![1]), &11);
    assert_eq!(destination.element(&vec![2]), &0);
  }

  #[test]
  fn test_copy_subtree_into_nested_shape() {
    let mut shape = ShapeUtil::make_tuple_shape(
      vec![create_array_shape(), create_array_shape()]);
    let mut source = ShapeTree::new(&mut shape);

    *source.mutable_element(&vec![]) = 10;
    *source.mutable_element(&vec![0]) = 11;
    *source.mutable_element(&vec![1]) = 12;

    let mut destination = ShapeTree::new_with_value(
      &mut create_nested_tuple_shape(), 0);
    destination.copy_subtree_from(&source, &vec![], &vec![2, 0]);

    assert_eq!(destination.element(&vec![]), &0);
    assert_eq!(destination.element(&vec![0]), &0);
    assert_eq!(destination.element(&vec![1]), &0);
    assert_eq!(destination.element(&vec![1, 0]), &0);
    assert_eq!(destination.element(&vec![1, 1]), &0);
    assert_eq!(destination.element(&vec![2]), &0);
    assert_eq!(destination.element(&vec![2, 0]), &10);
    assert_eq!(destination.element(&vec![2, 0, 0]), &11);
    assert_eq!(destination.element(&vec![2, 0, 1]), &12);
    assert_eq!(destination.element(&vec![2, 1]), &0);
  }

  #[test]
  fn copy_subtree_from_nested_shape() {
    let mut source = ShapeTree::new_with_value(
      &mut create_nested_tuple_shape(), 42);
    *source.mutable_element(&vec![1]) = 10;
    *source.mutable_element(&vec![1, 0]) = 11;
    *source.mutable_element(&vec![1, 1]) = 12;

    let mut tuple = ShapeUtil::make_tuple_shape(
      vec![create_array_shape(), create_array_shape()]);
    let mut destination = ShapeTree::new_with_value(
      &mut tuple, 0);
    destination.copy_subtree_from(&source, &vec![1], &vec![]);

    assert_eq!(destination.element(&vec![]), &10);
    assert_eq!(destination.element(&vec![0]), &11);
    assert_eq!(destination.element(&vec![1]), &12);
  }

  #[test]
  fn test_iterate_simple() {
    let shape_tree = ShapeTree::new_with_value(
      &mut create_nested_tuple_shape(), 42);

    let mut num_nodes = 0;
    for node in shape_tree.nodes() {
      assert_eq!(node.value, 42);
      num_nodes += 1;
    }
    assert_eq!(num_nodes, 10);
  }

  #[test]
  fn test_iterate_and_mutate() {
    let mut shape_tree = ShapeTree::new_with_value(
      &mut create_nested_tuple_shape(), 42);
    let mut i = 0;
    for node in shape_tree.mutable_nodes() {
      assert_eq!(node.value, 42);
      if i == 1 { node.value = 98; }
      i += 1;
    }

    shape_tree.nodes[0].value = 78;
    assert_eq!(shape_tree.nodes[0].value, 78);

    i = 0;
    for node in shape_tree.nodes() {
      if i == 0 {
        assert_eq!(node.value, 78);
      } else if i == 1 {
        assert_eq!(node.value, 98);
      } else {
        assert_eq!(node.value, 42);
      }
      i += 1;
    }
  }

  #[test]
  fn test_iterate_order() {
    let shape_tree = ShapeTree::new_with_value(
      &mut create_nested_tuple_shape(), 42);
    let mut v = vec![];
    for node in shape_tree.nodes() {
      let mut index: Vec<i64> = vec![];
      index.clone_from(&node.index);
      v.push(index);
    }

    assert_eq!(
      vec![
        vec![], vec![0], vec![1],
        vec![1, 0], vec![1, 1],
        vec![2],
        vec![2, 0],
        vec![2, 0, 0], vec![2, 0, 1],
        vec![2, 1]
      ],
      v);
  }

  #[test]
  fn test_find() {
    let shape_tree =
      ShapeTree::new_with_value(&mut create_nested_tuple_shape(), 42);
    let found = shape_tree.find(&vec![1, 0]);

    assert_eq!(found.index, vec![1, 0]);
    assert_eq!(found.value, 42);
  }

  #[test]
  fn test_iterate_order_leaves() {
    let shape_tree = ShapeTree::new_with_value(
      &mut create_nested_tuple_shape(), 42);
    let mut v = vec![];
    for node in shape_tree.leaves() {
      v.push(&node.index);
    }
    assert_eq!(vec![
      &vec![0],
      &vec![1, 0], &vec![1, 1],
      &vec![2, 0, 0], &vec![2, 0, 1],
      &vec![2, 1]
    ], v);
  }
}