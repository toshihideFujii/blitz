#![allow(dead_code)]

// This file declares a class to represent arbitrary precision
// floating point values and provide a variety of arithmetic
// operations on them.

use std::ops::*;

use super::{floating_point_mode::RoundingMode, ap_int::APInt};

// Enum that represents what fraction of the LSB truncated bits
// of an fp number represent.
enum LostFraction {
  ExactlyZero,
  LessThanZero,
  ExactlyHalf,
  MoreThanHalf
}

// Floating point semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Semantics {
  IEEEHalf,
  BFloat,
  IEEESingle,
  IEEEDouble,
  IEEEQuad,
  PPCDoubleDouble,
  Float8E5M2,
  Float8E5M2FNUZ,
  Float8E4M3FN,
  Float8E4M3FNUZ,
  X87DoubleExtended
}

// IEEE-754R 5.11: floating point comparison relations.
pub enum CmpResult {
  LessThan,
  Equal,
  GreaterThan,
  Unordered
}

// IEEE-754R 7: Default exception handling.
enum OpStatus {
  Ok,
  InvalidOp,
  DivByZero,
  Overflow,
  Underflow,
  Inexact
}

// Category of internally-represented number.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FltCategory {
  Infinity,
  Nan,
  Normal,
  Zero
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FltSemantics {
  // The largest Esuch that 2^E isrepresentable; this matches the
  // definition of IEEE 754.
  max_exponent: i32,
  // The smallest E such that 2^E is a normalized number; this matches
  // the definition of IEEE 754.
  min_exponent: i32,
  // Number of bits in the significand. This includs the integer bit.
  precision: u32,
  // Number of bits actually used in the semantics.
  size_in_bits: u32
}

impl FltSemantics {
  pub fn new() -> Self {
    FltSemantics { max_exponent: 0, min_exponent: 0,
      precision: 0, size_in_bits: 0 }
  }

  pub fn ieee_half() -> Self {
    FltSemantics { max_exponent: 15, min_exponent: -14,
       precision: 11, size_in_bits: 16 }
  }

  pub fn bloat() -> Self {
    FltSemantics { max_exponent: 127, min_exponent: -126,
      precision: 8, size_in_bits: 16 }
  }

  pub fn ieee_single() -> Self {
    FltSemantics { max_exponent: 127, min_exponent: -126,
      precision: 24, size_in_bits: 32 }
  }

  pub fn ieee_double() -> Self {
    FltSemantics { max_exponent: 1023, min_exponent: -1022,
      precision: 53, size_in_bits: 64 }
  }

  pub fn ieee_quad() -> Self {
    FltSemantics { max_exponent: 16383, min_exponent: -16382,
      precision: 113, size_in_bits: 128 }
  }

  pub fn float_8e5m2() -> Self {
    FltSemantics { max_exponent: 15, min_exponent: -14,
      precision: 3, size_in_bits: 8 }
  }

  pub fn x87_double_extended() -> Self {
    FltSemantics { max_exponent: 16383, min_exponent: -16382,
      precision: 64, size_in_bits: 80 }
  }

  pub fn bogus() -> Self {
    FltSemantics { max_exponent: 0, min_exponent: 0,
      precision: 0, size_in_bits: 0 }
  }

  pub fn ppc_double_double() -> Self {
    FltSemantics { max_exponent: -1, min_exponent: 0,
      precision: 0, size_in_bits: 128 }
  }

  pub fn is_representable_by() {}
} 

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IEEEFloat {
  // Note: this must be the first data member.
  // The semantics that this value obeys.
  semantics: FltSemantics,
  // What kind of floating point number this is.
  category: FltCategory,
  // Sign bit of the number.
  sign: u32
}

impl IEEEFloat {
  pub fn new() -> Self {
    IEEEFloat { semantics: FltSemantics::new(), category: FltCategory::Zero, sign: 0 }
  }

  pub fn new_from_semantics() {}

  pub fn make_zero(&self, _neg: bool) {}
  pub fn make_inf(&self, _neg: bool) {}
  pub fn make_nan(&self) {}
  pub fn make_largest(&self, _neg: bool) {}
  pub fn make_smallest(&self, _neg: bool) {}
  pub fn make_smallest_normalized(&self, _neg: bool) {}
  pub fn make_quiet(&self) {}
  pub fn compare_absolute_value(&self, _rhs: &APFloat) -> CmpResult {
    CmpResult::Equal
  }

