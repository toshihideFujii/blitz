#![allow(dead_code)]

// This file contains the declaration of the Constant class.

use super::{
  type_::{Type},
  constants::{ConstantInt, ConstantFP, ConstantAggregateZero,
  ConstantPointerNull, ConstantTokenNone, ConstantTargetNone}
};

enum PossibleRelocationsType {
  NoRelocation,
  LocalRelocation,
  GlobalRelocation
}

// This is an important base class in Blitz.
// It provides the common facilities of all constant values in
// an Blitz program. A constant is a value that is immutable at
// runtime. Functions are constants because their address is immutable.
// Same with global variables.
//
// All constants share the capabilities provided in this class.
// All constants can have a null value. They can have an operand list.
// Constants can be simple (integer and floating point values), complex
// (arrays and structures), or expression based (computations yielding
// a constant value composed of only certain operators and other constant
// values).
//
// Note that Constants are immutable (once created they never change)
// and are fully shared by structural equivalene.
// This means that two structurally equivalent constants will always have
// the same address.
// Constants are created on demend as needed and never deleted: this clients
// don't have to warry about the lifetime of the objects.
struct Constant {
  v_type: Box<dyn Type>
}

impl Constant {
  pub fn new() {}
  
  // Return true if this is the value that would be returned by get_null_value().
  pub fn is_null_value(&self) -> bool {
    // 0 is null.
    let const_int =
      self.v_type.as_any().downcast_ref::<ConstantInt>();
    if const_int.is_some() {
      return const_int.unwrap().is_zero();
    }
    // +0.0 is null.
    let const_fp =
      self.v_type.as_any().downcast_ref::<ConstantFP>();
    if const_fp.is_some() {
      return const_fp.unwrap().is_exactly_value(0.0);
    }
    // Constant zero is zero for aggregates, cpnull is null for pointets,
    // none for tokens.
    if self.v_type.as_any().downcast_ref::<ConstantAggregateZero>().is_some() ||
       self.v_type.as_any().downcast_ref::<ConstantPointerNull>().is_some() ||
       self.v_type.as_any().downcast_ref::<ConstantTokenNone>().is_some() ||
       self.v_type.as_any().downcast_ref::<ConstantTargetNone>().is_some()
    {
      return true;
    }
    false
  }

  // Return true if the value is one.
  pub fn is_one_value(&self) -> bool {
    // Check for 1 integers.
    let const_int =
      self.v_type.as_any().downcast_ref::<ConstantInt>();
    if const_int.is_some() {
      return const_int.unwrap().is_one();
    }
    false
  }

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