#![allow(dead_code)]

struct MachinePointerInfo {}
impl MachinePointerInfo {
  pub fn new() {}
  pub fn get_with_offset() {}
  pub fn is_dereferenceable() {}
  pub fn get_addr_space() {}
  pub fn get_constant_pool() {}
  pub fn get_fixed_stack() {}
  pub fn get_jump_table() {}
  pub fn get_got() {}
  pub fn get_stack() {}
  pub fn get_unknown_stack() {}
}

struct MachineMemOperand {}
impl MachineMemOperand {
  pub fn new() {}
  pub fn get_pointer_info() {}
  pub fn get_value() {}
  pub fn get_pseudo_value() {}
  pub fn get_flags() {}
  pub fn get_offset() {}
  pub fn get_addr_space() {}
  pub fn get_memory_type() {}
  pub fn get_size() {}
  pub fn get_size_in_bits() {}
  pub fn get_type() {}
  pub fn get_align() {}
  pub fn get_base_align() {}
  pub fn get_aa_info() {}
  pub fn get_ranges() {}
  pub fn get_sync_scope_id() {}
  pub fn get_success_ordering() {}
  pub fn get_failure_ordering() {}
  pub fn get_merged_ordering() {}
  pub fn is_load() {}
  pub fn is_store() {}
  pub fn is_volatile() {}
  pub fn is_non_temporal() {}
  pub fn is_dereferenceable() {}
  pub fn is_invariant() {}
  pub fn is_atomic() {}
  pub fn is_unordered() {}
  pub fn refine_alignment() {}
  pub fn set_value() {}
  pub fn set_offset() {}
  pub fn set_type() {}
  pub fn print() {}
}