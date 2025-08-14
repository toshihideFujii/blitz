#![allow(dead_code)]

use common::{blitz_data::ExecutionOptions, shape::{ProgramShape, Shape}};
use hlo::{hlo_module::HloModule, hlo_module_config::HloModuleConfig};
use crate::compiler::AotCompilationOptions;

// Update entry computation's computation layout by translating each shape
// with shape_representation_fn(shape). It can be used for example to add
// tiling info for each shape.
pub fn update_entry_computation_layout<F>(
  _module: &HloModule,
  _shape_representation_fn: &F,
  _empty_tiles_only: bool) where F: Fn(&Shape) -> Shape
{
  unimplemented!()
}

// Creates an HloModuleConfig for a given program shape and arguments.
// If execution_options does not set num_replicas, default_num_replicas is used.
// num_threads is optional; if not given, intra_op_parallelism_threads not set.
// aot_options is optional; if not given a default is used.
pub fn create_module_config(
  program_shape: &ProgramShape,
  _argument_shapes: &Vec<Shape>,
  _execution_options: &ExecutionOptions,
  _default_num_replicas: i64,
  _num_threads: Option<i64>,
  _aot_options: Option<&AotCompilationOptions>) -> Result<HloModuleConfig, String>
{
  let _config = HloModuleConfig::new(program_shape);
  //let computation_layout = config
  unimplemented!()    
}