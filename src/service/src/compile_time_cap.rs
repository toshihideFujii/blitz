#![allow(dead_code)]

use hlo::hlo_module::HloModule;

// Provide a common way to bound compiler analyses that potentially have
// overhead that is non-linear to the number of instructions in a module.
pub struct BoundNonLinearCompilerAnalysis {
  analysis_allowance: i64
}

impl BoundNonLinearCompilerAnalysis {
  // Sampling_rate specifies the proportion of all instructions expected to be
  // analyzed. e.g., if sampling_rate_=2, then every other instructions are
  // expected to be analyzed. If sample_rate <= 0, the analysis will be always
  // allowed to complete. Each analysis is allowed at least a constant number of
  // abstract cost units, before it is considered for early termination.
  pub fn new(_m: &HloModule, _pass_name: String, _sampling_rate: Option<i64>) -> Self {
    BoundNonLinearCompilerAnalysis { analysis_allowance: 0 }
  }

  pub fn deduct_cost(&mut self, cost_now: i64) -> bool {
    if self.analysis_allowance > 0 && cost_now > 0 {
      self.analysis_allowance -= cost_now;
      if self.analysis_allowance < 0 {
        self.analysis_allowance = 0;
      }
    }
    self.analysis_allowance != 0
  }

  pub fn continue_analysis(&self) -> bool {
    self.analysis_allowance != 0
  }

  pub fn analysis_allowance(&self) -> i64 {
    self.analysis_allowance
  }
}