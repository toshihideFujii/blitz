#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash, fmt::Debug};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenseMap<Key, Value> where Key: Eq + Hash{
  map: HashMap<Key, Value>
}

impl<Key, Value> DenseMap<Key, Value>
  where Key: Debug + Clone + PartialEq + Eq + Hash + Sized, Value: Debug + Clone {
  pub fn new() -> Self {
    DenseMap { map: HashMap::new() }
  }

  pub fn copy(other: &DenseMap<Key, Value>) -> Self {
    DenseMap { map: other.map.clone() }
  }

  pub fn empty(&self) -> bool {
    self.map.is_empty()
  }

  pub fn size(&self) -> usize {
    self.map.len()
  }

  // Grow the densemap so that it can contain at least num_entries
  // items before resizing again.
  pub fn reserve(&mut self, num_entries: usize) {
    self.map.reserve(num_entries)
  }

  pub fn clear(&mut self) {
    self.map.clear()
  }

  // Return true if the specified key is in the map, false otherwise.
  pub fn count(&self, k: Key) -> bool {
    self.map.contains_key(&k)
  }

  pub fn find(&self, k: Key) -> Option<&Value>{
    self.map.get(&k)
  }

  pub fn find_as() {}

  pub fn lookup() {}

  // Inserts key, value pair into the map if the key isn't already
  // in the map. If the key is already in the map, it returns false
  // and doesn't update the value.
  pub fn insert(&mut self, k: Key, v: Value) -> Option<Value> {
    if !self.map.contains_key(&k) {
      self.map.insert(k, v)
    } else {
      None
    }
  }

  pub fn insert_as() {}

  pub fn try_emplace() {}

  pub fn erase(&mut self, k: Key) -> Option<Value> {
    self.map.remove(&k)
  }

  pub fn iter(&self) -> std::collections::hash_map::Iter<Key, Value> {
    self.map.iter()
  }

  pub fn find_and_construct() {}

  pub fn is_pointer_into_buckets_array() {}

  pub fn get_pointer_into_buckets_array() {}

  pub fn destroy_all() {}

  pub fn init_empty() {}

  pub fn get_min_bucket_to_reserve_for_entries() {}

  pub fn move_from_old_buckets() {}

  pub fn copy_from() {}

  pub fn get_hash_value() {}

  pub fn get_empty_key() {}

  pub fn get_tombstone_key() {}

  fn increment_num_entries() {}

  fn decrement_num_entries() {}

  fn increment_num_tombstones() {}

  fn decrement_num_tombstones() {}

  fn allocate_buckets() {}

  fn grow() {}

  fn shrink_and_clear() {}

  fn insert_into_bucket() {}

  fn insert_into_bucket_with_lookup() {}
}

/*
impl<Key, Value> Iterator for DenseMap<Key, Value> {
  type Item = (&'static Key, &'static Value);
  fn next(&mut self) -> Option<Self::Item> {
    self.map.iter().next().clone()
  }
}
*/


#[cfg(test)]
mod tests {
  use super::*;
  use crate::adt::string_ref::StringRef;

  #[test]
  fn test_empty_map() {
    let map: DenseMap<u64, u64> = DenseMap::new();
    assert_eq!(map.size(), 0);
    assert_eq!(map.empty(), true);
    assert_eq!(map.count(0), false);
    assert_eq!(map.find(0), None);
  }

  #[test]
  fn test_single_entry_map() {
    let mut map: DenseMap<u64, u64> = DenseMap::new();
    map.insert(0, 1);

    assert_eq!(map.size(), 1);
    assert_eq!(map.empty(), false);

    assert_eq!(map.count(0), true);
    assert_eq!(map.find(0), Some(&1));
  }

  #[test]
  fn test_clear() {
    let mut map: DenseMap<u64, u64> = DenseMap::new();
    map.insert(0, 1);
    map.clear();
    assert_eq!(map.size(), 0);
    assert_eq!(map.empty(), true);
  }

  #[test]
  fn test_erase() {
    let mut map: DenseMap<u64, u64> = DenseMap::new();
    map.insert(0, 1);
    map.erase(0);
    assert_eq!(map.size(), 0);
    assert_eq!(map.empty(), true);
  }

  #[test]
  fn test_insert() {
    let mut map: DenseMap<u64, u64> = DenseMap::new();
    map.insert(0, 1);
    assert_eq!(map.size(), 1);
    assert_eq!(map.find(0), Some(&1));
  }

  #[test]
  fn test_copy() {
    let mut map: DenseMap<u64, u64> = DenseMap::new();
    map.insert(0, 1);
    let copy_map = DenseMap::copy(&map);
    assert_eq!(copy_map.size(), 1);
    assert_eq!(copy_map.find(0), Some(&1));
  }

  #[test]
  fn test_copy_from_empty() {
    let mut map: DenseMap<u64, u64> = DenseMap::new();
    map.insert(0, 1);
    map.insert(1, 2);
    map.insert(2, 3);
    map.clear();
    let copy_map = DenseMap::copy(&map);
    assert_eq!(copy_map.empty(), true);
  }

  #[test]
  fn test_iteration() {
    let mut visited = [false; 100];
    let mut map: DenseMap<u64, u64> = DenseMap::new();
    for i in 0..100 {
      map.insert(i as u64, i as u64);
    }
    assert_eq!(map.size(), 100);

    let iter = map.iter();
    for it in iter {
      let key_index = *it.0;
      visited[key_index as usize] = true;
    }

    for i in 0..100 {
      println!("index {}", i);
      assert_eq!(visited[i], true);
    }
  }

  #[test]
  fn test_equality() {
    let mut map1: DenseMap<u64, u64> = DenseMap::new();
    map1.insert(0, 0);
    map1.insert(1, 2);
    let mut map2: DenseMap<u64, u64> = DenseMap::new();
    map2.insert(0, 0);
    map2.insert(1, 2);
    let mut map3: DenseMap<u64, u64> = DenseMap::new();
    map3.insert(0, 0);
    map3.insert(1, 3);

    assert_eq!(map1 == map2, true);
    assert_eq!(map1 == map3, false);
  }

  #[test]
  fn test_stringref() {
    let mut map: DenseMap<StringRef, u64> = DenseMap::new();
    map.insert(StringRef::new_from_string("a"), 1);
    map.insert(StringRef::new_from_string("b"), 2);
    map.insert(StringRef::new_from_string("c"), 3);

    assert_eq!(map.size(), 3);
    assert_eq!(map.find(StringRef::new_from_string("a")), Some(&1));
    assert_eq!(map.find(StringRef::new_from_string("b")), Some(&2));
    assert_eq!(map.find(StringRef::new_from_string("c")), Some(&3));
    assert_eq!(map.find(StringRef::new_from_string("q")), None);

    assert_eq!(map.find(StringRef::new_from_string("")), None);
    map.insert(StringRef::new_from_string(""), 42);
    assert_eq!(map.find(StringRef::new_from_string("")), Some(&42));
  }
}