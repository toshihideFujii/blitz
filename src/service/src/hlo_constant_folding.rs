#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// A pass which performs constant folding in order to avoid unnecessary
// computation on constants.
pub struct HloConstantFolding {
  slow_op_counter: i64
}

impl HloConstantFolding {
  pub fn new() -> Self {
    HloConstantFolding { slow_op_counter: 0 }
  }

  pub fn name(&self) -> String {
    "constant-folding".to_string()
  }
  
  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: HashSet<String>) -> Result<bool, String>
  {
    Ok(true)
  }
}