#![allow(dead_code)]

use common::{blitz_data::DebugOptions, literal::Literal, shape::Shape};
use hlo::hlo_module::HloModule;

// The options used to configure an execute_replicated() call.
struct ReplicatedExecuteOptions<T> where T: Clone + Default + PartialEq + 'static {
  numm_replicas: i64,
  arguments: Vec<Literal<T>>,
  infeed_values: Vec<Literal<T>>,
  infeed_steps: i64,
  outfeed_shape: Shape,
  outfeed_values: Vec<Literal<T>>,
  run_hlo_passes: bool,
  use_threads: bool,
}

// A base class for running an HloModule.
pub struct HloRunnerInterface {}

impl HloRunnerInterface {
  pub fn new() {}
  pub fn create_module_from_string() {}
  pub fn read_module_from_binary_proto_file() {}

  // Read the hlo text dump file in HloModule::to_string format, creates and
  // returns the HloModule.
  pub fn read_module_from_hlo_text_file(
    &mut self, _filename: &String, _debug_options: &DebugOptions) -> Result<HloModule, String>
  {
    unimplemented!()
  }

  pub fn create_executable() {}
  pub fn create_executable_with_buffer_assignment() {}
  pub fn execute() {}
  pub fn execute_with_buffer_assignment() {}
  pub fn execute_with_executable() {}
  pub fn execute_replicated() {}
  pub fn name() {}
}