#![allow(dead_code)]

use std::{collections::HashMap, sync::OnceLock};
use crate::blitz_data::PrimitiveType;

static PRIMITIVE_TYPE_MAP:
  OnceLock<HashMap<&'static str, PrimitiveType>> = OnceLock::new();

fn get_or_create_map() -> &'static HashMap<&'static str, PrimitiveType> {
  let primitive_type_map = PRIMITIVE_TYPE_MAP.get_or_init(|| {
    let mut map = HashMap::new();
    map.insert("pred", PrimitiveType::Pred);
    map.insert("s4", PrimitiveType::S4);
    map.insert("s8", PrimitiveType::S8);
    map.insert("s16", PrimitiveType::S16);
    map.insert("s32", PrimitiveType::S32);
    map.insert("s64", PrimitiveType::S64);
    map.insert("u4", PrimitiveType::U4);
    map.insert("u8", PrimitiveType::U8);
    map.insert("u16", PrimitiveType::U16);
    map.insert("u32", PrimitiveType::U32);
    map.insert("u64", PrimitiveType::U64);
    map.insert("f16", PrimitiveType::F16);
    map.insert("f32", PrimitiveType::F32);
    map.insert("bf16", PrimitiveType::BF16);
    map.insert("f64", PrimitiveType::F64);
    map.insert("f8e5m2", PrimitiveType::F8E5M2);
    map.insert("f8e4m3fn", PrimitiveType::F8E4M3FN);
    map.insert("f8e4m3b11fnuz", PrimitiveType::F8E4M3B11FNUZ);
    map.insert("c64", PrimitiveType::C64);
    map.insert("c128", PrimitiveType::C128);
    map.insert("tuple", PrimitiveType::Tuple);
    map.insert("token", PrimitiveType::Token);
    map.insert("opaque", PrimitiveType::OpaqueType);
    map
  });
  primitive_type_map
}

pub fn significand_width(_t: &PrimitiveType) -> i64 {
  0 // TODO
}

pub fn exponent_width(t: &PrimitiveType) -> i64 {
  let total_bit_width = bit_width(t);
  let trailing_significand_field_width = significand_width(t) - 1;
  let sign_bit_width = 1;
  total_bit_width - (trailing_significand_field_width + sign_bit_width)
}

pub fn underflow_exponent(_t: &PrimitiveType) -> i64 {
  0 // TODO
}

pub fn overflow_exponent(_t: &PrimitiveType) -> i64 {
  unimplemented!()
}

pub fn exponent_bias(t: &PrimitiveType) -> i64 {
  (1 - underflow_exponent(t)) + 1
}

pub fn has_infinity(_t: &PrimitiveType) -> bool {
  unimplemented!()
}

pub fn has_nan(_t: &PrimitiveType) -> bool {
  unimplemented!()
}

pub fn has_negative_zero(_t: &PrimitiveType) -> bool {
  unimplemented!()
}

// Declarations of specializations for each native type which correspond to a
// Blitz primitive type.
pub fn native_to_primitive_type<T>(v: &T) -> PrimitiveType {
  if std::any::type_name_of_val(v) == "bool" {
    return PrimitiveType::Pred;
  }
  // Unsigned integer
  else if std::any::type_name_of_val(v) == "u8" {
    return PrimitiveType::U8;
  } else if std::any::type_name_of_val(v) == "u16" {
    return PrimitiveType::U16;
  } else if std::any::type_name_of_val(v) == "u32" {
    return PrimitiveType::U32;
  } else if std::any::type_name_of_val(v) == "u64" {
    return PrimitiveType::U64;
  } else if std::any::type_name_of_val(v) == "u8" {
    return PrimitiveType::U8;
  }
  // Signed integer
  else if std::any::type_name_of_val(v) == "i8" {
    return PrimitiveType::S8;
  } else if std::any::type_name_of_val(v) == "i16" {
    return PrimitiveType::S16;
  } else if std::any::type_name_of_val(v) == "i32" {
    return PrimitiveType::S32;
  } else if std::any::type_name_of_val(v) == "i64" {
    return PrimitiveType::S64;
  }
  // Floating point
  else if std::any::type_name_of_val(v) == "f32" {
    return PrimitiveType::F32;
  } else if std::any::type_name_of_val(v) == "f64" {
    return PrimitiveType::F64;
  }
  // Complex
  else if std::any::type_name_of_val(v) == "num_complex::Complex<f64>" {
    return PrimitiveType::C64;
  } else if std::any::type_name_of_val(v) == "c128" {
    return PrimitiveType::C128;
  }
  // Vec
  else if std::any::type_name_of_val(v) == "alloc::vec::Vec<i32>" {
    return PrimitiveType::S32;
  } else if std::any::type_name_of_val(v) == "alloc::vec::Vec<u32>" {
    return PrimitiveType::U32;
  }else if std::any::type_name_of_val(v) == "alloc::vec::Vec<f64>" {
    return PrimitiveType::F64;
  } else if std::any::type_name_of_val(v) == "alloc::vec::Vec<num_complex::Complex<f64>>" {
    return PrimitiveType::C64;
  } else if std::any::type_name_of_val(v) == "alloc::vec::Vec<i64>" {
    return PrimitiveType::S64;
  } else if std::any::type_name_of_val(v) == "alloc::vec::Vec<u64>" {
    return PrimitiveType::U64;
  }

  unreachable!()
}

