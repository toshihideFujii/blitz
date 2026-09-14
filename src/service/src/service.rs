#![allow(dead_code)]

use std::collections::HashSet;

use common::{
  blitz_data::{
    ChannelHandle, ChannelType,
    ComputationGraphStatsRequest, ComputationStatsResponse,
    DeviceHandle, ExecutionHandle, ExecutionOptions,
    ExecutionProfile, GlobalDataHandle
  }, computation_placer::DeviceAssignment, executable_run_options::ExecutableRunOptions, layout::Layout, layout_util::LayoutUtil, literal::Literal, shape::{ProgramShape, Shape, ShapeEqual}, shape_util::ShapeUtil
};

use hlo::{evaluator::hlo_evaluator::HloEvaluator, hlo_module::HloModule,
  hlo_module_config::HloModuleConfig, hlo_module_group::HloModuleGroup, hlo_proto::HloSnapshot};
use stream_executor::{platform::{Platform, StreamPriority},
  stream::Stream, stream_executor::StreamExecutor};

use crate::{
  allocation_tracker::AllocationTracker, backend::Backend,
  blitz_computation::BlitzComputation, channel_tracker::ChannelTracker,
  compilation_cache::CompilationCache, compiler::{AotCompilationOptions,
  AotCompilationResult, CompileOptions},
  //dump::dump_hlo_snapshot_if_enabled,
  dynamic_dimension_inference::DynamicDimensionInference,
  dynamic_padder::DynamicPadder, executable::Executable,
  execution_tracker::ExecutionTracker, hlo_module_util::{self, create_module_config},
  service_executable_run_options::ServiceExecutableRunOptions,
  shaped_buffer::ShapedBuffer, transfer_manager::TransferManager
};

// Options to configure the service when it is created.
#[derive(Debug, Clone)]
pub struct ServiceOptions {
  platform: Option<Platform>,
  number_of_replicas: i64,
  intra_op_parallelism_threads: i64,
  allowed_devices: Option<HashSet<i64>>
}

impl ServiceOptions {
  pub fn default() -> Self {
    ServiceOptions {
      platform: None,
      number_of_replicas: 1,
      intra_op_parallelism_threads: -1,
      allowed_devices: None
    }
  }

  // Set the platform backing the service, or nullptr for the default platform.
  pub fn set_platform(&mut self, platform: Platform) -> &mut Self {
    self.platform = Some(platform);
    self
  }

  pub fn platform(&self) -> &Option<Platform> {
    &self.platform
  }

  // Set the default number of replicas to use when compiling replicated
  // programs.
  pub fn set_number_of_replicas(&mut self, number_of_replicas: i64) -> &mut Self {
    self.number_of_replicas = number_of_replicas;
    self
  }

  pub fn number_of_replicas(&self) -> i64 {
    self.number_of_replicas
  }

  // Sets the thread pool size for parallel execution of an individual operator.
  pub fn set_intra_op_parallelism_threads(&mut self, num_threads: i64) -> &mut Self {
    self.intra_op_parallelism_threads = num_threads;
    self
  }

  pub fn intra_op_parallelism_threads(&self) -> i64 {
    self.intra_op_parallelism_threads
  }

  // Sets the allowed_devices set for selectively constructing stream executors
  // on the platform.
  pub fn set_allowed_devices(&mut self, allowed_devices: HashSet<i64>) -> &mut Self {
    self.allowed_devices = Some(allowed_devices);
    self
  }

  pub fn allowed_devices(&self) -> &Option<HashSet<i64>> {
    &self.allowed_devices
  }
}

// The Blitz service object, which is the same across all platforms. It maintains
// the service state of computations and allocations, and delegates
// target-specific requests to the target-specific infrastructure
// (target-specific compiler, StreamExecutor).
//#[derive(Debug)]
pub struct Service<'backend> {
  options: ServiceOptions,
  compilation_cache: CompilationCache,
  channel_tracker: ChannelTracker,
  allocation_tracker: AllocationTracker<'backend>,
  execution_tracker: ExecutionTracker,
  // Backend to compile and execute computations on.
  execute_backend: Option<&'backend Backend>,
}

