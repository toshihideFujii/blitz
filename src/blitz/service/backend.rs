#![allow(dead_code)]

// Options to configure the backend when it is created.
struct BackendOptions {}
impl BackendOptions {
  pub fn set_platform() {}
  pub fn platform() {}

  pub fn set_intra_op_parallelism_threads() {}
  pub fn intra_op_parallelism_threads() {}

  pub fn set_allowed_devices() {}
  pub fn allowed_devices() {}
}

// Class with encapsulates an BLITZ backend.
// It includes everything necessary to compile and execute computations
// on a particular platform.
struct Backend {}
impl Backend {
  pub fn create_backend() {}
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
  pub fn stream_borrower() {}
  pub fn device_ordinal_supported() {}
  pub fn device_name() {}
  pub fn device_equivalent() {}
  pub fn reset_devices() {}
}