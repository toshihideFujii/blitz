#![allow(dead_code)]

use common::{
  blitz_data::GlobalDataHandle,
  executable_run_options::ExecutableRunOptions,
  literal::Literal, shape::Shape
};

use service::{
  backend::Backend, blitz_computation::BlitzComputation, compiler::AotCompilationResult, executable::Executable, executable_build_options::ExecutableBuildOptions, hlo_proto::HloSnapshot, local_service::LocalService, service_executable_run_options::ServiceExecutableRunOptions, shaped_buffer::{ScopedShapedBuffer, ShapedBuffer}, stream_pool::StreamPool
};

use stream_executor::{
  device_memory_allocator::DeviceMemoryAllocator,
  platform::Platform, stream::Stream
};

//use crate::{/*blitz_computation::BlitzComputation,*/ executable_build_options::ExecutableBuildOptions};

pub struct LocalExecutable {
  // Compiled computation.
  executable: Executable,
  // Execution backend.
  backend: Option<Backend>,
  // Options used to build the executable.
  build_options: ExecutableBuildOptions
}

impl LocalExecutable {
  // Low-level constructor; LocalClient::Compile() is the usual way to create
  // executables.
  pub fn new(
    executable: Executable,
    backend: Backend,
    build_options: ExecutableBuildOptions) -> Self
  {
    let instance = LocalExecutable {
      executable: executable,
      backend: Some(backend),
      build_options: build_options
    };
    assert!(instance.build_options.device_ordinal() >= 0);
    instance
  }

  // Run the compiled computation with the given arguments and options and
  // return the result.
  pub fn run(
    &self,
    arguments: &Vec<&ShapedBuffer>,
    _run_options: &ExecutableRunOptions) -> Result<ScopedShapedBuffer, String>
  {
    let mut argument_shapes = vec![];
    for arg in arguments {
      argument_shapes.push(arg.on_device_shape());
    }

    let _async_callback =
      |options: &ExecutableRunOptions| -> Result<ScopedShapedBuffer, String>
    {
      self.run_async(arguments, options)
    };
    unimplemented!()
    //self.async_call_and_block_host_until_done::<ScopedShapedBuffer>
      //(&argument_shapes, run_options, &Box::new(async_callback))
  }

  // Similar to Run(), but need not block the host waiting for the computation
  // to complete before returning.
  pub fn run_async(
    &self,
    arguments: &Vec<&ShapedBuffer>,
    run_options: &ExecutableRunOptions) -> Result<ScopedShapedBuffer, String>
  {
    let mut argument_shapes = vec![];
    for arg in arguments {
      argument_shapes.push(arg.on_device_shape());
    }
    let options_and_stream =
      self.run_helper(&argument_shapes, run_options);
    if options_and_stream.is_err() {
      return Err(options_and_stream.err().unwrap());
    }

    let stream = run_options.stream();
    let mut snapshot: Option<HloSnapshot> = None;
    if self.executable.dumping_snapshot() {
      snapshot = Some(dump_arguments(
        self.backend.as_ref().unwrap(),
        &self.executable, arguments,
        stream.as_ref().unwrap().as_ref()));
    }

    let outputs =
      self.executable.execute_async_on_stream_wrapper(
        &options_and_stream.unwrap().0, arguments);
    if outputs.is_err() {
      return Err(outputs.err().unwrap());
    }

    // Transfer the outputs and save the snapshot to disk.
    if snapshot.is_some() {
      dump_outputs_and_save_snapshot(
        self.backend.as_ref().unwrap(),
        outputs.as_ref().unwrap(),
        snapshot.as_ref().unwrap(),
        stream.as_ref().unwrap().as_ref());
    }
    outputs
  }

  // Return the options used to build the executable.
  pub fn build_options(&self) -> &ExecutableBuildOptions {
    &self.build_options
  }

  // Return the built executable.
  pub fn executable(&self) -> &Executable {
    &self.executable
  }

