#![allow(dead_code)]

// This file defines classes to implement an intrusive doubly
// linked vec class (i.e. each node of the vec musst contain a
// next and previous field for the vec).)

use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct IPList<T> {
  vec: Vec<T>
}

impl<T> IPList<T> {
  pub fn new() -> Self {
    IPList { vec: Vec::new() }
  }

  pub fn swap() {}

  pub fn insert(&mut self, index: usize, val: T) {
    self.vec.insert(index, val);
  }

  pub fn insert_after(&mut self, index: usize, val: T) {
    let index_plus_one = index + 1;
    self.vec.insert(index_plus_one, val);
  }

  pub fn clone_from(&mut self, other: &mut IPList<T>) where T: Clone {
    *self = other.clone();
  }

  pub fn remove(&mut self, index: usize) {
    self.vec.remove(index);
  }

  pub fn erase() {}
  pub fn clear_and_leak_nodes_unsafely() {}
  pub fn transfer() {}

  pub fn size(&self) -> usize {
    self.vec.len()
  }

  pub fn clear(&mut self) {
    self.vec.clear();
  }

  pub fn push_front(&mut self, val: T) {
    self.vec.insert(0, val);
  }

  pub fn push_back(&mut self, val: T) {
    self.vec.push(val);
  }
  
  pub fn pop_front(&mut self) -> Option<T> {
    if !self.vec.is_empty() {
      return Some(self.vec.remove(0));
    }
    None
  }

  pub fn pop_back(&mut self) -> Option<T> {
    self.vec.pop()
  }

  pub fn splice(&mut self, range: Range<usize>, other: &mut IPList<T>)
    -> std::vec::Splice<std::vec::IntoIter<T>> where T: Clone
  {
    let vec = other.vec.clone();
    self.vec.splice(range, vec)
  }

  pub fn merge(&mut self, right: &mut IPList<T>) {
    self.vec.append(&mut right.vec)
  }

  // Get the previous node, or None for the list head.
  pub fn get_prev_node(&self, index: usize) -> Option<&T> {
    if index <= 0 {
      return None;
    }
    self.vec.get(index - 1)
  }

  // Get the next node, or None for the list tail.
  pub fn get_next_node(&self, index: usize) -> Option<&T> {
    self.vec.get(index + 1)
  }

  pub fn get(&self, index: usize) -> Option<&T> {
    self.vec.get(index)
  }

  pub fn front(&self) -> Option<&T> {
    self.vec.first()
  }

  pub fn back(&self) -> Option<&T> {
    self.vec.last()
  }
}

/*
impl<T> Iterator for IPList<T> {
  type Item = T;
}
*/

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Clone, PartialEq)]
  struct Node {
    value: i64
  }
  impl Node {
    pub fn new() -> Self {
      Node { value: 0 }
    }
    pub fn new_from(value: i64) -> Self {
      Node { value: value }
    }
    pub fn value(&self) -> i64 {
      self.value
    }
  }

  #[test]
  fn test_basic() {
    let mut list: IPList<Node> = IPList::new();
    list.push_back(Node::new_from(1));
    assert_eq!(list.back().unwrap().value(), 1);
    assert_eq!(list.get_prev_node(0), None);
    assert_eq!(list.get_next_node(0), None);

    list.push_back(Node::new_from(2));
    assert_eq!(list.back().unwrap().value(), 2);
    assert_eq!(list.get_next_node(0).unwrap().value(), 2);
    assert_eq!(list.get_prev_node(1).unwrap().value(), 1);
  }

  #[test]
  fn test_clone_from() {
    let mut l1: IPList<Node> = IPList::new();
    let mut l2: IPList<Node> = IPList::new();
    let mut l3: IPList<Node> = IPList::new();

    l1.push_back(Node::new());
    l1.push_back(Node::new_from(1));

    l2.clone_from(&mut l1);

    l3.push_back(Node::new_from(7));
    l3.clone_from(&mut l1);

    assert_eq!(l1.size(), 2);
    assert_eq!(l1.front().unwrap(), &Node::new());
    assert_eq!(l1.back().unwrap(), &Node::new_from(1));

    assert_eq!(l2.size(), 2);
    assert_eq!(l2.front().unwrap(), &Node::new());
    assert_eq!(l2.back().unwrap(), &Node::new_from(1));

    assert_eq!(l3.size(), 2);
    assert_eq!(l3.front().unwrap().value(), 0);
    assert_eq!(l3.back().unwrap().value(), 1);
  }

  #[test]
  fn test_add_node() {
    let mut l1: IPList<Node> = IPList::new();
    let node = Node::new_from(7);
    let node_clone = node.clone();

    l1.insert(0, node);
    assert_eq!(l1.size(), 1);
    assert_eq!(l1.front().unwrap(), &node_clone);

    let mut l2: IPList<Node> = IPList::new();
    l2.splice(0..0, &mut l1);
    assert_eq!(l2.size(), 1);
    assert_eq!(l2.front().unwrap(), &node_clone);

    l1.remove(0);
    assert_eq!(l1.size(), 0);
  }
}