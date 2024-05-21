#![allow(dead_code)]

struct Concept {}

struct Model {}

struct Object {}

impl Object {
  pub fn new() {}
  pub fn get_or_create() {}
}

// CUDA-specific implementation of the platform-agnostic
// StreamExecutorInterface.
pub struct GpuExecutor {
  device_optional: i64,
  cc_major: i64,
  cc_minor: i64,
  version: i64,
  blitz_state: Object
}

impl GpuExecutor {
  pub fn new() {}
  pub fn init() {}
  pub fn device_ordinal() {}
  pub fn get_kernel() {}
  pub fn unload_kernel() {}
  pub fn load_module() {}
  pub fn unload_module() {}
  pub fn create_or_share_constant() {}
  pub fn launch() {}
  pub fn submit() {}
  pub fn calculate_occupancy() {}
  pub fn allocate() {}
  pub fn deallocate() {}

  pub fn unified_memory_allocate() {}
  pub fn unified_memory_deallocate() {}
  pub fn collective_memory_allocate() {}
  pub fn collective_memory_deallocate() {}

  pub fn host_memory_allocate() {}
  pub fn host_memory_deallocate() {}
  pub fn host_memory_register() {}
  pub fn host_memory_unregister() {}
  pub fn synchronize_all_activity() {}

  pub fn synchronous_mem_zero() {}
  pub fn synchronous_mem_set() {}
  pub fn synchronous_memcpy() {}
  pub fn synchronous_memcpy_device_to_device() {}

  pub fn mem_zero() {}
  pub fn memset() {}
  pub fn memset32() {}
  pub fn memcpy() {}
  pub fn memcpy_device_to_device() {}
  pub fn host_callback() {}
  pub fn allocate_stream() {}
  pub fn deallocate_stream() {}
  pub fn create_stream_dependency() {}
  pub fn allocate_event() {}
  pub fn deallocate_event() {}
  pub fn record_event() {}
  pub fn wait_for_event() {}
  pub fn wait_for_event_on_external_stream() {}
  pub fn poll_for_event_status() {}
  pub fn block_host_until_done() {}
  pub fn enable_peer_access_to() {}
  pub fn can_enable_peer_access_to() {}
  pub fn device_memory_usage() {}
  pub fn get_symbol() {}
  pub fn create_device_description() {}
  pub fn create_blas() {}
  pub fn create_fft() {}
  pub fn create_dnn() {}

  pub fn platform_specific_context() {}
  pub fn gpu_context() {}
  pub fn get_or_create_xla_state() {}
  pub fn find_allocated_stream() {}
}