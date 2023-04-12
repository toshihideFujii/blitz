#![allow(dead_code)]

// This file declares a class to represent arbitrary precision
// floating point values and provide a variety of arithmetic
// operations on them.

// Enum that represents what fraction of the LSB truncated bits
// of an fp number represent.
enum LostFraction {
  ExactlyZero,
  LessThanZero,
  ExactlyHalf,
  MoreThanHalf
}

// Floating point semantics.
enum Semantics {
  IEEEHalf,
  BFloat,
  IEEESingle,
  IEEEDouble,
  IEEEQuad,
  PPCDoubleDouble,
  Float8E5M2,
  Float8E5M2FNUZ,
  Float8E4M3FNUZ,
  X87DoubleExtended
}

// IEEE-754R 5.11: floating point comparison relations.
enum CmpResult {
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
enum FltCategory {
  Infinity,
  Nan,
  Normal,
  Zero
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct APFloat {}

impl APFloat {
  pub fn enum_to_semantics() {}
  pub fn semantics_to_enum() {}
  pub fn ieee_half() {}
  pub fn b_float() {}
  pub fn ieee_single() {}
  pub fn ieee_double() {}
  pub fn ieee_quad() {}
  pub fn ppc_double_double() {}
  pub fn float_8e5m2() {}
  pub fn float_8e5m2_fnuz() {}
  pub fn float_8e4m3_fn() {}
  pub fn float_8e4m3_fnuz() {}
  pub fn x87_double_extended() {}

  pub fn bogus() {}

  pub fn semantics_precision() {}
  pub fn semantics_min_exponent() {}
  pub fn semantics_max_exponent() {}
  pub fn semantics_size_in_bits() {}
  pub fn semantics_int_size_in_bits() {}

  pub fn get_size_in_bits() {}

  pub fn new() {}
  pub fn needs_cleanup() {}

  pub fn add() {}
  pub fn subtract() {}
  pub fn multiply() {}
  pub fn divide() {}
  pub fn reminder() {}
  pub fn modulo() {}
  pub fn fuse_multiply_add() {}
  pub fn round_to_integral() {}
  pub fn next() {}

  pub fn cahnge_sign() {}
  pub fn convert() {}
  pub fn convert_to_integer() {}
  pub fn convert_from_apint() {}
  pub fn convert_from_sign_extended_integer() {}
  pub fn convert_from_zero_extended_integer() {}
  pub fn convert_from_string() {}
  pub fn bitcast_to_apint() {}
  pub fn convert_to_double() {}
  pub fn convert_to_float() {}

  pub fn compare() {}
  pub fn bitwise_is_equal() {}
  pub fn convert_to_hex_string() {}
  pub fn is_negative() {}
  pub fn is_normal() {}
  pub fn is_finite() {}
  pub fn is_zero() {}
  pub fn is_denormal() {}
  pub fn is_infinity() {}
  pub fn is_nan() {}
  pub fn is_signaling() {}

  pub fn get_category() {}
  pub fn get_semantics() {}
  pub fn is_non_zero() {}
  pub fn is_finite_non_zero() {}
  pub fn is_pos_zero() {}
  pub fn is_neg_zero() {}

  pub fn is_smallest() {}
  pub fn is_smallest_normalized() {}
  pub fn is_largest() {}
  pub fn is_integer() {}

  pub fn hash_value() {}

  pub fn to_string() {}
  pub fn get_exact_inverse() {}

  pub fn ilogb() {}
  pub fn scal_bn() {}
  pub fn frexp() {}

  pub fn make_largest() {}
  pub fn make_smallest() {}
  pub fn make_nan() {}
  pub fn make_inf() {}
  pub fn make_zero() {}
  pub fn make_quiet() {}
  pub fn make_smallest_normalized() {}
  pub fn compare_absolute_value() {}
}

struct IEEEFloat {}
impl IEEEFloat {
  pub fn new() {}
  pub fn needs_cleanup() {}

