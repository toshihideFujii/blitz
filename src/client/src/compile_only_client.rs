#![allow(dead_code)]

use common::{blitz_data::ExecutionOptions, shape::{ProgramShape, Shape}};
use service::{
  blitz_computation::BlitzComputation,
  compile_only_service::{AotBlitzComputationInstance, CompileOnlyService},
  compiler::{AotCompilationMetadata, AotCompilationOptions, AotCompilationResult}
};

// An Blitz Client specialization for doing ahead-of-time compilation.  This does
// not require (or attempt to instantiate) an execution-capable backend for the
// relevant platform.
pub struct CompileOnlyClient {
  compiler_service: CompileOnlyService
}

impl CompileOnlyClient {
  pub fn new() {}

  // Compiles a list of blitz computations for ahead-of-time execution.
  // This is intended for use in static compilation. The |options|
  // parameter describes the target for which the compiler should emit
  // code. |metadata|, if provided, is populated during compilation.
  pub fn compile_ahead_of_time(
    &self,
    computations: &Vec<AotComputationInstanceForClient>,
    options: &AotCompilationOptions,
    metadata: &Option<AotCompilationMetadata>) -> Result<Vec<AotCompilationResult>, String>
  {
    let mut service_instances: Vec<AotBlitzComputationInstance> = vec![];
    for instance in computations {
      let mut argument_layouts = vec![];
      argument_layouts.clone_from_slice(&instance.argument_layouts);
      let service_instance =
        AotBlitzComputationInstance::new(
          instance.computation.proto().clone(), 
          argument_layouts,
          instance.result_layout.clone());
      service_instances.push(service_instance);
    }
    
    self.compiler_service.compile_ahead_of_time(
      service_instances, options, metadata)
  }

  // Create a Hlo module config for the given program shape and arguments.
  // execution_options is optional; if not given a default is used.
  pub fn create_module_config(
    &self,
    _program_shape: &ProgramShape,
    _argument_shapes: &Vec<Shape>,
    _execution_options: &ExecutionOptions) -> Result<(), String> // TODO: how to get ?
  {
    unimplemented!()
  }

  // Returns the size of a pointer in bytes for a given triple.
  pub fn pointer_size_for_triple(&self, _triple: String) -> i64 {
    unimplemented!()
  }
}

// A description of an blitz computation to compile using CompileAheadOfTime.
pub struct AotComputationInstanceForClient {
  computation: BlitzComputation,
  argument_layouts: Vec<Shape>,
  result_layout: Shape
}