pub fn primitive_type_to_native() {}

pub fn primitive_type_switch<F, R>(f: &mut F, t: PrimitiveType) -> R
  where F: FnMut(PrimitiveType) -> R
{
  if is_array_type(&t) {
    return array_type_switch(f, &t);
  }
  if t == PrimitiveType::Tuple {
    return f(PrimitiveType::Tuple);
  }
  if t == PrimitiveType::Token {
    return f(PrimitiveType::Token);
  }
  if t == PrimitiveType::OpaqueType {
    return f(PrimitiveType::OpaqueType);
  }
  panic!("Unhandled type: {:?}", t);
}

pub fn is_f8_type(t: &PrimitiveType) -> bool {
  *t == PrimitiveType::F8E5M2 || *t == PrimitiveType::F8E4M3FN ||
  *t == PrimitiveType::F8E4M3B11FNUZ || *t == PrimitiveType::F8E5M2FNUZ ||
  *t == PrimitiveType::F8E4M3FNUZ
}

pub fn is_floating_point_type(t: &PrimitiveType) -> bool {
  *t == PrimitiveType::F16 || *t == PrimitiveType::F32 || *t == PrimitiveType::F64 ||
  *t == PrimitiveType::F64 || *t == PrimitiveType::BF16 || is_f8_type(t)
}

pub fn is_complex_type(t: &PrimitiveType) -> bool {
  *t == PrimitiveType::C64 || *t == PrimitiveType::C128
}

pub fn is_signed_integral_type(t: &PrimitiveType) -> bool {
  *t == PrimitiveType::S4 || *t == PrimitiveType::S8 || *t == PrimitiveType::S16 ||
  *t == PrimitiveType::S32 || *t == PrimitiveType::S64
}

pub fn is_unsigned_integral_type(t: &PrimitiveType) -> bool {
  *t == PrimitiveType::U4 || *t == PrimitiveType::U8 || *t == PrimitiveType::U16 ||
  *t == PrimitiveType::U32 || *t == PrimitiveType::U64
}

pub fn is_integral_type(t: &PrimitiveType) -> bool {
  is_unsigned_integral_type(t) || is_signed_integral_type(t)
}

pub fn is_4bit_type(t: &PrimitiveType) -> bool {
  *t == PrimitiveType::S4 || *t == PrimitiveType::U4
}

pub fn is_array_type(t: &PrimitiveType) -> bool {
  *t != PrimitiveType::Invalid &&
  *t != PrimitiveType::Tuple &&
  *t != PrimitiveType::OpaqueType &&
  *t != PrimitiveType::Token
}

pub fn primitive_type_bit_width() {
    
}

pub fn bit_width_array_helper() {
    
}

pub fn bit_width(_t: &PrimitiveType) -> i64 {
  0 // TODO
}

