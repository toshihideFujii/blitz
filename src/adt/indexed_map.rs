#![allow(dead_code)]

use std::ops::Index;
use super::small_vector::SmallVector;

struct IndexedMap<T> {
  storage: SmallVector<T>,
  val: T,
  to_index: usize
}

impl<T> IndexedMap<T> {
  pub fn new(val: T) -> Self {
    IndexedMap { storage: SmallVector::new(), val: val, to_index: 0 }
  }

  pub fn reserve(&mut self, s: usize) {
    self.storage.reserve(s)
  }

  pub fn resize(&mut self, s: usize) where T: Default {
    self.storage.resize(s)
  }

  pub fn clear(&mut self) {
    self.storage.clear()
  }

  pub fn grow(&mut self, n: usize) where T: Default {
    let new_size = n + 1;
    if new_size > self.storage.size() {
      self.resize(new_size);
    }
  }

  pub fn in_bounds(&self, n: usize) -> bool {
    n < self.storage.size()
  }

  pub fn size(&self) -> usize {
    self.storage.size()
  }
}


impl<T> Index<usize> for IndexedMap<T> {
  type Output = T;
  fn index(&self, n: usize) -> &Self::Output {
    debug_assert!(n < self.storage.size(), "index out of bounds!");
    &self.storage[n]
  }
}