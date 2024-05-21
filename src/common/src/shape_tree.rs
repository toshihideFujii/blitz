#![allow(dead_code)]

use crate::{shape::Shape, shape_util::ShapeUtil};

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
    instance.create_entry(shape, 0);
    instance
  }

  pub fn empty(&self) -> bool {
    self.entries.is_empty()
  }

  pub fn get(&self, index: usize) -> &Entry {
    self.entries.get(index).unwrap()
  }

  fn create_entry(&mut self, shape: &Shape, mut next_node_id: usize) {
    if !shape.is_tuple() {
      let entry = Entry::new(next_node_id, -1);
      self.entries.push(entry);
      return;
    }

    let entry = Entry::new(next_node_id, self.entries.len() as i64);
    self.entries.push(entry);

    next_node_id += 1;
    for i in 0..shape.tuple_shapes_size() {
      self.create_entry(shape.tuple_shapes(i), next_node_id);
    }
  }
}

// A ShapeTree<T> is a recursive data structure which mirrors the sstructure of a
// Blitz shape and holds a value of type T for each subshape (i.e. tuple or array)
// in the shape.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeTree<T> {
  nodes: Vec<(usize, T)>,
  index_table: IndexTable,
  //shape_storage: Shape,
  shape: Shape
}

impl<T> ShapeTree<T> where T: Default + Clone {
  pub fn default() -> Self {
    ShapeTree {
      nodes: Vec::new(),
      index_table: IndexTable::default(),
      shape: ShapeUtil::make_nil()
    }
  }

  pub fn new(shape: Shape) -> Self {
    let mut instance = ShapeTree {
      nodes: Vec::new(),
      index_table: IndexTable::new(&shape),
      shape: shape.clone()
    };
    instance.nodes = ShapeTree::create_nodes_default(&shape);
    instance
  }

  pub fn new_with_value(shape: Shape, init_value: T) -> Self {
    let mut instance = ShapeTree {
      nodes: Vec::new(),
      index_table: IndexTable::new(&shape),
      shape: shape.clone()
    };
    instance.nodes = ShapeTree::create_nodes(&shape, init_value);
    instance
  }

  // Returns the data element associated with the array in the shape at the
  // given index.
  pub fn element(&self, index: usize) -> &T {
    &self.find(index).1
  }

  pub fn mutable_element(&mut self, index: usize) -> &mut T {
    &mut self.find_mut(index).1
  }

  pub fn set_element_value(&mut self, index: usize, value: T) {
    let element = self.find_mut(index);
    element.1 = value;
  }

  // Return the shape represented with this ShapeTree.
  pub fn shape(&self) -> &Shape {
    &self.shape
  }

  pub fn replace_shape_ptr(&mut self, shape: Shape) {
    self.shape = shape;
  }

  // Returns true if the node at the given index is a leaf node (an array shape).
  pub fn is_leaf(&self, index: usize) -> bool {
    self.index_table.get(index).children_start_id == -1
  }

  // Returns an iterator pointing to the given ShapeIndex.
  pub fn find(&self, index: usize) -> &(usize, T) {
    self.nodes.get(index).unwrap()
  }

  pub fn find_mut(&mut self, index: usize) -> &mut (usize, T) {
    self.nodes.get_mut(index).unwrap()
  }

  // Returns the number of leaf nodes in the tree.
  pub fn leaf_count(&self) -> usize {
    let mut leaf_count = 0;
    for i in 0..self.nodes.len() {
      if self.is_leaf(i) {
        leaf_count += 1;
      }
    }
    leaf_count
  }

  // Recursively traverses the shape and calls the given function at each element.
  pub fn for_each_element<F>(&self, func: &F)
    where F: Fn(usize, &T)
  {
    for node in &self.nodes {
      func(node.0, &node.1);
    }
  }

  pub fn for_each_mutable_element<F>(&mut self, func: &mut F)
    where F: FnMut(usize, &mut T)
  {
    for node in &mut self.nodes {
      func(node.0, &mut node.1);
    }
  }