  pub fn needs_cleanup(&self) {}

  pub fn add_float(&self, _rhs: &IEEEFloat, _rm: RoundingMode) {}
  pub fn subtract_float(&self, _rhs: &IEEEFloat, _rm: RoundingMode) {}
  pub fn multiply_float(&self, _rhs: &IEEEFloat, _rm: RoundingMode) {}
  pub fn divide_float(&self, _rhs: &IEEEFloat, _rm: RoundingMode) {}
  pub fn reminder_float(&self, _rhs: &IEEEFloat) {}
  pub fn modulo_float(&self, _rhs: &IEEEFloat) {}
  pub fn fuse_multiply_add(&self) {}
  pub fn round_to_integral(&self, _rm: RoundingMode) {}
  pub fn next(&self) {}

  pub fn change_sign(&self) {}
  pub fn convert(&self) {}
  pub fn convert_to_integer(&self) {}

  pub fn convert_from_apint(&mut self, _input: &APInt, _is_signed: bool, _rm: RoundingMode) {}

  pub fn convert_from_sign_extended_integer(&self) {}
  pub fn convert_from_zero_extended_integer(&self) {}
  pub fn convert_from_string(&self) {}
  pub fn bitcast_to_apint(&self) {}
  pub fn convert_to_double(&self) {}
  pub fn convert_to_float(&self) {}

  pub fn compare(&self, _rhs: &IEEEFloat) -> CmpResult {
    CmpResult::Equal
  }
  pub fn bitwise_is_equal(&self) -> bool { false }
  pub fn convert_to_hex_string(&self) {}

  // IEEE-754R isSignMinus:
  // Returns true if and only if the current value is negative.
  pub fn is_negative(&self) -> bool {
    self.sign != 0
  }

  // IEEE-754R isNormal:
  // Returns true if and only if the current value is normal.
  pub fn is_normal(&self) -> bool {
    !self.is_denormal() && self.is_finite_non_zero()
  }

  // Returns true if and only if the current value is zero, subnormal,
  // or normal.
  pub fn is_finite(&self) -> bool {
    !self.is_nan() && !self.is_infinity()
  }

  // Returns true if and only if the float is plus or minus zero.
  pub fn is_zero(&self) -> bool {
    self.category == FltCategory::Zero
  }

  pub fn is_denormal(&self) -> bool { false }

  // IEEE-754R isInfinite:
  // returns true if and only if the float is infinity.
  pub fn is_infinity(&self) -> bool {
    self.category == FltCategory::Infinity
  }

  // Returns true if and only if the float is a quiet or signaling NaN.
  pub fn is_nan(&self) -> bool {
    self.category == FltCategory::Nan
  }

  pub fn is_signaling(&self) -> bool { false }

  pub fn get_category(&self) -> FltCategory {
    self.category.clone()
  }

  pub fn get_semantics(&self) {}

  pub fn is_non_zero(&self) -> bool {
    self.category != FltCategory::Zero
  }

  pub fn is_finite_non_zero(&self) -> bool {
    self.is_finite() && !self.is_zero()
  }

  pub fn is_pos_zero(&self) -> bool {
    self.is_zero() && !self.is_negative()
  }

  pub fn is_neg_zero(&self) -> bool {
    self.is_zero() && self.is_negative()
  }

  pub fn is_smallest(&self) -> bool { false }
  pub fn is_smallest_normalized(&self) -> bool { false }
  pub fn is_largest(&self) -> bool { false }
  pub fn is_integer(&self) -> bool { false }

  pub fn hash_value(&self) {}

  pub fn to_string(&self) {}
  pub fn get_exact_inverse(&self) {}

  pub fn ilogb(&self) {}
  pub fn scal_bn(&self) {}
  pub fn frexp(&self) {}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoubleAPFloat {}
impl DoubleAPFloat {
  pub fn new() -> Self {
    DoubleAPFloat {  }
  }

