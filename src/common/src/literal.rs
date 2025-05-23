#![allow(dead_code)]

use core::f64;
use std::{any::Any, fmt::Debug, mem::size_of, vec};
use num::{complex::Complex64, Complex};

use crate::{
  array3d::Array3D, blitz_data::PrimitiveType, index_util::IndexUtil, layout::Layout,
  layout_util::LayoutUtil, permutation_util::{inverse_permutation, is_permutation},
  primitive_util::{self, array_type_switch, complex_type_switch, floating_point_type_switch,
    integral_type_switch, is_array_type, is_complex_type, is_floating_point_type, is_integral_type,
    native_to_primitive_type, primitive_type_switch
  }, printer::{Printer, StringPrinter}, shape::{Shape, ShapeEqual}, shape_util::ShapeUtil
};

// Use just so many bytes that we don't increase the sizeof(Piece).
const MAX_INLINED_BYTES: usize = 100; // TODO

fn scalar_shape(t: &PrimitiveType) -> Shape {
  let mut f = |t: PrimitiveType| -> Shape {
    assert!(primitive_util::is_array_type(&t));
    let mut shape = Shape::new_from_type(&t);
    shape.mutable_layout();
    shape
  };
  primitive_util::array_type_switch(&mut f, t)
}

fn nil_shape() -> Shape {
  Shape::new()
}

fn try_intern_shape(shape: &Shape) -> Option<Shape> {
  if shape.is_tuple() && shape.tuple_shapes_size() == 0 {
    return Some(nil_shape());
  }
  if shape.is_array() && shape.dimensions_size() == 0 && shape.is_static() &&
    shape.layout().as_ref().unwrap().tiles_size() == 0 &&
    shape.layout().as_ref().unwrap().memory_space() == 0
  {
    return Some(scalar_shape(&shape.element_type()));
  }
  None
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralBase<T> where T: Clone + Default + PartialEq + 'static {
  root_piece: Piece<T>,
}

impl<T> LiteralBase<T> where T: Clone + Default + PartialEq + 'static {
  pub fn new() -> Self {
    LiteralBase {
      root_piece: Piece::new(),
    }
  }

  // Returns the shape of the literal.
  pub fn shape(&self) -> &Shape {
    self.root_piece().subshape()
  }

  pub fn mutable_shape(&mut self) -> &mut Shape {
    &mut self.mutable_root_piece().subshape
  }

  pub fn set_shape(&mut self, shape: Shape) {
    self.root_piece.set_subshape(shape);
  }

  // Returns a vec of the array for this literal for the given T
  // (e.g., float). CHECKs if the subshape of the literal at the given
  // ShapeIndex is not array. See primitive_util.h for the mapping from Blitz type
  // to native type.
  pub fn data(&self, shape_index: &Vec<i64>) -> &Vec<T> {
    self.piece(shape_index).data()
  }

  pub fn set_data(&mut self, shape_index: &Vec<i64>, data: Vec<T>) {
    self.mutable_piece(shape_index).set_data(data);
  }

  // Returns a const pointer to (or size of) the underlying buffer holding the
  // array at the given shape index.
  pub fn untyped_data(&self, shape_index: &Vec<i64>) -> &Vec<i64> {
    self.piece(shape_index).untyped_data()
  }

  pub fn size_bytes(&self, shape_index: &Vec<i64>) -> i64 {
    self.piece(shape_index).size_bytes_dense()
  }

  // Computes the size in bytes of the output of the Serialize method.
  pub fn serialized_size(&self) -> Result<i64, String> {
    unimplemented!()
  }

  // Serialize the Literal into the given output iterator, whose value_type must
  // be char.  It's up to the caller to ensure that output can store
  // SerializedSize() bytes of data.  This can be ensured by using
  // std::back_inserter, or by manually resizing the target container.
  // This serializer is useful for bypassing the 2GB protobuf serialization
  // limit with very large literals, and it should be faster than protobuf
  // serialization when performance is a concern.
  // The serialization format should not be relied on for forward/backward
  // compatibility.  If compatibility is required, you should use protobuf
  // serialization instead.
  pub fn serialize(&self) {
    unimplemented!()
  }

  // Serialize the Literal into the given string.  This method has the same
  // caveats as the Serialize() method above.
  pub fn serialize_to_string(&self) {
    unimplemented!()
  }

  // Serialize the Literal into a string and return it.  This method has the
  // same caveats as the Serialize() method above.
  pub fn serialize_as_string(&self) {
    unimplemented!()
  }

  // Returns this literal's data as a string. This literal must be a rank-1 u8 array.
  pub fn get_r1_u8_as_string(&self) -> String {
    assert!(self.shape().is_array());
    assert_eq!(self.shape().rank(), 1);
    assert_eq!(self.shape().element_type(), PrimitiveType::U8);
    ShapeUtil::elements_in(self.shape()).to_string() // TODO
  }

  // Prints a string representation of the literal value. The Shape of the
  // literal is a prefix of the literal value in the string.
  //
  // Warning: this function can take minutes for multi-million element Literals.
  pub fn print(&self, printer: &mut dyn Printer) {
    assert!(LayoutUtil::has_layout(self.shape()));
    self.print_helper(&vec![],
      true, false, false, printer);
  }

  // Similar to Print, but prints the result in a compact one-line form.
  pub fn print_oneline(&self, printer: &mut dyn Printer) {
    assert!(LayoutUtil::has_layout(self.shape()));
    self.print_helper(&vec![],
      true, false, true, printer);
  }

  // Prints a string representation of the literal value which does *not*
  // include the shape string.
  pub fn print_without_shape(&self, printer: &mut dyn Printer) {
    assert!(LayoutUtil::has_layout(self.shape()));
    self.print_helper(&vec![],
      false, false, false, printer);
  }

  // Similar to PrintWithoutShape, but prints the result in a compact one-line
  // form.
  pub fn print_without_shape_oneline(&self, printer: &mut dyn Printer) {
    assert!(LayoutUtil::has_layout(self.shape()));
    self.print_helper(&vec![],
      false, false, true, printer);
  }

  // Prints a string representation of the literal value which includes the
  // shape string with its layout.does *not* include the shape string.
  pub fn print_with_layout(&self, printer: &mut dyn Printer) {
    assert!(LayoutUtil::has_layout(self.shape()));
    self.print_helper(&vec![],
      true, true, false, printer);
  }

  // Similar to PrintWithLayout, but prints the result in a compact one-line
  // form.
  pub fn print_with_layout_oneline(&self, printer: &mut dyn Printer) {
    assert!(LayoutUtil::has_layout(self.shape()));
    self.print_helper(&vec![],
      true, true, true, printer);
  }

  // Returns a string representation of the literal value. The Shape of the
  // literal is a prefix of the literal value in the string.
  //
  // Warning: this function can take minutes for multi-million element Literals.
  pub fn to_string(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print(&mut printer);
    printer.to_string()
  }

  // Similar to ToString, but return the result in a compact one-line form.
  pub fn to_string_oneline(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print_oneline(&mut printer);
    printer.to_string()
  }

  // Returns a string representation of the literal value which does *not*
  // include the shape string.
  pub fn to_string_without_shape(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print_without_shape(&mut printer);
    printer.to_string()
  }

  // Similar to ToStringWithoutShape, but return the result in a compact
  // one-line form.
  pub fn to_string_without_shape_oneline(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print_without_shape_oneline(&mut printer);
    printer.to_string()
  }

  // Returns a string representation of the literal value which includes the
  // shape string with its layout.does *not* include the shape string.
  pub fn to_string_with_layout(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print_with_layout(&mut printer);
    printer.to_string()
  }

  // Similar to ToStringWithLayout, but return the result in a compact one-line
  // form.
  pub fn to_string_with_layout_oneline(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print_with_layout_oneline(&mut printer);
    printer.to_string()
  }

  // Gets an element in the literal at the given index. The multi_index is
  // CHECKed against the dimension sizes.
  pub fn get(&self, multi_index: &Vec<i64>, shape_index: &Vec<i64>) -> &T {
    self.piece(shape_index).get(multi_index)
  }

  // Get the dynamic size on dim_index in the literal at the given shape_index.
  pub fn get_dynamic_size(&self, dim_index: usize, shape_index: &Vec<i64>) -> i64 {
    self.piece(shape_index).get_dynamic_size(dim_index)
  }

  pub fn set_dynamic_size(&self, _dim_index: usize, _size: i32) {
    unimplemented!()
  }

  // Returns the element value at index (0, ..., 0), however many zeroes are
  // required for that index.
  pub fn get_first_element(&self) -> &T { 
    assert!(LayoutUtil::is_dense_array(self.shape()),
      "Only supported for dense arrays.");
    &self.data(&vec![])[0]
  }

  // As above but returns any integer type casted to an int64_t.
  pub fn get_first_integer(&self) -> Option<&i64> {
    if !is_integral_type(&self.shape().element_type()) {
      return None;
    }
    let mut f = |_t: PrimitiveType| -> Option<&i64> {
      let first_element: &dyn Any = self.get_first_element();
      if first_element.is::<i64>() {
        return first_element.downcast_ref::<i64>();
      }
      None
    };
    integral_type_switch(&mut f, &self.shape().element_type())
  }

  // As Get(), but determines the correct type and converts the value into text.
  pub fn get_as_string(&self, multi_index: &Vec<i64>, shape_index: &Vec<i64>) -> String {
    let subshape = ShapeUtil::get_subshape(self.shape(), shape_index);
    assert!(LayoutUtil::is_dense_array(&subshape));
    let mut f = |t: PrimitiveType| -> String {
      if is_integral_type(&t) {
        let value_any: &dyn Any = self.get(multi_index, shape_index);
        if value_any.downcast_ref::<i32>().is_some() {
          let value = value_any.downcast_ref::<i32>().unwrap();
          return value.to_string();
        }
        if value_any.downcast_ref::<i64>().is_some() {
          let value = value_any.downcast_ref::<i64>().unwrap();
          return value.to_string();
        }
        if value_any.downcast_ref::<u32>().is_some() {
          let value = value_any.downcast_ref::<u32>().unwrap();
          return value.to_string();
        }
        if value_any.downcast_ref::<u64>().is_some() {
          let value = value_any.downcast_ref::<u64>().unwrap();
          return value.to_string();
        }
      }
      if is_floating_point_type(&t) {
        //let value = *self.get::<f64>(multi_index, shape_index);
        //return value.to_string();
      }
      if is_complex_type(&t) {
        //let value = self.get::<Complex<f64>>(multi_index, shape_index);
        let mut str = "(".to_string();
        //str.push_str(&value.re().to_string());
        str.push_str(",");
        //str.push_str(&value.im().to_string());
        str.push_str(")");
        return str;
      }
      if t == PrimitiveType::Pred {
        //if *self.get::<bool>(multi_index, shape_index) {
          //return "true".to_string();
        //} else {
          //return "false".to_string();
        //}
      }
      //unreachable!("{:?}", primitive_type_name(&t));
      unreachable!();
    };
    array_type_switch(&mut f, &subshape.element_type())
  }

  // Return whether the value at the specified index is equal to the provided
  // generic `value` (T must be an arithmetic type).
  // Precondition: must be an array.
  pub fn is_equal_at<S>(&self, multi_index: &Vec<i64>, value: &S) -> bool
    where S: 'static
  {
    let as_i64 = self.get_integral_as_s64(multi_index);
    if as_i64.is_some() {
      let value_any: &dyn Any = value;
      let value_i64 = value_any.downcast_ref::<i64>();
      if value_i64.is_some() {
        return *as_i64.unwrap() == *value_i64.unwrap();
      }
      let value_f64 = value_any.downcast_ref::<f64>();
      if value_f64.is_some() {
        return *as_i64.unwrap() as f64 == *value_f64.unwrap();
      }
      let value_c64 = value_any.downcast_ref::<Complex64>();
      if value_c64.is_some() {
        if value_c64.unwrap().im == 0.0 {
          return *as_i64.unwrap() as f64 == value_c64.unwrap().re;
        }
      }
    }

    let as_f64 = self.get_as_double(multi_index);
    if as_f64.is_some() {
      let value_any: &dyn Any = value;
      let value_f64 = value_any.downcast_ref::<f64>();
      if value_f64.is_some() {
        return *as_f64.unwrap() == *value_f64.unwrap();
      }
      let value_i64 = value_any.downcast_ref::<i64>();
      if value_i64.is_some() {
        return *as_f64.unwrap() == *value_i64.unwrap() as f64;
      }
      let value_c64 = value_any.downcast_ref::<Complex64>();
      if value_c64.is_some() {
        if value_c64.unwrap().im == 0.0 {
          return *as_f64.unwrap() == value_c64.unwrap().re;
        }
      }
    }

    let as_c64 = self.get_as_complex_64(multi_index);
    if as_c64.is_some() {
      let value_any: &dyn Any = value;
      let value_f64 = value_any.downcast_ref::<f64>();
      if value_f64.is_some() {
        return as_c64.unwrap().im == 0.0 &&
          as_c64.unwrap().re == *value_f64.unwrap();
      }
      let value_i64 = value_any.downcast_ref::<i64>();
      if value_i64.is_some() {
        return as_c64.unwrap().im == 0.0 &&
          as_c64.unwrap().re == *value_i64.unwrap() as f64;
      }
      let value_c64 = value_any.downcast_ref::<Complex64>();
      if value_c64.is_some() {
        return as_c64.unwrap().re == value_c64.unwrap().re &&
          as_c64.unwrap().im == value_c64.unwrap().im;
      }
    }
    unreachable!("Unsupported type.");
  }

  pub fn is_equal_at_complex(&self, multi_index: &Vec<i64>, value: &Complex<f64>) -> bool {
    let as_f64 = self.get_as_double(multi_index);
    if as_f64.is_some() {
      return *as_f64.unwrap() == value.re && value.im == 0.0;
    }
    let as_complex128 = self.get_as_complex_64(multi_index);
    if as_complex128.is_some() {
      return as_complex128.unwrap() == value;
    }
    unreachable!("Unsupported type.");
  }

  // As Get(), but determines the correct type and converts the value into
  // int64_t.  This literal must be an array.
  pub fn get_integral_as_s64(&self, multi_index: &Vec<i64>) -> Option<&i64> {
    assert!(LayoutUtil::is_dense_array(self.shape()));
    let mut f = |t: PrimitiveType| -> Option<&i64> {
      if is_integral_type(&t) || t == PrimitiveType::Pred {
        let value: &dyn Any = self.get(multi_index, &vec![]);
        return value.downcast_ref::<i64>();
      }
      None
    };
    primitive_type_switch(&mut f, self.shape().element_type())
  }

  // As Get(), but determines the correct type, and converts the value into
  // double. This literal must be an array.
  pub fn get_as_double(&self, multi_index: &Vec<i64>) -> Option<&f64>
  {
    assert!(LayoutUtil::is_dense_array(self.shape()));
    let mut f = |t: PrimitiveType| -> Option<&f64> {
      if is_floating_point_type(&t) {
        let value: &dyn Any = self.get(multi_index, &vec![]);
        return value.downcast_ref::<f64>();
      }
      None
    };
    primitive_type_switch(&mut f, self.shape().element_type())
  }

  // As Get(), but determines the correct type, and converts the value into
  // complex128. All floating point types can be converted into complex128.
  // This literal must be an array.
  pub fn get_as_complex_64(&self, multi_index: &Vec<i64>) -> Option<&Complex<f64>> {
    let mut f = |t: PrimitiveType| -> Option<&Complex<f64>> {
      if is_array_type(&t) {
        if is_complex_type(&t) {
          let value: &dyn Any = self.get(multi_index, &vec![]);
          return value.downcast_ref::<Complex<f64>>();
        }
        if is_floating_point_type(&t) {
          let value: &dyn Any = self.get(multi_index, &vec![]);
          return value.downcast_ref::<Complex<f64>>();
        }
        if is_integral_type(&t) {
          let value: &dyn Any = self.get(multi_index, &vec![]);
          return value.downcast_ref::<Complex<f64>>();
        }
      }
      None
    };
    primitive_type_switch(&mut f, self.shape().element_type())
  }

  // Convert each element whose *linear* index is listed in "linear_indices"
  // to a double and return the sum of all of these elements.
  pub fn get_sum_as_double(&self, linear_indices: &Vec<i64>) -> Option<f64> {
    assert!(LayoutUtil::is_dense_array(self.shape()));
    if !is_floating_point_type(&self.shape().element_type()) {
      return None;
    }
    let mut f = |_t: PrimitiveType| -> Option<f64> {
      let mut sum = 0.0;
      let d = self.root_piece().data();
      for idx in linear_indices {
        let value: &dyn Any = &d[*idx as usize];
        sum += value.downcast_ref::<f64>().unwrap();
      }
      Some(sum)
    };
    floating_point_type_switch(&mut f, &self.shape().element_type())
  }

  pub fn set(&mut self, multi_index: &Vec<i64>, value: T) {
    self.mutable_root_piece().set(multi_index, value);
  }

  // Invokes the "per cell" callback for each element in the provided
  // literal with the element's indices and a string representation of
  // the element's value.
  //
  // This function is useful if you want a polymorphic representation
  // of the tensor's elements (turning it to a string for something
  // like representation in a protobuf).
  //
  // This literal must have a dense layout.
  pub fn each_cell_as_string() {}

  pub fn each_cell<F>(&self, per_cell: F) where F: Fn(&Vec<i64>, &T) {
    assert!(LayoutUtil::is_dense_array(self.shape()),
      "Only supported for dense arrray.");
    if ShapeUtil::is_zero_element_array(self.shape()) {
      return;
    }
    let mut indices = vec![0; self.shape().rank()];
    let mut shape_dynamic = self.shape().clone();
    for i in 0..shape_dynamic.rank() {
      shape_dynamic.set_dimensions(i,
        self.get_dynamic_size(i, &vec![]));
    }
    loop {
      per_cell(&indices, self.get(&indices, &vec![]));
      if !IndexUtil::bump_indices(&shape_dynamic, &mut indices) { break; }
    }
  }

  // Checks whether all of this literal's values are equal to the given
  // scalar literal.
  pub fn is_all(&self, scalar: &Literal<T>) -> bool
    where T: PartialEq
  {
    self.root_piece().is_all(scalar)
  }

  // Returns whether every element in this literal is equal to value.
  //
  // value is an int8_t because we expect this to be called with small
  // compile-time constants (0, -1, etc.) and so that whatever value you pass
  // can be represented exactly by floating-point types as small as 16 bits.
  //
  // If value doesn't fit in this literal's type, returns false.  Values of 1/0
  // are considered equal to true/false; other values are not considered equal
  // to true.
  //
  // Returns false if this literal is not array-shaped.
  pub fn is_all_int(&self, value: i64) -> bool
    where T: PartialEq
  {
    let t = self.shape().element_type();
    if !is_integral_type(&t ){
      return false;
    }
    let mut scalar: Literal<i64> = Literal::new_from_shape(
      &ShapeUtil::make_scalar_shape(&t));
    let mut f = |primitive_t: PrimitiveType| -> bool {
      if primitive_t == PrimitiveType::S32 || primitive_t == PrimitiveType::S64 {
        scalar.set(&vec![], value);
        let val: &dyn Any = &scalar;
        return self.root_piece().is_all(
          val.downcast_ref::<Literal<T>>().unwrap());
      }
      false
    };
    integral_type_switch(&mut f, &t)
  }

  // Like IsAll(int8_t), except we check whether the literal is equal to a
  // particular floating-point or complex number.
  //
  // Returns false if this literal is not a floating-point / complex value, or
  // if it's not an array.
  //
  // This casts value to the type of literal, then compares using ==, with the
  // caveat that NaNs are considered equal. Unlike IsAll, this does not
  // necessarily return false if the value does not fit in this literal's type.
  pub fn is_all_float(&self, value: f64) -> bool
    where T: PartialEq
  {
    let t = self.shape().element_type();
    if !is_floating_point_type(&t) {
      return false;
    }
    let mut scalar: Literal<f64> = Literal::new_from_shape(
      &ShapeUtil::make_scalar_shape(&t));
    let mut f = |primitive_t: PrimitiveType| -> bool {
      if primitive_t == PrimitiveType::F32 || primitive_t == PrimitiveType::F64 {
        scalar.set(&vec![], value);
        let val: &dyn Any = &scalar;
        return self.root_piece().is_all(
          val.downcast_ref::<Literal<T>>().unwrap());
      }
      false
    };
    floating_point_type_switch(&mut f, &t)
  }

  pub fn is_all_complex(&self, value: Complex<f64>) -> bool
    where T: PartialEq
  {
    let t = self.shape().element_type();
    if !is_complex_type(&t) {
      return false;
    }
    let mut scalar: Literal<Complex<f64>> = Literal::new_from_shape(
      &ShapeUtil::make_scalar_shape(&t));
    let mut f = |_primitive_t: PrimitiveType| -> bool {
      scalar.set(&vec![], value);
      let val: &dyn Any = &scalar;
      return self.root_piece().is_all(
        val.downcast_ref::<Literal<T>>().unwrap());
    };
    complex_type_switch(&mut f, &t)
  }

  // Determines if this literal consists of the first element og the literal.
  // Returns false if this literal is not an array.
  pub fn is_all_first(&self) -> bool
    where T: PartialEq
  {
    if !self.shape().is_array() {
      return false;
    }
    // Empty shapes are not all the first element since there is no first element.
    if ShapeUtil::is_zero_element_array(self.shape()) {
      return false;
    }
    let start_indices = vec![0; self.shape().rank()];
    let end_indices = vec![1; self.shape().rank()];
    let first: Literal<T> =
      self.slice(&start_indices, &end_indices);
    self.is_all(&first.base.reshape(&vec![]).unwrap())
  }

  // Returns the number of elements that have value equal to the given complex
  // value. Returns 0 if value does not fit in this literal's type or if the
  // literal is not an array.
  pub fn count_equal(&self, value: &T) -> usize {
    let t = self.shape().element_type();
    if !is_array_type(&t) {
      return 0;
    }
    let mut scalar: Literal<T> = Literal::new_from_shape(
      &ShapeUtil::make_scalar_shape(&t));
    let mut f = |_primitive_t: PrimitiveType| -> usize {
      scalar.set(&vec![], value.clone());
      self.root_piece().count_all(&scalar)
    };
    array_type_switch(&mut f, &t)
  }
