#![allow(dead_code)]

use stream_executor::platform::Platform;

use crate::backend::Backend;

// A base class for running an HloModule.
pub struct HloRunner {
  backend: Backend,
}

impl HloRunner {
  pub fn new(_platform: &dyn Platform, _intra_op_parallelism_threads: i64) {}

  pub fn transfer_literal_to_device() {}
  pub fn transfer_literals_to_device() {}
  pub fn transfer_literal_from_device() {}

  pub fn execute() {}
  pub fn execute_with_buffer_assignment() {}
  pub fn execute_with_executable() {}
  pub fn execute_with_device_buffers() {}
  pub fn execute_with_moved_device_buffers() {}
  pub fn execute_with_moved_device_buffers_and_buffer_assignment() {}
  pub fn create_executable() {}
  pub fn create_executable_with_buffer_assignment() {}

  pub fn execute_replicated() {}

  pub fn backend() {}
  pub fn name() {}
  pub fn device_shape_representation_fn() {}
}