impl<'backend> Service<'backend> {
  pub fn new(
    options: ServiceOptions,
    execute_backend: Option<&'backend Backend>) -> Self
  {
    if execute_backend.is_some() {
      let service = Service {
        options: options,
        compilation_cache: CompilationCache::default(),
        channel_tracker: ChannelTracker::default(), 
        allocation_tracker: AllocationTracker::new(execute_backend.unwrap()),
        execution_tracker: ExecutionTracker::default(),
        execute_backend: execute_backend
      };
      if execute_backend.unwrap().device_count() > 0 {
        debug_assert!(service.execute_backend.unwrap().device_count() >=
          service.options.number_of_replicas() as usize,
          "Requested replicas than there are devices.");
      }
      println!("Blitz service initialized for platform {:?} (this does not generatee
        that Blitz will be used). Devices:",
        service.execute_backend.unwrap().platform().name());
      let stream_executors =
        service.execute_backend.unwrap().stream_executors();
      for i in 0..execute_backend.unwrap().device_count() {
        let executor = &stream_executors[i];
        let description = executor.get_device_description();
        println!(" StreamExecutor device ({:?}): {:?}, {:?}",
          i, description.name(), description.platform_version());
      }

      return service;
    }
    unimplemented!()
  }

  // Unregisters a previously-allocated global handle.
  // If the handle given is not currently allocated, a NOT_FOUND status is
  // returned.
  pub fn unregister(&mut self, data: &GlobalDataHandle) -> Result<(), String> {
    self.allocation_tracker.unregister(data)
  }

  // Deconstructs a tuple. Returns a newly created GlobalDataHandle for each
  // element in the tuple.
  pub fn deconstruct_tuple(&self, data: &GlobalData) -> Result<Vec<GlobalData<'_, '_>>, String> {
    let elements =
      self.allocation_tracker.deconstruct_tuple(data.handle().clone());
    check_error(&elements);

    let mut out = vec![];
    for element in &elements.unwrap() {
      let global_data =
        GlobalData::new(element.clone(), self);
      out.push(global_data);
    }
    Ok(out)
  }

  // Compiles a computation into an executable. The request contains the whole
  // computation graph. Returns the handle to the executable.
  pub fn compile(
    &mut self,
    computation: &BlitzComputation,
    argument_shapes: &Vec<Shape>,
    execution_options: ExecutionOptions) -> Result<ExecutionHandle, String>
  {
    println!("running compile request");

    if !computation.has_host_program_shape() {
      let err_msg = "program shape may not be empty".to_string();
      return Err(err_msg);
    }

    if execution_options.device_handles_size() > 1 {
      let err_msg =
        "The compile request does not support multiple device handles.".to_string();
      return Err(err_msg);
    }

    let program_shape = ProgramShape::default();
    let module_config = self.create_module_config(
      &program_shape, argument_shapes, &execution_options, None);
    check_error(&module_config);

    println!("Compile created BlitzModuleConfig computation layout: {:?}",
      module_config.as_ref().unwrap().entry_computation_layout().to_string());

    let executable = self.build_executables(
        vec![computation.module()],
        vec![module_config.as_ref().unwrap()],
        self.execute_backend.as_ref().unwrap(),
        &vec![vec![self.execute_backend.unwrap().default_stream_executor().clone()]],
        &CompileOptions::default(),
        false);
    check_error(&executable);

    println!("successfully completed 'compile' request");
    //Ok(self.compilation_cache.insert(executable.unwrap()))
    unimplemented!()
  }

