#![allow(dead_code)]

struct SmallVector {
  size_: usize,
  capacity_: usize
}

impl SmallVector {
  pub fn size(&self) -> usize {
    self.size_
  }

  pub fn capacity(&self) -> usize {
    self.capacity_
  }

  pub fn empty(&self) -> bool {
    self.size_ == 0
  }

  pub fn set_size(&mut self, n: usize) {
    debug_assert!(n <= self.capacity());
    self.size_ = n
  }
}