#![allow(dead_code)]

use std::{collections::HashMap, usize};
use std::collections::hash_map::Iter;
use super::string_ref::StringRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringMapEntry<T> {
  key_length_: usize,
  second_: T
}

impl<T> StringMapEntry<T> {
  pub fn new() {}

  pub fn get_key_length(&self) -> usize {
    self.key_length_
  }

  pub fn get_value(&self) -> &T {
    &self.second_
  }

  pub fn set_value(&mut self, v: T) {
    self.second_ = v
  }

  pub fn get_key() {}

  pub fn get_key_data() {}

  pub fn first() {}

  pub fn get_string_map_entry_from_key_data() {}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringMap<T> {
  //hash_map_: HashMap<StringRef, StringMapEntry<T>>,
  hash_map_: HashMap<StringRef, T>,
}

impl<T> StringMap<T> {
  pub fn new() -> Self {
    StringMap { hash_map_: HashMap::new() }
  }

  //pub fn new_from(arr: [(StringRef, T); 10]) -> Self {
  //  StringMap { hash_map_: HashMap::from(arr) }
  //}

  pub fn new_from(rhs: StringMap<T>) -> Self {
    StringMap { hash_map_: rhs.hash_map_ }
  }

  pub fn rehash_table() {}

  pub fn lookup_bucket_for() {}

  // Look up the StringMapEntry for the specified key.
  pub fn find_key(&self, key: StringRef) -> Option<&T>/*Option<&StringMapEntry<T>>*/ {
    if self.hash_map_.is_empty() {
      return None;
    }
    self.hash_map_.get(&key)
  }

  // Remove the StringMapentry for the specified key from the table,
  // returning it.
  pub fn remove_key(&mut self, key: StringRef) -> Option<T>/*Option<StringMapEntry<T>>*/ {
    if self.hash_map_.is_empty() {
      return None;
    }
    self.hash_map_.remove(&key)
  }

  pub fn init() {}

  pub fn get_tombstone_val() {}

  //pub fn get_num_buckets(&self) -> u32 {
  //  self.num_buckets_
  //}

  pub fn get_num_items(&self) -> usize {
    self.hash_map_.len()
  }

  pub fn empty(&self) -> bool {
    self.hash_map_.is_empty()
  }

  pub fn size(&self) -> usize {
    self.hash_map_.len()
  }

  pub fn swap() {}

  pub fn begin(&self) -> Iter<StringRef, T> {
    self.hash_map_.iter()
  }

  pub fn end(&self) {}

  pub fn find(&self, key: StringRef) -> Option<&T>/*Option<&StringMapEntry<T>>*/ {
    self.find_key(key)
  }

  // Return the entry for the specified key.
  pub fn lookup(&self, key: StringRef) -> Option<&T> {
    self.find_key(key)
  }

  // Return 1 if the element is in the map, 0 otherwise.
  pub fn count(&self, key: StringRef) -> usize {
    if self.hash_map_.contains_key(&key) {
      return 1;
    } else {
      return 0;
    }
  }

  // Insert the specified key/value pair into the map.
  pub fn insert(&mut self, key: StringRef, value: T/*StringMapEntry<T>*/) -> Option<T>/*Option<StringMapEntry<T>>*/ {
    self.hash_map_.insert(key, value)
  }

  pub fn insert_or_assign() {}

  pub fn try_emplace() {}

  // Empties out the StringMap.
  pub fn clear(&mut self) {
    self.hash_map_.clear()
  }

  pub fn remove() {}