  pub fn for_each_element_with_status<F>(&self, func: &F) -> Result<(), String>
    where F: Fn(usize, &T) -> Result<(), String>
  {
    for node in &self.nodes {
      let result = func(node.0, &node.1);
      if result.is_err() { return result; }
    }
    Ok(())
  }

  pub fn for_each_mutable_element_with_status<F>(&mut self, func: &mut F) -> Result<(), String>
    where F: FnMut(usize, &T) -> Result<(), String>
  {
    for node in &self.nodes {
      let result = func(node.0, &node.1);
      if result.is_err() { return result; }
    }
    Ok(())
  }

  pub fn for_each_element_post_order() {}
  pub fn for_each_mutable_element_post_order() {}
  pub fn for_each_element_post_order_with_status() {}
  pub fn for_each_mutable_element_post_order_with_status() {}
  pub fn map() {}
  pub fn copy_subtree_from() {}
  pub fn sub_shape_tree() {}

  pub fn nodes(&self) -> &Vec<(usize, T)> {
    &self.nodes
  }

  pub fn mutable_nodes(&mut self) -> &mut Vec<(usize, T)> {
    &mut self.nodes
  }

  fn create_nodes_default(shape: &Shape) -> Vec<(usize, T)> {
    let mut nodes = Vec::new();
    let mut func = |_subshape: &Shape, index: usize| {
      //if !subshape.is_tuple() { nodes.push((index, T::default())); }
      nodes.push((index, T::default()));
    };
    ShapeUtil::for_each_mutable_subshape(shape, &mut func);
    nodes
  }

