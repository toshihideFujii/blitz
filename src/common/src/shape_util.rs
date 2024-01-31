#![allow(dead_code)]

use crate::{
  blitz_data::PrimitiveType,
  shape::Shape,
  primitive_util, overflow_util, layout_util::LayoutUtil,
};

pub struct ShapeUtil {}

impl ShapeUtil {
  pub fn elements_in() {}
  pub fn elements_in_recursive() {}

  pub fn has_primitive_type(shape: &Shape, primitive_type: &PrimitiveType) -> bool {
    if &shape.element_type() == primitive_type {
      return true;
    }
    for elt_shape in shape.tuple_shapes_vec() {
      if ShapeUtil::has_primitive_type(elt_shape, primitive_type) {
        return true;
      }
    }
    false
  }

  pub fn is_zero_element_array() {}

  pub fn byte_size_of(shape: &Shape, pointer_size: usize) -> i64 {
    ShapeUtil::validate_shape_with_optional_layout(shape);
    if shape.element_type() == PrimitiveType::Tuple {
      return ShapeUtil::byte_size_of_tuple_index_table(shape, pointer_size);
    } else if shape.is_array() {
      return ShapeUtil::byte_size_of_elements(shape);
    } else if shape.element_type() == PrimitiveType::Token {
      return 0;
    } else if shape.element_type() == PrimitiveType::OpaqueType {
      assert!(pointer_size > 0);
      return pointer_size as i64;
    }
    unreachable!("Primitive type has no definitive size.");
  }

  pub fn byte_size_of_primitive_type(primitive_type: &PrimitiveType) -> i64 {
    primitive_util::byte_width(primitive_type)
  }

  pub fn byte_size_of_tuple_index_table(shape: &Shape, pointer_size: usize) -> i64 {
    ShapeUtil::validate_shape(shape);
    assert!(shape.element_type() == PrimitiveType::Tuple);
    assert!(pointer_size > 0);
    (pointer_size * shape.tuple_shapes_size()) as i64
  }

  pub fn byte_size_of_elements(_shape: &Shape) -> i64 { 0 }
  pub fn print_human_string() {}
  pub fn print_human_string_with_layout() {}
  pub fn human_string() {}
  pub fn human_string_with_layout() {}

  pub fn same_dimensions(lhs: &Shape, rhs: &Shape) -> bool {
    assert!(lhs.is_array());
    assert!(rhs.is_array());
    if !ShapeUtil::same_rank(lhs, rhs) {
      return false;
    }
    for i in 0..lhs.rank() {
      if !lhs.is_unbounded_dynamic_dimension(i) &&
        !rhs.is_unbounded_dynamic_dimension(i) &&
        !lhs.dimensions(i) != rhs.dimensions(i) {
          return false;
        }
    }
    true
  }

  pub fn same_rank(lhs: &Shape, rhs: &Shape) -> bool {
    assert!(lhs.is_array());
    assert!(rhs.is_array());
    lhs.rank() == rhs.rank()
  }

  pub fn same_element_type(lhs: &Shape, rhs: &Shape) -> bool {
    lhs.element_type() == rhs.element_type()
  }

  pub fn same_element_type_ignoring_fp_precision(a: &Shape, b: &Shape) -> bool {
    if ShapeUtil::element_is_floating(a) &&
      ShapeUtil::element_is_floating(b) {
      return true;
    }
    ShapeUtil::same_element_type(a, b)
  }

  pub fn higher_precision_element_type(a: &Shape, b: &Shape) -> PrimitiveType {
    primitive_util::higher_precision_type(&a.element_type(), &b.element_type())
  }

  pub fn compatible() {}
  pub fn compatible_ignoring_element_type() {}
  pub fn compatible_kind() {}
  pub fn compatible_ignoring_fp_precision() {}

