#![allow(dead_code)]

// This family of functions identifies calls to builtin functions
// that allocate or free memory.

pub fn is_allocation_fn() {}
pub fn is_new_like_fn() {}
pub fn is_malloc_or_calloc_like_fn() {}
pub fn is_alloc_like_fn() {}
pub fn is_realloc_like_fn() {}
pub fn get_reallocated_operand() {}
pub fn is_lib_free_function() {}
pub fn get_freed_operand() {}
pub fn is_removable_alloc() {}
pub fn get_alloc_alignment() {}
pub fn get_alloc_size() {}
pub fn get_initial_valueof_allocation() {}
pub fn get_allocation_family() {}

struct ObjectsizeOpts {}

pub fn get_object_size() {}
pub fn lower_object_size_call() {}

struct ObjectSizeOffsetVisitor {}
impl ObjectSizeOffsetVisitor {
  pub fn new() {}
  pub fn compute() {}
  pub fn known_size() {}
  pub fn known_offset() {}
  pub fn both_known() {}
  pub fn visit_alloca_inst() {}
  pub fn visit_argument() {}
  pub fn visit_call_base() {}
  pub fn visit_constant_pointer_null() {}
  pub fn visit_extract_element_inst() {}
  pub fn visit_extract_value_inst() {}
  pub fn visit_global_alias() {}
  pub fn visit_global_variable() {}
  pub fn visit_int_to_ptr_inst() {}
  pub fn visit_load_inst() {}
  pub fn visit_phi_node() {}
  pub fn visit_select_inst() {}
  pub fn visit_undef_value() {}
  pub fn visit_instruction() {}

  fn find_load_size_offset() {}
  fn combine_size_offset() {}
  fn compute_impl() {}
  fn checked_zext_or_trunc() {}
}

struct ObjectSizeOffsetEvaluator {}
impl ObjectSizeOffsetEvaluator {
  pub fn new() {}
  pub fn unknown() {}
  pub fn compute() {}
  pub fn known_size() {}
  pub fn known_offset() {}
  pub fn any_known() {}
  pub fn both_known() {}

  pub fn visit_alloca_inst() {}
  pub fn visit_call_base() {}
  pub fn visit_extract_element_inst() {}
  pub fn visit_extract_value_inst() {}
  pub fn visit_gep_operator() {}
  pub fn visit_int_to_ptr_inst() {}
  pub fn visit_load_inst() {}
  pub fn visit_phi_node() {}
  pub fn visit_select_inst() {}
  pub fn visit_instruction() {}
}