#![allow(dead_code)]

use crate::{
  blitz_data::{PrimitiveType, DimLevelType},
  shape::{Shape, ProgramShape, ShapeEqual,},
  primitive_util, overflow_util,
  layout_util::LayoutUtil,
  printer::{StringPrinter, Printer}, layout::Tile, util::DimensionVector,
};

pub struct ShapeUtil {}

impl ShapeUtil {
  const ANNOTATION_PRINT_INTERVAL: usize = 5;

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

  fn print_tuple_shapes(printer: &mut dyn Printer, tuple_shapes: &Vec<Shape>, print_layout: bool) {
    if tuple_shapes.is_empty() {
      printer.append(&"()".to_string());
      return;
    }
    printer.append(&"(".to_string());
    ShapeUtil::print_shape(printer, &tuple_shapes[0], print_layout);
    for i in 1..tuple_shapes.len() {
      if i % ShapeUtil::ANNOTATION_PRINT_INTERVAL == 0 {
        let mut str = ", /*index=".to_string();
        str.push_str(&i.to_string());
        str.push_str(&"*/".to_string());
        printer.append(&str);
      } else {
        printer.append(&", ".to_string());
      }
      ShapeUtil::print_shape(printer, &tuple_shapes[i], print_layout);
    }
    printer.append(&")".to_string());
  }

  fn print_shape(printer: &mut dyn Printer, shape: &Shape, print_layout: bool) {
    if print_layout {
      ShapeUtil::print_human_string_with_layout(printer, shape);
    } else {
      ShapeUtil::print_human_string(printer, shape);
    }
  }

  pub fn print_human_string(printer: &mut dyn Printer, shape: &Shape) {
    if shape.is_tuple() {
      ShapeUtil::print_tuple_shapes(printer, &shape.tuple_shapes_vec(), false);
      return;
    }
    let primtive_type_name =
      primitive_util::lowercase_primitive_type_name(&shape.element_type());
    printer.append(&primtive_type_name);
    if shape.dimensions_vec().is_empty() {
      printer.append(&"[]".to_string());
      return;
    }
    printer.append(&"[".to_string());
    let print_one = |printer: &mut dyn Printer, i| {
      if shape.is_dynamic_dimension(i) {
        if shape.dimensions(i) != Shape::UNBOUNDED_SIZE {
          let mut str = "<=".to_string();
          str.push_str(&shape.dimensions(i).to_string());
          printer.append(&str);
        } else {
          printer.append(&"?".to_string());
        }
      } else {
        printer.append(&shape.dimensions(i).to_string())
      }
    };
    print_one(printer, 0);
    for i in 1..shape.dimensions_vec().len() {
      printer.append(&",".to_string());
      print_one(printer, i);
    }
    printer.append(&"]".to_string());
  }

  pub fn print_human_string_with_layout(printer: &mut dyn Printer, shape: &Shape) {
    if shape.is_tuple() {
      ShapeUtil::print_tuple_shapes(printer, shape.tuple_shapes_vec(), true);
      return;
    }
    ShapeUtil::print_human_string(printer, shape);
    if !shape.has_layout() {
      return;
    }
    if ShapeUtil::is_scalar(shape) {
      let layout_str =
        LayoutUtil::human_string(shape.layout().as_ref().unwrap());
      if layout_str != "{}".to_string() {
        printer.append(&layout_str);
      }
    } else if shape.is_array() {
      LayoutUtil::print_human_string(printer, shape.layout().as_ref().unwrap());
    }
  }

  pub fn print_human_string_for_program_shape(
    printer: &mut dyn Printer,
    program_shape: &ProgramShape)
  {
    printer.append(&"(".to_string());
    let shape_params = program_shape.parameters_vec();
    if !shape_params.is_empty() {
      let print_one = |printer: &mut dyn Printer, i| {
        if i < program_shape.parameter_names_size() {
          printer.append(&program_shape.parameter_names(i));
        } else {
          printer.append(&"(unknown)".to_string());
        }
        printer.append(&": ".to_string());
        ShapeUtil::print_human_string(printer, &shape_params[i])
      };
      print_one(printer, 0);
      for i in 1..shape_params.len() {
        printer.append(&", ".to_string());
        print_one(printer, i);
      }
    }
    printer.append(&") -> ".to_string());
    ShapeUtil::print_human_string(printer, program_shape.result());
  }