  pub fn true_rank(shape: &Shape) -> i64 {
    let mut acum: i64 = 0;
    for dimension in shape.dimensions_vec() {
      // Fo not count unit dimensions.
      if *dimension != 1 {
        acum += 1;
      }
    }
    acum
  }

  pub fn make_program_shape() {}

  pub fn is_scalar(shape: &Shape) -> bool {
    shape.is_array() && shape.rank() == 0
  }

  pub fn is_effective_scalar(shape: &Shape) -> bool {
    shape.is_array() && ShapeUtil::true_rank(shape) == 0
  }

  pub fn is_scalar_with_element_type(shape: &Shape, elt_t: PrimitiveType) -> bool {
    ShapeUtil::is_scalar(shape) && shape.element_type() == elt_t
  }

  pub fn create_dimension_vector_from_shape() {}

  pub fn get_dimension(shape: &Shape, dimension_number: i64) -> i64 {
    shape.dimensions(ShapeUtil::get_dimension_number(shape, dimension_number) as usize)
  }

  pub fn get_dimension_number(shape: &Shape, dimension_number: i64) -> i64 {
    let mut result = dimension_number;
    if result < 0 {
      result += shape.rank() as i64;
    }
    assert!(result >= 0);
    result
  }

  pub fn change_element_type(original: &Shape, t: &PrimitiveType) -> Shape {
    if original.is_tuple() {
      let mut new_operands = Vec::new();
      new_operands.reserve(original.tuple_shapes_size());
      for operand in original.tuple_shapes_vec() {
        new_operands.push(ShapeUtil::change_element_type(operand, t));
      }
      return ShapeUtil::make_tuple_shape(new_operands);
    } else {
      let mut new_shape = Shape::new_from(original);
      new_shape.set_element_type(t.clone());
      return new_shape;
    }
  }

  pub fn make_static_shape(original: &Shape) -> Shape {
    let mut result = Shape::new_from(original);
    result.clear_dynamic_dimensions();
    if result.has_layout() {
      result.mutable_lauout().as_mut().unwrap()
        .set_dynamic_shape_metadata_prefix_bytes(0);
    }
    result
  }

  pub fn make_tuple_shape(_shapes: Vec<Shape>) -> Shape {
    Shape::new_default() // TODO
  }

  pub fn make_tuple_shape_with_ptrs() {}

  pub fn make_maybe_tuple_shape(_shapes: Vec<Shape>) -> Shape {
    Shape::new_default() // TODO
  }

  pub fn make_opaque_shape() -> Shape {
    let mut result = Shape::new_default();
    result.set_element_type(PrimitiveType::OpaqueType);
    ShapeUtil::validate_shape_with_optional_layout(&result);
    result
  }

  pub fn make_token_shape() -> Shape {
    let mut result = Shape::new_default();
    result.set_element_type(PrimitiveType::Token);
    ShapeUtil::validate_shape_with_optional_layout(&result);
    result
  }

  pub fn append_shape_to_tuple(shape: Shape, tuple_shape: &mut Shape) {
    ShapeUtil::validate_shape_with_optional_layout(&shape);
    tuple_shape.add_tuple_shapes(shape);
  }

  pub fn update_tuple_shape(shape: Shape, index: usize, tuple_shape: &mut Shape) {
    assert!(index < tuple_shape.tuple_shapes_size());
    tuple_shape.set_tuple_shapes(index, shape);
  }

  pub fn update_dynamic_dimension() {}

  pub fn append_major_dimension(bound: i64, shape: &mut Shape) {
    assert!(LayoutUtil::is_dense_array(shape));
    if shape.has_layout() {
      let rank = shape.rank() as i64;
      shape.mutable_lauout().as_mut().unwrap().add_minor_to_major(rank);
    }
    shape.add_dimensions(bound);
    ShapeUtil::validate_shape(shape);
  }

  pub fn prepend_major_dimension() {}

