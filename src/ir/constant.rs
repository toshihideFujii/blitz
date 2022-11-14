#![allow(dead_code)]

// This file contains the declaration of the Constant class.

enum PossibleRelocationsType {
  NoRelocation,
  LocalRelocation,
  GlobalRelocation
}

struct Constant {}

impl Constant {
  pub fn is_null() {}

  pub fn is_one_value() {}

  pub fn is_not_one_value() {}

  pub fn is_all_ones_value() {}

  pub fn is_negative_zero_value() {}

  pub fn is_zero_value() {}

  pub fn is_not_min_signed_value() {}

  pub fn is_min_signed_value() {}

  pub fn is_finite_non_zero_fp() {}

  pub fn is_normal_fp() {}

  pub fn has_exact_inverse_fp() {}

  pub fn is_nan() {}

  pub fn is_element_wise_equal() {}

  pub fn contains_undef_or_poison_element() {}

  pub fn contains_poison_element() {}

  pub fn contains_constant_expression() {}

  pub fn is_thread_dependent() {}

  pub fn is_dll_import_dependent() {}

  pub fn is_constant_used() {}

  pub fn needs_relocation() {}

  pub fn needs_dynamic_relocation() {}

  pub fn get_aggregate_element() {}

  pub fn get_splat_value() {}

  pub fn get_unique_integer() {}

  pub fn destroy_constant() {}

  pub fn handle_operand_change() {}

  pub fn get_null_value() {}

  pub fn get_integer_value() {}

  pub fn remove_dead_constant_users() {}

  pub fn has_one_live_uses() {}

  pub fn strip_pointer_casts() {}

  pub fn replace_undefs_with() {}

  pub fn merge_undefs_with() {}

  pub fn is_manifest_constant() {}

  fn get_relocation_info() {}

  fn has_n_live_uses() {}
}

pub fn class_of() {}