// Returns the number of bytes in the representation for a given type.
pub fn byte_width(t: &PrimitiveType) -> i64 {
  match t {
    PrimitiveType::Pred => return 2, // ?
    PrimitiveType::S4 => return 0,
    PrimitiveType::S8 => return 1, // 8 / 8
    PrimitiveType::S16 => return 2, // 16 / 8
    PrimitiveType::S32 => return 4, // 32 / 8
    PrimitiveType::S64 => return 8, // 64 / 8
    PrimitiveType::U8 => return 1, // 8 / 8
    PrimitiveType::U16 => return 2, // 16 / 8
    PrimitiveType::U32 => return 4, // 32 / 8
    PrimitiveType::U64 => return 8, // 64 / 8
    PrimitiveType::BF16 => return 2, // 16 / 8
    PrimitiveType::F16 => return 2, // 16 / 8
    PrimitiveType::F32 => return 4, // 32 / 8
    PrimitiveType::F64 => return 8, // 64 / 8
    PrimitiveType::C64 => return 8, // 64 / 8
    PrimitiveType::C128 => return 16, // 128 / 8
    PrimitiveType::Token => return 0,
    PrimitiveType::Tuple => unreachable!("Tuple is an invalid type for bit_width."),
    PrimitiveType::OpaqueType => unreachable!("OpaqueType is an invalid type for bit_width."),
    _ => unreachable!("Unhandled primitive type."),   
  };
}

pub fn unsigned_integral_type_for_bit_width(src_bitwidth: i64) -> PrimitiveType {
  match src_bitwidth {
    4 => PrimitiveType::U4,
    8 => PrimitiveType::U8,
    16 => PrimitiveType::U16,
    32 => PrimitiveType::U32,
    64 => PrimitiveType::U64,
    _ => PrimitiveType::Invalid
  }
}

pub fn signed_integral_type_for_bit_width(src_bitwidth: i64) -> PrimitiveType {
  match src_bitwidth {
    4 => PrimitiveType::S4,
    8 => PrimitiveType::S8,
    16 => PrimitiveType::S16,
    32 => PrimitiveType::S32,
    64 => PrimitiveType::S64,
    _ => PrimitiveType::Invalid
  }
}

pub fn complex_component_type(complex_type: &PrimitiveType) -> PrimitiveType {
  match complex_type {
    PrimitiveType::C64 => PrimitiveType::F32,
    PrimitiveType::C128 => PrimitiveType::F64,
    _ => unreachable!("Primitive type is not complex.")
  }
}

pub fn complex_type(base_type: PrimitiveType) -> PrimitiveType {
  if base_type == PrimitiveType::F32 {
    return PrimitiveType::C64;
  } else if base_type == PrimitiveType::F64 {
    return PrimitiveType::C128;
  }
  PrimitiveType::Invalid
}

pub fn higher_precision_type(a: &PrimitiveType, _b: &PrimitiveType) -> PrimitiveType {
  a.clone() // TODO
}

// Returns true if a conversion from from_type to to_type loses no precision.
pub fn cast_preserves_values(
  from_t: &PrimitiveType,
  to_t: &PrimitiveType) -> bool
{
  // * -> *
  if from_t == to_t {
    return true;
  }
  // * -> F8E8M0FNU is not possible because zero cannot be represented.
  if to_t == &PrimitiveType::F8E8M0FNU {
    return false;
  }
  // PRED -> *
  if from_t == &PrimitiveType::Pred {
    return true;
  }
  // ~PRED -> PRED is not safe because it drops almost all numbers.
  if to_t == &PrimitiveType::Pred {
    return false;
  }
  // * -> C is safe if the components of * and C can be safely converted.
  if is_complex_type(to_t) {
    let mut from_component_t = from_t.clone();
    if is_complex_type(from_t) {
      from_component_t = complex_component_type(to_t);
    }
    let to_component_t =
      complex_component_type(to_t);
    return cast_preserves_values(&from_component_t, &to_component_t);
  }
  // ~C -> C is not safe because it drops imaginary components.
  if is_complex_type(from_t) {
    return false;
  }
  // F -> F is safe if the exponent/significand are preserved and `to_type`
  // preserves infinities/nans/unsigned zero in `from_type`.
  if is_floating_point_type(from_t) && is_floating_point_type(to_t) {
    // Target mantissa should be large enough.
    return significand_width(from_t) <= significand_width(to_t) &&
      // Target exponent should be large enough.
      exponent_width(from_t) <= exponent_width(to_t) &&
      // HasInfinity check.
      (!has_infinity(from_t) || has_infinity(to_t)) &&
      // HasNaN check.
      (!has_nan(from_t) || has_nan(to_t)) &&
      // HasNegativeZero check.
      (!has_negative_zero(from_t) || has_negative_zero(to_t)) &&
      // Minimum denormal should be representable by target type.
      (underflow_exponent(from_t) - significand_width(from_t)) >=
        (underflow_exponent(to_t) - significand_width(to_t)) &&
      // Maximum exponent may be larger with custom bias (e.g. F8E4M3B11FNUZ).
      overflow_exponent(from_t) <= overflow_exponent(to_t);
  }
  // F -> I is not safe because it drops fractional numbers.
  if !is_integral_type(from_t) {
    return false;
  }
  // An n-bit unsigned integer takes on values from [0, 2^n - 1].
  // An n-bit signed integer takes on values from [-2^(n-1), 2^(n-1) - 1].
  // from_bits/to_bits considers the number of non-sign bits.
  let mut from_bits = bit_width(from_t);
  if is_signed_integral_type(from_t) {
    from_bits = bit_width(from_t) - 1;
  }
  let mut to_bits = bit_width(to_t);
  if is_signed_integral_type(to_t) {
    to_bits = bit_width(to_t) - 1;
  }
  // I -> F is safe if the integer can be represented exactly.
  if is_floating_point_type(to_t) {
    // In both cases, we need to handle an exponent of n-1.
    // However, the significand needed to represent signed two's complement
    // numbers is smaller by one bit because it will only have a non-zero
    // trailing significand field when the exponent is smaller than n-1.
    return from_bits <= significand_width(to_t) &&
      bit_width(from_t) - 1 < overflow_exponent(to_t);
  }
  // S -> U is not safe because it drops negative numbers.
  if is_signed_integral_type(from_t) && is_unsigned_integral_type(to_t) {
    return false;
  }
  // I -> I is safe if the integer can be represented exactly; we've already
  // ensured that signed to unsigned conversions won't happen here.
  debug_assert!(is_integral_type(to_t));
  from_bits <= to_bits
}

