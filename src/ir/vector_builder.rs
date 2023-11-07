#![allow(dead_code)]

use crate::support::type_size::ElementCount;
use super::value::Value;

enum Behabior {
  ReportAndAbort,
  SilentlyReturnNone
}

struct VectorBuilder {
  // Explicit mask parameter.
  mask: Box<dyn Value>,
  // Explicit vector length parameter.
  explicit_vector_length: Box<dyn Value>,
  // Compile time vector length.
  static_vector_length: ElementCount
}

impl VectorBuilder {
  pub fn new() {}
  pub fn request_mask() {}
  pub fn request_evl() {}
  pub fn handle_error() {}
  pub fn return_with_error() {}
  pub fn get_module() {}
  pub fn get_context() {}
  pub fn get_all_true_mask() {}

  pub fn set_mask(&mut self, mask: Box<dyn Value>) -> &VectorBuilder {
    self.mask = mask;
    self
  }

  pub fn set_evl(&mut self, evl: Box<dyn Value>) -> &VectorBuilder {
    self.explicit_vector_length = evl;
    self
  }

  pub fn set_static_vl(&mut self, fixed_val: usize) -> &VectorBuilder {
    self.static_vector_length = ElementCount::get_fixed(fixed_val);
    self
  }

  pub fn create_vector_instruction() {}
}