  pub fn human_string(shape: &Shape) -> String {
    let mut printer = StringPrinter::new();
    ShapeUtil::print_human_string(&mut printer, shape);
    printer.to_string()
  }

  pub fn human_string_with_layout(shape: &Shape) -> String {
    let mut printer = StringPrinter::new();
    ShapeUtil::print_human_string_with_layout(&mut printer, shape);
    printer.to_string()
  }

  pub fn human_string_for_program_shape(program_shape: &ProgramShape) -> String {
    let mut printer = StringPrinter::new();
    ShapeUtil::print_human_string_for_program_shape(&mut printer, program_shape);
    printer.to_string()
  }

  // Returns whether the lhs and rhs shapes have the same dimensions, ignoring
  // the unbounded dimension sizes; note: does not check element type.
  pub fn same_dimensions(lhs: &Shape, rhs: &Shape) -> bool {
    assert!(lhs.is_array());
    assert!(rhs.is_array());
    if !ShapeUtil::same_rank(lhs, rhs) {
      return false;
    }
    for i in 0..lhs.rank() {
      if !lhs.is_unbounded_dynamic_dimension(i) &&
         !rhs.is_unbounded_dynamic_dimension(i) &&
          lhs.dimensions(i) != rhs.dimensions(i) {
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

  // Return true if the rank, dimension sizes, and element type are identical.
  // Layout is ignored. Tuple elements are compared recursively for compatibility.
  pub fn compatible(lhs: &Shape, rhs: &Shape) -> bool {
    ShapeEqual::new()
      .ignore_dynamic_dimension()
      .ignore_layout()
      .equal(lhs, rhs)
  }

  // Return true if the rank and dimension sizes are identical.
  // Element type and layout is ignored.
  // Tuple elements are compared recursively for compatibility.
  pub fn compatible_ignoring_element_type(lhs: &Shape, rhs: &Shape) -> bool {
    ShapeEqual::new()
      .ignore_dynamic_dimension()
      .ignore_element_type()
      .ignore_layout()
      .equal(lhs, rhs)
  }

  // Return true if the tuple tree shapes and leaf ranks are identical.
  // Leaf dimensions, element type, and layout are ignored.
  // Tuple elements are compared recursively for compatibility.
  pub fn compatible_kind(lhs: &Shape, rhs: &Shape) -> bool {
    ShapeEqual::new()
      .ignore_element_type()
      .ignore_layout()
      .ignore_dimensions()
      .ignore_dynamic_dimension()
      .equal(lhs, rhs)
  }

  // As compatible, but allow one of lhs and rhs to be BF16 while the other being F32.
  // Tuple elements are compared recursively for compatibility.
  pub fn compatible_ignoring_fp_precision(lhs: &Shape, rhs: &Shape) -> bool {
    ShapeEqual::new()
      .ignore_dynamic_dimension()
      .ignore_fp_precision()
      .ignore_layout()
      .equal(lhs, rhs)
  }

  pub fn equal() {}

  // As equal, but does not compare the element type.
  pub fn equal_ignoring_element_type(lhs: &Shape, rhs: &Shape) -> bool {
    let result = ShapeEqual::new().ignore_element_type().equal(lhs, rhs);
    if !result {
      println!("ShapeUtil::equal_ignoring_element_type differ.");
      println!("lhs={:?}, rhs={:?}", lhs.element_type(), rhs.element_type());
    }
    result
  }

  // As equal, but allow one of lhs and rhs to be F16 while the other is F32.
  pub fn equal_ignoring_fp_precision(lhs: &Shape, rhs: &Shape) -> bool {
    let result = ShapeEqual::new().ignore_fp_precision().equal(lhs, rhs);
    if !result {
      println!("ShapeUtil::equal_ignoring_fp_precision differ.");
      println!("lhs={:?}, rhs={:?}", lhs.element_type(), rhs.element_type());
    }
    result
  }

  pub fn equal_structure() {}

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

  // Creates a DimensionVector by copying dimensions from a given shape.
  pub fn create_dimension_vector_from_shape(shape: &Shape) -> DimensionVector {
    let mut dimensions: DimensionVector = Vec::new();
    dimensions.reserve(shape.dimensions_size());
    for i in 0..shape.dimensions_size() {
      dimensions.push(shape.dimensions(i));
    }
    dimensions
  }

  // Extracts the size of the shape's dimension at the dimension number.
  pub fn get_dimension(shape: &Shape, dimension_number: i64) -> i64 {
    shape.dimensions(ShapeUtil::get_dimension_number(shape, dimension_number) as usize)
  }

  // Resolves a dimension number, supporting negative indexing.
  // Negative indexing has similar semantics to Python.
  // For an N-dimensional array, dimension -1 is equivalent to dimension N-1,
  // -2 is equivalent to N-2, and so on.
  // This function always returns a positive dimension number for any given
  // dimension number (which itself can be negative).
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

  pub fn make_tuple_shape(shapes: Vec<Shape>) -> Shape {
    let mut result = Shape::new_default();
    result.set_element_type(PrimitiveType::Tuple);
    result.tuple_shapes_vec_mut().reserve(shapes.len());
    for shape in shapes {
      ShapeUtil::append_shape_to_tuple(shape, &mut result)
    }
    ShapeUtil::validate_shape_with_optional_layout(&result);
    result
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

  // Creates a token shape.
  // Values of this shape are used for ordering side-effecting operations.
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

  pub fn is_effectively_most_major_dimension(shape: &Shape, dimension: i64) -> bool {
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

  // Constructs a new shape with the given element type and sequence of dimensions.
  pub fn make_shape(elt_t: &PrimitiveType, dimensions: Vec<i64>) -> Shape {
    let mut shape = Shape::new_default();
    assert!(ShapeUtil::fill_new_shape(elt_t, &dimensions, &mut shape));
    shape
  }

  pub fn make_shape_dynamic(
    elt_t: &PrimitiveType,
    dimensions: Vec<i64>,
    dynamic_dimensions: Vec<bool>) -> Shape
  {
    ShapeUtil::make_validated_shape_dynamic(elt_t, dimensions, dynamic_dimensions)
  }

  pub fn make_scalar_shape(elt_t: &PrimitiveType) -> Shape {
    ShapeUtil::make_shape(elt_t, vec![])
  }

  // Constructs a new shape with the given element type and sequence of
  // dimensions. Method checks the element type is valid, the shape's
  // size fits in i64::max(), and dynamic size is not marked static.
  pub fn make_validated_shape(elt_t: &PrimitiveType, dimensions: Vec<i64>) -> Shape {
    let mut shape = Shape::new_default();
    if !ShapeUtil::fill_new_shape(elt_t, &dimensions, &mut shape) {
      assert!(false, "Invalid shape type={:?}, dims={:?}.", elt_t, dimensions);
    }
    shape
  }

  pub fn make_validated_shape_dynamic(
    elt_t: &PrimitiveType,
    dimensions: Vec<i64>,
    dynamic_dimensions: Vec<bool>) -> Shape
  {
    if dynamic_dimensions.len() != dimensions.len() {
      assert!(false, "Dynamic dimensions size {} did not match number of dimensions {}.",
        dynamic_dimensions.len(), dimensions.len());
    }
    let mut shape = Shape::new_default();
    if !ShapeUtil::fill_new_shape(elt_t, &dimensions, &mut shape) {
      assert!(false, "Invalid shape type={:?}, dims={:?}.",
        elt_t, dimensions);
    }
    for i in 0..dimensions.len() {
      shape.set_dynamic_dimension(i, dynamic_dimensions[i]);
      if shape.dimensions(i) == Shape::UNBOUNDED_SIZE && !dynamic_dimensions[i] {
        assert!(false, "Cannot make a dynamic dimension at dim={} as static.", i);
      }
    }
    shape
  }

  pub fn make_shape_with_type() {}

  fn make_shape_with_layout_internal(
    elt_t: &PrimitiveType,
    dimensions: Vec<i64>,
    minor_to_major: Vec<i64>,
    dim_level_types: Vec<DimLevelType>,
    dim_unique: Vec<bool>,
    dim_ordered: Vec<bool>,
    tiles: Vec<Tile>,
    index_primitive_t: &PrimitiveType,
    pointer_primitive_t: &PrimitiveType,
    mut elt_size_in_bits: i64,
    memory_space: i64,
    physical_shape: Option<Shape>) -> Shape
  {
    if dimensions.len() != minor_to_major.len() {
      assert!(false, "Dimensions size is {}, but layout size is {}",
        dimensions.len(), minor_to_major.len());
    }
    if *elt_t == PrimitiveType::OpaqueType || *elt_t == PrimitiveType::Tuple ||
      *elt_t == PrimitiveType::Token {
      assert!(false, "Unsupported element type.");
    }
    let mut shape = ShapeUtil::make_validated_shape(elt_t, dimensions);
    if elt_size_in_bits == (ShapeUtil::byte_size_of_primitive_type(elt_t) * 8) {
      elt_size_in_bits = 0;
    }
    let layout = LayoutUtil::make_layout(
      minor_to_major,
      dim_level_types,
      dim_unique,
      dim_ordered,
      tiles,
      index_primitive_t.clone(),
      pointer_primitive_t.clone(),
      elt_size_in_bits,
      memory_space,
      physical_shape,
      0
    );
    shape.set_layout(layout);
    shape
  }

  pub fn make_shape_with_dense_layout(
    elt_t: &PrimitiveType,
    dimensions: Vec<i64>,
    minor_to_major: Vec<i64>,
    tiles: Vec<Tile>,
    elt_size_in_bits: i64,
    memory_space: i64) -> Shape
  {
    ShapeUtil::make_shape_with_layout_internal(
      elt_t,
      dimensions,
      minor_to_major,
      Vec::new(),
      Vec::new(),
      Vec::new(),
      tiles,
      &PrimitiveType::Invalid,
      &PrimitiveType::Invalid,
      elt_size_in_bits,
      memory_space,
      None
    )
  }

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

  pub fn get_mutable_subshape(shape: &mut Shape, index_vec: Vec<i64>) -> &mut Shape {
    let mut return_shape: &mut Shape = shape;
    for i in index_vec {
      assert!(return_shape.is_tuple(), "Invalid index for shape.");
      return_shape = return_shape.mutable_tuple_shapes(i as usize);
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_dimension_helper_can_negative_index() {
    let matrix =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![2, 3]);
    assert_eq!(ShapeUtil::get_dimension(&matrix, -1), 3);
    assert_eq!(ShapeUtil::get_dimension(&matrix, -2), 2);
  }

  #[test]
  fn test_get_dimension_helper_example_in_documentation() {
    let shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 2, 3, 4]);
    assert_eq!(ShapeUtil::get_dimension(&shape, -1), 4);
  }

  fn test_negative_index_oob_fails() {
    // TODO
  }

  #[test]
  fn test_create_rank3_dimension_vector_from_shape() {
    let shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2, 7]);
    let dimensions = ShapeUtil::create_dimension_vector_from_shape(&shape);
    assert_eq!(dimensions, vec![3, 2, 7]);
  }

  #[test]
  fn test_rank1_dimension_indexing() {
    let shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3]);
    assert_eq!(shape.dimensions(0), 3);
  }

