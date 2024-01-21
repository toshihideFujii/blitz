#![allow(dead_code)]

use crate::{
  util::DimensionVector,
  layout::Layout,
  blitz_data::PrimitiveType,
  primitive_util,
};

pub struct Shape {
  element_type: PrimitiveType,
  dimensions: DimensionVector,
  tuple_shapes: Vec<Shape>,
  layout: Option<Layout>
}

impl Shape {
  pub fn new() {}
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

  pub fn is_integer() {}
  pub fn is_static() {}
  pub fn is_dynamic() {}

  pub fn is_unbounded_dynamic() {}
  pub fn is_unbounded_dynamic_dimension() {}
  pub fn set_unbounded_dynamic_dimension() {}

  pub fn is_bounded_dynamic_dimension() {}
  pub fn is_dynamic_dimension() {}
  pub fn set_dynamic_dimension() {}

  pub fn dinamic_dimensions() {}
  pub fn delete_dimension() {}

  pub fn element_type(&self) -> PrimitiveType {
    self.element_type.clone()
  }

  pub fn set_element_type(&mut self, value: PrimitiveType) {
    self.element_type = value;
  }

  pub fn dimensions_size() {}
  pub fn dimensions() {}
  pub fn dimensions_minor() {}
  pub fn set_dimensions() {}
  pub fn set_dimensions_minor() {}
  pub fn add_dimensions() {}
  pub fn clear_dimensions() {}

  pub fn tuple_shapes_size() {}
  pub fn tuple_shapes() {}
  pub fn add_tuple_shapes() {}
  pub fn clear_tuple_shapes() {}

  pub fn has_layout() {}
  pub fn layout() {}
  pub fn clear_layout() {}

  pub fn clear_dynamic_dimensions() {}
  pub fn swap() {}
  pub fn clear() {}

  pub fn serialize_as_string() {}
  pub fn short_debug_string() {}
  pub fn debug_string() {}
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