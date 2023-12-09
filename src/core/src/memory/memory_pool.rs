#![allow(dead_code)]

use std::sync::Mutex;

pub struct MemoryPool {
  pool_size: Mutex<usize>,
  pub memory_used: Mutex<usize>,
}

impl MemoryPool {
  pub fn new() -> Self {
    MemoryPool {
      pool_size: Mutex::new(0),
      memory_used: Mutex::new(0),
    }
  }

  pub fn pool_size(&self) -> usize {
    *self.pool_size.lock().unwrap()
  }

  pub fn memory_free(&self) -> usize {
    *self.pool_size.lock().unwrap() - *self.memory_used.lock().unwrap()
  }

  pub fn increment_pool_size(&mut self, delta: usize) {
    *self.pool_size.lock().unwrap() += delta;
  }

  pub fn decrement_pool_size(&mut self, delta: usize) {
    *self.pool_size.lock().unwrap() -= delta;
  }
}