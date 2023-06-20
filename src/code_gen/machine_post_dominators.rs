#![allow(dead_code)]

// This file exposes interfaces to post dominance information for
// target-specific code.

struct MachinePostDominatorTree {}
impl MachinePostDominatorTree {
  pub fn new() {}
  pub fn get_base() {}
  pub fn get_root_node() {}
  pub fn get_node() {}
  pub fn dominates() {}
  pub fn properly_dominates() {}
  pub fn is_virtual_root() {}
  pub fn find_nearest_common_dominator() {}
  pub fn run_on_machine_function() {}
  pub fn get_analysis_usage() {}
  pub fn release_memory() {}
  pub fn verify_analysis() {}
  pub fn print() {}
}