  pub fn erase(&mut self, key: StringRef) -> bool {
    if self.remove_key(key).is_none() {
      return false;
    } else {
      return true;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_map() {
    let test_map: StringMap<u32> = StringMap::new();
    assert_eq!(test_map.size(), 0);
    assert_eq!(test_map.empty(), true);

    let test_key = StringRef::new_from_string("key");
    let test_key_clone = test_key.clone();
    assert_eq!(test_map.count(test_key), 0);
    assert_eq!(test_map.find(test_key_clone), None);
  }

  #[test]
  fn test_single_entry_map() {
    let mut test_map: StringMap<u32> = StringMap::new();
    let test_key = StringRef::new_from_string("key");
    let test_key_clone = test_key.clone();
    let test_key_clone2 = test_key.clone();
    let result = test_map.insert(test_key, 1);
    assert_eq!(result, None);
    
    assert_eq!(test_map.size(), 1);
    assert_eq!(test_map.empty(), false);

    let elem = test_map.hash_map_.iter().next();
    assert_eq!(elem.unwrap().0.data(), "key");
    assert_eq!(elem.unwrap().1.clone(), 1);

    assert_eq!(test_map.count(test_key_clone), 1);
    assert_eq!(test_map.find(test_key_clone2), Some(&1));
  }

  #[test]
  fn test_clear() {
    let mut test_map: StringMap<u32> = StringMap::new();
    let test_key = StringRef::new_from_string("key");
    let test_key_clone = test_key.clone();
    let test_key_clone2 = test_key.clone();
    let result = test_map.insert(test_key, 1);
    assert_eq!(result, None);

    test_map.clear();
    assert_eq!(test_map.size(), 0);
    assert_eq!(test_map.empty(), true);
    assert_eq!(test_map.count(test_key_clone), 0);
    assert_eq!(test_map.find(test_key_clone2), None);
  }

  #[test]
  fn test_insert_and_erase() {
    let mut test_map: StringMap<u32> = StringMap::new();
    let test_key = StringRef::new_from_string("key");
    let test_key_clone = test_key.clone();
    let test_key_clone2 = test_key.clone();
    let mut result = test_map.insert(test_key, 1);
    assert_eq!(result, None);

    let test_other_key = StringRef::new_from_string("other_key");
    let test_other_key_clone = test_other_key.clone();
    result = test_map.insert(test_other_key, 2);
    assert_eq!(result, None);
    assert_eq!(test_map.erase(test_other_key_clone), true);

    assert_eq!(test_map.size(), 1);
    assert_eq!(test_map.empty(), false);
    let elem = test_map.hash_map_.iter().next();
    assert_eq!(elem.unwrap().0.data(), "key");
    assert_eq!(elem.unwrap().1.clone(), 1);
    assert_eq!(test_map.count(test_key_clone), 1);
    assert_eq!(test_map.find(test_key_clone2), Some(&1));
  }

  #[test]
  fn test_small_full_map() {
    let mut test_map: StringMap<i32> = StringMap::new();
    test_map.insert(StringRef::new_from_string("eins"), 1);
    test_map.insert(StringRef::new_from_string("zwei"), 2);
    test_map.insert(StringRef::new_from_string("drei"), 3);
    test_map.erase(StringRef::new_from_string("drei"));
    test_map.erase(StringRef::new_from_string("eins"));
    test_map.insert(StringRef::new_from_string("veir"), 4);
    test_map.insert(StringRef::new_from_string("funf"), 5);

    assert_eq!(test_map.size(), 3);
    assert_eq!(test_map.lookup(StringRef::new_from_string("eins")), None);
    assert_eq!(test_map.lookup(StringRef::new_from_string("zwei")), Some(&2));
    assert_eq!(test_map.lookup(StringRef::new_from_string("drei")), None);
    assert_eq!(test_map.lookup(StringRef::new_from_string("veir")), Some(&4));
    assert_eq!(test_map.lookup(StringRef::new_from_string("funf")), Some(&5));
  }

  #[test]
  fn test_copy_ctor() {
    let mut test_map: StringMap<i32> = StringMap::new();
    test_map.insert(StringRef::new_from_string("eins"), 1);
    test_map.insert(StringRef::new_from_string("zwei"), 2);
    test_map.insert(StringRef::new_from_string("drei"), 3);
    test_map.erase(StringRef::new_from_string("drei"));
    test_map.erase(StringRef::new_from_string("eins"));
    test_map.insert(StringRef::new_from_string("veir"), 4);
    test_map.insert(StringRef::new_from_string("funf"), 5);

    let test_map2 = StringMap::new_from(test_map);
    assert_eq!(test_map2.size(), 3);
    assert_eq!(test_map2.lookup(StringRef::new_from_string("eins")), None);
    assert_eq!(test_map2.lookup(StringRef::new_from_string("zwei")), Some(&2));
    assert_eq!(test_map2.lookup(StringRef::new_from_string("drei")), None);
    assert_eq!(test_map2.lookup(StringRef::new_from_string("veir")), Some(&4));
    assert_eq!(test_map2.lookup(StringRef::new_from_string("funf")), Some(&5)); 
  }

  #[test]
  fn test_equal_empty() {
    let test_map: StringMap<i32> = StringMap::new();
    let test_map2: StringMap<i32> = StringMap::new();
    assert_eq!(test_map == test_map2, true);
    assert_eq!(test_map != test_map2, false);
    assert_eq!(test_map, test_map);
  }

  #[test]
  fn test_equal_with_values() {
    let mut test_map: StringMap<i32> = StringMap::new();
    test_map.insert(StringRef::new_from_string("A"), 1);
    test_map.insert(StringRef::new_from_string("B"), 2);
    test_map.insert(StringRef::new_from_string("C"), 3);
    test_map.insert(StringRef::new_from_string("D"), 3);

    let mut test_map2: StringMap<i32> = StringMap::new();
    test_map2.insert(StringRef::new_from_string("A"), 1);
    test_map2.insert(StringRef::new_from_string("B"), 2);
    test_map2.insert(StringRef::new_from_string("C"), 3);
    test_map2.insert(StringRef::new_from_string("D"), 3);

    assert_eq!(test_map == test_map2, true);
    assert_eq!(test_map2 == test_map, true);
    assert_eq!(test_map != test_map2, false);
    assert_eq!(test_map2 != test_map, false);
    assert_eq!(test_map == test_map, true);
  }

  #[test]
  fn test_not_equal_missing_keys() {
    let mut test_map: StringMap<i32> = StringMap::new();
    test_map.insert(StringRef::new_from_string("A"), 1);
    test_map.insert(StringRef::new_from_string("B"), 2);

    let mut test_map2: StringMap<i32> = StringMap::new();
    test_map2.insert(StringRef::new_from_string("A"), 1);
    test_map2.insert(StringRef::new_from_string("B"), 2);
    test_map2.insert(StringRef::new_from_string("C"), 3);
    test_map2.insert(StringRef::new_from_string("D"), 3);

    assert_eq!(test_map == test_map2, false);
    assert_eq!(test_map2 == test_map, false);
    assert_eq!(test_map != test_map2, true);
    assert_eq!(test_map2 != test_map, true);
  }

  #[test]
  fn test_not_equal_with_different_values() {
    let mut test_map: StringMap<i32> = StringMap::new();
    test_map.insert(StringRef::new_from_string("A"), 1);
    test_map.insert(StringRef::new_from_string("B"), 2);
    test_map.insert(StringRef::new_from_string("C"), 100);
    test_map.insert(StringRef::new_from_string("D"), 3);

    let mut test_map2: StringMap<i32> = StringMap::new();
    test_map2.insert(StringRef::new_from_string("A"), 1);
    test_map2.insert(StringRef::new_from_string("B"), 2);
    test_map2.insert(StringRef::new_from_string("C"), 3);
    test_map2.insert(StringRef::new_from_string("D"), 3);

    assert_eq!(test_map == test_map2, false);
    assert_eq!(test_map2 == test_map, false);
    assert_eq!(test_map != test_map2, true);
    assert_eq!(test_map2 != test_map, true);
  }
}