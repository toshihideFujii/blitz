#![allow(dead_code)]

use common::{
  blitz_data::FftType, comparison_util::{ComparisonType, Direction}, literal::Literal, shape::Shape
};

use crate::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  //hlo_opcode::HloOpcode
};

pub struct HloTopKInstruction {
  k: i64,
  largest: bool,
}

impl HloTopKInstruction {
  pub fn new(
    _shape: &Shape,
    _input: &HloInstruction,
    k: i64,
    largest: bool) -> Self
  {
    HloTopKInstruction {
      k: k,
      largest: largest
    }
  }
}

pub struct HloConstantInstruction {
  literal: Literal
}

impl HloConstantInstruction {
  pub fn new(literal: Literal) -> Self {
    HloConstantInstruction {
      literal: literal
    }
  }

  // Returns the literal associated with this instruction.
  pub fn literal(&self) -> &Literal {
    &self.literal
  }

  pub fn mutable_lietral() {}
  pub fn has_literal() {}
}

pub struct HloParameterInstruction {
  parameter_number: i64,
  parameter_replicated_at_leaf_buffers: Vec<bool>,
}

impl HloParameterInstruction {
  pub fn new(parameter_number: i64, _shape: &Shape, _name: String) -> Self {
    HloParameterInstruction {
      parameter_number: parameter_number,
      parameter_replicated_at_leaf_buffers: Vec::new()
    }
  }

  pub fn parameter_number(&self) -> i64 {
    self.parameter_number
  }

  pub fn set_parameter_replicated_at_leaf_buffers() {}

  pub fn parameter_replicated_at_leaf_buffers(&self) -> &Vec<bool> {
    &self.parameter_replicated_at_leaf_buffers
  }
}

pub struct HloGetTupleElementInstruction {
  tuple_index: i64
}

impl HloGetTupleElementInstruction {
  pub fn new(_shape: &Shape, _operand: &HloInstruction, index: i64) -> Self {
    HloGetTupleElementInstruction {
      tuple_index: index
    }
  }

  pub fn tuple_index(&self) -> i64 {
    self.tuple_index
  }

  pub fn set_tuple_index(&mut self, tuple_index: i64) {
    self.tuple_index = tuple_index;
  }
}

pub struct HloIotaInstruction {
  iota_dimension: i64,
}

impl HloIotaInstruction {
  pub fn new(_shape: &Shape, iota_dimension: i64) -> Self {
    HloIotaInstruction {
      iota_dimension: iota_dimension
    }
  }

  // Returns the dimension sizes or numbers associated with this instruction.
  pub fn iota_dimension(&self) -> i64 {
    self.iota_dimension
  }
}

pub struct HloMapInstruction {}

impl HloMapInstruction {
  pub fn new(
    _shape: &Shape,
    _operands: Vec<HloInstruction>,
    _map_computation: HloComputation) -> Self
  {
    HloMapInstruction {  }
  }
}

pub struct HloAsyncInstruction {}

impl HloAsyncInstruction {
  pub fn new(_shape: &Shape, _operand: HloInstruction) -> Self {
    HloAsyncInstruction {  }
  }

  pub fn async_wrapped_computation() {}
  pub fn async_wrapped_instruction() {}
  pub fn async_wrapped_opcode() {}
}

pub struct HloAsyncStartInstruction {
  async_execution_thread: String,
}

impl HloAsyncStartInstruction {
  pub fn new(
    //_opcode: HloOpcode,
    _shape: &Shape,
    _operands: Vec<HloInstruction>,
    _async_computation: HloComputation,
    async_execution_thread: String) -> Self
  {
    HloAsyncStartInstruction {
      async_execution_thread: async_execution_thread
    }
  }
}

pub struct HloCopyStartInstruction {
  cross_program_prefetch_index: Option<i64>
}

impl HloCopyStartInstruction {
  pub fn new(
    _shape: &Shape,
    _operand: HloInstruction,
    cross_program_prefetch_index: Option<i64>) -> Self
  {
    HloCopyStartInstruction {
      cross_program_prefetch_index: cross_program_prefetch_index
    }
  }
}

pub struct HloCompareInstruction {}

impl HloCompareInstruction {
  pub fn new(
    _shape: &Shape,
    _lhs: &HloInstruction,
    _rhs: &HloInstruction,
    _direction: Direction,
    _t: ComparisonType) -> Self
  {
    HloCompareInstruction {  }
  }
}

pub struct HloReduceInstruction {}

impl HloReduceInstruction {
  pub fn new(
    _shape: &Shape,
    _args: Vec<HloInstruction>,
    _dimensions_to_reduce: Vec<i64>,
    _reduce_computation: HloComputation) -> Self
  {
    HloReduceInstruction {  }
  }
}

pub struct HloReducePrecisionInstruction {
  exponent_bits: i32,
  mantissa_bits: i32,
}

impl HloReducePrecisionInstruction {
  pub fn new(
    _shape: &Shape,
    _operand: HloInstruction,
    exponent_bits: i32,
    mantissa_bits: i32) -> Self
  {
    HloReducePrecisionInstruction {
      exponent_bits: exponent_bits,
      mantissa_bits: mantissa_bits
    }
  }
}

pub struct HloInfeedInstruction {
  infeed_config: String,
}

impl HloInfeedInstruction {
  pub fn new(
    _infeed_shape: &Shape,
    _token_operand: HloInstruction,
    infeed_config: String) -> Self
  {
    HloInfeedInstruction {
      infeed_config: infeed_config
    }
  }
}

pub struct HloOutfeedInstruction {
  outfeed_config: String,
}

impl HloOutfeedInstruction {
  pub fn new(
    _outfeed_shape: &Shape,
    _operand: HloInstruction,
    _token_operand: HloInstruction,
    outfeed_config: String) -> Self
  {
    HloOutfeedInstruction {
      outfeed_config: outfeed_config
    }
  }
}

