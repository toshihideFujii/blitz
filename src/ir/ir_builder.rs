#![allow(dead_code)]

// This file defines the IRBuilder class, which is used as a
// convenient way to create Blitz instructions with a consistent
// and simplified interface.

use crate::adt::ap_int::APInt;
use super::{constants::ConstantInt, blits_context::blits_context_mut,
  type_::{IntegerType, self}, /*instruction::InstructionBase,
  basic_block::BasicBlock, value::Value*/};

// This provides the default implementation of the IRBuilder
// 'inser_helper' method that is called whenever an instruction is
// created by IRBuilder and needs to be inserted.
struct IRBuilderDefaultInserter {}
impl IRBuilderDefaultInserter {
  pub fn new() {}
  //pub fn insert_helper(&self, i: InstructionBase, name: Twine, bb: Option<BasicBlock>) {
    //if bb.is_some() {
      //i.insert_into(bb.unwrap());
    //}
    //Value::set_name(&i, name);
  //}
}

struct IRBuilderCallbackInserter {}
impl IRBuilderCallbackInserter {
  pub fn new() {}
  pub fn insert_helper() {}
}

struct InsertPoint {}

struct InsertPointGuard {}

struct FastMathFlagGuard {}

// RAII object that stores the current default operand bundles and restores
// them when the object is destroyed.
struct OperandBundlesGuard {}
impl OperandBundlesGuard {
  pub fn new() {}
}


// This provides a uniform API for creating instructions and inserting
// them into a basic block either at the end of a BasicBlock, or at a specific 
// iterator location in a block.
struct IRBuilder {
  //context: BlitzContext
}

impl IRBuilder {
  pub fn new() {}
  pub fn insert() {}
  pub fn clear_insertion_point() {}
  pub fn get_insert_block() {}
  pub fn get_context() {}
  pub fn set_insert_point() {}
  pub fn set_current_debug_location() {}
  pub fn collect_metadata_to_copy() {}
  pub fn get_current_debug_location() {}
  pub fn set_inst_debug_location() {}
  pub fn add_metadata_to_inst() {}
  pub fn get_current_function_return_type() {}
  pub fn save_ip() {}
  pub fn save_and_clear_ip() {}
  pub fn restore_ip() {}
  pub fn get_default_fp_math_tag() {}
  pub fn get_fast_math_flags() {}
  pub fn clear_fast_math_flags() {}
  pub fn set_default_fp_math_tag() {}
  pub fn set_fast_math_flags() {}
  pub fn set_is_fp_constrained() {}
  pub fn get_is_fp_constrained() {}
  pub fn set_default_constrained_except() {}
  pub fn set_default_constrained_rounding() {}
  pub fn get_default_constrained_except() {}
  pub fn get_default_constrained_rounding() {}
  pub fn set_constrained_fp_function_attr() {}
  pub fn set_constrained_fp_call_attr() {}
  pub fn set_default_operand_bundles() {}

  // Make a new global variable with initializer type i8.
  pub fn create_global_string() {}

  // Get the constant value for i1 true.
  pub fn get_true(&mut self) -> ConstantInt {
    ConstantInt::get_true(blits_context_mut()).unwrap()
  }

  // Get the constant value for i1 false.
  pub fn get_false(&mut self) -> ConstantInt {
    ConstantInt::get_false(blits_context_mut()).unwrap()
  }

  // Get a constant value representing either true or false.
  pub fn get_int_1(&mut self, v: bool) -> ConstantInt {
    ConstantInt::get(&self.get_int_1_type(), v as i64, false)
  }

  // Get a constant 8-bit value.
  pub fn get_int_8(&mut self, c: u8) -> ConstantInt {
    ConstantInt::get(&self.get_int_8_type(), c as i64, false)
  }

  // Get a constant 16-bit value.
  pub fn get_int_16(&mut self, c: u16) -> ConstantInt {
    ConstantInt::get(&self.get_int_16_type(), c as i64, false)
  }

  // Get a constant 32-bit value.
  pub fn get_int_32(&mut self, c: u32) -> ConstantInt {
    ConstantInt::get(&self.get_int_32_type(), c as i64, false)
  }

  // Get a constant 64-bit value.
  pub fn get_int_64(&mut self, c: i64) -> ConstantInt {
    ConstantInt::get(&self.get_int_64_type(), c, false)
  }

  // Get a constant n-bit value, zero extended or truncated from a 64-bit value.
  pub fn get_int_n(&mut self, n: u32, c: i64) -> ConstantInt {
    ConstantInt::get(&self.get_int_n_type(n), c, false)
  }

  // Get a constant integer value.
  pub fn get_int(&mut self, apint: &APInt) -> ConstantInt {
    ConstantInt::get_from_apint(blits_context_mut(), apint.clone())
  }

  // Fetch the type representing a single bit.
  pub fn get_int_1_type(&mut self) -> IntegerType {
    type_::get_int_1_type(blits_context_mut())
  }

  // Fetch the type representing an 8-bit integer.
  pub fn get_int_8_type(&mut self) -> IntegerType {
    type_::get_int_8_type(blits_context_mut())
  }