  pub fn make_zero(&self, _neg: bool) {}
  pub fn make_inf(&self, _neg: bool) {}
  pub fn make_nan(&self) {}
  pub fn make_largest(&self, _neg: bool) {}
  pub fn make_smallest(&self, _neg: bool) {}
  pub fn make_smallest_normalized(&self, _neg: bool) {}
  pub fn make_quiet(&self) {}
  pub fn compare_absolute_value(&self, _rhs: &APFloat) -> CmpResult {
    CmpResult::Equal
  }
  pub fn needs_cleanup(&self) {}
  pub fn add_float(&self, _rhs: &DoubleAPFloat, _rm: RoundingMode) {}
  pub fn subtract_float(&self, _rhs: &DoubleAPFloat, _rm: RoundingMode) {}
  pub fn multiply_float(&self, _rhs: &DoubleAPFloat, _rm: RoundingMode) {}
  pub fn divide_float(&self, _rhs: &DoubleAPFloat, _rm: RoundingMode) {}
  pub fn reminder_float(&self, _rhs: &DoubleAPFloat) {}
  pub fn modulo_float(&self, _rhs: &DoubleAPFloat) {}

  pub fn round_to_integral(&self, _rm: RoundingMode) {}

  pub fn change_sign(&self) {}
  pub fn compare(&self, _rhs: &DoubleAPFloat) -> CmpResult {
    CmpResult::Equal
  }
  pub fn bitwise_is_equal(&self) -> bool { false }

  pub fn is_smallest(&self) -> bool { false }
  pub fn is_smallest_normalized(&self) -> bool { false }
  pub fn is_largest(&self) -> bool { false }
  pub fn is_integer(&self) -> bool { false }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct APFloat {
  // Sign bit of the number.
  sign: u32,
  semantics: Semantics,
  ieee: IEEEFloat,
  double: DoubleAPFloat
}

impl APFloat {
  pub fn new(_semantics: FltSemantics) -> Self {
    APFloat {
      sign: 0,
      semantics: Semantics::IEEEDouble,
      ieee: IEEEFloat::new(),
      double: DoubleAPFloat::new()
    }
  }

  pub fn new_from_apint(_s: FltSemantics, _apint: &APInt) -> Self {
    APFloat {
      sign: 0,
      semantics: Semantics::IEEEDouble,
      ieee: IEEEFloat::new(),
      double: DoubleAPFloat::new()
    }
  }

  // TODO
  pub fn get_ieee(&self) -> IEEEFloat {
    IEEEFloat::new()
  }