  // Executes an executable with the provided global data passes as immutable
  // arguments. The request contains the handle to the executable. Returns
  // global data output and execution timing.
  pub fn execute(
    &mut self,
    handle: &ExecutionHandle,
    arguments: &Vec<GlobalData>,
    _execution_profile: Option<ExecutionProfile>) -> Result<GlobalData<'_, '_>, String>
  {
    println!("running execute request");
    let executable = self.compilation_cache.lookup(handle);
    check_error(&executable);

    let replicas =
      self.replicas(&self.execute_backend.unwrap(),
      Some(&self.single_computation_device_handle()));
    check_error(&replicas);

    let replicated_arguments_wrapper =
      self.resolve_and_validate_arguments(arguments, &replicas.unwrap());
    check_error(&replicated_arguments_wrapper);
    let replicated_arguments = replicated_arguments_wrapper.unwrap();

    // Check that the replicated_arguments has the same shape and layout as the
    // module config used when creating the executable.
    let num_module_args = executable.as_ref().unwrap()
      .module_config().entry_computation_layout().parameter_count();
    if num_module_args != arguments.len() {
      let mut err_msg = "The executable expects ".to_string();
      err_msg.push_str(&num_module_args.to_string());
      err_msg.push_str(" arguments, but sees ");
      err_msg.push_str(&arguments.len().to_string());
      return Err(err_msg);
    }
    for i in 0..num_module_args {
      let shape_module = executable.as_ref().unwrap()
        .module_config().entry_computation_layout().parameter_shape(i);

      let shape_arg = replicated_arguments.first().unwrap()[i].on_device_shape();
      if !ShapeEqual::new().equal(shape_module, shape_arg) {
        let mut err_msg = "The executable expects the ".to_string();
        err_msg.push_str(&i.to_string());
        err_msg.push_str("th argument in shape ");
        err_msg.push_str(&ShapeUtil::human_string_with_layout(shape_module));
        err_msg.push_str(" but sees ");
        err_msg.push_str(&ShapeUtil::human_string_with_layout(shape_arg));
        return Err(err_msg);
      }
    }
    let stream =
      self.execute_backend.as_ref().unwrap().borrow_stream_by_executor(
        self.execute_backend.as_ref().unwrap().default_stream_executor(),
        StreamPriority::Default);

    let mut snapshot = HloSnapshot::default();
    if executable.as_ref().unwrap().dumping_snapshot() {
      //snapshot.set_hlo(); // TODO
      snapshot.set_execution_platform(
        self.execute_backend.as_ref().unwrap().platform().name());
      let result = record_arguments(
        replicated_arguments.first().unwrap(),stream.as_ref().unwrap(),
          self.execute_backend.unwrap().transfer_manager(),
          &mut snapshot);
      check_error(&result);
    }
/*
    let mut result_tag = "result of ".to_string();
    result_tag.push_str(&executable.as_ref().unwrap().module().name());
    let output = self.execute_and_register_result(
      executable.as_ref().unwrap(),
      &replicated_arguments,
      self.execute_backend.unwrap(),
      &self.single_computation_device_handle(),
      &result_tag,
      execution_profile.as_ref().unwrap());
    check_error(&output);

    if executable.as_ref().unwrap().dumping_snapshot() {
      let result_buffer =
        self.allocation_tracker.resolve_for_replica(
          output.as_ref().unwrap(), 0);
      check_error(&result_buffer);
      let result = record_result(
        &result_buffer.as_ref().unwrap(),
        stream.as_ref().unwrap(),
        self.execute_backend.as_ref().unwrap().transfer_manager(),
        &snapshot);
      check_error(&result);
      dump_hlo_snapshot_if_enabled(executable.as_ref().unwrap().module(), &snapshot);
    }
    
    let data =
      GlobalData::new(output.unwrap(), self);
    Ok(data)
*/
    unimplemented!()
  }

  // Executes one or more computations in parallel with the provided global data
  // passed as immutable arguments. Returns global data output for each
  // computation.
  pub fn execute_graph_parallel(
    &self,
    _computations: &Vec<BlitzComputationInstance>) -> Result<Vec<GlobalData<'_, '_>>, String>
  {
    unimplemented!()
  }

