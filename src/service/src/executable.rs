#![allow(dead_code)]

use std::collections::HashSet;

use common::{blitz_data::ExecutionProfile, shape::Shape, shape_tree::ShapeTree};
use hlo::{hlo_module::HloModule, hlo_module_config::HloModuleConfig};
use stream_executor::{device_memory_allocator::ScopedDeviceMemory, stream::Stream};

use crate::{
  hlo_profile_printer_data::HloProfilePrinterData, hlo_proto::HloProto, maybe_owning_device_memory::MaybeOwningDeviceMemory, service_executable_run_options::ServiceExecutableRunOptions, shaped_buffer::{ScopedShapedBuffer, ShapedBuffer}
};

// ExecutionInput buffers are in one of three states:
//
// 1) Owned by the caller and immutable.
// 2) Donated by the caller but returned on error.
// 3) Donated by the caller and freed on error.
//
// Case (1) buffers are stored as MaybeOwningDeviceMemory(DeviceMemoryBase).
// Case (2) buffers are stored as MaybeOwningDeviceMemory(OwningDeviceMemory),
//   with their indices present in unowned_indices_.
// Case (3) buffers are stored as MaybeOwningDeviceMemory(OwningDeviceMemory),
//   with their indices absent from unowned_indices_.
pub struct ExecutionInput {
  buffers: ShapeTree<MaybeOwningDeviceMemory>,
  unowned_indices: HashSet<Vec<usize>>,
  dynammic_shape: Option<Shape>,
  host_shape: Option<Shape>,
}

impl ExecutionInput {
  pub fn new(mut shape: Shape) -> Self {
    ExecutionInput {
      buffers: ShapeTree::new(&mut shape),
      unowned_indices: HashSet::new(),
      dynammic_shape: None,
      host_shape: None
    }
  }

  pub fn shape(&self) -> &Shape {
    if self.dynammic_shape.is_some() {
      self.dynammic_shape.as_ref().unwrap()
    } else {
      self.buffers.shape()
    }
  }

  pub fn host_shape(&self) -> &Shape {
    if self.host_shape.is_some() {
      self.host_shape.as_ref().unwrap()
    } else {
      self.shape()
    }
  }

  pub fn set_dynamic_shape(&mut self, _dynamc_shape: &Shape) {
    unimplemented!()
  }

  pub fn to_shaped_buffer() {}

  pub fn set_buffer(&mut self, _index: &Vec<usize>, _buffer: MaybeOwningDeviceMemory) {
    unimplemented!()
  }

  pub fn set_unowned_buffer(&mut self, _index: &Vec<usize>, _buffer: MaybeOwningDeviceMemory) {
    unimplemented!()
  }

  pub fn set_unowned_index(&mut self, index: Vec<usize>) {
    self.unowned_indices.insert(index);
  }

  pub fn clear_unowned_index(&mut self, _index: Vec<usize>) {
    unimplemented!()
  }

  pub fn unowned_indices(&self) -> &HashSet<Vec<usize>> {
    &self.unowned_indices
  }

  pub fn buffers(&self) -> &ShapeTree<MaybeOwningDeviceMemory> {
    &self.buffers
  }
  pub fn mutable_buffers(&mut self) -> &mut ShapeTree<MaybeOwningDeviceMemory> {
    &mut self.buffers
  }

  pub fn mutable_buffer(&mut self, _index: usize) -> &mut MaybeOwningDeviceMemory {
    //self.buffers.mutable_element(index)
    unimplemented!()
  }

  pub fn buffer(&self, _index: usize) -> &MaybeOwningDeviceMemory {
    //self.buffers.element(index)
    unimplemented!()
  }
}

// ExecutionOutput encapsulates the output buffers of a execution and the
// leftover buffers to be released by the caller.
pub struct ExecutionOutput {
  result: ScopedShapedBuffer,
  to_be_released: Vec<ScopedDeviceMemory<u8>>,
  aliased_indices: Vec<Vec<usize>>,
  output_shape_table: ScopedDeviceMemory<u8>,
}