// Returns the PrimitiveType matching the given name. The given name is expected
// to be lower-case.
pub fn string_to_primitive_type(name: &String) -> Option<&PrimitiveType> {
  let map = get_or_create_map();
  map.get(name.as_str())
}

pub fn primitive_type_name(t: &PrimitiveType) -> String {
  match t {
    PrimitiveType::BF16 => "BF16".to_string(),
    PrimitiveType::Pred => "PRED".to_string(),
    PrimitiveType::F16 => "F16".to_string(),
    PrimitiveType::F32 => "F32".to_string(),
    PrimitiveType::F64 => "F64".to_string(),
    PrimitiveType::S4 => "S4".to_string(),
    PrimitiveType::S8 => "S8".to_string(),
    PrimitiveType::S16 => "S16".to_string(),
    PrimitiveType::S32 => "S32".to_string(),
    PrimitiveType::S64 => "S64".to_string(),
    PrimitiveType::U4 => "U4".to_string(),
    PrimitiveType::U8 => "U8".to_string(),
    PrimitiveType::U16 => "U16".to_string(),
    PrimitiveType::U32 => "U32".to_string(),
    PrimitiveType::U64 => "U64".to_string(),
    PrimitiveType::C64 => "C64".to_string(),
    PrimitiveType::C128 => "C128".to_string(),
    PrimitiveType::F8E4M3FN => "F8E4M3FN".to_string(),
    PrimitiveType::F8E4M3FNUZ => "F8E4M3FNUZ".to_string(),
    PrimitiveType::F8E4M3B11FNUZ => "F8E4M3B11FNUZ".to_string(),
    PrimitiveType::F8E5M2 => "F8E5M2".to_string(),
    PrimitiveType::F8E5M2FNUZ => "F8E5M2FNUZ".to_string(),
    PrimitiveType::F8E8M0FNU => "F8E8M0FNU".to_string(),
    PrimitiveType::Tuple => "TUPLE".to_string(),
    PrimitiveType::Token => "TOKEN".to_string(),
    PrimitiveType::OpaqueType => "OPAQUETYPE".to_string(),
    PrimitiveType::Invalid => panic!("Invalid"),
  }
}

// Returns true if the given name is a primitive type string (lower-case).
pub fn is_primitive_type_name(name: &String) -> bool {
  let map = get_or_create_map();
  if map.get(&name.as_str()).is_some() {
    true
  } else {
    false
  }
}

pub fn is_canonical_representation() {
    
}

pub fn fits_in_integral_type() {
    
}