  pub fn append_minor_dimension(bound: i64, shape: &mut Shape) {
    assert!(LayoutUtil::is_dense_array(shape));
    shape.add_dimensions(bound);
    if shape.has_layout() {
      let rank = shape.rank() as i64;
      let layout = shape.mutable_lauout().as_mut().unwrap();
      layout.add_minor_to_major(0);
      let dim_idx = layout.minor_to_major_size() - 2;
      for i in dim_idx..0 {
        let layout_idx = layout.minor_to_major(i);
        layout.set_minor_to_major(i + 1, layout_idx);
      }
      layout.set_minor_to_major(0, rank - 1);
    }
    ShapeUtil::validate_shape(shape);
  }

  pub fn copy_dynamic_dimensions() {}

  pub fn is_eeffectively_most_major_dimension(shape: &Shape, dimension: i64) -> bool {
    for i in 0..shape.dimensions_size() {
      let major_dimension =
        LayoutUtil::major(shape.layout().as_ref().unwrap(),
          i as i64);
      if major_dimension == dimension {
        return true;
      }
      if shape.dimensions(major_dimension as usize) != 1 {
        return false;
      }
    }
    false
  }

  pub fn make_nil() -> Shape {
    ShapeUtil::make_tuple_shape(vec![])
  }

  pub fn is_initialized(shape: &Shape) -> bool {
    shape.element_type() != PrimitiveType::Invalid
  }

  pub fn make_shape(elt_t: &PrimitiveType, dimensions: Vec<i64>) -> Shape {
    let mut shape = Shape::new_default();
    assert!(ShapeUtil::fill_new_shape(elt_t, &dimensions, &mut shape));
    shape
  }

  pub fn make_scalar_shape(elt_t: &PrimitiveType) -> Shape {
    ShapeUtil::make_shape(elt_t, vec![])
  }

  pub fn make_validated_shape(elt_t: &PrimitiveType, dimensions: Vec<i64>) -> Shape {
    let mut shape = Shape::new_default();
    if !ShapeUtil::fill_new_shape(elt_t, &dimensions, &mut shape) {
      panic!("Invalid shape type, dims.");
    }
    shape
  }

  pub fn make_shape_with_type() {}
  pub fn make_shape_with_dense_layout() {}
  pub fn move_dim_to_major() {}

  pub fn make_shape_with_static_dimensions(shape: &Shape) -> Shape {
    let mut output = Shape::new_from(shape);
    output.clear_dynamic_dimensions();
    output
  }

  pub fn make_shape_with_descending_layout() {}
  pub fn make_shape_with_descending_layout_and_same_physical_layout() {}

  pub fn populate_shape(elt_t: &PrimitiveType, dimensions: Vec<i64>, shape: &mut Shape) {
    shape.clear();
    shape.set_element_type(elt_t.clone());
    for dimension in dimensions {
      shape.add_dimensions(dimension);
    }
    LayoutUtil::set_to_default_layout(shape);
    ShapeUtil::validate_shape(shape);
  }

  pub fn validate_shape(_shape: &Shape) {}

  pub fn validate_shape_with_optional_layout(_shape: &Shape) {}

  pub fn element_is_integral(shape: &Shape) -> bool {
    primitive_util::is_integral_type(&shape.element_type())
  }

  pub fn element_is_floating(shape: &Shape) -> bool {
    primitive_util::is_floating_point_type(&shape.element_type())
  }

  pub fn element_is_complex(shape: &Shape) -> bool {
    primitive_util::is_complex_type(&shape.element_type())
  }

  pub fn element_has_bit_width(shape: &Shape, bits: i64) -> bool {
    if !shape.is_array() {
      return false;
    }
    primitive_util::bit_width(&shape.element_type()) == bits
  }

  pub fn element_is_integral_with_bits(shape: &Shape, bits: i64) -> bool {
    ShapeUtil::element_is_integral(shape) &&
    ShapeUtil::element_has_bit_width(shape, bits)
  }

