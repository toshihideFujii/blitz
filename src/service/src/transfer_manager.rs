#![allow(dead_code)]

use common::{literal::Literal, shape::Shape};
use stream_executor::{device_memory_allocator::DeviceMemoryAllocator,
  stream::Stream, stream_executor::StreamExecutor};
use crate::shaped_buffer::{ScopedShapedBuffer, ShapedBuffer};

pub struct TransferMetadata {}

// The TransferManager interface lets backends provide platform-specific
// mechanisms for constructing literals from given device memory handles.
// This lets each platform customize how literals are transferred to/from the
// device in terms of padding, leading dimension, etc.
pub struct TransferManager {}

impl TransferManager {
  pub fn new() {}
  pub fn platform_id() {}
  pub fn host_shape_to_device_shape() {}

  // Returns a literal containing the data held in the given ShapedBuffer
  // using the provided executor. This operation is performed synchronously
  // without waiting for any other operation on a stream to complete.
  //
  // This function should be avoided in favor of the asynchronous version below.
  //
  // Optionally caller can specify platform-specific transfer metadata that
  // tells the actual implementation to do something special.
  pub fn transfer_literal_from_device(
    &self,
    _stream: &Stream,
    _device_buffer: &ShapedBuffer,
    _transfer_metadata: Option<&TransferMetadata>) -> Result<Literal, String>
  {
    unimplemented!()
  }

  // Transfers the given literal into the previously allocated device memory
  // represented by the given ShapedBuffer using the given executor. The shape
  // of the ShapedBuffer and DeviceShape(literal.shape()) must be compatible,
  // but need not have the same layout.
  //
  // This operation is performed synchronously without waiting for any other
  // operation on a stream to complete. This function should be avoided in favor
  // of the asynchronous version below.
  //
  // Optionally caller can specify platform-specific transfer metadata that
  // tells the actual implementation to do something special.
  pub fn transfer_literal_to_device(
    &self,
    _stream: &Stream,
    _literal: &Literal,
    _device_buffer: &ShapedBuffer,
    _transfer_metadata: Option<&TransferMetadata>) -> Result<(), String>
  {
    unimplemented!()
  }

  pub fn transfer_literal_to_device_async() {}
  pub fn transfer_array_to_device() {}
  pub fn transfer_array_to_device_async() {}
  pub fn transfer_array_from_device() {}
  pub fn read_dynamic_shapes() {}

  // Transfers the given literal into the Infeed interface of the device,
  // using the given executor.
  pub fn transfer_literal_to_infeed(
    &self, _executor: &StreamExecutor, _literal: &Literal) -> Result<(), String>
  {
    unimplemented!()
  }

  // Transfers the given literal from the Outfeed interface of the device,
  // using the given executor. The shape and layout are determined by the
  // shape and layout of `literal`.
  pub fn transfer_literal_from_outfeed(
    &self, _executor: &StreamExecutor, _literal: &Literal) -> Result<(), String>
  {
    unimplemented!()
  }

  pub fn reset_devices(&self, _executors: &Vec<StreamExecutor>) -> Result<(), String> {
    unimplemented!()
  }

  pub fn write_tuple_index_tables() {}
  pub fn write_tuple_index_tables_async() {}
  pub fn get_byte_size_requirement() {}
  pub fn choose_compact_layout_for_shape() {}
  pub fn choose_good_infeed_layout() {}

  // Allocates a ScopedShapedBuffer which can hold data with the given on-host
  // shape. The on-device shape may be different as indicated by
  // HostShapeToDeviceShape.
  pub fn allocate_scoped_shaped_buffer(
    &self,
    _on_host_shape: &Shape,
    _allocator: &DeviceMemoryAllocator,
    _device_ordinal: i64,
    _shape_representation_func: Option<&dyn Fn(&Shape)->Shape>
  ) -> Result<ScopedShapedBuffer, String>
  {
    unimplemented!()
  }

  pub fn can_shaped_buffer_be_accessed_now() {}
  pub fn can_buffer_be_accessed_now() {}
  pub fn register_transfer_manager() {}
  pub fn get_for_platform() {}
  pub fn write_single_tuple_index_table() {}
  pub fn pack_subbyte_types() {}
}