#![allow(dead_code)]

// This file defines the interface for the loop cache analysis.

struct IndexedReference {}
impl IndexedReference {
  pub fn new() {}
  pub fn is_valid() {}
  pub fn get_base_pointer() {}
  pub fn get_num_subscripts() {}
  pub fn get_subscript() {}
  pub fn get_first_subscript() {}
  pub fn get_last_subscript() {}
  pub fn has_special_reuse() {}
  pub fn has_temporal_reuse() {}
  pub fn compute_ref_cost() {}
  fn delinearize() {}
  fn try_delinearize_fixed_size() {}
  fn is_loop_invariant() {}
  fn is_consecutive() {}
  fn get_subscript_index() {}
  fn get_last_coefficient() {}
  fn is_coeff_for_loop_zero_or_invariant() {}
  fn is_simple_add_resurrence() {}
  fn is_aliased() {}
}

struct CacheCost {}
impl CacheCost {
  pub fn new() {}
  pub fn get_cache_cost() {}
  pub fn get_loop_cost() {}
  pub fn get_loop_costs() {}
  fn calculate_cahce_footprint() {}
  fn popilate_reference_groups() {}
  fn compute_loop_cache_cost() {}
  fn compute_ref_group_cache_cost() {}
  fn sort_loop_costs() {}
  pub fn loop_cache_printer_pass() {}
  pub fn run() {}
}