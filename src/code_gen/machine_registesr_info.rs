#![allow(dead_code)]

struct MachineRegisterInfo {}
impl MachineRegisterInfo {
  pub fn new() {}
  pub fn get_reg_use_def_list_head() {}
  pub fn get_next_operand_for_reg() {}
  pub fn get_target_register_info() {}
  pub fn reset_delegate() {}
  pub fn add_delegate() {}
  pub fn note_new_virtual_register() {}
  pub fn note_clone_virtual_register() {}
  pub fn is_ssa() {}
  pub fn leave_ssa() {}
  pub fn tracks_liveness() {}
  pub fn invalidate_liveness() {}
  pub fn should_track_sub_reg_liveness() {}
  pub fn sub_reg_liveness_enabled() {}
  pub fn is_updated_csrs_initialized() {}
  pub fn is_argument_register() {}
  pub fn is_fixed_register() {}
  pub fn is_general_purpose_register() {}
  pub fn disable_callee_saved_register() {}
  pub fn get_callee_saved_regs() {}
  pub fn set_callee_saved_regs() {}
  pub fn add_reg_operand_to_use_list() {}
  pub fn remove_reg_operand_from_use_list() {}
  pub fn move_operands() {}
  pub fn verify_use_list() {}
  pub fn verify_use_lists() {}
}