#![allow(dead_code)]

pub struct ExecutableInput {}

impl ExecutableInput {
  pub fn new() {}

  pub fn shape() {}
  pub fn host_shape() {}
  pub fn set_dynamic_shape() {}
  pub fn to_shaped_buffer() {}
  pub fn set_buffer() {}
  pub fn set_unowned_buffer() {}
  pub fn set_unowned_index() {}
  pub fn clear_unowned_index() {}
  pub fn unowned_indices() {}
  pub fn buffers() {}
  pub fn mutable_buffers() {}
  pub fn mutable_buffer() {}
  pub fn buffer() {}
}

pub struct ExecutionOutput {}

impl ExecutionOutput {
  pub fn new() {}
  pub fn add_aliased_index() {}
  pub fn add_to_be_released() {}
  pub fn commit() {}
  pub fn result() {}
  pub fn mutable_result() {}
  pub fn consume_result() {}
  pub fn to_be_released() {}
  pub fn consume_to_be_released() {}
  pub fn consume_aliased_indices() {}
}

// A given platform's compiler will produce an Executable -- this is a uniform
// interface that is used for launching compiled programs across platforms.
pub struct Executable {}

impl Executable {
  pub fn new() {}
  pub fn execute_on_stream() {}
  pub fn execute_async_on_stream() {}
}