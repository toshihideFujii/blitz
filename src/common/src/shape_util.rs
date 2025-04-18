#![allow(dead_code)]

use crate::{
  blitz_data::{DimLevelType, PrimitiveType},
  layout::Tile,
  layout_util::LayoutUtil,
  overflow_util::{self, overflow_safe_multiply},
  permutation_util::compose_permutation,
  primitive_util,
  printer::{Printer, StringPrinter},
  shape::{ProgramShape, Shape, ShapeEqual},
  util::DimensionVector
};

pub struct ShapeUtil {}

impl ShapeUtil {
  const ANNOTATION_PRINT_INTERVAL: usize = 5;

  // Returns the product of the statically bound dimensions.
  pub fn extent_product(shape: &Shape, bounded_dynamic_ok: bool) -> (i64, bool) {
    assert!(shape.is_array());
    assert_eq!(shape.dimensions_size(), shape.rank());
    let mut product = 1;
    let mut any_overflows = false;
    for dim in 0..shape.dimensions_size() {
      if bounded_dynamic_ok {
        if shape.is_unbounded_dynamic_dimension(dim) { continue; }
      } else {
        assert!(!shape.is_unbounded_dynamic_dimension(dim));
      }
      #[allow(unused_assignments)]
      let mut overflow = false;
      (product, overflow) =
        overflow_util::overflow_safe_multiply(product, shape.dimensions(dim));
      any_overflows |= overflow;
    }
    (product, any_overflows)
  }

  // Returns the number of elements are contained within the provided shape.
  pub fn elements_in(shape: &Shape) -> i64 {
    let result =
      ShapeUtil::extent_product(shape, false);
    assert!(!result.1);
    result.0
  }

  // As elements_in(), but recurses through tuples.
  pub fn elements_in_recursive(shape: &Shape) -> i64 {
    debug_assert!(shape.is_array() || shape.is_tuple());
    if shape.is_array() {
      return ShapeUtil::elements_in(shape);
    }
    let mut count = 0;
    for element_shape in shape.tuple_shapes_vec() {
      count += ShapeUtil::elements_in_recursive(element_shape);
    }
    count
  }

  // Returns true if shape has the primitive type, recurses through tuples.
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

  // Returns true if shape is an array with zero elements.
  pub fn is_zero_element_array(shape: &Shape) -> bool {
    if !shape.is_array() { return false; }
    if shape.dimensions_size() == 0 { return false; }

    for i in shape.dimensions_vec() {
      if *i == 0 { return true; }
    }
    false
  }

