#![allow(dead_code)]

pub struct StreamExecutorInterface {
}

impl StreamExecutorInterface {
  pub fn get_underlying_executor() {}
  pub fn init() {}
  pub fn get_device_description_str() {}
  pub fn device_ordinal() {}
  pub fn get_kernel() {}
  pub fn unload_module() {}
  pub fn load_module() {}
  pub fn launch() {}
  pub fn submit() {}
  pub fn unload_kernel() {}
  pub fn allocate() {}
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

  pub fn poll_for_event_status() {}
  pub fn allocate_stream() {}
  pub fn deallocate_stream() {}
  pub fn create_stream_dependency() {}
  pub fn block_host_until_done() {}
  pub fn get_status() {}
  pub fn enable_peer_access_to() {}
  pub fn can_enable_peer_access_to() {}

  pub fn get_device_load() {}
  pub fn device_memory_usage() {}
}