  // Fetch the type representing a 16-bit integer.
  pub fn get_int_16_type(&mut self) -> IntegerType {
    type_::get_int_16_type(blits_context_mut())
  }

  // Fetch the type representing a 32-bit integer.
  pub fn get_int_32_type(&mut self) -> IntegerType {
    type_::get_int_32_type(blits_context_mut())
  }

  // Fetch the type representing a 64-bit integer.
  pub fn get_int_64_type(&mut self) -> IntegerType {
    type_::get_int_64_type(blits_context_mut())
  }

  // Fetch the type representing a 128-bit integer.
  pub fn get_int_128_type(&mut self) -> IntegerType {
    type_::get_int_128_type(blits_context_mut())
  }

  // Fetch the type representing an n-bit integer.
  pub fn get_int_n_type(&self, n: u32) -> IntegerType {
    type_::get_int_n_type(blits_context_mut(), n)
  }

  pub fn get_half_type() {}
  pub fn get_b_float_type() {}
  pub fn get_float_type() {}
  pub fn get_double_type() {}
  pub fn get_void_type() {}
  pub fn get_ptr_type() {}
  pub fn get_int_8_ptr_type() {}
  pub fn get_int_ptr_type() {}
  pub fn get_index_type() {}

  // Create and insert a memset to the specified pointer and the specified
  // value.
  pub fn create_mem_set() {}
  pub fn create_element_unordered_atomic_mem_set() {}

  // Create and insert a memcpy between the specified pointers.
  pub fn create_mem_cpy() {}
  pub fn create_mem_transfer_inst() {}
  pub fn create_element_unordered_atomic_mem_cpy() {}
  pub fn create_element_unordered_atomic_mem_move() {}

  pub fn get_reduction_intrinsic() {}

