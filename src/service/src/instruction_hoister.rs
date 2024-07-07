#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// HLO pass that hoists parameters and constants to increase opportunities
// for prefetching.
pub struct InstructionHoister {
  hoist_parameters: bool,
  host_constants: bool,
}

impl InstructionHoister {
  pub fn new(hoist_parameters: bool, host_constants: bool) -> Self {
    InstructionHoister {
      hoist_parameters: hoist_parameters,
      host_constants: host_constants
    }
  }

  pub fn name(&self) -> String {
    "instruction-hoister".to_string()
  }

  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }
}