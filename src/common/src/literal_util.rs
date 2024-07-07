#![allow(dead_code)]

use crate::literal::Literal;

pub struct LiteralUtil {}

impl LiteralUtil {
  pub fn get_first_scalar_literal() {}
  pub fn get_scalar_literal() {}
  pub fn set_scalar_literal() {}

  pub fn create_r0() {}
  pub fn create_r1() {}
  pub fn create_r2() {}
  pub fn create_r2_with_layout() {}
  pub fn create_r3() {}
  pub fn create_r3_with_layout() {}
  pub fn create_r4() {}
  pub fn create_r4_with_layout() {}

  pub fn zero() {}
  pub fn one() {}
  pub fn min_value() {}
  pub fn max_value() {}
  pub fn nan_value() {}
  pub fn create_full_with_descending_layout() {}

  pub fn create_from_array() {}
  pub fn craete_from_array_with_layout() {}
  pub fn create_r2_from_array_2d() {}
  pub fn create_r2_from_array_2d_with_layout() {}
  pub fn create_r3_from_array_3d() {}
  pub fn create_r3_from_array_3d_with_layout() {}
  pub fn create_r4_from_array_4d() {}
  pub fn create_r4_from_array_4d_with_layout() {}


  pub fn make_tuple_owned(_elements: Vec<Literal>) -> Literal {
    unimplemented!()
  }
}