  #[test]
  fn test_rank2_dimension_indexing() {
    let shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    assert_eq!(shape.dimensions(1), 2);
    assert_eq!(shape.dimensions(0), 3);
  }

  #[test]
  fn test_rank3_dimension_indexing() {
    let shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2, 7]);
    assert_eq!(shape.dimensions(2), 7);
    assert_eq!(shape.dimensions(1), 2);
    assert_eq!(shape.dimensions(0), 3);
  }

  #[test]
  fn test_rank4_dimension_indexing() {
    let shape =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2, 7, 8]);
    assert_eq!(shape.dimensions(3), 8);  
    assert_eq!(shape.dimensions(2), 7);
    assert_eq!(shape.dimensions(1), 2);
    assert_eq!(shape.dimensions(0), 3);
  }

  #[test]
  fn test_compatible_identical_shapes() {
    let shape1 =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let shape2 =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    assert_eq!(ShapeUtil::compatible(&shape1, &shape2), true);
  }

  #[test]
  fn test_token_compatibility() {
    let token1 = ShapeUtil::make_token_shape();
    let token2 = ShapeUtil::make_token_shape();
    assert_eq!(ShapeUtil::compatible(&token1, &token2), true);

    let f32 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![]);
    assert_eq!(ShapeUtil::compatible(&token1, &f32), false);
    assert_eq!(ShapeUtil::compatible(&f32, &token1), false);

    let tuple1 = ShapeUtil::make_tuple_shape(vec![token1]);
    let tuple2 = ShapeUtil::make_tuple_shape(vec![token2]);
    assert_eq!(ShapeUtil::compatible(&tuple1, &tuple2), true);
  }

  fn test_tokens_equal_shapes() {}

  fn test_compatible_not_identical_shapes() {}

  #[test]
  fn test_compatible_ignoring_fp_precision() {
    let shape1 =
      ShapeUtil::make_shape(&PrimitiveType::BF16, vec![3, 2]);
    let shape2 =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    assert_eq!(ShapeUtil::compatible_ignoring_fp_precision(&shape1, &shape2), true);
  }

  #[test]
  fn test_incompatible_ignoring_fp_precision() {
    let shape1 =
      ShapeUtil::make_shape(&PrimitiveType::BF16, vec![3, 2]);
    let shape2 =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![2, 2]);
    assert_eq!(ShapeUtil::compatible_ignoring_fp_precision(&shape1, &shape2), false);
  }

  #[test]
  fn test_incompatible_different_element_shapes() {
    let shape1
      = ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let shape2
      = ShapeUtil::make_shape(&PrimitiveType::Pred, vec![3, 2]);
    assert_eq!(ShapeUtil::compatible(&shape1, &shape2), false);
  }

  #[test]
  fn test_equal_ignoring_fp_precision() {
    let shape1 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      vec![4, 3], vec![0, 1],
      Vec::new(), 0, 0);
    
    let shape2 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F16,
      vec![4, 3], vec![0, 1],
      Vec::new(), 0, 0);

    assert_eq!(ShapeUtil::equal_ignoring_fp_precision(&shape1, &shape2), true);
  }

  #[test]
  fn test_unequal_ignoring_fp_precision() {
    let shape1 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      vec![4, 3], vec![0, 1],
      Vec::new(), 0, 0);

    let shape2 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F16,
      vec![3, 4], vec![0, 1],
      Vec::new(), 0, 0);

    assert_eq!(ShapeUtil::equal_ignoring_fp_precision(&shape1, &shape2), false);
    /*
    let shape3 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      vec![3, 4], vec![0, 1],
      Vec::new(), 0, 0);

    let shape4 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F16,
      vec![3, 4], vec![1, 0],
      Vec::new(), 0, 0);

    assert_eq!(ShapeUtil::equal_ignoring_fp_precision(&shape3, &shape4), false);
    */
  }
}