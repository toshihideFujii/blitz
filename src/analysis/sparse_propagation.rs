#![allow(dead_code)]

// This file implements an abstract sparse conditional propagation
// algorithm, medeled after SCCP, but with a customizable lattice function.

struct AbstractLatticeFunction {}
impl AbstractLatticeFunction {
  pub fn new() {}
  pub fn get_undef_val() {}
  pub fn get_overdefined_val() {}
  pub fn get_untracked_val() {}
  pub fn is_untracked_value() {}
  pub fn compute_lattice_val() {}
  pub fn is_spacial_cased_phi() {}
  pub fn merge_values() {}
  pub fn compute_instruction_state() {}
  pub fn print_lattice_val() {}
  pub fn print_lattice_key() {}
  pub fn get_value_from_lattice_val() {}
}

struct SparseSolver {}
impl SparseSolver {
  pub fn new() {}
  pub fn solve() {}
  pub fn print() {}
  pub fn get_existing_value_state() {}
  pub fn get_value_state() {}
  pub fn is_edge_feasible() {}
  pub fn is_block_executable() {}
  pub fn mark_block_executable() {}
}