  fn create_nodes(shape: &Shape, value: T) -> Vec<(usize, T)> {
    let mut nodes = Vec::new();
    let mut func = |_subshape: &Shape, index: usize| {
      //if !subshape.is_tuple() { nodes.push((index, value.clone())); }
      nodes.push((index, value.clone()));
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

  fn shape_constructor_testing(shape: &Shape, expected_num_nodes: i64) {
    let mut int_tree: ShapeTree<i64> = ShapeTree::new(shape.clone());
    let mut num_nodes = 0;
    let mut func = |_index: usize, data: &mut i64| {
      assert_eq!(*data, 0);
      num_nodes += 1;
    };
    int_tree.for_each_mutable_element(&mut func);
    assert_eq!(expected_num_nodes, num_nodes);

    let mut bool_tree: ShapeTree<bool> = ShapeTree::new(shape.clone());
    let mut num_nodes = 0;
    let mut func = |_index: usize, data: &mut bool| {
      assert_eq!(*data, false);
      num_nodes += 1;
    };
    bool_tree.for_each_mutable_element(&mut func);
    assert_eq!(expected_num_nodes, num_nodes);
  }

  fn init_value_constructor_testing(shape: &Shape, expected_num_nodes: i64) {
    let mut tree: ShapeTree<i64> = ShapeTree::new_with_value(shape.clone(), 42);
    let mut num_nodes = 0;
    let mut func = |_index: usize, data: &mut i64| {
      assert_eq!(*data, 42);
      num_nodes += 1;
    };
    tree.for_each_mutable_element(&mut func);
    assert_eq!(expected_num_nodes, num_nodes);
  /*
    num_nodes = 0;
    let mut func2 = |_index: usize, data: &mut i64| {
      assert_eq!(*data, 42);
      *data = num_nodes;
      num_nodes += 1;
    };
    tree.for_each_mutable_element(&mut func2);

    num_nodes = 0;
    let mut func3 = |_index: usize, data: &i64| {
      assert_eq!(num_nodes, *data);
      num_nodes += 1;
    };
    tree.for_each_mutable_element(&mut func3);
    assert_eq!(expected_num_nodes, num_nodes);
  */
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
    shape_constructor_testing(&create_array_shape(), 1);
    shape_constructor_testing(&create_tuple_shape(), 4);
    shape_constructor_testing(&create_nested_tuple_shape(), 10);
  }

  #[test]
  fn test_init_value_constructor() {
    init_value_constructor_testing(&create_array_shape(), 1);
    init_value_constructor_testing(&create_tuple_shape(), 4);
    init_value_constructor_testing(&create_nested_tuple_shape(), 10);
  }

  #[test]
  fn test_empty_tuple_must_have_no_leaves() {
    let shape_tree: ShapeTree<i64> =
      ShapeTree::new(ShapeUtil::make_tuple_shape(vec![]));
    assert_eq!(shape_tree.leaf_count(), 0);
  }

  #[test]
  fn test_nested_empty_tuple() {
    let shape = ShapeUtil::make_tuple_shape(
      vec![ShapeUtil::make_tuple_shape(vec![]), create_array_shape()]);

    let shape_tree: ShapeTree<i64> = ShapeTree::new(shape.clone());
    assert_eq!(shape_tree.leaf_count(), ShapeUtil::get_leaf_count(&shape));
  }

  #[test]
  fn test_array_shape() {
    let mut shape_tree: ShapeTree<i64> = ShapeTree::new(create_array_shape());
    *shape_tree.mutable_element(0) = 42;
    assert_eq!(shape_tree.element(0), &42);
    *shape_tree.mutable_element(0) = 123;
    assert_eq!(shape_tree.element(0), &123);
    assert_eq!(ShapeUtil::compatible(&create_array_shape(), shape_tree.shape()), true);

    // Test the copy constructor.
    let mut copy = shape_tree.clone();
    assert_eq!(copy.element(0), &123);

    // Mutate the copy, and ensure the original doesn't change.
    *copy.mutable_element(0) = 99;
    assert_eq!(copy.element(0), &99);
    assert_eq!(shape_tree.element(0), &123);

    // Test the assignment operator.
    copy = shape_tree;
    assert_eq!(copy.element(0), &123);
  }

  #[test]
  fn test_tuple_shape() {
    let mut shape_tree: ShapeTree<i64> = ShapeTree::new(create_tuple_shape());
    *shape_tree.mutable_element(0) = 1;
    *shape_tree.mutable_element(1) = 42;
    *shape_tree.mutable_element(2) = 123;
    *shape_tree.mutable_element(3) = -100;
    assert_eq!(shape_tree.element(0), &1);
    assert_eq!(shape_tree.element(1), &42);
    assert_eq!(shape_tree.element(2), &123);
    assert_eq!(shape_tree.element(3), &-100);
    assert_eq!(ShapeUtil::compatible(&create_tuple_shape(), shape_tree.shape()), true);

    // Sum all elements in the shape.
    let mut sum = 0;
    let mut func = |_index: usize, data: &mut i64| {
      sum += *data;
    };
    shape_tree.for_each_mutable_element(&mut func);
    assert_eq!(sum, 66);

    // Test the copy constructor.
    let mut copy = shape_tree.clone();
    assert_eq!(copy.element(0), &1);
    assert_eq!(copy.element(1), &42);
    assert_eq!(copy.element(2), &123);
    assert_eq!(copy.element(3), &-100);

    // Write zero to all elements.
    let mut func2 = |_index: usize, data: &mut i64| {
      *data = 0;
    };
    shape_tree.for_each_mutable_element(&mut func2);
    assert_eq!(shape_tree.element(0), &0);
    assert_eq!(shape_tree.element(1), &0);
    assert_eq!(shape_tree.element(2), &0);
    assert_eq!(shape_tree.element(3), &0);
    assert_eq!(copy.element(0), &1);
    assert_eq!(copy.element(1), &42);
    assert_eq!(copy.element(2), &123);
    assert_eq!(copy.element(3), &-100);

    // Test the assignment operator.
    copy = shape_tree;
    assert_eq!(copy.element(0), &0);
    assert_eq!(copy.element(1), &0);
    assert_eq!(copy.element(2), &0);
    assert_eq!(copy.element(3), &0);
  }

  fn test_nested_tuple_shape() {
    //let mut shape_tree: ShapeTree<i64> = ShapeTree::new(create_nested_tuple_shape());
    //*shape_tree.mutable_element(0) = 42;
  }
}