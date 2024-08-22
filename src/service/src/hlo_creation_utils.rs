#![allow(dead_code)]

use common::{
  blitz_data::{FrontendAttributes, OpMetadata, PrimitiveType},
  comparison_util::ComparisonDirection, shape::Shape
};
use hlo::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  hlo_opcode::HloOpcode
};

pub fn make_unary_hlo() {}

// Creates a binary HLO instruction and adds it to the computation containing
// `lhs` and `rhs` (`lhs` and `rhs` must be in the same computation).
pub fn make_binary_hlo(
  _opcode: HloOpcode,
  _lhs: &HloInstruction,
  _rhs: &HloInstruction,
  _metadata: Option<OpMetadata>,
  _frontend_attributes: Option<FrontendAttributes>) -> Result<HloInstruction, String>
{
  unimplemented!()
}

// Creates a compare HLO instruction and adds it to the computation containing
// `lhs` and `rhs` (`lhs` and `rhs` must be in the same computation).
pub fn make_compare_hlo(
  _direction: &ComparisonDirection,
  _lhs: &HloInstruction,
  _rhs: &HloInstruction,
  _metadata: Option<OpMetadata>,
  _frontend_attributes: Option<FrontendAttributes>) -> Result<HloInstruction, String>
{
  unimplemented!()
}

pub fn make_pad_hlo() {}

pub fn make_slice_hlo() {}

pub fn make_convolve_hlo() {}

// Creates a transpose HLO instruction and adds it to the computation containing
// `operand`.
pub fn make_transpose_hlo(
  _operand: &HloInstruction,
  _dimensions: &Vec<i64>) -> Result<HloInstruction, String>
{
  unimplemented!()
}

// Creates a reshape HLO instruction and adds it to the computation containing
// `operand`.
pub fn make_reshape_hlo(
  _result_shape: &Shape,
  _operand: &HloInstruction) -> Result<HloInstruction, String>
{
  unimplemented!()
}

pub fn make_dynamic_slice_hlo() {}

pub fn make_dynamic_update_slice_hlo() {}

pub fn make_broadcast_hlo() {}

pub fn make_bget_tuple_element_hlo() {}

pub fn make_concat_hlo() {}

pub fn make_convert_hlo() {}

pub fn make_bitcast_hlo() {}

pub fn make_bitcast_convert_to_hlo() {}

pub fn make_iota_hlo() {}

pub fn make_dot_hlo() {}

pub fn make_map_hlo() {}

pub fn make_reduce_precision_hlo() {}

pub fn make_reduce_window_hlo() {}

pub fn make_reduce_hlo() {}

pub fn make_reverse_hlo() {}

pub fn make_select_hlo() {}

pub fn make_sort_hlo() {}

// Creates an R1 Constant HLO instruction of the given PrimitiveType with the
// given values and adds it to the given computation.
pub fn make_r1_constant_hlo<T>(
  _computation: &HloComputation,
  _t: PrimitiveType,
  _values: &Vec<T>) -> Result<HloInstruction, String>
{
  unimplemented!()
}

pub fn make_r0_constant_hlo() {}

pub fn make_scalar_like_hlo() {}

pub fn make_fusion_instruction() {}

// Some other miscellaneous helpers to generate common HLO patterns.  All of
// these add all the instructions they generate into the computation containing
// their operand(s).

// Collapses (via reshape) the first N (logical) dimensions of `operand` into a
// single leading dimension.  `operand` must have rank > `n` and `n` must not be
// 0.
//
// For instance if `operand` has shape f32[7,8,9] and n is 2 then the output is
// the `operand` reshaped to [56,9].
pub fn collapse_first_n_dims(
  _operand: &HloInstruction, _n: i64) -> Result<HloInstruction, String>
{
  unimplemented!()
}

// Prepends `n` degenerate dimensions (dimensions with bound = 1) to `operand`
// using a reshape.
//
// For instance if operand has shape f32[3,4,5] then this returns the operand
// reshaped to f32[1,3,4,5].  If the operand is a f32 scalar (i.e. has shape
// f32[]) then this returns the operand reshaped to f32[1].
pub fn prepend_degenerate_dims(
  _operand: &HloInstruction, _n: i64) -> Result<HloInstruction, String>
{
  unimplemented!()
}

pub fn expand_first_dim_into_n_dims() {}

pub fn elide_degenerate_dims() {}

// Inserts (via reshape) a set of degenerate dimensions (dimensions containing
// exactly one element), `dims_to_insert` into `operand`. The dimensions in
// `dims_to_insert` refer to the dimensions in the result, and hence should be
// less than the rank of the result. Also, `dims_to_insert` must be sorted.
//
// For example, if `operand` is of shape f32[12,21,8,34] and dims_to_insert is
// {0, 2}, then the result is `operand` reshaped to [1,12,1,21,8,34].
pub fn insert_degenerate_dims(
  _operand: &HloInstruction,
  _dims_to_insert: &Vec<i64>) -> Result<HloInstruction, String>
{
  unimplemented!()
}

pub fn pad_vector_with_zeros() {}

// Broadcasts a zero value of type `element_type` into a tensor with element
// type `element_type` and dimension bounds `broadcast_dimensions`.  The
// broadcast instruction is emitted into `computation`.
pub fn broadcast_zeros(
  _computation: &HloComputation,
  _element_type: &PrimitiveType,
  _broadcast_dimensions: &Vec<i64>) -> HloInstruction
{
  unimplemented!()
}

pub fn broadcast_ones() {}

pub fn create_computation_with_signature() {}

pub fn expand_degenerate_reshape() {}