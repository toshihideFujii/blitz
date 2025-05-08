#![allow(dead_code)]

use std::collections::HashSet;

use common::{shape::Shape, shape_tree::ShapeTree};
use hlo::{hlo_module::HloModule, hlo_module_config::HloModuleConfig};
use stream_executor::device_memory_allocator::ScopedDeviceMemory;

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
  pub fn execute_async_on_stream() {}

  // Convenience wrapper for calling Executable::ExecuteOnStream. Sets up a
  // timer for the execution, sets up HLO profiling if enabled, and fills in the
  // given ExecutionProfile if non-null.
  pub fn execute_async_on_stream_wrapper(
    &self,
    _run_options: &ServiceExecutableRunOptions,
    _arguments: &Vec<&ShapedBuffer>) -> Result<ScopedShapedBuffer, String>
  {
    unimplemented!()
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

  pub fn size_of_generated_code_in_bytes() {
      
  }

  // Dumping helpers.
  pub fn set_hlo_proto(&mut self, hlo_proto: HloProto) {
    self.hlo_proto = hlo_proto;
  }

  pub fn dumping_snapshot(&self) -> bool {
    unimplemented!()
  }

  pub fn debug_info(&self) -> &String {
    &self.debug_info
  }

  pub fn set_debug_info(&mut self, debug_info: String) {
    self.debug_info = debug_info;
  }
}