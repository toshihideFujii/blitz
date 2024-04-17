#![allow(dead_code)]

use std::collections::HashMap;

use hlo::hlo_computation::HloComputation;

pub struct CholeskyExpander {
  computation_cache: HashMap<String, HloComputation>
}

impl CholeskyExpander {
  pub fn new() {}
  pub fn name() -> String { "cholesky-expander".to_string() }
  pub fn expand_instruction() {}

  fn build_cholesky() {}
}