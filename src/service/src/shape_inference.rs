#![allow(dead_code)]

use common::{blitz_data::{ConvolutionDimensionNumbers, DotDimensionNumbers, FftType, GatherDimensionNumbers, PaddingConfig, PrimitiveType, SparsityDescriptor, TriangularSolveOptions, Window}, shape::{ProgramShape, Shape}};
use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};

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

  pub fn infer_binary_op_shape(
    _opcode: &HloOpcode,
    _lhs: &HloInstruction,
    _rhs: &HloInstruction) -> Result<Shape, String>
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

  pub fn infer_variadic_op_shape(
    _opcode: &HloOpcode,
    _operands: &Vec<HloInstruction>) -> Result<Shape, String>
  {
    unimplemented!()    
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

  pub fn infer_all_gether_done_shape(_all_gather_start_shape: &Shape) -> Result<Shape, String> {
    unimplemented!()
  }

  pub fn infer_all_reduce_done_shape(_operand_shape: &Shape) -> Result<Shape, String> {
    unimplemented!()
  }

  pub fn infer_collective_permute_done_shape(_operand_shape: &Shape) -> Result<Shape, String> {
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

  pub fn infer_pad_shape(
    _operand_shape: &Shape,
    _padding_value_shape: &Shape,
    _padding_config: &PaddingConfig) -> Result<Shape, String>
  {
    unimplemented!()    
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

  pub fn infer_reshape_shpae(
    _operand: &Shape,
    _dimensions: &Vec<i64>,
    _new_sizes: &Vec<i64>,
    _inferred_dimension: i64) -> Result<Shape, String>
  {
    unimplemented!()    
  }
}