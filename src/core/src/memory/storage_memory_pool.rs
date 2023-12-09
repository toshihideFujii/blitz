#![allow(dead_code)]

use common::utils::memory_mode::MemoryMode;

use super::memory_pool::MemoryPool;

pub struct StorageMemoryPool {
  inner: MemoryPool,
  memory_mode: MemoryMode
}

impl StorageMemoryPool {
  pub fn pool_name(&self) -> String {
    match self.memory_mode {
      MemoryMode::OnHeap => String::from("on-heap storage"),
      MemoryMode::OffHeap => String::from("off-heap storage"),
    }
  }

  pub fn memory_used(&self) -> usize {
    *self.inner.memory_used.lock().unwrap()
  }
}