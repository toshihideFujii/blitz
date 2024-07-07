#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// HLO pass which removes dead code from computations in the module using
// HloModule-scoped analysis (HloLivenessAnalysis).
pub struct HloModuleDCE {}

impl HloModuleDCE {
  pub fn new() -> Self {
    HloModuleDCE {  }
  }

  pub fn name(&self) -> String {
    "hlo-module-dce".to_string()
  }

  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    Ok(true)
  }
}