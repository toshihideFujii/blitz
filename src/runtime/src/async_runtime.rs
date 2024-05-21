#![allow(dead_code)]

pub trait AsyncTaskRunner {
  fn schedule() {}
}

pub struct AsyncRuntime {

}

impl AsyncRuntime {
  pub fn new() {}
  pub fn set() {}
  pub fn get_current_runtime() {}
  pub fn create_token() {}
  pub fn set_available() {}
  pub fn set_error() {}
  pub fn is_error() {}
  pub fn await_token() {}

  pub fn create_value() {}
  pub fn set_available() {}
  pub fn set_error_value() {}
  pub fn is_error_value() {}
  pub fn await_value() {}

  pub fn create_group() {}
  pub fn add_token_to_group() {}
  pub fn is_error_group() {}
  pub fn await_group() {}

  pub fn execute() {}

  pub fn get_storage() {}
  pub fn allocate_storage() {}
  pub fn get_async_value() {}
  pub fn add_ref() {}
  pub fn frop_ref() {}
  pub fn to_async_runtime_object() {}

  pub fn as_value() {}
  pub fn runner() {}
}

pub struct AsyncRuntimeObject {}

pub struct AsyncToken {}

impl AsyncToken {
  pub fn new() {}
  pub fn get_async_value() {}
}