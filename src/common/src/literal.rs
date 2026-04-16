#![allow(dead_code)]

use core::f64;
use std::{any::Any, fmt::Debug, mem::size_of, vec};
use num::{complex::Complex64, Complex};

use crate::{
  array3d::Array3D, blitz_data::PrimitiveType, index_util::IndexUtil, layout::Layout,
  layout_util::LayoutUtil, permutation_util::{inverse_permutation, is_permutation},
  primitive_util::{
    self, array_type_switch, complex_type_switch, floating_point_type_switch,
    integral_type_switch, is_array_type, is_complex_type, is_floating_point_type,
    is_integral_type, is_predicate_type, native_to_primitive_type, primitive_type_switch
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

// Create a nullary tuple.
fn nil_shape() -> Shape {
  //Shape::new()
  ShapeUtil::make_nil()
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

//#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
  root_piece: Piece,
}

impl Literal {
  pub fn default() -> Self {
    //Literal { root_piece: Piece::new() }
    Literal::new(
      &nil_shape(),
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
    let mut literal = Literal { root_piece: Piece::new() };
    assert!(leaf_array_value_state != ArrayValueState::Known ||
      LayoutUtil::has_layout(literal.shape()));

    let root_piece = literal.mutable_root_piece();
    root_piece.set_subshape(shape.clone());
    Literal::set_piece(shape, root_piece,
      allocate_arrays, leaf_array_value_state);

    literal
  }

  fn set_piece(
    shape: &Shape,
    piece: &mut Piece,
    allocate_arrays: bool,
    leaf_array_value_state: ArrayValueState)
  {
    if shape.is_tuple() {
      for subshape in shape.tuple_shapes_vec() {
        let mut child_piece: Piece = Piece::new();
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

  // Create a literal of the given shape. The literal is allocated sufficient
  // memory to hold the shape. Memory is uninitialized.
  pub fn new_from_shape(shape: &Shape) -> Self {
    Literal::new(
      shape,
      true,
      ArrayValueState::Known)
  }

  // Copy values from 'src_literal' rooted at 'src_shape_index' into this
  // literal rooted at 'dest_shape_index'. The subshape of this literal rooted
  // at 'dest_shape_index' must be compatible with the subshape of 'src_literal'
  // rooted at 'src_shape_index', but need not be arrays. If only_dynamic_bound
  // is true, only elements within dynamic bounds will be copied.
  pub fn copy_from(
    &mut self,
    src_literal: &Literal,
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
      |index: &Vec<i64>, piece: &mut Piece| -> Result<(), String>
    {
      if !piece.subshape().is_array() {
        return Ok(());
      }
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
      let result = piece.copy_from(
        src_literal.piece(&src_piece_index), only_dynamic_bound);
      if result.is_err() {
        return result;
      }
      Ok(())
    };
    self.mutable_root_piece().for_each_subpiece_with_status(&mut f)
  }

  pub fn copy_slice_from(
    &self,
    src_literal: &Literal,
    src_base: &Vec<i64>,
    dest_base: &Vec<i64>,
    _copy_size: &Vec<i64>) -> Result<(), String>
  {
    debug_assert!(self.shape().is_array());
    debug_assert!(src_literal.shape().is_array());
    debug_assert!(ShapeUtil::same_element_type(src_literal.shape(), self.shape()));
    debug_assert!(src_literal.shape().dimensions_vec().len() == src_base.len());
    debug_assert!(self.shape().dimensions_vec().len() == dest_base.len());

    let mut func = |_t: PrimitiveType| -> Result<(), String> {
      Ok(())
    };
    array_type_switch(&mut func, &self.shape().element_type())
  }

  fn copy_slice_from_internal(
    &self,
    _src_literal: &Literal,
    _src_base: &Vec<i64>,
    _dest_base: &Vec<i64>,
    _copy_size: &Vec<i64>) -> Result<(), String>
  {
    /*
    let linear_index =
      |shape: &Shape, multi_index: &Vec<i64>| -> i64
    {
      IndexUtil::multi_dimensional_index_to_linear_index(shape, multi_index)
    };
    let dest_data = self.mutable_data(&vec![]);
    let src_data = src_literal.data(&vec![]);
    if src_literal.shape().dimensions_vec().len() == 0 ||
      self.shape().dimensions_vec().len() == 0
    {
      // If any of the two shapes are scalars, just assign the value once.
      debug_assert!(copy_size.is_empty());
      //dest_data[linear_index(self.shape(), dest_base) as usize] =
        //src_data[linear_index(src_literal.shape(), src_base) as usize];
    } else if !ShapeUtil::is_zero_element_array(self.shape()) &&
      !ShapeUtil::is_zero_element_array(src_literal.shape())
    {
      
    }
    */
    Ok(())
  }

  // Returns the shape of the literal.
  pub fn shape(&self) -> &Shape {
    self.root_piece().subshape()
  }

  pub fn mutable_shape(&mut self) -> &mut Shape {
    &mut self.mutable_root_piece().subshape
  }

  pub fn mutable_shape_do_not_use(&mut self) -> &mut Shape {
    //&mut self.shape
    self.mutable_shape()
  }

  pub fn set_shape(&mut self, shape: Shape) {
    self.root_piece.set_subshape(shape);
  }

  // Returns a vec of the array for this literal for the given T
  // (e.g., float). CHECKs if the subshape of the literal at the given
  // ShapeIndex is not array. See primitive_util.h for the mapping from Blitz type
  // to native type.
  pub fn data(&self, shape_index: &Vec<i64>) -> &Vec<Box<dyn Any>> {
    self.piece(shape_index).data()
  }

  pub fn mutable_data(&mut self, shape_index: &Vec<i64>) -> &mut Vec<Box<dyn Any>> {
    self.mutable_piece(shape_index).mutable_data()
  }

  pub fn set_data(&mut self, shape_index: &Vec<i64>, data: Vec<Box<dyn Any>>) {
    self.mutable_piece(shape_index).set_data(data);
  }

  // Returns a const pointer to (or size of) the underlying buffer holding the
  // array at the given shape index. CHECKs if the subshape of the literal at
  // the given ShapeIndex is not array.
  pub fn untyped_data(&self, shape_index: &Vec<i64>) -> &Vec<Box<dyn Any>> {
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
  pub fn get<NativeT>(
    &self,
    multi_index: &Vec<i64>,
    shape_index: &Vec<i64>) -> Option<&NativeT>
    where NativeT: 'static
  {
    self.piece(shape_index).get::<NativeT>(multi_index)
  }

  pub fn get_linear<NativeT>(
    &self,
    linear_index: &Vec<i64>) -> Option<&NativeT>
    where NativeT: 'static
  {
    self.root_piece().get::<NativeT>(linear_index)
  }

  // Sets an element in the literal at the given index. The multi_index is
  // CHECKed against the dimension sizes.
  pub fn set<NativeT>(
    &mut self,
    multi_index: &Vec<i64>,
    shape_index: &Vec<i64>,
    value: NativeT)
    where NativeT: 'static
  {
    self.mutable_piece(shape_index).set(multi_index, value);
  }

  // Overloads of Set for array literals. CHECKs if the literal is not
  // array-shaped and dense.
  pub fn set_at_root<NativeT>(
    &mut self, multi_index: &Vec<i64>, value: NativeT)
    where NativeT: 'static
  {
    self.mutable_root_piece().set(multi_index, value);
  }

  // Get the dynamic size on dim_index in the literal at the given shape_index.
  pub fn get_dynamic_size(
    &self, dim_index: i64, shape_index: &Vec<i64>) -> i64
  {
    self.piece(shape_index).get_dynamic_size(dim_index)
  }

  pub fn set_dynamic_size(
    &mut self, dim_index: i64, shape_index: &Vec<i64>, size: i64)
  {
    let mut shape_index_clone = vec![];
    shape_index_clone.clone_from(shape_index);
    let subshape = ShapeUtil::get_mutable_subshape(
      self.mutable_shape_do_not_use(), shape_index_clone);

    assert!(LayoutUtil::is_dense_array(subshape));
    assert!(subshape.dimensions(dim_index as usize) >= size);
    subshape.set_dynamic_dimension(dim_index as usize, true);

    //assert_eq!(self.piece(shape_index).subshape(), subshape);
    self.mutable_piece(shape_index).set_dynamic_size(dim_index, size);
  }

  // Returns the element value at index (0, ..., 0), however many zeroes are
  // required for that index.
  pub fn get_first_element(&self) -> &Box<dyn Any> { 
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
  pub fn get_as_string(
    &self, multi_index: &Vec<i64>, shape_index: &Vec<i64>) -> String
  {
    let subshape = 
      ShapeUtil::get_subshape(self.shape(), shape_index);
    assert!(subshape.is_array());
    let mut f = |t: PrimitiveType| -> String {
      if is_integral_type(&t) {
        let value = self.get::<i32>(multi_index, shape_index);
        if value.is_some() {
          return value.unwrap().to_string();
        }
        let value = self.get::<i64>(multi_index, shape_index);
        if value.is_some() {
          return value.unwrap().to_string();
        }
        let value = self.get::<u32>(multi_index, shape_index);
        if value.is_some() {
          return value.unwrap().to_string();
        }
      }
      if is_floating_point_type(&t) {
        let value = self.get::<f32>(multi_index, shape_index);
        if value.is_some() {
          return value.unwrap().to_string();
        }
        let value = self.get::<f64>(multi_index, shape_index);
        if value.is_some() {
          return value.unwrap().to_string();
        }
      }
      if is_complex_type(&t) {
        let value = self.get::<Complex64>(multi_index, shape_index);
        if value.is_some() {
          let mut str = "(".to_string();
          str.push_str(&value.unwrap().re.to_string());
          str.push_str(", ");
          str.push_str(&value.unwrap().im.to_string());
          str.push_str(")");
        return str;
        }
      }
      if t == PrimitiveType::Pred {
        let value = *self.get::<bool>(multi_index, shape_index).unwrap();
        if value {
          return "true".to_string();
        } else {
          return "false".to_string();
        }
      }
      //unreachable!("{:?}", primitive_type_name(&t));
      unreachable!();
    };
    array_type_switch(&mut f, &subshape.element_type())
  }

  pub fn equal(&self, other: &Literal, layout_sensitive: bool) -> bool {
    // Checking the structure of tuple literals. Checks for dense arrays are
    // performed below.
    if !ShapeUtil::equal_structure(self.shape(), other.shape()) {
      return false;
    }
    let mut f =
      |index: &Vec<i64>, piece: &Piece| -> bool
    {
      let other_piece = other.piece(index);
      let subshape = piece.subshape();
      let other_subshape = other_piece.subshape();
      if subshape.element_type() != other_subshape.element_type() {
        return false;
      }
      if !piece.subshape().is_array() {
        return true;
      }
      if subshape.dimensions_size() != other_subshape.dimensions_size() {
        return false;
      }
      if layout_sensitive && (subshape.layout() != other_subshape.layout()) {
        return false;
      }
      for i in 0..subshape.dimensions_size() {
        if piece.get_dynamic_size(i as i64) !=
          other_piece.get_dynamic_size(i as i64)
        {
          return false;
        }
      }
      if !piece.equal_elements(other_piece) {
        return false;
      }
      true
    };
    self.root_piece().for_each_subpiece_with_bool(&mut f)
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
        return as_i64.unwrap() == *value_i64.unwrap();
      }
      let value_f64 = value_any.downcast_ref::<f64>();
      if value_f64.is_some() {
        return as_i64.unwrap() as f64 == *value_f64.unwrap();
      }
      let value_c64 = value_any.downcast_ref::<Complex64>();
      if value_c64.is_some() {
        if value_c64.unwrap().im == 0.0 {
          return as_i64.unwrap() as f64 == value_c64.unwrap().re;
        }
      }
    }

    let as_f64 = self.get_as_double(multi_index);
    if as_f64.is_some() {
      let value_any: &dyn Any = value;
      let value_f64 = value_any.downcast_ref::<f64>();
      if value_f64.is_some() {
        return as_f64.unwrap() == *value_f64.unwrap();
      }
      let value_i64 = value_any.downcast_ref::<i64>();
      if value_i64.is_some() {
        return as_f64.unwrap() == *value_i64.unwrap() as f64;
      }
      let value_c64 = value_any.downcast_ref::<Complex64>();
      if value_c64.is_some() {
        if value_c64.unwrap().im == 0.0 {
          return as_f64.unwrap() == value_c64.unwrap().re;
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
      return as_f64.unwrap() == value.re && value.im == 0.0;
    }
    let as_complex128 = self.get_as_complex_64(multi_index);
    if as_complex128.is_some() {
      return as_complex128.unwrap() == *value;
    }
    unreachable!("Unsupported type.");
  }

  // As Get(), but determines the correct type and converts the value into
  // int64_t.  This literal must be an array.
  pub fn get_integral_as_s64(&self, multi_index: &Vec<i64>) -> Option<i64> {
    assert!(self.shape().is_array());
    let mut f = |t: PrimitiveType| -> Option<i64> {
      if is_integral_type(&t) {
        let value = self.get::<i64>(multi_index, &vec![]);
        return Some(*value.unwrap());
      }
      if t == PrimitiveType::Pred {
        let value = self.get::<bool>(multi_index, &vec![]);
        if value.is_some() && *value.unwrap() == true {
          return Some(1);
        } else {
          return Some(0);
        }
      }
      None
    };
    primitive_type_switch(&mut f, self.shape().element_type())
  }

  // As Get(), but determines the correct type, and converts the value into
  // double. This literal must be an array.
  pub fn get_as_double(&self, multi_index: &Vec<i64>) -> Option<f64>
  {
    assert!(LayoutUtil::is_dense_array(self.shape()));
    let mut f = |t: PrimitiveType| -> Option<f64> {
      if is_floating_point_type(&t) {
        let value = self.get::<f64>(multi_index, &vec![]);
        return Some(*value.unwrap());
      }
      None
    };
    primitive_type_switch(&mut f, self.shape().element_type())
  }

  // As Get(), but determines the correct type, and converts the value into
  // complex128. All floating point types can be converted into complex128.
  // This literal must be an array.
  pub fn get_as_complex_64(&self, multi_index: &Vec<i64>) -> Option<Complex<f64>> {
    let mut f = |t: PrimitiveType| -> Option<Complex<f64>> {
      if is_array_type(&t) {
        if is_complex_type(&t) {
          let value =
            self.get::<Complex64>(multi_index, &vec![]);
          return Some(*value.unwrap());
        } else if is_floating_point_type(&t) {
          let r_value = self.get::<f64>(multi_index, &vec![]);
          return Some(Complex64::new(*r_value.unwrap(), 0.0));
        } else if is_integral_type(&t) {
          if self.get::<i64>(multi_index, &vec![]).is_some() {
            let r_value = self.get::<i64>(multi_index, &vec![]).unwrap();
            return Some(Complex64::new(*r_value as f64, 0.0));
          } else if self.get::<i32>(multi_index, &vec![]).is_some() {
            let r_value = self.get::<i32>(multi_index, &vec![]).unwrap();
            return Some(Complex64::new(*r_value as f64, 0.0));
          } else {
            unreachable!();
          }
        }
      }
      None
    };
    primitive_type_switch(&mut f, self.shape().element_type())
  }

  // Convert each element whose *linear* index is listed in "linear_indices"
  // to a double and return the sum of all of these elements.
  pub fn get_sum_as_double(&self, linear_indices: &Vec<i64>) -> Option<f64> {
    assert!(self.shape().is_array());
    if !is_floating_point_type(&self.shape().element_type()) {
      return None;
    }
    let mut f = |_t: PrimitiveType| -> Option<f64> {
      let mut sum = 0.0;
      let d = self.root_piece().data();
      for idx in linear_indices {
        let value= &d[*idx as usize];
        sum += value.downcast_ref::<f64>().unwrap();
      }
      Some(sum)
    };
    floating_point_type_switch(&mut f, &self.shape().element_type())
  }

  pub fn set_integral_as_s64(&mut self, _multi_index: &Vec<i64>, _value: i64) {
    unimplemented!()
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

  pub fn each_cell<NativeT, F>(&self, _per_cell: F) where F: Fn(&Vec<i64>, &NativeT) {
    /*
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
    */
  }

  // Checks whether all of this literal's values are equal to the given
  // scalar literal.
  pub fn is_all(&self, scalar: &Literal) -> bool {
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
  {
    if !self.shape().is_array() {
      return false;
    }
    let t = self.shape().element_type();
    if !is_integral_type(&t ){
      return false;
    }
    let mut scalar = Literal::new_from_shape(
      &ShapeUtil::make_scalar_shape(&t));
    let mut f = |primitive_t: PrimitiveType| -> bool {
      if is_integral_type(&primitive_t) {
        scalar.set_at_root::<i64>(&vec![], value);
        return self.root_piece().is_all(&scalar);
      }
      false
    };
    array_type_switch(&mut f, &t)
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
  {
    let t = self.shape().element_type();
    if !is_floating_point_type(&t) {
      return false;
    }
    let mut scalar: Literal = Literal::new_from_shape(
      &ShapeUtil::make_scalar_shape(&t));
    let mut f = |primitive_t: PrimitiveType| -> bool {
      if is_floating_point_type(&primitive_t) {
        scalar.set_at_root(&vec![], value);
        return self.root_piece().is_all(&scalar);
      }
      false
    };
    floating_point_type_switch(&mut f, &t)
  }

  pub fn is_all_complex(&self, value: Complex<f64>) -> bool
  {
    let t = self.shape().element_type();
    if !is_complex_type(&t) {
      return false;
    }
    let mut scalar: Literal = Literal::new_from_shape(
      &ShapeUtil::make_scalar_shape(&t));
    let mut f = |primitive_t: PrimitiveType| -> bool {
      if is_complex_type(&primitive_t) {
        scalar.set_at_root(&vec![], value);
        return self.root_piece().is_all(&scalar);
      }
      false
    };
    complex_type_switch(&mut f, &t)
  }

  // Determines if this literal consists of the first element og the literal.
  // Returns false if this literal is not an array.
  pub fn is_all_first(&self) -> bool
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
    let first: Literal = self.slice(&start_indices, &end_indices);
    self.is_all(&first.reshape(&vec![]).unwrap())
  }

  // Returns the count of the elements in the array at the given shape index in
  // this literal.
  pub fn element_count(&self, index: &Vec<i64>) -> i64 {
    if index.is_empty() {
      // Common case, avoid GetSubshape().
      return ShapeUtil::elements_in(self.shape());
    }
    ShapeUtil::elements_in(&ShapeUtil::get_subshape(self.shape(), index))
  }

  // Returns the number of elements that have value equal to the given complex
  // value. Returns 0 if value does not fit in this literal's type or if the
  // literal is not an array.
  pub fn count_equal<NativeT>(&self, value: NativeT) -> usize
    where NativeT: 'static + Clone
  {
    let t = self.shape().element_type();
    if !is_array_type(&t) {
      return 0;
    }
    let mut scalar= Literal::new_from_shape(&ShapeUtil::make_scalar_shape(&t));
    let mut f = |_primitive_t: PrimitiveType| -> usize {
      scalar.set_at_root::<NativeT>(&vec![], value.clone());
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
  pub fn is_zero(&self, indices: &Vec<i64>) -> bool
  {
    assert!(LayoutUtil::is_dense_array(self.shape()), "Only supported dense arrays.");
    let mut f = |t: PrimitiveType| -> bool {
      if t == PrimitiveType::S32 {
        return *self.get::<i32>(
          indices, &vec![]).unwrap() == 0;
      } else if t == PrimitiveType::S64 {
        return *self.get::<i64>(
          indices, &vec![]).unwrap() == 0;
      } else if t == PrimitiveType::U32 {
        return *self.get::<u32>(
          indices, &vec![]).unwrap() == 0;
      } else if t == PrimitiveType::U64 {
        return *self.get::<u64>(
          indices, &vec![]).unwrap() == 0;
      } else if t == PrimitiveType::F32 || t == PrimitiveType::F64 {
        return *self.get::<f64>(
          indices, &vec![]).unwrap() == 0.0;
      } else if t == PrimitiveType::C64 || t == PrimitiveType::C128 {
        let c0 = Complex64::new(0.0, 0.0);
        return *self.get::<Complex64>(
          indices, &vec![]).unwrap() == c0;
      }
      false
    };
    array_type_switch(&mut f, &self.shape().element_type())
  }

  // Converts this literal to the given shape. Returns an error is the
  // conversion is not possible.
  pub fn convert_to_shape(&self, dest_shape: &Shape) -> Result<Literal, String> {
    if !dest_shape.is_tuple() {
      return self.convert(&dest_shape.element_type());
    }
    unimplemented!()
  }

  // Converts this literal to another primitive type using a bitcast
  // conversion. Returns an error if the conversion is not possible. This
  // literal must be array-shaped.
  pub fn bitcast_convert(&self, _dest_shape: &Shape) -> Result<Literal, String> {
    unimplemented!()
  }

  // Converts this literal to another primitive type. Returns an error if the
  // conversion is not possible. This literal must be array-shaped.
  pub fn convert(&self, _primitive_dest_t: &PrimitiveType) -> Result<Literal, String> {
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
    &self, new_layout: &Layout, shape_index: &Vec<i64>) -> Literal
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

    let result = Literal::new_from_shape(&new_shape);
    //result.base = self.clone();
    result
  }

  // An overload of Relayout which changes the layout of the entire shape rather
  // than being limited to a single array within the shape.
  pub fn relayout_with_shape(&self, shape: &Shape) -> Literal {
    assert!(ShapeUtil::compatible(shape, self.shape()));
    let result: Literal = Literal::new_from_shape(self.shape());
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
  pub fn to_static(&mut self) -> Literal {
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
  pub fn to_bounded_dynamic(&self, bounded_shape: &Shape) -> Literal {
    assert!(bounded_shape.is_dynamic());
    let mut result = Literal::new_from_shape(bounded_shape);
    let mut f = |subshape: &Shape, _index: &Vec<i64>| {
      if !subshape.is_array() { return; }
      for i in 0..subshape.rank() {
        if bounded_shape.is_dynamic_dimension(i as i64) {
          result.set_dynamic_size(
            i as i64,
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
  pub fn reshape(&self, dimensions: &Vec<i64>) -> Result<Literal, String> {
    if !LayoutUtil::is_dense_array(self.shape()) {
      return Err("Reshape is only supported for dense arrays.".to_string());
    }
    if self.shape().is_dynamic() {
      return Err("Dynamic reshape is not implemented.".to_string());
    }

    let mut output: Literal = Literal::new_from_shape(self.shape());
    if !LayoutUtil::is_monotonic_with_dim0_major(
      self.shape().layout().as_ref().unwrap())
    {
      let layout =
        LayoutUtil::get_default_layout_for_rank(self.shape().rank() as i64);
      output = self.relayout(&layout, &vec![]);
    } else {
      //output = Literal {
        //base: self.clone(),
      //};
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
    Literal::set_piece_shapes(shape, self.mutable_root_piece());
  }

  fn set_piece_shapes(shape: &Shape, piece: &mut Piece) {
    piece.set_subshape(shape.clone());
    if shape.is_tuple() {
      for i in 0..ShapeUtil::tuple_element_count(shape) {
        let subshape = shape.tuple_shapes(i);
        Literal::set_piece_shapes(
          subshape, piece.mutable_child(i).unwrap());
      }
    }
  }

  // Creates a new literal by broadcasting this literal with `dimensions` to
  // yield a literal of shape `result_shape`.
  pub fn broadcast(
    &self,
    result_shape: &Shape,
    dimensions: &Vec<i64>) -> Result<Literal, String>
  {
    let src_shape = self.shape();
    if !src_shape.is_array() {
      return Err("Broadcast only support arrays.".to_string());
    }
    let primitive_size =
      ShapeUtil::byte_size_of_primitive_type(&src_shape.element_type());
    
    match primitive_size {
      0 => return Literal::broadcast_helper(
        0, self, src_shape, result_shape, dimensions),
      1 => return Literal::broadcast_helper(
        1, self, src_shape, result_shape, dimensions),
      2 => return Literal::broadcast_helper(
        2, self, src_shape, result_shape, dimensions),
      4 => return Literal::broadcast_helper(
        4, self, src_shape, result_shape, dimensions),
      8 => return Literal::broadcast_helper(
        8, self, src_shape, result_shape, dimensions),
      16 => return Literal::broadcast_helper(
        16, self, src_shape, result_shape, dimensions),
      _ => return Err("Unhandled primitive size".to_string())
    }
  }

  fn broadcast_helper(
    _primitive_size: i64,
    src: &Literal,
    src_shape: &Shape,
    result_shape: &Shape,
    dimensions: &Vec<i64>) -> Result<Literal, String>
  {
    for i in 0..dimensions.len() {
      if src_shape.dimensions(i) !=
        result_shape.dimensions(dimensions[i] as usize)
      {
        return Err("src_shape dimension != result_shape dimension".to_string());
      }
    }
    if src_shape.element_type() != result_shape.element_type() {
      return Err("src_shape element_type != result_shape element_type".to_string());
    }
    let mut result = Literal::new_from_shape(result_shape);
    if src_shape.is_dynamic() {
      for i in 0..dimensions.len() {
        if src_shape.is_dynamic_dimension(i as i64) {
          // Set any dynamic sizes in the new literal.
          let dynamic_size =
            src.get_dynamic_size(i as i64, &vec![]);
          result.set_dynamic_size(
            dimensions[i], &vec![], dynamic_size);
        }
      }
    }
    if ShapeUtil::elements_in(result_shape) == 0 {
      // Nothing to do.
      return Ok(result);
    }
    let _src_data = src.untyped_data(&vec![]);
    let _result_data = result.untyped_data(&vec![]);

    // Fast path for broadcasting a scalar to a result shape.
    if ShapeUtil::elements_in(src_shape) == 1 {
      let e = ShapeUtil::elements_in(result_shape);
      for _i in 0..e {
        // TODO
      }
      return Ok(result);
    }

    let _src_minor_to_mmajor =
      LayoutUtil::minor_to_major_from_shape(src_shape);
    let _result_minor_to_major =
      LayoutUtil::minor_to_major_from_shape(result_shape);

    //let func = |src_index: &Vec<i64>| -> bool {
      //true
    //};
    Ok(result)
  }

  // Creates a new literal by reordering the dimensions of this literal.
  // The given `permutation` must be a permutation of the dimension numbers
  // in the original literal, and it specifies the order of the new dimensions
  // in the result literal (i.e., new_order[i] = old_order[permutation[i]]).
  // For example, a transpose call on a literal of shape [3 x 8 x 4] and
  // `permutation` = {2, 0, 1} returns a new literal of shape [4 x 3 x 8].
  // This literal must be an array.
  pub fn transpose(&self, permutation: &Vec<i64>) -> Literal {
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
            inverse_permutation[i],
            &vec![],
            self.get_dynamic_size(i as i64, &vec![]));
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
  pub fn slice(
    &self,
    start_indices: &Vec<i64>,
    limit_indices: &Vec<i64>) -> Literal
  {
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
    
    let mut f = |t: PrimitiveType| {
      // TODO
      if is_integral_type(&t) {
        Literal::slice_internal::<i64>(
          &self, start_indices, &mut result_literal);
      } else if is_floating_point_type(&t) {
        Literal::slice_internal::<f64>(
          &self, start_indices, &mut result_literal);
      } else if is_predicate_type(&t) {
        Literal::slice_internal::<bool>(
          &self, start_indices, &mut result_literal);
      }
    };
    array_type_switch(&mut f, &result_shape.element_type());
    result_literal
  }

  fn slice_internal<NativeT>(
    src_literal: &Literal,
    start_indices: &Vec<i64>,
    result_literal: &mut Literal)
    where NativeT: 'static + Clone
  {
    let result_shape = &result_literal.shape().clone();    
    let mut new_indices = vec![0; result_shape.dimensions_vec().len()];
    let mut f = |indices: &Vec<i64>| -> NativeT {
      for i in 0..result_shape.dimensions_vec().len() {
        new_indices[i] = indices[i] + start_indices[i];
      }
      src_literal.get::<NativeT>(
        &new_indices, &vec![]).unwrap().clone()
    };
    let _ = result_literal.populate(&mut f);
    for dnum in 0..src_literal.shape().dimensions_vec().len() {
      if src_literal.shape().is_dynamic_dimension(dnum as i64) {
        let mut dynamic_size =
          src_literal.get_dynamic_size(dnum as i64, &vec![])
          - start_indices[dnum];
        assert!(dynamic_size >= 0);
        dynamic_size = i64::min(dynamic_size,
          result_shape.dimensions(dnum));
        result_literal.set_dynamic_size(
          dnum as i64,
          &vec![],
          dynamic_size);
      }
    }
  }

  // Creates a literal with a prepended dimension with bound "times"; e.g. a
  // f32[3x2] with times=4 will produce a f32[4x3x2] with the 3x2 from this
  // literal replicated four times.
  // This literal must be an array.
  pub fn replicate<NativeT>(&self, times: i64) -> Literal
    where NativeT: 'static + Clone
  {
    assert!(self.shape().is_array(), "only supported for dense arrays");
    let mut bounds = vec![times];
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
      let element = self.get::<NativeT>(
        &input_indices, &vec![]);
      literal.set_at_root(
        &output_indices, element.unwrap().clone());

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
  pub fn create_from_shape(shape: &Shape) -> Literal {
    let mut literal = Literal::new_from_shape(shape);
    literal.mutable_root_piece().for_each_mutable_subpiece(
      &mut |_index: &Vec<i64>, piece: &mut Piece| -> Result<(), String> {
        if piece.subshape().is_array() {
          let len = piece.size_bytes_dense() as usize;
          //let untyped_data = piece.mutable_untyped_data();
          for _i in 0..len {
            //untyped_data[i] = T::default();
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
  pub fn create_from_shape_with_unknown_leaf_arrays(shape: &Shape) -> Literal {
    Literal::new(
      shape,
      false,
      ArrayValueState::Unknown)
  }

  //Similar to create_from_shape() but marks all leaf arrays as undetermined.
  pub fn create_from_shape_with_undetermined_leaf_arrays(shape: &Shape) -> Literal {
    Literal::new(
      shape,
      false,
      ArrayValueState::Undetermined)
  }

  pub fn populate<F, NativeT>(
    &self, generator: &mut F) -> Result<(), String>
    where F: FnMut(&Vec<i64>)->NativeT
  {
    assert!(LayoutUtil::is_dense_array(self.shape()),
      "Only supported for dense arrays.");
    let mut f =
      |indexes: &Vec<i64>| -> NativeT
    {
      generator(indexes)
    };
    self.populate_internal(&mut f, false)
  }

  pub fn populate_internal<F, NativeT>(
    &self, _generator: &mut F, parallel: bool) -> Result<(), String>
    where F: FnMut(&Vec<i64>)->NativeT
  {
    assert!(LayoutUtil::is_dense_array(self.shape()));
    if !self.data(&vec![]).is_empty() {
      assert!(self.shape().element_type() ==
        native_to_primitive_type(&self.data(&vec![])[0]));
    }
    
    let populator =
      |_dest: &Vec<Box<dyn Any>>, _indices: &Vec<i64>, _thread_id: i64|
    {
      // TODO
    };
    self.populate_inplace_internal(populator, parallel);
    Ok(())
  }

  pub fn populate_inplace_internal<F>(&self, populator: F, parallel: bool)
    where F: Fn(&Vec<Box<dyn Any>>, &Vec<i64>, i64)
  {
    let dest_base = self.untyped_data(&vec![]);
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
        |indexes: &Vec<i64>, _thread_id: i64| -> Result<bool, String>
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
          //populator(&vec![dest_ptr], &minor_scan_indexes, thread_id);
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
      populator(&dest_base, &vec![], -1);
    }
  }

  pub fn populate_r1<NativeT>(&mut self, values: &Vec<NativeT>)
    where NativeT: 'static + Clone + Default
  {
    assert!(self.shape().is_array(), "nly supported for dense arrays");
    assert!(self.shape().dimensions_size() == 1);
    if self.shape().is_static() {
      assert_eq!(ShapeUtil::elements_in(self.shape()), values.len() as i64);
    } else {
      assert_eq!(self.get_dynamic_size(0, &vec![]), values.len() as i64);
    }
    assert_eq!(self.shape().element_type(),
      native_to_primitive_type(&NativeT::default()));
    let data_span =
      self.mutable_data(&vec![]);
    
    if data_span.is_empty() {
      for v in values {
        data_span.push(Box::new(v.clone()));
      }
    } else {
      let mut count = 0;
      for v in values {
        data_span[count] = Box::new(v.clone());
        count += 1;
      } 
    }
  }

  pub fn populate_r2<NativeT>(&mut self, values: &Vec<Vec<NativeT>>)
    where NativeT: 'static + Clone
  {
    assert!(self.shape().is_array(), "only supported for dense arrays");
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
        self.set_at_root::<NativeT>(&vec![dim0, dim1], value.clone());
        dim1 += 1;
      }
      assert_eq!(values_dim1_size, dim1 as usize);
      dim0 += 1;
    }
  }

  // Fills this literal with the given value.
  pub fn populate_with_value<NativeT>(&mut self, value: NativeT)
    where NativeT: 'static + Clone
  {
    assert!(self.shape().is_array(), "only supported for dense arrays");
    assert_eq!(self.shape().element_type(), native_to_primitive_type(&value));

    if !self.data(&vec![]).is_empty() {
      for v in self.mutable_data(&vec![]) {
        *v = Box::new(value.clone());
      }
    } else { // TODO
      if self.shape().dimensions_size() == 0 {
        self.mutable_data(&vec![]).push(Box::new(value.clone()));  
      } else if self.shape().dimensions_size() == 1 {
        for _i in 0..self.shape().dimensions(0) {
          self.mutable_data(&vec![]).push(Box::new(value.clone()));
        }
      } else if self.shape().dimensions_size() == 2 {
        let size = self.shape().dimensions(0) * self.shape().dimensions(1);
        for _i in 0..size {
          self.mutable_data(&vec![]).push(Box::new(value.clone()));
        } 
      }
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
        shape_size = self.get_dynamic_size(dim as i64, &vec![]);
      }
      assert_eq!(values.dim(dim), shape_size as usize);
    }

    // TODO
  }

  // This operation is the inverse of DecomposeTuple. The given elements are
  // moved into the tuple elements of a new tuple-shaped Literal which is
  // returned. Upon return, each of the Literals in 'elements' is set to a nil
  // shape (empty tuple).
  pub fn move_into_tuple(elements: &mut Vec<Literal>) -> Literal {
    let mut element_shapes = vec![];
    for elt in &mut *elements {
      element_shapes.push(elt.shape());
    }
    let mut literal = Literal::new_from_shape(
      &ShapeUtil::make_tuple_shape_with_ptrs(element_shapes));
    for i in 0..elements.len() {
      let result = literal.move_from(
        &mut elements[i], &vec![i as i64]);
      if result.is_err() {
        assert!(false);
      }
    }
    literal
  }

  // Similar to CopyFrom, but with move semantics. The subshape of this literal
  // rooted at 'dest_shape_index' must be *equal* to the shape 'src_literal'
  // (layouts and shapes must match), but need not be arrays. The memory
  // allocated in this literal for the subshape at dest_shape_index is
  // deallocated, and the respective buffers are replaced with those in
  // src_literal. Upon return, src_literal is set to a nil shape (empty tuple).
  pub fn move_from(
    &mut self,
    src_literal: &mut Literal,
    dest_shape_index: &Vec<i64>) -> Result<(), String>
  {
    let dest_subsgape =
      ShapeUtil::get_subshape(self.shape(), dest_shape_index);
    if !ShapeEqual::new().equal(&dest_subsgape, src_literal.shape()) {
      let err_msg = "Destination subshape not equal to source shape".to_string();
      return Err(err_msg);
    }
    let mut func =
      |src_index: &Vec<i64>, src_piece: &mut Piece| -> Result<(), String>
    {
      if !src_piece.subshape().is_array() {
        return Ok(());
      }
      let mut dest_index = vec![];
      dest_index.clone_from(dest_shape_index);
      for i in src_index {
        dest_index.push(*i);
      }
      let dest_piece = self.mutable_piece(&dest_index);
      dest_piece.deallocate_buffers();
      dest_piece.move_data_from(src_piece);
      Ok(())
    };
    src_literal.mutable_root_piece().for_each_mutable_subpiece(&mut func);

    *src_literal.mutable_shape() = nil_shape();
    *src_literal.mutable_root_piece() = Piece::new();
    src_literal.mutable_root_piece().set_subshape(nil_shape());
    Ok(())    
  }

  // Returns a vector containing the tuple elements of this Literal as separate
  // Literals. This Literal must be tuple-shaped and can be a nested tuple. The
  // elements are moved into the new Literals; no data is copied. Upon return
  // this Literal is set to a nil shape (empty tuple)
  pub fn decompose_tuple(&mut self) -> Vec<Literal> {
    assert!(self.shape().is_tuple());
    let mut elements = vec![];
    let tuple_element_count = ShapeUtil::tuple_element_count(self.shape());
    for i in 0..tuple_element_count {
      let mut literal = Literal::new(
        &ShapeUtil::get_subshape(self.shape(), &vec![i as i64]),
        false,
        ArrayValueState::Unknown);
      let mut func =
        |index: &Vec<i64>, dest_piece: &mut Piece| -> Result<(), String>
      {
        if dest_piece.subshape().is_tuple() {
          return Ok(());
        }
        let mut src_index = vec![i as i64];
        for j in index {
          src_index.push(*j);
        }
        let src_piece = self.piece(&src_index);

        // Move the respective buffer over to the element Literal.
        dest_piece.move_data_from(src_piece);
        Ok(())
      };
      literal.root_piece.for_each_mutable_subpiece(&mut func);
      elements.push(literal);
    }
    // Set this literal to be nil-shaped.
    *self = Literal::default();
    elements
  }

  fn piece(&self, shape_index: &Vec<i64>) -> &Piece {
    let mut piece = self.root_piece();
    for i in  shape_index {
      assert!(*i >= 0);
      assert!((*i as usize) < piece.children_size());
      piece = piece.child(*i as usize).unwrap();
    }
    piece
  }

  fn mutable_piece(&mut self, shape_index: &Vec<i64>) -> &mut Piece {
    let mut piece = self.mutable_root_piece();
    for i in  shape_index {
      assert!(*i >= 0);
      assert!((*i as usize) < piece.children_size());
      piece = piece.mutable_child(*i as usize).unwrap();
    }
    piece
  }

  // Returns the piece at the root of the shape.
  fn root_piece(&self) -> &Piece {
    &self.root_piece
  }

  fn mutable_root_piece(&mut self) -> &mut Piece {
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
    shape_index: &Vec<i64>,
    print_shape: bool,
    print_layout: bool,
    one_line: bool,
    printer: &mut dyn Printer)
  {
    let subshape =
      ShapeUtil::get_subshape(self.shape(), shape_index);
    let mut first_str = "(\n".to_string();
    if one_line { first_str = "( ".to_string(); }
    printer.append(&first_str);

    for i in 0..ShapeUtil::tuple_element_count(&subshape) {
      let mut element_index = vec![];
      element_index.clone_from(shape_index);
      element_index.push(i as i64);
      let mut delim_str = ",\n".to_string();
      if one_line { delim_str = ", ".to_string(); }
      if i > 0 {
        printer.append(&delim_str);
      }
      self.print_helper(&element_index,
        print_shape, print_layout, one_line, printer);
    }

    let mut last_str = "\n)".to_string();
    if one_line { last_str = " )".to_string(); }
    printer.append(&last_str);
  }

  fn dense_array_print_helper(
    &self,
    shape_index: &Vec<i64>,
    print_shape: bool,
    print_layout: bool,
    oneline: bool,
    printer: &mut dyn Printer)
  {
    let subshape =
      ShapeUtil::get_subshape(self.shape(), shape_index);
    let rank = subshape.rank();
    let mut linebreak = " ";
    if !oneline { linebreak = "\n"; }

    if print_shape {
      Literal::print_shape(print_layout, &subshape, printer);
      if subshape.is_dynamic() {
        printer.append(&"(".to_string());
        for i in 0..subshape.rank() {
          printer.append(&self.get_dynamic_size(
            i as i64, shape_index).to_string());
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
      dimensions.push(self.get_dynamic_size(i as i64, shape_index));
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
      #[allow(unused_assignments)]
      let mut elem = "".to_string();
      if subshape.element_type() == PrimitiveType::Pred && rank > 0 {
        if *self.get::<bool>(&accum_indices, shape_index).unwrap(){
          elem = "1".to_string();
        } else {
          elem = "0".to_string();
        }
      } else {
        elem = self.get_as_string(&accum_indices, shape_index);
      }
      printer.append(&elem);
    } else {
      printer.append(&Literal::brace_to_string(
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
      printer.append(&Literal::brace_to_string(
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
        Literal::print_shape(print_layout, &subshape, printer);
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

  // Array literals could be in one of the following three states:
  //   1) Known: we have evaluated and known the value of the array literal.
  //   2) Unknown: we have tried to evaluate the array literal, but its value
  //               cannot be evaluated statically.
  //   3) Undetermined: we haven't tried to evaluate the array literal.
  //  Unknown and Undetermined states are only meant to be used within
  //  HloEvaluator. The rest of XLA assumes array literals are all known.
  //  Literals that are unknown or undetermined can be copied from, using
  //  CopyFrom and Clone, or moved from using move constructor. Accessing values
  //  of such literals causes undefined behavior.
#[derive(Debug, Clone, PartialEq)]
pub enum ArrayValueState {
  Known,
  Unknown,
  Undetermined,
}

// A data structure representing a subshape at a particular ShapeIndex within
// the literal. For array-shaped ShapeIndexes, this data structure holds the
// pointer to the memory allocated for the array data.
//#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
  subshape: Shape,
  children: Option<Vec<Piece>>,
  data: Vec<Box<dyn Any>>,
  storage: Storage,
  dynamic_size_buffer: Vec<i64>,
  array_value_state: ArrayValueState,
}

impl Piece {
  pub fn new() -> Self {
    let mut instance = Piece {
      subshape: Shape::new(),
      children: Some(Vec::new()), //None,
      data: Vec::new(),
      storage: Storage::default(),
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
  pub fn data(&self) -> &Vec<Box<dyn Any>> {
    assert!(self.subshape().is_array(), "only supported for dense arrays");
    assert!(!self.subshape().has_layout() ||
      self.subshape().layout().as_ref().unwrap().element_size_in_bits() == 0,
      "not supported for layouts with custom bit size");
    
    &self.data
  }

  pub fn mutable_data(&mut self) -> &mut Vec<Box<dyn Any>> {
    assert!(self.subshape().is_array(), "only supported for dense arrays");
    assert!(!self.subshape().has_layout() ||
      self.subshape().layout().as_ref().unwrap().element_size_in_bits() == 0,
      "not supported for layouts with custom bit size");
    
    &mut self.data
  }

  pub fn set_data(&mut self, data: Vec<Box<dyn Any>>) {
    self.data = data;
  }

  // Returns the buffer holding the array data for this piece as a void*. This
  // piece must be array-shaped.
  pub fn untyped_data(&self) -> &Vec<Box<dyn Any>> {
    assert!(self.subshape().is_array());
    &self.data
  }

  pub fn mutable_untyped_data(&mut self) -> &mut Vec<Box<dyn Any>> {
    assert!(self.subshape().is_array());
    &mut self.data
  }

  // Gets an element in the array at the given index. The multi_index
  // is CHECKed against the dimension sizes of the array.  This piece must be
  // array-shaped.
  pub fn get<NativeT>(&self, multi_index: &Vec<i64>) -> Option<&NativeT>
    where NativeT: 'static
  {
    assert!(self.subshape().is_array(), "only supported for dense arrays");
    let index = IndexUtil::multi_dimensional_index_to_linear_index(
      self.subshape(), multi_index);
    self.data[index as usize].downcast_ref::<NativeT>()
  }

  // Sets an element in the array at the given index. The multi_index
  // is CHECKed against the dimension sizes of the array.  This piece must be
  // array-shaped.
  pub fn set<NativeT>(&mut self, multi_index: &Vec<i64>, value: NativeT)
    where NativeT: 'static
  {
    assert!(self.subshape().is_array(), "only supported for dense arrays");
    let index = IndexUtil::multi_dimensional_index_to_linear_index(
      self.subshape(), multi_index);
    self.data.insert(index as usize, Box::new(value));
  }

  pub fn get_dynamic_size(&self, dim_index: i64) -> i64 {
    assert!(self.subshape().is_array());
    if !self.subshape.is_dynamic_dimension(dim_index) {
      // This is a static dimension, return size.
      return self.subshape.dimensions(dim_index as usize);
    }
    self.dynamic_size_buffer()[dim_index as usize] as i64
  }

  pub fn set_dynamic_size(&mut self, dim_index: i64, size: i64) {
    assert!(self.subshape().is_array());
    assert!(self.subshape.is_dynamic_dimension(dim_index));
    self.mutable_dynamic_size_buffer()[dim_index as usize] = size;
  }

  pub fn move_data_from(&mut self, from: &Piece) {
    debug_assert!(!self.storage.is_dense_rep());
    debug_assert!(!self.storage.is_tuple_rep());

    // TODO
    let mut data = vec![];
    for from_value in from.data() {
      if from_value.downcast_ref::<bool>().is_some() {
        let value = *from_value.downcast_ref::<bool>().unwrap();
        let box_value: Box<dyn Any> = Box::new(value);
        data.push(box_value);
      } else if from_value.downcast_ref::<i32>().is_some() {
        let value = *from_value.downcast_ref::<i32>().unwrap();
        let box_value: Box<dyn Any> = Box::new(value);
        data.push(box_value);
      } else if from_value.downcast_ref::<i64>().is_some() {
        let value = *from_value.downcast_ref::<i64>().unwrap();
        let box_value: Box<dyn Any> = Box::new(value);
        data.push(box_value);
      } else if from_value.downcast_ref::<f64>().is_some() {
        let value = *from_value.downcast_ref::<f64>().unwrap();
        let box_value: Box<dyn Any> = Box::new(value);
        data.push(box_value);
      }
    }
    //self.storage.data = data;
    self.data = data;


  }

  pub fn copy_from(
    &mut self, src: &Piece, only_dynamic_bound: bool) -> Result<(), String>
  {
    assert!(self.subshape().is_array(), "only supported for dense arrays");
    assert!(src.subshape().is_array(), "only supported for dense arrays");

    if !only_dynamic_bound {
      assert!(ShapeUtil::compatible(self.subshape(), src.subshape()));
    }
    if src.array_value_state == ArrayValueState::Unknown ||
       src.array_value_state == ArrayValueState::Undetermined
    {
      if self.array_value_state == ArrayValueState::Known {
        self.deallocate_buffers(); // TODO
      }
      self.array_value_state = src.array_value_state.clone();
      return Ok(());
    } else {
      assert!(src.array_value_state == ArrayValueState::Known);
      if self.array_value_state == ArrayValueState::Undetermined ||
         self.array_value_state == ArrayValueState::Unknown
      {
        let result = self.allocate_buffers(); // TODO
        if result.is_err() { return result; }
      }
      self.array_value_state = src.array_value_state.clone();
    }
    if ShapeEqual::new().equal(self.subshape(), src.subshape()) {
      // If the layouts are equal it's faster just to memcpy.
      let mut src_data = vec![];
      for src_value in src.data() {
        if src_value.downcast_ref::<bool>().is_some() {
          let value = *src_value.downcast_ref::<bool>().unwrap();
          let box_value: Box<dyn Any> = Box::new(value);
          src_data.push(box_value);
        } else if src_value.downcast_ref::<i32>().is_some() {
          let value = *src_value.downcast_ref::<i32>().unwrap();
          let box_value: Box<dyn Any> = Box::new(value);
          src_data.push(box_value);
        } else if src_value.downcast_ref::<i64>().is_some() {
          let value = *src_value.downcast_ref::<i64>().unwrap();
          let box_value: Box<dyn Any> = Box::new(value);
          src_data.push(box_value);
        } else if src_value.downcast_ref::<f64>().is_some() {
          let value = *src_value.downcast_ref::<f64>().unwrap();
          let box_value: Box<dyn Any> = Box::new(value);
          src_data.push(box_value);
        }
      }
      *self.mutable_data() = src_data;
    } else {
      let mut f = |_primitive_t: PrimitiveType| {
        if only_dynamic_bound {
          // TODO
        } else {
          // TODO
        }
      };
      array_type_switch(&mut f, &self.subshape().element_type());
    }
    assert_eq!(self.dynamic_size_buffer_bytes(), src.dynamic_size_buffer_bytes());
    if self.subshape().is_dynamic() && src.subshape().is_dynamic() {
      let mut dynamic_size_buffer = vec![];
      dynamic_size_buffer.clone_from_slice(
        &src.dynamic_size_buffer()[0..self.dynamic_size_buffer_bytes()]);
      *self.mutable_dynamic_size_buffer() = dynamic_size_buffer;
    }
    Ok(())
  }

  pub fn allocate_buffers(&mut self) -> Result<(), String> {
    unimplemented!()
  }

  pub fn deallocate_buffers(&mut self) {
    //unimplemented!() // TODO
  }

  pub fn buffer(&self) {
    unimplemented!()
  }

  // Gets/sets the buffer holding dynamic sizes.
  pub fn dynamic_size_buffer(&self) -> &Vec<i64> {
    &self.dynamic_size_buffer
  }

  pub fn mutable_dynamic_size_buffer(&mut self) -> &mut Vec<i64> {
    &mut self.dynamic_size_buffer
  }

  pub fn dynamic_size_buffer_bytes(&self) -> usize {
    assert!(self.subshape().is_array());
    self.subshape.dimensions_size() * size_of::<i64>()
  }

  // Gets or sets the subshape of this piece. This reference points to a
  // subshape within the shape in the containing Literal (Literal::shape_).
  pub fn subshape(&self) -> &Shape {
    &self.subshape
  }

  pub fn set_subshape(&mut self, subshape: Shape) {
    self.subshape = subshape;
  }

  // Returns the size in bytes of the buffer holding the dense array data.
  pub fn size_bytes_dense(&self) -> i64 {
    assert!(self.subshape().is_array());
    ShapeUtil::byte_size_of(&self.subshape, -1)
  }

  // The dynamic metadata starts at the end of the data in the literal.
  // The literal can have any number of bytes. For example, it could be a PRED
  // with 7 elements. `dynamic_size_buffer_offset` returns the number of bytes
  // before the dynamic size information including whatever padding is needed
  // to align the start of the dynamic size information so that it is aligned
  // to a multiple of `sizeof(DynamicSizeType)`.
  pub fn dynamic_size_byffer_offset(&self) -> usize {
    unimplemented!()
  }

  // Total size in bytes, including the dynamic size addition.
  // The shape can become dynamic after this literal is allocated, so we
  // over-allocate the margin for the dynamic shape description in case we
  // need it.
  pub fn total_bytes_dense(&self) -> usize {
    self.dynamic_size_byffer_offset() + self.dynamic_size_buffer_bytes()
  }

  // Returns the number of elements in this piece's array.
  pub fn element_count(&self) -> i64 {
    ShapeUtil::elements_in(&self.subshape)
  }

  pub fn child(&self, index: usize) -> Option<&Piece> {
    assert!(self.children.is_some());
    self.children.as_ref().unwrap().get(index)
  }

  pub fn mutable_child(&mut self, index: usize) -> Option<&mut Piece> {
    assert!(self.children.is_some());
    self.children.as_mut().unwrap().get_mut(index)
  }

  // Adds a child piece to this piece's children.
  pub fn emplace_back(&mut self, child: Piece) {
    assert!(self.children.is_some());
    self.children.as_mut().unwrap().push(child);
  }

  // Returns the size of children pieces of this piece.
  pub fn children_size(&self) -> usize {
    assert!(self.children.is_some());
    self.children.as_ref().unwrap().len()
  }

  // Visitor functions that recursively traverses the piece and calls the
  // given function at each child piece. The function has the type:
  //    void (const ShapeIndex& index, const Piece& piece)
  pub fn for_each_subpiece<F>(&self, func: &mut F)
    where F: FnMut(usize, &Piece) -> Result<(), String>
  {
    let _ = Piece::for_each_helper(func, self, 0);
  }

  pub fn for_each_mutable_subpiece<F>(&mut self, func: &mut F)
    where F: FnMut(&Vec<i64>, &mut Piece) -> Result<(), String>
  {
    let _ = Piece::for_each_mutable_helper(func, self, &mut vec![]);
  }

  // Same as above, but the function has the type:
  //    absl::Status (const ShapeIndex& index, Piece& piece)
  // The first non-OK return value is returned by the function.
  pub fn for_each_subpiece_with_status<F>(
    &mut self, func: &mut F) -> Result<(), String>
      where F: FnMut(&Vec<i64>, &mut Piece) -> Result<(), String>
  {
    Piece::for_each_mutable_helper(func, self, &mut vec![])
  }

  // Same as above, but the function has the type:
  //    Bool (const ShapeIndex& index, const Piece& piece)
  // The first non-true return value is returned by the function.
  pub fn for_each_subpiece_with_bool<F>(&self, func: &mut F) -> bool
    where F: FnMut(&Vec<i64>, &Piece) -> bool
  {
    Piece::for_each_helper_bool(func, self, &mut vec![])
  }

  pub fn all_elements_equal_value<NativeT>(
    data: &Vec<NativeT>, value: &NativeT) -> bool
    where NativeT: PartialEq
  {
    for v in data {
      if v != value { return false; }
    }
    true
  }

  // Checks whether all elements of this Piece are equal to the given literal.
  // Returns false if this Piece is not an array.
  // Preconditions:
  //  - `scalar` is a scalar.
  //  - `scalar`'s type matches that of `this`.
  pub fn is_all(&self, scalar: &Literal) -> bool {
    assert!(ShapeUtil::is_scalar(scalar.shape()));
    if !self.subshape.is_array() { return false; }

    assert_eq!(self.subshape.element_type(), scalar.shape().element_type());
    let mut f = |t: PrimitiveType| -> bool {
      // TODO
      if is_integral_type(&t) {
        let elt = scalar.get_first_element().downcast_ref::<i64>().unwrap();
        let mut native_data = vec![];
        for v in self.data() {
          if v.downcast_ref::<i32>().is_some() {
            native_data.push(*v.downcast_ref::<i32>().unwrap() as i64);
          }
          else if v.downcast_ref::<i64>().is_some() {
            native_data.push(*v.downcast_ref::<i64>().unwrap());
          }
          else {
            assert!(false, "invalid TypeId:{:?}", v.type_id());
          }
        }
        return Piece::all_elements_equal_value::<i64>(&native_data, elt);
      } else if is_floating_point_type(&t) {
        let elt = scalar.get_first_element().downcast_ref::<f64>().unwrap();
        let mut native_data = vec![];
        for v in self.data() {
          native_data.push(*v.downcast_ref::<f64>().unwrap());
        }
        return Piece::all_elements_equal_value::<f64>(&native_data, elt);
      } else if is_complex_type(&t) {
        let elt =
          scalar.get_first_element().downcast_ref::<Complex64>().unwrap();
        let mut native_data = vec![];
        for v in self.data() {
          native_data.push(*v.downcast_ref::<Complex64>().unwrap());
        }
        return Piece::all_elements_equal_value::<Complex64>(&native_data, elt);
      }
      false
    };
    primitive_util::array_type_switch(&mut f, &self.subshape.element_type())
  }

  // Returns the number of elements with equal value to the given literal.
  // Returns 0 if this Piece is not an array.
  pub fn count_all(&self, scalar: &Literal) -> usize {
    assert!(ShapeUtil::is_scalar(scalar.shape()));
    if !self.subshape().is_array() {
      return 0;
    }
    assert_eq!(self.subshape().element_type(), scalar.shape().element_type());
    let mut f = |t: PrimitiveType| -> usize {
      let mut count = 0;
      // TODO
      if is_integral_type(&t) {
        if scalar.get_first_element().downcast_ref::<i32>().is_some() {
          let value = *scalar.get_first_element().downcast_ref::<i32>().unwrap();
          for v in self.data() {
            if value == *v.downcast_ref::<i64>().unwrap() as i32 {
              count += 1;
            }
          }
        } else if scalar.get_first_element().downcast_ref::<i64>().is_some() {
          let value = *scalar.get_first_element().downcast_ref::<i64>().unwrap();
          for v in self.data() {
            if value == *v.downcast_ref::<i64>().unwrap() {
              count += 1;
            }
          } 
        }
      } else if is_floating_point_type(&t) {
        if scalar.get_first_element().downcast_ref::<f64>().is_some() {
          let value = scalar.get_first_element().downcast_ref::<f64>().unwrap();
          for v in self.data() {
            if value == v.downcast_ref::<f64>().unwrap() {
              count += 1;
            }
            if value.is_nan() && v.downcast_ref::<f64>().unwrap().is_nan() {
              count += 1;
            }
          }
        }
      } else if is_predicate_type(&t) {
        if scalar.get_first_element().downcast_ref::<bool>().is_some() {
          let value = scalar.get_first_element().downcast_ref::<bool>().unwrap();
          for v in self.data() {
            if value == v.downcast_ref::<bool>().unwrap() {
              count += 1;
            }
          }
        }
      } else if is_complex_type(&t) {
        if scalar.get_first_element().downcast_ref::<Complex64>().is_some() {
          let value =
            scalar.get_first_element().downcast_ref::<Complex64>().unwrap();
          for v in self.data() {
            if value == v.downcast_ref::<Complex64>().unwrap() {
              count += 1;
            }
          }
        }
      }
      count
    };
    array_type_switch(&mut f, &self.subshape.element_type())
  }

  // Returns true if this piece and 'other' contain the same data. This piece
  // and 'other' must be array-shaped and compatible. If a literal has dynamic
  // shape, comparison is done only for the valid elements.
  pub fn equal_elements(&self, other: &Piece) -> bool {
    if self.data.len() != other.data.len() {
      return false;
    }
    for i in 0..self.data.len() {
      let (orig_data, other_data) =
        (&self.data()[i], &other.data()[i]);
      if orig_data.type_id() != other_data.type_id() {
        return false;
      }
      // i32
      if let (Some(orig_value), Some(other_value)) =
        (orig_data.downcast_ref::<i32>(), other_data.downcast_ref::<i32>())
      {
        //println!("orig: {:?}. other: {:?}", *orig_value, *other_value);
        if orig_value == other_value { continue; }
        else { return false; }
      }
      // i64
      if let (Some(orig_value), Some(other_value)) =
        (orig_data.downcast_ref::<i64>(), other_data.downcast_ref::<i64>())
      {
        if orig_value == other_value { continue; }
        else { return false; }
      }
      // f64
      else if let (Some(orig_value), Some(other_value)) =
        (orig_data.downcast_ref::<f64>(), other_data.downcast_ref::<f64>())
      {
        //println!("orig: {:?}, other: {:?}", orig_value, other_value);
        if orig_value == other_value { continue; }
        else { return false; }
      }
      // c64
      else if let (Some(orig_value), Some(other_value)) =
        (orig_data.downcast_ref::<Complex64>(), other_data.downcast_ref::<Complex64>())
      {
        //println!("orig: {:?}, other: {:?}", orig_value, other_value);
        if orig_value == other_value { continue; }
        else { return false; }
      }
      // bool
      else if let (Some(orig_value), Some(other_value)) =
        (orig_data.downcast_ref::<bool>(), other_data.downcast_ref::<bool>())
      {
        if orig_value == other_value { continue; }
        else { return false; }
      }
      // vec[f64]
      else if let (Some(orig_value), Some(other_value)) =
        (orig_data.downcast_ref::<Vec<f64>>(), other_data.downcast_ref::<Vec<f64>>())
      {
        if orig_value == other_value { continue; }
        else { return false; }
      }
      else {
        assert!(false, "unsupported data type: {:?}", other_data.type_id());
      }
    }
    true
  }

  // Returns true if this piece and other pieces have the same dynamic
  // dimension sizes.
  pub fn equal_dynamic_size(&self, _other: &Piece) -> bool {
    unimplemented!()
  }

  // See comments on ArrayValueState for detailed explanation.
  pub fn is_determined(&self) -> bool {
    if self.array_value_state == ArrayValueState::Undetermined {
      return false;
    }
    if self.subshape.is_tuple() {
      let mut are_all_leaf_arrays_determined = true;
      let mut func =
        |_index: usize, piece: &Piece| -> Result<(), String>
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
        |_index: usize, piece: &Piece| -> Result<(), String>
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

  fn get_tuple_rep(&self) -> &Option<Vec<Piece>> {
    &self.children
  }

  fn get_tuple_rep_mut(&mut self) -> &mut Option<Vec<Piece>> {
    &mut self.children
  }

  fn for_each_helper<F>(
    func: &mut F,
    piece: &Piece,
    index: usize) -> Result<(), String>
      where F: FnMut(usize, &Piece) -> Result<(), String>
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

  fn for_each_helper_bool<F>(
    func: &mut F,
    piece: &Piece,
    index: &mut Vec<i64>) -> bool
    where F: FnMut(&Vec<i64>, &Piece) -> bool
  {
    if !func(index, piece) {
      return false;
    }
    let tuple_rep = piece.get_tuple_rep(); // TODO
    if tuple_rep.is_some() {
      for i in 0..tuple_rep.as_ref().unwrap().len() {
        index.push(i as i64);
        if !Piece::for_each_helper_bool(
          func, &tuple_rep.as_ref().unwrap()[i], index) {
          return false;
        }
        index.pop();
      }
    }
    true
  }

  fn for_each_mutable_helper<F>(
    func: &mut F,
    piece: &mut Piece,
    index: &mut Vec<i64>) -> Result<(), String>
    where F: FnMut(&Vec<i64>, &mut Piece) -> Result<(), String>
  {
    let mut result = func(index, piece);
    if result.is_err() { return result; }
    let tuple_rep = piece.get_tuple_rep_mut();
    if tuple_rep.is_some() {
      let children = tuple_rep.as_mut().unwrap();
      let size = children.len();
      for i in 0..size {
        index.push(i as i64);
        result = Piece::for_each_mutable_helper(
          func, children.get_mut(i).unwrap(), index);
        if result.is_err() { return result; }
        index.pop();
      }
    }
    Ok(())
  }
}

// Uninitialized state representation.
pub struct Uninitialized {}

// Children pieces for tuple shaped pieces.
pub struct TupleRep {
  children: Vec<Piece>
}

// Out of line dense array storage.
pub struct DenseRep {
  data: Vec<Box<dyn Any>>,
}

// Inlined dense array storage.
pub struct DenseInlinedRep {
  data: Vec<Box<dyn Any>>,
}

// A wrapper around the piece representations with cached data pointer.
pub struct Storage {
  data: Vec<Box<dyn Any>>,
  rep_uninitialized: Option<Uninitialized>,
  rep_tuple: Option<TupleRep>,
  rep_dense: Option<DenseRep>,
  rep_dense_inlined: Option<DenseInlinedRep>,
}

impl Storage {
  pub fn default() -> Self {
    Storage {
      data: Vec::new(),
      rep_uninitialized: None,
      rep_tuple: None,
      rep_dense: None,
      rep_dense_inlined: None,
    }
  }

  pub fn is_uninitialized_rep(&self) -> bool {
    self.rep_uninitialized.is_some()
  }

  pub fn is_dense_rep(&self) -> bool {
    self.rep_dense.is_some()
  }

  pub fn is_dense_inlined_rep(&self) -> bool {
    self.rep_dense_inlined.is_some()
  }

  pub fn is_tuple_rep(&self) -> bool {
    self.rep_tuple.is_some()
  }

  pub fn emplace(&self) {
    unimplemented!()
  }

  pub fn get_dense_inlined_rep(&self) -> &Option<DenseInlinedRep>{
    &self.rep_dense_inlined
  }

  pub fn get_dense_rep(&self) -> &Option<DenseRep> {
    &self.rep_dense
  }

  pub fn get_tuple_rep(&self) -> &Option<TupleRep> {
    &self.rep_tuple
  }

  pub fn data(&self) -> &Vec<Box<dyn Any>> {
    &self.data
  }
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
  use super::*;
  use crate::literal_util::LiteralUtil;

  #[test]
  fn test_literal_scalar_to_string() {
    let true_lit = LiteralUtil::create_r0::<bool>(true);
    assert_eq!(true_lit.to_string(), "pred[] true".to_string());

    let false_lit = LiteralUtil::create_r0::<bool>(false);
    assert_eq!(false_lit.to_string(), "pred[] false".to_string());

    let s32_lit = LiteralUtil::create_r0::<i32>(-999);
    assert_eq!(s32_lit.to_string(), "s32[] -999".to_string());

    let s64_lit = LiteralUtil::create_r0::<i64>(-128);
    assert_eq!(s64_lit.to_string(), "s64[] -128".to_string());

    let f32_lit = LiteralUtil::create_r0::<f32>(3.14);
    assert_eq!(f32_lit.to_string(), "f32[] 3.14".to_string());

    let f64_lit = LiteralUtil::create_r0::<f64>(3.14159);
    assert_eq!(f64_lit.to_string(), "f64[] 3.14159".to_string());

    let c64_lit = LiteralUtil::create_r0(Complex64::new(3.14, 2.78));
    assert_eq!(c64_lit.to_string(), "c64[] (3.14, 2.78)".to_string());

    // TODO
  }

  #[test]
  fn test_literal_vector_to_string() {
    let pred_vec = LiteralUtil::create_r1(&vec![true, false, true]);
    assert_eq!(pred_vec.to_string(), "pred[3] {1, 0, 1}".to_string());
  }

  #[test]
  fn test_literal_linear_indexing() {
    let vec = LiteralUtil::create_r1(&vec![1.0, 2.0, 3.0]);
    assert_eq!(vec.get_linear(&vec![0]), Some(&1.0));
    assert_eq!(vec.get_linear(&vec![1]), Some(&2.0));
    assert_eq!(vec.get_linear(&vec![2]), Some(&3.0));
  }

  #[test]
  fn test_r2_to_string() {
    let literal = LiteralUtil::create_r2(
      &vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    let expected = "s32[3,2] {
 { 1, 2 },
 { 3, 4 },
 { 5, 6 }
}".to_string();
    assert_eq!(literal.to_string(), expected);
  }

  #[test]
  fn test_r2_dynamic_to_string() {
    let mut literal = LiteralUtil::create_r2(
      &vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    literal.set_dynamic_size(0, &vec![], 2);
    let expected = "s32[<=3,2](2,2) {
 { 1, 2 },
 { 3, 4 }
}".to_string();
    assert_eq!(literal.to_string(), expected);
    // A Less trivial case where the memory layout is not consecutive.
    let mut literal2 = LiteralUtil::create_r2(
      &vec![vec![1, 2, 3], vec![4, 5, 6]]);
    literal2.set_dynamic_size(1, &vec![], 2);
    let expected2 = "s32[2,<=3](2,2) {
 { 1, 2 },
 { 4, 5 }
}".to_string();
    assert_eq!(literal2.to_string(), expected2);
  }

  #[test]
  fn test_r2_bool_dynamic_to_string() {
    let mut literal = LiteralUtil::create_r2(
      &vec![vec![true, true, true], vec![true, true, true], vec![true, true, true]]);
    literal.set_dynamic_size(0, &vec![], 2);
    let expected = "pred[<=3,3](2,3) {
 { 1, 1, 1 },
 { 1, 1, 1 }
}".to_string();
    assert_eq!(literal.to_string(), expected);
  }

  #[test]
  fn test_tuple_to_string() {
    let scalar = LiteralUtil::create_r0(1.0);
    let matrix = LiteralUtil::create_r2(&vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let tuple = LiteralUtil::make_tuple(&vec![&scalar, &matrix]);
    let expected = "(
f64[] 1,
f64[2,2] {
 { 1, 2 },
 { 3, 4 }
}
)".to_string();
    assert_eq!(tuple.to_string(), expected);
  }

  #[test]
  fn test_scalar_equality() {
    let f64_42 = LiteralUtil::create_r0::<f64>(42.0);
    let f64_42_clone = LiteralUtil::create_r0::<f64>(42.0);
    assert!(f64_42.equal(&f64_42, false));
    assert!(f64_42.equal(&f64_42_clone, false));

    let f64_123 = LiteralUtil::create_r0::<f64>(123.0);
    assert!(!f64_42.equal(&f64_123, false));
  }

  #[test]
  fn test_non_scalar_equality() {
    let matrix = LiteralUtil::create_r2::<f64>(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix_clone = LiteralUtil::create_r2::<f64>(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix_different = LiteralUtil::create_r2::<f64>(
      &vec![vec![4.0, 3.0], vec![1.0, 2.0]]);

    assert!(matrix.equal(&matrix, false));
    assert!(matrix.equal(&matrix_clone, false));
    assert!(!matrix.equal(&matrix_different, false));
  
    let vector_literal = LiteralUtil::create_r1::<f64>(
      &vec![1.0, 2.0, 3.0, 4.0]);
    let scalar = LiteralUtil::create_r0::<f64>(1.0);
    let nil = Literal::new_from_shape(&ShapeUtil::make_nil());

    assert!(!matrix.equal(&vector_literal, false));
    assert!(!matrix.equal(&scalar, false));
    assert!(!matrix.equal(&nil, false));
    assert!(nil.equal(&nil, false));
  }

  #[test]
  fn test_token_equality() {
    let token0 = LiteralUtil::create_token::<f64>();
    let token1 = LiteralUtil::create_token::<f64>();
    let scalar = LiteralUtil::create_r0::<f64>(1.0);

    assert!(token0.equal(&token1, false));
    assert!(!token0.equal(&scalar, false));

    let tuple_token0_1 =
      LiteralUtil::make_tuple(&vec![&token0]);
    let tuple_token0_2 =
      LiteralUtil::make_tuple(&vec![&token0]);
    tuple_token0_1.equal(&tuple_token0_2, false);

    let tuple_token0_scalar =
      LiteralUtil::make_tuple(&vec![&token0, &scalar]);
    let tuple_token1_scalar =
      LiteralUtil::make_tuple(&vec![&token1, &scalar]);
    assert!(tuple_token0_scalar.equal(&tuple_token1_scalar, false));

    let tuple_scalar_token1 =
      LiteralUtil::make_tuple(&vec![&scalar, &token1]);
    assert!(!tuple_token0_scalar.equal(&tuple_scalar_token1, false));
  }

  #[test] // FAIL
  fn test_different_layout_equality() {
    // Test equality with literals which have different layouts.
    let mut col_major = Literal::new_from_shape(
      &ShapeUtil::make_shape_with_dense_layout(
        &PrimitiveType::F64,
        &vec![2, 2],
        &vec![0, 1],
        vec![],
        1,
        0,
        0));
    col_major.set_at_root(&vec![0, 0], 1.0);
    col_major.set_at_root(&vec![0, 1], 2.0);
    col_major.set_at_root(&vec![1, 0], 3.0);
    col_major.set_at_root(&vec![1, 1], 4.0);

    let mut row_major = Literal::new_from_shape(
      &ShapeUtil::make_shape_with_dense_layout(
        &PrimitiveType::F64,
        &vec![2, 2],
        &vec![1, 0],
        vec![],
        1,
        0,
        0));
    row_major.set_at_root(&vec![0, 0], 1.0);
    row_major.set_at_root(&vec![0, 1], 2.0);
    row_major.set_at_root(&vec![1, 0], 3.0);
    row_major.set_at_root(&vec![1, 1], 4.0);

    assert!(row_major.equal(&col_major, false));
  }

  #[test]
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
    let tuple2 =
      LiteralUtil::make_tuple(&vec![&scalar_clone, &matrix]);
    assert!(tuple1.equal(&tuple2, false));

    // Tuple with elements reversed.
    let reversed_tuple =
      LiteralUtil::make_tuple(&vec![&matrix, &scalar]);
    assert!(!tuple1.equal(&reversed_tuple, false));

    // Tuple with different value.
    let scalar_42 = LiteralUtil::create_r0::<f64>(42.0);
    let different_tuple = LiteralUtil::make_tuple(
      &vec![&scalar_42, &matrix]);
    assert!(!tuple1.equal(&different_tuple, false));
  }

  #[test] // FAIL
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
    assert!(tuple1.equal(&tuple2, false));
  }

  #[test]
  fn test_c64_equality() {
    let c1 = Complex64::new(1.0, 2.0);
    let c2 = Complex64::new(3.0, 4.0);
    let vec = LiteralUtil::create_r1(&vec![c1, c2]);

    let vec_clone =
      LiteralUtil::create_r1(&vec![c1, c2]);
    assert!(vec.equal(&vec_clone, false));

    let vec_reversed =
      LiteralUtil::create_r1(&vec![c2, c1]);
    assert!(!vec.equal(&vec_reversed, false));
  }

  #[test]
  fn test_is_all_tuple() {
    let elt1 = LiteralUtil::create_r0(0.0);
    let elt2 = LiteralUtil::create_r2(
      &vec![vec![0.0, 0.0], vec![0.0, 0.0]]);
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
    let tuple = LiteralUtil::make_tuple(
      &vec![&scalar, &matrix]);

    let x = Literal::create_from_shape(tuple.shape());
    assert!(tuple.equal(&x, false));
  }

  #[test]
  fn test_is_all_int() {
    let i64_min = i64::MIN;
    let literal = LiteralUtil::create_r0(i64::MIN);
    assert_eq!(literal.is_all_int(i64_min), true);

    let l_42 = LiteralUtil::create_r0(42);
    assert_eq!(l_42.is_all_int(42), true);
    let l_421 = LiteralUtil::create_r0(42.0001);
    assert_eq!(l_421.is_all_float(42.0001), true);

    let l_100 = LiteralUtil::create_r1(&vec![100, 100, 100]);
    assert_eq!(l_100.is_all_int(100), true);

    let l_8 = LiteralUtil::create_r2(
      &vec![vec![8, 8], vec![8, 8]]);
    assert_eq!(l_8.is_all_int(8), true);
    let l_8_9 = LiteralUtil::create_r2(
      &vec![vec![8, 8], vec![8, 9]]);
    assert_eq!(l_8_9.is_all_int(8), false);
    let l_9_8 = LiteralUtil::create_r2(
      &vec![vec![9, 8], vec![8, 8]]);
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

    // TODO
  }

  #[test]
  fn test_count_equal_int() {
    let l1 = LiteralUtil::create_r1::<i64>(&vec![]);
    assert_eq!(l1.count_equal(1), 0);

    let l2 = LiteralUtil::create_r1::<i64>(
      &vec![1, 2, 3, 4, 5, 100]);
    assert_eq!(l2.count_equal(2), 1);

    let l3 = LiteralUtil::create_r1::<i64>(
      &vec![0, 3, 6, 0, 9, 18, 0]);
    assert_eq!(l3.count_equal(0), 3);

    let l4 = LiteralUtil::create_r1::<i64>(
      &vec![234, 345, 4, 45, 5467, 5467, 5467]);
    assert_eq!(l4.count_equal(5467), 3);
  }

  #[test]
  fn test_count_equal_float() {
    let l1 = LiteralUtil::create_r1::<f64>(&vec![]);
    assert_eq!(l1.count_equal(0.0), 0);

    let l2 = LiteralUtil::create_r1(
      &vec![1.1, 2.2, 3.3, 4.4, 5.5, 100.6]);
    assert_eq!(l2.count_equal(3.3), 1);

    let l3 = LiteralUtil::create_r1(
      &vec![7.62, 3.0, 7.75, 7.62, 7.3, 2.0, 7.62]);
    assert_eq!(l3.count_equal(7.62), 3);

    let l4 = LiteralUtil::create_r1(
      &vec![f64::NAN, 0.0, 6.8, f64::NAN, f64::NAN, f64::NAN, 63.12, 24.6, f64::NAN]);
    assert_eq!(l4.count_equal(f64::NAN), 5);
  }

  #[test]
  fn test_count_equal_bool() {
    let l1 = LiteralUtil::create_r1(&vec![false, true]);
    assert_eq!(l1.count_equal(false), 1);
  }

  #[test]
  fn test_count_equal_complex() {
    let c1 = Complex64::new(1.0, 2.0);
    let c2 = Complex64::new(3.0, 4.0);
    let c3 = Complex64::new(5.0, 6.0);
    let c4 = Complex64::new(6.0, 7.0);

    let l1 = LiteralUtil::create_r1(&vec![c1, c2, c3, c4]);
    assert_eq!(l1.count_equal(c3), 1);
  }

  #[test]
  fn test_is_zero() {
    let scalar_zero = LiteralUtil::create_r0(0.0);
    let scalar_one = LiteralUtil::create_r0(1.0);
    assert_eq!(scalar_zero.is_zero(&vec![]), true);
    assert_eq!(scalar_one.is_zero(&vec![]), false);

    let array = LiteralUtil::create_r2(
      &vec![vec![1, 2, 0, 3], vec![1, 0, 1, 2]]);
    assert_eq!(array.is_zero(&vec![0, 1]), false);
    assert_eq!(array.is_zero(&vec![0, 2]), true);
    assert_eq!(array.is_zero(&vec![1, 1]), true);
    assert_eq!(array.is_zero(&vec![1, 2]), false);

    let c0 = Complex64::new(0.0, 0.0);
    let l_c_0 = LiteralUtil::create_r0(c0);
    let c_non_0 = Complex64::new(0.5, 0.0);
    let l_c_non_0 = LiteralUtil::create_r0(c_non_0);
    assert_eq!(l_c_0.is_zero(&vec![]), true);
    assert_eq!(l_c_non_0.is_zero(&vec![]), false);
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
  fn test_replicate_r2_u32() {
    
  }

  #[test]
  fn test_get_as_double() {
    let m = LiteralUtil::create_r2(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    assert_eq!(m.get_as_double(&vec![0, 0]), Some(1.0));
    assert_eq!(m.get_as_double(&vec![1, 0]), Some(3.0));
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
    assert_eq!(c1.get_as_complex_64(&vec![]), Some(value));

    let c2 = LiteralUtil::create_r0(1.0);
    assert_eq!(c2.get_as_complex_64(&vec![]), Some(value));

    let other_value = Complex64::new(1.0, 2.0);
    let c5 = LiteralUtil::create_r0(other_value);
    assert_eq!(c5.get_as_complex_64(&vec![]), Some(other_value));

    let value_1 = Complex64::new(1.0, 0.0);
    let c6 = LiteralUtil::create_r0(1);
    assert_eq!(c6.get_as_complex_64(&vec![]), Some(value_1));
  }

  #[test] // FAIL
  fn test_slice_on_bool() {
    let c1 = LiteralUtil::create_r1(&vec![true, true, false]);
    let slice_c1 = c1.slice(&vec![0], &vec![3]); 
    assert!(c1.equal(&slice_c1, false));
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
    let c1 = Literal::create_from_shape_with_unknown_leaf_arrays(
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![4, 4]));
    assert_eq!(c1.is_known(&vec![]), false);
  }

  #[test]
  fn test_create_from_shape_with_unknown_leaf_arrays_s4_tuple() {
    let mut inner_shape = ShapeUtil::make_shape(
      &PrimitiveType::S4, vec![4, 4]);
    inner_shape.mutable_layout().as_mut().unwrap().set_element_size_in_bits(4);

    let c1 =
      Literal::create_from_shape_with_unknown_leaf_arrays(&inner_shape);
    assert_eq!(c1.is_known(&vec![]), false);
  }

  #[test]
  fn test_create_partially_known_tuple() {
    let c1 = Literal::create_from_shape_with_unknown_leaf_arrays(
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![4, 4]));
    let c2 = LiteralUtil::create_r0(10);
    let c3 = LiteralUtil::make_tuple(&vec![&c1, &c2]);
    let c4 = LiteralUtil::create_r0(100);
    let c5 = LiteralUtil::make_tuple(&vec![&c4, &c3]);
    assert_eq!(c5.is_known(&vec![]), false);
  }

  #[test]
  fn test_copy_from_partially_known_tuple() {
    let c1 = Literal::create_from_shape_with_unknown_leaf_arrays(
      &ShapeUtil::make_shape(&PrimitiveType::F64, vec![4, 4]));
    let c2 = LiteralUtil::create_r0(10);
    let c3 = LiteralUtil::make_tuple(&vec![&c1, &c2]);
    let c4 = LiteralUtil::create_r0(100);
    let c5 = LiteralUtil::make_tuple(&vec![&c4, &c3]);
    let mut c6 = Literal::create_from_shape(c5.shape());
    let result = c6.copy_from(
      &c5, &vec![1], &vec![1], false);
    assert!(result.is_ok());
    assert!(!c6.is_known(&vec![]));
  }

  #[test]
  fn test_copy_from_partially_known_tuple_unknown_tuple_element() {
    let c1 = Literal::create_from_shape_with_unknown_leaf_arrays(
      &ShapeUtil::make_tuple_shape(vec![
        ShapeUtil::make_shape(&PrimitiveType::F64, vec![4, 4]),
        ShapeUtil::make_shape(&PrimitiveType::F64, vec![4, 4])]));
    let c2 = LiteralUtil::create_r0(10);
    let c3 = LiteralUtil::make_tuple(&vec![&c1, &c2]);
    let c4 = LiteralUtil::create_r0(100);
    let c5 = LiteralUtil::make_tuple(&vec![&c4, &c3]);
    let mut c6 = Literal::create_from_shape(c5.shape());
    let mut c1_copy = Literal::create_from_shape(c1.shape());
    let mut c2_copy = Literal::create_from_shape(c2.shape());

    let mut result = c6.copy_from(
      &c5, &vec![1], &vec![1], false);
    assert!(result.is_ok());
    result = c1_copy.copy_from(
      &c6, &vec![], &vec![1, 0], false);
    assert!(result.is_ok());
    result = c2_copy.copy_from(
      &c6, &vec![], &vec![1, 1], false);
    assert!(result.is_ok());

    assert!(!c6.is_known(&vec![]));
    assert!(!c1_copy.is_known(&vec![]));
    assert!(c2_copy.is_known(&vec![]));
  }

    #[test]
  fn test_populate_r1_i64() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::S64, vec![1]);
    let mut output = Literal::new_from_shape(&shape);
    let vec: Vec<i64> = vec![77];
    output.populate_r1(&vec);

    let expected = LiteralUtil::create_r1(&vec);
    assert!(output.equal(&expected, false));
  }

  #[test]
  fn test_populate_r1_c64() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::C64, vec![1]);
    let mut output = Literal::new_from_shape(&shape);
    let c = Complex64::new(77.0, 88.0);
    output.populate_r1(&vec![c]);

    let expected = LiteralUtil::create_r1(&vec![c]);
    assert!(output.equal(&expected, false))
  }

  #[test]
  fn test_populate_r2_c64() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::C64, vec![2, 2]);
    let mut output = Literal::new_from_shape(&shape);
    let c1 = Complex64::new(7.0, 8.0);
    let c2 = Complex64::new(9.0, 10.0);
    let c3 = Complex64::new(1.0, 2.0);
    let c4 = Complex64::new(3.0, 4.0);
    output.populate_r2(&vec![vec![c1, c2], vec![c3, c4]]);

    let expected =
      LiteralUtil::create_r2(&vec![vec![c1, c2], vec![c3, c4]]);
    assert!(output.equal(&expected, false));
  }

  #[test]
  fn test_populate_with_value_r0_float() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::F64, vec![]);
    let mut output = Literal::new_from_shape(&shape);
    output.populate_with_value(0.25);

    let expected = LiteralUtil::create_r0(0.25);
    assert!(output.equal(&expected, false));
  }

  #[test]
  fn test_populate_with_value_r1_float() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::F64, vec![3]);
    let mut output = Literal::new_from_shape(&shape);
    output.populate_with_value(0.5);

    let expecteed =
      LiteralUtil::create_r1(&vec![0.5, 0.5, 0.5]);
    assert!(output.equal(&expecteed, false));
  }

  #[test]
  fn test_populate_with_value_r2_float() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::F64, vec![2, 2]);
    let mut output = Literal::new_from_shape(&shape);
    output.populate_with_value(2.0);

    let expected = LiteralUtil::create_r2(
      &vec![vec![2.0, 2.0], vec![2.0, 2.0]]);
    assert!(output.equal(&expected, false));
  }

  #[test]
  fn test_populate_with_value_r1_s64() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::S64, vec![3]);
    let mut output = Literal::new_from_shape(&shape);
    output.populate_with_value(-7 as i64);

    let vec: Vec<i64> = vec![-7, -7, -7];
    let expecteed =
      LiteralUtil::create_r1(&vec);
    assert!(output.equal(&expecteed, false));
  }

  #[test]
  fn test_copy_from_scalars() {
    let mut zero = LiteralUtil::create_r0(0);
    let nine = LiteralUtil::create_r0(9);
    let result = zero.copy_from(
      &nine, &vec![], &vec![], false);
    assert!(result.is_ok());
    assert!(zero.equal(&nine, false));

    // TODO
  }

  #[test]
  fn test_copy_from_nil_shape() {
    let mut nil_literal_0 =
      Literal::create_from_shape(&ShapeUtil::make_nil());
    let nil_literal_1 =
      Literal::create_from_shape(&ShapeUtil::make_nil());
    // This doesn't actually do any copying, but it should succeed.
    let result = nil_literal_0.copy_from(
      &nil_literal_1, &vec![], &vec![], false);
    assert!(result.is_ok());
  }

  #[test]
  fn test_copy_from_arrays() {
    let mut scalar_42 = LiteralUtil::create_r0(42.0);
    let scalar_123 = LiteralUtil::create_r0(123.0);
    assert!(!scalar_42.equal(&scalar_123, false));

    let result = scalar_42.copy_from(
      &scalar_123, &vec![], &vec![], false);
    assert!(result.is_ok());
    assert!(scalar_42.equal(&scalar_123, false));
    assert_eq!(scalar_42.get::<f64>(&vec![], &vec![]).unwrap(), &123.0);

    let mut matrix_1234 =
      LiteralUtil::create_r2(&vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matric_5678 =
      LiteralUtil::create_r2(&vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    assert!(!matrix_1234.equal(&matric_5678, false));
    assert_eq!(matrix_1234.get::<f64>(&vec![0, 0], &vec![]).unwrap(), &1.0);

    let result = matrix_1234.copy_from(
      &matric_5678, &vec![], &vec![], false);
    assert!(result.is_ok());
    assert!(matrix_1234.equal(&matric_5678, false));
    assert_eq!(matrix_1234.get::<f64>(&vec![0, 0], &vec![]).unwrap(), &5.0);
  }

  #[test]
  fn test_copy_from_tuples() {
    let matrix =
      LiteralUtil::create_r2(&vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let nil_literal = Literal::create_from_shape(&ShapeUtil::make_nil());
    let inner_elements = vec![
      LiteralUtil::create_r0(42),
      LiteralUtil::create_r1(&vec![23.0, 44.0]),
    ];
    let inner_tuple = LiteralUtil::make_tuple(
      &vec![&inner_elements[0], &inner_elements[1], &nil_literal]);
    let mut nested_tuple =
      LiteralUtil::make_tuple(&vec![&matrix, &inner_tuple]);
    
    // Create a tuple the same shape as the inner tuple of nested_tuple but with
    // different values..
    let i32_minus5 = LiteralUtil::create_r0(-5);
    let double_2_4 = LiteralUtil::create_r1(&vec![2.0, 4.0]);
    let tuple = LiteralUtil::make_tuple(
      &vec![&i32_minus5, &double_2_4, &nil_literal]);

    assert_eq!(nested_tuple.get::<i32>(&vec![], &vec![1, 0]).unwrap(), &42);
    assert_eq!(nested_tuple.get::<f64>(&vec![0], &vec![1, 1]).unwrap(), &23.0);
    assert_eq!(nested_tuple.get::<f64>(&vec![1], &vec![1, 1]).unwrap(), &44.0);

    // Overwrite the inner tuple element of nested_tuple with the contents of
    // 'tuple'.
    let result = nested_tuple.copy_from(
      &tuple, &vec![1], &vec![], false);
    assert!(result.is_ok());

    assert_eq!(nested_tuple.get::<i32>(&vec![], &vec![1, 0]).unwrap(), &-5);
    assert_eq!(nested_tuple.get::<f64>(&vec![0], &vec![1, 1]).unwrap(), &2.0);
    assert_eq!(nested_tuple.get::<f64>(&vec![1], &vec![1, 1]).unwrap(), &4.0);
  }

  #[test]
  fn test_copy_between_same_tuple() {
    let elements = vec![
      LiteralUtil::create_r0(-2), LiteralUtil::create_r0(4)];
    let mut tuple =
      LiteralUtil::make_tuple(&vec![&elements[0], &elements[1]]);
    assert_eq!(tuple.get::<i32>(&vec![], &vec![0]).unwrap(), &-2);
    assert_eq!(tuple.get::<i32>(&vec![], &vec![1]).unwrap(), &4);

    // Copy from one element to the other.
    let tuple_clone = 
      LiteralUtil::make_tuple(&vec![&elements[0], &elements[1]]);
    let result = tuple.copy_from(
      &tuple_clone, &vec![1], &vec![0], false);
    assert!(result.is_ok());
    assert_eq!(tuple.get::<i32>(&vec![], &vec![0]).unwrap(), &-2);
    assert_eq!(tuple.get::<i32>(&vec![], &vec![1]).unwrap(), &-2);
  }

  #[test]
  fn test_copy_from_different_shapes() {
    let mut matrix =
      LiteralUtil::create_r2(&vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let vector = LiteralUtil::create_r1(&vec![5.0, 7.0]);
    let result = matrix.copy_from(
      &vector, &vec![], &vec![], false);
    assert!(result.is_err());
    assert!(result.err().unwrap().contains(
      "Destination subshape incompatible with source subshape"));
  }

  #[test]
  fn test_literal_move() {
    let matrix =
      LiteralUtil::create_r2(&vec![vec![1.0, 2.0], vec![3.0, 4.0]]);   
    let literal = matrix;
    assert!(ShapeEqual::new().equal(
      &ShapeUtil::make_shape(&PrimitiveType::F64, vec![2, 2]), literal.shape()));
    assert_eq!(literal.get::<f64>(&vec![0, 0], &vec![]).unwrap(), &1.0);
    assert_eq!(literal.get::<f64>(&vec![0, 1], &vec![]).unwrap(), &2.0);
    assert_eq!(literal.get::<f64>(&vec![1, 0], &vec![]).unwrap(), &3.0);
    assert_eq!(literal.get::<f64>(&vec![1, 1], &vec![]).unwrap(), &4.0);
  }

  #[test]
  fn test_decompose_tuple() {
    let nil_literal = Literal::new_from_shape(&ShapeUtil::make_nil());
    let inner_elements = vec![
      LiteralUtil::create_r0(42),
      LiteralUtil::create_r1(&vec![23.0, 44.0])
    ];
    let tuple_elements = vec![
      LiteralUtil::create_r2(&vec![vec![1, 2], vec![3, 4]]),
      LiteralUtil::make_tuple(&vec![&inner_elements[0], &inner_elements[1], &nil_literal])
    ];
    let mut nested_tuple = LiteralUtil::make_tuple(
      &vec![&tuple_elements[0], &tuple_elements[1], &nil_literal]);

    assert!(!ShapeUtil::is_empty_tuple(nested_tuple.shape()));
    let elements = nested_tuple.decompose_tuple();
    assert!(ShapeUtil::is_empty_tuple(nested_tuple.shape()));
    assert_eq!(elements.len(), 3);

    assert!(ShapeUtil::compatible(elements[0].shape(),
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![2, 2])));
    assert_eq!(elements[0].get::<i32>(&vec![0, 0], &vec![]).unwrap(), &1);
    assert_eq!(elements[0].get::<i32>(&vec![0, 1], &vec![]).unwrap(), &2);
    assert_eq!(elements[0].get::<i32>(&vec![1, 0], &vec![]).unwrap(), &3);
    assert_eq!(elements[0].get::<i32>(&vec![1, 1], &vec![]).unwrap(), &4);

    assert!(ShapeUtil::compatible(elements[1].shape(),
      &ShapeUtil::make_tuple_shape(vec![
        ShapeUtil::make_shape(&PrimitiveType::S32, vec![]),
        ShapeUtil::make_shape(&PrimitiveType::F64, vec![2]),
        ShapeUtil::make_nil()])));
    assert_eq!(elements[1].get::<i32>(&vec![], &vec![0]).unwrap(), &42);
    assert_eq!(elements[1].get::<f64>(&vec![0], &vec![1]).unwrap(), &23.0);
    assert_eq!(elements[1].get::<f64>(&vec![1], &vec![1]).unwrap(), &44.0);

    assert!(ShapeUtil::compatible(elements[2].shape(), &ShapeUtil::make_nil()));
  }

  #[test]
  fn test_decompose_empty_tuple() {
    let mut nil_literal = Literal::new_from_shape(&ShapeUtil::make_nil());
    let elements = nil_literal.decompose_tuple();
    assert_eq!(elements.len(), 0);
  }

  #[test]
  fn test_populate_r1_dynamic() {
    let mut literal = Literal::new_from_shape(
      &ShapeUtil::make_shape(&PrimitiveType::U32, vec![20]));
    literal.set_dynamic_size(0, &vec![], 10);
    let values: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    literal.populate_r1(&values);

    let expected =
      "u32[<=20](10) {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}".to_string();
    assert_eq!(literal.to_string(), expected);
  }

  #[test]
  fn test_populate_r2_dynamic_dim0() {
    let mut literal = Literal::new_from_shape(
      &ShapeUtil::make_shape(&PrimitiveType::U32, vec![5, 2]));
    literal.set_dynamic_size(0, &vec![], 3);
    let values: Vec<Vec<u32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    literal.populate_r2(&values);

    let expected = "u32[<=5,2](3,2) {
 { 1, 2 },
 { 3, 4 },
 { 5, 6 }
}".to_string();
    assert_eq!(literal.to_string(), expected);
  }

  #[test] // FAIL
  fn test_populate_r2_dynamic_dim1() {
    let mut literal = Literal::new_from_shape(
      &ShapeUtil::make_shape(&PrimitiveType::U32, vec![2, 5]));
    literal.set_dynamic_size(1, &vec![], 3);
    let values: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    literal.populate_r2(&values);

    let expected = "u32[2,<=5](2,3) {
 { 1, 2, 3 },
 { 4, 5, 6 }
}".to_string();
    assert_eq!(literal.to_string(), expected);
  }

  #[test]
  fn test_move_into_tuple() {
    let mut elements = vec![];
    elements.push(LiteralUtil::create_r0(1.0));
    elements.push(LiteralUtil::create_r1(&vec![4, 8]));

    let mut inner_elements = vec![];
    inner_elements.push(LiteralUtil::create_r0(42));
    inner_elements.push(LiteralUtil::create_r1(&vec![23.0, 44.0]));
    elements.push(LiteralUtil::make_tuple(
      &vec![&inner_elements[0], &inner_elements[1]]));

    let literal = Literal::move_into_tuple(&mut elements);
    assert!(literal.shape().is_tuple());
    assert_eq!(ShapeUtil::tuple_element_count(literal.shape()), 3);

    assert_eq!(literal.get::<f64>(&vec![], &vec![0]).unwrap(), &1.0);
    assert_eq!(literal.get::<i32>(&vec![0], &vec![1]).unwrap(), &4);
    assert_eq!(literal.get::<i32>(&vec![1], &vec![1]).unwrap(), &8);
    assert_eq!(literal.get::<i32>(&vec![], &vec![2, 0]).unwrap(), &42);
    assert_eq!(literal.get::<f64>(&vec![0], &vec![2, 1]).unwrap(), &23.0);
    assert_eq!(literal.get::<f64>(&vec![1], &vec![2, 1]).unwrap(), &44.0);

    for elt in &elements {
      assert!(ShapeUtil::is_empty_tuple(elt.shape()));
    }
  }

  #[test]
  fn test_move_into_empty_tuple() {
    let mut empty_literals: Vec<Literal> = vec![];
    let literal = Literal::move_into_tuple(&mut empty_literals);
    assert!(literal.shape().is_tuple());
    assert_eq!(ShapeUtil::tuple_element_count(literal.shape()), 0);
  }

  #[test]
  fn test_literal_move_assignment() {
    let mut literal = Literal::default();
    assert!(ShapeEqual::new().equal(&ShapeUtil::make_nil(), literal.shape()));

    let matrix = LiteralUtil::create_r2(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    literal = matrix;

    assert!(ShapeEqual::new().equal(
      &ShapeUtil::make_shape(&PrimitiveType::F64, vec![2, 2]), literal.shape()));
    assert_eq!(literal.get::<f64>(&vec![0, 0], &vec![]).unwrap(), &1.0);
    assert_eq!(literal.get::<f64>(&vec![0, 1], &vec![]).unwrap(), &2.0);
    assert_eq!(literal.get::<f64>(&vec![1, 0], &vec![]).unwrap(), &3.0);
    assert_eq!(literal.get::<f64>(&vec![1, 1], &vec![]).unwrap(), &4.0);
  }

  #[test]
  fn test_get_set_tuple() {
    let r0 = LiteralUtil::create_r0(42.0);
    let r2 = LiteralUtil::create_r2(
      &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let mut tuple = LiteralUtil::make_tuple(&vec![&r0, &r2]);

    assert_eq!(tuple.get::<f64>(&vec![], &vec![0]).unwrap(), &42.0);
    tuple.set::<f64>(&vec![], &vec![0], -5.0);
    assert_eq!(tuple.get::<f64>(&vec![], &vec![0]).unwrap(), &-5.0);

    assert_eq!(tuple.get::<f64>(&vec![1, 0], &vec![1]).unwrap(), &3.0);
    tuple.set::<f64>(&vec![1, 0], &vec![1], -4.0);
    assert_eq!(tuple.get::<f64>(&vec![1, 0], &vec![1]).unwrap(), &-4.0);
  }

  #[test] // FAIL
  fn test_create_from_shape_zero_initialized() {
    // Literals constructed using CreateFromShape should be zero initialized.
    let scalar_f64 = Literal::new_from_shape(
      &ShapeUtil::make_shape(&PrimitiveType::F64, vec![]));
    assert_eq!(scalar_f64.get::<f64>(&vec![], &vec![]).unwrap(), &0.0);
  }

/*
  #[test]
  fn test_broadcast_vector_to_matrix_0() {
    let literal = LiteralUtil::create_r1(&vec![1, 2]);
    let broadcast_literal = 
      literal.broadcast(&ShapeUtil::make_shape(
        &PrimitiveType::S64, vec![2, 2]), &vec![0]);
    assert!(broadcast_literal.is_ok());
    assert!(broadcast_literal.unwrap().equal(
      &LiteralUtil::create_r2(&vec![vec![1, 1], vec![2, 2]]), false));
  }
*/

}