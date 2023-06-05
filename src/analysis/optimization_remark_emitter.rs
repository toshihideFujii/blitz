#![allow(dead_code)]

// Optimization diagnostic interfaces.
// It's packaged as an analysis pass so that by using this service
// passes become dependent on BFI as well.
// BFI is used to compute the "hotness" of the diagnostic message.

struct OptimizationRemarkEmitter {}
impl OptimizationRemarkEmitter {
  pub fn new() {}
  pub fn invalidate() {}
  pub fn enabled() {}
  pub fn emit() {}
  pub fn allow_extra_analysis() {}
  fn compute_hotness() {}
  fn should_emit_verbose() {}
}

struct OptimizationRemarkEmitterWrapperPass {}
impl OptimizationRemarkEmitterWrapperPass {
  pub fn new() {}
  pub fn run_on_function() {}
  pub fn get_analysis_usage() {}
  pub fn get_ore() {}
}

struct OptimizationRemarkEmitterAnalysis {}
impl OptimizationRemarkEmitterAnalysis {
  pub fn new() {}
  pub fn run() {}
}