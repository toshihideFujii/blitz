#![allow(dead_code)]

use crate::{
  blitz_data::{DimLevelType, PrimitiveType},
  layout::{Tile, Layout},
  shape::Shape, util::DimensionVector, printer::Printer
};

pub fn set_default_layout_to_container(minor_to_major: &mut Vec<i64>) {
  let size = minor_to_major.len();
  for i in 0..size {
    minor_to_major[i] = (size - 1 - i) as i64;
  }
}

pub struct LayoutUtil {}

impl LayoutUtil {
  pub fn make_layout(
    minor_to_major: Vec<i64>,
    dim_level_types: Vec<DimLevelType>,
    dim_unique: Vec<bool>,
    dim_ordered: Vec<bool>,
    tiles: Vec<Tile>,
    index_primitive_type: PrimitiveType,
    pointer_primitive_type: PrimitiveType,
    element_size_in_bits: i64,
    memory_space: i64,
    physical_shape: Option<Shape>,
    dynamic_shape_metadata_prefix_bytes: i64,
  ) -> Layout
  {
    let mut layout = Layout::new();
    for dimension_number in minor_to_major {
      layout.add_minor_to_major(dimension_number);
    }
    for dim_level_type in dim_level_types {
      layout.add_dim_level_type(dim_level_type);
    }
    for unique in dim_unique {
      layout.add_dim_unique(unique);
    }
    for ordered in dim_ordered {
      layout.add_dim_ordered(ordered);
    }
    for tile in tiles {
      for dim in tile.dimensions() {
        if *dim < 0 && *dim != Tile::COMBINE_DIMENSION {
          unreachable!("Tile dimension size needs to be minimum i64 value if it's negative.")
        }
      }
      layout.add_tiles(tile);
    }
    layout.set_index_primitive_type(index_primitive_type);
    layout.set_pointer_primitive_type(pointer_primitive_type);
    layout.set_element_size_in_bits(element_size_in_bits);
    layout.set_memory_space(memory_space);
    if physical_shape.is_some() {
      layout.set_physical_shape(physical_shape.unwrap());
    }
    layout.set_dynamic_shape_metadata_prefix_bytes(dynamic_shape_metadata_prefix_bytes);
    layout
  }

  pub fn make_descending_layout() {}
  pub fn make_ascending_layout() {}

  pub fn make_layout_from_major_to_minor(major_to_minor: Vec<i64>) -> Layout {
    let mut layout = Layout::new();
    for i in major_to_minor.len()..0 {
      layout.add_minor_to_major(major_to_minor[i]);
    }
    layout
  }

  pub fn create_default_layout_for_rank(_rank: i64) -> Layout {
    Layout::new()
  }

  pub fn get_default_layout_for_shape(shape: &Shape) -> Layout {
    if shape.is_opaque() || shape.is_token() {
      return Layout::new();
    }
    assert!(shape.is_array());
    LayoutUtil::create_default_layout_for_rank(shape.dimensions_size() as i64)
  }

  pub fn get_default_layout_for_rank(rank: i64) -> Layout {
    LayoutUtil::create_default_layout_for_rank(rank)
  }

  pub fn get_default_layout_for_r2() -> Layout {
    LayoutUtil::create_default_layout_for_rank(2)
  }

  pub fn get_default_layout_for_r3() -> Layout {
    LayoutUtil::create_default_layout_for_rank(3)
  }

  pub fn get_default_layout_for_r4() -> Layout {
    LayoutUtil::create_default_layout_for_rank(4)
  }

  pub fn set_to_default_layout(shape: &mut Shape) {
    if shape.is_tuple() {
      for elt_shape in shape.tuple_shapes_vec_mut() {
        LayoutUtil::set_to_default_layout(elt_shape);
      }
      shape.clear_layout();
    } else if shape.is_array() {
      let dim_size = shape.dimensions_size();
      let minor_to_major =
        shape.mutable_lauout().as_mut().unwrap().minor_to_major_vec_mut();
      minor_to_major.resize(dim_size, 0);
      set_default_layout_to_container(minor_to_major);
    } else {
      shape.clear_layout();
    }
  }

  pub fn get_with_default_layout() {}
  pub fn validate_layout_for_shape() {}

  pub fn clear_layout(shape: &mut Shape) {
    shape.clear_layout();
    for elt_shape in shape.tuple_shapes_vec_mut() {
      LayoutUtil::clear_layout(elt_shape);
    }
  }

  pub fn clear_tiles() {}

  pub fn is_dense_array(_shape: &Shape) -> bool {
    false
  }

  pub fn is_sparse_array() {}
  pub fn is_coo_array() {}
  pub fn is_csr_array() {}

  // Returns whether the given layout has a dense in-memory representation.
  pub fn is_dense(layout: &Layout) -> bool {
    for i in 0..layout.dim_level_types_size() {
      if layout.dim_level_type(i) != DimLevelType::Dense {
        return false;
      }
    }
    true
  }

