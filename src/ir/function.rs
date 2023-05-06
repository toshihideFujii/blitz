#![allow(dead_code)]

// This file contains the declaration of the Function class,
// which represents a single function/procedure.

use crate::adt::string_ref::StringRef;
use super::{
  type_::{FunctionType, Type},
  blits_context::BlitzContext,
  value::{ValueType, Value}, calling_conv::*
};

#[derive(PartialEq, Clone)]
pub enum ProfileCountType {
  Real,
  Sunthetic
}

// Class to represent profile counts.
// This class represents both real and synthetic profile counts.
pub struct ProfileCount {
  count: u64,
  pct: ProfileCountType
}

impl ProfileCount {
  pub fn new(count: u64, pct: ProfileCountType) -> Self {
    ProfileCount { count: count, pct: pct }
  }

  pub fn get_count(&self) -> u64 {
    self.count
  }

  pub fn get_type(&self) -> ProfileCountType {
    self.pct.clone()
  }

  pub fn is_synthetic(&self) -> bool {
    self.pct == ProfileCountType::Sunthetic
  }
}

#[derive(Debug)]
pub struct Function {
  v_type: FunctionType,
  v_id: ValueType,
  sub_class_data: u32,
  int_id: u32,
  has_blitz_reserved_name: bool
}

impl Function {
  pub fn get_function(&self) -> &Function {
    self
  }

  pub fn get_instruction_count(&self) -> u32 {
    0
  }
  
  // Return the FunctionType for me.
  pub fn get_function_type(&self) -> &FunctionType{
    &self.v_type
  }

  // Returns the type of the ret val.
  pub fn get_return_type(&self) -> &Box<dyn Type> {
    self.v_type.get_return_type()
  }
  
  // Return a reference to the BlitzContext associated with this function.
  pub fn get_context(&self) -> &BlitzContext {
    self.v_type.get_context()
  }

  // Return true if this function takes a variable number of arguments.
  pub fn is_var_arg(&self) -> bool {
    self.get_function_type().is_var_arg()
  }

  pub fn is_materializable() {}
  pub fn set_is_materializable() {}

  // This method returns the ID number of the specified function, or
  // Intrinsic::not_intrinsic if the function is not an intrinsic, or
  // if the pointer is null.
  pub fn get_intrinsic_id(&self) -> u32 {
    self.int_id
  }

  // Return true if the function's name starts with "blitz.".
  pub fn is_intrinsic(&self) -> bool {
    self.has_blitz_reserved_name
  }

  // Return true if int_id is an intrinsic specific to a certain target.
  pub fn is_target_intrinsic_id(_int_id: u32) -> bool {
    false
  }

  // Return true if this function is an intrinsic and the intrinsic is
  // specific to a certain target.
  pub fn is_target_intrinsic(&self) -> bool {
    Function::is_target_intrinsic_id(self.int_id)
  }

  pub fn is_constrained_fp_intrinsic(&self) -> bool {
    false
  }

  pub fn lookup_intrinsic_id(_nmae: StringRef) -> u32 {
    0
  }

  // Recalculate the id for this function if it is an intrinsic defined in
  // blitz/intrinsics.rs.
  pub fn recalculate_intrinsic_id(&mut self) {
    let name = self.get_name();
    if !name.starts_with("blitz.") {
      self.has_blitz_reserved_name = false;
      self.int_id = 0;
      return;
    }
    self.has_blitz_reserved_name = true;
    self.int_id = Function::lookup_intrinsic_id(name);
  }

  // Get the calling convention of this function.
  pub fn get_calling_conv(&self) -> u32 {
    (self.get_subclass_data_from_value() >> 4) & CALLING_CONV_MAX_ID
  }

  // Set the calling convention of this function.
  pub fn set_calling_conv(&mut self, cc: u32) {
    let val = (self.get_subclass_data_from_value() & 0xc00f) | (cc << 4);
    self.set_value_subclass_data(val);
  }

  pub fn set_entry_count() {}

  // Get the entry count for this function.
  // Entry count is the number of times the function was executed.
  // When allow_synthetic is false, only pgo_data will be returned.
  pub fn get_entry_count(&self, _allow_synthetic: bool) -> Option<ProfileCount> {
    None
  }

  // Return true if the function is annotated with profile data.
  pub fn has_profile_data(&self, include_synthetic: bool) -> bool {
    self.get_entry_count(include_synthetic).is_some()
  }

