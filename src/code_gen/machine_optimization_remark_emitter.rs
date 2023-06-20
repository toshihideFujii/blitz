#![allow(dead_code)]

// Optimization diagnostic interfaces for machine passes.

struct DiagnosticInfoMIROptimization {}
impl DiagnosticInfoMIROptimization {
  pub fn new() {}
  pub fn class_of() {}
  pub fn get_block() {}
}

struct MachineOptimizationRemark {}
impl MachineOptimizationRemark {
  pub fn new() {}
  pub fn class_of() {}
  pub fn is_enabled() {}
}

struct MachineOptimizationRemarkMissed {}
impl MachineOptimizationRemarkMissed {
  pub fn new() {}
  pub fn class_of() {}
  pub fn is_enabled() {}
}

struct MachineOptimizationRemarkAnalysis {}
impl MachineOptimizationRemarkAnalysis {
  pub fn new() {}
  pub fn class_of() {}
  pub fn is_enabled() {}
}

struct MachineOptimizationRemarkEmitter {}
impl MachineOptimizationRemarkEmitter {
  pub fn new() {}
  pub fn emit() {}
  pub fn allow_external_analysis() {}
  pub fn get_bfi() {}
  fn compute_hotness() {}
  fn should_emit_verbose() {}
}

struct MachineOptimizationRemarkEmitterPass {}
impl MachineOptimizationRemarkEmitterPass {
  pub fn new() {}
  pub fn run_on_machine_function() {}
  pub fn get_analysis_usage() {}
  pub fn get_ore() {}
}