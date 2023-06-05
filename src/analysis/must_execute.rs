#![allow(dead_code)]

struct LoopSafetyInfo {}
impl LoopSafetyInfo {
  pub fn new() {}
  pub fn compute_block_colors() {}
  pub fn get_block_colors() {}
  pub fn copy_colors() {}
  pub fn block_may_throw() {}
  pub fn any_block_may_throw() {}
  pub fn all_loop_paths_lead_to_block() {}
  pub fn compute_loop_safety_info() {}
  pub fn is_guaranteed_to_execute() {}
}

struct SimpleLoopSafetyInfo {}
impl SimpleLoopSafetyInfo {
  pub fn new() {}
  pub fn block_may_throw() {}
  pub fn any_block_may_throw() {}
  pub fn compute_loop_safety_info() {}
  pub fn is_guaranteed_to_execute() {}
}

struct ICFLoopSafetyInfo {}
impl ICFLoopSafetyInfo {
  pub fn new() {}
  pub fn block_may_throw() {}
  pub fn any_block_may_throw() {}
  pub fn compute_loop_safety_info() {}
  pub fn is_guaranteed_to_execute() {}
  pub fn doen_not_write_memory_before() {}
  pub fn insert_instruction_to() {}
  pub fn remove_instruction() {}
}
