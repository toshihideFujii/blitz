#![allow(dead_code)]

use crate::{blitz_data::PrimitiveType, primitive_util::{is_complex_type, is_floating_point_type, is_signed_integral_type, is_unsigned_integral_type}};

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonDirection {
  Eq,
  Ne,
  Ge,
  Gt,
  Le,
  Lt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonType {
  Float,
  FloatTotalOrder,
  Signed,
  Unsigned,
}


pub fn string_to_comparison_direction(
  _direction: &String) -> Result<ComparisonDirection, String>
{
  unimplemented!()    
}

pub fn string_to_comparison_type(
  _comparison: &String) -> Result<ComparisonType, String>
{
  unimplemented!()    
}

pub fn default_comparison_type(t: &PrimitiveType) -> ComparisonType {
  if is_floating_point_type(t) || is_complex_type(t) {
    return ComparisonType::Float;
  }
  if is_signed_integral_type(t) {
    return ComparisonType::Signed;
  }
  if is_unsigned_integral_type(t) || *t == PrimitiveType::Pred {
    return ComparisonType::Unsigned;
  }
  panic!("Unexpected: {:?}", t);
}