#![allow(dead_code)]

use common::shape::Shape;

use crate::{compiler::{AotCompilationMetadata, AotCompilationOptions, AotCompilationResult, Compiler}, hlo_proto::HloModuleProto};

// An Blitz Service specialization for ahead-of-time compilation.  This only
// instantiates a Compiler object for the relevant platform; it does not
// instantiate or require an execution backend.
pub struct CompileOnlyService {
  // The compiler for the target platform.  This is included in place of
  // the Service::execute_backend_'s compiler, since execute_backend_ is a
  // nullptr in CompileOnlyService.
  compiler: Compiler
}

impl CompileOnlyService {
  pub fn new_service() {}

  // Compiles a list of blitz computations for ahead-of-time execution.  This is
  // intended for use in static compilation.  See
  // |CompileOnlyClient::CompileAheadOfTime| for additional details.
  pub fn compile_ahead_of_time(
    &self,
    _computations: Vec<AotBlitzComputationInstance>,
    _options: &AotCompilationOptions,
    _metadata: &Option<AotCompilationMetadata>) -> Result<Vec<AotCompilationResult>, String>
  {
    unimplemented!()
  }

  fn compile_only_service() {}
}

// A description of a blitz computation to compile using CompileAheadOfTime.
pub struct AotBlitzComputationInstance {
  computation: HloModuleProto,
  argument_lsyouts: Vec<Shape>,
  result_layout: Shape
}

impl AotBlitzComputationInstance {
  pub fn new(
    computation: HloModuleProto,
    argument_layouts: Vec<Shape>,
    result_layout: Shape) -> Self
  {
    AotBlitzComputationInstance {
      computation: computation,
      argument_lsyouts: argument_layouts,
      result_layout: result_layout
    }    
  }
}