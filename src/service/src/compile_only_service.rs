#![allow(dead_code)]

use common::{blitz_data::ExecutionOptions, shape::{ProgramShape, Shape}, shape_util::ShapeUtil};
use hlo::{hlo_module::HloModule, hlo_module_group::HloModuleGroup, hlo_proto::HloModuleProto};
use stream_executor::platform::Platform;

use crate::{compiler::{AotCompilationMetadata, AotCompilationOptions,
  AotCompilationResult, Compiler}, hlo_module_util::create_module_config,
  service::{Service, ServiceOptions}};

// An Blitz Service specialization for ahead-of-time compilation.  This only
// instantiates a Compiler object for the relevant platform; it does not
// instantiate or require an execution backend.
pub struct CompileOnlyService<'backend> {
  pub service: Service<'backend>,
  // The compiler for the target platform.  This is included in place of
  // the Service::execute_backend_'s compiler, since execute_backend_ is a
  // nullptr in CompileOnlyService.
  compiler: Compiler
}

impl<'backend> CompileOnlyService<'backend> {
  pub fn new(options: ServiceOptions, compiler: Compiler) -> Self {
    CompileOnlyService {
      service: Service::new(options, None),
      compiler: compiler
    }
  }

  // Factory for creating a CompileOnlyService. The parameter platform is the
  // platform that the service should target. If platform is null then the
  // default platform is used.
  pub fn new_service_by_platform(platform: Platform) -> Self {
    let mut default_options = ServiceOptions::default();
    default_options.set_platform(platform);
    CompileOnlyService::new_service_by_options(default_options)
  }

  pub fn new_service_by_options(options: ServiceOptions) -> Self {
    let platform = options.platform();
    if platform.is_none() {
      // TODO
    }
    let compiler =
      Compiler::get_for_platform(platform.as_ref().unwrap());
     check_error(&compiler);
    CompileOnlyService::new(options.clone(), compiler.unwrap().clone())
  }

  // Compiles a list of blitz computations for ahead-of-time execution.  This is
  // intended for use in static compilation.  See
  // |CompileOnlyClient::CompileAheadOfTime| for additional details.
  pub fn compile_ahead_of_time(
    &self,
    mut computations: Vec<AotBlitzComputationInstance>,
    options: &AotCompilationOptions,
    metadata: &Option<AotCompilationMetadata>) -> Result<Vec<AotCompilationResult>, String>
  {
    let mut hlo_modules: Vec<HloModule> = vec![];
    let debug_options = options.debug_options();

    let mut execution_options = ExecutionOptions::default();
    execution_options.set_debug_options(debug_options.clone());

    // Capture replica_count, num_cores, and device_assignment in ExecutionOptions
    // to later save in a proto dump.
    if options.replica_count() > 0 {
      execution_options.set_num_replicas(options.replica_count());
      if options.has_static_device_assignment() {
        debug_assert_eq!(options.replica_count(),
          options.static_device_assignment().replica_count());
      }
    }
    if options.num_cores() > 0 {
      execution_options.set_num_partitions(options.num_cores());
      if options.has_static_device_assignment() {
        debug_assert_eq!(options.num_cores(),
          options.static_device_assignment().computation_count());
      }
    }
    if options.has_static_device_assignment() {
      // TODO
    }
    execution_options.set_use_auto_spmd_partitioning(
      options.use_spmd_partitioning());
    execution_options.set_use_shardy_partitioner(
      options.use_shardy_partitioner());
    execution_options.set_use_auto_spmd_partitioning(
      options.use_auto_spmd_partitioning());
    for _t in &options.auto_spmd_partitioning_mesh_shape() {
      // TODO
    }
    for _t in &options.auto_spmd_partitioning_mesh_ids() {
      // TODO
    }
    execution_options.set_deduplicate_hlo(options.deduplicate_hlo());
    for instance in &mut computations {
      let result = instance.computation.has_host_program_shape();
      debug_assert!(result);

      let mut update_shape_with_empty_tiles =
        |_subshape: &mut Shape, _index: &Vec<i64>|
      {
        unimplemented!()
      };
      let mut result_layout = instance.mutable_result_layout();
      ShapeUtil::for_each_mutable_subshape(
        &mut result_layout, &mut update_shape_with_empty_tiles);

      // TODO

      for _shape in &instance.argument_layouts {
        //ShapeUtil::for_each_mutable_subshape(shape, &mut update_shape_with_empty_tiles);
      }

      let program_shape = ProgramShape::default();
      // TODO
      let module_config = create_module_config(
        &program_shape, &instance.argument_layouts, &execution_options,
        0, None, Some(options));
      check_error(&module_config);

      let hlo_module = HloModule::create_from_proto(
        &instance.computation, module_config.as_ref().unwrap(),
        true, None);

      // TODO: dump_hlo_module_if_enabled()
      hlo_modules.push(hlo_module);
    }

    self.compiler.compile_ahead_of_time_by_metadata(
      &HloModuleGroup::new(hlo_modules[0].name()),
      options, metadata.as_ref().unwrap())
  }
}

// A description of a blitz computation to compile using CompileAheadOfTime.
pub struct AotBlitzComputationInstance {
  computation: HloModuleProto,
  argument_layouts: Vec<Shape>,
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
      argument_layouts: argument_layouts,
      result_layout: result_layout
    }    
  }

  pub fn mutable_result_layout(&mut self) -> &mut Shape {
    &mut self.result_layout
  }
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}