  // Validates that the given arguments and options satisfy various constraints
  // of the computation.
  //
  // The given ExecutableRunOptions override any values from TF_BLITZ_FLAGS
  // environment variable.
  fn validate_execution_options(
    &self,
    _run_options: &ExecutableRunOptions,
    _backend: &Backend) -> Result<(), String>
  {
    unimplemented!()    
  }

  // Returns a literal containing the contents of the given ShapedBuffer.
  fn literal_from_shaped_buffer(&self) {}

  fn run_helper(
    &self,
    _argument_shapes: &Vec<&Shape>,
    _run_options: &ExecutableRunOptions
  ) -> Result<(ServiceExecutableRunOptions, StreamPool), String>
  {
    unimplemented!()  
  }

  // The ordinal of the device which this executable was compiled for. The
  // executable can run on all equivalent devices (as determined by
  // Backend::devices_equivalent).
  fn build_device_ordinal(&self) -> i64 {
    self.build_options.device_ordinal()
  }

  fn async_call_and_block_host_until_done<T>(
    &self,
    _argument_shapes: &Vec<&Shape>,
    _run_options: &ExecutableRunOptions,
    _async_callback: &Box<dyn Fn(&ExecutableRunOptions)->Result<T, String>>
  ) -> Result<T, String>
  {
    unimplemented!()    
  }
}

// An Blitz Client specialization for use when the client and service run in
// the same process.
pub struct LocalClient {
  local_service: LocalService,
}

impl LocalClient {
  pub fn new() {}

  // Build and return LocalExecutable objects (one per partition, as specified
  // by the build options). The executable is compiled using the given
  // BlitzComputation, argument layouts and options.
  //
  // The given ExecutableBuildOptions overrides any values from BLITZ_FLAGS
  // environment variable.
  pub fn compile(
    &self,
    computation: &BlitzComputation,
    argument_layouts: &Vec<Shape>,
    options: &ExecutableBuildOptions) -> Result<Vec<LocalExecutable>, String>
  {
    let updated_options =
      update_build_options(options, self.default_device_ordinal());
    if updated_options.is_err() {
      return Err(updated_options.err().unwrap());
    }

    let _executables =
      self.local_service.compile_executables(
        computation, argument_layouts, &updated_options.unwrap());

    // TODO
    unimplemented!()
  }

  // Same as Compile() above, but return AotCompilationResult objects (instead
  // of LocalExecutable objects), which can be persisted to later load
  // LocalExecutable(s) using the Load() method below.
  pub fn compile_ahead_of_time(
    &self,
    computation: &BlitzComputation,
    argument_layouts: &Vec<Shape>,
    options: &ExecutableBuildOptions) -> Result<Vec<AotCompilationResult>, String>
  {
    let updated_options =
      update_build_options(options, self.default_device_ordinal());
    if updated_options.is_err() {
      return Err(updated_options.err().unwrap());
    }
    self.local_service.compile_aot_results(
      computation, argument_layouts, &updated_options.unwrap())
  }

  // Return a LocalExecutable object loaded from a serialized AotCompilationResult.
  pub fn load(
    &self,
    _serialized_aot_result: &String, 
    _options: &ExecutableBuildOptions) -> Result<LocalExecutable, String>
  {
    unimplemented!()    
  }

  // Copy the literal data to the device with the given ordinal and return as a
  // ScopedShapedBuffer. If non-null the given memory allocator is used for
  // device memory allocation. If null, the default memory allocator for the
  // device is used.
  pub fn literal_to_shaped_buffer(
    &self,
    _literal: &Literal,
    _device_ordinal: i64,
    _allocator: &DeviceMemoryAllocator) -> Result<ScopedShapedBuffer, String>
  {
    unimplemented!()    
  }

  // Transfer the BorrowingLiteral to the device with the given ordinal.
  pub fn transfer_to_local_server(
    &self,
    _literal: &Literal,
    _device_ordinal: i64) -> Result<GlobalDataHandle, String>
  {
    unimplemented!()    
  }

