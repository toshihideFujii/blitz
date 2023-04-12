#![allow(dead_code)]

// This file contains the declarations for the subclasses of
// Constant, which represent the different flavors of constant
// values that live in Bllitz.
// Note that Constants are immutable (once created they never change)
// and are fully shared by structural equivalence.
// This means that two structurally equivalent constants will
// always have the same address.
// Constants are created on demand as needed and never deleted:
// thus clients don't have to worry about the lifetime of the objects.

use crate::adt::{ap_int::APInt, ap_float::APFloat};

use super::{
  blits_context::BlitzContext,
  type_::{IntegerType, self, Type},
  value::{Value, ValueType},
};


// This is the shared class of boolean and integer constants.
// This class represents both boolean and integral constants.
// Class for constant integers.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantInt {
  v_type: IntegerType,
  subclass_id: u32,
  val: APInt
}

impl ConstantInt {
  pub fn new(v_type: IntegerType, val: APInt) -> Self {
    ConstantInt {
      v_type: v_type,
      subclass_id: 0,
      val: val
    }
  }

  pub fn get_true(&self, context: &mut BlitzContext) -> Option<ConstantInt> {
    let mut c_context = context.clone();
    let pimpl = context.get_impl();
    if pimpl.get_true_value().is_none() {
      let t = type_::get_int_1_type(&mut c_context);
      let int_1 = ConstantInt::get(t, 1, false);
      pimpl.set_true_value(Some(int_1));
    }
    pimpl.get_true_value()
  }

  pub fn get_false(&self, context: &mut BlitzContext) -> Option<ConstantInt> {
    let mut c_context = context.clone();
    let pimpl = context.get_impl();
    if pimpl.get_false_value().is_none() {
      let t = type_::get_int_1_type(&mut c_context);
      let int_0 = ConstantInt::get(t, 0, false);
      pimpl.set_false_value(Some(int_0));
    }
    pimpl.get_false_value()
  }

  pub fn get_bool(&self, context: &mut BlitzContext, v: bool) -> Option<ConstantInt> {
    if v {
      self.get_true(context)
    } else {
      self.get_false(context)
    }
  }

  // Return a ConstantInt with the specified value and an implied Type.
  // The type is the integer type that corresponds to the bit width of the value.
  pub fn get_from_apint(mut context: BlitzContext, v: APInt) -> ConstantInt {
    let c_clone = context.clone();
    let v_clone = v.clone();
    let pimpl = context.get_impl();
    let slot = pimpl.int_constants.find(&v);
    if slot.is_none() {
      let int_type = IntegerType::get(c_clone, v.get_bit_width());
      let const_int = ConstantInt::new(int_type, v);
      let const_int_clone = const_int.clone();
      pimpl.int_constants.insert(v_clone, const_int);
      return const_int_clone;
    }
    slot.unwrap().clone()
  }

  // Return a ConstantInt with the specified integer value for the specified type.
  pub fn get(t: IntegerType, v: u64, is_signed: bool) -> ConstantInt {
    let val = APInt::new(t.get_bit_width(), v as i64, is_signed);
    ConstantInt::get_from_apint(t.get_context().clone(), val)
  }

  // Return a ConstantInt with the specified value for the specified type.
  // The value v will be canonicalized to an unsigned APInt.
  pub fn get_signed(&self, t: IntegerType, v: i64) -> ConstantInt {
    ConstantInt::get(t, v as u64, true)
  }

  // Return the content as an APInt value reference.
  // This allows clients to obtain a full-precision copy of the value.
  pub fn get_value(&self) -> &APInt {
    &self.val
  }

  // Return the bitwidth of this constant.
  pub fn get_bit_width(&self) -> u32 {
    self.val.get_bit_width()
  }

  // Return the constant as a 64-bit unsigned integer value after it
  // has been zero extended as appropriate for the type of this constant.
  pub fn get_zext_value(&self) -> u64 {
    self.val.get_z_ext_value()
  }

  // Return the constant as a 64-bit integer value after it has been
  // sign extended as appropriate for the type of this constant.
  pub fn get_sext_value(&self) -> i64 {
    self.val.get_s_ext_value()
  }

  pub fn get_maybe_align_value() {}
  pub fn get_align_value() {}

  // A helper method that can be used to deetrmine if the constant contained
  // within is equal to a constant.
  pub fn equals_int(&self, v: u64) -> bool {
    self.val == v
  }