impl ExecutionOutput {
  pub fn new() {}

  pub fn add_aliased_index(&mut self, index: Vec<usize>) {
    self.aliased_indices.push(index);
  }

  pub fn add_to_be_released(&mut self, mem: ScopedDeviceMemory<u8>) {
    self.to_be_released.push(mem);
  }

  // Should be called once it is known that the execute operation succeeded,
  // before returning the ExecutionOutput to the caller.
  pub fn commit(&mut self) -> &mut Self {
    self.aliased_indices.clear();
    self
  }

  pub fn result(&self) -> &ScopedShapedBuffer {
    &self.result
  }

  pub fn mutable_result(&mut self) -> &mut ScopedShapedBuffer {
    &mut self.result
  }

  pub fn consume_result(&mut self) -> &ScopedShapedBuffer {
    self.aliased_indices.clear();
    &self.result
  }

  pub fn to_be_released(&self) -> &Vec<ScopedDeviceMemory<u8>> {
    &self.to_be_released
  }

  pub fn consume_to_be_released() {}

  pub fn consume_aliased_indices(&mut self) -> Vec<Vec<usize>> {
    let mut aliased = vec![];
    aliased.clone_from_slice(&self.aliased_indices);
    self.aliased_indices.clear();
    aliased
  }
}

// A given platform's compiler will produce an Executable -- this is a uniform
// interface that is used for launching compiled programs across platforms.
pub struct Executable {
  hlo_module: Option<HloModule>,
  execution_count: i64,
  hlo_profile_printer_data: Option<HloProfilePrinterData>,
  debug_info: String,
  hlo_proto: HloProto,
}

impl Executable {
  pub fn new() {}
  pub fn execute_on_stream() {}

  // Same as ExecuteOnStream(), but runs this executable on multiple
  // streams. arguments[i] contains the arguments to the execution on
  // run_options[i]->stream() and the returned value is at index i of the
  // returned vector.
  pub fn execute_on_streams(
    &self,
    run_options: &Vec<ServiceExecutableRunOptions>,
    arguments: &Vec<Vec<ShapedBuffer>>) -> Result<Vec<ScopedShapedBuffer>, String>
  {
    debug_assert!(run_options.len() == arguments.len());
    let mut return_values: Vec<ScopedShapedBuffer> = vec![];
    if run_options.len() == 1 {

    }
    for i in 0..run_options.len() {
      let rv =
        self.execute_async_on_stream(&run_options[i], &arguments[i]);
      check_error(&rv);
      return_values.push(rv.unwrap());
    }
    for options in run_options {
      debug_assert!(options.stream().is_some());
      let result =
        options.stream().as_ref().unwrap().block_host_until_done();
      check_error(&result);
    }
    Ok(return_values)
  }

  pub fn execute_on_stream_wrapper(
    &self,
    run_options: &ServiceExecutableRunOptions,
    arguments: &Vec<ShapedBuffer>) -> Result<ScopedShapedBuffer, String>
  {
    let result =
      self.execute_async_on_stream_wrapper(run_options, arguments);
    check_error(&result);
    let block_status =
      run_options.stream().as_ref().unwrap().block_host_until_done();
    check_error(&block_status);
    result
  }

  pub fn execute_async_on_stream(
    &self,
    _run_options: &ServiceExecutableRunOptions,
    _arguments: &Vec<ShapedBuffer>) -> Result<ScopedShapedBuffer, String>
  {
    unimplemented!()
  }

