#![allow(dead_code)]

use crate::stream_executor::StreamExecutor;

pub struct DeviceMemoryAllocator {}

impl DeviceMemoryAllocator {
  pub fn new() {}
  pub fn allocate() {}
  pub fn deallocate() {}
  pub fn platform() {}
  pub fn allows_asynchronous_deallocation() {}
  pub fn get_stream() {}
}

// Default memory allocator for a platform which uses
// StreamExecutor::allocate/deallocate.
pub struct StreamExecutorMemoryAllocator {
  stram_executors: Vec<StreamExecutor>
}

impl StreamExecutorMemoryAllocator {
  pub fn new() {}
  pub fn allocate() {}
  pub fn deallocate() {}

  pub fn allows_asynchronous_deallocation(&self) -> bool {
    false
  }
  
  pub fn get_stream() {}
  pub fn get_stream_executor() {}
}