pub struct HloSendInstruction {}

impl HloSendInstruction {
  pub fn new(
    _operand: HloInstruction,
    _token: HloInstruction,
    _channel_id: i64,
    _is_host_transfer: bool) -> Self
  {
    HloSendInstruction {  } 
  }
}

pub struct HloSendDoneInstruction {}

impl HloSendDoneInstruction {
  pub fn new(_operand: HloInstruction, _is_host_transffer: bool) -> Self {
    HloSendDoneInstruction {  }
  }
}

pub struct HloRecvInstruction {}

impl HloRecvInstruction {
  pub fn new(
    _shape: &Shape,
    _token: HloInstruction,
    _channel_id: i64,
    _is_host_transfer: bool) -> Self
  {
    HloRecvInstruction {  }
  }
}

pub struct HloRecvDoneInstruction {}

impl HloRecvDoneInstruction {
  pub fn new(_operand: HloRecvInstruction, _is_host_transfer: bool) -> Self {
    HloRecvDoneInstruction {  }
  }
}

pub struct HloSliceInstruction {
  slice_starts: Vec<i64>,
  slice_limits: Vec<i64>,
  slice_strides: Vec<i64>,
}

impl HloSliceInstruction {
  pub fn new(
    _shape: &Shape,
    _operand: HloInstruction,
    start_indices: Vec<i64>,
    limit_indices: Vec<i64>,
    strides: Vec<i64>) -> Self
  {
    HloSliceInstruction {
      slice_starts: start_indices,
      slice_limits: limit_indices,
      slice_strides: strides
    }
  }
}

pub struct HloDynamicSliceInstruction {}

impl HloDynamicSliceInstruction {
  pub fn new(
    _shape: &Shape,
    _operand: HloInstruction,
    _start_indices: Vec<HloInstruction>,
    _slice_sizes: Vec<usize>) -> Self
  {
    HloDynamicSliceInstruction {  }
  }
}

pub struct HloDynamicUpdateSliceInstruction {}

impl HloDynamicUpdateSliceInstruction {
  pub fn new(
    _shape: &Shape,
    _operand: HloInstruction,
    _update: HloInstruction,
    _start_indices: Vec<HloInstruction>) -> Self
  {
    HloDynamicUpdateSliceInstruction {  }
  }
}

pub struct HloConcatenateInstruction {}

impl HloConcatenateInstruction {
  pub fn new(_shape: &Shape, _operands: Vec<HloInstruction>, _dimension: i64) -> Self {
    HloConcatenateInstruction {  }
  }
}

pub struct HloBatchNormTrainingInstruction {}

impl HloBatchNormTrainingInstruction {
  pub fn new(
    _shape: &Shape,
    _operand: HloInstruction,
    _scale: HloInstruction,
    _offset: HloInstruction,
    _epsilon: f64,
    _feature_index: i64) -> Self
  {
    HloBatchNormTrainingInstruction {  }
  }
}

pub struct HloBatchNormInferenceInstruction {}

impl HloBatchNormInferenceInstruction {
  pub fn new(
    _shape: &Shape,
    _operand: HloInstruction,
    _scale: HloInstruction,
    _offset: HloInstruction,
    _mean: HloInstruction,
    _variance: HloInstruction,
    _epsilon: f64,
    _feature_index: i64) -> Self
  {
    HloBatchNormInferenceInstruction {  }    
  }
}

pub struct HloBatchNormGradInstruction {}

impl HloBatchNormGradInstruction {
  pub fn new(
    _shape: &Shape,
    _operand: HloInstruction,
    _scale: HloInstruction,
    _mean: HloInstruction,
    _variance: HloInstruction,
    _grad_output: HloInstruction,
    _epsilon: f64,
    _feature_index: i64) -> Self
  {
    HloBatchNormGradInstruction {  }    
  }
}

pub struct HloBroadcastInstruction {}

impl HloBroadcastInstruction {
  pub fn new(
    _shape: &Shape,
    _operand: HloInstruction,
    _broadcast_dimension: Vec<i64>) -> Self
  {
    HloBroadcastInstruction {  }
  }
}

pub struct HloReshapeInstruction {}

impl HloReshapeInstruction {
  pub fn new(_shape: &Shape, _operand: HloInstruction, _inferred_dimension: i64) -> Self {
    HloReshapeInstruction {  }
  }
}

pub struct HloDynamicReshapeInstruction {}

impl HloDynamicReshapeInstruction {
  pub fn new(
    _shape: &Shape,
    _data_operand: HloInstruction,
    _dim_sizes: Vec<HloInstruction>) -> Self
  {
    HloDynamicReshapeInstruction {  }    
  }
}

pub struct HloTransposeInstruction {}

impl HloTransposeInstruction {
  pub fn new(_shape: &Shape, _operand: HloInstruction, _dimensions: Vec<i64>) -> Self {
    HloTransposeInstruction {  }
  }
}

pub struct HloSortInstruction {
  is_stable: bool
}

impl HloSortInstruction {
  pub fn new(
    _shape: &Shape,
    _dimension: i64,
    _operands: Vec<HloInstruction>,
    _compare: HloComputation,
    is_stable: bool) -> Self
  {
    HloSortInstruction {
      is_stable: is_stable
    }
  }
}

pub struct HloCallInstruction {}

impl HloCallInstruction {
  pub fn new(_shape: &Shape, _called_computation_root: HloInstruction) -> Self {
    HloCallInstruction {  }
  }
}

pub struct HloFftInstruction {
  fft_type: FftType
}

impl HloFftInstruction {
  pub fn fft_type(&self) -> FftType {
    self.fft_type.clone()
  }
}