  pub fn add() {}
  pub fn subtract() {}
  pub fn multiply() {}
  pub fn divide() {}
  pub fn reminder() {}
  pub fn modulo() {}
  pub fn fuse_multiply_add() {}
  pub fn round_to_integral() {}
  pub fn next() {}

  pub fn cahnge_sign() {}
  pub fn convert() {}
  pub fn convert_to_integer() {}
  pub fn convert_from_apint() {}
  pub fn convert_from_sign_extended_integer() {}
  pub fn convert_from_zero_extended_integer() {}
  pub fn convert_from_string() {}
  pub fn bitcast_to_apint() {}
  pub fn convert_to_double() {}
  pub fn convert_to_float() {}

  pub fn compare() {}
  pub fn bitwise_is_equal() {}
  pub fn convert_to_hex_string() {}
  pub fn is_negative() {}
  pub fn is_normal() {}
  pub fn is_finite() {}
  pub fn is_zero() {}
  pub fn is_denormal() {}
  pub fn is_infinity() {}
  pub fn is_nan() {}
  pub fn is_signaling() {}

  pub fn get_category() {}
  pub fn get_semantics() {}
  pub fn is_non_zero() {}
  pub fn is_finite_non_zero() {}
  pub fn is_pos_zero() {}
  pub fn is_neg_zero() {}

  pub fn is_smallest() {}
  pub fn is_smallest_normalized() {}
  pub fn is_largest() {}
  pub fn is_integer() {}

  pub fn hash_value() {}

  pub fn to_string() {}
  pub fn get_exact_inverse() {}

  pub fn ilogb() {}
  pub fn scal_bn() {}
  pub fn frexp() {}

  pub fn make_largest() {}
  pub fn make_smallest() {}
  pub fn make_nan() {}
  pub fn make_inf() {}
  pub fn make_zero() {}
  pub fn make_quiet() {}
  pub fn make_smallest_normalized() {}
  pub fn compare_absolute_value() {}
}

struct DoubleAPFloat {}
impl DoubleAPFloat {
  pub fn new() {}
  pub fn needs_cleanup() {}

  pub fn add() {}
  pub fn subtract() {}
  pub fn multiply() {}
  pub fn divide() {}
  pub fn reminder() {}
  pub fn modulo() {}
  pub fn fuse_multiply_add() {}
  pub fn round_to_integral() {}
  pub fn next() {}

  pub fn cahnge_sign() {}
  pub fn convert() {}
  pub fn convert_to_integer() {}
  pub fn convert_from_apint() {}
  pub fn convert_from_sign_extended_integer() {}
  pub fn convert_from_zero_extended_integer() {}
  pub fn convert_from_string() {}
  pub fn bitcast_to_apint() {}
  pub fn convert_to_double() {}
  pub fn convert_to_float() {}

  pub fn compare() {}
  pub fn bitwise_is_equal() {}
  pub fn convert_to_hex_string() {}
  pub fn is_negative() {}
  pub fn is_normal() {}
  pub fn is_finite() {}
  pub fn is_zero() {}
  pub fn is_denormal() {}
  pub fn is_infinity() {}
  pub fn is_nan() {}
  pub fn is_signaling() {}

  pub fn get_category() {}
  pub fn get_semantics() {}
  pub fn is_non_zero() {}
  pub fn is_finite_non_zero() {}
  pub fn is_pos_zero() {}
  pub fn is_neg_zero() {}

  pub fn is_smallest() {}
  pub fn is_smallest_normalized() {}
  pub fn is_largest() {}
  pub fn is_integer() {}

  pub fn hash_value() {}

  pub fn to_string() {}
  pub fn get_exact_inverse() {}

  pub fn ilogb() {}
  pub fn scal_bn() {}
  pub fn frexp() {}

  pub fn make_largest() {}
  pub fn make_smallest() {}
  pub fn make_nan() {}
  pub fn make_inf() {}
  pub fn make_zero() {}
  pub fn make_quiet() {}
  pub fn make_smallest_normalized() {}
  pub fn compare_absolute_value() {}
}