  // Convenience wrapper for calling Executable::ExecuteOnStream. Sets up a
  // timer for the execution, sets up HLO profiling if enabled, and fills in the
  // given ExecutionProfile if non-null.
  pub fn execute_async_on_stream_wrapper(
    &self,
    run_options: &ServiceExecutableRunOptions,
    arguments: &Vec<ShapedBuffer>) -> Result<ScopedShapedBuffer, String>
  {
    let mut state =
      execute_wrapper_before_execution(self, run_options);
    let return_value =
      self.execute_async_on_stream(run_options, arguments);
    
    let result = execute_wrapper_after_execution(
      self, &mut state,
      &return_value,
      run_options.stream().as_ref().unwrap());
    check_error(&result);

    return_value
  }

  // Returns whether this executable was compiled with HLO profilings support
  // enabled. If not, the caller should not expect an hlo_execution_profile
  // passed to ExecuteOnStream above to be populated during execution.
  pub fn hlo_profiling_enabled(&self) -> bool {
    self.hlo_profile_printer_data.is_some()
  }

  pub fn module(&self) -> &HloModule {
    assert!(self.hlo_module.is_some());
    self.hlo_module.as_ref().unwrap()
  }

  pub fn shared_module(&self) -> &HloModule {
    self.hlo_module.as_ref().unwrap()
  }

  pub fn has_module(&self) -> bool {
    self.hlo_module.is_some()
  }

  pub fn module_config(&self) -> &HloModuleConfig {
    assert!(self.hlo_module.is_some());
    self.hlo_module.as_ref().unwrap().config()
  }

  pub fn result_shape() {
      
  }

  pub fn size_of_generated_code_in_bytes(&self) -> i64 {
    -1
  }

  // Dumping helpers.
  pub fn set_hlo_proto(&mut self, hlo_proto: HloProto) {
    self.hlo_proto = hlo_proto;
  }

  pub fn dumping_snapshot(&self) -> bool {
    if self.has_module() {
      self.module_config().debug_options().blitz_dump_hlo_snapshots()
    } else {
      false   
    }
  }

  pub fn debug_info(&self) -> &String {
    &self.debug_info
  }

  pub fn set_debug_info(&mut self, debug_info: String) {
    self.debug_info = debug_info;
  }
}

struct ExecuteAsyncOnStreamWrapperState {
  profile: Option<ExecutionProfile>
}

impl ExecuteAsyncOnStreamWrapperState {
  fn new(profile: ExecutionProfile) -> Self {
    ExecuteAsyncOnStreamWrapperState { profile: Some(profile) }
  }
}

fn execute_wrapper_before_execution(
  _executable: &Executable,
  run_options: &ServiceExecutableRunOptions) -> ExecuteAsyncOnStreamWrapperState
{
  let profile =
    run_options.run_options().execution_profile().clone();
  let state =
    ExecuteAsyncOnStreamWrapperState::new(profile);
  println!("enqueueing executable on stream...");
  state
}

fn execute_wrapper_after_execution(
  executable: &Executable,
  state: &mut ExecuteAsyncOnStreamWrapperState,
  return_status: &Result<ScopedShapedBuffer, String>,
  stream: &Stream) -> Result<(), String>
{
  if return_status.is_err() {
    if state.profile.is_some() {
      let status = stream.block_host_until_done();
      if status.is_err() {
        panic!("Failed to block_host_until_done: {:?}", status.err().unwrap());
      }
    }
    return Err("".to_string());
  }

  if state.profile.is_some() {
    // We block instead of using an async callback because reading the timer
    // value may call back into the driver on GPU, which is not allowed.
    let result = stream.block_host_until_done();
    check_error(&result);

    let executable_size_in_bytes = executable.size_of_generated_code_in_bytes();
    if state.profile.as_ref().unwrap().compute_time_ns() == 0 {
      let value = state.profile.as_ref().unwrap().compute_and_transfer_time_ns();
      state.profile.as_mut().unwrap().set_compute_time_ns(value);
    }
    if executable_size_in_bytes != 0 {
      state.profile.as_mut().unwrap().
        set_executable_size_in_bytes(executable_size_in_bytes);
    }
  }

  Ok(())
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}