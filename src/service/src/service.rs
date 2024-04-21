#![allow(dead_code)]

pub struct ServiceOptions {
  number_of_replicas: i64,
  intra_op_parallelism_threads: i64
}

impl ServiceOptions {
  pub fn new() {}
  pub fn set_platform() {}
  pub fn platform() {}
  pub fn set_number_of_replicas() {}
  pub fn number_of_replicas() {}
  pub fn set_intra_op_parallelism_threads() {}
  pub fn intra_op_parallelism_threads() {}
  pub fn set_allowed_devices() {}
  pub fn allowed_devices() {}
}

pub struct Service {}

impl Service {
  pub fn new() {}
  pub fn unregister() {}
  pub fn deconstruct_tuple() {}
  pub fn compile() {}
  pub fn execute() {}
  pub fn execute_graph_parallel() {}
  pub fn get_device_handles() {}
  pub fn transfer_to_client() {}
  pub fn transfer_to_server() {}
  pub fn transfer_to_infeed() {}
  pub fn transfer_from_outfeed() {}
  pub fn reset_device() {}
  pub fn compute_constant_graph() {}
  pub fn get_shape() {}
  pub fn get_computation_graph_stats() {}
  pub fn create_channel_handle() {}
  pub fn backend() {}
  pub fn mutable_backend() {}
  pub fn create_module_config() {}
  pub fn validate_result_shape() {}
}