/*
  // Literal consists entirely of an iota.
  pub fn is_r1_iota(&self) -> bool {
    if !self.shape().is_array() {
      return false;
    }
    assert!(LayoutUtil::is_dense_array(self.shape()), "Only supported for dense arrays.");
    if self.shape().rank() != 1 {
      return false;
    }
    let mut f = |t: PrimitiveType| -> bool {
      let elements = ShapeUtil::elements_in(self.shape());
      for idx in 0..elements {
        if is_integral_type(&t) {
          if *self.get::<i64>(&vec![idx], &vec![]) != idx {
            return false;
          }
        } else if is_floating_point_type(&t) {
          if *self.get::<f64>(&vec![idx], &vec![]) != idx as f64 {
            return false;
          } 
        } else if is_complex_type(&t) {
          let complex: Complex<f64> = Complex::new(idx as f64, 0.0);
          let value =
            self.get::<Complex<f64>>(&vec![idx], &vec![]);
          if *value != complex {
            return false;
          }
        } else {
          // pred is not iota.
          return false;
        }
      }
      true
    };
    array_type_switch(&mut f, &self.shape().element_type())
  }
*/

/*
  // Returns the stride if the literal is a strided iota.
  pub fn is_r1_strided_iota(&self) -> Option<i64> {
    if !self.shape().is_array() || self.shape().rank() != 1 {
      return None;
    }
    assert!(LayoutUtil::is_dense_array(self.shape()), "Only supported for dense arrays.");
    let elements = ShapeUtil::elements_in(self.shape());
    let primitive_t = self.shape().element_type();
    if elements <= 1 || is_integral_type(&primitive_t) {
      return None;
    }
    let mut f = |_t: PrimitiveType| -> Option<i64> {
      let stride = self.get::<i64>(&vec![1], &vec![]);
      if *stride == 0 { return None; }
      for idx in 0..elements {
        let value = self.get::<i64>(&vec![idx], &vec![]);
        if *value != idx * (*stride) { return None; }
      }
      Some(*stride)
    };
    integral_type_switch(&mut f, &self.shape().element_type())
  }
*/

  // Returns whether this literal is zero at the specified index. This literal
  // must be an array with a dense layout.
  pub fn is_zero_i64(&self, indices: &Vec<i64>) -> bool
    where T: PartialEq<i64>
  {
    assert!(LayoutUtil::is_dense_array(self.shape()), "Only supported dense arrays.");
    let mut f = |t: PrimitiveType| -> bool {
      if t == PrimitiveType::S32 || t == PrimitiveType::S64 {
        return *self.get(indices, &vec![]) == 0;
      } else if t == PrimitiveType::U32 || t == PrimitiveType::U64 {
        return *self.get(indices, &vec![]) == 0;
      }
      false
    };
    array_type_switch(&mut f, &self.shape().element_type())
  }

  pub fn is_zero_f64(&self, indices: &Vec<i64>) -> bool
    where T: PartialEq<f64>
  {
    assert!(LayoutUtil::is_dense_array(self.shape()), "Only supported dense arrays.");
    let mut f = |t: PrimitiveType| -> bool {
      if t == PrimitiveType::F32 || t == PrimitiveType::F64 {
        return *self.get(indices, &vec![]) == 0.0;
      }
      false
    };
    array_type_switch(&mut f, &self.shape().element_type())
  }

  pub fn is_zero_c64(&self, indices: &Vec<i64>) -> bool
    where T: PartialEq<Complex<f64>>
  {
    assert!(LayoutUtil::is_dense_array(self.shape()), "Only supported dense arrays.");
    let mut f = |t: PrimitiveType| -> bool {
      if t == PrimitiveType::C64 || t == PrimitiveType::C128 {
        let c0 = Complex64::new(0.0, 0.0);
        return *self.get(indices, &vec![]) == c0;
      }
      false
    };
    array_type_switch(&mut f, &self.shape().element_type())
  }

  // Returns the count of the elements in the array at the given shape index
  // in this literal.
  pub fn element_count(&self, index_vec: &Vec<i64>) -> i64 {
    if index_vec.is_empty() {
      return ShapeUtil::elements_in(self.shape());
    }
    ShapeUtil::elements_in(
      &ShapeUtil::get_subshape(self.shape(), index_vec))
  }

  // Converts this literal to the given shape. Returns an error is the
  // conversion is not possible.
  pub fn convert_to_shape(&self, dest_shape: &Shape) -> Result<Literal<T>, String> {
    if !dest_shape.is_tuple() {
      return self.convert(&dest_shape.element_type());
    }
    unimplemented!()
  }

  // Converts this literal to another primitive type using a bitcast
  // conversion. Returns an error if the conversion is not possible. This
  // literal must be array-shaped.
  pub fn bitcast_convert(&self, _dest_shape: &Shape) -> Result<Literal<T>, String> {
    unimplemented!()
  }

  // Converts this literal to another primitive type. Returns an error if the
  // conversion is not possible. This literal must be array-shaped.
  pub fn convert(&self, _primitive_dest_t: &PrimitiveType) -> Result<Literal<T>, String> {
    unimplemented!()
  }

  // Creates a new value that has the equivalent value as this
  // literal, but conforms to new_layout; e.g. a literal matrix that was in {0,
  // 1} minor-to-major dimension layout can be re-layed-out as {1, 0}
  // minor-to-major dimension layout and the value in the cell at any given
  // logical index (i0, i1) will be the same.
  //
  // For tuple shaped literals, shape_index should be used to select the inner
  // array that the new layout applies to.
  //
  // Note: this is useful when the client wants to ensure that a value placed in
  // the XLA allocation tracker has a particular layout; for efficiency
  // purposes or avoiding unimplemented operation/layout combinations.
  pub fn relayout(
    &self, new_layout: &Layout, shape_index: &Vec<i64>) -> Literal<T>
  {
    let mut shape_index_clone = vec![];
    shape_index_clone.clone_from(shape_index);
    let mut new_shape = self.shape().clone();
    let subshape = ShapeUtil::get_mutable_subshape(
      &mut new_shape, shape_index_clone);

    assert!(LayoutUtil::validate_layout_for_shape(new_layout, subshape).is_ok());
    subshape.set_layout(new_layout.clone());

    // s4 literals are stored in uint8_t/int8_t, therefore element_size_in_bits
    // must be removed.
    if subshape.layout().as_ref().unwrap().element_size_in_bits() == 4 {
      subshape.mutable_layout().as_mut().unwrap().set_element_size_in_bits(0);
    }

    let mut result = Literal::new_from_shape(&new_shape);
    result.base = self.clone();
    result
  }

  // An overload of Relayout which changes the layout of the entire shape rather
  // than being limited to a single array within the shape.
  pub fn relayout_with_shape(&self, shape: &Shape) -> Literal<T> {
    assert!(ShapeUtil::compatible(shape, self.shape()));
    let result: Literal<T> = Literal::new_from_shape(self.shape());
    let mut f = |subshape: &Shape, _index: &Vec<i64>| {
      if subshape.is_array() {
        // TODO
        /*
        result.copy_from(
          &mut self,
          index,
          index
          false);
        */
      }
    };
    ShapeUtil::for_each_subshape(shape, &mut f);
    result
  }

  // Generate a new literal whose static sizes are equal to the previous
  // literal's dynamic sizes.
  pub fn to_static(&mut self) -> Literal<T> {
    let new_shape = self.mutable_shape();
    let mut f = |subshape: &mut Shape, _index: &Vec<i64>| {
      if !subshape.is_array() { return; }
      for i in 0..subshape.rank() {
        if !subshape.is_dynamic_dimension(i as i64) { continue; }
        subshape.set_dynamic_dimension(i, false);
        //subshape.set_dimensions(i, 
          //self.get_dynamic_size(i, index));
      }
      // TODO
    };
    ShapeUtil::for_each_mutable_subshape(new_shape, &mut f);
    let result = Literal::new_from_shape(new_shape);
    // TODO
    /*
    result.copy_from(
      &mut self,
        index,
        index
        false);
    */
    result
  }

  // Expand a static literal into a new one with a bounded dynamic literal. The
  // static dimensions of the original literal becomes dynamic dimensions of the
  // new literal, where the argument `bounded_shape` becomes the bounded shape
  // of the new literal.
  //
  // Precondition: bounded_shape.is_dynamic()
  pub fn to_bounded_dynamic(&self, bounded_shape: &Shape) -> Literal<T> {
    assert!(bounded_shape.is_dynamic());
    let mut result = Literal::new_from_shape(bounded_shape);
    let mut f = |subshape: &Shape, _index: &Vec<i64>| {
      if !subshape.is_array() { return; }
      for i in 0..subshape.rank() {
        if bounded_shape.is_dynamic_dimension(i as i64) {
          result.set_dynamic_size(
            i,
            &vec![],
            subshape.dimensions(i));
        }
      }
    };
    ShapeUtil::for_each_subshape(self.shape(), &mut f);
    // TODO
    /*
    result.copy_from(
      &mut self,
        index,
        index
        false);
    */
    result
  }

  // Creates a new literal by reshaping this literal to have the given
  // dimensions. The total number of elements must not change; The
  // implementation currently only supports monotonic dim0-major layouts.
  // This literal must be an array.
  #[allow(unused_assignments)]
  pub fn reshape(&self, dimensions: &Vec<i64>) -> Result<Literal<T>, String> {
    if !LayoutUtil::is_dense_array(self.shape()) {
      return Err("Reshape is only supported for dense arrays.".to_string());
    }
    if self.shape().is_dynamic() {
      return Err("Dynamic reshape is not implemented.".to_string());
    }

    let mut output: Literal<T> = Literal::new_from_shape(self.shape());
    if !LayoutUtil::is_monotonic_with_dim0_major(
      self.shape().layout().as_ref().unwrap())
    {
      let layout =
        LayoutUtil::get_default_layout_for_rank(self.shape().rank() as i64);
      output = self.relayout(&layout, &vec![]);
    } else {
      output = Literal {
        base: self.clone(),
        //shape: self.shape().clone()
      };
    }

    // Because the layout is monotonic, we can simply reuse the same sequence of
    // values without changing their order.
    let mut dim = vec![];
    dim.clone_from(dimensions);
    let shape = ShapeUtil::make_shape(
      &self.shape().element_type(), dim);
    output.set_mutable_shape_do_not_use(&shape);

    let elements_before = ShapeUtil::elements_in(self.shape());
    let elements_after = ShapeUtil::elements_in(output.shape());
    if elements_before != elements_after {
      let err_msg = "Shapes before and after Literal::reshape have
        different numbers of dimensions.".to_string();
      return Err(err_msg);
    }
    Ok(output)
  }

  pub fn set_mutable_shape_do_not_use(&mut self, shape: &Shape) {
    LiteralBase::set_piece_shapes(shape, self.mutable_root_piece());
  }

  fn set_piece_shapes(shape: &Shape, piece: &mut Piece<T>) {
    piece.set_subshape(shape.clone());
    if shape.is_tuple() {
      for i in 0..ShapeUtil::tuple_element_count(shape) {
        let subshape = shape.tuple_shapes(i);
        LiteralBase::set_piece_shapes(
          subshape, piece.mutable_child(i).unwrap());
      }
    }
  }
  // Creates a new literal by broadcasting this literal with `dimensions` to
  // yield a literal of shape `result_shape`.
  pub fn broadcast(
    &self,
    _result_shape: &Shape,
    _dimensions: &Vec<i64>) -> Result<Literal<T>, String>
  {
    unimplemented!()
  }

  // Creates a new literal by reordering the dimensions of this literal.
  // The given `permutation` must be a permutation of the dimension numbers
  // in the original literal, and it specifies the order of the new dimensions
  // in the result literal (i.e., new_order[i] = old_order[permutation[i]]).
  // For example, a transpose call on a literal of shape [3 x 8 x 4] and
  // `permutation` = {2, 0, 1} returns a new literal of shape [4 x 3 x 8].
  // This literal must be an array.
  pub fn transpose(&self, permutation: &Vec<i64>) -> Literal<T> {
    assert!(LayoutUtil::is_dense_array(self.shape()),
      "Only supported for dense arrays.");
    assert!(self.shape().rank() == permutation.len() && is_permutation(permutation));

    // To transpose the array, we just permute the dimensions and layout, and
    // do a straight memory copy of the raw data set.
    // This is considerably faster than iterating over every array element using
    // the EachCell<>() and Set<>() APIs.
    let mut permuted_shape = ShapeUtil::permute_dimensions(permutation, self.shape());

    // Replace the layout with one affine to this shape, such that a
    // transpose operation can be performed by leaving the flat values
    // representation intact.
    // For example, consider the shape F32[11,8]{1,0} under a {1,0} permutation.
    // The shape with affine layout resulting from that operation will be
    // F32[8,11]{0,1}, since it leaves the original most minor (the 8 sized), the
    // most minor.
    //
    // Essentially, given MinMaj(Di) the position of the Di dimension within the
    // minor to major vector, and given T(Di) the index that the original Di
    // dimension has within the transposed array, a layout is affine if
    // MinMaj(Di) == TMinMaj(T(Di)), with TMinMaj() being the minor to major
    // vector of the affine layout.
    let inverse_permutation =
      inverse_permutation(permutation);
    assert!(LayoutUtil::is_dense_array(&permuted_shape));
    let layout = permuted_shape.mutable_layout();
    layout.as_mut().unwrap().clear_minor_to_major();
    for index in LayoutUtil::minor_to_major_from_shape(self.shape()) {
      layout.as_mut().unwrap().add_minor_to_major(inverse_permutation[*index as usize]);
    }
    
    let mut new_literal = Literal::new_from_shape(&permuted_shape);
    if self.shape().is_dynamic() {
      for i in 0..self.shape().rank() {
        if self.shape().is_dynamic_dimension(i as i64) {
          new_literal.set_dynamic_size(
            inverse_permutation[i] as usize,
            &vec![],
            self.get_dynamic_size(i, &vec![]));
        }
      }
    }
    // TODO
    new_literal
  }

  // Creates a sub-array from this literal by extracting the indices
  // [start_index, limit_index) of each dimension. The result literal has the
  // same rank and layout as for the given literal. The number of indices in
  // start_indices and limit_indices must be the rank of the literal, and the
  // indices follow the order of the dimensions.
  // This literal must be an array.
  pub fn slice(&self, start_indices: &Vec<i64>, limit_indices: &Vec<i64>) -> Literal<T> {
    assert!(self.shape().is_array(), "Tuple is not supported for slice.");

    let mut result_dimensions = vec![];
    for dnum in 0..self.shape().rank() {
      assert!(start_indices[dnum] >= 0);
      assert!(limit_indices[dnum] <= self.shape().dimensions(dnum));

      let dimension = limit_indices[dnum] - start_indices[dnum];
      assert!(dimension >= 0);
      result_dimensions.push(dimension);
    }
    let mut result_shape = ShapeUtil::make_shape_with_dense_layout(
      &self.shape().element_type(), &result_dimensions,
      LayoutUtil::minor_to_major_from_shape(self.shape()),
      vec![], 1,
      0, 0);

    ShapeUtil::copy_dynamic_dimensions(&mut result_shape, self.shape());
    let mut result_literal = Literal::new_from_shape(&result_shape);
    
    let mut f = |_t: PrimitiveType| {
      LiteralBase::slice_internal(&self, start_indices, &mut result_literal);
    };
    array_type_switch(&mut f, &result_shape.element_type());
    result_literal
  }

  fn slice_internal(
    src_literal: &LiteralBase<T>,
    start_indices: &Vec<i64>,
    result_literal: &mut Literal<T>)
  {
    let result_shape = &result_literal.shape().clone();    
    let mut new_indices = vec![0; result_shape.rank()];
    let f = |indices: &Vec<i64>| -> T {
      for i in 0..result_shape.rank() {
        new_indices[i] = indices[i] + start_indices[i];
      }
      src_literal.get(&new_indices, &vec![]).clone()
    };
    result_literal.populate(f);
    for dnum in 0..src_literal.shape().rank() {
      if src_literal.shape().is_dynamic_dimension(dnum as i64) {
        let mut dynamic_size =
          src_literal.get_dynamic_size(dnum, &vec![])
          - start_indices[dnum];
        assert!(dynamic_size >= 0);
        dynamic_size = i64::min(dynamic_size,
          result_shape.dimensions(dnum));
        result_literal.set_dynamic_size(
          dnum,
          &vec![],
          dynamic_size);
      }
    }
  }

  // Creates a literal with a prepended dimension with bound "times"; e.g. a
  // f32[3x2] with times=4 will produce a f32[4x3x2] with the 3x2 from this
  // literal replicated four times.
  // This literal must be an array.
  pub fn replicate(&self, _times: i64) -> Literal<T> {
    assert!(LayoutUtil::is_dense_array(self.shape()),
      "Only supported for dense arrays.");
    let mut bounds = vec![];
    for bound in self.shape().dimensions_vec() {
      bounds.push(*bound);
    }

    let mut bounds_clone = vec![];
    bounds_clone.clone_from_slice(&bounds);
    let shape = ShapeUtil::make_shape(
      &self.shape().element_type(), bounds_clone);
    let mut literal = Literal::new_from_shape(&shape);
    let elements = ShapeUtil::elements_in(literal.shape());
    if elements == 0 {
      return literal;
    }

    let mut output_indices = vec![0; bounds.len()];
    let input_indices = vec![0; bounds.len()];

    let mut done = false;
    loop {
      if done == true { break; }
      let element = self.get(
        &input_indices, &vec![]);
      literal.set(&output_indices, element.clone());

      done = true;
      for n in 0..output_indices.len() {
        output_indices[n] += 1;
        if output_indices[n] < bounds[n] {
          done = false;
          break;
        }
        output_indices[n] = 0;
      }
    }
    literal
  }

  // Returns true if the leaf arrays of the literal within the given shape_index
  // are all determined.
  pub fn is_determined(&self, shape_index: &Vec<i64>) -> bool {
    self.piece(shape_index).is_determined()
  }

  // Returns true if the leaf arrays of the literal within the given shape_index
  // are all known.
  pub fn is_known(&self, shape_index: &Vec<i64>) -> bool {
    self.piece(shape_index).is_known()
  }

  // Creates a new literal object with the shape specified as parameter.
  // The content of the literal values is the default value of the primitive
  // type of literal itself (0 for numeric types, and false for predicates).
  pub fn create_from_shape(shape: &Shape) -> Literal<T> {
    let mut literal = Literal::new_from_shape(shape);
    literal.mutable_root_piece().for_each_mutable_subpiece(
      &mut |_index: &Vec<i64>, piece: &mut Piece<T>| -> Result<(), String> {
        if piece.subshape().is_array() {
          let len = piece.size_bytes_dense() as usize;
          let untyped_data = piece.mutable_untyped_data();
          for i in 0..len {
            untyped_data[i] = T::default();
          }
          return Ok(());
        } else {
          return Err("piece.subshape is not array type.".to_string());
        }
      }
    );
    literal
  }

  // These two functions are only supposed to be used by HloEvaluator.
  //Similar to create_from_shape() but marks all leaf arrays as unknown.
  pub fn create_from_shape_with_unknown_leaf_arrays(shape: &Shape) -> Literal<T> {
    Literal::new(
      shape,
      false,
      ArrayValueState::Unknown)
  }

  //Similar to create_from_shape() but marks all leaf arrays as undetermined.
  pub fn create_from_shape_with_undetermined_leaf_arrays(shape: &Shape) -> Literal<T> {
    Literal::new(
      shape,
      false,
      ArrayValueState::Undetermined)
  }

  fn piece(&self, shape_index: &Vec<i64>) -> &Piece<T> {
    let mut piece = self.root_piece();
    for i in  shape_index {
      assert!(*i >= 0);
      assert!((*i as usize) < piece.children_size());
      piece = piece.child(*i as usize).unwrap();
    }
    piece
  }

  fn mutable_piece(&mut self, shape_index: &Vec<i64>) -> &mut Piece<T> {
    let mut piece = self.mutable_root_piece();
    for i in  shape_index {
      assert!(*i >= 0);
      assert!((*i as usize) < piece.children_size());
      piece = piece.mutable_child(*i as usize).unwrap();
    }
    piece
  }

  // Returns the piece at the root of the shape.
  fn root_piece(&self) -> &Piece<T> {
    &self.root_piece
  }

  fn mutable_root_piece(&mut self) -> &mut Piece<T> {
    &mut self.root_piece
  }

  fn print_shape(print_layout: bool, shape: &Shape, printer: &mut dyn Printer) {
    if print_layout {
      ShapeUtil::print_human_string_with_layout(printer, shape);
    } else {
      ShapeUtil::print_human_string(printer, shape);
    }
  }

  fn tuple_print_helper(
    &self,
    _shape_index: &Vec<i64>,
    _print_shape: bool,
    _print_layout: bool,
    _one_line: bool,
    _printer: &mut dyn Printer)
  {
      
  }

  fn dense_array_print_helper(
    &self,
    shape_index: &Vec<i64>,
    print_shape: bool,
    print_layout: bool,
    oneline: bool,
    printer: &mut dyn Printer)
  {
    let subshape = ShapeUtil::get_subshape(self.shape(), shape_index);
    let rank = subshape.rank();
    let mut linebreak = " ";
    if !oneline { linebreak = "\n"; }

    if print_shape {
      LiteralBase::<T>::print_shape(print_layout, &subshape, printer);
      if subshape.is_dynamic() {
        printer.append(&"(".to_string());
        for i in 0..subshape.rank() {
          printer.append(&self.get_dynamic_size(i, shape_index).to_string());
          if i < subshape.rank() - 1 {
            printer.append(&",".to_string());
          }
        }
        printer.append(&")".to_string());
      }
      printer.append(&" ".to_string());
    }

    let mut indices: Vec<i64> = vec![];
    let mut dimensions: Vec<i64> = vec![];
    for i in 0..subshape.rank() {
      dimensions.push(self.get_dynamic_size(i, shape_index));
    }
    self.print_recursive(
      shape_index,
      &dimensions,
      &mut indices,
      oneline,
      linebreak,
      &subshape,
      printer,
      rank);
  }

  fn brace_to_string(
    brace: &String,
    dimensions: &Vec<i64>,
    accum_indices: &mut Vec<i64>,
    oneline: bool,
    linebreak: &str,
    rank: usize) -> String
  {
    // Handle 1D tensor
    if rank == 1 { return brace.clone(); }

    // Handle the innermost tensor of a 2D+ tensor.
    if dimensions.len() == 1 && brace == &"{".to_string() {
      let mut result = "".to_string();
      if !oneline { result.push_str(&" ".to_string()); }
      result.push_str(&brace);
      if dimensions[0] > 1 { result.push_str(&" ".to_string()); }
      return result;
    }
    if dimensions.len() == 1 && brace == &"}".to_string() {
      let mut result = "".to_string();
      if dimensions[0] > 1 { result = " ".to_string(); }
      result.push_str(&brace);
      return result;
    }

    // Handle the non-innermost tensors of a 2D+ tensor.
    if brace == &"{".to_string() {
      if rank > 3 && !accum_indices.is_empty() && accum_indices.len() < rank {
        let index = accum_indices.len() - 1;
        let value = accum_indices.last().unwrap(); // CHECK
        let size = dimensions.first().unwrap(); // CHECK
        let mut result = brace.clone();
        result.push_str(" /*i");
        result.push_str(index.to_string().as_str());
        result.push_str("=");
        result.push_str(value.to_string().as_str());
        result.push_str("*/");
        if *size > 0 { result.push_str(linebreak.to_string().as_str()); }
        return result;
      }
      let mut result = brace.clone();
      result.push_str(linebreak);
      return result;
    }

    let mut result = linebreak.to_string();
    result.push_str(brace);
    result
  }

  fn print_recursive(
    &self,
    shape_index: &Vec<i64>,
    dimensions: &Vec<i64>,
    accum_indices: &mut Vec<i64>,
    oneline: bool,
    linebreak: &str,
    subshape: &Shape,
    printer: &mut dyn Printer,
    rank: usize)
  {
    // dimensions.size() decreases by 1 at each recursive call,
    // and accum_indices->size() increases by 1.
    // Their sum is equal to the rank of the tensor.
    assert_eq!(dimensions.len() + accum_indices.len(), rank);

    if dimensions.is_empty() {
      // Display predicates as 0s and 1s so that the string is more dense.
      let mut elem = "".to_string();
      if subshape.element_type() == PrimitiveType::Pred && rank > 0 {
        // TODO
      } else {
        elem = self.get_as_string(&accum_indices, shape_index);
      }
      printer.append(&elem);
    } else {
      printer.append(&LiteralBase::<T>::brace_to_string(
        &"{".to_string(), dimensions, accum_indices, oneline, linebreak, rank));
      for i in 0..dimensions[0] {
        accum_indices.push(i);
        let mut span: Vec<i64> = vec![];
        if !dimensions.is_empty() {
          let mut dim_clone: Vec<i64> = vec![];
          dim_clone.clone_from(&dimensions);
          span = dim_clone.drain(1..).collect();
        }
        self.print_recursive(shape_index, &span, accum_indices, // CHECK: dimensions ??
            oneline, linebreak, subshape, printer, rank);
        accum_indices.pop();
        if i < dimensions[0] - 1 {
          printer.append(&",".to_string());
          if dimensions.len() > 1 {
            printer.append(&linebreak.to_string());
          } else {
            printer.append(&" ".to_string());
          }
        }
      }
      printer.append(&LiteralBase::<T>::brace_to_string(
        &"}".to_string(), dimensions, accum_indices, oneline, linebreak, rank));
    }
  }

  fn print_helper(
    &self,
    shape_index: &Vec<i64>,
    print_shape: bool,
    print_layout: bool,
    oneline: bool,
    printer: &mut dyn Printer)
  {
    let subshape =
      ShapeUtil::get_subshape(self.shape(), shape_index);
    assert!(LayoutUtil::has_layout(self.shape()));
    assert!(LayoutUtil::has_layout(&subshape));

    if subshape.is_tuple() {
      self.tuple_print_helper(shape_index, print_shape,
        print_layout, oneline, printer);
    } else if subshape.is_token() {
      printer.append(&"token".to_string());
    } else {
      assert!(LayoutUtil::is_dense_array(&subshape));
      if self.is_known(shape_index) {
        self.dense_array_print_helper(shape_index, print_shape,
          print_layout, oneline, printer);
      } else {
        LiteralBase::<T>::print_shape(print_layout, &subshape, printer);
        printer.append(&" ".to_string());
        if self.is_determined(shape_index) {
          printer.append(&"unknown".to_string());
        } else {
          printer.append(&"undetermined".to_string());
        }
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal<T> where T: Clone + Default + PartialEq + 'static {
  pub base: LiteralBase<T>,
  //shape: Shape,
}

impl<T> Literal<T> where T: Clone + Default + PartialEq + 'static {
  // Create a literal of the given shape. The literal is allocated sufficient
  // memory to hold the shape. Memory is uninitialized.
  pub fn new_from_shape(shape: &Shape) -> Self {
    Literal::new(
      shape,
      true,
      ArrayValueState::Known)
  }

  // 'allocate_arrays' indicates whether to allocate memory for the arrays in
  // the shape. If false, buffer pointers inside of the Literal::Pieces are set
  // to nullptr.
  pub fn new(
    shape: &Shape,
    allocate_arrays: bool,
    leaf_array_value_state: ArrayValueState) -> Self
  {
    let mut literal = Literal {
      base: LiteralBase::new(),
      //shape: shape.clone()
    };
    assert!(leaf_array_value_state != ArrayValueState::Known ||
      LayoutUtil::has_layout(literal.shape()));

    let mut root_piece: Piece<T> = Piece::new();
    root_piece.set_subshape(shape.clone());
    Literal::set_piece(shape, &mut root_piece,
      allocate_arrays, leaf_array_value_state);

    //literal.base.root_piece.set_subshape(shape.clone());
    literal.base.root_piece = root_piece;
    literal
  }

  pub fn create_from_shape(shape: &Shape) -> Literal<T> {
    LiteralBase::create_from_shape(shape)
  }

  pub fn create_from_shape_with_unknown_leaf_arrays(shape: &Shape) -> Literal<T> {
    LiteralBase::create_from_shape_with_unknown_leaf_arrays(shape)
  }

  pub fn shape(&self) -> &Shape {
    self.base.shape()
  }

  pub fn set_shape(&mut self, shape: Shape) {
    //self.shape = shape.clone();
    self.base.set_shape(shape);
  }

  pub fn root_piece(&self) -> &Piece<T> {
    self.base.root_piece()
  }

  pub fn mutable_root_piece(&mut self) -> &mut Piece<T> {
    self.base.mutable_root_piece()
  }

  pub fn data_default(&self) -> &Vec<T> {
    unimplemented!()
  }

  pub fn data(&self, shape_index: &Vec<i64>) -> &Vec<T> {
    self.base.data(shape_index)
  }

  pub fn set_data(&mut self, shape_index: &Vec<i64>, data: Vec<T>) {
    self.base.set_data(shape_index, data);
  }

  fn set_piece(
    shape: &Shape,
    piece: &mut Piece<T>,
    allocate_arrays: bool,
    leaf_array_value_state: ArrayValueState)
  {
    if shape.is_tuple() {
      for subshape in shape.tuple_shapes_vec() {
        let mut child_piece: Piece<T> = Piece::new();
        child_piece.set_subshape(subshape.clone());
        Literal::set_piece(subshape, &mut child_piece, allocate_arrays,
          leaf_array_value_state.clone());
        piece.emplace_back(child_piece);
      }
    } else if shape.is_array() {
      assert!(LayoutUtil::is_dense_array(shape),
        "Literal array storage is currently only supported for dense array.");
      piece.set_array_value_state(leaf_array_value_state.clone());
      if leaf_array_value_state == ArrayValueState::Known && allocate_arrays {
        // TODO
      }
    }   
  }

  // Copy values from 'src_literal' rooted at 'src_shape_index' into this
  // literal rooted at 'dest_shape_index'. The subshape of this literal rooted
  // at 'dest_shape_index' must be compatible with the subshape of 'src_literal'
  // rooted at 'src_shape_index', but need not be arrays. If only_dynamic_bound
  // is true, only elements within dynamic bounds will be copied.
  pub fn copy_from(
    &mut self,
    src_literal: &mut Literal<T>,
    dest_shape_index: &Vec<i64>,
    src_shape_index: &Vec<i64>,
    only_dynamic_bound: bool) -> Result<(), String>
  {
    let dest_subshape =
      ShapeUtil::get_subshape(self.shape(), dest_shape_index);
    let src_subshape =
      ShapeUtil::get_subshape(src_literal.shape(), src_shape_index);

    if only_dynamic_bound {
      let mut bound_shape = &dest_subshape;
      if dest_subshape.is_static() {
        bound_shape = &src_subshape;
      }
      let mut compact_shape = &src_subshape;
      if dest_subshape.is_static() {
        compact_shape = &dest_subshape;
      }
      assert!(ShapeUtil::dynamic_shape_is_compatible(compact_shape, bound_shape));
    } else {
      if !ShapeUtil::compatible(&dest_subshape, &src_subshape) {
        return Err("Destination subshape incompatible with source subshape.".to_string());
      }
    }

    let mut f =
      |index: &Vec<i64>, piece: &mut Piece<T>| -> Result<(), String>
    {
      if !piece.subshape().is_array() { return Ok(()); }

      // Determine if this index is in the part of this literal that we want
      // to copy over from src_literal.
      let mut in_subtree_to_copy = true;
      if index.len() > 0 && dest_shape_index.len() > 0 {
        for i in 0..dest_shape_index.len() {
          if index[i] != dest_shape_index[i] {
            in_subtree_to_copy = false;
            break;
          }
        }
      }
      if !in_subtree_to_copy {
        return Ok(());
      }
      // Construct the index of the corresponding piece in the source literal.
      let mut src_piece_index = vec![];
      src_piece_index.clone_from(src_shape_index);
      for i in dest_shape_index.len()..index.len() {
        src_piece_index.push(index[i]);
      }
        
      // TODO

      Ok(())
    };
    self.mutable_root_piece().for_each_subpiece_with_status(&mut f)
  }

  // Returns the element value at index (0, ..., 0), however many zeroes are
  // required for that index.
  pub fn get_first_element(&self) -> &T {
    assert!(LayoutUtil::is_dense_array(self.shape()),
      "Only supported for dense arrays.");
    &self.data(&vec![])[0]
  }

  pub fn is_all_int(&self, value: i64) -> bool where T: PartialEq {
    self.base.is_all_int(value)
  }

  pub fn is_all_float(&self, value: f64) -> bool
    where T: PartialEq
  {
    self.base.is_all_float(value)
  }

  pub fn is_all_complex(&self, value: Complex<f64>) -> bool
    where T: PartialEq
  {
    self.base.is_all_complex(value)
  }

  pub fn is_all_first(&self) -> bool where T: PartialEq {
    self.base.is_all_first()
  }

  pub fn is_zero_i64(&self, indices: &Vec<i64>) -> bool
    where T: PartialEq<i64>
  {
    self.base.is_zero_i64(indices)
  }

  pub fn is_zero_f64(&self, indices: &Vec<i64>) -> bool
    where T: PartialEq<f64>
  {
    self.base.is_zero_f64(indices)
  }

  pub fn is_zero_c64(&self, indices: &Vec<i64>) -> bool
    where T: PartialEq<Complex64>
  {
    self.base.is_zero_c64(indices)    
  }

  // Returns true if the leaf arrays of the literal within the given shape index
  // are all known.
  // See comments on ArrayValueState for detailed explanation.
  pub fn is_known(&self, shape_index: &Vec<i64>) -> bool {
    self.base.is_known(shape_index)
  }

  pub fn element_count(&self) -> usize {
    unimplemented!()
  }

  pub fn count_equal(&self, value: &T) -> usize {
    self.base.count_equal(value)
  }

  pub fn is_equal_at<S>(&self, multi_index: &Vec<i64>, value: &S) -> bool
    where S: 'static
  {
    self.base.is_equal_at(multi_index, value)
  }

  pub fn get(&self, multi_index: &Vec<i64>, shape_index: &Vec<i64>) -> &T {
    self.base.get(multi_index, shape_index)
  }

  // As Get(), but determines the correct type, and converts the value into
  // double. This literal must be an array.
  pub fn get_as_double(&self, multi_index: &Vec<i64>) -> Option<&f64> {
    self.base.get_as_double(multi_index)
  }

  // Convert each element whose *linear* index is listed in "linear_indices"
  // to a double and return the sum of all of these elements.
  pub fn get_sum_as_double(&self, linear_indices: &Vec<i64>) -> Option<f64> {
    self.base.get_sum_as_double(linear_indices)
  }

  pub fn get_as_complex_64(&self, multi_index: &Vec<i64>) -> Option<&Complex<f64>> {
    self.base.get_as_complex_64(multi_index)
  }

  pub fn set(&mut self, multi_index: &Vec<i64>, value: T) {
    self.base.set(multi_index, value);
  }

  pub fn get_dynamic_size(&self, dim_index: usize, shape_index: &Vec<i64>) -> i64 {
    self.base.get_dynamic_size(dim_index, shape_index)
  }

  pub fn set_dynamic_size(
    &mut self, dim_index: usize, shape_index: &Vec<i64>, size: i64)
  {
    let mut shape_index_clone = vec![];
    shape_index_clone.clone_from(shape_index);
    let subshape = ShapeUtil::get_mutable_subshape(
      self.mutable_shape_do_not_use(), shape_index_clone);

    assert!(LayoutUtil::is_dense_array(subshape));
    assert!(subshape.dimensions(dim_index) >= size);
    subshape.set_dynamic_dimension(dim_index, true);

    //assert_eq!(self.piece(shape_index).subshape(), subshape);
    self.mutable_piece(shape_index).set_dynamic_size(dim_index, size);
  }

  pub fn untyped_data(&self) -> &Vec<i64> {
    self.base.untyped_data(&vec![])
  }

  pub fn piece(&self, shape_index: &Vec<i64>) -> &Piece<T> {
    self.base.piece(shape_index)
  }

  pub fn mutable_piece(&mut self, shape_index: &Vec<i64>) -> &mut Piece<T> {
    self.base.mutable_piece(shape_index)
  }

  pub fn slice(
    &self, start_indices: &Vec<i64>, limit_indices: &Vec<i64>) -> Literal<T>
  {
    self.base.slice(start_indices, limit_indices)
  }

  pub fn populate<F>(&self, mut generator: F)
    where F: FnMut(&Vec<i64>)->T
  {
    assert!(LayoutUtil::is_dense_array(self.shape()),
      "Only supported for dense arrays.");
    let f = |indexes: &Vec<i64>| -> T {
      generator(indexes)
    };
    let _ = self.populate_internal(f, false);
  }

  pub fn populate_internal<F>(
    &self, _generator: F, _parallel: bool) -> Result<(), String>
    where F: FnMut(&Vec<i64>)->T
  {
    assert!(LayoutUtil::is_dense_array(self.shape()));
    assert!(self.shape().element_type() ==
      native_to_primitive_type(&self.data(&vec![])[0]));
    
    Ok(())
  }

  pub fn populate_inplace_internal<F>(&self, populator: F, parallel: bool)
    where F: Fn(&Vec<i64>, &Vec<i64>, i64)
  {
    let dest_base = self.untyped_data();
    if self.shape().rank() > 0 {
      let mut stride_config = StrideConfig::new(
        self.shape(),
        self.shape(),
        self.shape().dimensions_vec());
      let primitive_size =
        ShapeUtil::byte_size_of_primitive_type(&self.shape().element_type());
      let num_elements = ShapeUtil::elements_in(self.shape());

      // If we are rank-1 and we are `parallel`, it is better to use a smaller
      // `step` than what `StrideConfig` does: stick the entire dimension in the
      // inner-most loop.
      if parallel && self.shape().rank() == 1 {
        let thread_count = ShapeUtil::get_for_each_index_parallel_thread_count();
        stride_config.dest_stride = num_elements;
        stride_config.minor_loop_size = num_elements;
        if num_elements> 32 {
          let mut num = num_elements / (thread_count as i64);
          num = i64::max(num, 1);
          stride_config.dest_stride = num;
          stride_config.minor_loop_size = num;
          stride_config.step = vec![0; stride_config.minor_loop_size as usize];
        }
      }

      let init_func =
        |indexes: &Vec<i64>, thread_id: i64| -> Result<bool, String>
      {
        let index = IndexUtil::multi_dimensional_index_to_linear_index(
          self.shape(), indexes);
        let mut minor_scan_indexes = vec![];
        minor_scan_indexes.clone_from_slice(indexes);

        let mut dest_ptr = (dest_base.len() as i64) + index * primitive_size;
        let dest_end = (dest_base.len() as i64)
          + (i64::min(index + stride_config.minor_loop_size, num_elements))
          * primitive_size;

        while dest_ptr < dest_end {
          populator(&vec![dest_ptr], &minor_scan_indexes, thread_id);
          let mut value =
            minor_scan_indexes[stride_config.minor_dimension as usize];
          value += 1;
          minor_scan_indexes[stride_config.minor_dimension as usize] = value;
          dest_ptr += primitive_size;
        }
        Ok(true)
      };

      if parallel {
        ShapeUtil::for_each_index_parallel(
          self.shape(),
          &stride_config.base,
          &stride_config.dimensions,
          &stride_config.step, &init_func);
      } else {
        let f =
          |indexes: &Vec<i64>| -> Result<bool, String> {
          let _ = init_func(indexes, -1);
          Ok(true)
        };
        ShapeUtil::for_each_index(
          self.shape(),
          &stride_config.base,
          &stride_config.dimensions,
          &stride_config.step,
          &f);
      }
    } else {
      // For scalars.
      populator(dest_base, &vec![], -1);
    }
  }

  pub fn mutable_shape_do_not_use(&mut self) -> &mut Shape {
    //&mut self.shape
    self.base.mutable_shape()
  }

  pub fn set_mutable_shape_do_not_use(&mut self, shape: &Shape) {
    self.base.set_mutable_shape_do_not_use(shape);
  }

  pub fn populate_r1(&mut self, values: &Vec<T>) {
    assert!(LayoutUtil::is_dense_array(self.shape()),
      "Only supported for dense arrays.");
    assert!(self.shape().rank() == 1);
    if self.shape().is_static() {
      assert_eq!(ShapeUtil::elements_in(self.shape()), values.len() as i64);
    } else {
      assert_eq!(self.get_dynamic_size(0, &vec![]), values.len() as i64);
    }

    let mut native_value = &T::default();
    if !values.is_empty() {
      native_value = &values[0];
    }
    assert_eq!(self.shape().element_type(), native_to_primitive_type(native_value));
    let mut data = vec![];
    if !values.is_empty() {
      data.clone_from(values);
    }
    self.set_data(&vec![], data);
  }

  pub fn populate_r2(&mut self, values: &Vec<Vec<T>>) {
    assert!(LayoutUtil::is_dense_array(self.shape()),
      "Only supported for dense arrays.");
    assert!(self.shape().rank() == 2);
    assert!(self.shape().element_type() == native_to_primitive_type(&values[0]));

    let values_dim0_size = values.len();
    let values_dim1_size = values[0].len();
    let mut literal_dim0_size = self.shape().dimensions(0);
    if self.shape().is_dynamic_dimension(0) {
      literal_dim0_size = self.get_dynamic_size(0, &vec![]);
    }
    let mut literal_dim1_size = self.shape().dimensions(1);
    if self.shape().is_dynamic_dimension(1) {
      literal_dim1_size = self.get_dynamic_size(1, &vec![]);
    }
    assert_eq!(values_dim0_size, literal_dim0_size as usize);
    assert_eq!(values_dim1_size, literal_dim1_size as usize);

    let mut dim0 = 0;
    for inner_list in values {
      let mut dim1 = 0;
      for value in inner_list {
        self.set(&vec![dim0, dim1], value.clone());
        dim1 += 1;
      }
      assert_eq!(values_dim1_size, dim1 as usize);
      dim0 += 1;
    }
  }

  // Fills this literal with the given value.
  pub fn populate_with_value(&mut self, value: T) {
    assert!(LayoutUtil::is_dense_array(self.shape()));
    assert_eq!(self.shape().element_type(), native_to_primitive_type(&value));

    let dimensions = self.shape().dimensions_vec();
    if dimensions.len() == 0 {
      let mut data = vec![];
      data.push(value);
      self.set_data(&vec![], data);
    } else if dimensions.len() == 1 {
      let data = vec![value; dimensions[0] as usize];
      self.set_data(&vec![], data);
    } else if dimensions.len() == 2 {
      //let sub = vec![value; dimensions[1] as usize];
      //let data = vec![sub; dimensions[1] as usize];
      //self.set_data(&vec![], data);
    }
  }

  pub fn populate_from_array_3d<NativeT>(&self, values: Array3D<NativeT>)
    where NativeT: Default + Clone
  {
    assert!(LayoutUtil::is_dense_array(self.shape()),
      "Only supported for dense arrays.");
    assert!(self.shape().is_array());
    assert_eq!(self.shape().element_type(),
      native_to_primitive_type(&NativeT::default()));
    assert_eq!(self.shape().rank(), values.num_dimensions());

    for dim in 0..values.num_dimensions() {
      let mut shape_size = self.shape().dimensions(dim);
      if self.shape().is_dynamic_dimension(dim as i64) {
        shape_size = self.get_dynamic_size(dim, &vec![]);
      }
      assert_eq!(values.dim(dim), shape_size as usize);
    }

    // TODO
  }

  pub fn to_string(&self) -> String {
    self.base.to_string()
  }
}

// A read-only view of a Literal. A LiteralSlice contains pointers to shape and
// literal buffers always owned by others.
/*
pub struct LiteralSlice {
  base: LiteralBase
}

impl LiteralSlice {
  pub fn new() {}
  fn root_piece() {}
}
*/

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayValueState {
  Known,
  Unknown,
  Undetermined,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Piece<T> where T: Clone + Default {
  subshape: Shape,
  children: Option<Vec<Piece<T>>>,
  data: Vec<T>,
  buffer: Vec<T>,
  dynamic_size_buffer: Vec<i64>,
  array_value_state: ArrayValueState,
}

impl<T> Piece<T> where T: Clone + Default {
  pub fn new() -> Self {
    let mut instance = Piece {
      subshape: Shape::new(),
      children: Some(Vec::new()), //None,
      data: Vec::new(),
      buffer: Vec::new(),
      dynamic_size_buffer: vec![0; 100], // Temp
      array_value_state: ArrayValueState::Undetermined
    };
    instance.data.reserve(100);
    instance
  }

  pub fn get_array_value_state(&self) -> ArrayValueState {
    self.array_value_state.clone()
  }

  pub fn set_array_value_state(&mut self, state: ArrayValueState) {
    self.array_value_state = state;
  }

  // Returns the buffer holding the array data for this piece as an array
  // slice. This piece must be array-shaped.
  pub fn data(&self) -> &Vec<T> {
    assert!(LayoutUtil::is_dense_array(self.subshape()),
      "Only supported for dense array.");
    assert!(!self.subshape().has_layout() ||
      self.subshape().layout().as_ref().unwrap().element_size_in_bits() == 0,
      "Not supported for layouts with custom bit size.");

    if !self.data.is_empty() {
      assert_eq!(self.subshape().element_type(),
        native_to_primitive_type(&self.data[0]));
    }

    &self.data
  }

  pub fn set_data(&mut self, data: Vec<T>) {
    self.data = data;
  }

  // Returns the buffer holding the array data for this piece as an array
  // slice. This piece must be array-shaped.
  pub fn untyped_data(&self) -> &Vec<i64> {
    assert!(LayoutUtil::is_dense_array(self.subshape()));
    //self.buffer()
    unimplemented!()
  }

  pub fn mutable_untyped_data(&mut self) -> &mut Vec<T> {
    assert!(LayoutUtil::is_dense_array(self.subshape()));
    self.mutable_buffer()
  }

  pub fn get(&self, multi_index: &Vec<i64>) -> &T {
    assert!(LayoutUtil::is_dense_array(self.subshape()),
      "Only supported for dense arrays.");
    let index = IndexUtil::multi_dimensional_index_to_linear_index(
      self.subshape(), multi_index);
    &self.data[index as usize]
  }

  pub fn set(&mut self, multi_index: &Vec<i64>, value: T) {
    assert!(LayoutUtil::is_dense_array(self.subshape()),
      "Only supported for dense arrays.");
    let index = IndexUtil::multi_dimensional_index_to_linear_index(
      self.subshape(), multi_index);
    self.data.insert(index as usize, value);
  }

  pub fn get_dynamic_size(&self, dim_index: usize) -> i64 {
    assert!(LayoutUtil::is_dense_array(self.subshape()));
    if !self.subshape.is_dynamic_dimension(dim_index as i64) {
      return self.subshape.dimensions(dim_index);
    }
    self.dynamic_size_buffer()[dim_index] as i64
  }

  pub fn set_dynamic_size(&mut self, dim_index: usize, size: i64) {
    assert!(LayoutUtil::is_dense_array(self.subshape()));
    assert!(self.subshape.is_dynamic_dimension(dim_index as i64));
    self.mutable_dynamic_size_buffer()[dim_index] = size;
  }

  pub fn allocate_buffers(&self) {
    let bytes = self.total_bytes_dense();
    if bytes > MAX_INLINED_BYTES {
      assert!(self.buffer().is_empty());

    } else {
        
    }
  }

  pub fn deallocate_buffers(&mut self) {
    self.buffer.clear();
  }

  // Gets the buffer holding the array data.
  pub fn buffer(&self) -> &Vec<T> {
    &self.buffer
  }

  pub fn mutable_buffer(&mut self) -> &mut Vec<T> {
    &mut self.buffer
  }

  pub fn set_buffer(&self) {
    unimplemented!()
  }

  pub fn move_data_from() {}

  pub fn copy_from(
    &mut self, src: &Piece<T>, only_dynamic_bound: bool) -> Result<(), String>
  {
    assert!(LayoutUtil::is_dense_array(self.subshape()),
      "Only supported for dense arrays.");
    assert!(LayoutUtil::is_dense_array(src.subshape()),
      "Only supported for dense arrays.");

    if !only_dynamic_bound {
      assert!(ShapeUtil::compatible(self.subshape(), src.subshape()));
    }

    if src.array_value_state == ArrayValueState::Unknown ||
       src.array_value_state == ArrayValueState::Undetermined
    {
      if self.array_value_state == ArrayValueState::Known {
        // TODO
      }
      self.array_value_state = src.array_value_state.clone();
      return Ok(());
    } else {
      assert!(src.array_value_state == ArrayValueState::Known);
      if self.array_value_state == ArrayValueState::Undetermined ||
         self.array_value_state == ArrayValueState::Unknown
      {
        // TODO
      }
      self.array_value_state = src.array_value_state.clone();
    }

    if ShapeEqual::new().equal(self.subshape(), src.subshape()) {
      // If the layouts are equal it's faster just to memcpy.
    } else {
      
    }
    assert_eq!(self.dynamic_size_buffer_bytes(), src.dynamic_size_buffer_bytes());
    if self.subshape().is_dynamic() && src.subshape().is_dynamic() {
      // TODO
    }

    Ok(())
  }

  pub fn dynamic_size_buffer(&self) -> &Vec<i64> {
    &self.dynamic_size_buffer
  }

  pub fn mutable_dynamic_size_buffer(&mut self) -> &mut Vec<i64> {
    &mut self.dynamic_size_buffer
  }

  pub fn dynamic_size_buffer_bytes(&self) -> usize {
    assert!(LayoutUtil::is_dense_array(self.subshape()));
    self.subshape.dimensions_size() * size_of::<i32>()
  }

  // Gets the subshape of this piece.
  pub fn subshape(&self) -> &Shape {
    &self.subshape
  }

  // Sets the subshape of this piexe.
  pub fn set_subshape(&mut self, subshape: Shape) {
    self.subshape = subshape;
  }

  // Returns the size in bytes of the buffer holding the dense array data.
  pub fn size_bytes_dense(&self) -> i64 {
    assert!(LayoutUtil::is_dense_array(&self.subshape));
    ShapeUtil::byte_size_of(&self.subshape, -1)
  }

  pub fn dynamic_size_byffer_offset(&self) -> usize { 0 }

  // Total size in bytes, including the dynamic size addition.
  pub fn total_bytes_dense(&self) -> usize {
    self.dynamic_size_byffer_offset() + self.dynamic_size_buffer_bytes()
  }

  // Returns the number of elements in this piece's array.
  pub fn element_count(&self) -> i64 {
    ShapeUtil::elements_in(&self.subshape)
  }

  pub fn child(&self, index: usize) -> Option<&Piece<T>> {
    assert!(self.children.is_some());
    self.children.as_ref().unwrap().get(index)
  }

  pub fn mutable_child(&mut self, index: usize) -> Option<&mut Piece<T>> {
    assert!(self.children.is_some());
    self.children.as_mut().unwrap().get_mut(index)
  }

  // Adds a child piece to this piece's children.
  pub fn emplace_back(&mut self, child: Piece<T>) {
    assert!(self.children.is_some());
    self.children.as_mut().unwrap().push(child);
  }

  // Returns the size of children pieces of this piece.
  pub fn children_size(&self) -> usize {
    assert!(self.children.is_some());
    self.children.as_ref().unwrap().len()
  }

  // Visitor functions that resursively traverses the piece and calls the
  // given function at each child piece.
  pub fn for_each_subpiece<F>(&self, func: &mut F)
    where F: FnMut(usize, &Piece<T>) -> Result<(), String>
  {
    let _ = Piece::for_each_helper(func, self, 0);
  }

  pub fn for_each_mutable_subpiece<F>(&mut self, func: &mut F)
    where F: FnMut(&Vec<i64>, &mut Piece<T>) -> Result<(), String>
  {
    let _ = Piece::for_each_mutable_helper(func, self, &vec![]);
  }

  // Same as above, but the function has the type:
  //    absl::Status (const ShapeIndex& index, Piece& piece)
  // The first non-OK return value is returned by the function.
  pub fn for_each_subpiece_with_status<F>(&mut self, func: &mut F) -> Result<(), String>
    where F: FnMut(&Vec<i64>, &mut Piece<T>) -> Result<(), String>
  {
    Piece::for_each_mutable_helper(func, self, &vec![])
  }

  pub fn for_each_subpiece_with_bool() {}

  pub fn all_elements_equal_value(data: &Vec<T>, value: &T) -> bool
    where T: PartialEq
  {
    for v in data {
      if v != value { return false; }
    }
    true
  }

  // Checks whether all elements of this Piece are equal to the given literal.
  // Returns false if this Piece is not an array.
  pub fn is_all(&self, scalar: &Literal<T>) -> bool
    where T: PartialEq
  {
    assert!(ShapeUtil::is_scalar(scalar.shape()));
    if !self.subshape.is_array() { return false; }

    assert!(LayoutUtil::is_dense_array(self.subshape()));
    assert_eq!(self.subshape.element_type(), scalar.shape().element_type());

    let mut f = |_t: PrimitiveType| -> bool {
      Piece::all_elements_equal_value(self.data(), scalar.get_first_element())
    };
    primitive_util::array_type_switch(&mut f, &self.subshape.element_type())
  }

  // Returns the number of elements with equal value to the given literal.
  // Returns 0 if this Piece is not an array.
  pub fn count_all(&self, scalar: &Literal<T>) -> usize where T: PartialEq {
    assert!(ShapeUtil::is_scalar(scalar.shape()));
    if !self.subshape().is_array() {
      return 0;
    }
    assert!(LayoutUtil::is_dense_array(self.subshape()));
    assert_eq!(self.subshape().element_type(), scalar.shape().element_type());

    let mut f = |_t: PrimitiveType| -> usize {
      let mut count = 0;
      let value = scalar.get_first_element();
      for elt in self.data() {
        if elt == value {
          count += 1;
        }
        let elt_any: &dyn Any = elt;
        let value_any: &dyn Any = value;
        let elt_nan = elt_any.downcast_ref::<f64>();
        let value_nan = value_any.downcast_ref::<f64>();
        if elt_nan.is_some() && value_nan.is_some() {
          if elt_nan.unwrap().is_nan() && value_nan.unwrap().is_nan() {
            count += 1;
          }
        }
      }
      count
    };
    array_type_switch(&mut f, &self.subshape.element_type())
  }

  pub fn equal_elements() {}
  pub fn equal_dynamic_size() {}

  // See comments on ArrayValueState for detailed explanation.
  pub fn is_determined(&self) -> bool {
    if self.array_value_state == ArrayValueState::Undetermined {
      return false;
    }
    if self.subshape.is_tuple() {
      let mut are_all_leaf_arrays_determined = true;
      let mut func =
        |_index: usize, piece: &Piece<T>| -> Result<(), String>
      {
        if !piece.subshape.is_array() {
          return Ok(());
        }
        are_all_leaf_arrays_determined &= piece.is_known();
        Ok(())
      };
      Piece::for_each_subpiece(self, &mut func);
      return are_all_leaf_arrays_determined;
    }
    true
  }

  pub fn is_known(&self) -> bool {
    if self.array_value_state != ArrayValueState::Known {
      return false;
    }
    if self.subshape.is_tuple() {
      let mut are_all_leaf_arrays_known = true;
      let mut func =
        |_index: usize, piece: &Piece<T>| -> Result<(), String>
      {
        if !piece.subshape.is_array() {
          return Ok(());
        }
        are_all_leaf_arrays_known &= piece.is_known();
        Ok(())
      };
      Piece::for_each_subpiece(self, &mut func);
      return are_all_leaf_arrays_known;
    }
    true
  }

  fn get_dense_rep() {}

  fn get_tuple_rep(&self) -> &Option<Vec<Piece<T>>> {
    &self.children
  }

  fn get_tuple_rep_mut(&mut self) -> &mut Option<Vec<Piece<T>>> {
    &mut self.children
  }

  fn for_each_helper<F>(func: &mut F, piece: &Piece<T>, index: usize) -> Result<(), String>
    where F: FnMut(usize, &Piece<T>) -> Result<(), String>
  {
    let mut result = func(index, piece);
    if result.is_err() { return result; }
    let tuple_rep = piece.get_tuple_rep();
    if tuple_rep.is_some() {
      let children = tuple_rep.as_ref().unwrap();
      let size = children.len();
      for i in 0..size {
        result = Piece::for_each_helper(func, children.get(i).unwrap(), index);
        if result.is_err() { return result; }
      }
    }
    Ok(())
  }

  fn for_each_mutable_helper<F>(
    func: &mut F,
    piece: &mut Piece<T>,
    index: &Vec<i64>) -> Result<(), String>
      where F: FnMut(&Vec<i64>, &mut Piece<T>) -> Result<(), String>
  {
    let mut result = func(index, piece);
    if result.is_err() { return result; }
    let tuple_rep = piece.get_tuple_rep_mut();
    if tuple_rep.is_some() {
      let children = tuple_rep.as_mut().unwrap();
      let size = children.len();
      for i in 0..size {
        result = Piece::for_each_mutable_helper(
          func, children.get_mut(i).unwrap(), index);
        if result.is_err() { return result; }
      }
    }
    Ok(())
  }

  fn for_each_helper_bool() {}
}

struct StrideConfig {
  dimensions: Vec<i64>,
  base: Vec<i64>,
  step: Vec<i64>,
  minor_dimension: i64,
  dest_stride: i64,
  source_stride: i64,
  minor_loop_size: i64
}

impl StrideConfig {
  pub fn new(
    source_shape: &Shape,
    dest_shape: &Shape,
    dimensions: &Vec<i64>) -> Self
  {
    let mut minor_dimension = 0;
    let mut source_stride = 0;
    let mut dest_stride = 0;

    if !dimensions.is_empty() {
      if dimensions[LayoutUtil::minor(
          source_shape.layout().as_ref().unwrap(),
          0) as usize] >=
         dimensions[LayoutUtil::minor(
          dest_shape.layout().as_ref().unwrap(),
          0) as usize]
      {
        minor_dimension =
          LayoutUtil::minor(
            dest_shape.layout().as_ref().unwrap(), 0);
        dest_stride =
          IndexUtil::get_dimension_stride(dest_shape, minor_dimension);
      } else {
        minor_dimension =
          LayoutUtil::minor(
            dest_shape.layout().as_ref().unwrap(), 0);
        source_stride =
          IndexUtil::get_dimension_stride(source_shape, minor_dimension);
      }
    }
    
    let mut instance = StrideConfig {
      dimensions: vec![],
      base: vec![],
      step: vec![],
      minor_dimension: minor_dimension,
      dest_stride: dest_stride,
      source_stride: source_stride,
      minor_loop_size: dimensions[minor_dimension as usize]
    };
    instance.dimensions.clone_from_slice(&dimensions);
    instance.step.resize(minor_dimension as usize, 0);
    instance.step[minor_dimension as usize] = instance.minor_loop_size;
    instance
  }
}

#[cfg(test)]
mod tests {
  use core::f64;
use std::i8;

use num::complex::Complex64;
use crate::{literal::Literal, literal_util::LiteralUtil, shape_util::ShapeUtil};
  use crate::blitz_data::PrimitiveType;
  //use super::*;

  #[test]
  fn test_literal_scalar_to_string() {
    let true_lit = LiteralUtil::create_r0::<bool>(true);
    assert_eq!(true_lit.to_string(), "pred[] true".to_string());
  }

  #[test]
  fn test_scalar_equality() {
    let f64_42 = LiteralUtil::create_r0::<f64>(42.0);
    let f64_42_clone = LiteralUtil::create_r0::<f64>(42.0);
    assert_eq!(f64_42, f64_42);
    assert_eq!(f64_42, f64_42_clone);

    let f64_123 = LiteralUtil::create_r0::<f64>(123.0);
    assert_ne!(f64_42, f64_123);
  }

  #[test]
  fn test_non_scalar_equality() {
    let matrix = LiteralUtil::create_r2::<f64>(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix_clone = LiteralUtil::create_r2::<f64>(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix_different = LiteralUtil::create_r2::<f64>(
      &vec![vec![4.0, 3.0], vec![1.0, 2.0]]);

    assert_eq!(matrix, matrix);
    assert_eq!(matrix, matrix_clone);
    assert_ne!(matrix, matrix_different);

    let vector_literal = LiteralUtil::create_r1::<f64>(
      &vec![1.0, 2.0, 3.0, 4.0]);
    let scalar = LiteralUtil::create_r0::<f64>(1.0);
    let nil = Literal::new_from_shape(&ShapeUtil::make_nil());

    assert_ne!(matrix, vector_literal);
    assert_ne!(matrix, scalar);
    assert_ne!(matrix, nil);
    assert_eq!(nil, nil);
  }

  #[test]
  fn test_token_equality() {
    let token0 = LiteralUtil::create_token::<f64>();
    let token1 = LiteralUtil::create_token::<f64>();
    let token_f64 = LiteralUtil::create_token::<f64>();
    let scalar = LiteralUtil::create_r0::<f64>(1.0);

    assert_eq!(token0, token1);
    assert_ne!(token_f64, scalar);

    assert_eq!(LiteralUtil::make_tuple(&vec![&token0]),
      LiteralUtil::make_tuple(&vec![&token0]));
    assert_eq!(LiteralUtil::make_tuple(&vec![&token0, &scalar]),
      LiteralUtil::make_tuple(&vec![&token1, &scalar]));
    assert_ne!(LiteralUtil::make_tuple(&vec![&token0, &scalar]),
      LiteralUtil::make_tuple(&vec![&scalar, &token1]));
  }

  #[test] // fail
  fn test_different_layout_equality() {
    // Test equality with literals which have different layouts.
    let mut col_major: Literal<f64> = Literal::new_from_shape(
      &ShapeUtil::make_shape_with_dense_layout(
        &PrimitiveType::F64,
        &vec![2, 2],
        &vec![0, 1],
        vec![],
        1,
        0,
        0));
    col_major.set(&vec![0, 0], 1.0);
    col_major.set(&vec![0, 1], 2.0);
    col_major.set(&vec![1, 0], 3.0);
    col_major.set(&vec![1, 1], 4.0);

    let mut row_major: Literal<f64> = Literal::new_from_shape(
      &ShapeUtil::make_shape_with_dense_layout(
        &PrimitiveType::F64,
        &vec![2, 2],
        &vec![1, 0],
        vec![],
        1,
        0,
        0));
    row_major.set(&vec![0, 0], 1.0);
    row_major.set(&vec![0, 1], 2.0);
    row_major.set(&vec![1, 0], 3.0);
    row_major.set(&vec![1, 1], 4.0);

    assert_eq!(row_major, col_major);
  }

  #[test] // FAIL
  fn test_tuple_equality() {
    // Test equality with tuples.
    let scalar = LiteralUtil::create_r0::<f64>(1.0);
    let matrix = LiteralUtil::create_r2::<f64>(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let tuple1 = LiteralUtil::make_tuple(
      &vec![&scalar, &matrix]);

    // Tuple with the same elements. One element is shared with the original
    // tuple, the other is a clone of the element in the original tuple.
    let scalar_clone = LiteralUtil::create_r0::<f64>(1.0);
    let tuple2 = LiteralUtil::make_tuple(
      &vec![&scalar_clone, &matrix]);
    assert_eq!(tuple1, tuple2);

    // Tuple with elements reversed.
    let reserved_tuple = LiteralUtil::make_tuple(
      &vec![&matrix, &scalar]);
    assert_ne!(tuple1, reserved_tuple);

    // Tuple with different value.
    let scalar_42 = LiteralUtil::create_r0::<f64>(42.0);
    let different_tuple = LiteralUtil::make_tuple(
      &vec![&scalar_42, &matrix]);
    assert_ne!(tuple1, different_tuple);
  }

  #[test]
  fn test_dynamic_shape_equality() {
    let mut r1 = LiteralUtil::create_r1::<f64>(
      &vec![1.0, 2.0]);
    r1.set_dynamic_size(0, &vec![], 1);
    let mut r2 = LiteralUtil::create_r2::<f64>(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    r2.set_dynamic_size(0, &vec![], 1);
    let tuple1 = LiteralUtil::make_tuple(&vec![&r1, &r2]);

    // TODO
    // Tuple with the same elements. One element is shared with the original
    // tuple, the other is a clone of the element in the original tuple.
    let mut r1_clone = LiteralUtil::create_r1(&vec![1.0, 3.0]);
    r1_clone.set_dynamic_size(0, &vec![], 1);
    let tuple2 = LiteralUtil::make_tuple(
      &vec![&r1_clone, &r2]);
    assert_eq!(tuple1, tuple2);
  }

  #[test]
  fn test_c64_equality() {
    let c1 = Complex64::new(1.0, 2.0);
    let c2 = Complex64::new(3.0, 4.0);
    let vec = LiteralUtil::create_r1(&vec![c1, c2]);

    let vec_clone =
      LiteralUtil::create_r1(&vec![c1, c2]);
    assert_eq!(vec, vec_clone);

    let vec_reversed =
      LiteralUtil::create_r1(&vec![c2, c1]);
    assert_ne!(vec, vec_reversed);
  }

  #[test]
  fn test_is_all_tuple() {
    let elt1 = LiteralUtil::create_r0(0.0);
    let elt2 =
      LiteralUtil::create_r2(&vec![vec![0.0, 0.0], vec![0.0, 0.0]]);
    let tuple = LiteralUtil::make_tuple(&vec![&elt1, &elt2]);

    // Tuples should always return false for IsAll.
    assert_eq!(tuple.is_all_int(0), false);
    assert_eq!(tuple.is_all_int(1), false);
  }

  #[test]
  fn test_create_from_shape_tuple() {
    let scalar = LiteralUtil::create_r0(0.0);
    let matrix =
      LiteralUtil::create_r2(&vec![vec![0.0, 0.0], vec![0.0, 0.0]]);
    let tuple = LiteralUtil::make_tuple(&vec![&scalar, &matrix]);

    let x: Literal<f64> = Literal::create_from_shape(tuple.shape());
    assert_eq!(tuple, x);
  }

  #[test]
  fn test_is_all_int() {
    let i64_min = i64::MIN;
    let literal: Literal<i64> = LiteralUtil::create_r0(i64::MIN);
    assert_eq!(literal.is_all_int(i64_min), true);

    let l_42: Literal<i64> = LiteralUtil::create_r0(42);
    assert_eq!(l_42.is_all_int(42), true);
    //let l_421 = LiteralUtil::create_r0(42.0001);

    let l_100: Literal<i64> = LiteralUtil::create_r1(&vec![100, 100, 100]);
    assert_eq!(l_100.is_all_int(100), true);

    let l_8: Literal<i64> = LiteralUtil::create_r2(&vec![vec![8, 8], vec![8, 8]]);
    assert_eq!(l_8.is_all_int(8), true);
    let l_8_9: Literal<i64> = LiteralUtil::create_r2(&vec![vec![8, 8], vec![8, 9]]);
    assert_eq!(l_8_9.is_all_int(8), false);
    let l_9_8: Literal<i64> = LiteralUtil::create_r2(&vec![vec![9, 8], vec![8, 8]]);
    assert_eq!(l_9_8.is_all_int(8), false);
  }

  #[test]
  fn test_is_all_float() {
    // IsAllFloat always returns false when the literal is not floating-point.
    assert_eq!(LiteralUtil::create_r0::<bool>(false).is_all_float(0.0), false);
    assert_eq!(LiteralUtil::create_r0::<i8>(0).is_all_float(0.0), false);
    assert_eq!(LiteralUtil::create_r0::<u8>(0).is_all_float(0.0), false);
    assert_eq!(LiteralUtil::create_r0::<i32>(0).is_all_float(0.0), false);

    assert_eq!(LiteralUtil::create_r0::<f64>(0.0).is_all_float(0.0), true);
    assert_eq!(LiteralUtil::create_r0::<f64>(0.5).is_all_float(0.5), true);
    assert_eq!(LiteralUtil::create_r0::<f64>(-0.5).is_all_float(-0.5), true);
    assert_eq!(LiteralUtil::create_r0::<f64>(-0.5).is_all_float(-0.49), false);
    assert_eq!(LiteralUtil::create_r2::<f64>(
      &vec![vec![0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0]]).is_all_float(0.0), false);
    assert_eq!(LiteralUtil::create_r2::<f64>(
      &vec![vec![0.5, 0.5, 0.5], vec![0.5, 0.5, 0.5]]).is_all_float(0.5), true);
  }

  #[test]
  fn test_is_all_complex() {
    let c0_0 = Complex64::new(0.0, 0.0);
    assert_eq!(LiteralUtil::create_r0::<bool>(false).is_all_complex(c0_0), false);
    assert_eq!(LiteralUtil::create_r0::<i8>(0).is_all_complex(c0_0), false);
    assert_eq!(LiteralUtil::create_r0::<u8>(0).is_all_complex(c0_0), false);
    assert_eq!(LiteralUtil::create_r0::<i32>(0).is_all_complex(c0_0), false);
    assert_eq!(LiteralUtil::create_r0::<f64>(0.0).is_all_complex(c0_0), false);

    let c8_9 = Complex64::new(8.0, 9.0);
    let c7_9 = Complex64::new(7.0, 9.0);
    assert_eq!(LiteralUtil::create_r2::<Complex64>(&vec![vec![c8_9], vec![c8_9]])
      .is_all_complex(Complex64::new(8.0, 9.0)), true);
    assert_eq!(LiteralUtil::create_r2::<Complex64>(&vec![vec![c7_9], vec![c8_9]])
      .is_all_complex(Complex64::new(8.0, 9.0)), false);
    assert_eq!(LiteralUtil::create_r2::<Complex64>(&vec![vec![c8_9], vec![c7_9]])
      .is_all_complex(Complex64::new(8.0, 9.0)), false);
  }

  #[test]
  fn test_is_all_first() {
    assert_eq!(LiteralUtil::create_r1::<bool>(&vec![false, true]).is_all_first(), false);
  }

  #[test]
  fn test_count_equal_int() {
    let l1: Literal<i64> = LiteralUtil::create_r1(&vec![]);
    assert_eq!(l1.count_equal(&1), 0);

    let l2: Literal<i64> = LiteralUtil::create_r1(
      &vec![1, 2, 3, 4, 5, 100]);
    assert_eq!(l2.count_equal(&2), 1);

    let l3: Literal<i64> = LiteralUtil::create_r1(
      &vec![0, 3, 6, 0, 9, 18, 0]);
    assert_eq!(l3.count_equal(&0), 3);

    let l4: Literal<i64> = LiteralUtil::create_r1(
      &vec![234, 345, 4, 45, 5467, 5467, 5467]);
    assert_eq!(l4.count_equal(&5467), 3);
  }

  #[test]
  fn test_count_equal_float() {
    let l1: Literal<f64> = LiteralUtil::create_r1(&vec![]);
    assert_eq!(l1.count_equal(&0.0), 0);

    let l2: Literal<f64> = LiteralUtil::create_r1(
      &vec![1.1, 2.2, 3.3, 4.4, 5.5, 100.6]);
    assert_eq!(l2.count_equal(&3.3), 1);

    let l3: Literal<f64> = LiteralUtil::create_r1(
      &vec![7.62, 3.0, 7.75, 7.62, 7.3, 2.0, 7.62]);
    assert_eq!(l3.count_equal(&7.62), 3);

    let l4: Literal<f64> = LiteralUtil::create_r1(
      &vec![f64::NAN, 0.0, 6.8, f64::NAN, f64::NAN, f64::NAN, 63.12, 24.6, f64::NAN]);
    assert_eq!(l4.count_equal(&f64::NAN), 5);
  }

  #[test]
  fn test_count_equal_bool() {
    let l1 = LiteralUtil::create_r1(&vec![false, true]);
    assert_eq!(l1.count_equal(&false), 1);
  }

  #[test]
  fn test_count_equal_complex() {
    let c1 = Complex64::new(1.0, 2.0);
    let c2 = Complex64::new(3.0, 4.0);
    let c3 = Complex64::new(5.0, 6.0);
    let c4 = Complex64::new(6.0, 7.0);

    let l1 = LiteralUtil::create_r1(&vec![c1, c2, c3, c4]);
    assert_eq!(l1.count_equal(&c3), 1);
  }

  #[test]
  fn test_is_zero() {
    let scalar_zero = LiteralUtil::create_r0(0.0);
    let scalar_one = LiteralUtil::create_r0(1.0);
    assert_eq!(scalar_zero.is_zero_f64(&vec![]), true);
    assert_eq!(scalar_one.is_zero_f64(&vec![]), false);

    let array: Literal<i64> = LiteralUtil::create_r2(
      &vec![vec![1, 2, 0, 3], vec![1, 0, 1, 2]]);
    assert_eq!(array.is_zero_i64(&vec![0, 1]), false);
    assert_eq!(array.is_zero_i64(&vec![0, 2]), true);
    assert_eq!(array.is_zero_i64(&vec![1, 1]), true);
    assert_eq!(array.is_zero_i64(&vec![1, 2]), false);

    let c0 = Complex64::new(0.0, 0.0);
    let l_c_0 = LiteralUtil::create_r0(c0);
    let c_non_0 = Complex64::new(0.5, 0.0);
    let l_c_non_0 = LiteralUtil::create_r0(c_non_0);
    assert_eq!(l_c_0.is_zero_c64(&vec![]), true);
    assert_eq!(l_c_non_0.is_zero_c64(&vec![]), false);
  }

  #[test]
  fn test_reshape_r0() {
    //let original = LiteralUtil::create_r0(1.7);
    //let reshape = original.
  }

  #[test]
  fn test_slice_r0_i64() {
    /*
    let input: Literal<i64> = LiteralUtil::create_r0(1);
    let result: Literal<i64> =
      input.slice(&vec![], &vec![]);
    assert_eq!(input, result);
    */
  }

  #[test]
  fn test_populate_r1_i64() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::S64, vec![1]);
    let mut output: Literal<i64> = Literal::new_from_shape(&shape);
    output.populate_r1(&vec![77]);

    let expected: Literal<i64> = LiteralUtil::create_r1(&vec![77]);
    assert_eq!(output, expected);
  }

  #[test]
  fn test_populate_r1_c64() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::C64, vec![1]);
    let mut output: Literal<Complex64> = Literal::new_from_shape(&shape);
    let c = Complex64::new(77.0, 88.0);
    output.populate_r1(&vec![c]);

    let expected: Literal<Complex64> = LiteralUtil::create_r1(&vec![c]);
    assert_eq!(output, expected);
  }

  #[test]
  fn test_populate_r2_c64() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::C64, vec![2, 2]);
    let mut output: Literal<Complex64> = Literal::new_from_shape(&shape);
    let c1 = Complex64::new(7.0, 8.0);
    let c2 = Complex64::new(9.0, 10.0);
    let c3 = Complex64::new(1.0, 2.0);
    let c4 = Complex64::new(3.0, 4.0);
    output.populate_r2(&vec![vec![c1, c2], vec![c3, c4]]);

    let expected: Literal<Complex64> =
      LiteralUtil::create_r2(&vec![vec![c1, c2], vec![c3, c4]]);
    assert_eq!(output, expected);
  }

  #[test]
  fn test_populate_with_value_r0_float() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::F64, vec![]);
    let mut output = Literal::new_from_shape(&shape);
    output.populate_with_value(0.25);

    let expected = LiteralUtil::create_r0(0.25);
    assert_eq!(output, expected);
  }

  #[test]
  fn test_populate_with_value_r1_float() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::F64, vec![3]);
    let mut output = Literal::new_from_shape(&shape);
    output.populate_with_value(0.5);

    let expecteed =
      LiteralUtil::create_r1(&vec![0.5, 0.5, 0.5]);
    assert_eq!(output, expecteed);
  }

  // Fail
  #[test]
  fn test_populate_with_value_r2_float() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::F64, vec![2, 2]);
    let mut output = Literal::new_from_shape(&shape);
    output.populate_with_value(2.0);

    let expected = LiteralUtil::create_r2(
      &vec![vec![2.0, 2.0], vec![2.0, 2.0]]);
    assert_eq!(output, expected);
  }

  #[test]
  fn test_populate_with_value_r1_s64() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::S64, vec![3]);
    let mut output: Literal<i64> = Literal::new_from_shape(&shape);
    output.populate_with_value(-7);

    let expecteed =
      LiteralUtil::create_r1(&vec![-7, -7, -7]);
    assert_eq!(output, expecteed);
  }

  #[test]
  fn test_replicate_r2_u32() {
    
  }

  #[test]
  fn test_get_set_tuple() {
    let r0 = LiteralUtil::create_r0(42.0);
    let r2 = LiteralUtil::create_r2(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let _tuple = LiteralUtil::make_tuple(&vec![&r0, &r2]);

    //assert_eq!(tuple.get(&vec![], &vec![0]), &42.0);
  }

  #[test]
  fn test_get_as_double() {
    let m = LiteralUtil::create_r2(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    assert_eq!(m.get_as_double(&vec![0, 0]), Some(&1.0));
    assert_eq!(m.get_as_double(&vec![1, 0]), Some(&3.0));
  }

  #[test]
  fn test_get_sum_as_double() {
    let m = LiteralUtil::create_r2(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    assert_eq!(m.get_sum_as_double(&vec![0, 3]), Some(1.0 + 4.0));
    assert_eq!(m.get_sum_as_double(&vec![0, 1, 2, 3]), Some(1.0 + 2.0 + 3.0 + 4.0));

    let vals = vec![1.0; 1024];
    let v = LiteralUtil::create_r1(&vals);
    let mut indices = vec![];
    let mut i = 0;
    while i < 1024 {
      indices.push(i);
      assert_eq!(v.get_sum_as_double(&indices), Some((i as f64 + 2.0) / 2.0));
      i += 2;
    };
  }

  #[test]
  fn test_get_as_complex_64() {
    let value = Complex64::new(1.0, 0.0);
    let c1 = LiteralUtil::create_r0(value);
    assert_eq!(c1.get_as_complex_64(&vec![]), Some(&value));

    let c2 = LiteralUtil::create_r0(1.0);
    assert_eq!(c2.get_as_complex_64(&vec![]), None/*Some(&value)*/);

    let other_value = Complex64::new(1.0, 2.0);
    let c5 = LiteralUtil::create_r0(other_value);
    assert_eq!(c5.get_as_complex_64(&vec![]), Some(&other_value));

    let c6: Literal<i64> = LiteralUtil::create_r0(1);
    assert_eq!(c6.get_as_complex_64(&vec![]).is_some(), false);
  }

  #[test]
  fn test_is_equal_at() {
    let val_double = 4.0;
    let val_integral: i64 = 4;
    let c1 = LiteralUtil::create_r0(val_integral.clone());
    assert_eq!(c1.is_equal_at(&vec![], &val_double), true);
    assert_eq!(c1.is_equal_at(&vec![], &val_integral), true);

    let c2 = LiteralUtil::create_r0(val_double.clone());
    assert_eq!(c2.is_equal_at(&vec![], &val_double), true);
    assert_eq!(c2.is_equal_at(&vec![], &val_integral), true);

    let val_complex = Complex64::new(val_double, 0.0);
    assert_eq!(c1.is_equal_at(&vec![], &val_complex), true);
    assert_eq!(c2.is_equal_at(&vec![], &val_complex), true);

    let c4 = LiteralUtil::create_r0(val_complex.clone());
    assert_eq!(c4.is_equal_at(&vec![], &val_double), true);
    assert_eq!(c4.is_equal_at(&vec![], &val_integral), true);
    assert_eq!(c4.is_equal_at(&vec![], &val_complex), true);
  }

  #[test]
  fn test_create_from_shape_with_unknown_leaf_arrays() {
    let c1: Literal<f64> = Literal::create_from_shape_with_unknown_leaf_arrays(
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![4, 4]));
    assert_eq!(c1.is_known(&vec![]), false);
  }

  #[test]
  fn test_create_from_shape_with_unknown_leaf_arrays_s4_tuple() {
    let mut inner_shape = ShapeUtil::make_shape(
      &PrimitiveType::S4, vec![4, 4]);
    inner_shape.mutable_layout().as_mut().unwrap().set_element_size_in_bits(4);

    let c1: Literal<i64> =
      Literal::create_from_shape_with_unknown_leaf_arrays(&inner_shape);
    assert_eq!(c1.is_known(&vec![]), false);
  }

  #[test]
  fn test_create_partially_known_tuple() {
    let c1: Literal<i32> = Literal::create_from_shape_with_unknown_leaf_arrays(
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![4, 4]));
    let c2 = LiteralUtil::create_r0(10);
    let c3 = LiteralUtil::make_tuple(&vec![&c1, &c2]);
    let c4 = LiteralUtil::create_r0(100);
    let c5 = LiteralUtil::make_tuple(&vec![&c4, &c3]);
    assert_eq!(c5.is_known(&vec![]), false);
  }

  fn test_copy_from_partially_known_tuple() {}

  #[test]
  fn test_populate_r1_dynamic() {
    let mut literal: Literal<u32> = Literal::new_from_shape(
      &ShapeUtil::make_shape(&PrimitiveType::U32, vec![20]));
    literal.set_dynamic_size(0, &vec![], 10);
    literal.populate_r1(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let expected = "u32[<=20](10) {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}".to_string();
    assert_eq!(literal.to_string(), expected);
  }

  #[test]
  fn test_populate_r2_dynamic_dim0() {
    let mut literal: Literal<u32> = Literal::new_from_shape(
      &ShapeUtil::make_shape(&PrimitiveType::U32, vec![5, 2]));
    literal.set_dynamic_size(0, &vec![], 3);
    literal.populate_r2(&vec![vec![1, 2], vec![3, 4], vec![5, 6]]);

    let expected = "u32[<=5,2](3,2) {
 { 1, 2 },
 { 3, 4 },
 { 5, 6 }
}".to_string();
    assert_eq!(literal.to_string(), expected);
  }

  #[test]
  fn test_populate_r2_dynamic_dim1() {
    let mut literal: Literal<u32> = Literal::new_from_shape(
      &ShapeUtil::make_shape(&PrimitiveType::U32, vec![2, 5]));
    literal.set_dynamic_size(1, &vec![], 3);
    literal.populate_r2(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
/* 
    let expected = "u32[2,<=5](2,3) {
 { 1, 2, 3 },
 { 4, 5, 6 }
}".to_string();
    assert_eq!(literal.to_string(), expected);
    */
  }
}