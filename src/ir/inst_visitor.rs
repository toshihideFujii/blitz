#![allow(dead_code)]

// Base class for instruction visitors.
struct InstVisitor {}
impl InstVisitor {
  pub fn new() {}
  pub fn visit_module() {}
  pub fn visit_function() {}
  pub fn visit_basic_block() {}
  pub fn visit_icmp_inst() {}
  pub fn visit_fcmp_inst() {}
  pub fn visit_alloca_inst() {}
  pub fn visit_load_inst() {}
  pub fn visit_store_inst() {}
  pub fn visit_atomic_cmp_xchg_inst() {}
  pub fn visit_atomic_bmw_inst() {}
  pub fn visit_fence_inst() {}
  pub fn visit_get_element_ptr_inst() {}
  pub fn visit_phi_node() {}
  pub fn visit_trunc_inst() {}
  pub fn visit_zext_inst() {}
  pub fn visit_sext_inst() {}
  pub fn visit_fp_trunc_inst() {}
  pub fn visit_fp_ext_inst() {}
  pub fn visit_fp_to_ui_inst() {}
  pub fn visit_fp_to_si_inst() {}
  pub fn visit_ui_to_fp_inst() {}
  pub fn visit_si_to_fp_inst() {}
  pub fn visit_ptr_to_int_inst() {}
  pub fn visit_int_to_ptr_inst() {}
  pub fn visit_bit_cast_inst() {}
  pub fn visit_addr_scace_cast_inst() {}
  pub fn visit_select_inst() {}
  pub fn visit_va_arg_inst() {}
  pub fn visit_extract_element_inst() {}
  pub fn visit_insert_element_inst() {}
  pub fn visit_shuffle_vector_inst() {}
  pub fn visit_extract_value_inst() {}
  pub fn visit_insert_value_inst() {}
  pub fn visit_landing_pad_inst() {}
  pub fn visit_funclet_pad_inst() {}
  pub fn visit_cleanup_pad_inst() {}
  pub fn visit_freeze_inst() {}

  pub fn visit_dbg_declare_inst() {}
  pub fn visit_dbg_value_inst() {}
  pub fn visit_dbg_variable_intrinsic() {}
  pub fn visit_dbg_label_inst() {}
  pub fn visit_dbg_info_intrinsic() {}
  pub fn visit_mem_set_inst() {}
  pub fn visit_mem_set_inline_inst() {}
  pub fn visit_mem_cpy_inst() {}
  pub fn visit_mem_cpy_inline_inst() {}
  pub fn visit_mem_move_inst() {}
  pub fn visit_mem_transfer_inst() {}
  pub fn visit_mem_intrinsic() {}
  pub fn visit_va_start_inst() {}
  pub fn visit_va_end_inst() {}
  pub fn visit_va_copy_inst() {}
  pub fn visit_intrinsic_inst() {}
  pub fn visit_call_inst() {}
  pub fn visit_invoke_inst() {}
  pub fn visit_call_br_inst() {}

  pub fn visit_return_inst() {}
  pub fn visit_branch_inst() {}
  pub fn visit_switch_inst() {}
  pub fn visit_indirect_br_inst() {}
  pub fn visit_resume_inst() {}
  pub fn visit_unreachable_inst() {}
  pub fn visit_cleanup_return_inst() {}
  pub fn visit_catch_return_inst() {}
  pub fn visit_catch_switch_inst() {}
  pub fn visit_terminator() {}

  pub fn visit_cast_inst() {}
  pub fn visit_unary_operator() {}
  pub fn visit_binary_operator() {}
  pub fn visit_cmp_inst() {}
  pub fn visit_unary_instruction() {}

  pub fn visit_call_base() {}

  pub fn visit_instruction() {}

  fn delegate_call_inst() {}
}