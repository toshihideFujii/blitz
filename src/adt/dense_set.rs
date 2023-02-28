#![allow(dead_code)]
use std::{hash::Hash, fmt::Debug};

use crate::adt::dense_map::DenseMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct DenseSetEmpty {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenseSet<Key> where Key: Debug + Clone +  Eq + Hash {
  map: DenseMap<Key, DenseSetEmpty>
}

impl<Key> DenseSet<Key> where Key: Debug + Clone +  Eq + Hash {
  pub fn new() -> Self {
    DenseSet { map: DenseMap::new() }
  }

  pub fn new_from_vec(v: Vec<Key>) -> Self {
    let mut set: DenseSet<Key> = DenseSet { map: DenseMap::new() };
    for k in v {
      set.map.insert(k, DenseSetEmpty {});
    }
    set
  }

  pub fn empty(&self) -> bool {
    self.map.empty()
  }

  pub fn size(&self) -> usize {
    self.map.size()
  }

  pub fn get_memory_size() {}

  // Grow the DenseSet so that it has at least size buckets.
  // Will not shrink the size of the set.
  pub fn resize(&self, _size: usize) {
  }

  // Grow the DenseSet so that it can contain at least size items
  // before resizing again.
  pub fn reserve(&mut self, size: usize) {
    self.map.reserve(size)
  }

  pub fn clear(&mut self) {
    self.map.clear()
  }

  // Return true if the specified key is in the set, false otherwise.
  pub fn count(&self, k: Key) -> bool {
    self.map.count(k)
  }

  pub fn erase(&mut self, k: Key) -> bool {
    if let Some(_) = self.map.erase(k) {
      return true;
    } else {
      return false;
    }
  }

  pub fn swap() {}

  pub fn find(&self, k: Key) -> Option<Key> {
    let result = k.clone();
    if let Some(_) = self.map.find(k) {
      return Some(result);
    } else {
      return None;
    }
  }

  // Check if the set contains the given element.
  pub fn contains(&self, k: Key) -> bool {
    if let Some(_) = self.map.find(k) {
      return true;
    } else {
      return false;
    }
  }

  pub fn find_as() {}

  pub fn insert(&mut self, k: Key) -> bool {
    let v = DenseSetEmpty{};
    if let Some(_) = self.map.insert(k, v) {
      return true;
    } else {
      return false;
    }
  }

  pub fn insert_as() {}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_double_entry_sest() {
    let mut set: DenseSet<i64> = DenseSet::new();
    set.insert(0);
    set.insert(1);
    assert_eq!(set.size(), 2);
    assert_eq!(set.count(2), false);
  }

  #[test]
  fn test_initializer_list() {
    let v = vec![1, 2, 1, 4];
    let set: DenseSet<i64> = DenseSet::new_from_vec(v);
    assert_eq!(set.size(), 3);
    assert_eq!(set.count(1), true);
    assert_eq!(set.count(2), true);
    assert_eq!(set.count(4), true);
    assert_eq!(set.count(3), false);
  }

  #[test]
  fn test_initializer_list_non_pow2_len() {
    let v = vec![1, 2, 3];
    let set: DenseSet<i64> = DenseSet::new_from_vec(v);
    assert_eq!(set.size(), 3);
    assert_eq!(set.count(1), true);
    assert_eq!(set.count(2), true);
    assert_eq!(set.count(3), true);
  }

  #[test]
  fn test_empty_initializer_list() {
    let v = vec![];
    let set: DenseSet<i64> = DenseSet::new_from_vec(v);
    assert_eq!(set.size(), 0);
    assert_eq!(set.count(0), false);
  }

  #[test]
  fn test_find() {
    let mut set: DenseSet<u64> = DenseSet::new();
    set.insert(0);
    set.insert(1);
    set.insert(2);

    assert_eq!(set.size(), 3);
    assert_eq!(set.find(0), Some(0));
    assert_eq!(set.find(1), Some(1));
    assert_eq!(set.find(2), Some(2));
    assert_eq!(set.find(3), None);

    // TODO: find_as()
  }

  #[test]
  fn test_equality_comparison() {
    let v1 = vec![1, 2, 3, 4];
    let s1 = DenseSet::new_from_vec(v1);
    let v2 = vec![4, 3, 2, 1];
    let s2 = DenseSet::new_from_vec(v2);
    let v3 = vec![2, 3, 4, 5];
    let s3 = DenseSet::new_from_vec(v3);

    assert_eq!(s1 == s2, true);
    assert_eq!(s1 == s3, false);
  }

}