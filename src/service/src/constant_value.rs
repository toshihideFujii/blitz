#![allow(dead_code)]

// Class used to represent a constant.
pub struct ConstantValue {
  value: u64,
  bit_width: i32,
  is_signed: bool
}

impl ConstantValue {
  pub fn new() {}
  pub fn get_zero() {}
  pub fn get_one() {}
  pub fn get_signed() {}
  pub fn get_unsigned() {}
  pub fn from_literal() {}
  pub fn add() {}
  pub fn sub() {}
  pub fn div() {}
  pub fn modulo() {}
  pub fn mul() {}
  pub fn lt() {}
  pub fn gt() {}
  pub fn get_signed_value() {}
  pub fn get_unsigned_value() {}
  pub fn get_bit_width() {}
  pub fn is_signed() {}
  pub fn to_string() {} 
}