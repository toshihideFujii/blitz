#![allow(dead_code)]


use crate::{blitz_data::PrimitiveType, layout::Layout, literal::{Literal, LiteralBase, /*LiteralSlice*/}};

// Utilities for dealing with Literal protobufs.
pub struct LiteralUtil {}

impl LiteralUtil {
  // Returns a literal scalar representing the first element.
  pub fn get_first_scalar_literal(_literal: &Literal) -> &Literal {
    unimplemented!()
  }

  // Returns a literal scalar representing the element at `multi_index`.
  pub fn get_scalar_literal(_literal: LiteralBase, _multi_index: Vec<i64>) -> Literal {
    unimplemented!()
  }

  pub fn set_scalar_literal() {}

  // Creates a new literal of a given rank. To minimize ambiguity (for users
  // and the compiler) these CreateR[0-2] methods should explicitly specify the
  // native type. For example:
  //
  //  CreateR1<float>({1.0, 42.0});
  //  CreateR2<uint32_t>({{1, 2}, {3, 4}});
  //
  // The variants not ending with WithLayout use the default XLA layout for the
  // literal's linear representation in memory.
  pub fn create_r0<T>(_value: T) -> Literal {
    unimplemented!()
  }

  pub fn create_r1<T>(_primitive_type: PrimitiveType, _value: T) -> Literal {
    unimplemented!()
  }

  pub fn create_r2<T>(_values: Vec<T>) -> Literal {
    unimplemented!()
  }

  pub fn create_r2_with_layout<T>(_values: Vec<T>, _layout: &Layout) -> Literal {
    unimplemented!()
  }

  pub fn create_r3() {}
  pub fn create_r3_with_layout() {}
  pub fn create_r4() {}
  pub fn create_r4_with_layout() {}

  // Creates a scalar literal value zero of the given primitive type.
  pub fn zero(_primitive_type: PrimitiveType) -> Literal {
    unimplemented!()
  }

  // Creates a scalar literal value one of the given primitive type.
  pub fn one(_primitive_type: PrimitiveType) -> Literal {
    unimplemented!()
  }

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