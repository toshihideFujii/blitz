#![allow(dead_code)]

use std::result::Result;
use crate::{
  blitz_data::{DimLevelType, PrimitiveType},
  layout::{Tile, Layout, LayoutEqual},
  shape::Shape, util::DimensionVector, printer::Printer, shape_util::ShapeUtil, primitive_util
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
    tail_padding_alignment_in_elements: i64,
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
    layout.set_tail_padding_alignment_in_elements(tail_padding_alignment_in_elements);
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

  // Returns a layout with descending (i.e. {n-1, n-2, ... 0}) minor-to-major dimensions.
  pub fn make_descending_layout(rank: i64) -> Layout {
    let mut layout = vec![];
    if rank > 0 {
      let mut i = rank as usize - 1;
      loop {
        layout.push(i as i64);
        if i == 0 { break; }
        i -= 1;
      }
    }
    LayoutUtil::make_layout(layout, vec![],
      vec![], vec![], vec![],
      1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,
      0, 0, 
      None, 0)
  }

  // Returns a layout with ascending (i.e. {0, 1, ... n-1}) minor-to-major dimensions.
  pub fn make_ascending_layout(rank: i64) -> Layout {
    let mut layout = vec![];
    for i in 0..rank {
      layout.push(i);
    }
    LayoutUtil::make_layout(layout, vec![],
      vec![], vec![], vec![],
      1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,
      0, 0, 
      None, 0)
  }

  pub fn make_layout_from_major_to_minor(major_to_minor: Vec<i64>) -> Layout {
    let mut layout = Layout::new();
    for i in major_to_minor.len()..0 {
      layout.add_minor_to_major(major_to_minor[i]);
    }
    layout
  }

  // Internal helper that creates a default layout for an array of the given rank.
  fn create_default_layout_for_rank(rank: i64) -> Layout {
    let mut layout = Layout::new();
    let minor_to_major = layout.minor_to_major_vec_mut();
    minor_to_major.resize(rank as usize, 0);
    set_default_layout_to_container(minor_to_major);
    layout
  }

  // Returns default layout for the given shape.
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

  // Sets the default layout on the shape.
  pub fn set_to_default_layout(shape: &mut Shape) {
    if shape.is_tuple() {
      for elt_shape in shape.tuple_shapes_vec_mut() {
        LayoutUtil::set_to_default_layout(elt_shape);
      }
      shape.clear_layout();
    } else if shape.is_array() {
      let dim_size = shape.dimensions_size();
      let minor_to_major =
        shape.mutable_layout().as_mut().unwrap().minor_to_major_vec_mut();
      minor_to_major.resize(dim_size, 0);
      set_default_layout_to_container(minor_to_major);
    } else {
      shape.clear_layout();
    }
  }

  pub fn get_with_default_layout() {}

  // Validates that the layout within the given shape is correct.
  // Theh check is performed for all subshapes as well.
  // If missing layouts are allowed the check does not fail on array shapes
  // without layouts.
  pub fn validate_layout_in_shape(
    shape: &Shape,
    allow_missing_layouts: bool) -> Result<(), String>
  {
    if shape.is_tuple() {
      if shape.has_layout() {
        return Err("Tuple should not have a layout field.".to_string());
      }
      for subshape in shape.tuple_shapes_vec() {
        let result =
          LayoutUtil::validate_layout_in_shape(subshape, allow_missing_layouts);  
        if result != Ok(()) { return result; }
      }
      return Ok(());
    } else if shape.is_array() {
      if !shape.has_layout() {
        if allow_missing_layouts {
          return Ok(());
        }
        return Err("Shape does not have a layout.".to_string());
      }
      return LayoutUtil::validate_layout_for_shape(
        shape.layout().as_ref().unwrap(), shape);
    } else {
      // Token, opaque, etc.
      if shape.has_layout() {
        return Err("Shape of primitive type should not have a layout.".to_string());
      }
      return Ok(());
    }
  }

  // Validates that the provided layout satisfies invariants for the given shape.
  pub fn validate_layout_for_shape(
    layout: &Layout,
    shape: &Shape) -> Result<(), String>
  {
    if shape.is_tuple() {
      return Err("A single layout is not valid for thple shapes.".to_string());
    }
    if !shape.is_array() {
      if layout.minor_to_major_size() != 0 {
        return Err("Shape of primitive type should not have a non-trivial layout.".to_string());
      }
      return Ok(());
    }
    if layout.minor_to_major_size() != shape.rank() {
      return Err("Layout minor_to_major size is not same as shape's rank.".to_string());
    }
    let mut dimensions_in_layout = vec![false; shape.rank()];
    for i in 0..shape.rank() {
      let dim = layout.minor_to_major(i);
      if dim < 0 || dim >= shape.rank() as i64 {
        return Err("Layout minor_to_major field has out-of-bounds value.".to_string());
      }
      if dimensions_in_layout[dim as usize] {
        return Err("Layout minor_to_major field has duplicate alues.".to_string());
      }
      dimensions_in_layout[dim as usize] = true;
    }
    if layout.dim_level_types_size() > 0 {
      if layout.dim_level_types_size() != shape.rank() {
        return Err("Layout dim_level_types size is not same as shape's rank.".to_string());
      }
    }
    if layout.dim_unique_size() > 0 {
      if layout.dim_unique_size() != shape.rank() {
        return Err("Layout dim_unique size is not same as shape's rank.".to_string());
      }
    }
    if layout.dim_ordered_size() > 0 {
      if layout.dim_ordered_size() != shape.rank() {
        return Err("Layout dim_ordered size is not same as shape's rank.".to_string());
      }
    }
    if layout.tail_padding_alignment_in_elements() <= 0 {
      return Err("Layout tail_padding_alignment_in_elements field is <= 0.".to_string());
    }
    if LayoutUtil::is_sparse(layout) {
      if layout.tiles_size() > 0 {
        return Err("Layout has tiles, but the shape is a sparse array.".to_string());
      }
      if layout.has_physical_shape() {
        let err =
          ShapeUtil::validate_shape(layout.physical_shape().as_ref().unwrap());
        if !err.is_ok() { return err; }
        let shape_fn =
          |subshape: &Shape, _index: usize| -> Result<(), String> {
          if subshape.has_layout() &&
            subshape.layout().as_ref().unwrap().has_physical_shape()
          {
            return Err("Layout has a physical shape, whose layout also has a physical shape.".to_string());
          }
          return Ok(());
        };
        ShapeUtil::for_each_subshape_with_status(
          &mut layout.physical_shape().as_ref().unwrap(),
          &shape_fn);
        if layout.index_primitive_type() != PrimitiveType::Invalid &&
          !primitive_util::is_unsigned_integral_type(&layout.index_primitive_type())
        {
          return Err("Index_primitive_type is not an unsigned integer type.".to_string());
        }
        if layout.pointer_primitive_type() != PrimitiveType::Invalid &&
          !primitive_util::is_unsigned_integral_type(&layout.index_primitive_type())
        {
          return Err("Pointer_primitive_type is not an unsigned integer type.".to_string());
        }
      }
    } else {
      if layout.index_primitive_type() != PrimitiveType::Invalid {
        return Err("Layout has a index_primitive_type, but is not a sparse array.".to_string());
      }
      if layout.pointer_primitive_type() != PrimitiveType::Invalid {
        return Err("Layout has a pointer_primitive_type, but is not a sparse array.".to_string());
      }
      if layout.has_physical_shape() {
        return Err("Layout has a physical_shape, but is not a sparse array.".to_string());
      }
      for tile in layout.tiles_vec() {
        if tile.dimensions().is_empty() {
          return Err("Layout has invalid tiles.".to_string());
        }
        for dim in tile.dimensions() {
          if *dim == 0 {
            return Err("Layout has invalid tiles.".to_string());
          }
        }
      }
    }
    for dim in 0..shape.rank() {
      let dim_level_type = LayoutUtil::get_dim_level_type(layout, dim as i64);
      let dim_unique = LayoutUtil::dim_unique(layout, dim as i64);
      let dim_ordered = LayoutUtil::dim_ordered(layout, dim as i64);
      if !LayoutUtil::validate_dim_level(dim_level_type, dim_unique, dim_ordered) {
        return Err("Layout dimension has invalid level encoding.".to_string());
      }
    }
    Ok(())
  }

  // Clears the layout in the given shape. After this function is called,
  // has_layout() will return false for the shape.
  pub fn clear_layout(shape: &mut Shape) {
    shape.clear_layout();
    for elt_shape in shape.tuple_shapes_vec_mut() {
      LayoutUtil::clear_layout(elt_shape);
    }
  }

  pub fn clear_tiles() {}

  // Returns whether the given shape is an array and has a dense in-memory
  // representation.
  pub fn is_dense_array(shape: &Shape) -> bool {
    shape.is_array() && (!shape.has_layout() ||
    LayoutUtil::is_dense(shape.layout().as_ref().unwrap()))
  }

  // Returns whether the given shape is an array and has a sparse in-memory
  // representation.
  pub fn is_sparse_array(shape: &Shape) -> bool {
    shape.is_array() && shape.has_layout() &&
    LayoutUtil::is_sparse(shape.layout().as_ref().unwrap())
  }

  // Returns whether the given shape is a sparse array and has a COO
  // (coordinate matrix) in-memory representation.
  pub fn is_coo_array(shape: &Shape) -> bool {
    shape.is_array() && shape.has_layout() &&
    LayoutUtil::is_coo(shape.layout().as_ref().unwrap())
  }

  // Returns whether the given shape is a sparse array and has a CSR
  // (compressed sparse row) in-memory representation.
  pub fn is_csr_array(shape: &Shape) -> bool {
    shape.is_array()&& shape.rank() == 2 && shape.has_layout() &&
    LayoutUtil::is_csr(shape.layout().as_ref().unwrap())
  }

  pub fn is_csc_array(shape: &Shape) -> bool {
    shape.is_array()&& shape.rank() == 2 && shape.has_layout() &&
    LayoutUtil::is_csc(shape.layout().as_ref().unwrap())
  }

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

  // Returns whether the given shape has a layout.
  // For tuple shapes, true is only returned if all elements have layouts.
  pub fn has_layout(shape: &Shape) -> bool {
    if shape.is_tuple() {
      for subshape in shape.tuple_shapes_vec() {
        if !LayoutUtil::has_layout(subshape) { return false; }
      }
      return true;
    } else if !shape.is_array() {
      return true;
    }
    shape.has_layout()
  }

  // Returns whether anny subshapes of the shape have custom (!=0)
  // element_size_in_bits.
  pub fn has_custom_element_size_in_bits(shape: &Shape) -> bool {
    if shape.is_tuple() {
      for subshape in shape.tuple_shapes_vec() {
        if LayoutUtil::has_custom_element_size_in_bits(subshape) {
          return true;
        }
      }
      return false;
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

  // Copies the layout from 'src' to 'dst'. Recursively copies layouts of tuples.
  // 'src' and 'dst' need not be compatible to have the same layout but the two
  // shapes must have the same tuple structure (if any) and arrays must have
  // the same rank. Within the shapes must have the same number of dimensions.
  pub fn copy_layout_between_shapes(src: &Shape, dst: &mut Shape) {
    if src.is_tuple() != dst.is_tuple() {
      assert!(false, "Cannnot copy layout from shape: shape structure differs.");
      return;
    }
    if src.is_tuple() {
      if ShapeUtil::tuple_element_count(src) !=
         ShapeUtil::tuple_element_count(dst)
      {
        assert!(false, "Cannnot copy layout from shape: tuple element count differs.");
        return;
      }
      for i in 0..ShapeUtil::tuple_element_count(src) {
        LayoutUtil::copy_layout_between_shapes(
          src.tuple_shapes(i), dst.mutable_tuple_shapes(i));
      }
    } else {
      if src.has_layout() {
        if src.rank() != dst.rank() {
          assert!(false, "Cannot copy layout from shape: ranks differs.");
          return;
        }
        let _ = LayoutUtil::validate_layout_for_shape(
          src.layout().as_ref().unwrap(),
          &dst);
        dst.set_layout(src.layout().as_ref().unwrap().clone());
      } else {
        dst.clear_layout();
      }
    }
  }

  // Returns true if the layouts of lhs and rhs are equal, false otherwise.
  // Recursively compares layouts of tuples.
  // lhs and rhs need not be compatible to have the same layout but the two
  // shapes must have the same tuple structure (if any) and arrays must have
  // the same rank. Element type is ignored.
  pub fn layouts_in_shapes_equal(lhs: &Shape, rhs: &Shape) -> bool {
    if lhs.is_tuple() {
      if !rhs.is_tuple() ||
          ShapeUtil::tuple_element_count(lhs) !=
          ShapeUtil::tuple_element_count(rhs) {
        return false;
      }
      for i in 0..ShapeUtil::tuple_element_count(rhs) {
        if !LayoutUtil::layouts_in_shapes_equal(
          lhs.tuple_shapes(i),
          rhs.tuple_shapes(i)) {
          return false;
        }
      }
      return true;
    }
    if lhs.is_array() {
      if lhs.rank() != rhs.rank() {
        return false;
      }
      if !lhs.has_layout() && !rhs.has_layout() {
        return true;
      }
      if !lhs.has_layout() || !rhs.has_layout() {
        return false;
      }
      let l1 = lhs.layout().as_ref().unwrap();
      let l2 = rhs.layout().as_ref().unwrap();
      return LayoutEqual::new().equal(l1, l2);
    }
    true
  }

  pub fn are_dimensions_consecutive() {}

  // Constructs a new layout by making the given dimension 'dim' in the given
  // 'layout' as the most major dimension.
  pub fn move_dim_to_major(layout: &mut Layout, dim: i64) -> Layout {
    if Some(&dim) == layout.minor_to_major_vec_mut().last() {
      return layout.clone();
    }
    let mut ret = layout.clone();
    ret.clear_minor_to_major();
    let mut minor_to_major = vec![];
    minor_to_major.resize(layout.minor_to_major_vec().len(), 0);
    minor_to_major.clone_from_slice(layout.minor_to_major_vec_mut());
    for d in minor_to_major {
      if d != dim {
        ret.add_minor_to_major(d);
      }
    }
    ret.add_minor_to_major(dim);
    ret
  }

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
    assert!(dim < layout.dim_unique_size() as i64);
    layout.dim_unique(dim as usize)
  }

  pub fn dim_ordered(layout: &Layout, dim: i64) -> bool {
    if layout.dim_ordered_size() == 0 {
      return true;
    }
    assert!(dim < layout.dim_ordered_size() as i64);
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

  // Returns true if 'byte_strides' is major to minor order, i.e. the strides
  // form a cumulative product of the byte size and dimensions in reverse order
  // and the smallest stride is the byte size for 'element_type'.
  pub fn byte_strides_is_major_to_minor(
    byte_strides: Vec<i64>,
    dims: Vec<i64>,
    element_type: &PrimitiveType
  ) -> bool {
    assert_eq!(dims.len(), byte_strides.len());
    let mut stride = ShapeUtil::byte_size_of_primitive_type(element_type);
    let num = dims.len() - 1;
    for i in (0..=num).rev() {
      if byte_strides[i] != stride {
        return false;
      }
      stride *= dims[i];
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_shape_with_layout(
    elt_t: PrimitiveType,
    dimensions: Vec<i64>,
    minor_to_major: Vec<i64>,
    dim_level_types: Vec<DimLevelType>
  ) -> Shape
  {
    let mut shape = ShapeUtil::make_shape(&elt_t, dimensions);
    let layout = LayoutUtil::make_layout(
      minor_to_major, dim_level_types, Vec::new(),
      Vec::new(), Vec::new(),
      1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid, 0,
      0, None, 0);
    shape.set_layout(layout);
    shape
  }

  #[test]
  fn test_tuple_layout_comparison() {
    let shape = ShapeUtil::make_tuple_shape(
      vec![make_shape_with_layout(
        PrimitiveType::F32, vec![2, 3], vec![0, 1], vec![])]
    );
    let other_shape = ShapeUtil::make_tuple_shape(
      vec![make_shape_with_layout(
        PrimitiveType::F32, vec![2, 2], vec![0, 1], vec![])]
    );

    let tuple0 = ShapeUtil::make_tuple_shape(vec![]);
    let tuple1 = ShapeUtil::make_tuple_shape(vec![shape.clone()]);
    let tuple2 = ShapeUtil::make_tuple_shape(vec![shape.clone(), shape.clone()]);

    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&tuple0, &tuple0), true);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&tuple0, &tuple1), false);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&tuple0, &tuple2), false);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&tuple1, &tuple0), false);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&tuple2, &tuple0), false);

    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&tuple1, &tuple1), true);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&tuple1, &tuple2), false);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&tuple2, &tuple1), false);

    let other_tuple2 =
      ShapeUtil::make_tuple_shape(vec![shape.clone(), other_shape.clone()]);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&tuple2, &tuple2), true);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&tuple2, &other_tuple2), true);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&other_tuple2, &tuple2), true);
  }

  #[test]
  fn test_copy_layout_dense_array() {
    let mut src = make_shape_with_layout(
      PrimitiveType::F32,
      vec![2, 3],
      vec![0, 1], vec![]);

    let mut dst = make_shape_with_layout(
      PrimitiveType::F32,
      vec![2, 3],
      vec![1, 0], vec![]);

    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), false);
    LayoutUtil::copy_layout_between_shapes(&src, &mut dst);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), true);

    // Should work if destination has no layout.
    dst.clear_layout();
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), false);
    LayoutUtil::copy_layout_between_shapes(&src, &mut dst);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), true);

    // If src is cleared, then destination should be cleared.
    src.clear_layout();
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), false);
    assert_eq!(dst.has_layout(), true);
    LayoutUtil::copy_layout_between_shapes(&src, &mut dst);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), true);
    assert_eq!(dst.has_layout(), false);
  }

  #[test] // TODO
  fn test_copy_csr_array() {
    let src = make_shape_with_layout(
      PrimitiveType::F32, vec![2, 3], vec![1, 0], 
      vec![DimLevelType::Dense, DimLevelType::Compressed]);
    
    let dst = make_shape_with_layout(
      PrimitiveType::F32, vec![2, 3], vec![0, 1],
      vec![]);

    assert_eq!(LayoutUtil::is_sparse_array(&src), true);
    assert_eq!(LayoutUtil::is_sparse_array(&dst), false);

    //assert_eq!(LayoutUtil::is_csr_array(&src), true);
    //assert_eq!(LayoutUtil::is_csr_array(&dst), false);
  }

  #[test]
  fn test_copy_layout_tuple() {
    let s1 = make_shape_with_layout(
      PrimitiveType::F32, vec![2, 3], 
      vec![0, 1], vec![]);
    let s2 = make_shape_with_layout(
      PrimitiveType::F32, vec![42, 123], 
      vec![1, 0], vec![]);

    let s3 = make_shape_with_layout(
      PrimitiveType::F32, vec![], 
      vec![], vec![]);
    let s4 = make_shape_with_layout(
      PrimitiveType::F32, vec![1, 2, 3], 
      vec![0, 2, 1], vec![]);
    let s5 = ShapeUtil::make_tuple_shape(vec![s3, s4]);

    let src = ShapeUtil::make_tuple_shape(vec![s1, s2, s5]);

    let d1 = make_shape_with_layout(
      PrimitiveType::F32, vec![2, 3], 
      vec![1, 0], vec![]);
    let d2 = make_shape_with_layout(
      PrimitiveType::F32, vec![42, 123], 
      vec![1, 0], vec![]);

    let d3 = make_shape_with_layout(
      PrimitiveType::F32, vec![], 
      vec![], vec![]);
    let d4 = make_shape_with_layout(
      PrimitiveType::F32, vec![1, 2, 3], 
      vec![1, 2, 0], vec![]);
    let d5 = ShapeUtil::make_tuple_shape(vec![d3, d4]);

    let mut dst = ShapeUtil::make_tuple_shape(vec![d1, d2, d5]);

    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), false);
    LayoutUtil::copy_layout_between_shapes(&src, &mut dst);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), true);
  }

  #[test]
  fn test_copy_layout_not_compatible_same_rank() {
    let src = make_shape_with_layout(
      PrimitiveType::F32, vec![123, 42, 7], 
      vec![2, 0, 1], vec![]);

    let mut dst = make_shape_with_layout(
      PrimitiveType::F32, vec![2, 3, 5], 
      vec![1, 0], vec![]);

    LayoutUtil::copy_layout_between_shapes(&src, &mut dst);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), true);
  }

  fn test_copy_layout_not_compatible_different_rank() {}
  fn test_copy_layout_not_compatible_tuple() {}
  fn test_copy_layout_bogus_layout() {}

  #[test]
  fn test_copy_token_layout() {
    let src = ShapeUtil::make_token_shape();
    let mut dst = ShapeUtil::make_token_shape();

    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), true);
    LayoutUtil::copy_layout_between_shapes(&src, &mut dst);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), true);
  }

  #[test]
  fn test_copy_opaque_layout() {
    let src = ShapeUtil::make_opaque_shape();
    let mut dst = ShapeUtil::make_opaque_shape();

    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), true);
    LayoutUtil::copy_layout_between_shapes(&src, &mut dst);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), true);
  }

  #[test]
  fn test_copy_tuple_layout_with_token_and_opaque() {
    let s1 = make_shape_with_layout(
      PrimitiveType::F32, vec![2, 3],
      vec![0, 1], vec![]);
    let s2 = make_shape_with_layout(
      PrimitiveType::F32, vec![42, 123],
      vec![1, 0], vec![]);
    let s3 = ShapeUtil::make_token_shape();

    let s4 = ShapeUtil::make_opaque_shape();
    let s5 = make_shape_with_layout(
      PrimitiveType::F32, vec![],
      vec![], vec![]);
    let s6 = make_shape_with_layout(
      PrimitiveType::F32, vec![1, 2, 3],
      vec![0, 2, 1], vec![]);
    let s7 = ShapeUtil::make_tuple_shape(vec![s4, s5, s6]);

    let src = ShapeUtil::make_tuple_shape(vec![s1, s2, s3, s7]);

    let d1 = make_shape_with_layout(
      PrimitiveType::F32, vec![2, 3],
      vec![1, 0], vec![]);
    let d2 = make_shape_with_layout(
      PrimitiveType::F32, vec![42, 123],
      vec![1, 0], vec![]);
    let d3 = ShapeUtil::make_token_shape();

    let d4 = ShapeUtil::make_opaque_shape();
    let d5 = make_shape_with_layout(
      PrimitiveType::F32, vec![],
      vec![], vec![]);
    let d6 = make_shape_with_layout(
      PrimitiveType::F32, vec![1, 2, 3],
      vec![1, 2, 0], vec![]);
    let d7 = ShapeUtil::make_tuple_shape(vec![d4, d5, d6]);

    let mut dst = ShapeUtil::make_tuple_shape(vec![d1, d2, d3, d7]);

    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), false);
    LayoutUtil::copy_layout_between_shapes(&src, &mut dst);
    assert_eq!(LayoutUtil::layouts_in_shapes_equal(&src, &dst), true);
  }

  #[test]
  fn test_clear_layout_tuple() {
    let s1 = make_shape_with_layout(
      PrimitiveType::F32, vec![2, 3],
      vec![1, 0], vec![]);
    let s2 = make_shape_with_layout(
      PrimitiveType::F32, vec![42, 123],
      vec![1, 0], vec![]);

    let s3 = make_shape_with_layout(
      PrimitiveType::F32, vec![],
      vec![], vec![]);
    let s4 = make_shape_with_layout(
      PrimitiveType::F32, vec![1, 2, 3],
      vec![1, 2, 0], vec![]);
    let s5 = ShapeUtil::make_tuple_shape(vec![s3, s4]);

    let mut shape = ShapeUtil::make_tuple_shape(vec![s1, s2, s5]);

    assert_eq!(LayoutUtil::has_layout(&shape), true);
    assert_eq!(shape.tuple_shapes(0).has_layout(), true);
    assert_eq!(shape.tuple_shapes(2).tuple_shapes(1).has_layout(), true);

    LayoutUtil::clear_layout(&mut shape);
    assert_eq!(LayoutUtil::has_layout(&shape), false);
    assert_eq!(shape.tuple_shapes(0).has_layout(), false);
    assert_eq!(shape.tuple_shapes(2).tuple_shapes(1).has_layout(), false);
  }

  #[test]
  fn test_clear_layout_opaque_and_token() {
    let mut opaque = ShapeUtil::make_opaque_shape();
    assert_eq!(LayoutUtil::has_layout(&opaque), true);
    LayoutUtil::clear_layout(&mut opaque);
    assert_eq!(LayoutUtil::has_layout(&opaque), true);

    let mut token = ShapeUtil::make_token_shape();
    assert_eq!(LayoutUtil::has_layout(&token), true);
    LayoutUtil::clear_layout(&mut token);
    assert_eq!(LayoutUtil::has_layout(&token), true);
  }

  #[test]
  fn test_set_to_default_layout_tuple() {
    let s1 = make_shape_with_layout(
      PrimitiveType::F32, vec![2, 3, 4],
      vec![1, 0, 2], vec![]);
    let s2 = make_shape_with_layout(
      PrimitiveType::F32, vec![42, 123, 7],
      vec![1, 2, 0], vec![]);

    let s3 = make_shape_with_layout(
      PrimitiveType::F32, vec![],
      vec![], vec![]);
    let s4 = make_shape_with_layout(
      PrimitiveType::F32, vec![1, 2, 3, 4],
      vec![3, 1, 2, 0], vec![]);
    let s5 = ShapeUtil::make_tuple_shape(vec![s3, s4]);

    let mut shape = ShapeUtil::make_tuple_shape(vec![s1, s2, s5]);
    assert_eq!(LayoutEqual::new().equal(
      shape.tuple_shapes(0).layout().as_ref().unwrap(),
      shape.tuple_shapes(1).layout().as_ref().unwrap()),
      false
    );
    LayoutUtil::set_to_default_layout(&mut shape);
    assert_eq!(LayoutEqual::new().equal(
      shape.tuple_shapes(0).layout().as_ref().unwrap(),
      shape.tuple_shapes(1).layout().as_ref().unwrap()),
      true
    );
    assert_eq!(LayoutEqual::new().equal(
      &LayoutUtil::get_default_layout_for_shape(shape.tuple_shapes(0)),
      shape.tuple_shapes(1).layout().as_ref().unwrap()),
      true
    );
  }

  #[test]
  fn test_default_layout_getter_major_to_minor() {
    let layout_r2 = LayoutUtil::make_layout(
      vec![1, 0], vec![], vec![],
      vec![], vec![], 1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,0,
      0, None, 0);
    assert_eq!(LayoutEqual::new().equal(
      &layout_r2, &LayoutUtil::get_default_layout_for_r2()), true);

    let layout_r3 = LayoutUtil::make_layout(
      vec![2, 1, 0], vec![], vec![],
      vec![], vec![], 1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,0,
      0, None, 0);
    assert_eq!(LayoutEqual::new().equal(
      &layout_r3, &LayoutUtil::get_default_layout_for_r3()), true);

    let layout_r4 = LayoutUtil::make_layout(
      vec![3, 2, 1, 0], vec![], vec![],
      vec![], vec![], 1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,0,
      0, None, 0);
    assert_eq!(LayoutEqual::new().equal(
      &layout_r4, &LayoutUtil::get_default_layout_for_r4()), true);

    let layout_r5 = LayoutUtil::make_layout(
      vec![4, 3, 2, 1, 0], vec![], vec![],
      vec![], vec![], 1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,0,
      0, None, 0);
    assert_eq!(LayoutEqual::new().equal(
      &layout_r5,
      &LayoutUtil::get_default_layout_for_shape(
        &ShapeUtil::make_shape(&PrimitiveType::F32, vec![10, 20, 30, 15, 25]))),
      true);
  }

  #[test]
  fn test_make_descending() {
    assert_eq!(LayoutEqual::new().equal(
      &LayoutUtil::make_descending_layout(5),
      &LayoutUtil::make_layout(vec![4, 3, 2, 1, 0], vec![], vec![], vec![],
        vec![], 1, PrimitiveType::Invalid, PrimitiveType::Invalid, 0, 0, None, 0)),
      true);

    assert_eq!(LayoutEqual::new().equal(
      &LayoutUtil::make_descending_layout(1),
      &LayoutUtil::make_layout(vec![0], vec![], vec![], vec![],
        vec![], 1, PrimitiveType::Invalid, PrimitiveType::Invalid, 0, 0, None, 0)),
      true);

    assert_eq!(LayoutEqual::new().equal(
      &LayoutUtil::make_descending_layout(0),
      &LayoutUtil::make_layout(vec![], vec![], vec![], vec![],
        vec![], 1, PrimitiveType::Invalid, PrimitiveType::Invalid, 0, 0, None, 0)),
      true);
  }

  #[test]
  fn test_make_ascending() {
    assert_eq!(LayoutEqual::new().equal(
      &LayoutUtil::make_ascending_layout(5),
      &LayoutUtil::make_layout(vec![0, 1, 2, 3, 4], vec![], vec![], vec![],
        vec![], 1, PrimitiveType::Invalid, PrimitiveType::Invalid, 0, 0, None, 0)),
      true);

    assert_eq!(LayoutEqual::new().equal(
      &LayoutUtil::make_ascending_layout(1),
      &LayoutUtil::make_layout(vec![0], vec![], vec![], vec![],
        vec![], 1, PrimitiveType::Invalid, PrimitiveType::Invalid, 0, 0, None, 0)),
      true);

    assert_eq!(LayoutEqual::new().equal(
      &LayoutUtil::make_ascending_layout(0),
      &LayoutUtil::make_layout(vec![], vec![], vec![], vec![],
        vec![], 1, PrimitiveType::Invalid, PrimitiveType::Invalid, 0, 0, None, 0)),
      true);
  }

  fn test_human_string_with_tiling() {}

  #[test]
  fn test_validate_layout_valid_array_layout() {
    let shape = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32, vec![2, 3], vec![0, 1],
      vec![], 1,
      0, 0);

    assert_eq!(LayoutUtil::validate_layout_in_shape(&shape, false), Ok(()));
    assert_eq!(LayoutUtil::validate_layout_in_shape(&shape, true), Ok(()));
  }

  #[test]
  fn test_validate_layout_invalid_array_layout() {
    let mut shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![2, 3]);
    let layout = LayoutUtil::make_layout(
      vec![0, 1, 2], vec![], vec![],
      vec![], vec![], 1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,
      0, 0,
      None, 0);

    shape.set_layout(layout);
    let result =
      LayoutUtil::validate_layout_in_shape(&shape, false);
    assert_eq!(result.err(),
      Some("Layout minor_to_major size is not same as shape's rank.".to_string()));

    let result =
      LayoutUtil::validate_layout_in_shape(&shape, true);
    assert_eq!(result.err(),
      Some("Layout minor_to_major size is not same as shape's rank.".to_string()));
  }

  #[test]
  fn test_validate_layout_invalid_dim_level_types() {
    let mut shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![2, 3]);
    let layout = LayoutUtil::make_layout(
      vec![0, 1], vec![], vec![],
      vec![], vec![], 1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,
      0, 0,
      None, 0);
    
    shape.set_layout(layout);
    let l = shape.mutable_layout().as_mut().unwrap();
    l.add_dim_level_type(DimLevelType::Dense);
    l.add_dim_level_type(DimLevelType::Dense);
    l.add_dim_level_type(DimLevelType::Dense);

    let result =
      LayoutUtil::validate_layout_in_shape(&shape, false);
    assert_eq!(result.err(),
      Some("Layout dim_level_types size is not same as shape's rank.".to_string()));

    let result =
      LayoutUtil::validate_layout_in_shape(&shape, true);
    assert_eq!(result.err(),
      Some("Layout dim_level_types size is not same as shape's rank.".to_string()));
  }

  #[test]
  fn test_validate_layout_missing_array_layout() {
    let mut shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![2, 3]);
    LayoutUtil::clear_layout(&mut shape);

    let result =
      LayoutUtil::validate_layout_in_shape(&shape, false);
    assert_eq!(result.err(), Some("Shape does not have a layout.".to_string()));

    let result =
      LayoutUtil::validate_layout_in_shape(&shape, true);
    assert_eq!(result.err(), None);
  }

  #[test]
  fn test_validate_layout_sparse() {
    let mut shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![2, 3]);
    let layout =
      LayoutUtil::make_layout(vec![1, 0],
      vec![DimLevelType::Dense, DimLevelType::Compressed],
      vec![], vec![],
      vec![Tile::new(vec![10, 10])], 1, 
      PrimitiveType::Invalid, 
      PrimitiveType::Invalid,
      0, 0,
      None, 0);
    shape.set_layout(layout);
    
    let result =
      LayoutUtil::validate_layout_in_shape(&shape, false);
    assert_eq!(result.err(),
      Some("Layout has tiles, but the shape is a sparse array.".to_string()));
    shape.mutable_layout().as_mut().unwrap()
      .clear_tiles();
    assert_eq!(LayoutUtil::validate_layout_in_shape(&shape, false), Ok(()));

    let s1 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![6]);
    shape.mutable_layout().as_mut().unwrap()
      .set_physical_shape(s1);
    assert_eq!(LayoutUtil::validate_layout_in_shape(&shape, false), Ok(()));

    let s2 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![10]);
    shape.mutable_layout().as_mut().unwrap()
      .mutable_physical_shape().as_mut().unwrap()
      .mutable_layout().as_mut().unwrap()
      .set_physical_shape(s2);
    let result =
      LayoutUtil::validate_layout_in_shape(&shape, false);
    assert_eq!(result.err(),
      Some("Layout has a physical_shape, but is not a sparse array.".to_string()));

    shape.mutable_layout().as_mut().unwrap()
      .mutable_physical_shape().as_mut().unwrap()
      .clear_layout();
    shape.mutable_layout().as_mut().unwrap()
      .clear_dim_level_types();
    let result =
    LayoutUtil::validate_layout_in_shape(&shape, false);
    assert_eq!(result.err(),
      Some("Layout has a physical_shape, but is not a sparse array.".to_string()));

    let layout2 =
      LayoutUtil::make_layout(vec![1, 0],
      vec![DimLevelType::Dense, DimLevelType::Dense],
      vec![true, false], vec![],
      vec![], 1, 
      PrimitiveType::Invalid, 
      PrimitiveType::Invalid,
      0, 0,
      None, 0);
    shape.set_layout(layout2);

    let result =
    LayoutUtil::validate_layout_in_shape(&shape, false);
    assert_eq!(result.err(),
      Some("Layout dimension has invalid level encoding.".to_string()));
  }

  #[test]
  fn test_validate_layout_tuple_subshapes_with_missing_layouts() {
    let sub_1_1_1 =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 2]);
    let sub_1_1 = ShapeUtil::make_tuple_shape(vec![sub_1_1_1]);
    let mut sub_1_2 =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 2]);
    LayoutUtil::clear_layout(&mut sub_1_2);
    let sub_1 = ShapeUtil::make_tuple_shape(vec![sub_1_1, sub_1_2]);
    let mut sub_2_1 =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![9]);
    LayoutUtil::clear_layout(&mut sub_2_1);
    let sub_2 = ShapeUtil::make_tuple_shape(vec![sub_2_1]);
    let mut shape = ShapeUtil::make_tuple_shape(vec![sub_1, sub_2]);

    let mut result =
      LayoutUtil::validate_layout_in_shape(&shape, false);
    assert_eq!(result.err(), Some("Shape does not have a layout.".to_string()));

    result = LayoutUtil::validate_layout_in_shape(&shape, true);
    assert_eq!(result, Ok(()));

    shape.mutable_tuple_shapes(1).mutable_tuple_shapes(0)
      .set_layout(LayoutUtil::make_layout(vec![0, 2, 3],
        vec![], vec![], vec![],
        vec![], 1,
        PrimitiveType::Invalid,
        PrimitiveType::Invalid,
        0, 0,
        None, 0));

    result = LayoutUtil::validate_layout_in_shape(&shape, true);
    assert_eq!(result.err(),
      Some("Layout minor_to_major size is not same as shape's rank.".to_string()));
  }

  #[test]
  fn test_move_dim_to_major() {
    let mut layout = LayoutUtil::make_layout(vec![2, 1, 0],
      vec![], vec![], vec![],
      vec![], 1, 
      PrimitiveType::Invalid, 
      PrimitiveType::Invalid,
      0, 0,
      None, 0);
    let layout_clone = layout.clone();

    let new_layout = LayoutUtil::move_dim_to_major(&mut layout, 0);
    assert_eq!(LayoutEqual::new().equal(&new_layout, &layout_clone), true);

    let new_layout2 = LayoutUtil::move_dim_to_major(&mut layout, 1);
    assert_eq!(LayoutEqual::new().equal(
      &new_layout2,
      &LayoutUtil::make_layout(vec![2, 0, 1],
        vec![], vec![], vec![], vec![], 1, 
        PrimitiveType::Invalid, PrimitiveType::Invalid,
        0, 0, None, 0)),
        true);
  }

  #[test]
  fn test_strides_is_major_to_minor() {
    let byte_strides = vec![3960, 440, 44, 4];
    assert_eq!(LayoutUtil::byte_strides_is_major_to_minor(
      byte_strides, vec![8, 9, 10, 11], &PrimitiveType::F32), true);
  }

  #[test]
  fn test_strides_not_major_to_minor_inner_most_stride_incorrect() {
    let byte_strides = vec![1880, 220, 22, 2];
    assert_eq!(LayoutUtil::byte_strides_is_major_to_minor(
      byte_strides, vec![8, 9, 10, 11], &PrimitiveType::F32), false);
  }

  #[test]
  fn test_strides_not_major_to_minor() {
    let byte_strides = vec![1880, 440, 44, 4];
    assert_eq!(LayoutUtil::byte_strides_is_major_to_minor(
      byte_strides, vec![8, 9, 10, 11], &PrimitiveType::F32), false);
  }

  #[test]
  fn test_has_custom_element_size_in_bits() {
    let mut shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 2]);
    assert_eq!(LayoutUtil::has_custom_element_size_in_bits(&shape), false);

    shape.mutable_layout().as_mut().unwrap().set_element_size_in_bits(0);
    assert_eq!(LayoutUtil::has_custom_element_size_in_bits(&shape), false);

    shape.mutable_layout().as_mut().unwrap().set_element_size_in_bits(32);
    assert_eq!(LayoutUtil::has_custom_element_size_in_bits(&shape), true);

    let s1 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 2]);
    let s2 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 2]);
    let s3 = ShapeUtil::make_tuple_shape(vec![s1, s2]);
    let s4 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 2]);
    let mut tuple_shape = ShapeUtil::make_tuple_shape(vec![s3, s4]);
    assert_eq!(LayoutUtil::has_custom_element_size_in_bits(&tuple_shape), false);

    ShapeUtil::get_mutable_subshape(&mut tuple_shape, vec![0, 1])
      .mutable_layout().as_mut().unwrap()
      .set_element_size_in_bits(32);
    assert_eq!(LayoutUtil::has_custom_element_size_in_bits(&tuple_shape), true);
  }
}