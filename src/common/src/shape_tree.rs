#![allow(dead_code)]

use crate::shape::Shape;

struct Entry {
  node_id: usize,
  children_start_id: i64
}

struct IndexTable {
  entries: Vec<Entry>
}

impl IndexTable {
  pub fn new() {}
  pub fn empty() {}

  pub fn get(&self, index: usize) -> &Entry {
    self.entries.get(index).unwrap()
  }
}

pub struct ShapeTree<T> {
  nodes: Vec<(usize, T)>,
  index_table: IndexTable,
  //shape_storage: Shape,
  shape: Shape
}

impl<T> ShapeTree<T> {
  pub fn new() {}

  // Returns the data element associated with the array in the shape at the
  // given index.
  pub fn element(&self, index: usize) -> &T {
    &self.find(index).1
  }

  pub fn mutable_element(&mut self, index: usize) -> &mut T {
    &mut self.find_mut(index).1
  }

  // Return the shape represented with this ShapeTree.
  pub fn shape(&self) -> &Shape {
    &self.shape
  }

  pub fn replace_shape_ptr() {}

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
    self.nodes.len()
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
    where F: FnMut(usize, &T)
  {
    for node in &self.nodes {
      func(node.0, &node.1);
    }
  }

  pub fn for_each_element_with_status() {}
  pub fn for_each_mutable_element_with_status() {}
  pub fn for_each_element_post_order() {}
  pub fn for_each_mutable_element_post_order() {}
  pub fn for_each_element_post_order_with_status() {}
  pub fn for_each_mutable_element_post_order_with_status() {}
  pub fn map() {}
  pub fn copy_subtree_from() {}
  pub fn sub_shape_tree() {}

}