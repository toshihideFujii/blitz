#![allow(dead_code)]

// This file defines LoopPass class.
// All loop optimization and transformation passes are derived
// from LoopPass.

struct LoopPass {}
impl LoopPass {
  pub fn new() {}
  pub fn create_printer_pass() {}
  pub fn run_on_loop() {}
  pub fn do_initialization() {}
  pub fn do_finalization() {}
  pub fn prepare_pass_manager() {}
  pub fn assign_pass_manager() {}
  pub fn get_potential_pass_manager_type() {}
  pub fn skip_loop() {}
}

struct LPPassManager {}
impl LPPassManager {
  pub fn new() {}
  pub fn run_on_function() {}
  pub fn get_analysis_usage() {}
  pub fn get_pass_name() {}
  pub fn get_as_pm_data_manager() {}
  pub fn get_as_pass() {}
  pub fn dump_pass_structure() {}
  pub fn get_contained_pass() {}
  pub fn get_pass_manager_type() {}
  pub fn add_loop() {}
  pub fn mark_loop_as_deleted() {}
}

struct LCSSAVerificationPass {}
impl LCSSAVerificationPass {
  pub fn new() {}
  pub fn run_on_function() {}
  pub fn get_analysis_usage() {}
}