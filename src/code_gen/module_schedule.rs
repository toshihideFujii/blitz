#![allow(dead_code)]

struct ModuleSchedule {}
impl ModuleSchedule {
  pub fn new() {}
  pub fn get_loop() {}
  pub fn get_num_stages() {}
  pub fn get_first_cycle() {}
  pub fn get_final_cycle() {}
  pub fn get_stage() {}
  pub fn get_cycle() {}
  pub fn set_stage() {}
  pub fn get_instructions() {}
  pub fn dump() {}
  pub fn print() {}
}

struct ModuleScheduleExpander {}
impl ModuleScheduleExpander {
  pub fn new() {}
  pub fn generate_pipelined_loop() {}
  pub fn generate_prolog() {}
  pub fn generate_epilog() {}
  pub fn generate_existing_phis() {}
  pub fn generate_phis() {}
  pub fn remove_dead_instructions() {}
  pub fn split_lifetimes() {}
  pub fn add_branches() {}
  pub fn compute_delta() {}
  pub fn update_mem_operands() {}
  pub fn clone_instr() {}
  pub fn clone_and_change_instr() {}
  pub fn update_instruction() {}
  pub fn find_def_in_loop() {}
  pub fn get_prev_map_val() {}
  pub fn rewrite_phi_values() {}
  pub fn rewrite_scheduled_instr() {}
  pub fn is_loop_carried() {}
  pub fn get_stages_for_reg() {}
  pub fn get_stages_for_phi() {}

  pub fn expand() {}
  pub fn cleanup() {}
  pub fn get_rewritten_kernel() {}
}