  pub fn get_import_guids() {}
  pub fn set_section_prefix() {}
  pub fn has_gc() {}
  pub fn get_gc() {}
  pub fn set_gc() {}
  pub fn clear_gc() {}
  pub fn get_attributes() {}
  pub fn set_attributes() {}
  pub fn add_attribute_at_index() {}
  pub fn add_fn_attr() {}
  pub fn add_fn_attrs() {}
  pub fn add_ret_attr() {}
  pub fn add_ret_attrs() {}
  pub fn add_param_attr() {}
  pub fn add_param_attrs() {}
  pub fn remove_attribute_at_index() {}
  pub fn remove_fn_attr() {}
  pub fn remove_fn_attrs() {}
  pub fn remove_ret_attr() {}
  pub fn remove_ret_attrs() {}
  pub fn remove_param_attr() {}
  pub fn remove_param_attrs() {}
  pub fn has_fn_atribute() {}
  pub fn has_ret_attribute() {}
  pub fn has_param_attribute() {}
  pub fn get_attribute_at_index() {}
  pub fn get_fn_attribute() {}
  pub fn get_param_attribute() {}
  pub fn remove_param_undef_implying_attrs() {}
  pub fn get_fn_stack_align() {}
  pub fn has_stack_protector_fn_attr() {}
  pub fn add_dereferenceable_param_attr() {}
  pub fn add_dereferenceable_or_null_param_attr() {}
  pub fn get_param_alignment() {}
  pub fn get_param_align() {}
  pub fn get_param_stack_align() {}
  pub fn get_param_by_val_type() {}
  pub fn get_param_struct_ret_type() {}
  pub fn get_param_in_alloca_type() {}
  pub fn get_param_by_ref_type() {}
  pub fn get_param_preallocated_type() {}
  pub fn get_param_dereferenceable_bytes() {}
  pub fn get_param_dereferenceable_or_null_bytes() {}
  pub fn is_presplit_coroutine() {}
  pub fn set_presplit_coroutine() {}
  pub fn set_splitted_coroutine() {}
  pub fn does_not_access_memory() {}
  pub fn set_does_not_access_memory() {}
  pub fn only_reads_memory() {}
  pub fn set_only_reads_memory() {}
  pub fn only_writes_memory() {}
  pub fn set_only_writes_memory() {}
  pub fn only_accesses_arg_memory() {}
  pub fn set_only_accesses_arg_memory() {}
  pub fn only_accesses_inaccessible_memory() {}
  pub fn set_only_accesses_inaccessible_memory() {}
  pub fn only_accesses_inaccessible_mem_or_arg_mem() {}
  pub fn set_only_accesses_inaccessible_mem_or_arg_mem() {}
  pub fn does_not_return() {}
  pub fn set_does_not_return() {}
  pub fn does_no_cf_check() {}
  pub fn does_not_throw() {}
  pub fn set_does_not_throw() {}
  pub fn cannot_duplicate() {}
  pub fn set_cannot_duplicate() {}
  pub fn is_convergent() {}
  pub fn set_convergent() {}
  pub fn set_not_convergent() {}
  pub fn is_speculatable() {}
  pub fn set_speculatable() {}
  pub fn does_not_free_memory() {}
  pub fn set_does_not_free_memory() {}
  pub fn has_no_sync() {}
  pub fn set_no_sync() {}
  pub fn does_not_recurse() {}
  pub fn set_does_not_recurse() {}
  pub fn must_progress() {}
  pub fn set_must_progress() {}
  pub fn will_return() {}
  pub fn set_will_return() {}
  pub fn get_uw_table_kind() {}
  pub fn has_uw_table() {}
  pub fn set_uw_table_kind() {}
  pub fn needs_unwind_table_entry() {}
  pub fn has_struct_ret_attr() {}
  pub fn return_does_not_alias() {}
  pub fn set_return_does_not_alias() {}
  pub fn has_opt_none() {}
  pub fn has_min_size() {}
  pub fn has_opt_size() {}
  pub fn get_denormal_mode() {}
  pub fn copy_attributes_from() {}
  pub fn delete_body() {}
  pub fn remove_from_parent() {}
  pub fn earse_from_parent() {}
  pub fn steal_argument_list_from() {}
  pub fn get_basic_block_list() {}
  pub fn get_sublist_access() {}
  pub fn get_entry_block() {}
  pub fn get_value_symbol_table() {}

  pub fn begin() {}
  pub fn end() {}
  pub fn size() {}
  pub fn empty() {}
  pub fn front() {}
  pub fn back() {}

  pub fn arg_begin() {}
  pub fn arg_end() {}
  pub fn get_arg() {}
  pub fn args() {}
  pub fn arg_size() {}
  pub fn arg_empty() {}

  pub fn has_personality_fn() {}
  pub fn get_personality_fn() {}
  pub fn set_personality_fn() {}

  pub fn has_prefix_data() {}
  pub fn get_prefix_data() {}
  pub fn set_prefix_data() {}

  pub fn has_prologue_data() {}
  pub fn get_prologue_data() {}
  pub fn set_prologue_data() {}

  pub fn print() {}

  pub fn view_cfg() {}
  pub fn view_cfg_only() {}

  pub fn drop_all_references() {}
  pub fn has_address_taken() {}
  pub fn is_def_trivially_dead() {}
  pub fn calls_function_that_returns_twice() {}
  pub fn set_sub_program() {}
  pub fn is_debug_info_for_profiling() {}
  pub fn null_pointer_is_defined() {}
}

impl Value for Function {
  fn get_type(&self) -> &dyn Type {
    &self.v_type
  }

  fn get_context(&self) -> &BlitzContext {
    self.v_type.get_context()
  }

  fn get_value_id(&self) -> ValueType {
    self.v_id.clone()
  }

  fn get_subclass_data_from_value(&self) -> u32 {
    self.sub_class_data
  }

  fn set_value_subclass_data(&mut self, val: u32) {
    self.sub_class_data = val;
  }
}