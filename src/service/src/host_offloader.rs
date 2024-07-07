#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// This pass does "host memory offloading".
pub struct  HostOffloader {
  host_memory_space_color: i64
}

impl HostOffloader {
  pub fn new(host_memory_space_color: i64) -> Self {
    HostOffloader {
      host_memory_space_color: host_memory_space_color
    }
  }

  pub fn name() -> String {
    "host-offloader".to_string()
  }

  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }

  fn add_all_position_to_be_moved_to_host_memory() {}
  fn handle_input_streaming() {}
  fn handle_streamed_buffer() {}
  fn create_copy_for_input_streaming() {}
  fn try_parameter_streaming() {}
  fn try_output_streaming() {}
  fn handle_move_to_host_custom_call() {}
  fn memory_only_offload_starting_with_dus() {}
  fn memory_only_offload_starting_with_copy() {}
  fn memory_only_offload_insert_copies() {}
  fn dynamify_slice() {}

}