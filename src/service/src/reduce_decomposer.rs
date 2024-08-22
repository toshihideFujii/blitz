#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{hlo_instruction::HloInstruction, hlo_module::HloModule};

pub struct ReduceDecomposer {}

impl ReduceDecomposer {
  pub fn new() -> Self {
    ReduceDecomposer {  }
  }

  pub fn name(&self) -> String {
    "reduce-decomposer".to_string()
  }

  pub fn run(
    &mut self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }
}

struct VariadicReductionLayoutEqualizer {}

impl VariadicReductionLayoutEqualizer {
  pub fn new() {}
  pub fn run_on_module() {}
  pub fn handle_reduce() {}
}

struct ReduceDecomposerVisitor {}

impl ReduceDecomposerVisitor {
  pub fn new() {}

  pub fn run_on_module() {}
  
  pub fn handle_reduce(_hlo: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  fn get_output() {}
  fn expected_output_shape() {}
}