#![allow(dead_code)]

// This file contains the declaration of the Constant class.

use super::{
  type_::{Type, FixedVectorType, /*TypeID, IntegerType,*/ /*IntegerType*/},
  constants::{ConstantFP, ConstantAggregateZero,
  ConstantPointerNull, ConstantTokenNone, ConstantTargetNone},
  value::ValueType,
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
#[derive(Debug)]
pub struct ConstantTemp {
  v_type: Box<dyn Type>,
  v_id: ValueType,
  use_list: Vec<u64>, // TODO: Use
  num_user_operands: u32
}

impl ConstantTemp {
  
  // Return true if this is the value that would be returned by get_null_value().
  pub fn is_null_value(&self) -> bool {
    // Constant zero is zero for aggregates, cpnull is null for pointets, none for tokens.
    if self.v_type.as_any().downcast_ref::<ConstantAggregateZero>().is_some() ||
       self.v_type.as_any().downcast_ref::<ConstantPointerNull>().is_some() ||
       self.v_type.as_any().downcast_ref::<ConstantTokenNone>().is_some() ||
       self.v_type.as_any().downcast_ref::<ConstantTargetNone>().is_some()
    { return true; }

    false
  }

  // Return true if the value is one.
  pub fn is_one_value(&self) -> bool {
    // Check for constant splat vectors of 1 values.
    if self.v_type.as_ref().is_vector_type() {
      let splat_val = self.get_splat_value(false);
      if splat_val.is_some() { return splat_val.unwrap().is_one_value(); }
    }

    false
  }

  // Return true if the value is not one value, or, for vectors,
  // does not contain one value elements.
  pub fn is_not_one_value(&self) -> bool {
    !self.is_one_value()
  }

  // Return true if this is the value that would be returned by
  // get_all_ones_value().
  pub fn is_all_ones_value(&self) -> bool {
    // Check for constant splat vectors of 1 values.
    if self.v_type.as_ref().is_vector_type() {
      let splat_val = self.get_splat_value(false);
      if splat_val.is_some() { return splat_val.unwrap().is_all_ones_value(); }
    }

    false
  }

  // Return true if the value is what would be returned by
  // get_zero_value_for_negation().
  pub fn is_negative_zero_value(&self) -> bool {
    // Equivalent for a vector of -0.0's.
    if self.v_type.as_ref().is_vector_type() {
      let splat_val = self.get_splat_value(false);
      if splat_val.is_some() { return splat_val.unwrap().is_negative_zero_value(); }
    }

    // We've already handled true fp case; any other fp vectors can't represent -0.0.
    if self.v_type.is_fp_or_fpvector_type() { return false; }

    false
  }

  // Return true if the value is negative zero or null value.
  pub fn is_zero_value(&self) -> bool {
    // Check for constant splat vectors of 0 values.
    if self.v_type.as_ref().is_vector_type() {
      let splat_val = self.get_splat_value(false);
      if splat_val.is_some() { return splat_val.unwrap().is_zero_value(); } // <- OK?
    }

    // Otherwise, just use +0.0.
    self.is_null_value()
  }

  // Return true if the value is not the smallest signed value, or,
  // for vectors, does not contain smallest signed value.
  pub fn is_not_min_signed_value(&self) -> bool {
    // Check thst vectors don't contain INT_MIN.
    let vec = self.v_type.as_any().downcast_ref::<FixedVectorType>();
    if vec.is_some() {
      for i in 0..vec.unwrap().get_num_elements() {
        let elt = self.get_aggregate_element(i as u32);
        if elt.is_none() || !elt.unwrap().is_not_min_signed_value() {
          return false;
        }
      }
      return true;
    }

    // Check for splats that aren't INT_MIN.
    if self.v_type.as_ref().is_vector_type() {
      let splat_val = self.get_splat_value(false);
      if splat_val.is_some() { return splat_val.unwrap().is_not_min_signed_value(); }
    }

    false
  }

  // Return true if the value is the smallest signed value.
  pub fn is_min_signed_value(&self) -> bool {
    !self.is_not_min_signed_value()
  }

  // Return true if this is a finite and non-zero floating-point scalar
  // constant or a fixed width vector constant with all finite and non--zero
  // elements.
  pub fn is_finite_non_zero_fp(&self) -> bool {
    let vec = self.v_type.as_any().downcast_ref::<FixedVectorType>();
    if vec.is_some() {
      for i in 0..vec.unwrap().get_num_elements() {
        let elt = self.get_aggregate_element(i as u32);
        if elt.is_none() { return false; }
        let temp = elt.unwrap();
        let cfp = temp.v_type.as_any().downcast_ref::<ConstantFP>();
        if cfp.is_none() || !cfp.unwrap().get_value_apf().is_finite_non_zero() {
          return false;
        }
      }
      return true;
    }

    if self.v_type.as_ref().is_vector_type() {
      let splat_val = self.get_splat_value(false);
      if splat_val.is_some() { return splat_val.unwrap().is_finite_non_zero_fp(); }
    }

    false
  }

  // Return true if this is a normal (as opposed to denormal, infinitym nan,
  // or zero) floating-point scalar constant or a vector constant with all
  // normal elements.
  pub fn is_normal_fp(&self) -> bool {
    let vec = self.v_type.as_any().downcast_ref::<FixedVectorType>();
    if vec.is_some() {
      for i in 0..vec.unwrap().get_num_elements() {
        let elt = self.get_aggregate_element(i as u32);
        if elt.is_none() { return false; }
        let temp = elt.unwrap();
        let cfp = temp.v_type.as_any().downcast_ref::<ConstantFP>();
        if cfp.is_none() || !cfp.unwrap().get_value_apf().is_normal() {
          return false;
        }
      }
      return true;
    }

    if self.v_type.as_ref().is_vector_type() {
      let splat_val = self.get_splat_value(false);
      if splat_val.is_some() { return splat_val.unwrap().is_normal_fp(); }
    }

    false
  }

  // Return true if this scalar has an exact multiplicative inverse or this
  // vector has an exact multiplicative inverse for each element in the vector.
  pub fn has_exact_inverse_fp(&self) -> bool {
    let vec = self.v_type.as_any().downcast_ref::<FixedVectorType>();
    if vec.is_some() {
      for i in 0..vec.unwrap().get_num_elements() {
        let elt = self.get_aggregate_element(i as u32);
        if elt.is_none() { return false; }
        let temp = elt.unwrap();
        let cfp = temp.v_type.as_any().downcast_ref::<ConstantFP>();
        if cfp.is_none() || !cfp.unwrap().get_value_apf().get_exact_inverse(None) {
          return false;
        }
      }
      return true;
    }

    if self.v_type.as_ref().is_vector_type() {
      let splat_val = self.get_splat_value(false);
      if splat_val.is_some() { return splat_val.unwrap().has_exact_inverse_fp(); }
    }

    false
  }

  // Return true if this is a floating-point NaN constant or a vector
  // floating-point constant with all NaN elements.
  pub fn is_nan(&self) -> bool {
    let vec = self.v_type.as_any().downcast_ref::<FixedVectorType>();
    if vec.is_some() {
      for i in 0..vec.unwrap().get_num_elements() {
        let elt = self.get_aggregate_element(i as u32);
        if elt.is_none() { return false; }
        let temp = elt.unwrap();
        let cfp = temp.v_type.as_any().downcast_ref::<ConstantFP>();
        if cfp.is_none() || !cfp.unwrap().is_nan() {
          return false;
        }
      }
      return true;
    }

    if self.v_type.as_ref().is_vector_type() {
      let splat_val = self.get_splat_value(false);
      if splat_val.is_some() { return splat_val.unwrap().is_nan(); }
    }

    false
  }

  // Return true if this constant and a constant 'y' are element-wise equal.
  pub fn is_element_wise_equal(&self) -> bool { false }

  // For aggregates (struct/array/vector) return the constant that corresponds
  // to the specified element if possible, or null if not.
  pub fn get_aggregate_element(&self, _elt: u32) -> Option<ConstantTemp> {
    None
  }

  // If all elements of the vector constant have the same value,
  // return that value. Otherwise, return None.
  pub fn get_splat_value(&self, _allow_undefs: bool) -> Option<ConstantTemp> {
    None
  }
}

pub trait Constant {
    
  // Return true if this is the value that would be returned by get_null_value().
  fn is_null_value(&self) -> bool { false }

  // Return true if the value is one.
  fn is_one_value(&self) -> bool { false }

  // Return true if the value is not one value, or, for vectors,
  // does not contain one value elements.
  fn is_not_one_value(&self) -> bool {
    !self.is_one_value()
  }

  // Return true if this is the value that would be returned by
  // get_all_ones_value().
  fn is_all_ones_value(&self) -> bool { false }

  // Return true if the value is what would be returned by
  // get_zero_value_for_negation().
  fn is_negative_zero_value(&self) -> bool { false }

  // Return true if the value is negative zero or null value.
  fn is_zero_value(&self) -> bool { false }

  // Return true if the value is not the smallest signed value, or,
  // for vectors, does not contain smallest signed value.
  fn is_not_min_signed_value(&self) -> bool { false }

  // Return true if the value is the smallest signed value.
  fn is_min_signed_value(&self) -> bool { false }

  // Return true if this is a finite and non-zero floating-point scalar
  // constant or a fixed width vector constant with all finite and non--zero
  // elements.
  fn is_finite_non_zero_fp(&self) -> bool { false }

  // Return true if this is a normal (as opposed to denormal, infinitym nan,
  // or zero) floating-point scalar constant or a vector constant with all
  // normal elements.
  fn is_normal_fp(&self) -> bool { false }

  // Return true if this scalar has an exact multiplicative inverse or this
  // vector has an exact multiplicative inverse for each element in the vector.
  fn has_exact_inverse_fp(&self) -> bool { false }

  // Return true if this is a floating-point NaN constant or a vector
  // floating-point constant with all NaN elements.
  fn is_nan(&self) -> bool { false }

  // Return true if this constant and a constant 'y' are element-wise equal.
  fn is_element_wise_equal(&self) -> bool { false }
  fn contains_undef_or_poison_element(&self) {}
  fn contains_poison_element(&self) {}
  fn contains_undef_element(&self) {}
  fn contains_constant_expression(&self) {}
  fn is_thread_dependent(&self) {}
  fn is_dll_import_dependent(&self) {}
  fn is_constant_used(&self) {}
  fn needs_relocation(&self) {}
  fn needs_dynamic_relocation(&self) {}
  // For aggregates (struct/array/vector) return the constant that corresponds
  // to the specified element if possible, or null if not.
  //fn get_aggregate_element(&self, _elt: u32) -> Option<Constant> {
    //None
  //}
  // If all elements of the vector constant have the same value,
  // return that value. Otherwise, return None.
  //fn get_splat_value(&self, _allow_undefs: bool) -> Option<Constant> {
    //None
  //}
  fn get_unique_integer(&self) {}
  fn destroy_constant(&self) {}
  fn handle_operand_change(&self) {}
  fn get_null_value(&self, _t: Box<dyn Type>) /*-> Constant*/ {
  }
  fn get_integer_value(&self) {}
  fn remove_dead_constant_users(&self) {}
  fn has_one_live_uses(&self) {}
  fn strip_pointer_casts(&self) {}
  fn replace_undefs_with(&self) {}
  fn merge_undefs_with(&self) {}
  fn is_manifest_constant(&self) {}
  fn get_relocation_info(&self) {}
  fn has_n_live_uses(&self) {}
}