  // Returns the number of bytes required for an allocation of shape.
  // The 'pointer_size' parameter is used for calculating the size of
  // tuple shapes. This includes only the size of the top-level buffer.
  pub fn byte_size_of(shape: &Shape, pointer_size: i64) -> i64 {
    let err =
      ShapeUtil::validate_shape_with_optional_layout(shape);
    if !err.is_ok() {
      assert!(false, "validate_shape_with_optional_layout() failed.");
    }
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

  pub fn byte_size_of_tuple_index_table(shape: &Shape, pointer_size: i64) -> i64 {
    let err = ShapeUtil::validate_shape(shape);
    if !err.is_ok() {
      assert!(false, "validate_shape() failed.")
    }
    assert!(shape.element_type() == PrimitiveType::Tuple);
    assert!(pointer_size > 0);
    pointer_size * (shape.tuple_shapes_size() as i64)
  }

  // Returns the number of bytes required for the elements in an allocation
  // of shape, which must be an array shape.
  pub fn byte_size_of_elements(shape: &Shape) -> i64 {
    assert_eq!(ShapeUtil::validate_shape_with_optional_layout(shape), Ok(()));
    assert!(LayoutUtil::is_dense_array(shape));
    let allocated_element_count = ShapeUtil::elements_in(shape);

    let element_size_in_bits = shape.layout().as_ref().unwrap().element_size_in_bits();
    if shape.has_layout() && element_size_in_bits != 0 {
      let num_bits = (allocated_element_count * element_size_in_bits) as u32;
      return num_bits.div_ceil(8) as i64 // TODO: Is it ok ?
    }

    allocated_element_count *
      ShapeUtil::byte_size_of_primitive_type(&shape.element_type())
  }

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

  // Two shapes have same structure if all subshape indices of lhs are presented
  // on rhs and vice versa.
  // A nested tuple shape of (F32, (S32[2], F32[2, 2])) is structurally equal to
  // (S32, (F32[3], S32[2])) as their structures are both (,(,))
  //
  // In contrast, (F32, (F32, F32)) is structurally different from
  // ((F32, F32), F32) as the former has structure (,(,)) while the latter has
  // ((,),)
  pub fn equal_structure(_lhs: &Shape, _rhs: &Shape) -> bool {
    /*
    let mut equal = true;

    let func_rhs = |subshape: &Shape, index_vec: &Vec<usize>| {
      equal = equal & ShapeUtil::index_is_valid(rhs, index_vec);
    };
    ShapeUtil::for_each_mutable_subshape(lhs, &mut func_rhs);

    let func_lhs = |subshape: &Shape, index_vec: &Vec<usize>| {
      equal = equal & ShapeUtil::index_is_valid(lhs, index_vec);
    };
    ShapeUtil::for_each_mutable_subshape(rhs, &mut func);

    equal
    */
    unimplemented!()
  }

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
      result.mutable_layout().as_mut().unwrap()
        .set_dynamic_shape_metadata_prefix_bytes(0);
    }
    result
  }

  // Creates a tuple shape from a slice of element shapes within the tuple.
  pub fn make_tuple_shape(shapes: Vec<Shape>) -> Shape {
    let mut result = Shape::new();
    result.set_element_type(PrimitiveType::Tuple);
    result.tuple_shapes_vec_mut().reserve(shapes.len());
    for shape in shapes {
      ShapeUtil::append_shape_to_tuple(shape, &mut result)
    }
    let err =
      ShapeUtil::validate_shape_with_optional_layout(&result);
    if !err.is_ok() {
      assert!(false, "validate_shape_with_optional_layout() failed.");
    }
    result
  }

  // Creates a tuple from a slice of element shapes within the tuple.
  // If only one shape is passed, returns that.
  pub fn make_maybe_tuple_shape(shapes: Vec<Shape>) -> Shape {
    if shapes.len() == 1 { return shapes[0].clone(); }
    ShapeUtil::make_tuple_shape(shapes)
  }

  pub fn make_opaque_shape() -> Shape {
    let mut result = Shape::new();
    result.set_element_type(PrimitiveType::OpaqueType);
    let err =
      ShapeUtil::validate_shape_with_optional_layout(&result);
    if !err.is_ok() {
      assert!(false, "validate_shape_with_optional_layout() failed.");
    }
    result
  }

  // Creates a token shape.
  // Values of this shape are used for ordering side-effecting operations.
  pub fn make_token_shape() -> Shape {
    let mut result = Shape::new();
    result.set_element_type(PrimitiveType::Token);
    let err =
      ShapeUtil::validate_shape_with_optional_layout(&result);
    if !err.is_ok() {
      assert!(false, "validate_shape_with_optional_layout() failed.");
    }
    result
  }

  pub fn append_shape_to_tuple(shape: Shape, tuple_shape: &mut Shape) {
    let err =
      ShapeUtil::validate_shape_with_optional_layout(&shape);
    if !err.is_ok() {
      assert!(false, "validate_shape_with_optional_layout() failed.");
    }
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
      shape.mutable_layout().as_mut().unwrap().add_minor_to_major(rank);
    }
    shape.add_dimensions(bound);
    let err = ShapeUtil::validate_shape(shape);
    if !err.is_ok() {
      assert!(false, "validate_shape() failed.");
    }
  }

  pub fn prepend_major_dimension() {}

  pub fn append_minor_dimension(bound: i64, shape: &mut Shape) {
    assert!(LayoutUtil::is_dense_array(shape));
    shape.add_dimensions(bound);
    if shape.has_layout() {
      let rank = shape.rank() as i64;
      let layout = shape.mutable_layout().as_mut().unwrap();
      layout.add_minor_to_major(0);
      let dim_idx = layout.minor_to_major_size() - 2;
      for i in dim_idx..0 {
        let layout_idx = layout.minor_to_major(i);
        layout.set_minor_to_major(i + 1, layout_idx);
      }
      layout.set_minor_to_major(0, rank - 1);
    }
    let err = ShapeUtil::validate_shape(shape);
    if !err.is_ok() {
      assert!(false, "validate_shape() failed.");
    }
    
  }

  // Copy the dynamic dimensions property from one shape to another.
  pub fn copy_dynamic_dimensions(to: &mut Shape, from: &Shape) {
    assert_eq!(to.rank(), from.rank());
    for i in 0..from.rank() {
      to.set_dynamic_dimension(i, from.is_dynamic_dimension(i));
    }
    let result = ShapeUtil::validate_shape(to);
    assert!(result.is_ok());
  }

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

  // Returns an empty tuple shape. Cna be used as a sentinel shape value.
  pub fn make_nil() -> Shape {
    ShapeUtil::make_tuple_shape(vec![])
  }

  pub fn is_initialized(shape: &Shape) -> bool {
    shape.element_type() != PrimitiveType::Invalid
  }

  // Constructs a new shape with the given element type and sequence of dimensions.
  pub fn make_shape(elt_t: &PrimitiveType, dimensions: Vec<i64>) -> Shape {
    let mut shape = Shape::new();
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
  pub fn make_validated_shape(elt_t: &PrimitiveType, dimensions: &Vec<i64>) -> Shape {
    let mut shape = Shape::new();
    if !ShapeUtil::fill_new_shape(elt_t, dimensions, &mut shape) {
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
    let mut shape = Shape::new();
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
    dimensions: &Vec<i64>,
    minor_to_major: &Vec<i64>,
    dim_level_types: Vec<DimLevelType>,
    dim_unique: Vec<bool>,
    dim_ordered: Vec<bool>,
    tiles: Vec<Tile>,
    tail_padding_alignment_in_elements: i64,
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
      tail_padding_alignment_in_elements,
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

  // Constructs a new dense array shape with the given minor_to_major order in
  // its Layout. Returns a value shape such that shape.has_layout().
  pub fn make_shape_with_dense_layout(
    elt_t: &PrimitiveType,
    dimensions: &Vec<i64>,
    minor_to_major: &Vec<i64>,
    tiles: Vec<Tile>,
    tail_padding_alignment_in_elements: i64,
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
      tail_padding_alignment_in_elements,
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

  pub fn populate_shape(
    elt_t: &PrimitiveType,
    dimensions: Vec<i64>,
    shape: &mut Shape) -> Result<(), String>
  {
    shape.clear();
    shape.set_element_type(elt_t.clone());
    for dimension in dimensions {
      shape.add_dimensions(dimension);
    }
    LayoutUtil::set_to_default_layout(shape);
    ShapeUtil::validate_shape(shape)
  }

  // Validates that the provided shape satisfies invariants.
  pub fn validate_shape(shape: &Shape) -> Result<(), String> {
    let err =
      ShapeUtil::validate_shape_with_optional_layout_internal(shape);
    if !err.is_ok() {
      return err;
    }
    LayoutUtil::validate_layout_in_shape(shape, false)
  }

  // Validates that the provided shape satisfies invariants, except those that
  // pertain to layout.
  pub fn validate_shape_with_optional_layout(shape: &Shape) -> Result<(), String> {
    let err =
      ShapeUtil::validate_shape_with_optional_layout_internal(shape);
    if !err.is_ok() {
      return err;
    }
    LayoutUtil::validate_layout_in_shape(shape, true)
  }

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

  // Returns whether the shape is a tuple with at least one element which
  // is also a tuple.
  pub fn is_nested_tuple(shape: &Shape) -> bool {
    if !shape.is_tuple() { return false; }
    for s in shape.tuple_shapes_vec() {
      if s.is_tuple() { return true; }
    }
    false
  }

  // Returns true if shape is an empty tuple.
  pub fn is_empty_tuple(shape: &Shape) -> bool {
    shape.is_tuple() && shape.tuple_shapes_vec().is_empty()
  }

  // Returns the number of elements in the given tuple shape.
  pub fn tuple_element_count(shape: &Shape) -> usize {
    shape.tuple_shapes_size()
  }

  pub fn get_tuple_element_shape(shape: &Shape, index: usize) -> &Shape {
    assert!(ShapeUtil::tuple_element_count(shape) >= index);
    let err = 
      ShapeUtil::validate_shape_with_optional_layout(shape.tuple_shapes(index));
    if !err.is_ok() {
      assert!(false, "validate_shape_with_optional_layout() failed.");
    }
    shape.tuple_shapes(index)
  }

  pub fn subshape_count() {}
  pub fn slice_tuple() {}
  pub fn complex_component_shape() {}

  pub fn index_is_valid(shape: &Shape, index_vec: &Vec<usize>) -> bool {
    let mut subshape: &Shape = shape;
    for i in index_vec {
      if !subshape.is_tuple() || *i >= subshape.tuple_shapes_size() /*|| *i < 0*/ {
        return false;
      }
      subshape = subshape.tuple_shapes(*i);
    }
    true
  }

  // Returns a particular nested shape within the given shape argument.s
  pub fn get_subshape(shape: &Shape, index_vec: &Vec<i64>) -> Shape {
    //let mut return_shape: &Shape = shape;
    let mut return_shape = shape.clone();
    for i in index_vec {
      assert!(return_shape.is_tuple(), "Invalid index for shape.");
      return_shape = return_shape.tuple_shapes(*i as usize).clone();
    }
    return_shape
  }

  pub fn get_mutable_subshape(shape: &mut Shape, index_vec: Vec<i64>) -> &mut Shape {
    let mut return_shape = shape;
    for i in index_vec {
      assert!(return_shape.is_tuple(), "Invalid index for shape.");
      return_shape = return_shape.mutable_tuple_shapes(i as usize);
    }
    return_shape
  }

  pub fn try_get_subshape(shape: &Shape, index_vec: &Vec<i64>) -> Result<Shape, String> {
    let mut return_shape = shape;
    for i in index_vec {
      if !return_shape.is_tuple() ||
        *i < 0 ||
        *i as usize >= return_shape.tuple_shapes_size() {
          return Err("Shape index is not a valid subshape index for tuple
            shape.".to_string());
      }
      return_shape = return_shape.tuple_shapes(*i as usize);
    }
    Ok(return_shape.clone())
  }

  pub fn is_leaf_index(shape: &Shape, index_vec: &Vec<i64>) -> bool {
    !ShapeUtil::get_subshape(shape, index_vec).is_tuple()
  }

  // Returns the number of leaves in the shape.
  pub fn get_leaf_count(shape: &Shape) -> usize {
    if !shape.is_tuple() {
      return 1;
    }
    ShapeUtil::get_leaf_count_tuple(shape)
  }

  pub fn get_leaf_count_tuple(shape: &Shape) -> usize {
    debug_assert!(shape.is_tuple());
    let mut count = 0;
    for subshape in shape.tuple_shapes_vec() {
      if subshape.is_tuple() {
        count += ShapeUtil::get_leaf_count(subshape);
      } else {
        count += 1;
      }
    }
    count
  }

  pub fn get_leaf_shapes() {}

  // Calls the given visitor function for each subshape of the given shape.
  pub fn for_each_subshape<T>(shape: &Shape, func:  &mut T)
    where T: FnMut(&Shape, &Vec<i64>)
  {
    let mut pass_func =
      |subshape: &Shape, index: &Vec<i64>| -> Result<(), String> {
      func(subshape, index);
      Ok(())
    };
    ShapeUtil::for_each_subshape_with_status(shape, &mut pass_func)
  }

  pub fn for_each_mutable_subshape<T>(shape: &mut Shape, func: &mut T)
    where T: FnMut(&mut Shape, &Vec<i64>)
  {
    let mut pass_func =
      |subshape: &mut Shape, index: &Vec<i64>| -> Result<(), String> {
      func(subshape, index);
      Ok(())
    };
    ShapeUtil::for_each_mutable_subshape_with_status(shape, &mut pass_func)
  }

  // Calls the given visitor function for each leaf subshape of the given shape.
  // Subshapes are visited in DFS pre-order starting with the entire shape (index {}).
  pub fn for_each_mutable_leaf_shape<T>(shape: &Shape, func: &mut T)
    where T: FnMut(&Shape, &Vec<i64>)
  {
    let _pass_func = |sub_shape: &Shape, index_vec: &Vec<i64>| {
      if ShapeUtil::is_leaf_index(shape, index_vec) {
        func(sub_shape, index_vec)
      }
    };
    //ShapeUtil::for_each_subshape(shape, pass_func);
  }

  // Variants of for_each_subshape wchich propagate status from the
  // visitor functions.
  pub fn for_each_subshape_with_status<T>(shape: &Shape, func: &mut T)
    where T: FnMut(&Shape, &Vec<i64>) -> Result<(), String>
  {
    ShapeUtil::for_each_subshape_with_status_helper(shape, func, &vec![0]);
  }

  pub fn for_each_mutable_subshape_with_status<T>(shape: &mut Shape, func: &mut T)
    where T: FnMut(&mut Shape, &Vec<i64>) -> Result<(), String>
  {
    ShapeUtil::for_each_mutable_subshape_with_status_helper(shape, func, &vec![0]);
  }

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

  // Permutes the dimensions by the given permutation, so
  // return_value.dimensions[i] = argument.dimensions[permutation[i]].
  //
  // Postcondition: For any valid permutation,
  //
  //   !HasLayout(shape) ||
  //   TransposeIsBitcast(shape, PermuteDimensions(permutation, shape),
  //                      permutation).
  pub fn permute_dimensions(_permutation: &Vec<i64>, _shape: &Shape) -> Shape {
    unimplemented!()
  }

  pub fn inserted_or_deleted_sized_dimensions() {}
  pub fn dimensions_unmodified_by_reshape() {}
  pub fn rehsape_leaves_dimensions_unmodified() {}

  // Returns whether a transpose from input_shape to output_shape with dimension
  // mapping "dimension_mapping" produces a result which is bit-wise identical
  // to its input and thus may be replaced with a bitcast.
  pub fn transpose_is_bitcast(
    input_shape: &Shape,
    output_shape: &Shape,
    dimension_mapping: Vec<i64>,
    ignore_element_type: bool) -> bool
  {
    debug_assert!(LayoutUtil::is_dense_array(input_shape),
      "{:?}", input_shape.to_string(true));
    debug_assert!(LayoutUtil::is_dense_array(output_shape),
      "{:?}", output_shape.to_string(true));
    debug_assert!(input_shape.has_layout(), "{:?}", input_shape.to_string(true));
    debug_assert!(output_shape.has_layout(), "{:?}", output_shape.to_string(true));

    if !ignore_element_type &&
       !ShapeUtil::same_element_type(input_shape, output_shape)
    {
      return false;
    }

    let permutations = compose_permutation(
      dimension_mapping,
      output_shape.layout().as_ref().unwrap().minor_to_major_vec().clone());
    
    &permutations == input_shape.layout().as_ref().unwrap().minor_to_major_vec()
  }

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

  // Returns true if `dynamic_shape` has dimensions that are less-equal to the
  // "bounded_shape". Shapes must be arrays.
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

  // Same as DynamicArrayShapeIsCompatible() but supports tuples.
  pub fn dynamic_shape_is_compatible(
    dynamic_shape: &Shape, bounded_shape: &Shape) -> bool
  {
    let mut compatible = true;
    let mut f = |subshape: &Shape, index: &Vec<i64>| {
      if compatible {
        let subshape_result =
          ShapeUtil::try_get_subshape(bounded_shape, index);
        if subshape_result.is_ok() {
          if subshape.is_tuple() {
            if !subshape_result.as_ref().unwrap().is_tuple() {
              compatible = false;
            }
          } else {
            if subshape_result.as_ref().unwrap().is_tuple() {
              compatible = false;
            } else if !subshape.is_static() &&
              !ShapeUtil::dynamic_array_shape_is_compatible(
                subshape,
                subshape_result.as_ref().unwrap()){
              compatible = false;
            }
          }
        } else {
          compatible = false;
        }
      }
    };
    ShapeUtil::for_each_subshape(dynamic_shape, &mut f);
    compatible
  }

  pub fn for_each_index<F>(
    shape: &Shape,
    base: &Vec<i64>,
    count: &Vec<i64>,
    incr: &Vec<i64>,
    visitor_func: &F) where F: Fn(&Vec<i64>) -> Result<bool, String>
  {
    let _ = ShapeUtil::for_each_index_with_status(
      shape, base, count, incr, visitor_func);
  }

  pub fn for_each_index_with_status<F>(
    shape: &Shape,
    base: &Vec<i64>,
    count: &Vec<i64>,
    incr: &Vec<i64>,
    visitor_func: &F
  ) -> Result<(), String>
    where F: Fn(&Vec<i64>) -> Result<bool, String>
  {
    ShapeUtil::for_each_index_internal(shape, base, count, incr, visitor_func)
  }

  pub fn for_each_index_internal<F>(
    shape: &Shape,
    base: &Vec<i64>,
    count: &Vec<i64>,
    incr: &Vec<i64>,
    visitor_func: &F) -> Result<(), String>
    where F: Fn(&Vec<i64>) -> Result<bool, String>
  {
    let mut s = ForEachState::new(shape, base, count, incr);
    if s.is_zero_element_array() { return Ok(()); }

    // Allows handling R0 arrays, such that the visitor function will be called
    // once with the proper empty indexes.
    let mut n = -1;
    let rank = s.rank as i64;
    while n < rank {
      let should_continue = visitor_func(&s.indexes_span);
      if should_continue.is_err() {
        return Err(should_continue.err().unwrap());
      }
      if !should_continue.unwrap() { break; }
      // Increments dimensions in minor to major order.
      n = s.increment_dim();
    }
    Ok(())
  }

  pub fn for_each_index_no_status() {}


  // A parallel version of ForEachIndex(WithStatus). This can only be used if
  // the visitor_function is thread-safe and the order of iteration does not
  // matter.
  //
  // Please use GetForEachIndexParallelThreadCount() to get the number of
  // threads in the threadpool of ForEachIndexParallel*. This will not change
  // during the runtime of the process. Please DO NOT use
  // tsl::port::MaxParallelism() for this purpose, as it may change.
  pub fn for_each_index_parallel<F>(
    shape: &Shape,
    base: &Vec<i64>,
    count: &Vec<i64>,
    incr: &Vec<i64>,
    visitor_func: &F) where F: Fn(&Vec<i64>, i64) -> Result<bool, String>
  {
    // The parallel version of ForEachIndexInternal can never fail.
    ShapeUtil::for_each_index_parallel_with_status(
      shape, base, count, incr, visitor_func)
  }

  pub fn for_each_index_parallel_with_status<F>(
    shape: &Shape,
    base: &Vec<i64>,
    count: &Vec<i64>,
    incr: &Vec<i64>,
    visitor_func: &F) where F: Fn(&Vec<i64>, i64) -> Result<bool, String>
  {
    // The parallel version of ForEachIndexInternal can never fail.
    ShapeUtil::for_each_index_internal_parallel(
      shape, base, count, incr, visitor_func)
  }

  pub fn for_each_index_internal_parallel<F>(
    _shape: &Shape,
    _base: &Vec<i64>,
    _count: &Vec<i64>,
    _incr: &Vec<i64>,
    _visitor_func: &F) where F: Fn(&Vec<i64>, i64) -> Result<bool, String>
  {
    unimplemented!()   
  }

  // Returns the number of threads in the threadpool of ForEachIndexParallel*.
  pub fn get_for_each_index_parallel_thread_count() -> usize {
    unimplemented!()
  }

  pub fn get_normalized_transpose_shape() {}
  pub fn get_normalized_logical_transpose_shape() {}

  // Strips device-specific information, namely tiling and memory-space
  // information, from a shape.
  pub fn device_shape_to_host_shape(_s: Shape) -> Shape {
    unimplemented!()
  }

  pub fn element_can_upcast(from: &Shape, to: &Shape) -> bool {
    ShapeUtil::higher_precision_element_type(from, to) == to.element_type()
  }

  // Computes byte strides of an array shape. Shape must have a layout.
  pub fn byte_strides_inner(shape: &Shape, strides: &mut Vec<i64>) -> Result<(), String> {
    assert!(shape.is_array());
    assert!(shape.has_layout());
    assert!(shape.dimensions_size() == strides.len());

    let mut stride = ShapeUtil::byte_size_of_primitive_type(&shape.element_type());
    for i in shape.layout().as_ref().unwrap().minor_to_major_vec() {
      strides[*i as usize] = stride;
      stride *= shape.dimensions(*i as usize);
    }
    Ok(())
  }

  // Same as above but returns the stride array.
  pub fn byte_strides(shape: &Shape) -> Vec<i64> {
    let mut strides = vec![0; shape.dimensions_size()];
    if ShapeUtil::byte_strides_inner(shape, &mut strides).is_err() {
      assert!(false, "byte_strides_inner failed.");
    }
    strides
  }

  pub fn array_size() {}
  pub fn array_data_size() {}

  fn fill_new_shape(
    elt_t: &PrimitiveType,
    dimensions: &Vec<i64>,
    shape: &mut Shape) -> bool
  {
    let mut dense_shape_size = -1;
    if primitive_util::is_array_type(elt_t) {
      dense_shape_size = primitive_util::byte_width(elt_t);
    }
    assert_eq!(dense_shape_size,
      ShapeUtil::byte_size_of_primitive_type(elt_t));

    shape.set_element_type(elt_t.clone());
    let ndims = dimensions.len();
    shape.mutable_layout();
    let mut static_extent_product = dense_shape_size;
    let mut any_overflows = false;

    for i in 0..ndims {
      let d = dimensions[i];
      if d != Shape::UNBOUNDED_SIZE {
        #[allow(unused_assignments)]
        let mut overflow = false;
        (static_extent_product, overflow) =
          overflow_safe_multiply(static_extent_product, d);
        any_overflows |= overflow;
      }
      shape.add_dimensions(d);
      shape.add_minor_to_major((ndims as i64)- 1 - (i as i64));
    }

    if any_overflows { return false; }
    true
  }

  // Validates the shape size is sane.
  // This makes sure it's safe to do calculations in i64 without overflowing.
  fn validate_shape_size(shape: &Shape) -> Result<(), String> {
    if !shape.is_array() { return Ok(()); }

    let extent_result =
      ShapeUtil::extent_product(shape, true);
    let byte_size =
      ShapeUtil::byte_size_of_primitive_type(&shape.element_type());

    let overflow_result =
      overflow_util::overflow_safe_multiply(extent_result.0, byte_size);
    if extent_result.1 || overflow_result.1 {
      return Err("Shape size may overflow i64.".to_string());
    }

    Ok(())
  }

  // Validates all of the non-layout properties of the shape.
  // This is a helper used by both the layout-optional and layout-required
  // public method.
  fn validate_shape_with_optional_layout_internal(
    shape: &Shape) -> Result<(), String>
  {
    if shape.element_type() == PrimitiveType::Invalid {
      // || !primitive_type_is_valid()
      return Err("Shape has invalid primitive type.".to_string());
    }
    if shape.element_type() == PrimitiveType::Tuple {
      if shape.dimensions_size() != 0 {
        return Err("Tuples must not have dimensions specified.".to_string());
      }
      for subshape in shape.tuple_shapes_vec() {
        let err =
          ShapeUtil::validate_shape_with_optional_layout_internal(subshape);
        if !err.is_ok() {
          return err;
        }
      }
      return Ok(());
    }
    if shape.tuple_shapes_size() > 0 {
      return Err("Non-tuple shape has tuple_shapes field.".to_string());
    }
    if shape.element_type() == PrimitiveType::Token ||
       shape.element_type() == PrimitiveType::OpaqueType {
      if shape.dimensions_size() != 0 {
        return Err("Shape has token or opaque type, but has dimensions field.".to_string());
      }
      if shape.has_layout() {
        return Err("Shape has token or opaque type, but has lauout field.".to_string());
      }
      return Ok(());
    }
    let mut any_overflows = false;
    let product = 1;
    for i in 0..shape.rank() {
      let dimension = shape.dimensions(i);
      if dimension == Shape::UNBOUNDED_SIZE {
        continue;
      }
      if dimension < 0 {
        return Err("Shape's dimensions must not be < 0.".to_string());
      }
      let overflow_result =
        overflow_util::overflow_safe_multiply(product, dimension);
      any_overflows |= overflow_result.1;
    }
    if any_overflows {
      return Err("Shape's dimensions overflow".to_string());
    }
    let err = ShapeUtil::validate_shape_size(shape);
    if err.is_err() {
      return err;
    }

    Ok(())
  }

  // Helper for for_each_subshape which visits the subshapes of the given shape
  // in DFS pre-order starting with the index.
  fn for_each_subshape_with_status_helper<T>(
    shape: &Shape,
    func: &mut T,
    index: &Vec<i64>) where T: FnMut(&Shape, &Vec<i64>) -> Result<(), String>
  {
    let result = func(shape, index);
    if result.is_err() {
      assert!(false, "for_each_subshape_with_status_helper is failed.");
    }
    if shape.is_tuple() {
      for i in 0..ShapeUtil::tuple_element_count(shape) {
        ShapeUtil::for_each_subshape_with_status_helper(
          shape.tuple_shapes(i), func, &vec![i as i64]);
      }
    }
  }

  fn for_each_mutable_subshape_with_status_helper<T>(
    shape: &mut Shape,
    func: &mut T,
    index: &Vec<i64>) where T: FnMut(&mut Shape, &Vec<i64>) -> Result<(), String>
  {
    let result = func(shape, index);
    if result.is_err() {
      assert!(false, "for_each_subshape_with_status_helper is failed.");
    }
    if shape.is_tuple() {
      for i in 0..ShapeUtil::tuple_element_count(shape) {
        ShapeUtil::for_each_mutable_subshape_with_status_helper(
          shape.mutable_tuple_shapes(i), func, &vec![i as i64]);
      }
    }
  }
}

// Keeps track of the iteration state for the ForEach...Internal routines
struct ForEachState {
  shape: Shape,
  base: Vec<i64>,
  count: Vec<i64>,
  incr: Vec<i64>,
  minor_to_major: Vec<i64>,
  rank: usize,
  indexes: Vec<i64>,
  indexes_ptr: Vec<i64>,
  indexes_span: Vec<i64>
}

impl ForEachState {
  pub fn new(s: &Shape, b: &Vec<i64>, c: &Vec<i64>, i: &Vec<i64>) -> Self {
    let mut instance = ForEachState {
      shape: s.clone(),
      base: vec![],
      count: vec![],
      incr: vec![],
      minor_to_major: vec![],
      rank: 0,
      indexes: vec![],
      indexes_ptr: vec![],
      indexes_span: vec![]
    };
    instance.base.clone_from(b);
    instance.count.clone_from(c);
    instance.incr.clone_from(i);
    instance.minor_to_major.clone_from(
      s.layout().as_ref().unwrap().minor_to_major_vec());
    instance.rank = LayoutUtil::minor_to_major_from_shape(s).len();
    instance.indexes.clone_from(b);

    if instance.rank != 0 {
      instance.indexes_ptr.clone_from(&instance.indexes);
    }
    instance.indexes_span.clone_from(&instance.indexes);
    instance
  }

  pub fn increment_dim(&mut self) -> i64 {
    let mut n = 0;
    for i in 0..self.rank {
      let dim = self.minor_to_major[i];
      self.indexes_ptr[dim as usize] += self.incr[dim as usize];
      if self.indexes_ptr[dim as usize] <
        self.base[dim as usize] + self.count[dim as usize]
      { break; }
      self.indexes_ptr[dim as usize] = self.base[dim as usize];
      n += 1;
    }
    n
  }

  pub fn is_zero_element_array(&self) -> bool {
    ShapeUtil::is_zero_element_array(&self.shape)
  }

  // Returns the number of visited elements assuming that the iteration will
  // not be interrupted.
  pub fn calculate_num_steps(&self) -> i64 {
    if self.is_zero_element_array() { return 0; }
    let mut size = 1;
    // This works for rank = 0 as well.
    for i in 0..self.rank {
      // When the count is zero, it can mean that the given dimension is fixed,
      // but we still iterate over the others.
      if self.count[i] == 0 { continue; }
      let dim = 1 + (self.count[i] - 1) / self.incr[i];
      size *= dim;
    }
    size
  }
}

struct ParallelState {}

#[cfg(test)]
mod tests {
  use crate::layout::Layout;

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
      &vec![4, 3], &vec![0, 1],
      Vec::new(), 1, 0, 0);
    
    let shape2 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F16,
      &vec![4, 3], &vec![0, 1],
      Vec::new(), 1, 0, 0);

    assert_eq!(ShapeUtil::equal_ignoring_fp_precision(&shape1, &shape2), true);
  }

  #[test]
  fn test_unequal_ignoring_fp_precision() {
    let shape1 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      &vec![4, 3], &vec![0, 1], vec![],
      1, 0, 0);
    let shape2 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F16,
      &vec![3, 4], &vec![0, 1], vec![],
      1, 0, 0);
    assert_eq!(ShapeUtil::equal_ignoring_fp_precision(&shape1, &shape2), false);
    
    let shape3 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      &vec![3, 4], &vec![0, 1], vec![],
      1, 0, 0);
    let shape4 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F16,
      &vec![3, 4], &vec![1, 0], vec![],
      1, 0, 0);
    assert_eq!(ShapeUtil::equal_ignoring_fp_precision(&shape3, &shape4), false);
    
    let shape5 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      &vec![4, 3], &vec![0, 1], vec![],
      1, 0, 0);
    let shape6 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::Pred,
      &vec![4, 3], &vec![0, 1], vec![],
      1, 0, 0);
    assert_eq!(ShapeUtil::equal_ignoring_fp_precision(&shape5, &shape6), false);
  }

  #[test]
  fn test_equal_ignoring_element_type() {
    let s1 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      &vec![4, 3], &vec![0, 1], vec![],
      1, 0, 0);
    let s2 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F16,
      &vec![4, 3], &vec![0, 1], vec![],
      1, 0, 0);
    assert_eq!(ShapeUtil::equal_ignoring_element_type(&s1, &s2), true);

    let s3 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::S32,
      &vec![4, 3], &vec![0, 1], vec![],
      1, 0, 0);
    let s4 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F16,
      &vec![4, 3], &vec![0, 1], vec![],
      1, 0, 0);
    assert_eq!(ShapeUtil::equal_ignoring_element_type(&s3, &s4), true);

    let s5 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      &vec![4, 3], &vec![0, 1], vec![],
      1, 0, 0);
    let s6 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::Pred,
      &vec![4, 3], &vec![0, 1], vec![],
      1, 0, 0);
    assert_eq!(ShapeUtil::equal_ignoring_element_type(&s5, &s6), true);
  }

  #[test]
  fn test_unequal_ignoring_element_type() {
    let s1 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      &vec![4, 3], &vec![0, 1], vec![],
      1, 0, 0);
    let s2 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F16,
      &vec![3, 4], &vec![0, 1], vec![],
      1, 0, 0);
    assert_eq!(ShapeUtil::equal_ignoring_element_type(&s1, &s2), false);

    let s3 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F32,
      &vec![3, 4], &vec![0, 1], vec![],
      1, 0, 0);
    let s4 = ShapeUtil::make_shape_with_dense_layout(
      &PrimitiveType::F16,
      &vec![3, 4], &vec![1, 0], vec![],
      1, 0, 0);
    assert_eq!(ShapeUtil::equal_ignoring_element_type(&s3, &s4), false);
  }

  #[test]
  fn test_equal_dynamic_shapes() {
    let s1 = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::F32, vec![4, 3],
      vec![true, false]);
    let s2 = ShapeUtil::make_shape_dynamic(
        &PrimitiveType::F32, vec![4, 3],
        vec![true, false]);
    assert_eq!(ShapeEqual::new().equal(&s1, &s2), true);

    let s3 = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::F32, vec![4, 3],
      vec![true, false]);
    let s4 = ShapeUtil::make_shape_dynamic(
        &PrimitiveType::F32, vec![4, 3],
        vec![false, false]);
    assert_eq!(ShapeEqual::new().equal(&s3, &s4), false);
  }

  #[test]
  fn test_compatible_dynamic_shapes() {
    let mut a = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::F32, vec![4, 3],
      vec![true, false]);  
    a.set_layout(Layout::new_from(
      vec![1, 0], vec![], vec![],
      vec![], vec![], 1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,
      0, 0, None,
      0));

    let mut b = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::F32, vec![4, 3],
      vec![true, false]);  
    b.set_layout(Layout::new_from(
      vec![0, 1], vec![], vec![],
      vec![], vec![], 1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,
      0, 0, None,
      0));

    let mut c = ShapeUtil::make_shape_dynamic(
      &PrimitiveType::F32, vec![4, 3],
      vec![false, true]);  
    c.set_layout(Layout::new_from(
      vec![0, 1], vec![], vec![],
      vec![], vec![], 1,
      PrimitiveType::Invalid,
      PrimitiveType::Invalid,
      0, 0, None,
      0));

    assert_eq!(ShapeUtil::compatible(&a, &a), true);
    assert_eq!(ShapeUtil::compatible(&a, &b), true);
    assert_eq!(ShapeUtil::compatible(&a, &c), true);
  }

  #[test]
  fn test_compatible_tuples() {
    let s1 =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let s2 =
      ShapeUtil::make_shape(&PrimitiveType::Pred, vec![4, 5]);
    let tuple1 = ShapeUtil::make_tuple_shape(vec![s1, s2]);

    let s3 =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let s4 =
      ShapeUtil::make_shape(&PrimitiveType::Pred, vec![4, 5]);
    let tuple2 = ShapeUtil::make_tuple_shape(vec![s3, s4]);

    assert_eq!(ShapeUtil::compatible(&tuple1, &tuple2), true);
  }

  #[test]
  fn test_make_maybe_tuple_shape() {
    let s1 = ShapeUtil::make_maybe_tuple_shape(
      vec![ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2])]);
    assert_eq!(ShapeUtil::compatible(&s1,
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2])), true);
  }

  #[test]
  fn test_compatible_tuple_ignoring_fp_precision() {
    let s1 = ShapeUtil::make_shape(&PrimitiveType::BF16, vec![3, 2]);
    let s2 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![4, 5]);
    let tuple1 = ShapeUtil::make_tuple_shape(vec![s1, s2]);

    let s3 = ShapeUtil::make_shape(&PrimitiveType::F64, vec![3, 2]);
    let s4 = ShapeUtil::make_shape(&PrimitiveType::BF16, vec![4, 5]);
    let tuple2 = ShapeUtil::make_tuple_shape(vec![s3, s4]);

    assert_eq!(ShapeUtil::compatible_ignoring_fp_precision(&tuple1, &tuple2), true);
  }

  #[test]
  fn test_incompatible_tuples_with_swapped_elements() {
    let s1 = ShapeUtil::make_shape(&PrimitiveType::Pred, vec![4, 5]);
    let s2 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let tuple1 = ShapeUtil::make_tuple_shape(vec![s1, s2]);

    let s3 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let s4 = ShapeUtil::make_shape(&PrimitiveType::Pred, vec![4, 5]);
    let tuple2 = ShapeUtil::make_tuple_shape(vec![s3, s4]);

    assert_eq!(ShapeUtil::compatible(&tuple1, &tuple2), false);
    assert_eq!(ShapeUtil::compatible_ignoring_element_type(&tuple1, &tuple2), false);
  }

  #[test]
  fn test_incompatible_tuples_ignoring_fp_precision() {
    let s1 = ShapeUtil::make_shape(&PrimitiveType::BF16, vec![4, 5]);
    let s2 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let tuple1 = ShapeUtil::make_tuple_shape(vec![s1, s2]);

    let s3 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let s4 = ShapeUtil::make_shape(&PrimitiveType::BF16, vec![4, 5]);
    let tuple2 = ShapeUtil::make_tuple_shape(vec![s3, s4]);

    assert_eq!(ShapeUtil::compatible_ignoring_fp_precision(&tuple1, &tuple2), false);
  }

  #[test]
  fn test_incompatible_tuples_with_different_primitive_type() {
    let s1 = ShapeUtil::make_shape(&PrimitiveType::Pred, vec![4, 5]);
    let s2 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let tuple1 = ShapeUtil::make_tuple_shape(vec![s1, s2]);

    let s3 = ShapeUtil::make_shape(&PrimitiveType::Pred, vec![4, 5]);
    let s4 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![3, 2]);
    let tuple2 = ShapeUtil::make_tuple_shape(vec![s3, s4]);

    assert_eq!(ShapeUtil::compatible(&tuple1, &tuple2), false);
    assert_eq!(ShapeUtil::compatible_ignoring_element_type(&tuple1, &tuple2), true);
  }

  #[test]
  fn test_incompatible_tuples_with_different_dimensions() {
    let s1 = ShapeUtil::make_shape(&PrimitiveType::Pred, vec![4, 5]);
    let s2 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let tuple1 = ShapeUtil::make_tuple_shape(vec![s1, s2]);

    let s3 = ShapeUtil::make_shape(&PrimitiveType::Pred, vec![4, 5]);
    let s4 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![4, 2]);
    let tuple2 = ShapeUtil::make_tuple_shape(vec![s3, s4]);

    assert_eq!(ShapeUtil::compatible(&tuple1, &tuple2), false);
  }

  #[test]
  fn test_incompatible_scalar_vs_tuple() {
    let shape1 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![]);

    let s1 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 2]);
    let s2 = ShapeUtil::make_shape(&PrimitiveType::U32, vec![]);
    let shape2 = ShapeUtil::make_tuple_shape(vec![s1, s2]);

    assert_eq!(ShapeUtil::compatible(&shape1, &shape2), false);
    assert_eq!(ShapeUtil::compatible(&shape2, &shape1), false);
    assert_eq!(ShapeUtil::compatible_ignoring_element_type(&shape1, &shape2), false);
    assert_eq!(ShapeUtil::compatible_ignoring_element_type(&shape2, &shape1), false);
    assert_eq!(ShapeUtil::compatible_ignoring_fp_precision(&shape1, &shape2), false);
    assert_eq!(ShapeUtil::compatible_ignoring_fp_precision(&shape2, &shape1), false);
  }

  #[test]
  fn test_opaque_vs_array() {
    let s1 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![5, 7]);
    let s2 = ShapeUtil::make_opaque_shape();
    assert_eq!(ShapeUtil::compatible(&s1, &s2), false);
    assert_eq!(ShapeUtil::compatible(&s2, &s1), false);
    assert_eq!(ShapeUtil::compatible_ignoring_fp_precision(&s1, &s2), false);
    assert_eq!(ShapeUtil::compatible_ignoring_fp_precision(&s2, &s1), false);
    assert_eq!(ShapeUtil::compatible_ignoring_element_type(&s1, &s2), false);
    assert_eq!(ShapeUtil::compatible_ignoring_element_type(&s2, &s1), false);
  }

  #[test]
  fn test_scalar_default_layout_equal_scalar_empty_min2maj() {
    let scalar_default_layout =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![]);
    assert_eq!(scalar_default_layout.has_layout(), true);

    let scalar_empty_min2maj =
      ShapeUtil::make_shape_with_dense_layout(&PrimitiveType::F32,
        &vec![], &vec![], vec![],
        1, 0, 0);
    assert_eq!(scalar_empty_min2maj.has_layout(), true);

    assert_eq!(ShapeEqual::new().equal(&scalar_default_layout, &scalar_empty_min2maj), true);
  }

  #[test]
  fn test_byte_size_of_without_padding() {
    assert_eq!(ShapeUtil::byte_size_of_primitive_type(&PrimitiveType::F32), 4);
    assert_eq!(ShapeUtil::byte_size_of(
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![]), -1), 4);
    assert_eq!(ShapeUtil::byte_size_of(
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![10, 20]), -1), 800);

    assert_eq!(ShapeUtil::byte_size_of_primitive_type(&PrimitiveType::F64), 8);
    assert_eq!(ShapeUtil::byte_size_of(
      &ShapeUtil::make_shape(&PrimitiveType::F64, vec![]), -1), 8);
    assert_eq!(ShapeUtil::byte_size_of(
      &ShapeUtil::make_shape(&PrimitiveType::F64, vec![10, 20]), -1), 1600);

    assert_eq!(ShapeUtil::byte_size_of_primitive_type(&PrimitiveType::C64), 8);
    assert_eq!(ShapeUtil::byte_size_of(
      &ShapeUtil::make_shape(&PrimitiveType::C64, vec![]), -1), 8);
    assert_eq!(ShapeUtil::byte_size_of(
      &ShapeUtil::make_shape(&PrimitiveType::C64, vec![10, 20]), -1), 1600);
  }

  #[test]
  fn test_byte_strides() {
    let s1 =
      ShapeUtil::make_shape(&PrimitiveType::F32, vec![3, 5, 7]);
    let s2 =
      ShapeUtil::make_shape(&PrimitiveType::F16, vec![5, 7, 9]);

    assert_eq!(ShapeUtil::byte_strides(&s1), vec![140, 28, 4]);
    assert_eq!(ShapeUtil::byte_strides(&s2), vec![126, 18, 2]);
  }

  #[test]
  fn test_nil_shape() {
    assert_eq!(ShapeUtil::is_empty_tuple(&ShapeUtil::make_nil()), true);
    assert_eq!(ShapeUtil::is_empty_tuple(
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 2, 3])), false);
    assert_eq!(ShapeUtil::is_empty_tuple(
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![0, 1])), false);
    
    let s1 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    let t1 = ShapeUtil::make_tuple_shape(vec![s1]);
    assert_eq!(ShapeUtil::is_empty_tuple(&t1), false);

    let s2 = ShapeUtil::make_shape(&PrimitiveType::F32, vec![0]);
    let t2 = ShapeUtil::make_tuple_shape(vec![s2]);
    assert_eq!(ShapeUtil::is_empty_tuple(&t2), false);
  }

  #[test]
  fn test_nessted_tuple() {
    let t1 = ShapeUtil::make_tuple_shape(vec![]);
    assert_eq!(ShapeUtil::is_nested_tuple(&t1), false);

    let s2 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    let t2 = ShapeUtil::make_tuple_shape(vec![s2]);
    assert_eq!(ShapeUtil::is_nested_tuple(&t2), false);

    let t3_1 = ShapeUtil::make_tuple_shape(vec![]);
    let t3 = ShapeUtil::make_tuple_shape(vec![t3_1]);
    assert_eq!(ShapeUtil::is_nested_tuple(&t3), true);

    let s4_1 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    let s4_2 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    let t4 = ShapeUtil::make_tuple_shape(vec![s4_1, s4_2]);
    assert_eq!(ShapeUtil::is_nested_tuple(&t4), false);

    let s5_1 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    let t5_1 = ShapeUtil::make_tuple_shape(vec![]);
    let t5 = ShapeUtil::make_tuple_shape(vec![s5_1, t5_1]);
    assert_eq!(ShapeUtil::is_nested_tuple(&t5), true);

    let t6_1 = ShapeUtil::make_tuple_shape(vec![]);
    let s6_1 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    let t6 = ShapeUtil::make_tuple_shape(vec![t6_1, s6_1]);
    assert_eq!(ShapeUtil::is_nested_tuple(&t6), true);

    let t7_1 = ShapeUtil::make_tuple_shape(vec![]);
    let t7_2 = ShapeUtil::make_tuple_shape(vec![]);
    let t7 = ShapeUtil::make_tuple_shape(vec![t7_1, t7_2]);
    assert_eq!(ShapeUtil::is_nested_tuple(&t7), true);
  }

  #[test]
  fn test_elements_in() {
    let s1 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![]));
    assert_eq!(s1, 1);

    let s2 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![0]));
    assert_eq!(s2, 0);

    let s3 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![1]));
    assert_eq!(s3, 1);

    let s4 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![1, 1]));
    assert_eq!(s4, 1);

    let s5 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![2]));
    assert_eq!(s5, 2);

    let s6 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![2, 1]));
    assert_eq!(s6, 2);

    let s7 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![3, 5]));
    assert_eq!(s7, 15);

    let s8 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![3, 0, 5]));
    assert_eq!(s8, 0);

    let s9 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![0, 3, 0]));
    assert_eq!(s9, 0);

    let s10 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![1, 3, 5]));
    assert_eq!(s10, 15);

    let s11 = ShapeUtil::elements_in(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![13, 17]));
    assert_eq!(s11, 221);
  }

  #[test]
  fn test_has_primitive_type() {
    let s1 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    assert_eq!(ShapeUtil::has_primitive_type(&s1, &PrimitiveType::S32), true);

    let s2 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    assert_eq!(ShapeUtil::has_primitive_type(&s2, &PrimitiveType::S16), false);

    let s3 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![0]);
    assert_eq!(ShapeUtil::has_primitive_type(&s3, &PrimitiveType::S32), true);

    let s4 = ShapeUtil::make_tuple_shape(vec![]);
    assert_eq!(ShapeUtil::has_primitive_type(&s4, &PrimitiveType::S32), false);

    let s5_1 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    let s5_2 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    let s5 = ShapeUtil::make_tuple_shape(vec![s5_1, s5_2]);
    assert_eq!(ShapeUtil::has_primitive_type(&s5, &PrimitiveType::S32), true);

    let s6_1 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    let s6_2_1 = ShapeUtil::make_shape(&PrimitiveType::S16, vec![]);
    let s6_2 = ShapeUtil::make_tuple_shape(vec![s6_2_1]);
    let s6 = ShapeUtil::make_tuple_shape(vec![s6_1, s6_2]);
    assert_eq!(ShapeUtil::has_primitive_type(&s6, &PrimitiveType::S16), true);
  }

  #[test]
  fn test_is_zero_element_array() {
    let s1 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s1), false);

    let s2 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![0]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s2), true);

    let s3 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![1]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s3), false);

    let s4 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![1, 1]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s4), false);

    let s5 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![2]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s5), false);

    let s6 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![2, 1]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s6), false);

    let s7 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![3, 5]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s7), false);

    let s8 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![3, 0, 5]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s8), true);

    let s9 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![0, 3, 0]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s9), true);

    let s10 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![1, 3, 5]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s10), false);

    let s11 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![13, 17]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s11), false);

    let s12 = ShapeUtil::make_nil();
    assert_eq!(ShapeUtil::is_zero_element_array(&s12), false);

    let s13 = ShapeUtil::make_tuple_shape(vec![]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s13), false);

    let s14_1 = ShapeUtil::make_shape(&PrimitiveType::S32, vec![0, 3, 0]);
    let s14 = ShapeUtil::make_tuple_shape(vec![s14_1]);
    assert_eq!(ShapeUtil::is_zero_element_array(&s14), false);
  }

  #[test]
  fn test_same_dimensions() {
    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![]),
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![])), true);

    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![]),
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![])), true);

    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![1]),
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![1])), true);

    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![0]),
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![0])), true);

    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![2]),
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![2])), true);

    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![1]),
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![2])), false);

    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![0, 0]),
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![0])), false);

    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![1]),
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 1])), false);

    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![]),
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![1])), false);

    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![1]),
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 0])), false);

    assert_eq!(ShapeUtil::same_dimensions(
      &ShapeUtil::make_shape(&PrimitiveType::S32, vec![1, 1]),
      &ShapeUtil::make_shape(&PrimitiveType::F32, vec![1, 2])), false);
  }

  #[test]
  fn test_get_subshape() {
    let mut array_shape = ShapeUtil::make_shape(
      &PrimitiveType::S32, vec![42, 42, 123]);
    assert_eq!(ShapeEqual::new().equal(&array_shape,
      &ShapeUtil::get_subshape(&array_shape, &vec![])), true);
    
    let original = array_shape.clone();
    let sub = ShapeUtil::get_mutable_subshape(&mut array_shape, vec![]);
    assert_eq!(ShapeEqual::new().equal(&original, sub), true);

    let tuple_shape = ShapeUtil::make_tuple_shape(
      vec![array_shape.clone(), array_shape.clone(), array_shape.clone()]);
    assert_eq!(ShapeEqual::new().equal(
      &ShapeUtil::get_subshape(&tuple_shape, &vec![]), &tuple_shape), true);
    assert_eq!(ShapeEqual::new().equal(
      &ShapeUtil::get_subshape(&tuple_shape, &vec![0]), &array_shape), true);
    assert_eq!(ShapeEqual::new().equal(
      &ShapeUtil::get_subshape(&tuple_shape, &vec![1]), &array_shape), true);
    assert_eq!(ShapeEqual::new().equal(
      &ShapeUtil::get_subshape(&tuple_shape, &vec![2]), &array_shape), true);

    let nested_tuple_shape = ShapeUtil::make_tuple_shape(vec![
      array_shape.clone(),
      ShapeUtil::make_tuple_shape(vec![array_shape.clone(), array_shape.clone()]),
      ShapeUtil::make_tuple_shape(vec![
        ShapeUtil::make_tuple_shape(vec![
          array_shape.clone(), array_shape.clone()
        ]),
        array_shape.clone()
      ])
    ]);
    assert_eq!(ShapeEqual::new().equal(&nested_tuple_shape,
      &ShapeUtil::get_subshape(&nested_tuple_shape, &vec![])), true);
    assert_eq!(ShapeEqual::new().equal(&array_shape,
      &ShapeUtil::get_subshape(&nested_tuple_shape, &vec![0])), true);
    assert_eq!(ShapeEqual::new().equal(
      &ShapeUtil::make_tuple_shape(vec![array_shape.clone(), array_shape.clone()]),
      &ShapeUtil::get_subshape(&nested_tuple_shape, &vec![1])), true);
    assert_eq!(ShapeEqual::new().equal(
      &ShapeUtil::make_tuple_shape(vec![array_shape.clone(), array_shape.clone()]),
      &ShapeUtil::get_subshape(&nested_tuple_shape, &vec![2, 0])), true);
  }
}