  // Specialize the get_type() method to always return an IntegerType,
  // which reduces the amount of casting needed in parts of the compiler.
  pub fn get_type(&self) -> IntegerType {
    self.v_type.clone()
  }

  pub fn is_value_valid_for_type() {}

  pub fn is_negative(&self) -> bool {
    self.val.is_negative()
  }

  // This is just a convenience method to make client code smaller for 
  // a common code.
  pub fn is_zero(&self) -> bool {
    self.val.is_zero()
  }

  // This is just a convenience method to make client code smaller for 
  // a common code.
  pub fn is_one(&self) -> bool {
    self.val.is_one()
  }

  // This function will return true if every bit in this constant is set
  // to one.
  pub fn is_minus_one(&self) -> bool {
    self.val.is_all_ones()
  }

  // This function will return true if this constant represents the largest
  // value that may be represented by the constants's type.
  pub fn is_max_value(&self, is_signed: bool) -> bool {
    if is_signed {
      self.val.is_max_signed_value()
    } else {
      self.val.is_max_value()
    }
  }

  // This function will return true if this constant represents the smallest
  // value that may be represented by the constants's type.
  pub fn is_min_value(&self, is_signed: bool) -> bool {
    if is_signed {
      self.val.is_min_signed_value()
    } else {
      self.val.is_min_value()
    }
  }

  // This function will return true if this constant represents a value with
  // active bits bigger than 64 bits or a value greater than the given u64 value.
  pub fn uge(&self, num: u64) -> bool {
    self.val.uge(num)
  }

  // If the value is smaller than the specified limit, return it, otherwise
  // return the limit value.
  pub fn get_limited_value(&self, limit: u64) -> u64 {
    self.val.get_limited_value(limit)
  }

  // Methods to support type inquiry through isa, cast, and dyn_cast.
  pub fn class_of(v: &dyn Value) -> bool {
    v.get_value_id() == ValueType::ConstantIntVal as u32
  }
}

impl Value for ConstantInt {
  fn get_type(&self) -> &dyn Type {
    &self.v_type
  }

  fn get_value_id(&self) -> u32 {
    self.subclass_id
  }
}

// Floating point values (float, double).
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantFP {
  v_type: IntegerType,
  subclass_id: u32,
  val: APFloat
}

impl ConstantFP {
  pub fn new() {}
  pub fn get_zero_value_for_negation(&self) {}
  pub fn get() {}
  pub fn get_nan() {}
  pub fn get_q_nan() {}
  pub fn get_s_nan() {}
  pub fn get_zero() {}
  pub fn get_negative_zero() {}
  pub fn get_infinity() {}
  pub fn is_value_valid_for_type() {}
  pub fn get_value_apf() {}
  pub fn get_value() {}
  pub fn is_zero() {}
  pub fn is_negative() {}
  pub fn is_infinity() {}
  pub fn is_nan() {}
  pub fn is_exactly_value(&self, _v: f64) -> bool { false }

  // Methods to support type inquiry through isa, cast, and dyn_cast.
  pub fn class_of(v: &dyn Value) -> bool {
    v.get_value_id() == ValueType::ConstantFPVal as u32
  }
}

impl Value for ConstantFP {
  fn get_type(&self) -> &dyn Type {
    &self.v_type
  }

  fn get_value_id(&self) -> u32 {
    self.subclass_id
  }
}

// All zero aggregate value.
pub struct ConstantAggregateZero {
  v_type: Box<dyn Type>,
  subclass_id: u32,
}

impl ConstantAggregateZero {
  pub fn new() {}
  pub fn get() {}
  pub fn get_sequential_element() {}
  pub fn get_struct_element() {}
  pub fn get_element_value() {}
  pub fn get_element_count() {}

  // Methods to support type inquiry through isa, cast, and dyn_cast.
  pub fn class_of(v: &dyn Value) -> bool {
    v.get_value_id() == ValueType::ConstantAggregateZeroVal as u32
  }
}

impl Value for ConstantAggregateZero {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_value_id(&self) -> u32 {
    self.subclass_id
  }
}

struct ConstantArray {}

struct ConstantStruct {}

struct ConstantVector {}

pub struct ConstantPointerNull {}

struct ConstantDataSequential {}

struct ConstantDataArray {}

struct ConstantDataVector {}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantTokenNone {}

pub struct ConstantTargetNone {}

struct BlockAddress {}

struct DS0LocalEquivalent {}

struct NoCFIValue {}

struct ConstantExpr {}

struct UndefValue {}

struct PoisonValue {}