#![allow(dead_code)]

use std::{collections::HashSet, hash::Hash};
use crate::ir::attribute_impl::AttributeSetNode;

use super::small_vector::SmallVector;

// This class is used to gather all the unique data bits of a node.
// When all the bits are gathered this class is used to produce a 
// hash value for the node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FoldingSetNodeID {
  bits: SmallVector<u32>
}

impl FoldingSetNodeID {
  // Vector of all the data bits that make the node unique.
  // Use a SmallVector to avoid a heap allocation in the common case.
  pub fn new() -> Self {
    FoldingSetNodeID { bits: SmallVector::new() }
  }

  /*
  pub fn add_pointer<T>(&self, ptr: T) {
    let i: u32 = ptr as u32;
    self.add_integer_unsigned_32(i)
  }
  */

  pub fn add_integer_i32(&mut self, i: i32) {
    self.bits.push_back(i as u32);
  }

  pub fn add_integer_u32(&mut self, i: u32) {
    self.bits.push_back(i);
  }

  //pub fn add_integer_long() {}
  //pub fn add_integer_unsigned_long() {}

  pub fn add_integer_i64(&mut self, i: i64) {
    self.bits.push_back(i as u32);
  }

  pub fn add_integer_u64(&mut self, i: u64) {
    self.bits.push_back(i as u32);
  }

  pub fn add_boolean(&mut self, b: bool) {
    if b {
      self.add_integer_u32(1);
    } else {
      self.add_integer_u32(0);
    }
  }

  pub fn add_string() {}

  // Adds the bit data of another ID to this.
  pub fn add_node_id(&mut self, id: &mut FoldingSetNodeID) {
    self.bits.append(&mut id.bits)
  }

  pub fn add_attr_set_node_id(&mut self, node: &AttributeSetNode) {
    self.add_integer_u64(node.id);
  }

  pub fn add() {}

  // Clear the accumulated profile, allowing this FoldingSetNodeID
  // object to be used to compute a new profile.
  pub fn clear(&mut self) {
    self.bits.clear()
  }

  // Compute a strong hash value for this FoldingSetNodeId, used to
  // lookup the node in the FoldingSet.
  pub fn compute_hash() {}
  pub fn intern() {}
}

struct Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct FoldingSet<T> where T: PartialEq + Eq + Hash {
  set: HashSet<T>
}

impl<T> FoldingSet<T> where T: Hash + Eq {
  pub fn new() -> Self {
    FoldingSet { set: HashSet::new() }
  }

  // Remove all nodes from the folding set.
  pub fn clear(&mut self) {
    self.set.clear()
  }

  // Returns the number of nodes in the folding set.
  pub fn size(&self) -> usize {
    self.set.len()
  }

  // Returns true if the there are no nodes in the folding set.
  pub fn empty(&self) -> bool {
    self.set.is_empty()
  }

  // Returns the number of nodes permitted in the folding set
  // before a rebucket operation is performed.
  pub fn capacity(&self) -> usize {
    self.set.capacity()
  }

  // Increase the number of buckets such that adding the elt_count-th
  // node won't cause a rebucket operation.
  pub fn reserve(&mut self, elt_count: usize) {
    self.set.reserve(elt_count)
  }

  // Remove a node from the folding set, returning true if one
  // was removed or false if the node was not in the folding set.
  pub fn remove_node(&mut self, n: &T) -> bool {
    self.set.remove(n)
  }

  pub fn get_or_insert_node() {}
  //pub fn find_node_or_insert_pos(&self, id: &FoldingSetNodeID) -> T {
    //self.set.get(value)
  //}

  // Insert the specified node into the folding set, knowing that
  // it is not already in the folding set.
  pub fn insert_node(&mut self, n: T) -> bool {
    self.set.insert(n)
  }

  pub fn get_node_profile() {}
  pub fn node_equals() {}
  pub fn compute_node_hash() {}
  pub fn get_folding_set_info() {}
}

struct ContextualFoldingSet {}

struct FoldingSetVector {}
impl FoldingSetVector {
  pub fn clear() {}
  pub fn find_node_or_insert_pos() {}
  pub fn get_or_insert_node() {}
  pub fn insert_node() {}
  pub fn size() {}
  pub fn empty() {}
}

fn profile(x: u32, mut id: FoldingSetNodeID) {
  id.add_integer_u32(x)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Clone, Hash, PartialEq, Eq)]
  struct TrivialPair {
    key: u32,
    value: u32
  }
  impl TrivialPair {
    pub fn new(k: u32, v: u32) -> Self {
      TrivialPair { key: k, value: v }
    }
    pub fn profile(&self, id: &mut FoldingSetNodeID) {
      id.add_integer_u32(self.key);
      id.add_integer_u32(self.value);
    }
  }

  #[test]
  fn test_id_comparison() {
    /*
    let mut set: FoldingSet<TrivialPair> = FoldingSet::new();
    let t = TrivialPair::new(99, 42);
    let t_clone = t.clone();
    set.insert_node(t);

    let mut id = FoldingSetNodeID::new();
    t_clone.profile(&mut id);
    */
  }

  #[test]
  fn test_remove_node_that_is_present() {
    let mut set: FoldingSet<u32> = FoldingSet::new();
    set.insert_node(1);
    assert_eq!(set.size(), 1);
    let was_there = set.remove_node(&1);
    assert_eq!(was_there, true);
    assert_eq!(set.size(), 0);
  }

  #[test]
  fn test_remove_node_that_is_absent() {
    let mut set: FoldingSet<u32> = FoldingSet::new();
    let was_there = set.remove_node(&1);
    assert_eq!(was_there, false);
    assert_eq!(set.size(), 0);
  }

  #[test]
  fn test_empty_is_true() {
    let set: FoldingSet<u32> = FoldingSet::new();
    assert_eq!(set.empty(), true);
  }

  #[test]
  fn test_empty_is_false() {
    let mut set: FoldingSet<u32> = FoldingSet::new();
    set.insert_node(1);
    assert_eq!(set.empty(), false);
  }

  #[test]
  fn test_empty_clear_on_empty() {
    let mut set: FoldingSet<u32> = FoldingSet::new();
    set.clear();
    assert_eq!(set.empty(), true);
  }

  #[test]
  fn test_empty_clear_on_non_empty() {
    let mut set: FoldingSet<u32> = FoldingSet::new();
    set.insert_node(1);
    set.clear();
    assert_eq!(set.empty(), true);
  }

  #[test]
  fn test_capacity_larger_than_reserve() {
    let mut set: FoldingSet<u32> = FoldingSet::new();
    let old_capacity = set.capacity();    
    set.reserve(old_capacity + 1);
    assert_eq!(set.capacity() >= old_capacity + 1, true);
  }

  #[test]
  #[should_panic]
  fn test_small_reserve_changes_nothing() {
    let mut set: FoldingSet<u32> = FoldingSet::new();
    let old_capacity = set.capacity();    
    set.reserve(old_capacity - 1);
  }
}