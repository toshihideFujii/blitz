#![allow(dead_code)]

// Provide a common way to bound compiler analyses that potentially have
// overhead that is non-linear to the number of instructions in a module.
pub struct BoundNonLinearCompilerAnalysis {
  analysis_allowance: i64
}

impl BoundNonLinearCompilerAnalysis {
  pub fn new() {}

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