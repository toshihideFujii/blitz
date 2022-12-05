#![allow(dead_code)]

/*
This file contains the declaration of the Function
class, which represents a single function/procedure.
*/

#[derive(PartialEq, Clone)]
enum ProfileCountType {
  Real,
  Sunthetic
}

// Class to represent profile counts.
struct ProfileCount {
  count_: u64,
  pct_: ProfileCountType
}

impl ProfileCount {
  pub fn new() {}

  pub fn get_count(&self) -> u64 {
    self.count_
  }

  pub fn get_type(&self) -> ProfileCountType {
    self.pct_.clone()
  }

  pub fn is_synthetic(&self) -> bool {
    self.pct_ == ProfileCountType::Sunthetic
  }
}

struct Function {}

impl Function {
  pub fn get_function() {}

  pub fn get_instruction_count() {}

  pub fn get_function_type() {}

  pub fn get_return_type() {}

  pub fn get_context() {}

  pub fn is_var_arg() {}

  pub fn is_materializable() {}

  pub fn set_is_materializable() {}

  pub fn get_intrinsic_id() {}

  pub fn is_intrinsic() {}

  pub fn is_target_intrinsic() {}

  pub fn is_constrained_fp_intrinsic() {}

  pub fn lookup_intrinsic_id() {}

  pub fn recalculate_intrinsic_id() {}

  pub fn get_calling_conv() {}

  pub fn set_calling_conv() {}

  pub fn set_entry_count() {}

  pub fn get_entry_count() {}

  pub fn has_profile_data() {}

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