  // Copy the data from the device contained in the given ShapedBuffer and
  // return as a Literal.
  pub fn shaped_buffer_to_literal(
    &self, _shaped_buffer: &ShapedBuffer) -> Result<Literal, String>
  {
    unimplemented!()
  }

  // Converts a GlobalDataHandle into a pointer to a ShapedBuffer that's valid
  // as long as the handle is valid.
  pub fn global_data_to_shaped_buffer(
    &self,
    data: &GlobalDataHandle,
    replica_number: i64) -> Result<ShapedBuffer, String>
  {
    self.local_service.global_data_to_shaped_buffer(data, replica_number)
  }

  // Transfer the given literal to the infeed queue of the given device.
  pub fn transfer_to_infeed_local(
    &self, literal: &Literal, device_ordinal: i64) -> Result<(), String>
  {
    let executor =
      self.backend().stream_executor(device_ordinal);
    if executor.is_none() {
      return Err("Executor is not exist.".to_string());
    }
    self.backend().transfer_manager().transfer_literal_to_infeed(
      executor.unwrap().as_ref(), literal)
  }

  // Transfer and return a value from the outfeed of the given device. The
  // shape of the object to transfer is determined by `literal`'s shape.
  pub fn transfer_from_outfeed_local(
    &self, device_ordinal: i64, literal: &Literal) -> Result<(), String>
  {
    let executor =
      self.backend().stream_executor(device_ordinal);
    if executor.is_none() {
      return Err("Executor is not exist.".to_string());
    }

    self.backend().transfer_manager().transfer_literal_from_outfeed(
      executor.unwrap().as_ref(), literal)
  }

  // Returns the device ordinal that corresponds to the given replica number.
  //
  // This returns an error if there is not a one-to-one correspondence of
  // replicas to device ordinals, but is useful as a short term mechanism for
  // the "easy" case where a single replica is a single device.
  pub fn replica_number_to_device_ordinal(
    &self, replica_number: i64) -> Result<i64, String>
  {
    self.local_service.replica_number_to_device_ordinal(replica_number)
  }

  // Returns the platform that the underlying service targets.
  pub fn platform(&self) -> &dyn Platform {
    self.local_service.backend().platform()
  }

  // Returns the number of devices on the system of the service platform
  // type. Not all devices may be supported by the service (see
  // device_ordinal_supported method).
  pub fn device_count(&self) -> usize {
    self.local_service.backend().device_count()
  }

  // Returns the default device ordinal that the service will run computations
  // on if no device ordinal is specified in execute options.
  pub fn default_device_ordinal(&self) -> i64 {
    self.local_service.backend().default_device_ordinal()
  }

  // Returns whether the device with the given ordinal can be used by the
  // service to execute computations. Not all devices of a particular platform
  // may be usable by the service (eg, a GPU with insufficient CUDA compute
  // capability).
  pub fn device_ordinal_supported(&self, device_ordinal: i64) -> bool {
    self.local_service.backend().device_ordinal_supported(device_ordinal)
  }

  // Returns the backend used to execute computations.
  pub fn backend(&self) -> &Backend {
    self.local_service.backend()
  }

  pub fn local_service(&self) -> &LocalService {
    &self.local_service
  }
}

pub fn update_build_options(
  _options: &ExecutableBuildOptions,
  _default_device_ordinal: i64) -> Result<ExecutableBuildOptions, String>
{
  unimplemented!()    
}

pub fn dump_arguments(
  _backend: &Backend,
  _executable: &Executable,
  _arguments: &Vec<&ShapedBuffer>,
  _stream: &dyn Stream) -> HloSnapshot
{
  unimplemented!()    
}

pub fn dump_outputs_and_save_snapshot(
  _backend: &Backend,
  _outputs: &ScopedShapedBuffer, // check
  _snapshot: &HloSnapshot,
  _stream: &dyn Stream)
{
    
}