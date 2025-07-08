#![allow(dead_code)]

use common::{
  blitz_data::{ConvolutionDimensionNumbers, DotDimensionNumbers, FftType, GatherDimensionNumbers, PaddingConfig, PrimitiveType, ScatterDimensionNummbers, SparsityDescriptor, TriangularSolveOptions, Window}, shape::{ProgramShape, Shape}, shape_util::ShapeUtil
};
use hlo::{
  hlo_instruction::HloInstruction, hlo_opcode::{hlo_opcode_string, HloOpcode}
};

// For a given operation and input shapes, infers what the resulting shape
// is for the operation.
pub struct ShapeInference {}

impl ShapeInference {
  pub fn infer_unary_op_shape(
    _opcode: &HloOpcode,
    _operand: &HloInstruction) -> Result<Shape, String>
  {
    unimplemented!()
  }

  pub fn infer_unary_op_shape_by_shape(
    _opcode: &HloOpcode,
    _shape: &Shape) -> Result<Shape, String>
  {
    unimplemented!()
  }

  pub fn infer_binary_op_shape(
    _opcode: &HloOpcode,
    _lhs: &HloInstruction,
    _rhs: &HloInstruction) -> Result<Shape, String>
  {
    unimplemented!()
  }

  pub fn infer_binary_op_shape_by_dimensions(
    _opcode: &HloOpcode,
    _lhs: &Shape,
    _rhs: &Shape,
    _broadcast_dimensions: &Vec<i64>) -> Result<Shape, String>
  {
    unimplemented!()
  }

  pub fn infer_ternary_op_shape(
    _opcode: &HloOpcode,
    _lhs: &HloInstruction,
    _rhs: &HloInstruction,
    _ehs: &HloInstruction) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_ternary_op_shape_by_shape(
    _opcode: HloOpcode,
    _lhs: &Shape,
    _rhs: &Shape,
    _ehs: &Shape) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_variadic_op_shape(
    _opcode: HloOpcode,
    _operands: &Vec<HloInstruction>) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  // Infers the shape produced by applying the given variadic operation to the
  // given input operand shapes.
  pub fn infer_variadic_op_shape_by_opshapes(
    opcode: HloOpcode,
    operand_shapes: &Vec<Shape>) -> Result<Shape, String>
  {
    for shape in operand_shapes {
      let result =
        ShapeUtil::validate_shape_with_optional_layout(shape);
      if result.is_err() { return Err(result.err().unwrap()); }
    }
    match opcode {
      HloOpcode::Tuple => {
        let mut result = ShapeUtil::make_tuple_shape(vec![]);
        for shape in operand_shapes {
          ShapeUtil::append_shape_to_tuple((*shape).clone(), &mut result);
        }
        return Ok(result);
      }
      HloOpcode::Sort => {
        if operand_shapes.len() == 1 {
          return Ok(operand_shapes[0].clone());
        } else {
          for operand in 1..operand_shapes.len() {
            if !ShapeUtil::same_dimensions(
              &operand_shapes[0], &operand_shapes[operand])
            {
              let mut err_msg = "Sort keys and values dimensions must match.".to_string();
              err_msg.push_str("Keys shape is: ");
              err_msg.push_str(&ShapeUtil::human_string(&operand_shapes[0]));
              err_msg.push_str("\n, ");
              err_msg.push_str("Values shape (operand index ");
              err_msg.push_str(&operand.to_string());
              err_msg.push_str(") is: ");
              err_msg.push_str(&ShapeUtil::human_string(&operand_shapes[operand]));
              return Err(err_msg);
            }
          }
          let mut op_shapes = vec![];
          for shape in operand_shapes {
            op_shapes.push((*shape).clone());
          }
          return Ok(ShapeUtil::make_tuple_shape(op_shapes));
        }
      }
      _ => {
        let mut err_msg = "Unknown operation ".to_string();
        err_msg.push_str(&hlo_opcode_string(&opcode));
        return Err(err_msg);
      }
    }
  }


