#![allow(dead_code)]

use crate::{
  util::DimensionVector,
  layout::Layout,
  blitz_data::PrimitiveType,
  primitive_util, layout_util::LayoutUtil,
};

#[derive(Clone)]
pub struct Shape {
  element_type: PrimitiveType,
  dimensions: DimensionVector,
  dynamic_dimensions: Vec<bool>,
  tuple_shapes: Vec<Shape>,
  layout: Option<Layout>
}

impl Shape {
  pub const UNBOUNDED_SIZE: i64 = i64::MIN;

  pub fn new_default() -> Self {
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

  pub fn print() {}
  pub fn to_string() {}

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

  pub fn is_static(&self) -> bool { false}

  pub fn is_dynamic(&self) -> bool {
    !self.is_static()
  }

  pub fn is_unbounded_dynamic() {}

  pub fn is_unbounded_dynamic_dimension(&self, dimension: usize) -> bool {
    self.dimensions[dimension] == Shape::UNBOUNDED_SIZE
  }

  pub fn set_unbounded_dynamic_dimension(&mut self, dimension: usize) {
    self.dynamic_dimensions[dimension] = true;
    self.dimensions[dimension] = Shape::UNBOUNDED_SIZE;
  }

  pub fn is_bounded_dynamic_dimension(&self, dimension: usize) -> bool {
    self.is_dynamic_dimension(dimension) &&
    !self.is_bounded_dynamic_dimension(dimension)
  }

  pub fn is_dynamic_dimension(&self, dimension: usize) -> bool {
    self.dynamic_dimensions[dimension]
  }

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

  pub fn tuple_shapes_size(&self) -> usize {
    self.tuple_shapes.len()
  }

  pub fn tuple_shapes(&self, index: usize) -> &Shape {
    &self.tuple_shapes[index]
  }

  pub fn set_tuple_shapes(&mut self, index: usize, shape: Shape) {
    self.tuple_shapes[index] = shape;
  }

  pub fn add_tuple_shapes(&mut self, shape: Shape) {
    //self.tuple_shapes.push(Shape::new_default());
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

  pub fn mutable_lauout(&mut self) -> &mut Option<Layout>{
    &mut self.layout
  }

  pub fn clear_layout(&mut self) {
    self.layout = None;
  }

  pub fn clear_dynamic_dimensions(&mut self) {
    if !self.is_tuple() {
      if self.is_dynamic() {
        self.mutable_lauout().as_mut().unwrap()
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
      self.mutable_lauout().as_mut().unwrap().minor_to_major_vec_mut();
    minor_to_major.push(value);
  }
}

pub struct ProgramShape {
  parameters: Vec<Shape>,
  parameter_names: Vec<String>,
  result: Shape,
}

impl ProgramShape {
  pub fn new() {}
  pub fn print() {}
  pub fn to_string() {}

  pub fn parameters_size() {}
  pub fn parameters() {}
  pub fn add_parameters() {}
  pub fn clear_parameters() {}
  pub fn result() {}
  pub fn parameter_names_size() {}
  pub fn parameter_names() {}
  pub fn set_parameter_names() {}
  pub fn add_parameter_names() {}
  pub fn clear_parameter_names() {}
  pub fn short_debug_string() {}
  pub fn debug_string() {}
}