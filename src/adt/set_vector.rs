#![allow(dead_code)]

use std::{fmt::Debug, hash::Hash};
use std::{ops::*};

use crate::adt::dense_set::DenseSet;

// A vector that has set insertion semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct SetVector<T> where T: Debug +Clone + Hash + Eq {
  set: DenseSet<T>,
  vector: Vec<T>
}

impl<T> SetVector<T> where T: Debug + Clone + Hash + Eq {
  pub fn new() -> Self {
    SetVector { set: DenseSet::new(), vector: Vec::new() }
  }

  pub fn get_array_ref() {}

  // Clear the SetVector and return the underlying vector.
  pub fn take_vector() {}

  // Determine if the SetVector is empty or not.
  pub fn empty(&self) -> bool {
    self.vector.is_empty()
  }

  // Determine the number of elements in the SetVector.
  pub fn size(&self) -> usize {
    self.vector.len()
  }

  // Return the first element of the SetVector.
  pub fn front(&self) -> Option<&T> {
    debug_assert!(!self.empty(), "Cannot call front() on empty SetVector.");
    self.vector.get(0)
  }

  // Return the last element of the SetVector.
  pub fn back(&self) -> Option<&T>{
    debug_assert!(!self.empty(), "Cannot call back() on empty SetVector.");
    self.vector.get(self.vector.len())
  }

  // Insert a new element into the SetVector.
  pub fn insert(&mut self, x: T) -> bool {
    let value = x.clone();
    if self.set.insert(x) == true {
      self.vector.push(value);
      return true;
    }
    return false;
  }

  // Remove an item from the SetVector.
  pub fn remove(&mut self, x: T) {
    let x_clone = x.clone();
    if self.set.erase(x) {
      let mut index: usize = 0;
      for i in self.vector.iter() {
        if *i == x_clone {
          self.vector.remove(index);
          break;
        }
        index += 1;
      }
    }
  }

  // Erase a single element from the set vector.
  pub fn erase(&mut self, k: T) {
    let k_clone = k.clone();
    let k_clone2 = k.clone();
    //debug_assert!(self.count(k) == true, "Corrupted SetVector instances.");
    self.set.erase(k_clone);
    let mut index: usize = 0;
    for i in self.vector.iter() {
      if *i == k_clone2 {
        self.vector.remove(index);
        break;
      }
      index += 1;
    }
  }

  pub fn remove_if() {}

  // Check if the SetVector contains the given key.
  pub fn contains(&self, k: T) -> bool {
    self.set.contains(k)
  }

  pub fn count(&self, k: T) -> bool {
    self.set.count(k)
  }

  // Completely ckear the SetVector
  pub fn clear(&mut self) {
    self.set.clear();
    self.vector.clear();
  }

  // Remove the last element of the SetVector.
  pub fn pop_back(&mut self) {
    let val = self.back().unwrap().clone();
    self.set.erase(val);
    self.vector.pop();
  }

  pub fn pop_back_val(&mut self) -> T {
    let val = self.back().unwrap().clone();
    self.pop_back();
    val
  }

  pub fn set_union() {}

  pub fn set_subtract() {}

  pub fn swap() {}
}

impl<T> Index<usize> for SetVector<T> where T: Debug + Clone + Hash + Eq {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
      self.vector.get(index).unwrap()
    }
  }

impl<T> Iterator for SetVector<T> where T: Debug + Clone + Hash + Eq {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    let mut iter = self.vector.iter();
    iter.next().cloned()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_erase() {
    let mut s: SetVector<i32> = SetVector::new();
    s.insert(0);
    s.insert(1);
    s.insert(2);
    s.erase(1);

    assert_eq!(s.size(), 2);
    assert_eq!(s.contains(0), true);
    assert_eq!(s.contains(1), false);
    assert_eq!(s.contains(2), true);
  }

  #[test]
  fn test_contains() {
    let mut s: SetVector<i32> = SetVector::new();
    s.insert(0);
    s.insert(1);
    s.insert(2);

    assert_eq!(s.contains(0), true);
    assert_eq!(s.contains(1), true);
    assert_eq!(s.contains(2), true);
    assert_eq!(s.contains(-1), false);

    s.insert(2);
    assert_eq!(s.contains(2), true);
    s.remove(2);
    assert_eq!(s.contains(2), false);
  }
}