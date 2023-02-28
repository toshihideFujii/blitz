#![allow(dead_code)]

use std::{ops::*};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct SmallVector<T> {
  vec: Vec<T>
}

impl<T> SmallVector<T> {
  pub fn new() -> Self {
    SmallVector { vec: Vec::new() }
  }

  pub fn size(&self) -> usize {
    self.vec.len()
  }

  pub fn capacity(&self) -> usize {
    self.vec.capacity()
  }

  pub fn empty(&self) -> bool {
    self.vec.is_empty()
  }

  pub fn set_size(&mut self, n: usize) {
    debug_assert!(n <= self.capacity());
    self.vec.reserve(n)
  }

  pub fn get_first_el() {}

  pub fn grow_pod() {}

  pub fn is_small() {}

  pub fn reset_to_small() {}

  pub fn is_reference_to_range() {}

  pub fn is_reference_to_storage() {}

  pub fn is_range_in_storage() {}

  pub fn is_safe_to_reference_after_resize() {}

  pub fn begin(&self) {}

  pub fn end() {}

  pub fn rbegin() {}

  pub fn rend() {}

  pub fn size_in_bytes() {}

  pub fn max_size() {}

  pub fn capacity_in_bytes() {}

  pub fn data() {}

  pub fn front(&self) -> Option<&T> {
    self.vec.get(0)
  }

  pub fn back(&self) -> Option<&T> {
    self.vec.get(self.size())
  }

  pub fn push_back(&mut self, element: T) {
    self.vec.push(element)
  }

  pub fn pop_back(&mut self) -> Option<T> {
    self.vec.pop()
  }

  pub fn clear(&mut self) {
    self.vec.clear()
  }

  pub fn resize(&mut self, new_len: usize) where T: std::default::Default {
    self.vec.resize_with(new_len, Default::default);
  }

  // Like resize, but requires that n is less than size().
  pub fn truncate(&mut self, n: usize) {
    self.vec.truncate(n)
  }

  pub fn reserve(&mut self, n: usize) {
    self.vec.reserve(n)
  }

  pub fn pop_back_n(&mut self, num_items: usize) {
    debug_assert!(self.size() >= num_items);
    self.truncate(self.size() - num_items)
  }

  pub fn swap() {}

  pub fn append(&mut self, rhs: &mut SmallVector<T>) where T: std::clone::Clone {
    let mut vec = Vec::new();
    vec.extend_from_slice(&rhs.vec);
    self.vec.append(&mut vec);
  }

  pub fn assign() {}

  pub fn erase(&mut self, index: usize) -> T {
    self.vec.remove(index)
  }

  pub fn insert(&mut self, index: usize, element: T) {
    self.vec.insert(index, element)
  }

  pub fn find(&self, x: &T) -> Option<usize> where T: PartialEq {
    for i in 0..self.vec.len() {
      if *self.vec.get(i).unwrap() == *x {
        return Some(i);
      }
    }
    None
  }
}

impl<T> Index<usize> for SmallVector<T> {
  type Output = T;
  fn index(&self, index: usize) -> &Self::Output {
    self.vec.get(index).unwrap()
  }
}

impl<T> Iterator for SmallVector<T> where T: Clone {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    let mut iter = self.vec.iter();
    iter.next().cloned()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_empty<T>(v: &SmallVector<T>) {
    assert_eq!(v.size(), 0);
    assert_eq!(v.empty(), true);
  }

  fn assert_values_in_order<T>(v: &SmallVector<T>, size: usize, values: &Vec<T>)
    where T: std::fmt::Debug + std::cmp::PartialEq {
    assert_eq!(v.size(), size);
    for i in 0..size {
      assert_eq!(v.index(i), values.get(i).unwrap());
    }
  }

  fn make_sequence(v: &mut SmallVector<u64>, start: u64, end: u64) {
    for i in start..=end {
      v.push_back(i);
    }
  }

  #[test]
  fn test_empty() {
    let v: SmallVector<u64> = SmallVector::new();
    assert_empty(&v);
  }