  pub fn infer_concat_op_shape(
    _arg_shapes: &Vec<Shape>,
    _dimension: i64) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_convert_shape(
    _operand_shape: &Shape,
    _new_element_t: &PrimitiveType) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_bitcast_convert_shape(
    _operand_shape: &Shape,
    _new_element_t: &PrimitiveType) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_stochastic_convert_shape(
    _operand_shape: &Shape,
    _random_shape: &Shape,
    _new_element_t: &PrimitiveType) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_fft_shape(
    _input: &Shape,
    _fft_t: &FftType,
    _fft_length: &Vec<i64>) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_triangular_solve_shape(
    _a: &Shape,
    _b: &Shape,
    _options: &TriangularSolveOptions) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_cholesky_shape(_a: &Shape) -> Result<Shape, String> {
    unimplemented!()
  }

  pub fn infer_all_gather_shape(
    _operand_shapes: &Vec<Shape>,
    _all_gather_dimension: i64,
    _shard_count: i64) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_all_gether_done_shape(_all_gather_start_shape: &Shape) -> Result<Shape, String> {
    unimplemented!()
  }

  pub fn infer_all_reduce_shape(_operand_shapes: &Vec<Shape>) -> Result<Shape, String> {
    unimplemented!()
  }

  pub fn infer_reduce_scatter_shape(
    _operand_shapes: &Vec<Shape>,
    _scatter_dimension: i64,
    _shard_count: i64) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_all_reduce_done_shape(_operand_shape: &Shape) -> Result<Shape, String> {
    unimplemented!()
  }

  pub fn infer_collective_permute_shape(
    _operand_shapes: &Vec<Shape>,
    _inplace: bool) -> Result<Shape, String>
  {
    unimplemented!()
  }

  pub fn infer_collective_permute_done_shape(
    _operand_shape: &Shape) -> Result<Shape, String>
  {
    unimplemented!()
  }

  pub fn infer_collective_broadcast_shape(
    _operand_shapes: &Vec<Shape>) -> Result<Shape, String>
  {
    unimplemented!()
  }

  pub fn infer_reduce_precision_shape(
    _operand_shape: &Shape,
    _operand_bits: i64,
    _mantissa_bits: i64) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_reverse_shape(
    _operand_shape: &Shape,
    _dimensions: &Vec<i64>) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_topk_shape(_operand_shape: &Shape, _k: i64) -> Result<Shape, String> {
    unimplemented!()
  }

  pub fn infer_get_tuple_element_shape(_arg: &Shape, _index: i64) -> Result<Shape, String> {
    unimplemented!()
  }

  pub fn infer_tranpose_shape(
    _operand: &Shape,
    _dimensions: &Vec<i64>) -> Result<Shape, String>
  {
    unimplemented!()
  }

