#![allow(dead_code)]

pub struct BackendOptions {
  infra_op_parallelism_threads: i64,
  allowed_devices: Option<i64>
}

impl BackendOptions {
  pub fn new() {}
  pub fn set_platform() {}
  pub fn platform() {}
  pub fn set_infra_op_parallelism_threads() {}
  pub fn infra_op_parallelism_threads() {}
  pub fn set_allowed_devices() {}
}

// Class which encapsulates an Blitz backend. It includes everything necessary
// to compile and execute computations on a particular platform.
pub struct Backend {}

impl Backend {
  pub fn new() {}
  pub fn platform() {}
  pub fn compiler() {}
  pub fn memory_allocator() {}
  pub fn shared_memory_allocator() {}
  pub fn transfer_manager() {}
  pub fn computation_placer() {}
  pub fn device_count() {}
  pub fn default_device_ordinal() {}
  pub fn stream_executors() {}
  pub fn stream_executor() {}
  pub fn default_stream_executor() {}
  pub fn borrow_stream() {}
  pub fn stream_borrower_with_priority() {}
  pub fn device_ordinal_supported() {}
  pub fn device_name() {}
  pub fn devices_equivalent() {}
  pub fn eigen_intra_op_thread_pool_device() {}
  pub fn eigen_intra_op_thread_pool() {}
  pub fn reset_devices() {}
}