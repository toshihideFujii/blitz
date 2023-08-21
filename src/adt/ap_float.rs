#![allow(dead_code)]

// This file declares a class to represent arbitrary precision
// floating point values and provide a variety of arithmetic
// operations on them.

use std::ops::*;

use super::{floating_point_mode::RoundingMode, ap_int::APInt, string_ref::StringRef};

// Enum that represents what fraction of the LSB truncated bits
// of an fp number represent.
enum LostFraction {
  ExactlyZero,
  LessThanZero,
  ExactlyHalf,
  MoreThanHalf
}

#[derive(Debug, Clone, PartialEq)]
pub enum APFType {
  IEEEFloat,
  DoubleAPFloat
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
  Float8E4M3B11FNUZ,
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
// Underflow or Overflow are always returned or-ed with Inexact.
#[derive(Debug, Clone, PartialEq)]
pub enum OpStatus {
  Ok = 0x00,
  InvalidOp = 0x01,
  DivByZero = 0x02,
  Overflow = 0x04,
  Underflow = 0x08,
  Inexact = 0x10
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
pub enum FltNonfiniteBehavior {
  IEEE754,
  NanOnly
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FltNanEncoding {
  IEEE,
  AllOnes,
  NegativeZero
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
  size_in_bits: u32,

  non_finite_behavior: FltNonfiniteBehavior,
  nan_encoding: FltNanEncoding
}

impl FltSemantics {
  pub fn new() -> Self {
    FltSemantics {
      max_exponent: 0, min_exponent: 0, precision: 0, size_in_bits: 0,
      non_finite_behavior: FltNonfiniteBehavior::IEEE754,
      nan_encoding: FltNanEncoding::IEEE
    }
  }

  pub fn ieee_half() -> Self {
    FltSemantics {
      max_exponent: 15, min_exponent: -14, precision: 11, size_in_bits: 16,
      non_finite_behavior: FltNonfiniteBehavior::IEEE754,
      nan_encoding: FltNanEncoding::IEEE
    }
  }

  pub fn bfloat() -> Self {
    FltSemantics {
      max_exponent: 127, min_exponent: -126, precision: 8, size_in_bits: 16,
      non_finite_behavior: FltNonfiniteBehavior::IEEE754,
      nan_encoding: FltNanEncoding::IEEE
    }
  }

  pub fn ieee_single() -> Self {
    FltSemantics {
      max_exponent: 127, min_exponent: -126, precision: 24, size_in_bits: 32,
      non_finite_behavior: FltNonfiniteBehavior::IEEE754,
      nan_encoding: FltNanEncoding::IEEE
    }
  }

  pub fn ieee_double() -> Self {
    FltSemantics {
      max_exponent: 1023, min_exponent: -1022, precision: 53, size_in_bits: 64,
      non_finite_behavior: FltNonfiniteBehavior::IEEE754,
      nan_encoding: FltNanEncoding::IEEE
    }
  }

  pub fn ieee_quad() -> Self {
    FltSemantics {
      max_exponent: 16383, min_exponent: -16382, precision: 113, size_in_bits: 128,
      non_finite_behavior: FltNonfiniteBehavior::IEEE754,
      nan_encoding: FltNanEncoding::IEEE
    }
  }

  pub fn float_8e5m2() -> Self {
    FltSemantics {
      max_exponent: 15, min_exponent: -14, precision: 3, size_in_bits: 8,
      non_finite_behavior: FltNonfiniteBehavior::IEEE754,
      nan_encoding: FltNanEncoding::IEEE
    }
  }

  pub fn float_8e5m2_fnuz() -> Self {
    FltSemantics {
      max_exponent: 15, min_exponent: -15, precision: 3, size_in_bits: 8,
      non_finite_behavior: FltNonfiniteBehavior::NanOnly,
      nan_encoding: FltNanEncoding::NegativeZero
    }
  }

  pub fn float_8e4m3_fn() -> Self {
    FltSemantics {
      max_exponent: 8, min_exponent: -6, precision: 4, size_in_bits: 8,
      non_finite_behavior: FltNonfiniteBehavior::NanOnly,
      nan_encoding: FltNanEncoding::AllOnes
    }
  }

