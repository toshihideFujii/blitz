#![allow(dead_code)]

pub struct LayoutUtil {}

impl LayoutUtil {
  pub fn make_layout() {}

  pub fn make_descending_layout() {}
  pub fn make_ascending_layout() {}
  pub fn get_default_layout_for_shape() {}
  pub fn get_default_layout_for_rank() {}
  pub fn get_default_layout_for_r2() {}
  pub fn get_default_layout_for_r3() {}
  pub fn get_default_layout_for_r4() {}

  pub fn set_to_default_layout() {}
  pub fn get_with_default_layout() {}
  pub fn validate_layout_for_shape() {}

  pub fn clear_layout() {}
  pub fn clear_tiles() {}

  pub fn is_dense_array() {}
  pub fn is_sparse_array() {}
  pub fn is_coo_array() {}
  pub fn is_csr_array() {}
  pub fn is_dense() {}
  pub fn is_sparse() {}
  pub fn is_coo() {}
  pub fn is_csr() {}
  pub fn is_csc() {}
  pub fn is_monotonic_with_dim0_minor() {}
  pub fn is_monotonic_with_dim0_major() {}

  pub fn has_layout() {}
  pub fn has_custom_element_size_in_bits() {}

  pub fn minor_to_major() {}
  pub fn major() {}
  pub fn minor() {}

  pub fn make_logical_to_physical() {}
  pub fn print_human_string() {}

  pub fn copy_layout_between_sjapes() {}
  pub fn layouts_in_shapes_equal() {}
  pub fn are_dimensions_consecutive() {}
  pub fn move_dim_to_major() {}
  pub fn linear_index() {}
  pub fn memory_space() {}
  pub fn get_dim_level_type() {}
  pub fn dim_unique() {}
  pub fn dim_ordered() {}
  pub fn validate_din_level() {}
  pub fn byte_strides_id_mejot_to_minor() {}
}