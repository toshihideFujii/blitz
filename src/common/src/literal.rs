#![allow(dead_code)]

use std::mem::size_of;

use crate::{
  blitz_data::PrimitiveType,
  layout_util::LayoutUtil,
  primitive_util,
  printer::Printer,
  shape::Shape,
  shape_util::ShapeUtil
};

fn scalar_shape(t: &PrimitiveType) -> Shape {
  primitive_util::array_type_switch(|t: &PrimitiveType| -> Shape {
    assert!(primitive_util::is_array_type(t));
    let mut shape = Shape::new_from_type(t);
    shape.mutable_layout();
    shape
  }, t)
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

pub struct LiteralBase {
  root_piece: Piece,
}

impl LiteralBase {
  pub fn new() -> Self {
    LiteralBase {
      root_piece: Piece::new(),
    }
  }

  // Returns the shape of the literal.
  pub fn shape(&self) -> &Shape {
    unimplemented!();
  }

  // Returns a vec of the array for this literal for the given T
  // (e.g., float).
  pub fn data<T>(&self, shape_index: usize) -> &Vec<T> {
    self.piece(shape_index).data()
  }

  // Returns a const pointer to (or size of) the underlying buffer holding the
  // array at the given shape index.
  pub fn untyped_data(&self, shape_index: usize) -> &Vec<u8> {
    self.piece(shape_index).untyped_data()
  }

  pub fn size_bytes(&self, shape_index: usize) -> i64 {
    self.piece(shape_index).size_bytes_dense()
  }

  // Returns this literal's data as a string. This literal must be a rank-1 u8 array.
  pub fn get_r1_u8_as_string(&self) -> String {
    assert!(self.shape().is_array());
    assert_eq!(self.shape().rank(), 1);
    assert_eq!(self.shape().element_type(), PrimitiveType::U8);
    ShapeUtil::elements_in(self.shape()).to_string() // TODO
  }

  pub fn print() {}
  pub fn print_oneline() {}
  pub fn print_without_shape() {}
  pub fn print_without_shape_oneline() {}
  pub fn print_with_layout() {}
  pub fn print_with_layout_oneline() {}
  pub fn to_string() {}
  pub fn to_string_oneline() {}
  pub fn to_string_without_shape() {}
  pub fn to_string_without_shape_oneline() {}
  pub fn to_string_with_layout() {}
  pub fn to_string_with_layout_oneline() {}
  pub fn get() {}

  // Get the dynamic size on dim_index in the literal at the given shape_index.
  pub fn get_dynamic_size(&self, dim_index: usize, shape_index: usize) -> i32 {
    self.piece(shape_index).get_dynamic_size(dim_index)
  }

  pub fn get_first_element() {}
  pub fn get_first_integer() {}
  pub fn get_as_string() {}
  pub fn is_equal_at() {}
  pub fn get_integral_as_s64() {}
  pub fn get_as_double() {}
  pub fn get_as_complex_128() {}
  pub fn get_sum_as_double() {}
  pub fn each_cell_as_string() {}
  pub fn each_cell() {}

  // Checks whether all of this literal's values are equal to the given
  // scalar literal.
  pub fn is_all(&self, scalar: &Literal) -> bool {
    self.root_piece().is_all(scalar)
  }

  pub fn is_all_float() {}
  pub fn is_all_complex() {}

  // Determines if this literal consists of the first element og the literal.
  pub fn is_all_first(&self) -> bool {
    false
  }

  pub fn count_equal() {}
  pub fn is_r1_iota() {}
  pub fn is_r1_strided_iota() {}
  pub fn is_zero() {}

  // Returns the count of the elements in the array at the given shape index
  // in this literal.
  pub fn element_count(&self, index_vec: &Vec<i64>) -> i64 {
    if index_vec.is_empty() {
      return ShapeUtil::elements_in(self.shape());
    }
    ShapeUtil::elements_in(
      &ShapeUtil::get_subshape(self.shape(), index_vec))
  }

  pub fn convert_to_shape() {}
  pub fn bitcast_convert() {}
  pub fn convert() {}
  pub fn relay_out() {}
  pub fn to_static() {}
  pub fn to_bounded_dynamic() {}
  pub fn reshape() {}
  pub fn broadcast() {}
  pub fn transpose() {}
  pub fn slice() {}
  pub fn replicate() {}

  // Returns true if the leaf arrays of the literal within the given shape_index
  // are all determined.
  pub fn is_determined(&self, shape_index: usize) -> bool {
    self.piece(shape_index).is_determined()
  }

  // Returns true if the leaf arrays of the literal within the given shape_index
  // are all known.
  pub fn is_known(&self, shape_index: usize) -> bool {
    self.piece(shape_index).is_known()
  }

  // Creates a new literal object with the shape specified as parameter.
  // The content of the literal values is the default value of the primitive
  // type of literal itself (0 for numeric types, and false for predicates).
  pub fn create_from_shape(shape: &Shape) -> Literal {
    let mut literal = Literal::new_from_shape(shape);
    literal.mutable_root_piece().for_each_mutable_subpiece(
      &mut |_index: usize, piece: &mut Piece| -> Result<(), String> {
        if piece.subshape().is_array() {
          let len = piece.size_bytes_dense() as usize;
          let untyped_data = piece.mutable_untyped_data();
          for i in 0..len {
            untyped_data[i] = 0;
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

  fn piece(&self, shape_index: usize) -> &Piece {
    let mut piece = self.root_piece();
    assert!(shape_index < piece.children_size());
    for i in 0..shape_index {
      piece = piece.child(i).unwrap();
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
    _shape_index: usize,
    _print_shape: bool,
    _print_layout: bool,
    _one_line: bool,
    _printer: &mut dyn Printer)
  {
      
  }

  fn dense_array_print_helper(
    &self,
    _shape_index: usize,
    _print_shape: bool,
    _print_layout: bool,
    _one_line: bool,
    _printer: &mut dyn Printer)
  {
    
  }

  fn print_helper(
    &self,
    shape_index: usize,
    print_shape: bool,
    print_layout: bool,
    one_line: bool,
    printer: &mut dyn Printer)
  {
    let subshape =
      ShapeUtil::get_subshape(self.shape(), &vec![shape_index as i64]);
    assert!(LayoutUtil::has_layout(self.shape()));
    assert!(LayoutUtil::has_layout(&subshape));

    if subshape.is_tuple() {
      self.tuple_print_helper(shape_index, print_shape,
        print_layout, one_line, printer);
    } else if subshape.is_token() {
      printer.append(&"token".to_string());
    } else {
      assert!(LayoutUtil::is_dense_array(&subshape));
      if self.is_known(shape_index) {
        self.dense_array_print_helper(shape_index, print_shape,
          print_layout, one_line, printer);
      } else {
        LiteralBase::print_shape(print_layout, &subshape, printer);
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

pub struct Literal {
  base: LiteralBase,
  shape: Shape,
}

impl Literal {
  pub fn new_from_shape(_shape: &Shape) -> Self {
    Literal {
      base: LiteralBase::new(),
      shape: Shape::new(),
    }
  }

  pub fn new(
    shape: &Shape,
    allocate_arrays: bool,
    leaf_array_value_state: ArrayValueState) -> Self
  {
    let mut literal = Literal::new_from_shape(shape);
    let interned_shape = try_intern_shape(shape);
    if interned_shape.is_some() {
      literal.shape = interned_shape.unwrap();
    }
    assert!(leaf_array_value_state != ArrayValueState::Known ||
      LayoutUtil::has_layout(literal.shape()));
    assert!(!LayoutUtil::has_custom_element_size_in_bits(shape),
      "Literal does not support layouts with custom bit size.");

    let mut root_piece = Piece::new();
    root_piece.set_subshape(shape.clone());
    Literal::set_piece(&literal.shape, &mut root_piece,
      allocate_arrays, leaf_array_value_state);

    literal.base.root_piece.set_subshape(shape.clone());
    literal
  }

  pub fn shape(&self) -> &Shape {
    self.base.shape()
  }

  pub fn root_piece(&self) -> &Piece {
    self.base.root_piece()
  }

  pub fn mutable_root_piece(&mut self) -> &mut Piece {
    self.base.mutable_root_piece()
  }

  fn set_piece(
    shape: &Shape,
    piece: &mut Piece,
    allocate_arrays: bool,
    leaf_array_value_state: ArrayValueState)
  {
    if shape.is_tuple() {
      for subshape in shape.tuple_shapes_vec() {
        let mut child_piece = Piece::new();
        child_piece.set_subshape(subshape.clone());
        Literal::set_piece(subshape, &mut child_piece, allocate_arrays,
          leaf_array_value_state.clone());
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayValueState {
  Known,
  Unknown,
  Undetermined,
}

pub struct Piece {
  subshape: Shape,
  children: Option<Vec<Piece>>,
  data: Vec<u8>,
  array_value_state: ArrayValueState,
}

impl Piece {
  pub fn new() -> Self {
    Piece {
      subshape: Shape::new(),
      children: None,
      data: vec![],
      array_value_state: ArrayValueState::Undetermined
    }
  }

  pub fn get_array_value_state(&self) -> ArrayValueState {
    self.array_value_state.clone()
  }

  pub fn set_array_value_state(&mut self, state: ArrayValueState) {
    self.array_value_state = state;
  }

  // Returns the buffer holding the array data for this piece as an array
  // slice. This piece must be array-shaped.
  pub fn data<T>(&self) -> &Vec<T> {
    unimplemented!();
  }

  // Returns the buffer holding the array data for this piece as an array
  // slice. This piece must be array-shaped.
  pub fn untyped_data(&self) -> &Vec<u8> {
    assert!(LayoutUtil::is_dense_array(self.subshape()));
    self.buffer()
  }

  pub fn mutable_untyped_data(&mut self) -> &mut Vec<u8> {
    assert!(LayoutUtil::is_dense_array(self.subshape()));
    self.mutable_buffer()
  }

  pub fn get() {}
  pub fn set() {}

  pub fn get_dynamic_size(&self, dim_index: usize) -> i32 {
    assert!(LayoutUtil::is_dense_array(self.subshape()));
    if !self.subshape.is_dynamic_dimension(dim_index) {
      return self.subshape.dimensions(dim_index) as i32;
    }
    self.dynamic_size_buffer()[dim_index]
  }

  pub fn set_dynamic_size(&mut self, dim_index: usize, size: i32) {
    assert!(LayoutUtil::is_dense_array(self.subshape()));
    assert!(self.subshape.is_dynamic_dimension(dim_index));
    self.dynamic_size_buffer()[dim_index] = size;
  }

  pub fn allocate_buffers() {}
  pub fn deallocate_buffers() {}

  // Gets the buffer holding the array data.
  pub fn buffer(&self) -> &Vec<u8> {
    &self.data
  }

  pub fn mutable_buffer(&mut self) -> &mut Vec<u8> {
    &mut self.data
  }

  pub fn set_buffer() {}
  pub fn move_data_from() {}

  pub fn dynamic_size_buffer(&self) -> Vec<i32> { vec![] }

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

  pub fn child(&self, index: usize) -> Option<&Piece> {
    assert!(self.children.is_some());
    self.children.as_ref().unwrap().get(index)
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

  // Visitor functions that resursively traverses the piece and calls the
  // given function at each child piece.
  pub fn for_each_subpiece<T>(&self, func: &mut T)
    where T: FnMut(usize, &Piece) -> Result<(), String>
  {
    let _ = Piece::for_each_helper(func, self, 0);
  }

  pub fn for_each_mutable_subpiece<T>(&mut self, func: &mut T)
    where T: FnMut(usize, &mut Piece) -> Result<(), String>
  {
    let _ = Piece::for_each_mutable_helper(func, self, 0);
  }

  pub fn for_each_subpiece_with_status() {}
  pub fn for_each_subpiece_with_bool() {}

  // Checks whether all elements of this Piece are equal to the given literal.
  // Returns false if this Piece is not an array.
  pub fn is_all(&self, scalar: &Literal) -> bool {
    assert!(ShapeUtil::is_scalar(scalar.shape()));
    if !self.subshape.is_array() { return false; }

    assert!(LayoutUtil::is_dense_array(self.subshape()));
    assert_eq!(self.subshape.element_type(), scalar.shape().element_type());

    primitive_util::array_type_switch(|_t: &PrimitiveType| -> bool {
      // TODO
      true
    }, &self.subshape.element_type())
  }

  pub fn count_all() {}
  pub fn equal_elements() {}
  pub fn equal_dynamic_size() {}
  pub fn copy_from() {}

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

  fn for_each_helper<T>(func: &mut T, piece: &Piece, index: usize) -> Result<(), String>
    where T: FnMut(usize, &Piece) -> Result<(), String>
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

  fn for_each_mutable_helper<T>(
    func: &mut T,
    piece: &mut Piece,
    index: usize) -> Result<(), String>
      where T: FnMut(usize, &mut Piece) -> Result<(), String>
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