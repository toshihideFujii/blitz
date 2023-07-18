#![allow(dead_code)]

// This file defines a DIBulder that is useful for creating
// debugging information entries in IR form.

use crate::{adt::{string_ref::StringRef, ap_int::APInt},
/*ir::debug_info_metadata::DIEnumerator*/};

use super::{module::Module, blits_context::BlitzContext,
  debug_info_metadata::DICompileUnit, function::Function};

struct DIBulder {
  m: Module,
  c: BlitzContext,
  cu_node: DICompileUnit,
  declare_fn: Function,
  value_fn: Function,
  label_fn: Function,
  assign_fn: Function,
  allow_unresolved_nodes: bool
}

impl DIBulder {
  pub fn new() {}
  pub fn track_if_unresolved() {}
  pub fn insert_declare() {}
  pub fn insert_label() {}
  pub fn insert_dbg_intrinsic() {}
  pub fn insert_dbg_value_intrinsic() {}
  pub fn finalize() {}
  pub fn finalize_subprogram() {}
  pub fn create_compile_unit() {}
  pub fn create_file() {}
  pub fn create_macro() {}
  pub fn create_temp_macro_file() {}

  // Create a single enumerator value.
  pub fn create_enumarator(&self, name: StringRef, _value: APInt) /*-> DIEnumerator*/ {
    debug_assert!(!name.empty(), "Unable to create enumerator without name.");
    //DIEnumerator::get(self.c, value, false, name) // TODO
  }

  pub fn create_unspecified_type() {}
  pub fn create_null_ptr_type() {}
  pub fn create_basic_type() {}

  // Create debugging information entry for a string.
  pub fn create_string_type(&self, name: StringRef, _size_in_bits: u64) {
    debug_assert!(!name.empty(), "Unable to create enumerator without name.");
  }
  
  pub fn create_qualified_type() {}
  pub fn create_pointer_type() {}
  pub fn create_member_pointer_type() {}
  pub fn create_reference_type() {}
  pub fn create_typedef() {}
  pub fn create_friend() {}
  pub fn create_inheritance() {}
  pub fn create_member_type() {}
  pub fn create_variant_member_type() {}
  pub fn create_bitfield_member_type() {}
  pub fn create_static_member_type() {}
  pub fn create_class_type() {}
  pub fn create_struct_type() {}
  pub fn create_union_type() {}
  pub fn create_template_type_parameter() {}
  pub fn create_template_value_parameter() {}
  pub fn create_template_template_parameter() {}
  pub fn create_template_template_parameter_pack() {}
  pub fn create_array_type() {}
  pub fn create_vector_type() {}
  pub fn create_enumeration_type() {}
  pub fn create_set_type() {}
  pub fn create_subroutine_type() {}
  pub fn create_aratificial_subprogram() {}
  pub fn create_artificial_type() {}
  pub fn create_forward_decl() {}
  pub fn create_replaceable_composite_type() {}
  pub fn retain_type() {}
  pub fn create_unspecified_parameter() {}
  pub fn get_or_create_array() {}
  pub fn get_or_create_macro_array() {}
  pub fn get_or_create_type_array() {}
  pub fn get_or_create_subrange() {}
  pub fn get_or_create_generic_subrange() {}
  pub fn create_global_variable_expression() {}
  pub fn create_temp_global_variable_fwd_decl() {}
  pub fn create_auto_variable() {}
  pub fn create_label() {}
  pub fn create_parameter_variable() {}
  pub fn create_expression() {}
  pub fn create_constant_value_expression() {}
  pub fn create_function() {}
  pub fn create_temp_function_fwd_decl() {}
  pub fn create_method() {}
  pub fn create_common_block() {}
  pub fn create_namespace() {}
  pub fn create_module() {}
  pub fn create_lexical_block_file() {}
  pub fn create_lexical_block() {}
  pub fn create_imported_module() {}
  pub fn create_imported_declaration() {}
  pub fn insert_dbg_assign() {}
  pub fn replace_arrays() {}
  pub fn replace_temporary() {}
}