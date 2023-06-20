#![allow(dead_code)]

struct LiveRegUnit {}

struct Trace {}
impl Trace {
  pub fn new() {}
  pub fn get_block_num() {}
  pub fn print() {}
  pub fn get_instr_count() {}
  pub fn get_resource_depth() {}
  pub fn get_resource_length() {}
  pub fn get_critical_path() {}
  pub fn get_instr_cycles() {}
  pub fn get_instr_slack() {}
  pub fn get_phi_depth() {}
  pub fn is_dep_in_trace() {}
}

struct Ensemble {}
impl Ensemble {
  pub fn new() {}
  pub fn compute_trace() {}
  pub fn compute_depth_resources() {}
  pub fn compute_height_resources() {}
  pub fn compute_cross_block_critical_path() {}
  pub fn compute_instr_depths() {}
  pub fn compute_instr_heights() {}
  pub fn add_live_ins() {}

  pub fn get_name() {}
  pub fn print() {}
  pub fn invalidate() {}
  pub fn verify() {}
  pub fn get_trace() {}
  pub fn update_depth() {}
  pub fn update_depths() {}
}

struct MachineTraceMetrics {}
impl MachineTraceMetrics {
  pub fn new() {}
  pub fn get_analysis_usage() {}
  pub fn run_on_machine_function() {}
  pub fn release_memory() {}
  pub fn verify_analysis() {}
  pub fn get_resources() {}
  pub fn get_proc_resource_cycles() {}
}