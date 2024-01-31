#![allow(dead_code)]

pub trait ServiceInterface {
  fn transfer_to_client();
  fn transfer_to_server();
  fn transfer_to_infeed();
  fn transfer_from_outfeed();
  fn reset_device();
  fn compile();
  fn execute();
  fn execute_graph_parallel();
  fn deconstruct_tuple();
  fn get_computation_graph_stats();
  fn get_shape();
  fn create_channel_handle();
  fn get_device_handles();
  fn compute_constant_graph();
  fn unregister();
}