  pub fn make_zero(&self, neg: bool) {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.make_zero(neg);
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.make_zero(neg);
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn make_inf(&self, neg: bool) {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.make_inf(neg);
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.make_inf(neg);
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn make_nan(&self) {}

  pub fn make_largest(&self, neg: bool) {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.make_largest(neg);
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.make_largest(neg);
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn make_smallest(&self, neg: bool) {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.make_smallest(neg);
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.make_smallest(neg);
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn make_smallest_normalized(&self, neg: bool) {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.make_smallest_normalized(neg);
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.make_smallest_normalized(neg);
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn make_quiet(&self) {}

  pub fn compare_absolute_value(&self, rhs: &APFloat) -> CmpResult {
    debug_assert!(self.get_semantics() == rhs.get_semantics(),
      "Should only compare APFloats with the same semantics.");
    if self.semantics != Semantics::PPCDoubleDouble {
      return self.ieee.compare_absolute_value(rhs);
    } else if self.semantics == Semantics::PPCDoubleDouble {
      return self.double.compare_absolute_value(rhs);
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn needs_cleanup(&self) {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.needs_cleanup();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.needs_cleanup();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  // Factory for positive and negative zero.
  pub fn get_zero(sem: FltSemantics, negative: bool) -> APFloat {
    let val = APFloat::new(sem);
    val.make_zero(negative);
    val
  }

  // Factory for positive and negative infinity.
  pub fn get_inf(sem: FltSemantics, negative: bool) -> APFloat {
    let val = APFloat::new(sem);
    val.make_inf(negative);
    val
  }

  pub fn get_nan() {}
  pub fn get_q_nan() {}
  pub fn get_s_nan() {}

  // Returns the largest finite number in the given semantics.
  pub fn get_largest(sem: FltSemantics, negative: bool) -> APFloat {
    let val = APFloat::new(sem);
    val.make_largest(negative);
    val
  }

  // Returns the smallest (by magnitude) finite number in the given semantics.
  pub fn get_smallest(sem: FltSemantics, negative: bool) -> APFloat {
    let val = APFloat::new(sem);
    val.make_smallest(negative);
    val
  }

  // Returns the smallest (by magnitude) normalized finite number in the
  // given semantics.
  pub fn get_smallest_normalized(sem: FltSemantics, negative: bool) -> APFloat {
    let val = APFloat::new(sem);
    val.make_smallest_normalized(negative);
    val
  }

  pub fn get_all_ones_value() {}
  pub fn profile() {}

  pub fn enum_to_semantics(s: Semantics) -> FltSemantics {
    match s {
      Semantics::IEEEHalf =>
        return FltSemantics { max_exponent: 15, min_exponent: -14, precision: 11, size_in_bits: 16 },
      Semantics::BFloat =>
        return FltSemantics { max_exponent: 127, min_exponent: -126, precision: 8, size_in_bits: 16 },
      Semantics::IEEESingle =>
        return FltSemantics { max_exponent: 127, min_exponent: -126, precision: 24, size_in_bits: 32 },
      Semantics::IEEEDouble =>
        return FltSemantics { max_exponent: 1023, min_exponent: -1022, precision: 53, size_in_bits: 64 },
      Semantics::IEEEQuad =>
        return FltSemantics { max_exponent: 16383, min_exponent: -16382, precision: 113, size_in_bits: 128 },
      Semantics::PPCDoubleDouble =>
        return FltSemantics { max_exponent: -1, min_exponent: 0, precision: 0, size_in_bits: 128 },
      Semantics::Float8E5M2 =>
        return FltSemantics { max_exponent: 15, min_exponent: -14, precision: 3, size_in_bits: 8 },
      Semantics::Float8E5M2FNUZ => // TODO
        return FltSemantics { max_exponent: 15, min_exponent: -15, precision: 3, size_in_bits: 8 },
        Semantics::Float8E4M3FN => // TODO
        return FltSemantics { max_exponent: 8, min_exponent: -6, precision: 4, size_in_bits: 8 },
        Semantics::Float8E4M3FNUZ => // TODO
        return FltSemantics { max_exponent: 7, min_exponent: -7, precision: 4, size_in_bits: 8 },
        Semantics::X87DoubleExtended =>
        return FltSemantics { max_exponent: 16383, min_exponent: -16382, precision: 64, size_in_bits: 80 },
    };
  }

  pub fn semantics_to_enum(&self) {}

  pub fn ieee_half() -> FltSemantics {
    FltSemantics { max_exponent: 15, min_exponent: -14,
      precision: 11, size_in_bits: 16 }
  }

  pub fn b_float() -> FltSemantics {
    FltSemantics { max_exponent: 127, min_exponent: -126,
      precision: 8, size_in_bits: 16 }
  }

  pub fn ieee_single() -> FltSemantics {
    FltSemantics { max_exponent: 127, min_exponent: -126,
      precision: 24, size_in_bits: 32 }
  }

  pub fn ieee_double() -> FltSemantics {
    FltSemantics { max_exponent: 1023, min_exponent: -1022,
      precision: 53, size_in_bits: 64 }
  }

  pub fn ieee_quad() -> FltSemantics {
    FltSemantics { max_exponent: 16383, min_exponent: -16382,
      precision: 113, size_in_bits: 128 }
  }

  pub fn ppc_double_double() -> FltSemantics {
    FltSemantics { max_exponent: -1, min_exponent: 0,
      precision: 0, size_in_bits: 128 }
  }

  pub fn float_8e5m2() -> FltSemantics {
    FltSemantics { max_exponent: 15, min_exponent: -14,
      precision: 3, size_in_bits: 8 }
  }

  pub fn float_8e5m2_fnuz() -> FltSemantics {
    FltSemantics { max_exponent: 15, min_exponent: -15,
      precision: 3, size_in_bits: 8 }
  }

  pub fn float_8e4m3_fn() -> FltSemantics {
    FltSemantics { max_exponent: 8, min_exponent: -6,
      precision: 4, size_in_bits: 8 }
  }

  pub fn float_8e4m3_fnuz() -> FltSemantics {
    FltSemantics { max_exponent: 7, min_exponent: -7,
      precision: 4, size_in_bits: 8 }
  }

  pub fn x87_double_extended() -> FltSemantics {
    FltSemantics { max_exponent: 16383, min_exponent: -16382,
      precision: 64, size_in_bits: 80 }
  }

  pub fn bogus() -> FltSemantics {
    FltSemantics { max_exponent: 0, min_exponent: 0,
      precision: 0, size_in_bits: 0 }
  }

  pub fn semantics_precision(s: &FltSemantics) -> u32 {
    s.precision
  }

  pub fn semantics_min_exponent(s: &FltSemantics) -> i32 {
    s.min_exponent
  }

  pub fn semantics_max_exponent(s: &FltSemantics) -> i32 {
    s.max_exponent
  }

  pub fn semantics_size_in_bits(s: &FltSemantics) -> u32 {
    s.size_in_bits
  }

  pub fn semantics_int_size_in_bits(s: &FltSemantics, is_signed: bool) -> i32 {
    let mut min_bit_width = APFloat::semantics_max_exponent(s) + 1;
    if is_signed {
      min_bit_width += 1;
    }
    min_bit_width
  }

  pub fn get_size_in_bits(s: &FltSemantics) -> u32 {
    s.size_in_bits
  }

  pub fn add_float(&self, rhs: &APFloat, rm: RoundingMode) -> APFloat {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.add_float(&rhs.ieee, rm);
      return self.clone();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.add_float(&rhs.double, rm);
      return self.clone();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn subtract_float(&self, rhs: &APFloat, rm: RoundingMode) -> APFloat {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.subtract_float(&rhs.ieee, rm);
      return self.clone();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.subtract_float(&rhs.double, rm);
      return self.clone();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn multiply_float(&self, rhs: &APFloat, rm: RoundingMode) -> APFloat {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.multiply_float(&rhs.ieee, rm);
      return self.clone();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.multiply_float(&rhs.double, rm);
      return self.clone();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn divide_float(&self, rhs: &APFloat, rm: RoundingMode) -> APFloat {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.divide_float(&rhs.ieee, rm);
      return self.clone();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.divide_float(&rhs.double, rm);
      return self.clone();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn reminder(&self, rhs: &APFloat) -> APFloat {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.reminder_float(&rhs.ieee);
      return self.clone();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.reminder_float(&rhs.double);
      return self.clone();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn modulo(&self, rhs: &APFloat) -> APFloat {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.modulo_float(&rhs.ieee);
      return self.clone();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.modulo_float(&rhs.double);
      return self.clone();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn fuse_multiply_add(&self) {}

  pub fn round_to_integral(&self, rm: RoundingMode) {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.round_to_integral(rm);
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.round_to_integral(rm);
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn next(&self) {}

  pub fn change_sign(&self) {
    if self.semantics != Semantics::PPCDoubleDouble {
      self.ieee.change_sign();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      self.double.change_sign();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn clear_sign(&self) {
    if self.is_negative() {
      self.change_sign();
    }
  }

  pub fn copy_sign(&self, rhs: &APFloat) {
    if self.is_negative() != rhs.is_negative() {
      self.change_sign();
    }
  }

  pub fn convert(&self) {}
  pub fn convert_to_integer(&self) {}
  pub fn convert_from_apint(&mut self, _input: &APInt, _is_signed: bool, _rm: RoundingMode) {}
  pub fn convert_from_sign_extended_integer(&self) {}
  pub fn convert_from_zero_extended_integer(&self) {}
  pub fn convert_from_string(&self) {}
  pub fn bitcast_to_apint(&self) -> APInt {
    // TODO
    APInt::new_zero()
  }
  pub fn convert_to_double(&self) {}
  pub fn convert_to_float(&self) {}

  pub fn compare(&self, rhs: &APFloat) -> CmpResult {
    if self.semantics != Semantics::PPCDoubleDouble {
      return self.ieee.compare(&rhs.ieee);
    } else if self.semantics == Semantics::PPCDoubleDouble {
      return self.double.compare(&rhs.double);
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn bitwise_is_equal(&self) -> bool {
    if self.semantics != Semantics::PPCDoubleDouble {
      return self.ieee.bitwise_is_equal();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      return self.double.bitwise_is_equal();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn convert_to_hex_string(&self) {}

  pub fn is_zero(&self) -> bool {
    self.get_category() == FltCategory::Zero
  }

  pub fn is_infinity(&self) -> bool {
    self.get_category() == FltCategory::Infinity
  }

  pub fn is_nan(&self) -> bool {
    self.get_category() == FltCategory::Nan
  }

  pub fn is_negative(&self) -> bool {
    self.get_ieee().is_negative()
  }

  pub fn is_denormal(&self) -> bool { false }

  pub fn is_signaling(&self) -> bool {
    self.get_ieee().is_signaling()
  }

  pub fn is_normal(&self) -> bool {
    !self.is_denormal() && self.is_finite_non_zero()
  }

  pub fn is_finite(&self) -> bool {
    !self.is_nan() && !self.is_infinity()
  }

  pub fn get_category(&self) -> FltCategory {
    self.get_ieee().get_category()
  }

  // TODO
  pub fn get_semantics(&self) -> FltSemantics {
    FltSemantics::new()
  }

  pub fn is_non_zero(&self) -> bool {
    !self.is_zero()
  }

  pub fn is_finite_non_zero(&self) -> bool {
    self.is_finite() && !self.is_zero()
  }

  pub fn is_pos_zero(&self) -> bool {
    self.is_zero() && !self.is_negative()
  }

  pub fn is_neg_zero(&self) -> bool {
    self.is_zero() && self.is_negative()
  }

  pub fn is_pos_infinity(&self) -> bool {
    self.is_infinity() && !self.is_negative()
  }

  pub fn is_neg_infinity(&self) -> bool {
    self.is_infinity() && self.is_negative()
  }

  pub fn is_smallest(&self) -> bool {
    if self.semantics != Semantics::PPCDoubleDouble {
      return self.ieee.is_smallest();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      return self.double.is_smallest();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn is_largest(&self) -> bool {
    if self.semantics != Semantics::PPCDoubleDouble {
      return self.ieee.is_largest();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      return self.double.is_largest();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn is_integer(&self) -> bool {
    if self.semantics != Semantics::PPCDoubleDouble {
      return self.ieee.is_integer();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      return self.double.is_integer();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn is_ieee(&self) -> bool {
    self.semantics != Semantics::PPCDoubleDouble
  }

  pub fn is_smallest_normalized(&self) -> bool {
    if self.semantics != Semantics::PPCDoubleDouble {
      return self.ieee.is_smallest_normalized();
    } else if self.semantics == Semantics::PPCDoubleDouble {
      return self.double.is_smallest_normalized();
    } else {
      panic!("Unexpected semantics!");
    }
  }

  pub fn to_string(&self) {}
  pub fn print(&self) {}
  pub fn dump(&self) {}
  pub fn get_exact_inverse(&self, _inv: Option<APFloat>) -> bool { false }
  pub fn hash_value(&self) {}

  pub fn ilogb(&self) {}
  pub fn scal_bn(&self) {}
  pub fn frexp(&self) {}
}

impl Add<APFloat> for APFloat {
  type Output = APFloat;
  fn add(self, rhs: APFloat) -> Self::Output {
    self.add_float(&rhs, RoundingMode::NearestTiesToEven)
  }
}

impl Sub<APFloat> for APFloat {
  type Output = APFloat;
  fn sub(self, rhs: APFloat) -> Self::Output {
    self.subtract_float(&rhs, RoundingMode::NearestTiesToEven)
  }
}

impl Mul<APFloat> for APFloat {
  type Output = APFloat;
  fn mul(self, rhs: APFloat) -> Self::Output {
    self.multiply_float(&rhs, RoundingMode::NearestTiesToEven)
  }
}

impl Div<APFloat> for APFloat {
  type Output = APFloat;
  fn div(self, rhs: APFloat) -> Self::Output {
    self.divide_float(&rhs, RoundingMode::NearestTiesToEven)
  }
}