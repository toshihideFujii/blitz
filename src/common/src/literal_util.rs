#![allow(dead_code)]

use crate::{array3d::Array3D, blitz_data::PrimitiveType,
  layout::Layout, layout_util::LayoutUtil, literal::{Literal /*LiteralSlice*/},
  primitive_util::native_to_primitive_type, shape_util::ShapeUtil};

// Utilities for dealing with Literal protobufs.
pub struct LiteralUtil {}

impl LiteralUtil {
  // Returns a literal scalar representing the first element.
  pub fn get_first_scalar_literal(_literal: &Literal) -> &Literal
  {
    unimplemented!()
  }

  // Returns a literal scalar representing the element at `multi_index`.
  pub fn get_scalar_literal<T>(
    _literal: Literal, _multi_index: Vec<i64>) -> Literal
    where T: Clone + Default + PartialEq
  {
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
  // The variants not ending with WithLayout use the default Blitz layout for the
  // literal's linear representation in memory.
  pub fn create_r0<NativeT>(value: NativeT) -> Literal
    where NativeT: 'static + Clone + Default
  {
    let shape = ShapeUtil::make_shape(
      &native_to_primitive_type(&value), vec![]);
    let mut literal = Literal::new_from_shape(&shape);
    literal.set_at_root(&vec![], value);
    literal
  }

  pub fn create_r1<NativeT>(values: &Vec<NativeT>) -> Literal
    where NativeT: 'static + Clone + Default
  {
    let mut native_value = &NativeT::default();
    if !values.is_empty() {
      native_value = &values[0];
    }
    let shape = ShapeUtil::make_shape(
      &native_to_primitive_type(native_value),
      vec![values.len() as i64]);
    let mut literal = Literal::new_from_shape(&shape);
    literal.populate_r1::<NativeT>(values);
    literal
  }

  pub fn create_r2_with_layout<NativeT>(
    values: &Vec<Vec<NativeT>>, layout: &Layout) -> Literal
    where NativeT: 'static + Clone + Default
  {
    let mut value = &NativeT::default();
    if !values.is_empty() {
      value = &values[0][0];
    }
    let shape = ShapeUtil::make_shape_with_dense_layout(
      &native_to_primitive_type::<NativeT>(value),
      &vec![values.len() as i64, values[0].len() as i64],
      layout.minor_to_major_vec(),
      vec![],
      1,
      0,
      0);
    let mut literal = Literal::new_from_shape(&shape);
    literal.populate_r2::<NativeT>(values);
    literal
  }

  pub fn create_r2<NativeT>(values: &Vec<Vec<NativeT>>) -> Literal
    where NativeT: 'static + Clone + Default
  {
    LiteralUtil::create_r2_with_layout(values,
      &LayoutUtil::get_default_layout_for_r2())
  }

  pub fn create_r3<NativeT>(
    values: &Vec<Vec<Vec<NativeT>>>) -> Literal
    where NativeT: Clone + Default + PartialEq
  {
    LiteralUtil::create_r3_with_layout(values,
      &LayoutUtil::get_default_layout_for_r3())
  }

  pub fn create_r3_with_layout<NativeT>(
   _values: &Vec<Vec<Vec<NativeT>>>, _layout: &Layout) -> Literal
    where NativeT: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  pub fn create_r4() {}
  pub fn create_r4_with_layout() {}

  // Creates a scalar literal value zero of the given primitive type.
  pub fn zero<T>(_primitive_type: &PrimitiveType) -> Literal
    where T: Clone
  {
    unimplemented!()
  }

  // Creates a scalar literal value one of the given primitive type.
  pub fn one<T>(_primitive_type: &PrimitiveType) -> Literal
    where T: Clone
  {
    unimplemented!()
  }

  pub fn min_value<T>(_primitive_type: &PrimitiveType) -> Literal
    where T: Clone
  {
    unimplemented!()
  }

  pub fn max_value<T>(_primitive_type: &PrimitiveType) -> Literal
    where T: Clone
  {
    unimplemented!()
  }

  pub fn nan_value() {}
  pub fn create_full_with_descending_layout() {}

  pub fn create_from_array() {}

  pub fn craete_from_array_with_layout<NativeT>(
    values: &Array3D<NativeT>, layout: &Layout) -> Literal
    where NativeT: Clone + Default + PartialEq
  {
    let _literal = ShapeUtil::make_shape_with_dense_layout(
      &native_to_primitive_type(&NativeT::default()),
      values.dimensions(),
      layout.minor_to_major_vec(),
      vec![],
      1,
      0,
      0);

    unimplemented!()
  }

  pub fn create_r2_from_array_2d() {}
  pub fn create_r2_from_array_2d_with_layout() {}
  pub fn create_r3_from_array_3d() {}

  pub fn create_r3_from_array_3d_with_layout<NativeT>(
    _values: Array3D<NativeT>, _layout: &Layout) -> Literal
    where NativeT: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  pub fn create_r4_from_array_4d() {}
  pub fn create_r4_from_array_4d_with_layout() {}

  // Create a constant token literal. Token types have no value.
  pub fn create_token<T>() -> Literal
    where T: Clone + Default + PartialEq
  {
    Literal::new_from_shape(&ShapeUtil::make_token_shape())
  }

  // Creates a new Literal object with its values havings the primitive_type
  // type, and with dimensions defined by the dimensions parameter.
  // The content of the literal values is the default value of the primitive
  // type of literal itself (0 for numeric types, and false for predicates).
  pub fn create_from_dimensions(
    primitive_t: &PrimitiveType, dimensions: Vec<i64>) -> Literal
  {
    Literal::new_from_shape(
      &ShapeUtil::make_shape(primitive_t, dimensions))
  }

  // Returns a tuple literal composed of given literals. Data is copied from the
  // given elements into the returned literal.
  pub fn make_tuple(elements: &Vec<&Literal>) -> Literal
  {
    let mut element_shapes = vec![];
    for element in elements {
      element_shapes.push(element.shape().clone());
    }
    let mut literal = Literal::new_from_shape(
      &ShapeUtil::make_tuple_shape(element_shapes));
    for i in 0..elements.len() {
      let result = literal.copy_from(
        elements[i],
        &vec![i as i64],
        &vec![],
        false);
      debug_assert!(result.is_ok());
    }
    literal
  }

  pub fn make_tuple_owned(_elements: Vec<Literal>) -> Literal
  {
    unimplemented!()
  }
}