  pub fn is_sparse(layout: &Layout) -> bool {
    !LayoutUtil::is_dense(layout)
  }

  pub fn is_coo(layout: &Layout) -> bool {
    if layout.dim_level_types_size() == 0 ||
      layout.dim_level_type(0) != DimLevelType::Compressed {
      return false;
    }
    for i in 1..layout.dim_level_types_size() {
      if layout.dim_level_type(i) != DimLevelType::Singleton {
        return false;
      }
    }
    true
  }

  pub fn is_csr(layout: &Layout) -> bool {
    LayoutUtil::is_monotonic_with_dim0_major(layout) &&
    layout.dim_level_types_size() == 2 &&
    layout.dim_level_type(0) == DimLevelType::Dense &&
    layout.dim_level_type(1) == DimLevelType::Compressed
  }

  pub fn is_csc(layout: &Layout) -> bool {
    LayoutUtil::is_monotonic_with_dim0_minor(layout) &&
    layout.dim_level_types_size() == 2 &&
    layout.dim_level_type(0) == DimLevelType::Dense &&
    layout.dim_level_type(1) == DimLevelType::Compressed
  }

  pub fn is_monotonic_with_dim0_minor(_layout: &Layout) -> bool {
    false
  }

  pub fn is_monotonic_with_dim0_major(_layout: &Layout) -> bool {
    false
  }

  pub fn has_layout(shape: &Shape) -> bool {
    if shape.is_tuple() {
      for subshape in shape.tuple_shapes_vec() {
        if !subshape.has_layout() { return false; }
      }
      return true;
    } else if !shape.is_array() {
      return true;
    }
    shape.has_layout()
  }

  pub fn has_custom_element_size_in_bits(shape: &Shape) -> bool {
    if shape.is_tuple() {
      // TODO
    } else if !shape.is_array() {
      return false;
    }
    shape.has_layout() &&
    shape.layout().as_ref().unwrap().element_size_in_bits() != 0
  }

  pub fn minor_to_major_from_shape(shape: &Shape) -> &DimensionVector {
    assert!(shape.is_array());
    shape.layout().as_ref().unwrap().minor_to_major_vec()
  }

  pub fn minor_to_major_from_layout(layout: &Layout) -> &DimensionVector {
    layout.minor_to_major_vec()
  }

  pub fn major(layout: &Layout, physical_dimension_number: i64) -> i64 {
    assert!(0 <= physical_dimension_number);
    assert!(physical_dimension_number < layout.minor_to_major_size() as i64);
    let phys_dim_num =
      layout.minor_to_major_size() as i64 - 1 - physical_dimension_number;
    LayoutUtil::minor(layout, phys_dim_num)
  }

  pub fn minor(layout: &Layout, physical_dimension_number: i64) -> i64 {
    assert!(0 <= physical_dimension_number);
    assert!(physical_dimension_number < layout.minor_to_major_size() as i64);
    layout.minor_to_major(physical_dimension_number as usize)
  }

  pub fn make_logical_to_physical() {}

  pub fn print_human_string(printer: &mut dyn Printer, layout: &Layout) {
    layout.print(printer)
  }

  pub fn human_string(layout: &Layout) -> String {
    layout.to_string()
  }

  pub fn copy_layout_between_sjapes() {}
  pub fn layouts_in_shapes_equal() {}
  pub fn are_dimensions_consecutive() {}
  pub fn move_dim_to_major() {}
  pub fn linear_index() {}

  pub fn memory_space(shape: &Shape) -> i64 {
    if shape.has_layout() {
      shape.layout().as_ref().unwrap().memory_space()
    } else {
      Layout::DEFAULT_MEMORY_SPACE
    }
  }

  pub fn get_dim_level_type(layout: &Layout, dim: i64) -> DimLevelType {
    if layout.dim_level_types_size() == 0 {
      return DimLevelType::Dense;
    }
    assert!((dim as usize) < layout.dim_level_types_size());
    layout.dim_level_type(dim as usize)
  }

  pub fn dim_unique(layout: &Layout, dim: i64) -> bool {
    if layout.dim_unique_size() == 0 {
      return true;
    }
    assert!(dim < layout.dim_unique_size());
    layout.dim_unique(dim as usize)
  }

  pub fn dim_ordered(layout: &Layout, dim: i64) -> bool {
    if layout.dim_ordered_size() == 0 {
      return true;
    }
    assert!(dim < layout.dim_ordered_size());
    layout.dim_ordered(dim as usize)
  }

  pub fn validate_dim_level(
    dim_level_type: DimLevelType,
    dim_unique: bool,
    dim_ordered: bool) -> bool
  {
    match dim_level_type {
      DimLevelType::Dense => return dim_unique && dim_ordered,
      _ => return true,
    }
  }

  pub fn byte_strides_is_mejor_to_minor() {}
}