  pub fn element_is_signed(shape: &Shape) -> bool {
    primitive_util::is_signed_integral_type(&shape.element_type()) ||
    primitive_util::is_floating_point_type(&shape.element_type())
  }

  pub fn is_array_primitive_type(primitive_t: &PrimitiveType) -> bool {
    primitive_util::is_array_type(primitive_t)
  }

  pub fn is_nested_tuple(shape: &Shape) -> bool {
    if !shape.is_tuple() {
      return false;
    }
    for s in shape.tuple_shapes_vec() {
      if s.is_tuple() {
        return true;
      }
    }
    false
  }

  pub fn is_empty_tuple(shape: &Shape) -> bool {
    shape.is_tuple() && shape.tuple_shapes_vec().is_empty()
  }

  pub fn tuple_element_count(shape: &Shape) -> usize {
    shape.tuple_shapes_size()
  }

  pub fn get_tuple_element_shape(shape: &Shape, index: usize) -> &Shape {
    assert!(ShapeUtil::tuple_element_count(shape) >= index);
    ShapeUtil::validate_shape_with_optional_layout(shape.tuple_shapes(index));
    shape.tuple_shapes(index)
  }

  pub fn subshape_count() {}
  pub fn slice_tuple() {}
  pub fn complex_component_shape() {}

  pub fn index_is_valid(shape: &Shape, index_vec: Vec<i64>) -> bool {
    let mut subshape: &Shape = shape;
    for i in index_vec {
      if !subshape.is_tuple() || i as usize >= subshape.tuple_shapes_size() || i < 0 {
        return false;
      }
      subshape = subshape.tuple_shapes(i as usize);
    }
    true
  }

  pub fn get_subshape(shape: &Shape, index_vec: Vec<i64>) -> &Shape {
    let mut return_shape: &Shape = shape;
    for i in index_vec {
      assert!(return_shape.is_tuple(), "Invalid index for shape.");
      return_shape = return_shape.tuple_shapes(i as usize);
    }
    return_shape
  }

  pub fn try_get_subshape(shape: &Shape, index_vec: Vec<i64>) -> &Shape {
    let mut return_shape: &Shape = shape;
    for i in index_vec {
      if !return_shape.is_tuple() ||
        i < 0 ||
        i as usize >= return_shape.tuple_shapes_size() {
          panic!("Shape index {} is not a valid subshape index for tuple shape.", i);
      }
      return_shape = return_shape.tuple_shapes(i as usize);
    }
    return_shape
  }

  pub fn is_leaf_index(shape: &Shape, index_vec: Vec<i64>) -> bool {
    !ShapeUtil::get_subshape(shape, index_vec).is_tuple()
  }

  pub fn get_leaf_count(shape: &Shape) -> usize {
    if !shape.is_tuple() {
      return 1;
    }
    let mut count = 0;
    for subshape in shape.tuple_shapes_vec() {
      count += ShapeUtil::get_leaf_count(subshape);
    }
    count
  }

  pub fn get_leaf_shapes() {}
  pub fn for_each_subshape() {}
  pub fn for_each_leaf_shape() {}
  pub fn for_each_subshape_with_status() {}
  pub fn for_each_subshape_post_order() {}
  pub fn for_each_subshape_post_order_with_status() {}

  pub fn has_degenerate_dimensions(shape: &Shape) -> bool {
    assert!(shape.is_array());
    for i in shape.dimensions_vec() {
      if *i == 1 { return true; }
    }
    false
  }

  pub fn drop_degenerate_dimensions() {}
  pub fn permute_dimensions() {}
  pub fn inserted_or_deleted_sized_dimensions() {}
  pub fn dimensions_unmodified_by_reshape() {}
  pub fn rehsape_leaves_dimensions_unmodified() {}
  pub fn transpose_is_bitcast() {}
  pub fn reshape_is_bitcast() {}
  pub fn is_reshape_or_transpose_bitcast() {}
  pub fn deduce_transpose_dimensions_for_bitcast() {}
  pub fn decompose_bitcast_to_trt() {}
  pub fn decompose_bitcast() {}
  pub fn align_layouts() {}

