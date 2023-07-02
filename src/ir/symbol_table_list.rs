#![allow(dead_code)]

// This file defines a generic class that is used to implement the
// automatic sumbol table manipulation that occurs when you put (for
// example) a named instruction into a basic block.

use std::ops::Range;
use crate::adt::ilist::IPList;

#[derive(Debug)]
pub struct SymbolTableList<T> /* where T: PartialEq*/{
  list: IPList<T>
}

impl<T> SymbolTableList<T> /*where T: PartialEq */{
  pub fn new() -> Self {
    SymbolTableList { list: IPList::new() }
  }
  
  pub fn swap() {}
  
  pub fn insert(&mut self, index: usize, val: T) {
    self.list.insert(index, val)
  }
    
  pub fn insert_after(&mut self, index: usize, val: T) {
    self.list.insert_after(index, val)  
  }
  
  //pub fn clone_from(&mut self, other: &mut IPList<T>) where T: Clone {
    //self.list.clone_from(other)
  //}
    
  pub fn remove(&mut self, index: usize) {
    self.list.remove(index)
  }
    
  pub fn erase() {}
  pub fn clear_and_leak_nodes_unsafely() {}
  pub fn transfer() {}
  
  pub fn size(&self) -> usize {
    self.list.size()
  }

  pub fn empty(&self) -> bool {
    self.list.size() == 0
  }
    
  pub fn clear(&mut self) {
    self.list.clear()
  }
    
  pub fn push_front(&mut self, val: T) {
    self.list.push_front(val)
  }
    
  pub fn push_back(&mut self, val: T) {
    self.list.push_back(val)
  }
      
  pub fn pop_front(&mut self) -> Option<T> {
    self.list.pop_front()
  }
    
  pub fn pop_back(&mut self) -> Option<T> {
    self.list.pop_back()
  }
  
  pub fn splice(&mut self, range: Range<usize>, other: &mut IPList<T>)
    -> std::vec::Splice<std::vec::IntoIter<T>> where T: Clone
  {
    self.list.splice(range, other)
  }
    
  pub fn merge(&mut self, right: &mut IPList<T>) {
    self.list.merge(right)
  }
    
  // Get the previous node, or None for the list head.
  pub fn get_prev_node(&self, index: usize) -> Option<&T> {
    self.list.get_prev_node(index)
  }
    
  // Get the next node, or None for the list tail.
  pub fn get_next_node(&self, index: usize) -> Option<&T> {
    self.list.get_next_node(index)
  }

  pub fn get(&self, index: usize) -> Option<&T> {
    self.list.get(index)
  }
    
  pub fn front(&self) -> Option<&T> {
    self.list.front()
  }
    
  pub fn back(&self) -> Option<&T> {
    self.list.back()
  }

  pub fn set_sym_tab_object(&self) {}
}
/*
impl<T> Iterator for SymbolTableList<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    //self.list.g
  }
}
*/