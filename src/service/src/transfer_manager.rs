#![allow(dead_code)]

use common::literal::Literal;
use stream_executor::stream_executor::StreamExecutor;

// The TransferManager interface lets backends provide platform-specific
// mechanisms for constructing literals from given device memory handles.
// This lets each platform customize how literals are transferred to/from the
// device in terms of padding, leading dimension, etc.
pub struct TransferManager {}

impl TransferManager {
  pub fn new() {}
  pub fn platform_id() {}
  pub fn host_shape_to_device_shape() {}
  pub fn transfer_literal_from_device() {}
  pub fn transfer_literal_to_device() {}
  pub fn transfer_literal_to_device_async() {}
  pub fn transfer_array_to_device() {}
  pub fn transfer_array_to_device_async() {}
  pub fn transfer_array_from_device() {}
  pub fn read_dynamic_shapes() {}

  // Transfers the given literal into the Infeed interface of the device,
  // using the given executor.
  pub fn transfer_literal_to_infeed<T>(
    &self, _executor: &dyn StreamExecutor, _literal: &Literal<T>) -> Result<(), String>
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  // Transfers the given literal from the Outfeed interface of the device,
  // using the given executor. The shape and layout are determined by the
  // shape and layout of `literal`.
  pub fn transfer_literal_from_outfeed<T>(
    &self, _executor: &dyn StreamExecutor, _literal: &Literal<T>) -> Result<(), String>
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  pub fn reset_devices() {}
  pub fn write_tuple_index_tables() {}
  pub fn write_tuple_index_tables_async() {}
  pub fn get_byte_size_requirement() {}
  pub fn choose_compact_layout_for_shape() {}
  pub fn choose_good_infeed_layout() {}
  pub fn allocate_scoped_shaped_buffer() {}
  pub fn can_shaped_buffer_be_accessed_now() {}
  pub fn can_buffer_be_accessed_now() {}
  pub fn register_transfer_manager() {}
  pub fn get_for_platform() {}
  pub fn write_single_tuple_index_table() {}
  pub fn pack_subbyte_types() {}
}