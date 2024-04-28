#![allow(dead_code)]

use crate::{platform::Platform, stream_executor_internal::StreamExecutorInterface};

pub struct StreamExecutor {
  platform: Platform,
  implementation: StreamExecutorInterface,
  device_ordinal: i64,
}

impl StreamExecutor {
  pub fn new() {}

  pub fn platform_specific_handle() {}
  pub fn init() {}

  pub fn platform(&self) -> &Platform {
    &self.platform
  }

  pub fn get_kernel() {}
  pub fn unload_kernel() {}
  pub fn load_module() {}
  pub fn unload_module() {}
  pub fn create_or_share_constant() {}
  pub fn allocate_array() {}
  pub fn allocate_owned_array() {}
  pub fn allocate_scalar() {}
  pub fn allocate_owned_scalar() {}
  pub fn get_untyped_symbol() {}
  pub fn deallocate() {}
  pub fn unified_memory_allocate() {}
  pub fn unified_memory_deallocate() {}
  pub fn collective_memory_allocate() {}
  pub fn collective_memory_deallocate() {}
  pub fn host_memory_allocate() {}
  pub fn host_memory_deallocate() {}
  pub fn synchronize_all_activity() {}
  pub fn synchronous_mem_zero() {}
  pub fn synchronous_memory_h2d() {}
  pub fn synchronous_memcpy() {}
  pub fn mem_zero() {}
  pub fn memset32() {}
  pub fn enable_peer_access_to() {}
  pub fn can_enable_peer_access_to() {}
  pub fn get_device_description() {}
  pub fn get_device_load() {}
  pub fn device_memory_usage() {}

  pub fn device_ordinal(&self) -> i64 {
    self.device_ordinal
  }

  pub fn implementation() {}
  pub fn create_typed_kernel() {}
  pub fn launch() {}
  pub fn submit() {}
  pub fn as_fft() {}
  pub fn as_dnn() {}
  pub fn as_blas() {}
  pub fn get_allocate_stats() {}
  pub fn clear_allocate_stats() {}
  pub fn get_allocator() {}
  pub fn find_allocated_stream() {}
}