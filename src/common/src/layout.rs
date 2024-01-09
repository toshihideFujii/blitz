#![allow(dead_code)]

pub struct Tile {}

impl Tile {
  pub fn new() {}

  pub fn print() {}
  pub fn to_string() {}

  pub fn dimension() {}
  pub fn dimensions() {}
  pub fn add_dimensions() {}
  pub fn clear_dimensions() {}
  pub fn absl_hash_value() {}
}

pub struct Layout {
  n_dim_level_types: u8,
  n_dim_unique: u8,
  n_dim_ordered: u8,
  element_size_in_bits: u16,
  memory_space: i8,
  dynamic_shape_metadata_prefix_bytes: i64,
}

impl Layout {
  pub fn new() {}
  pub fn print() {}
  pub fn to_string() {}

  pub fn dim_level_types_size() {}
  pub fn dim_level_tye() {}
  pub fn add_dim_level_type() {}
  pub fn clear_dim_level_types() {}

  pub fn dim_unique_size() {}
  pub fn dim_unique() {}
  pub fn add_dim_unique() {}
  pub fn dim_ordered_size() {}
  pub fn dim_ordered() {}
  pub fn set_dim_ordered() {}
  pub fn add_dim_ordered() {}

  pub fn minor_to_major_size() {}
  pub fn minor_to_major() {}
  pub fn set_minor_to_major() {}
  pub fn add_minor_to_major() {}
  pub fn clear_minor_to_major() {}

  pub fn delete_dimension() {}
  pub fn tiles_size() {}
  pub fn tiles() {}
  pub fn add_tiles() {}
  pub fn clear_tiles() {}

  pub fn element_size_in_bits() {}
  pub fn set_element_size_in_bits() {}
  pub fn index_primitive_type() {}
  pub fn set_index_primitive_type() {}
  pub fn pointer_primitive_type() {}
  pub fn set_pointer_primitive_type() {}

  pub fn memory_space() {}
  pub fn set_memory_space() {}
  pub fn has_physical_shape() {}
  pub fn physical_shape() {}
  pub fn clear_physical_shape() {}
  pub fn dynamic_shape_metadata_prefix_bytes() {}
  pub fn set_dynamic_shape_metadata_prefix_bytes() {}

  pub fn swap() {}
  pub fn clear() {}
  pub fn absl_hash_value() {}
}