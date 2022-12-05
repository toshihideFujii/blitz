#![allow(dead_code)]

/*
This file contains the declaration of the BasicBlock
class.
*/

struct BasicBlock {}

impl BasicBlock {
  pub fn new() {}

  pub fn set_parent() {}

  pub fn get_context() {}

  pub fn get_parent() {}

  pub fn get_module() {}

  pub fn get_terminator() {}

  pub fn get_terminating_deoptimize_call() {}

  pub fn get_post_dominating_deoptimize_call() {}

  pub fn get_terminating_must_tail_call() {}

  pub fn get_first_non_phi() {}

  pub fn get_first_non_phi_or_dbg() {}

  pub fn get_first_non_phi_or_dbg_or_lifetime() {}

  pub fn get_first_insertion_pt() {}

  pub fn get_first_non_phi_or_dbg_or_alloca() {}

  pub fn instructions_without_debug() {}

  pub fn size_without_debug() {}

  pub fn remove_from_parent() {}

  pub fn earse_from_parent() {}

  pub fn move_before() {}

  pub fn move_after() {}

  pub fn insert_into() {}

  pub fn get_single_predecessor() {}

  pub fn get_unique_predecessor() {}

  pub fn has_n_predecessors() {}

  pub fn has_n_predecessors_or_more() {}

  pub fn get_single_successor() {}

  pub fn get_unique_successor() {}

  pub fn print() {}

  pub fn begin() {}

  pub fn end() {}

  pub fn rbegin() {}

  pub fn rend() {}

  pub fn size() {}

  pub fn empty() {}

  pub fn front() {}

  pub fn back() {}

  pub fn phis() {}

  pub fn get_inst_list() {}

  pub fn get_sublist_access() {}

  pub fn get_value_symbol_table() {}

  pub fn drop_all_references() {}

  pub fn remove_predecessor() {}

  pub fn can_split_predecessors() {}

  pub fn split_basic_block() {}

  pub fn split_basic_block_before() {}

  pub fn has_address_taken() {}

  pub fn replace_phi_uses_with() {}

  pub fn replace_successors_phi_uses_with() {}

  pub fn is_ehpad() {}

  pub fn is_landing_pad() {}

  pub fn get_landing_pad_inst() {}

  pub fn is_legal_to_hoist_into() {}

  pub fn is_entry_block() {}

  pub fn get_irr_loop_header_weight() {}

  pub fn is_instr_order_valid() {}

  pub fn invalidate_orders() {}

  pub fn renumber_instructions() {}

  pub fn validate_instr_ordering() {}

  fn get_basic_block_bits() {}

  fn set_basic_block_bits() {}

  fn adjust_block_address_ref_count() {}

  fn set_value_subclass_data() {}

  fn skip_debug_intrinsics() {}
}