  pub fn float_8e4m3_fnuz() -> Self {
    FltSemantics {
      max_exponent: 7, min_exponent: -7, precision: 4, size_in_bits: 8,
      non_finite_behavior: FltNonfiniteBehavior::NanOnly,
      nan_encoding: FltNanEncoding::NegativeZero
    }
  }

  pub fn float_8e4m3b11_fnuz() -> Self {
    FltSemantics {
      max_exponent: 4, min_exponent: -10, precision: 4, size_in_bits: 8,
      non_finite_behavior: FltNonfiniteBehavior::NanOnly,
      nan_encoding: FltNanEncoding::NegativeZero
    }
  }

  pub fn x87_double_extended() -> Self {
    FltSemantics {
      max_exponent: 16383, min_exponent: -16382, precision: 64, size_in_bits: 80,
      non_finite_behavior: FltNonfiniteBehavior::IEEE754,
      nan_encoding: FltNanEncoding::IEEE
    }
  }

  pub fn bogus() -> Self {
    FltSemantics {
      max_exponent: 0, min_exponent: 0, precision: 0, size_in_bits: 0,
      non_finite_behavior: FltNonfiniteBehavior::IEEE754,
      nan_encoding: FltNanEncoding::IEEE
    }
  }

  pub fn ppc_double_double() -> Self {
    FltSemantics {
      max_exponent: -1, min_exponent: 0, precision: 0, size_in_bits: 128,
      non_finite_behavior: FltNonfiniteBehavior::IEEE754,
      nan_encoding: FltNanEncoding::IEEE
    }
  }

  // Returns true if any number described by this semantics can be precisely
  // represented by the specified semantics.
  // Does not take into acount the value of FltNonFiniteBehavior.
  pub fn is_representable_by(&self, s: &FltSemantics) -> bool {
    s.min_exponent <= self.min_exponent &&
    self.max_exponent <= s.max_exponent &&
    self.precision <= s.precision
  }
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
  pub fn make_nan(&self, _snan: bool, _neg: bool, _fill: Option<APInt>) {}
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
  pub fn convert_to_integer(&self, _input: u64, _width: u32, _is_signed: bool,
    _rm: RoundingMode, _is_exact: bool) {}

  pub fn convert_from_apint(&mut self, _input: &APInt, _is_signed: bool, _rm: RoundingMode) {}

  pub fn convert_from_sign_extended_integer(&self) {}
  pub fn convert_from_zero_extended_integer(&self) {}
  pub fn convert_from_string(&self, _s: StringRef, _rm: RoundingMode) {}
  pub fn bitcast_to_apint(&self) -> APInt { APInt::new_zero() }
  pub fn convert_to_double(&self) -> f64 { 0.0 }
  pub fn convert_to_float(&self) -> f32 { 0.0 }

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
  pub fn make_nan(&self, _snan: bool, _neg: bool, _fill: Option<APInt>) {}
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
  pub fn convert_to_integer(&self, _input: u64, _width: u32, _is_signed: bool,
    _rm: RoundingMode, _is_exact: bool) {}
  pub fn convert_from_apint(&mut self, _input: &APInt, _is_signed: bool, _rm: RoundingMode) {}
  pub fn convert_from_string(&self, _s: StringRef, _rm: RoundingMode) {}
  pub fn bitcast_to_apint(&self) -> APInt { APInt::new_zero() }
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
  semantics: FltSemantics,
  ieee: IEEEFloat,
  double: DoubleAPFloat
}

impl APFloat {
  pub fn new(semantics: FltSemantics) -> Self {
    APFloat {
      sign: 0,
      semantics: semantics,
      ieee: IEEEFloat::new(),
      double: DoubleAPFloat::new()
    }
  }

  pub fn new_from_apint(s: FltSemantics, _apint: &APInt) -> Self {
    APFloat {
      sign: 0,
      semantics: s,
      ieee: IEEEFloat::new(),
      double: DoubleAPFloat::new()
    }
  }

  // TODO
  pub fn get_ieee(&self) -> IEEEFloat {
    IEEEFloat::new()
  }

  pub fn make_zero(&self, neg: bool) {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.make_zero(neg);
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.make_zero(neg);
    }
    panic!("Unexpected semantics.");
  }

  pub fn make_inf(&self, neg: bool) {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.make_inf(neg);
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.make_inf(neg);
    }
    panic!("Unexpected semantics.");
  }

