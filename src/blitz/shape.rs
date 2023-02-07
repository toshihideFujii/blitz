#![allow(dead_code)]

// A shape describes the number of dimensions in a array,
// the bounds of each dimension, and the primitive component type.
// For tuples, shape describes the structure (number of elements and nesting).
struct Shape {}

impl Shape {
  pub fn to_string() {}

  pub fn rank() {}

  pub fn is_array() {}
  pub fn is_tuple() {}
  pub fn is_token() {}
  pub fn is_opaque() {}

  pub fn is_integer() {}
  pub fn is_static() {}
  pub fn is_dynamic() {}

  pub fn is_dynamic_dimension() {}
  pub fn set_dynamic_dimansion() {}

  pub fn delete_dimension() {}

  pub fn element_type() {}
  pub fn dimensions() {}
  pub fn set_dimensions() {}
  pub fn add_dimensions() {}
  pub fn clear_dimensions() {}
}