  // Requests one or more device handles from the target.
  //
  // When N device handles are requested and the number of replicas is R, at
  // least N * R devices must be available. The devices are assigned based on
  // the device ordinals such that the first R available devices are assigned to
  // the first set of replicas, and the next R devices to the second set of
  // replicas, etc. Each returned device handle represents the device with the
  // replica id 0.
  pub fn get_device_handles(&self,
    device_count: i64) -> Result<Vec<DeviceHandle>, String>
  {
    let available_device_count =
      self.execute_backend.as_ref().unwrap().device_count();
    let replica_count = self.options.number_of_replicas();
    if replica_count <= 0 {
      let err_msg = "Replica count must be a positive integer".to_string();
      return Err(err_msg);
    }
    if (available_device_count as i64) < device_count * replica_count {
      let mut err_msg = "Requested logical device count (".to_string();
      err_msg.push_str(&device_count.to_string());
      err_msg.push_str(") with replica count (");
      err_msg.push_str(&replica_count.to_string());
      err_msg.push_str(") exceeds the number of available physical devices
        on the target (");
      err_msg.push_str(&available_device_count.to_string());
      err_msg.push_str(")");
      return Err(err_msg);
    }
    let mut out = vec![];
    for i in 0..device_count {
      let mut device_handle = DeviceHandle::default();
      device_handle.set_handle(i);
      device_handle.set_device_count(device_count);
      out.push(device_handle);
    }
    Ok(out)
  }

  // Requests that global data be transferred to the client in literal form.
  pub fn transfer_to_client(
    &self,
    data: &GlobalData,
    shape_with_layout: Option<Shape>) -> Result<Literal, String>
  {
    let shaped_buffer_wrapper =
      self.allocation_tracker.resolve_for_replica(data.handle(), 0);
    check_error(&shaped_buffer_wrapper);

    let shaped_buffer = shaped_buffer_wrapper.unwrap();
    #[allow(unused_assignments)]
    let mut return_shape = Shape::default();
    if shape_with_layout.is_some() {
      return_shape = Shape::new_from(shape_with_layout.as_ref().unwrap());
      if !LayoutUtil::has_layout(&return_shape) {
        let err_msg =
          "shape_wiith_layout must have layout is present.".to_string();
        return Err(err_msg);
      }
      if return_shape.has_layout() &&
        return_shape.layout().as_ref().unwrap().element_size_in_bits() != 0
      {
        let err_msg = "shape_with_layout cannot have layout's
          element_size_in_bits field set".to_string();
        return Err(err_msg); 
      }
    } else {
      return_shape = Shape::new_from(shaped_buffer.on_device_shape());
      if return_shape.has_layout() &&
        return_shape.layout().as_ref().unwrap().element_size_in_bits() != 0
      {
        // Literals do not support element_size_in_bits
        return_shape.mutable_layout().as_mut().unwrap().set_element_size_in_bits(0);
      }
    }

    let stream_wrapper =
      self.execute_backend.as_ref().unwrap().borrow_stream(
        shaped_buffer.physical_device_ordinal(), StreamPriority::Default);
    check_error(&stream_wrapper);

    let stream = stream_wrapper.unwrap();
    let result_literal_wrapper = 
      self.execute_backend.unwrap().transfer_manager().transfer_literal_from_device(
        &stream, &shaped_buffer, None);
    check_error(&result_literal_wrapper);

    let result_literal = result_literal_wrapper.unwrap();
    if LayoutUtil::layouts_in_shapes_equal(
      &return_shape, result_literal.shape())
    {
      return Ok(result_literal);
    }
    Ok(result_literal.relayout_with_shape(&return_shape))
  }

  // Transfers data from a literal provided by the client, into device memory.
  pub fn transfer_to_server(
    &mut self,
    literal: &Literal,
    device_handle: Option<&DeviceHandle>) -> Result<GlobalData<'_, '_>, String>
  {
    let shape = literal.shape();
    #[allow(unused_assignments)]
    let mut replicas = vec![];
    if device_handle.is_some() {
      replicas = self.replicas(
        self.execute_backend.unwrap(), device_handle).unwrap();
    } else {
      replicas = self.replicas(
        self.execute_backend.unwrap(),
        Some(&self.single_computation_device_handle())).unwrap();
    }

    // Allocate memory in each replica and transfer the data to all replicas.
    let replicated_buffers = vec![];
    for executor in &replicas {
      let device_shape_representation =
        |shape: &Shape| -> Shape
      {
        self.execute_backend.unwrap().compiler()
          .default_device_shape_representation(shape) 
      };

      let shaped_buffer_wrapper =
        self.execute_backend.unwrap().transfer_manager()
          .allocate_scoped_shaped_buffer(
            shape,
            self.execute_backend.unwrap().memory_allocator(),
            executor.device_ordinal(),
            Some(&device_shape_representation));
      check_error(&shaped_buffer_wrapper);

      let stream_wrapper =
        self.execute_backend.unwrap().borrow_stream_by_executor(
          executor, StreamPriority::Default);
      check_error(&stream_wrapper);

      let shaped_buffer = shaped_buffer_wrapper.unwrap();
      let stream = stream_wrapper.unwrap();
      let result = self.execute_backend.unwrap()
        .transfer_manager().transfer_literal_to_device(
          &stream,
          literal,
          shaped_buffer.base(),
          None);
      check_error(&result);
    }

    let mut tag = "transfer_to_server literal of shape".to_string();
    tag.push_str(&ShapeUtil::human_string(shape));
    let out_wrapper = self.allocation_tracker
      .register_replicated_buffers(replicated_buffers, &tag);
    check_error(&out_wrapper);
    
    let out = out_wrapper.unwrap();
    Ok(GlobalData::new(out, self))
  }

  // Transfers data from a literal provided by the client, into the Infeed
  // buffer of the device.
  pub fn transfer_to_infeed(
    &self,
    literal: &Literal,
    replica_id: i64,
    device_handle: Option<&DeviceHandle>) -> Result<(), String>
  {
    let replica_count = self.options.number_of_replicas();
    if replica_id < 0 || replica_id >= replica_count {
      let mut err_msg = "The replica_id=".to_string();
      err_msg.push_str(&replica_id.to_string());
      err_msg.push_str(" on transer_to_infeed_request not in range [0,");
      err_msg.push_str(" replica_count=");
      err_msg.push_str(&replica_count.to_string());
      err_msg.push_str(").");
      return Err(err_msg);
    }

    let mut replicas_wrapper = self.replicas(
      self.execute_backend.unwrap(),
      Some(&self.single_computation_device_handle()));
    check_error(&replicas_wrapper);

    let mut replicas = replicas_wrapper.unwrap();
    let mut executor = &replicas[replica_id as usize];
    if device_handle.is_some() {
      replicas_wrapper = self.replicas(
        self.execute_backend.unwrap(), device_handle);
      check_error(&replicas_wrapper);

      replicas = replicas_wrapper.unwrap();
      executor = &replicas[replica_id as usize];
    }

    self.execute_backend.unwrap().transfer_manager()
      .transfer_literal_to_infeed(executor, literal)
  }

  // Transfers data from the Outfeed othe device to the literal provided by the
  // client.
  pub fn transfer_from_outfeed(
    &self,
    shape_with_layout: &Shape,
    replica_id: i64,
    device_handle: Option<&DeviceHandle>) -> Result<Literal, String>
  {
    let replica_count = self.options.number_of_replicas();
    if replica_id < 0 || replica_id >= replica_count {
      let mut err_msg = "The replica_id=".to_string();
      err_msg.push_str(&replica_id.to_string());
      err_msg.push_str(" on transfer_from_outfeed_request not in range [0, ");
      err_msg.push_str(&replica_count.to_string());
      err_msg.push_str(")");
      return Err(err_msg);
    }

    let mut replicas_wrapper = self.replicas(
      self.execute_backend.unwrap(),
      Some(&self.single_computation_device_handle()));
    check_error(&replicas_wrapper);

    let mut replicas = replicas_wrapper.unwrap();
    let mut executor = &replicas[replica_id as usize];
    if device_handle.is_some() {
      replicas_wrapper = self.replicas(
        self.execute_backend.unwrap(), device_handle);
      check_error(&replicas_wrapper);

      replicas = replicas_wrapper.unwrap();
      executor = &replicas[replica_id as usize];
    }

    let literal: Literal = Literal::create_from_shape(shape_with_layout);
    let result = self.execute_backend.unwrap()
      .transfer_manager().transfer_literal_from_outfeed(executor, &literal);
    check_error(&result);
    
    Ok(literal)
  }

  // Resets devices, clearing all existing state on all the devices associated
  // with this service (including memory allocated on the devices).
  //
  // ResetDevice may only be called where no previous Execution state on the
  // device is used by the next Execution.
  //
  // ResetDevice should be called before an Execution that expect the device to
  // be in the reset state. For example, if the prior Execution modifies device
  // state (e.g., architectural state) that the next Execution depends on.
  pub fn reset_device(&self) -> Result<(), String> {
    debug_assert!(self.execute_backend.is_some());
    self.execute_backend.unwrap().reset_devices()
  }

  pub fn compute_constant_graph(
    &self,
    computation: &BlitzComputation,
    output_layout: Option<&Layout>) -> Result<Literal, String>
  {
    if computation.has_host_program_shape() {
      let err_msg = "program shape may not be empty".to_string();
      return Err(err_msg);
    }
    if computation.host_program_shape().parameters_size() != 0 {
      let err_msg =
        "constant computation may not depend on any parameters".to_string();
      return Err(err_msg);
    }
    let program_shape = computation.host_program_shape();
    let result = ShapeUtil::validate_shape(program_shape.result());
    check_error(&result);

    if output_layout.is_some() {
      let result = LayoutUtil::validate_layout_for_shape(
        output_layout.as_ref().unwrap(), program_shape.result());
      check_error(&result);
    }

    let config = HloModuleConfig::new(program_shape);
    let module = HloModule::new(computation.name().clone(), config);
    let dynamic_padder = DynamicPadder::default();
    let result = dynamic_padder.run(); // TODO
    check_error(&result);

    let dynamic_dimension_inference =
      DynamicDimensionInference::run(&module); // TODO
    check_error(&dynamic_dimension_inference);

    let mut evaluator: HloEvaluator = HloEvaluator::default();
    evaluator.set_dynamic_dimension_inference(); // TODO
    let result_literal_wrapper =
      evaluator.evaluate_module(&module);
    check_error(&result_literal_wrapper);

    let mut result_literal = result_literal_wrapper.unwrap();
    if output_layout.is_some() {
      result_literal = result_literal.relayout(
        output_layout.unwrap(), &vec![]);
    }
    Ok(result_literal)
  }

  // Returns the shape (with layout) of an array associated with a given data
  // handle.
  pub fn get_shape(&self, data: &GlobalData) -> Result<&Shape, String> {
    let buffer =
      self.allocation_tracker.resolve_for_replica(data.handle(), 0);
    check_error(&buffer);

    //Ok(buffer.as_ref().unwrap().on_device_shape())
    unimplemented!()
  }

  // Retrieves the statistics of a computation.
  pub fn get_computation_graph_stats(
    &self,
    _arg: &ComputationGraphStatsRequest,
    _result: &ComputationStatsResponse) -> Result<(), String>
  {
    unimplemented!()
  }

  // Creates a unique channel handle that can be used for Send/Recv
  // instructions.
  pub fn create_channel_handle(
    &mut self, t: ChannelType) -> Result<ChannelHandle, String>
  {
    self.channel_tracker.new_channel(t.clone())
  }

  pub fn backend(&self) -> &Backend {
    &self.execute_backend.unwrap()
  }

  pub fn mutable_backend(&mut self) -> &mut Backend {
    //self.execute_backend.as_mut().unwrap()
    unimplemented!()
  }

  // Create a Hlo module config for the given program shape and arguments.
  // aot_options is optional; if not given a default is used.
  pub fn create_module_config(
    &self,
    program_shape: &ProgramShape,
    argument_shapes: &Vec<Shape>,
    execution_options: &ExecutionOptions,
    aot_options: Option<&AotCompilationOptions>) -> Result<HloModuleConfig, String>
  {
    let default_num_replicas = self.options.number_of_replicas();
    let num_threads = 0;
    if self.execute_backend.is_some() { // TODO
      // TODO
    }

    create_module_config(program_shape, argument_shapes, execution_options,
      default_num_replicas, Some(num_threads), aot_options)
  }

  // Convenience function which checks whether the given client_shape
  // (presumably passed by the client to set the result layout) is valid for the
  // given computation result shape.
  pub fn validate_result_shape(
    client_shape: &Shape, result_shape: &Shape) -> Result<(), String>
  {
    if !ShapeUtil::compatible(client_shape, result_shape) {
      let mut err_msg =
        "shape used to set computation result layout ".to_string();
      err_msg.push_str(&ShapeUtil::human_string_with_layout(client_shape));
      err_msg.push_str(" is not compatible with result shape ");
      err_msg.push_str(&ShapeUtil::human_string(result_shape));
      return Err(err_msg);
    }
    Ok(())
  }

  // Builds an Executable for the given parameters.
  //
  // If device_allocator is not null, the compiler may use it to allocate temp
  // buffers, which the compiler is responsible for freeing.  The allocator
  // given here need not match the allocator used when running the executable.
  pub fn build_executables(
    &self,
    modules: Vec<&HloModule>,
    module_configs: Vec<&HloModuleConfig>,
    backend: &Backend,
    executors: &Vec<Vec<StreamExecutor>>,
    options: &CompileOptions,
    run_backend_only: bool) -> Result<Vec<Executable>, String>
  {
    println!("build_executable on service");

    println!("computations:");
    for module in &modules {
      println!("{:?}", module.name());
    }
    debug_assert_eq!(modules.len(), module_configs.len());
    let mut module_g = HloModuleGroup::new(modules[0].name());
    let shape_representation_fn = |_shape: &Shape| -> Shape {
      unimplemented!();
    };
    for module in modules {
      //module.set_layout_canonicalization_callback();
      hlo_module_util::update_entry_computation_layout(
        module,
        &shape_representation_fn, //Compiler::default_device_shape_representation,
        true);
      module_g.push_back(module.clone());
    }

    let mut executables: Vec<Executable> = vec![];
    if !run_backend_only {
      executables = backend.compiler().compile(
        &module_g, executors, options).unwrap();
    } else {
      let modules = module_g.consume_modules();
      for module in modules {
        let executable =
          backend.compiler().run_backend(module, &executors[0][0], options);
        check_error(&executable);
        executables.push(executable.unwrap());
      }
    }
    Ok(executables)
  }

  // Returns the stream executors assigned to the replicas represented by the
  // given device handle. Each device_handle is a virtual replicated device that
  // represents a set of physical devices for the replicas.
  pub fn replicas(
    &self,
    _backend: &Backend,
    _device_handle: Option<&DeviceHandle>) -> Result<Vec<StreamExecutor>, String>
  {
    unimplemented!()    
  }

  // Returns the device handle that represents the replicated device for a
  // single computation that is not model-parallelized.
  pub fn single_computation_device_handle(&self) -> DeviceHandle {
    let mut device_handle = DeviceHandle::default();
    device_handle.set_handle(0);
    device_handle.set_device_count(1);
    device_handle
  }

  // Same as BuildExecutable() above, but builds a list of
  // AotCompilationResult(s), which can be persisted to later load Executable
  // objects.
  pub fn build_aot_results(
    &self,
    modules: &Vec<HloModule>,
    module_configs: &Vec<HloModuleConfig>,
    backend: &Backend,
    executors: &Vec<Vec<StreamExecutor>>,
    _options: &CompileOptions,
    run_backend_only: bool) -> Result<Vec<AotCompilationResult>, String>
  {
    println!("build_aot_results on service");
    println!("computations");
    for module in  modules {
      println!("{:?}", module.name());
    }

    debug_assert_eq!(modules.len(), module_configs.len());
    let mut module_group = HloModuleGroup::new(modules[0].name());
    for module in modules {
      module_group.push_back(module.clone());
    }

    let mut aot_options =
      AotCompilationOptions::new(backend.compiler().platform_id());
    aot_options.set_executor(executors[0][0].clone());
    //aot_options.set_device_allocator(options.device_allocator.unwrap());
    aot_options.set_run_backend_only(run_backend_only);

    let aot_results =
      backend.compiler().compile_ahead_of_time(&module_group, &aot_options);
    check_error(&aot_results);

    aot_results
  }

  // Resolves the given argument handles in the allocation tracker and returns
  // the corresponding allocations for every replica. The function also verifies
  // that each allocation matches the execution platform and device ordinal of
  // the corresponding replica.
  pub fn resolve_and_validate_arguments(
    &self,
    _arguments: &Vec<GlobalData>,
    _stream_executors: &Vec<StreamExecutor>) -> Result<Vec<Vec<ShapedBuffer>>, String>
  {
    unimplemented!()    
  }

  // Runs the given executable with the given arguments and register the result
  // in the allocation tracker. The handle of the result from the tracker is
  // returned. If the parameter "profile" is not null, it points to an
  // ExecutionProfile object which will be filled in with profile data.
  fn execute_and_register_result(
    &mut self,
    executable: &Executable,
    arguments: &Vec<Vec<ShapedBuffer>>,
    backend: &Backend,
    device_handle: &DeviceHandle,
    result_tag: &String,
    profile: &ExecutionProfile) -> Result<GlobalDataHandle, String>
  {
    let mut streams = vec![];
    let replicas_wrapper =
      self.replicas(backend, Some(device_handle));
    check_error(&replicas_wrapper);
    let replicas = replicas_wrapper.unwrap();

    for executor in &replicas {
      let stream =
        backend.borrow_stream_by_executor(executor, StreamPriority::Default);
      check_error(&stream);
      streams.push(stream.unwrap());
    }

    let mut device_assignment = DeviceAssignment::new(
      self.options.number_of_replicas() as usize, 1);
    for replica in 0..replicas.len() {
      device_assignment.set_data(
        replica, 0, replicas[replica].device_ordinal());
    }

    if executable.module_config().has_static_device_assignment() {
      // TODO
    }

    // Set up run options.
    let mut run_options = vec![];
    for stream in &streams {
      let mut options = ExecutableRunOptions::default();
      options.set_stream(stream.clone());
      options.set_device_ordinal(stream.parent().device_ordinal());
      options.set_local_device_count(backend.device_count() as i64);
      //options.set_allocator(backend.memory_allocator().clone());
      //options.set_device_assignment();
      options.set_execution_profile(profile.clone());

      let se_run_options =
        ServiceExecutableRunOptions::new(options);
      run_options.push(se_run_options);
    }

    if self.options.number_of_replicas() == 1 {
      //let result =
        //executable.execute_on_stream_wrapper(run_options, &arguments[0]);
    }

    let mut replicated_arguments = vec![];
    for arg in arguments {
      let mut values = vec![];
      values.clone_from(arg);
      replicated_arguments.push(values);
    }

    let results =
      executable.execute_on_streams(&run_options, &replicated_arguments);
    check_error(&results);

    debug_assert!(!results.as_ref().unwrap().is_empty());
    self.allocation_tracker.register_replicated_buffers(
      results.unwrap(), result_tag)
  }

  // Prepare the executors for executing parallel.
  fn get_executors(
    &self,
    execution_options: &ExecutionOptions,
    requests_size: i64,
    request_index: i64) -> Result<Vec<StreamExecutor>, String>
  {
    if execution_options.device_handles().is_empty() {
      let err_msg = "device handles must be given to execute
        parallel computations".to_string();
      return Err(err_msg);
    }
    if requests_size > 1 && execution_options.device_handles_size() > 1 {
      let mut err_msg = "Parallel requests with multiple device
        handles s not supported. Found ".to_string();
      err_msg.push_str(&requests_size.to_string());
      err_msg.push_str("  parallel requests, with request ");
      err_msg.push_str(&request_index.to_string());
      err_msg.push_str(" containing ");
      err_msg.push_str(&execution_options.device_handles_size().to_string());
      err_msg.push_str(" device handles.");
      return Err(err_msg);
    }
    let mut executors = vec![];
    for device_handle in execution_options.device_handles() {
      let replicas =
        self.replicas(self.execute_backend.as_ref().unwrap(),
        Some(device_handle));
      check_error(&replicas);
      let executor = replicas.unwrap()[0].clone();
      executors.push(executor);
    }
    Ok(executors)
  }

  // Prepare the arguments for executing parallel.
  fn get_arguments(
    &self,
    execution_options: &ExecutionOptions,
    arguments: &Vec<GlobalData>) -> Result<Vec<Vec<ShapedBuffer>>, String>
  {
    let replicas = self.replicas(
      self.execute_backend.as_ref().unwrap(),
      Some(&execution_options.device_handles()[0]));
    check_error(&replicas);

    let replicated_arguments =
      self.resolve_and_validate_arguments(arguments,
        replicas.as_ref().unwrap());
    check_error(&replicated_arguments);

    replicated_arguments
  }
}

// A GlobalData object represents a globally-accessible allocation of
// data in the associated Blitz service.
pub struct GlobalData<'backend, 'service> {
  handle: GlobalDataHandle,
  parent: &'service Service<'backend>
}

impl<'backend, 'service> GlobalData<'backend, 'service> {
  pub fn new(
    handle: GlobalDataHandle,
    parent: &'service Service<'backend>) -> Self
  {
    GlobalData {
      handle: handle,
      parent: parent
    }
  }

  pub fn handle(&self) -> &GlobalDataHandle {
    &self.handle
  }
}

// A struct to represent a computation instance to be executed.
// * If execution_options.device_handles is not empty, the computation is
//   executed on the devices associated with the handles by partitioning the
//   computation based on the attached sharding attributes. Otherwise, a
//   device is chosen by the service.
pub struct BlitzComputationInstance<'backend, 'service> {
  computation: BlitzComputation,
  arguments: Vec<GlobalData<'backend, 'service>>,
  execution_options: ExecutionOptions,
  execution_profile: ExecutionProfile,
}

impl<'backend, 'service> BlitzComputationInstance<'backend, 'service> {
  pub fn new(
    computation: BlitzComputation,
    arguments: Vec<GlobalData<'backend, 'service>>,
    execution_options: ExecutionOptions,
    execution_profile: ExecutionProfile) -> Self
  {
    BlitzComputationInstance {
      computation: computation,
      arguments: arguments,
      execution_options: execution_options,
      execution_profile: execution_profile
    }    
  }
}

// Records the arguments used to invoke a computation in an HloSnapshot proto.
fn record_arguments(
  arguments: &Vec<ShapedBuffer>,
  stream: &Stream,
  transfer_manager: &TransferManager,
  module: &mut HloSnapshot) -> Result<(), String>
{
  module.clear_arguments();
  for arg in arguments {
    let literal =
      transfer_manager.transfer_literal_from_device( // TODO
        stream, arg, None);
    check_error(&literal);
    module.add_arguments::<i64>(literal.unwrap());
  }
  Ok(())
}

// Records the result of a computation in a HloSnapshot proto.
fn record_result(
  _result: &ShapedBuffer,
  _stream: &Stream,
  _transfer_manager: &TransferManager,
  _module: &HloSnapshot) -> Result<(), String>
{
  unimplemented!()
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}