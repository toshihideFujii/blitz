#![allow(dead_code)]
use crate::accounting_allocator::AccountingAllocator;

pub struct Zone {
  allocator: AccountingAllocator,
  name: String,
  supports_compression: bool,
  sealed: bool,
}

impl Zone {
  pub fn new(
    allocator: AccountingAllocator,
    name: String,
    supports_compression: bool) -> Self
  {
    Zone {
      allocator: allocator,
      name: name,
      supports_compression: supports_compression,
      sealed: false
    }
  }

  pub fn supports_compression(&self) -> bool {
    self.supports_compression
  }

  pub fn allocate() {}
  pub fn delete() {}
  pub fn allocate_array() {}
  pub fn allocate_vector() {}
  pub fn new_vector() {}
  pub fn clone_vector() {}
  pub fn delete_array() {}

  pub fn seal(&mut self) {
    self.sealed = true;
  }

  pub fn reset() {}
  pub fn segment_bytes_allocated() {}
  pub fn name() {}
  pub fn allocation_size() {}
  pub fn allocation_size_for_tracing() {}
  pub fn freed_size_for_tracing() {}
  pub fn allocator() {}
}