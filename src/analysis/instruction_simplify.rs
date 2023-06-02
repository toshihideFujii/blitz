#![allow(dead_code)]

struct InstrInfoQuery {}
impl InstrInfoQuery {
  pub fn new() {}
  pub fn get_metadata() {}
  pub fn has_no_unsigned_wrap() {}
  pub fn has_no_signed_wrap() {}
  pub fn is_exact() {}
}

struct SimplifyQuery {}
impl SimplifyQuery {
  pub fn new() {}
  pub fn get_with_instruction() {}
  pub fn get_without_undef() {}
  pub fn is_undef_value() {}
  pub fn simplify_add_inst() {}
  pub fn simplify_sub_inst() {}
  pub fn simplify_mul_inst() {}
  pub fn simplify_sdiv_inst() {}
  pub fn simplify_udiv_inst() {}
  pub fn simplify_srem_inst() {}
  pub fn simplify_urem_inst() {}
  pub fn simplify_fneg_inst() {}
  pub fn simplify_fadd_inst() {}
  pub fn simplify_fsub_inst() {}
  pub fn simplify_fmul_inst() {}
  pub fn simplify_fmaf_mul() {}
  pub fn simplify_fdiv_inst() {}
  pub fn simplify_frem_inst() {}
  pub fn simplify_shl_inst() {}
  pub fn simplify_lshr_inst() {}
  pub fn simplify_ashr_inst() {}
  pub fn simplify_and_inst() {}
  pub fn simplify_or_inst() {}
  pub fn simplify_xor_inst() {}
  pub fn simplify_icmp_inst() {}
  pub fn simplify_fcmp_inst() {}
  pub fn simplify_select_inst() {}
  pub fn simplify_gep_inst() {}
  pub fn simplify_insert_value_inst() {}
  pub fn simplify_insert_element_inst() {}
  pub fn simplify_extract_value_inst() {}
  pub fn simplify_extract_element_inst() {}
  pub fn simplify_cast_inst() {}
  pub fn simplify_shuffle_vector_inst() {}
  pub fn simplify_cmp_inst() {}
  pub fn simplify_un_op() {}
  pub fn simplify_bin_op() {}
  pub fn simplify_call() {}
  pub fn simplify_constrained_fp_call() {}
  pub fn simplify_freeze_inst() {}
  pub fn simplify_load_inst() {}
  pub fn simplify_instruction() {}
  pub fn simplify_instruction_with_operands() {}
  pub fn simplify_with_op_replaced() {}
  pub fn replace_and_recursively_simplify() {}
  pub fn get_best_simplify_query() {}
}