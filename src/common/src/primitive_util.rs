#![allow(dead_code)]

use crate::blitz_data::PrimitiveType;

pub fn significant_width() {}

pub fn exponent_width() {}

pub fn underflow_exponent() {}

pub fn overflow_exponent() {}

pub fn exponent_bias() {}

pub fn has_infinity() {}

pub fn native_to_primitive_type() {}

pub fn primitive_type_to_native() {}

pub fn primitive_type_switch() {}

pub fn is_f8_type(t: &PrimitiveType) -> bool {
  *t == PrimitiveType::F8E5M2 || *t == PrimitiveType::F8E4M3FN ||
  *t == PrimitiveType::F8E4M3B11FNUZ || *t == PrimitiveType::F8E5M2FNUZ ||
  *t == PrimitiveType::F8E4M3FNUZ
}

pub fn is_floating_point_type(t: &PrimitiveType) -> bool {
  *t == PrimitiveType::F16 || *t == PrimitiveType::F32 ||
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
  *t != PrimitiveType::Tuple &&
  *t!= PrimitiveType::OpaqueType &&
  *t != PrimitiveType::Token
}

pub fn primitive_type_bit_width() {
    
}

pub fn bit_width_array_helper() {
    
}

pub fn bit_width() {
    
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

pub fn complex_component_type(complex_type: PrimitiveType) -> PrimitiveType {
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

pub fn higher_precision_type() {
    
}

pub fn cast_preserves_values() {
    
}

pub fn lowercase_primitive_type_name() {
    
}

pub fn string_to_primitive_type() {
    
}

pub fn is_primitive_type_name() {
    
}

pub fn is_canonical_representation() {
    
}

pub fn fits_in_integral_type() {
    
}