  pub fn make_nan(&self, snan: bool, neg: bool, fill: Option<APInt>) {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.make_nan(snan, neg, fill.clone());
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.make_nan(snan, neg, fill.clone());
    }
    panic!("Unexpected semantics.");
  }

  pub fn make_largest(&self, neg: bool) {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.make_largest(neg);
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.make_largest(neg);
    }
    panic!("Unexpected semantics.");
  }

  pub fn make_smallest(&self, neg: bool) {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.make_smallest(neg);
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.make_smallest(neg);
    }
    panic!("Unexpected semantics.");
  }

  pub fn make_smallest_normalized(&self, neg: bool) {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.make_smallest_normalized(neg);
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics){
      self.double.make_smallest_normalized(neg);
    }
    panic!("Unexpected semantics.");
  }

  pub fn make_quiet(&self) {}

  pub fn compare_absolute_value(&self, rhs: &APFloat) -> CmpResult {
    debug_assert!(self.get_semantics() == rhs.get_semantics(),
      "Should only compare APFloats with the same semantics.");
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.compare_absolute_value(rhs);
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.compare_absolute_value(rhs);
    }
    panic!("Unexpected semantics.");
  }

  pub fn needs_cleanup(&self) {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.needs_cleanup();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.needs_cleanup();
    }
    panic!("Unexpected semantics.");
  }

  // Factory for positive and negative zero.
  // Param negative true if the number should be neative.
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

  // Factory for NaN values.
  // Param negative - true if the NaN gernerated should be negative.
  // Param payload - the unspecified fill bits for creating the NaN, 0 by default.
  // The value is truncated as necessary.
  pub fn get_nan(sem: &FltSemantics, negative: bool, payload: u64) -> APFloat {
    if payload != 0 {
      let int_payload = APInt::new(64,
        payload as i64, false);
      return APFloat::get_q_nan(sem, negative, Some(int_payload));
    } else {
      return APFloat::get_q_nan(sem, negative, None);
    }
  }

  // Factory for QNaN values.
  pub fn get_q_nan(sem: &FltSemantics, negative: bool,
    payload: Option<APInt>) -> APFloat
  {
    let val = APFloat::new(sem.clone());
    val.make_nan(false, negative, payload);
    val
  }

  // Factory for SNaN values.
  pub fn get_s_nan(sem: &FltSemantics, negative: bool,
    payload: Option<APInt>) -> APFloat
  {
    let val = APFloat::new(sem.clone());
    val.make_nan(true, negative, payload);
    val
  }

  // Returns the largest finite number in the given semantics.
  pub fn get_largest(sem: &FltSemantics, negative: bool) -> APFloat {
    let val = APFloat::new(sem.clone());
    val.make_largest(negative);
    val
  }

  // Returns the smallest (by magnitude) finite number in the given semantics.
  pub fn get_smallest(sem: &FltSemantics, negative: bool) -> APFloat {
    let val = APFloat::new(sem.clone());
    val.make_smallest(negative);
    val
  }

  // Returns the smallest (by magnitude) normalized finite number in the
  // given semantics.
  pub fn get_smallest_normalized(sem: &FltSemantics, negative: bool) -> APFloat {
    let val = APFloat::new(sem.clone());
    val.make_smallest_normalized(negative);
    val
  }

  // Returns a float which is bitcasted from an all one value int.
  // Param semantics - type float semantics.
  pub fn get_all_ones_value(semantics: &FltSemantics) -> APFloat {
    APFloat::new_from_apint(semantics.clone(),
      &APInt::get_all_ones(semantics.size_in_bits))
  }

  pub fn profile() {}

  pub fn enum_to_semantics(s: Semantics) -> FltSemantics {
    match s {
      Semantics::IEEEHalf => return FltSemantics::ieee_half(),
      Semantics::BFloat => return FltSemantics::bfloat(),
      Semantics::IEEESingle => return FltSemantics::ieee_single(),
      Semantics::IEEEDouble => return FltSemantics::ieee_double(),
      Semantics::IEEEQuad => return FltSemantics::ieee_quad(),
      Semantics::PPCDoubleDouble => return FltSemantics::ppc_double_double(),
      Semantics::Float8E5M2 => return FltSemantics::float_8e5m2(),
      Semantics::Float8E5M2FNUZ => return FltSemantics::float_8e5m2_fnuz(),
      Semantics::Float8E4M3FN => return FltSemantics::float_8e4m3_fn(),
      Semantics::Float8E4M3FNUZ => return FltSemantics::float_8e4m3_fnuz(),
      Semantics::Float8E4M3B11FNUZ => return FltSemantics::float_8e4m3b11_fnuz(),
      Semantics::X87DoubleExtended => return FltSemantics::x87_double_extended(),
    };
  }

  pub fn semantics_to_enum(sem: &FltSemantics) -> Semantics {
    if *sem == FltSemantics::ieee_half() { return Semantics::IEEEHalf; }
    if *sem == FltSemantics::bfloat() { return Semantics::BFloat; }
    if *sem == FltSemantics::ieee_single() { return Semantics::IEEESingle; }
    if *sem == FltSemantics::ieee_double() { return Semantics::IEEEDouble; }
    if *sem == FltSemantics::ieee_quad() { return Semantics::IEEEQuad; }
    if *sem == FltSemantics::ppc_double_double() { return Semantics::PPCDoubleDouble; }
    if *sem == FltSemantics::float_8e5m2() { return Semantics::Float8E5M2; }
    if *sem == FltSemantics::float_8e5m2_fnuz() { return Semantics::Float8E5M2FNUZ; }
    if *sem == FltSemantics::float_8e4m3_fn() { return Semantics::Float8E4M3FN; }
    if *sem == FltSemantics::float_8e4m3_fnuz() { return Semantics::Float8E4M3FNUZ; }
    if *sem == FltSemantics::float_8e4m3b11_fnuz() { return Semantics::Float8E4M3B11FNUZ; }
    if *sem == FltSemantics::x87_double_extended() { return Semantics::X87DoubleExtended; }
    panic!("Unknown floating semantics.");
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
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.add_float(&rhs.ieee, rm);
      return self.clone();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.add_float(&rhs.double, rm);
      return self.clone();
    }
    panic!("Unexpected semantics.");
  }

  pub fn subtract_float(&self, rhs: &APFloat, rm: RoundingMode) -> APFloat {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.subtract_float(&rhs.ieee, rm);
      return self.clone();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.subtract_float(&rhs.double, rm);
      return self.clone();
    }
    panic!("Unexpected semantics.");
  }

  pub fn multiply_float(&self, rhs: &APFloat, rm: RoundingMode) -> APFloat {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.multiply_float(&rhs.ieee, rm);
      return self.clone();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.multiply_float(&rhs.double, rm);
      return self.clone();
    }
    panic!("Unexpected semantics.");
  }

  pub fn divide_float(&self, rhs: &APFloat, rm: RoundingMode) -> APFloat {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.divide_float(&rhs.ieee, rm);
      return self.clone();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.divide_float(&rhs.double, rm);
      return self.clone();
    }
    panic!("Unexpected semantics.");
  }

  pub fn reminder(&self, rhs: &APFloat) -> APFloat {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.reminder_float(&rhs.ieee);
      return self.clone();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.reminder_float(&rhs.double);
      return self.clone();
    }
    panic!("Unexpected semantics.");
  }

  pub fn modulo(&self, rhs: &APFloat) -> APFloat {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.modulo_float(&rhs.ieee);
      return self.clone();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.modulo_float(&rhs.double);
      return self.clone();
    }
    panic!("Unexpected semantics.");
  }

  pub fn fuse_multiply_add(&self) {}

  pub fn round_to_integral(&self, rm: RoundingMode) {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.round_to_integral(rm.clone());
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.round_to_integral(rm.clone());
    }
    panic!("Unexpected semantics.");
  }

  pub fn next(&self) {}

  pub fn change_sign(&self) {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      self.ieee.change_sign();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      self.double.change_sign();
    }
    panic!("Unexpected semantics.");
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

  pub fn convert(&self, _to_s: &FltSemantics, _rm: RoundingMode)
    -> (OpStatus, bool)
  {
    (OpStatus::InvalidOp, false)
  }

  pub fn convert_to_integer(&self, input: u64, width: u32, is_signed: bool,
    rm: RoundingMode, is_exact: bool)
  {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.convert_to_integer(input, width, is_signed, rm, is_exact)
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.convert_to_integer(input, width, is_signed, rm, is_exact)
    }
    panic!("Unexpected semantics.");
  }

  pub fn convert_from_apint(&mut self, input: &APInt,
    is_signed: bool, rm: RoundingMode)
  {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.convert_from_apint(input, is_signed, rm)
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.convert_from_apint(input, is_signed, rm)
    }
    panic!("Unexpected semantics.");
  }

  pub fn convert_from_sign_extended_integer(&self) {}
  pub fn convert_from_zero_extended_integer(&self) {}

  pub fn convert_from_string(&self, s: StringRef, rm: RoundingMode) {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.convert_from_string(s, rm)
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.convert_from_string(s, rm)
    }
    panic!("Unexpected semantics.");
  }

  pub fn bitcast_to_apint(&self) -> APInt {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.bitcast_to_apint()
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.bitcast_to_apint()
    }
    panic!("Unexpected semantics.");
  }

  // Converts this APFloat to host double value.
  // The APFloat must be built using semantics, that can be represented by the
  // host double type without loss of precision.
  // It can be IEEESingle and shorter semantics, like IEEEhalf.
  pub fn convert_to_double(&self) -> f64 {
    if self.semantics == FltSemantics::ieee_double() {
      return self.get_ieee().convert_to_double();
    }
    debug_assert!(self.semantics.is_representable_by(&FltSemantics::ieee_double()),
      "Float semantics is not representable by IEEEdouble.");
    let temp = self.clone();
    let status =
      temp.convert(&&FltSemantics::ieee_double(), RoundingMode::NearestTiesToEven);
    debug_assert!(status.0 != OpStatus::Inexact && status.1 == false,
      "Unexpected imprecision.");

    temp.get_ieee().convert_to_double()
  }

  // Converts this APFloat to host float value.
  // The APFloat must be built using semantics, that can be represented by the
  // host float type without loss of precision.
  // It can be IEEESingle and shorter semantics, like IEEEhalf.
  pub fn convert_to_float(&self) -> f32 {
    if self.semantics == FltSemantics::ieee_single() {
      return self.get_ieee().convert_to_float();
    }
    debug_assert!(self.semantics.is_representable_by(&FltSemantics::ieee_single()),
      "Float semantics is not representable by IEEEsingle.");
    let temp = self.clone();
    let status =
      temp.convert(&FltSemantics::ieee_single(), RoundingMode::NearestTiesToEven);
    debug_assert!(status.0 != OpStatus::Inexact && status.1 == false,
      "Unexpected imprecision.");

    temp.get_ieee().convert_to_float()
  }

  pub fn compare(&self, rhs: &APFloat) -> CmpResult {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.compare(&rhs.ieee);
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.compare(&rhs.double);
    }
    panic!("Unexpected semantics.");
  }

  pub fn bitwise_is_equal(&self) -> bool {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.bitwise_is_equal();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.bitwise_is_equal();
    }
    panic!("Unexpected semantics.");
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

  pub fn get_semantics(&self) -> &FltSemantics {
    &self.semantics
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
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.is_smallest();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.is_smallest();
    }
    panic!("Unexpected semantics.");
  }

  pub fn is_largest(&self) -> bool {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.is_largest();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.is_largest();
    }
    panic!("Unexpected semantics.");
  }

  pub fn is_integer(&self) -> bool {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.is_integer();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.is_integer();
    }
    panic!("Unexpected semantics.");
  }

  pub fn is_ieee(&self) -> bool {
    APFloat::uses_layout(APFType::IEEEFloat, &self.semantics)
  }

  pub fn is_smallest_normalized(&self) -> bool {
    if APFloat::uses_layout(APFType::IEEEFloat, &self.semantics) {
      return self.ieee.is_smallest_normalized();
    }
    if APFloat::uses_layout(APFType::DoubleAPFloat, &self.semantics) {
      return self.double.is_smallest_normalized();
    }
    panic!("Unexpected semantics.");
  }

  pub fn to_string(&self) {}
  pub fn print(&self) {}
  pub fn dump(&self) {}
  pub fn get_exact_inverse(&self, _inv: Option<APFloat>) -> bool { false }
  pub fn hash_value(&self) {}

  pub fn ilogb(&self) {}
  pub fn scal_bn(&self) {}
  pub fn frexp(&self) {}

  fn uses_layout(t: APFType, s: &FltSemantics) -> bool {
    if t == APFType::DoubleAPFloat {
      return s == &FltSemantics::ppc_double_double();
    } else {
      return s != &FltSemantics::ppc_double_double();
    }
  }
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