  pub fn delete_dimension(dim_to_delete: i64, shape: &mut Shape) {
    assert!(shape.is_array());
    shape.delete_dimension(dim_to_delete);
  }

  pub fn delete_dimensions(dims_to_delete: Vec<i64>, shape: &mut Shape) {
    for dim in dims_to_delete {
      ShapeUtil::delete_dimension(dim, shape);
    }
  }

  pub fn filter_dimensions(p: fn(i64) -> bool, shape: &mut Shape) {
    assert!(shape.is_array());
    let mut dims_to_delete: Vec<i64> = Vec::new();
    for i in shape.dimensions_vec().len() - 1..0 {
      if !p(i as i64) {
        dims_to_delete.push(i as i64);
      }
    }
    ShapeUtil::delete_dimensions(dims_to_delete, shape);
  }

  pub fn dynamic_array_shape_is_compatible(
    dynamic_shape: &Shape,
    bounded_shape: &Shape) -> bool
  {
    if dynamic_shape.rank() != bounded_shape.rank() {
      return false;
    }
    for i in 0..dynamic_shape.rank() {
      if dynamic_shape.dimensions(i) > bounded_shape.dimensions(i) {
        return false;
      }
    }
    true
  }

  pub fn dynamic_shape_is_compatible() {}
  pub fn for_each_index_with_status() {}
  pub fn for_each_index() {}
  pub fn for_each_index_no_status() {}
  pub fn for_each_index_parallel() {}
  pub fn get_for_each_index_parallel_thread_count() {}
  pub fn for_each_index_parallel_with_status() {}
  pub fn get_normalized_transpose_shape() {}
  pub fn get_normalized_logical_transpose_shape() {}
  pub fn device_shape_to_host_shape() {}

  pub fn element_can_upcast(from: &Shape, to: &Shape) -> bool {
    ShapeUtil::higher_precision_element_type(from, to) == to.element_type()
  }

  pub fn byte_strides(shape: &Shape, mut strides: Vec<i64>) {
    assert!(shape.is_array());
    assert!(shape.has_layout());
    assert!(shape.dimensions_size() == strides.len());

    let mut stride = ShapeUtil::byte_size_of_primitive_type(&shape.element_type());
    for i in shape.layout().as_ref().unwrap().minor_to_major_vec() {
      strides[*i as usize] = stride;
      stride *= shape.dimensions(*i as usize);
    }
  }

  pub fn array_size() {}
  pub fn array_data_size() {}

  fn fill_new_shape(
    elt_t: &PrimitiveType,
    dimensions: &Vec<i64>,
    shape: &mut Shape) -> bool
  {
    let mut dense_shape_size = 0;
    if primitive_util::is_array_type(elt_t) {
      dense_shape_size = primitive_util::byte_width(elt_t);
    }

    if dense_shape_size <= 0 {
      return false;
    }

    shape.set_element_type(elt_t.clone());
    let ndims = dimensions.len();
    let mut is_unbounded_dynamic = false;
    for dim in dimensions {
      if *dim == Shape::UNBOUNDED_SIZE {
        is_unbounded_dynamic = true;
        break;
      }
    }

    for i in 0..ndims {
      let d = dimensions[i];
      if d < 0 && d != Shape::UNBOUNDED_SIZE {
        return false;
      }
      if !is_unbounded_dynamic {
        dense_shape_size =
          overflow_util::multiply_without_overflow(dense_shape_size, d);
        if dense_shape_size < 0 {
          return false;
        }
      }
      shape.add_dimensions(d);
      shape.add_minor_to_major((ndims as i64)- 1 - (i as i64));
    }

    true
  }
}