  pub fn infer_slice_shape(
    _arg: &Shape,
    _starts: &Vec<i64>,
    _limits: &Vec<i64>,
    _strides: &Vec<i64>) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  // Infers the shape produced by a dynamic slice operation of size specified
  // in 'slice_sizes', with dynamic start indices shape 'start_indices_shape'.
  pub fn infer_dynamic_slice_shape(
    _operand_shape: &Shape,
    _start_index_shapes: &Vec<Shape>,
    _slice_sizes: &Vec<i64>,
    _allow_scalar_indices: bool) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_dynamic_update_slice_shape(
    _operand_shape: &Shape,
    _update_shape: &Shape,
    _start_index_shapes: &Vec<Shape>,
    _allow_scalar_indices: bool) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  // Infers the shape produced by a dynamic reshape operation from the element
  // type of its operand and the new dimension sizes specified. The result shape
  // will have dynamic dimensions as specific in `dim_is_dynamic` and bound
  // `new_size_bounds`.
  pub fn infer_dynamic_reshape_shape(
    operand: &Shape,
    dim_size_shapes: &Vec<Shape>,
    new_size_bounds: &Vec<i64>,
    dims_are_dynamic: &Vec<bool>) -> Result<Shape, String>
  {
    if new_size_bounds.len() != dims_are_dynamic.len() {
      let mut err_msg = "DynamicReshape has to have the same number of
        elements in new_sizes (".to_string();
      err_msg.push_str(&new_size_bounds.len().to_string());
      err_msg.push_str(") and dims_are_dynamic (");
      err_msg.push_str(&dims_are_dynamic.len().to_string());
      err_msg.push_str(")");
      return Err(err_msg);
    }
    for dim_size_shape in  dim_size_shapes{
      if dim_size_shape.element_type() != PrimitiveType::S32 &&
        !dim_size_shape.dimensions_vec().is_empty()
      {
        let mut err_msg =
          "DynamicReshape's size has to be scalaar S32, got (".to_string();
        err_msg.push_str(&dim_size_shape.to_string(false));
        err_msg.push_str(")");
        return Err(err_msg);
      }
    }
    let mut dimensions = vec![];
    dimensions.clone_from(new_size_bounds);
    let mut dynamic_dimensions = vec![];
    dynamic_dimensions.clone_from(dims_are_dynamic);
    let inferred_shape = ShapeUtil::make_shape_dynamic(
      &operand.element_type(), dimensions, dynamic_dimensions);

    if ShapeUtil::elements_in(operand) != ShapeUtil::elements_in(&inferred_shape) {
      let mut err_msg =
        "Reshape operation has mismatched element counts: from=".to_string();
      err_msg.push_str(&ShapeUtil::elements_in(operand).to_string());
      err_msg.push_str(" (");
      err_msg.push_str(&ShapeUtil::human_string(operand));
      err_msg.push_str(") to=");
      err_msg.push_str(&ShapeUtil::elements_in(&inferred_shape).to_string());
      err_msg.push_str(" (");
      err_msg.push_str(&ShapeUtil::human_string(&inferred_shape));
      err_msg.push_str(").");
      return Err(err_msg);
    }
    Ok(inferred_shape)
  }

  // Helper that infers the shape produced by a pad operation based on the
  // padding configuration.
  pub fn infer_pad_shape(
    operand_shape: &Shape,
    padding_value_shape: &Shape,
    padding_config: &PaddingConfig) -> Result<Shape, String>
  {
    if !operand_shape.is_array() {
      let err_msg =
        "Pad operation does not support tuple-shape operands.".to_string();
      return Err(err_msg);
    }
    if !ShapeUtil::is_scalar(padding_value_shape) {
      let err_msg =
        "Pad operation does not support non-scalar padding values".to_string();
      return Err(err_msg);
    }
    if operand_shape.dimensions_vec().len() != padding_config.dimensions_size() {
      let mut err_msg = "The rank of the operand and the padding 
        configuration do not match: ".to_string();
      err_msg.push_str(&ShapeUtil::human_string(operand_shape));
      err_msg.push_str(" vs ");
      err_msg.push_str(&padding_config.short_debug_string());
      return Err(err_msg);
    }
    if !ShapeUtil::same_element_type_ignoring_fp_precision(
      operand_shape, padding_value_shape)
    {
      let err_msg =
        "The element types of the operands to Pad do not match".to_string();
      return Err(err_msg);
    }
    for p in padding_config.dimensions_vec() {
      if p.interior_padding() < 0 {
        let mut err_msg = "Interior padding connot be negative: ".to_string();
        err_msg.push_str(&padding_config.short_debug_string());
        return Err(err_msg);
      }
    }
    if !padding_value_shape.is_static() {
      let err_msg = "Dynamic padding value is not supported".to_string();
      return Err(err_msg);
    }
    let mut dimensions = vec![];
    let mut is_dunamic = vec![false; operand_shape.dimensions_vec().len()];
    for i in 0..operand_shape.dimensions_vec().len() {
      let p = padding_config.dimensions(i as i64);
      if operand_shape.is_unbounded_dynamic_dimension(i) {
        dimensions[i] = Shape::UNBOUNDED_SIZE;
      } else {
        let op_dim = i64::max(operand_shape.dimensions(i)-1, 0);
        dimensions[i] = operand_shape.dimensions(i) + p.edge_padding_low() +
          p.edge_padding_high() + op_dim * p.interior_padding();
        if dimensions[i] < 0 {
          let mut err_msg =
            "Padding result in negative size for dimension ".to_string();
          err_msg.push_str(&i.to_string());
          return Err(err_msg);
        }
      }
      is_dunamic[i] = operand_shape.is_dynamic_dimension(i as i64);
    }

    Ok(ShapeUtil::make_shape_dynamic(
      &ShapeUtil::higher_precision_element_type(
        operand_shape, padding_value_shape),
      dimensions,
      is_dunamic))
  }

  pub fn infer_batch_norm_training_shape(
    _operand_shape: &Shape,
    _scale_shape: &Shape,
    _offset_shape: &Shape,
    _feature_index: i64) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_batch_norm_inference_shape(
    _operand_shape: &Shape,
    _scale_shape: &Shape,
    _offset_shape: &Shape,
    _mean_shape: &Shape,
    _variance_shape: &Shape,
    _feature_index: i64) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_batch_norm_grad_shape(
    _operand_shape: &Shape,
    _scale_shape: &Shape,
    _mean_shape: &Shape,
    _variance_shape: &Shape,
    _output_grad_shape: &Shape,
    _feature_index: i64) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_gather_shape(
    _input_shape: &Shape,
    _start_indices_shape: &Shape,
    _gather_dim_numbers: &GatherDimensionNumbers,
    _slice_sizes: &Vec<i64>) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_get_dimension_size_shape(
    _shape: &Shape,
    _dimension: i64) -> Result<Shape, String>
  {
    unimplemented!()
  }

  pub fn infer_set_dimension_size_shape(
    _shape: &Shape,
    _val_shape: &Shape,
    _dimension: i64) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_dot_op_shape(
    _lhs: &Shape,
    _rhs: &Shape,
    _dimension_numbers: &DotDimensionNumbers,
    _preferred_element_type: Option<PrimitiveType>,
    _sparsity: &Vec<SparsityDescriptor>) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_convolve_shape(
    _lhs: &Shape,
    _rhs: &Shape,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _window: &Window,
    _dnums: &ConvolutionDimensionNumbers,
    _preferred_element_type: Option<PrimitiveType>) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_while_shape(
    _condition: &ProgramShape,
    _body: &ProgramShape,
    _init: &Shape) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_broadcast_shape(
    _operand: &Shape,
    _broadcast_sizes: &Vec<i64>) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_reshape_shape(
    _arg_shapes: &Vec<Shape>,
    _dimensions_to_reduce: &Vec<i64>,
    _to_apply: &ProgramShape) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_window_from_dimensions(
    _window_dimensions: &Vec<i64>,
    _window_strides: &Vec<i64>,
    _padding: &Vec<(i64, i64)>,
    _lhs_dilation: &Vec<i64>,
    _rhs_dilation: &Vec<i64>,
    _window_reversal: Option<&Vec<bool>>) -> Result<Window, String>
  {
    unimplemented!()    
  }

  pub fn infer_call_shape(
    _shapes: &Vec<Shape>,
    _program: &ProgramShape) -> Result<Shape, String>
  {
    unimplemented!()
  }

  // Infers the shape produced by scattering the given source shape to the
  // selected indices of each window on the operand shape.
  pub fn infer_select_and_scatter_shape(
    _operand_shape: &Shape,
    _select_shape: &ProgramShape,
    _window: &Window,
    _source_shape: &Shape,
    _init_value_shape: &Shape,
    _scatter_shape: &ProgramShape) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_conditional_shape(
    _branch_index: &Shape,
    _branch_computations: &Vec<ProgramShape>,
    _branch_operands: &Vec<Shape>) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_scatter_shape(
    _arg_shapes: &Vec<Shape>,
    _to_apply_shape: &ProgramShape,
    _scatter_dim_numbers: &ScatterDimensionNummbers) -> Result<Shape, String>
  {
    unimplemented!()    
  }

  pub fn infer_scalar_broadcast_shape(
    _shapes: &Vec<Shape>) -> Result<Option<Shape>, String>
  {
    unimplemented!()
  }
}