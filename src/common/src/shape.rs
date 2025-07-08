#![allow(dead_code)]

use crate::{
  util::DimensionVector,
  layout::{Layout, LayoutEqual},
  blitz_data::PrimitiveType,
  primitive_util,
  layout_util::LayoutUtil,
  shape_util::ShapeUtil,
  printer::Printer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shape {
  element_type: PrimitiveType,
  dimensions: DimensionVector,
  dynamic_dimensions: Vec<bool>,
  tuple_shapes: Vec<Shape>,
  layout: Option<Layout>
}

impl Shape {
  pub const UNBOUNDED_SIZE: i64 = i64::MIN;

  pub fn new() -> Self {
    Shape {
      element_type: PrimitiveType::Invalid,
      dimensions: Vec::new(),
      dynamic_dimensions: Vec::new(),
      tuple_shapes: Vec::new(),
      layout: None,
    }
  }

  pub fn new_from(shape: &Shape) -> Self {
    let mut dimensions = Vec::new();
    dimensions.clone_from_slice(&shape.dimensions);
    let mut dynamic_dimensions = Vec::new();
    dynamic_dimensions.clone_from_slice(&shape.dynamic_dimensions);
    let mut tuple_shapes = Vec::new();
    tuple_shapes.clone_from_slice(&shape.tuple_shapes);
    Shape {
      element_type: shape.element_type.clone(),
      dimensions: dimensions,
      dynamic_dimensions: dynamic_dimensions,
      tuple_shapes: tuple_shapes,
      layout: shape.layout.clone(),
    }
  }

  pub fn new_from_type(t: &PrimitiveType) -> Self {
    Shape {
      element_type: t.clone(),
      dimensions: Vec::new(),
      dynamic_dimensions: Vec::new(),
      tuple_shapes: Vec::new(),
      layout: None,
    }
  }

  // Returns a human-readable string that represents the given shape, with or
  // without layout.
  pub fn print(&self, printer: &mut dyn Printer, print_layout: bool) {
    if print_layout {
      ShapeUtil::print_human_string_with_layout(printer, self);
    } else {
      ShapeUtil::print_human_string(printer, self);
    }
  }

  // Returns a human-readable string that represents the given shape, with or
  // without layout.
  pub fn to_string(&self, print_layout: bool) -> String {
    if print_layout {
      ShapeUtil::human_string_with_layout(self)
    } else {
      ShapeUtil::human_string(self)
    }
  }

  pub fn rank(&self) -> usize {
    debug_assert!(self.is_array(), "Non-arrays do not have a rank.");
    self.dimensions.len()
  }

  pub fn is_array(&self) -> bool {
    primitive_util::is_array_type(&self.element_type)
  }

  pub fn is_tuple(&self) -> bool {
    self.element_type == PrimitiveType::Tuple
  }

  pub fn is_token(&self) -> bool {
    self.element_type == PrimitiveType::Token
  }

  pub fn is_opaque(&self) -> bool {
    self.element_type == PrimitiveType::OpaqueType
  }

  pub fn is_integer(&self) -> bool {
    if primitive_util::is_integral_type(&self.element_type) {
      return true;
    }
    if self.is_tuple() {
      return false; // TODO
    }
    false
  }

  // Returns true if no array dimension in the shape is dynamically sized.
  // Tuple shapes are traversed recursively.
  pub fn is_static(&self) -> bool {
    if self.is_tuple() {
      for subshape in &self.tuple_shapes {
        if !subshape.is_static() { return false; }
      }
    }
    for dyn_dim in &self.dynamic_dimensions {
      if *dyn_dim { return false; }
    }
    true
  }

  pub fn is_dynamic(&self) -> bool {
    !self.is_static()
  }

  // Returns true if the shape has one or more dimensions with unbounded sizes.
  // Tuple shapes are traversed recursively.
  pub fn is_unbounded_dynamic(&self) -> bool {
    if self.is_tuple() {
      for subshape in &self.tuple_shapes {
        if subshape.is_unbounded_dynamic() { return true; }
      }
    }
    for dim in &self.dimensions {
      if *dim == Shape::UNBOUNDED_SIZE { return true; }
    }
    false
  }

  pub fn is_unbounded_dynamic_dimension(&self, dimension: usize) -> bool {
    self.dimensions[dimension] == Shape::UNBOUNDED_SIZE
  }

  pub fn set_unbounded_dynamic_dimension(&mut self, dimension: usize) {
    self.dynamic_dimensions[dimension] = true;
    self.dimensions[dimension] = Shape::UNBOUNDED_SIZE;
  }

  pub fn is_bounded_dynamic_dimension(&self, dimension: i64) -> bool {
    self.is_dynamic_dimension(dimension) &&
    !self.is_bounded_dynamic_dimension(dimension)
  }

  // Returns true if the given dimension is dynamically-sized.
  pub fn is_dynamic_dimension(&self, dimension: i64) -> bool {
    self.dynamic_dimensions[dimension as usize]
  }

  pub fn clear_is_dynamic_dimension(&mut self) {
    unimplemented!()
  }

  // Returns true if the given dimension is statically-sized.
  pub fn is_static_dimension(&self, dimension: usize) -> bool {
    !self.dynamic_dimensions[dimension]
  }

  // Sets whether or not the given dimension is dynamically-sized.
  pub fn set_dynamic_dimension(&mut self, dimension: usize, is_dynamic: bool) {
    self.dynamic_dimensions[dimension] = is_dynamic;
  }

  pub fn dinamic_dimensions(&self) -> &Vec<bool> {
    &self.dynamic_dimensions
  }

  pub fn delete_dimension(&mut self, dim_to_delete: i64) {
    assert!(self.is_array());
    assert!(dim_to_delete >= 0);
    assert!(self.dimensions.len() > dim_to_delete as usize);
    self.dimensions.remove(dim_to_delete as usize);
    self.dynamic_dimensions.remove(dim_to_delete as usize);
    if LayoutUtil::has_layout(&self) {
      self.layout.as_mut().unwrap().delete_dimension(dim_to_delete);
    }
  }

  pub fn element_type(&self) -> PrimitiveType {
    self.element_type.clone()
  }

  pub fn set_element_type(&mut self, value: PrimitiveType) {
    self.element_type = value;
  }

  pub fn dimensions_size(&self) -> usize {
    self.dimensions.len()
  }

  pub fn dimensions(&self, index: usize) -> i64 {
    self.dimensions[index]
  }

  pub fn dimensions_minor(&self, index: usize) -> i64 {
    assert!(self.has_layout());
    self.dimensions[self.layout.as_ref().unwrap().minor_to_major(index) as usize]
  }

  pub fn set_dimensions(&mut self, index: usize, value: i64) {
    self.dimensions[index] = value;
  }

  pub fn set_dimensions_minor(&mut self, index: usize, value: i64) {
    assert!(self.has_layout());
    let dim_index = self.layout.as_ref().unwrap().minor_to_major(index) as usize;
    self.dimensions[dim_index] = value;
  }

  pub fn add_dimensions(&mut self, value: i64) {
    self.dimensions.push(value);
    self.dynamic_dimensions.push(false);
  }

  pub fn clear_dimensions(&mut self) {
    self.dimensions.clear();
    self.dynamic_dimensions.clear();
  }

  pub fn dimensions_vec(&self) -> &DimensionVector{
    &self.dimensions
  }

  pub fn mutable_dimensions_vec(&mut self) -> &mut DimensionVector {
    &mut self.dimensions
  }

  pub fn tuple_shapes_size(&self) -> usize {
    self.tuple_shapes.len()
  }

  pub fn tuple_shapes(&self, index: usize) -> &Shape {
    &self.tuple_shapes[index]
  }

  pub fn mutable_tuple_shapes(&mut self, index: usize) -> &mut Shape {
    &mut self.tuple_shapes[index]
  }

  pub fn set_tuple_shapes(&mut self, index: usize, shape: Shape) {
    self.tuple_shapes[index] = shape;
  }

  pub fn add_tuple_shapes(&mut self, shape: Shape) {
    self.tuple_shapes.push(shape);
  }

  pub fn clear_tuple_shapes(&mut self) {
    self.tuple_shapes.clear();
  }

  pub fn tuple_shapes_vec(&self) -> &Vec<Shape> {
    &self.tuple_shapes
  }

  pub fn tuple_shapes_vec_mut(&mut self) -> &mut Vec<Shape>{
    &mut self.tuple_shapes
  }

  pub fn has_layout(&self) -> bool {
    self.layout.is_some()
  }

  pub fn layout(&self) -> &Option<Layout> {
    &self.layout
  }

  pub fn mutable_layout(&mut self) -> &mut Option<Layout> {
    assert!(self.is_array());
    if !self.has_layout() {
      self.layout = Some(Layout::new());
    }
    &mut self.layout
  }

  pub fn set_layout(&mut self, layout: Layout) {
    self.layout = Some(layout);
  }

  pub fn clear_layout(&mut self) {
    self.layout = None;
  }

  pub fn clear_dynamic_dimensions(&mut self) {
    if !self.is_tuple() {
      if self.is_dynamic() {
        self.mutable_layout().as_mut().unwrap()
          .set_dynamic_shape_metadata_prefix_bytes(0);
      }
      return;
    }
    for sub_shape in &mut self.tuple_shapes {
      sub_shape.clear_dynamic_dimensions();
    }
  }

  pub fn swap() {}

  pub fn clear(&mut self) {
    self.element_type = PrimitiveType::Invalid;
    self.clear_dimensions();
    self.clear_tuple_shapes();
    self.clear_layout();
  }

  pub fn serialize_as_string() {}
  pub fn short_debug_string() {}
  pub fn debug_string() {}

  pub fn add_minor_to_major(&mut self, value: i64) {
    let minor_to_major =
      self.mutable_layout().as_mut().unwrap().minor_to_major_vec_mut();
    minor_to_major.push(value);
  }
}

pub struct ShapeEqual {
  ignore_layout: bool,
  ignore_tiles_in_layout: bool,
  ignore_element_size_in_layout: bool,
  ignore_memory_space_in_layout: bool,
  ignore_element_type: bool,
  ignore_fp_precision: bool,
  ignore_dynamic_dimension: bool,
  ignore_dimensions: bool,
  ignore_tail_padding_alignment_in_elements_in_layout: bool,
}

impl ShapeEqual {
  pub fn new() -> Self {
    ShapeEqual {
      ignore_layout: false,
      ignore_tiles_in_layout: false,
      ignore_element_size_in_layout: false,
      ignore_memory_space_in_layout: false,
      ignore_element_type: false,
      ignore_fp_precision: false,
      ignore_dynamic_dimension: false,
      ignore_dimensions: false,
      ignore_tail_padding_alignment_in_elements_in_layout: false,
    }
  }

  pub fn equal(&self, lhs: &Shape, rhs: &Shape) -> bool {
    if lhs.is_tuple() {
      if rhs.is_tuple() {
        let mut l_iter = lhs.tuple_shapes.iter();
        let mut r_iter = rhs.tuple_shapes.iter();
        loop {
          let l = l_iter.next();
          let r = r_iter.next();
          if l.is_none() && r.is_none() { return true; }
          if !self.equal(l.as_ref().unwrap(), r.as_ref().unwrap()) {
            return false;
          }
        }
      } else {
        return false;
      }
    } else if !lhs.is_array() {
      return lhs.element_type() == rhs.element_type();
    }

    if !rhs.is_array() {
      return false;
    }

    if !self.ignore_element_type {
      if (self.ignore_fp_precision &&
        !ShapeUtil::same_element_type_ignoring_fp_precision(lhs, rhs)) ||
        (!self.ignore_fp_precision &&
        !ShapeUtil::same_element_type(lhs, rhs))
      {
        println!("compare_shapes: lhs element type != rhs element type.");
        return false;
      }
    }

    if !self.ignore_dimensions {
      if !ShapeUtil::same_dimensions(lhs, rhs) {
        println!("compare_shapes: lhs dimensions != rhs dimensions.");
        return false;
      }
    } else {
      if !ShapeUtil::same_rank(lhs, rhs) {
        println!("compare_shapes: lhs rank != rhs rank.");
        return false;
      }
    }

    if !self.ignore_layout {
      if lhs.is_array() {
        let mut layout_equal = LayoutEqual::new();
        if lhs.has_layout() || rhs.has_layout() {
          if !lhs.has_layout() || !rhs.has_layout() {
            return false;
          }
          if self.ignore_tiles_in_layout {
            layout_equal.ignore_tiles();
          }
          if self.ignore_element_size_in_layout {
            layout_equal.ignore_element_size();
          }
          if self.ignore_memory_space_in_layout {
            layout_equal.ignore_memory_space();
          }
          if self.ignore_tail_padding_alignment_in_elements_in_layout {
            layout_equal.ignore_tail_padding_alignment_in_elements();
          }
          let lhs_layout = lhs.layout().as_ref().unwrap();
          let rhs_layout = rhs.layout().as_ref().unwrap();
          if !layout_equal.equal(lhs_layout, rhs_layout) {
            return false;
          }
        }
      }
    }

    if !self.ignore_dynamic_dimension {
      for i in 0..lhs.rank() {
        if lhs.is_dynamic_dimension(i as i64) != rhs.is_dynamic_dimension(i as i64) {
          return false;
        }
      }
    }

    true
  }

  pub fn ignore_layout(&mut self) -> &mut Self {
    self.ignore_layout = true;
    self
  }

  pub fn ignore_tiles_in_layout(&mut self) -> &mut Self {
    self.ignore_tiles_in_layout = true;
    self
  }

  pub fn ignore_element_size_in_layout(&mut self) -> &mut Self {
    self.ignore_element_size_in_layout = true;
    self
  }

  pub fn ignore_memory_space_in_layout(&mut self) -> &mut Self {
    self.ignore_memory_space_in_layout = true;
    self
  }

  pub fn minor_to_major_only_in_layout(&mut self) -> &mut Self {
    self.ignore_tiles_in_layout = true;
    self.ignore_element_size_in_layout = true;
    self.ignore_memory_space_in_layout = true;
    self.ignore_tail_padding_alignment_in_elements_in_layout = true;
    self
  }

  pub fn ignore_element_type(&mut self) -> &mut Self {
    self.ignore_element_type = true;
    self
  }

  pub fn ignore_fp_precision(&mut self) -> &mut Self {
    self.ignore_fp_precision = true;
    self
  }

  pub fn ignore_dynamic_dimension(&mut self) -> &mut Self {
    self.ignore_dynamic_dimension = true;
    self
  }

  pub fn ignore_dimensions(&mut self) -> &mut Self {
    self.ignore_dimensions = true;
    self
  }

  pub fn ignore_tail_padding_alignment_in_elements_in_elements(&mut self) -> &mut Self {
    self.ignore_tail_padding_alignment_in_elements_in_layout = true;
    self
  }
}

#[derive(Debug, Clone)]
pub struct ProgramShape {
  parameters: Vec<Shape>,
  parameter_names: Vec<String>,
  // The shape of the result of the computation represented by this object.
  result: Shape,
}

impl ProgramShape {
  pub fn new() -> Self {
    ProgramShape {
      parameters: Vec::new(),
      parameter_names: Vec::new(),
      result: Shape::new(),
    }
  }

  pub fn print(&self, printer: &mut dyn Printer) {
    ShapeUtil::print_human_string_for_program_shape(printer, self);
  }

  pub fn to_string(&self) -> String {
    ShapeUtil::human_string_for_program_shape(self)
  }

  pub fn parameters_size(&self) -> usize {
    self.parameters.len()
  }

  pub fn parameters(&self, index: usize) -> &Shape {
    &self.parameters[index]
  }

  pub fn add_parameter(&mut self, shape: Shape) {
    self.parameters.push(shape);
  }

  pub fn set_parameter(&mut self, index: usize, shape: Shape) {
    self.parameters[index] = shape;
  }

  pub fn add_parameters(&mut self) {
    //self.parameters.
  }

  pub fn clear_parameters(&mut self) {
    self.parameters.clear();
  }

  pub fn parameters_vec(&self) -> &Vec<Shape> {
    &self.parameters
  }

  // Methods for accessing and manipulating the Shape of the result.
  pub fn result(&self) -> &Shape {
    &self.result
  }

  pub fn mutable_result(&mut self) -> &mut Shape {
    &mut self.result
  }

  pub fn set_result(&mut self, shape: Shape) {
    self.result = shape;
  }

  pub fn parameter_names_size(&self) -> usize {
    self.parameter_names.len()
  }

  pub fn parameter_names(&self, index: usize) -> String {
    self.parameter_names[index].clone()
  }

  pub fn set_parameter_name(&mut self, index: usize, value: String) {
    self.parameter_names[index] = value;
  }

  pub fn add_parameter_names(&mut self, value: String) {
    self.parameter_names.push(value);
  }

  pub fn add_parameter_names_empty(&mut self) {
    self.parameter_names.push("".to_string());
  }

  pub fn clear_parameter_names(&mut self) {
    self.parameter_names.clear();
  }

  pub fn short_debug_string() {}
  pub fn debug_string() {}
}

#[cfg(test)]
mod tests {
  use crate::layout::Tile;
  use super::*;

  fn make_tuple() -> Shape {
    let opaque = ShapeUtil::make_opaque_shape();
    let scalar = ShapeUtil::make_shape(&PrimitiveType::F32, vec![]);
    let matrix = ShapeUtil::make_shape(&PrimitiveType::U32, vec![1, 2]);
    let matrix2 =
      ShapeUtil::make_shape_with_dense_layout(
        &PrimitiveType::S32,
        &vec![3, 4],
        &vec![0, 1],
        Vec::new(),
        1,
        0,
        0
      );
    
    let tuple_vec = vec![opaque.clone(), scalar.clone(), matrix.clone(), matrix2.clone()];
    ShapeUtil::make_tuple_shape(tuple_vec)
  }

  #[test]
  fn test_shape_to_string() {
    let opaque = ShapeUtil::make_opaque_shape();
    let token = ShapeUtil::make_token_shape();
    let scalar = ShapeUtil::make_shape(&PrimitiveType::F32, vec![]);
    let matrix = ShapeUtil::make_shape(&PrimitiveType::U32, vec![1, 2]);
    let matrix2 =
      ShapeUtil::make_shape_with_dense_layout(
        &PrimitiveType::S32,
        &vec![3, 4],
        &vec![0, 1],
        Vec::new(),
        1,
        0,
        0
      );
    let scalar_with_tile = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      &Vec::new(),
      &Vec::new(),
      vec![Tile::new(vec![256])],
      1,
      0,
      0
    );

    assert_eq!(opaque.to_string(false), "opaque[]".to_string());
    assert_eq!(token.to_string(false), "token[]".to_string());
    assert_eq!(scalar.to_string(false), "f32[]".to_string());
    assert_eq!(matrix.to_string(false), "u32[1,2]".to_string());
    assert_eq!(matrix2.to_string(false), "s32[3,4]".to_string());

    assert_eq!(opaque.to_string(true), "opaque[]".to_string());
    assert_eq!(scalar.to_string(true), "f32[]".to_string());
    assert_eq!(scalar_with_tile.to_string(true), "f32[]{:T(256)}".to_string());
    assert_eq!(matrix.to_string(true), "u32[1,2]{1,0}".to_string());
    assert_eq!(matrix2.to_string(true), "s32[3,4]{0,1}".to_string());

    let tuple_vec = vec![opaque.clone(), scalar.clone(), matrix.clone(), matrix2.clone()];
    let tuple = ShapeUtil::make_tuple_shape(tuple_vec);
    assert_eq!(tuple.to_string(false), "(opaque[], f32[], u32[1,2], s32[3,4])".to_string());
    assert_eq!(tuple.to_string(true), "(opaque[], f32[], u32[1,2]{1,0}, s32[3,4]{0,1})".to_string());

    let nested_tuple_vec = vec![tuple.clone(), matrix.clone(), token.clone()];
    let nested_tuple = ShapeUtil::make_tuple_shape(nested_tuple_vec);
    assert_eq!(nested_tuple.to_string(false),
      "((opaque[], f32[], u32[1,2], s32[3,4]), u32[1,2], token[])".to_string());
    assert_eq!(nested_tuple.to_string(true),
      "((opaque[], f32[], u32[1,2]{1,0}, s32[3,4]{0,1}), u32[1,2]{1,0}, token[])".to_string());
  }

  #[test]
  fn test_dynamic_shape_to_string() {
    let mut array_shape = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::F32,
      vec![23, 44, 55],
      vec![true, false, true]);
    assert_eq!(array_shape.to_string(false), "f32[<=23,44,<=55]".to_string());

    array_shape.set_dynamic_dimension(2, false);
    assert_eq!(array_shape.to_string(false), "f32[<=23,44,55]".to_string());

    let unbounded = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::F32,
      vec![Shape::UNBOUNDED_SIZE, 784],
      vec![true, false]);
    assert_eq!(unbounded.to_string(false), "f32[?,784]".to_string());
  }

  #[test]
  fn test_is_static() {
    let opaque = ShapeUtil::make_opaque_shape();
    let token = ShapeUtil::make_token_shape();
    let scalar = ShapeUtil::make_shape(&PrimitiveType::F32, vec![]);
    let matrix = ShapeUtil::make_shape(&PrimitiveType::U32, vec![1, 2]);
    let matrix2 =
    ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::S32,
      &vec![3, 4],
      &vec![0, 1],
      Vec::new(),
      1,
      0,
      0
    );

    let tuple_vec = vec![opaque.clone(), scalar.clone(), matrix.clone(), matrix2.clone()];
    let tuple = ShapeUtil::make_tuple_shape(tuple_vec);

    let nested_tuple_vec = vec![tuple.clone(), matrix.clone(), token.clone()];
    let nested_tuple = ShapeUtil::make_tuple_shape(nested_tuple_vec);

    assert_eq!(opaque.is_static(), true);
    assert_eq!(token.is_static(), true);
    assert_eq!(matrix.is_static(), true);
    assert_eq!(tuple.is_static(), true);
    assert_eq!(nested_tuple.is_static(), true);

    let mut dynamic_matrix = matrix.clone();
    assert_eq!(dynamic_matrix.is_static(), true);
    dynamic_matrix.set_dynamic_dimension(1, true);
    assert_eq!(dynamic_matrix.is_static(), false);

    let mut dynamic_tuple = tuple.clone();
    assert_eq!(dynamic_tuple.is_static(), true);
    ShapeUtil::get_mutable_subshape(&mut dynamic_tuple, vec![2])
      .set_dynamic_dimension(1, true);
    assert_eq!(dynamic_tuple.is_static(), false);

    let unbounded = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::F32,
      vec![Shape::UNBOUNDED_SIZE, 784],
      vec![true, false]);
    assert_eq!(unbounded.is_static(), false);
  }

  #[test]
  fn test_is_dynamic() {
    let matrix = ShapeUtil::make_shape(&PrimitiveType::U32, vec![1, 2]);
    assert_eq!(matrix.is_dynamic(), false);
    assert_eq!(matrix.is_unbounded_dynamic(), false);

    let dynamic_matrix = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::S32,
      vec![5, 2],
      vec![true, false]);
    assert_eq!(dynamic_matrix.is_dynamic(), true);
    assert_eq!(dynamic_matrix.is_unbounded_dynamic(), false);

    let unbounded = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::F32,
      vec![Shape::UNBOUNDED_SIZE, 784],
      vec![true, false]);
    assert_eq!(unbounded.is_dynamic(), true);
    assert_eq!(unbounded.is_unbounded_dynamic(), true);

    let mut unbounded_tuple = make_tuple();
    assert_eq!(unbounded_tuple.is_unbounded_dynamic(), false);
    ShapeUtil::get_mutable_subshape(&mut unbounded_tuple, vec![2])
      .set_dynamic_dimension(1, true);
    assert_eq!(unbounded_tuple.is_unbounded_dynamic(), false);
    ShapeUtil::get_mutable_subshape(&mut unbounded_tuple, vec![2])
      .set_dimensions(1, Shape::UNBOUNDED_SIZE);
    assert_eq!(unbounded_tuple.is_unbounded_dynamic(), true);
  }

  #[test]
  fn test_is_dynamic_dimension() {
    let mut dynamic_matrix =
      ShapeUtil::make_shape(&PrimitiveType::U32, vec![1, 2]);
    dynamic_matrix.set_dynamic_dimension(1, true);
    assert_eq!(dynamic_matrix.is_dynamic_dimension(0), false);
    assert_eq!(dynamic_matrix.is_dynamic_dimension(1), true);

    let mut dynamic_tuple = make_tuple();
    assert_eq!(dynamic_tuple.is_static(), true);
    ShapeUtil::get_mutable_subshape(&mut dynamic_tuple, vec![2])
      .set_dynamic_dimension(1, true);
    assert_eq!(dynamic_tuple.is_static(), false);

    let unbounded = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::F32,
      vec![Shape::UNBOUNDED_SIZE, 784],
      vec![true, false]);
    assert_eq!(unbounded.is_dynamic_dimension(0), true);
    assert_eq!(unbounded.is_dynamic_dimension(1), false);
  }
}