  #[test]
  fn test_push_pop() {
    let mut v: SmallVector<u64> = SmallVector::new();
    v.push_back(1);
    let mut vec: Vec<u64> = vec![1];
    assert_values_in_order(&v, 1, &vec);
    assert_eq!(v.empty(), false);

    v.push_back(2);
    vec.push(2);
    assert_values_in_order(&v, 2, &vec);

    v.reserve(v.size() + 1);
    v.insert(0, 2);
    vec.insert(0, 2);
    assert_values_in_order(&v, 3, &vec);

    v.pop_back();
    vec.pop();
    assert_values_in_order(&v, 2, &vec);

    v.pop_back_n(2);
    assert_empty(&v);
  }

  #[test]
  fn test_clear() {
    let mut v: SmallVector<u64> = SmallVector::new();
    v.reserve(2);
    make_sequence(&mut v, 1, 2);
    v.clear();
    assert_empty(&v);
  }

  #[test]
  fn test_resize_shrink() {
    let mut v: SmallVector<u64> = SmallVector::new();
    v.reserve(3);
    make_sequence(&mut v, 1, 3);
    v.resize(1);
    let vec: Vec<u64> = vec![1];
    assert_values_in_order(&v, 1, &vec);
  }

  #[test]
  fn test_truncate() {
    let mut v: SmallVector<u64> = SmallVector::new();
    v.reserve(2);
    make_sequence(&mut v, 1, 2);
    v.truncate(1);

    let vec: Vec<u64> = vec![1];
    assert_values_in_order(&v, 1, &vec);
    v.truncate(1);
    assert_values_in_order(&v, 1, &vec);
    v.truncate(0);
    assert_empty(&v);
  }

  #[test]
  fn test_resize_grow() {
    let mut v: SmallVector<u64> = SmallVector::new();
    v.resize(2);
    assert_eq!(v.size(), 2);
  }

  #[test]
  fn test_overflow() {
    let mut v: SmallVector<u64> = SmallVector::new();
    make_sequence(&mut v, 1, 10);
    assert_eq!(v.size(), 10);

    for i in 0..10 {
      assert_eq!(*v.index(i), (i+1) as u64);
    }
    v.resize(1);
    let vec = vec![1];
    assert_values_in_order(&v, 1, &vec);
  }

  #[test]
  fn test_append_small_vector() {
    let mut v: SmallVector<u64> = SmallVector::new();
    let mut u: SmallVector<u64> = SmallVector::new();
    make_sequence(&mut u, 2, 3);

    v.push_back(1);
    v.append(&mut u);

    let vec: Vec<u64> = vec![1, 2, 3];
    assert_values_in_order(&v, 3, &vec);
  }

  #[test]
  fn test_erase() {
    let mut v: SmallVector<u64> = SmallVector::new();
    make_sequence(&mut v, 1, 3);

    v.erase(0);
    let vec: Vec<u64> = vec![2, 3];
    assert_values_in_order(&v, 2, &vec);
  }

  #[test]
  fn test_insert() {
    let mut v: SmallVector<u64> = SmallVector::new();
    make_sequence(&mut v, 1, 3);

    v.insert(1, 77);
    let vec: Vec<u64> = vec![1, 77, 2, 3];
    assert_values_in_order(&v, 4, &vec);
  }

  #[test]
  fn test_comparison_less_than() {
    let mut v: SmallVector<u64> = SmallVector::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(4);

    let mut u: SmallVector<u64> = SmallVector::new();
    u.push_back(1);
    u.push_back(4);

    assert_eq!(v < u, true);
    assert_eq!(v <= u, true);
    assert_eq!(v > u, false);
    assert_eq!(v >= u, false);

    let mut w: SmallVector<u64> = SmallVector::new();
    w.push_back(1);
    w.push_back(2);
    w.push_back(4);

    assert_eq!(v < w, false);
    assert_eq!(v <= w, true);
    assert_eq!(v > w, false);
    assert_eq!(v >= w, true);
  }

  #[test]
  fn test_direct_vector() {
    let mut v: SmallVector<u64> = SmallVector::new();
    v.reserve(4);
    assert_eq!(v.capacity() <= 4, true);
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_back(4);
    assert_eq!(v.size(), 4);

    assert_eq!(v.index(0), &1);
    assert_eq!(v.index(1), &2);
    assert_eq!(v.index(2), &3);
    assert_eq!(v.index(3), &4);
  }
}