// Returns true if `type` is smaller than 8 bits and is not PRED.
pub fn is_sub_byte_non_pred_type(t: &PrimitiveType) -> bool {
  is_array_type(t) && *t != PrimitiveType::Pred && bit_width(t) < 8
}

struct PrimitiveTypeNameGenerator {
  lowercase_name: HashMap<PrimitiveType, String>,
}

impl PrimitiveTypeNameGenerator {
  pub fn new() -> Self {
    let mut generator = PrimitiveTypeNameGenerator {
      lowercase_name: HashMap::new()
    };
    generator.lowercase_name.insert(PrimitiveType::S32, "s32".to_string());
    generator.lowercase_name.insert(PrimitiveType::U32, "u32".to_string());
    generator.lowercase_name.insert(PrimitiveType::F32, "f32".to_string());
    generator.lowercase_name.insert(PrimitiveType::OpaqueType, "opaque".to_string());
    generator.lowercase_name.insert(PrimitiveType::Token, "token".to_string());
    generator
  }

  pub fn lowercase_name(&self, t: &PrimitiveType) -> String {
    self.lowercase_name.get(t).unwrap().clone()
  }
}

pub fn lowercase_primitive_type_name(t: &PrimitiveType) -> String {
  let gen = PrimitiveTypeNameGenerator::new();
  gen.lowercase_name(t)
}

pub fn integral_type_switch<R, F>(f: &mut F, t: &PrimitiveType) -> R
  where F: FnMut(PrimitiveType) -> R
{
  match t {
    PrimitiveType::S4 => return f(PrimitiveType::S4),
    PrimitiveType::S8 => return f(PrimitiveType::S8),
    PrimitiveType::S16 => return f(PrimitiveType::S16),
    PrimitiveType::S32 => return f(PrimitiveType::S32),
    PrimitiveType::S64 => return f(PrimitiveType::S64),
    PrimitiveType::U4 => return f(PrimitiveType::U4),
    PrimitiveType::U8 => return f(PrimitiveType::U8),
    PrimitiveType::U16 => return f(PrimitiveType::U16),
    PrimitiveType::U32 => return f(PrimitiveType::U32),
    PrimitiveType::U64 => return f(PrimitiveType::U64),
    _ => unreachable!("Not an integral data type: {:?}.", t)
  }
}

pub fn floating_point_type_switch<R, F>(f: &mut F, t: &PrimitiveType) -> R
  where F: FnMut(PrimitiveType) -> R
{
  match t {
    PrimitiveType::F8E4M3FN => return f(PrimitiveType::F8E4M3FN),
    PrimitiveType::F8E4M3B11FNUZ => return f(PrimitiveType::F8E4M3B11FNUZ),
    PrimitiveType::F8E4M3FNUZ => return f(PrimitiveType::F8E4M3FNUZ),
    PrimitiveType::F8E5M2 => return f(PrimitiveType::F8E5M2),
    PrimitiveType::F8E5M2FNUZ => return f(PrimitiveType::F8E5M2FNUZ),
    PrimitiveType::F16 => return f(PrimitiveType::F16),
    PrimitiveType::BF16 => return f(PrimitiveType::BF16),
    PrimitiveType::F32 => return f(PrimitiveType::F32),
    PrimitiveType::F64 => return f(PrimitiveType::F64),
    _ => unreachable!("Not an floating point data type: {:?}.", t)
  }
}

pub fn complex_type_switch<R, F>(f: &mut F, t: &PrimitiveType) -> R
  where F: FnMut(PrimitiveType) -> R
{
  match t {
    PrimitiveType::C64 => return f(PrimitiveType::C64),
    PrimitiveType::C128 => return f(PrimitiveType::C128),
    _ => unreachable!("Not a complex data type: {:?}.", t)
  }
}

pub fn array_type_switch<R, F>(f: &mut F, t: &PrimitiveType) -> R
  where F: FnMut(PrimitiveType) -> R
{
  if is_array_type(t) {
    if is_floating_point_type(t) {
      return floating_point_type_switch(f, t);
    }
    if is_integral_type(t) {
      return integral_type_switch(f, t);
    }
    if is_complex_type(t) {
      return complex_type_switch(f, t);
    }
    if *t == PrimitiveType::Pred {
      return f(PrimitiveType::Pred);
    }
  }
  unreachable!("Not an array data type.");
}