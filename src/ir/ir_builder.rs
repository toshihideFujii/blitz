#![allow(dead_code)]

// This file defines the IRBuilder class, which is used as a
// convenient way to create Blitz instructions with a consistent
// and simplified interface.

use crate::{adt::{ap_int::APInt, twine::Twine}, support::alignment::MaybeAlign};
use super::{constants::ConstantInt, blits_context::{blits_context_mut, blits_context},
  type_::{IntegerType, self, BasicType, PointerType, Type}, data_layout::DataLayout,
  value::Value, metadata::MDNode, instruction::{OpCode, Instruction}, basic_block::BasicBlock, global_value::IntrinsicID, /*instruction::InstructionBase,
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
  bb: Option<BasicBlock>,
  insert_pt: Option<Box<dyn Instruction>>
}

impl IRBuilder {
  pub fn new() {}
  pub fn insert_inst(&self) {}
  pub fn insert_val(&self, _v: &dyn Value, _name: &Twine) {
    //let i = v.as_any().downcast_ref::<dyn Instruction>();
  }

  // Clear the insertion point: created instructions will not be inserted
  // into a block.
  pub fn clear_insertion_point(&mut self) {
    self.bb = None;
    self.insert_pt = None;
  }

  pub fn get_insert_block(&self) -> &Option<BasicBlock> {
    &self.bb
  }

  // This specifies that created instructions shoould be inserted to the
  // end of the specified block.
  pub fn set_insert_point(&mut self, the_bb: BasicBlock) {
    self.bb = Some(the_bb);
    //self.insert_pt = Some(the_bb);
  }

  // This specifies that created instructions shoould be inserted before
  // the specified instruction.
  pub fn set_insert_point_at_inst(&mut self, _i: &dyn Instruction) {}

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
  pub fn get_true(&self) -> ConstantInt {
    ConstantInt::get_true(blits_context_mut()).unwrap()
  }

  // Get the constant value for i1 false.
  pub fn get_false(&self) -> ConstantInt {
    ConstantInt::get_false(blits_context_mut()).unwrap()
  }

  // Get a constant value representing either true or false.
  pub fn get_int_1(&self, v: bool) -> ConstantInt {
    ConstantInt::get(&self.get_int_1_type(), v as i64, false)
  }

  // Get a constant 8-bit value.
  pub fn get_int_8(&self, c: u8) -> ConstantInt {
    ConstantInt::get(&self.get_int_8_type(), c as i64, false)
  }

  // Get a constant 16-bit value.
  pub fn get_int_16(&self, c: u16) -> ConstantInt {
    ConstantInt::get(&self.get_int_16_type(), c as i64, false)
  }

  // Get a constant 32-bit value.
  pub fn get_int_32(&self, c: u32) -> ConstantInt {
    ConstantInt::get(&self.get_int_32_type(), c as i64, false)
  }

  // Get a constant 64-bit value.
  pub fn get_int_64(&self, c: u64) -> ConstantInt {
    ConstantInt::get(&self.get_int_64_type(), c as i64, false)
  }

  // Get a constant n-bit value, zero extended or truncated from a 64-bit value.
  pub fn get_int_n(&self, n: u32, c: i64) -> ConstantInt {
    ConstantInt::get(&self.get_int_n_type(n), c, false)
  }

  // Get a constant integer value.
  pub fn get_int(&self, apint: &APInt) -> ConstantInt {
    ConstantInt::get_from_apint(blits_context_mut(), apint.clone())
  }

  // Fetch the type representing a single bit.
  pub fn get_int_1_type(&self) -> IntegerType {
    type_::get_int_1_type(blits_context_mut())
  }

  // Fetch the type representing an 8-bit integer.
  pub fn get_int_8_type(&self) -> IntegerType {
    type_::get_int_8_type(blits_context_mut())
  }

  // Fetch the type representing a 16-bit integer.
  pub fn get_int_16_type(&self) -> IntegerType {
    type_::get_int_16_type(blits_context_mut())
  }

  // Fetch the type representing a 32-bit integer.
  pub fn get_int_32_type(&self) -> IntegerType {
    type_::get_int_32_type(blits_context_mut())
  }

  // Fetch the type representing a 64-bit integer.
  pub fn get_int_64_type(&self) -> IntegerType {
    type_::get_int_64_type(blits_context_mut())
  }

  // Fetch the type representing a 128-bit integer.
  pub fn get_int_128_type(&self) -> IntegerType {
    type_::get_int_128_type(blits_context_mut())
  }

  // Fetch the type representing an n-bit integer.
  pub fn get_int_n_type(&self, n: u32) -> IntegerType {
    type_::get_int_n_type(blits_context_mut(), n)
  }

  // Fetch the type representing a 16-bit floating point value.
  pub fn get_half_type(&self) -> BasicType {
    type_::get_half_type(blits_context())
  }

  // Fetch the type representing a 16-bit brain floating point value.
  pub fn get_b_float_type(&self) -> BasicType {
    type_::get_b_float_type(&blits_context())
  }

  // Fetch the type representing a 32-bit floating point value.
  pub fn get_float_type(&self) -> BasicType {
    type_::get_float_type(blits_context())
  }

  // Fetch the type representing a 64-bit floating point value.
  pub fn get_double_type(&self) -> BasicType {
    type_::get_double_type(blits_context())
  }

  // Fetch the type representing void.
  pub fn get_void_type(&self) -> BasicType {
    type_::get_void_type(blits_context())
  }

  // Fetch the type representing a pointer.
  pub fn get_ptr_type(&self) {}

  // Fetch the type representing a pointer to an 8-bit integer value.
  pub fn get_int_8_ptr_type(&self, address_space: usize) -> PointerType {
    type_::get_int_8_ptr_type(blits_context(), address_space)
  }

  // Fetch the type of an integer with size at least as big as that of a pointer
  // in the given address space.
  pub fn get_int_ptr_type(&self, _dl: &DataLayout, _address_space: usize) {
    //dl.get_int_ptr_type(t)
  }

  // Fetch the type of an integer that should be used to index GEP operations
  // within address_space.
  pub fn get_index_type(&self, dl: &DataLayout, address_space: usize) -> IntegerType {
    dl.get_index_type(address_space)
  }

  // Create and insert a memset to the specified pointer and the specified value.
  pub fn create_mem_set(&self, _ptr: &dyn Value, _val: &dyn Value, size: u64,
    _align: MaybeAlign, _is_volatile: bool, _t_baa_tag: Option<&dyn MDNode>,
    _scope_tag: Option<&dyn MDNode>, _no_alias_tag: Option<&dyn MDNode>)
  {
    let _int_size = self.get_int_64(size);
  }

  pub fn create_element_unordered_atomic_mem_set() {}

  // Create and insert a memcpy between the specified pointers.
  pub fn create_mem_cpy() {}
  pub fn create_mem_transfer_inst() {}
  pub fn create_element_unordered_atomic_mem_cpy() {}
  pub fn create_element_unordered_atomic_mem_move() {}

  fn get_reduction_intrinsic(&self, _id: IntrinsicID, src: &dyn Value) {
    let _m = self.get_insert_block().as_ref().unwrap().
      get_parent().as_ref().unwrap().get_parent();
    let _ops = vec![src];
    let _types = vec![src.get_type()];
  }

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

  pub fn create_cast(&self, _op: OpCode, _v: &dyn Value, _dst_t: &dyn Type,
    _name: Twine) -> Option<&dyn Value>
  {
    //if v.get_type().get_type_id() == dst_t.get_type_id() {
      //return v;
    //}
    None
  }

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



  fn get_casted_int_8_ptr_value(&self, val: &dyn Value) -> Option<PointerType> {
    let ptr = val.as_any().downcast_ref::<PointerType>().unwrap().clone();
    if ptr.is_opaque_or_pointee_type_matches(&self.get_int_8_type()) {
      return Some(ptr);
    }
    None
  }
}