  pub fn create_f_add_reduce() {}
  pub fn create_f_mul_reduce() {}
  pub fn create_add_reduce() {}
  pub fn create_mul_reduce() {}
  pub fn create_and_reduce() {}
  pub fn create_or_reduce() {}
  pub fn create_xor_reduce() {}
  pub fn create_int_max_reduce() {}
  pub fn create_int_min_reduce() {}
  pub fn create_fp_max_reduce() {}
  pub fn create_fp_min_reduce() {}
  pub fn create_lifetime_start() {}
  pub fn create_lifetime_end() {}
  pub fn create_invariant_start() {}
  pub fn create_thread_local_address() {}
  pub fn create_masked_load() {}
  pub fn create_masked_store() {}
  pub fn create_masked_gather() {}
  pub fn create_masked_scatter() {}
  pub fn create_masked_expand_load() {}
  pub fn create_masked_compress_store() {}
  pub fn create_assumption() {}
  pub fn create_no_alias_scope_declaration() {}
  pub fn create_gc_statepoint_call() {}
  pub fn create_gc_statepoint_invoke() {}
  pub fn create_gc_result() {}
  pub fn create_gc_relocate() {}
  pub fn create_gc_get_pointer_base() {}
  pub fn create_gc_get_pointer_offset() {}
  pub fn create_vscale() {}
  pub fn create_element_count() {}
  pub fn create_type_size() {}
  pub fn create_step_vector() {}
  pub fn create_unary_intrinsic() {}
  pub fn create_binary_intrinsic() {}
  pub fn create_intrinsic() {}
  pub fn create_min_num() {}
  pub fn create_max_num() {}
  pub fn create_minimum() {}
  pub fn create_maximum() {}
  pub fn create_copy_sign() {}
  pub fn create_arithmetic_fence() {}
  pub fn create_extract_vector() {}
  pub fn create_insert_vector() {}
  pub fn create_ret_void() {}
  pub fn create_ret() {}
  pub fn create_aggregate_ret() {}
  pub fn create_br() {}
  pub fn create_cond_br() {}
  pub fn create_switch() {}
  pub fn create_indirect_br() {}
  pub fn create_invoke() {}
  pub fn create_call_br() {}
  pub fn create_resume() {}
  pub fn create_cleanup_ret() {}
  pub fn create_catch_switch() {}
  pub fn create_catch_pad() {}
  pub fn create_cleanup_pad() {}
  pub fn create_catch_ret() {}
  pub fn create_unreachable() {}
  pub fn create_add() {}
  pub fn create_nsw_add() {}
  pub fn create_nuw_add() {}
  pub fn create_sub() {}
  pub fn create_nsw_sub() {}
  pub fn create_nuw_sub() {}
  pub fn create_mul() {}
  pub fn create_nsw_mul() {}
  pub fn create_nuw_mul() {}
  pub fn create_udiv() {}
  pub fn create_exact_udiv() {}
  pub fn create_sdiv() {}
  pub fn create_exact_sdiv() {}
  pub fn create_urem() {}
  pub fn create_srem() {}
  pub fn create_shl() {}
  pub fn create_lshr() {}
  pub fn create_ashr() {}
  pub fn create_and() {}
  pub fn create_or() {}
  pub fn create_xor() {}
  pub fn create_fadd() {}
  pub fn create_fadd_fmf() {}
  pub fn create_fsub() {}
  pub fn create_fsub_fmf() {}
  pub fn create_fmul() {}
  pub fn create_fmul_fmf() {}
  pub fn create_fdiv() {}
  pub fn create_fdiv_fmf() {}
  pub fn create_frem() {}
  pub fn create_frem_fmf() {}
  pub fn create_bin_op() {}
  pub fn create_logical_and() {}
  pub fn create_logical_or() {}
  pub fn create_logical_op() {}
  pub fn create_constrained_fp_bin_op() {}
  pub fn create_neg() {}
  pub fn create_nsw_neg() {}
  pub fn create_nuw_neg() {}
  pub fn create_fneg() {}
  pub fn create_fneg_fmf() {}
  pub fn create_not() {}
  pub fn create_nary_op() {}
  pub fn create_alloca() {}
  pub fn create_load() {}
  pub fn create_store() {}
  pub fn create_aligned_load() {}
  pub fn create_aligned_store() {}
  pub fn create_fence() {}
  pub fn create_atomic_cmp_xchg() {}
  pub fn create_atomic_rmw() {}
  pub fn create_gep() {}
  pub fn create_inbounds_gep() {}
  pub fn create_const_gep_132() {}
  pub fn create_const_inbounds_gep_132() {}
  pub fn create_const_gep_232() {}
  pub fn create_const_inbounds_gep_232() {}
  pub fn create_const_gep_164() {}
  pub fn create_const_inbounds_gep_164() {}
  pub fn create_const_gep_264() {}
  pub fn create_const_inbounds_gep_264() {}
  pub fn create_struct_gep() {}
  pub fn create_global_string_ptr() {}
  pub fn create_trunc() {}
  pub fn create_zext() {}
  pub fn create_sext() {}
  pub fn create_zext_or_trunc() {}
  pub fn create_sext_or_trunc() {}
  pub fn create_fp_to_ui() {}
  pub fn create_fp_to_si() {}
  pub fn create_ui_to_fp() {}
  pub fn create_si_to_fp() {}
  pub fn create_fp_trunc() {}
  pub fn create_fp_ext() {}
  pub fn create_ptr_to_int() {}
  pub fn create_int_to_ptr() {}
  pub fn create_bit_cast() {}
  pub fn create_addr_space_cast() {}
  pub fn create_zext_or_bit_cast() {}
  pub fn create_sext_or_bit_cast() {}
  pub fn create_trunc_or_bit_cast() {} 
  pub fn create_cast() {}
  pub fn create_ptr_cast() {}
  pub fn create_ptr_bit_cast_or_addr_space_cast() {}
  pub fn create_int_cast() {}
  pub fn create_bit_or_ptr_cast() {}
  pub fn create_fp_cast() {}
  pub fn create_constrained_fp_cast() {}
  pub fn create_icmp_eq() {}
  pub fn create_icmp_ne() {}
  pub fn create_icmp_ugt() {}
  pub fn create_icmp_uge() {}
  pub fn create_icmp_ult() {}
  pub fn create_icmp_ule() {}
  pub fn create_icmp_sgt() {}
  pub fn create_icmp_sge() {}
  pub fn create_icmp_slt() {}
  pub fn create_icmp_sle() {}
  pub fn create_fcmp_oeq() {}
  pub fn create_fcmp_ogt() {}
  pub fn create_fcmp_oge() {}
  pub fn create_fcmp_olt() {}
  pub fn create_fcmp_ole() {}
  pub fn create_fcmp_one() {}
  pub fn create_fcmp_ord() {}
  pub fn create_fcmp_unq() {}
  pub fn create_fcmp_ueq() {}
  pub fn create_fcmp_ugt() {}
  pub fn create_fcmp_uge() {}
  pub fn create_fcmp_ult() {}
  pub fn create_fcmp_ule() {}
  pub fn create_fcmp_une() {}
  pub fn create_icmp() {}
  pub fn create_fcmp() {}
  pub fn create_cmp() {}
  pub fn create_fcmps() {}
  pub fn create_constrained_fpcmp() {}
  pub fn create_phi() {}
  pub fn create_call() {}
  pub fn create_constrained_fp_call() {}
  pub fn create_select() {}
  pub fn create_vaarg() {}
  pub fn create_extract_element() {}
  pub fn create_insert_element() {}
  pub fn create_shuffle_vector() {}
  pub fn create_extract_value() {}
  pub fn create_insert_value() {}
  pub fn create_landing_pad() {}
  pub fn create_is_null() {}
  pub fn create_is_not_null() {}
  pub fn create_is_not_neg() {}
  pub fn create_ptr_diff() {}
  pub fn create_launder_invariant_group() {}
  pub fn create_string_invariant_group() {}
  pub fn create_vector_reverse() {}
  pub fn create_vector_splice() {}
  pub fn create_vector_splat() {}
  pub fn create_extract_integer() {}
  pub fn create_preserve_array_access_index() {}
  pub fn create_preserve_union_access_index() {}
  pub fn create_preserve_struct_access_index() {}
}