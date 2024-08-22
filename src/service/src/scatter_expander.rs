#![allow(dead_code)]

use common::{
  blitz_data::ScatterDimensionNummbers,
  comparison_util::ComparisonDirection,
  shape::Shape
};

use hlo::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule, hlo_opcode::HloOpcode
};

use crate::hlo_creation_utils::{
  broadcast_zeros, collapse_first_n_dims, make_binary_hlo, make_compare_hlo,
  make_r1_constant_hlo, make_transpose_hlo, prepend_degenerate_dims
};

#[derive(Debug, Clone, PartialEq)]
pub enum ScatterExpanderMode {
  EliminateAllScatters,
  EliminateSimpleScatters,
  EliminateIndeterminisitcScatters
}

// This pass rewrites scatter operations into (roughly) while loops of
// dynamic-update-slices.
//
// This pass can be used in two ways:
//
//   - kEliminateAllScatters: For backends that don't support scatter, this pass
//     can convert every scatter into a loop.
//
//   - kEliminateSimpleScatters: For backends that *do* support scatter, this
//     pass can strength-reduce "simple" scatters -- specifically, scatters that
//     can be represented without a loop -- to dynamic-update-slices.
//
//   - kEliminateIndeterminisitcScatters: For backends that *do* support
//     scatter, this pass converts scatters with potentially indeterminisitc
//     behavior, because of non-unique indices or non-associative combiner
//     functions. There may be false positives, but no false negatives, i.e.
//     some scatters are converted even when deterministic in practice.
//
// Note that even in kEliminateSimpleScatters mode, this pass may still expand a
// scatter into a loop (with a trip-count of 1).  It's up to other
// simplification passes to remove the loop.
pub struct ScatterExpander {
  mode: ScatterExpanderMode
}

impl ScatterExpander {
  pub fn new(mode: ScatterExpanderMode) -> Self {
    ScatterExpander { mode: mode }
  }

  pub fn name(&self) -> String {
    "scatter-expander".to_string()
  }

  pub fn instruction_matches_pattern(&self, instruction: &HloInstruction) -> bool {
    self.mode == ScatterExpanderMode::EliminateAllScatters ||
    (self.mode == ScatterExpanderMode::EliminateSimpleScatters &&
     scatter_trip_count(instruction) == 1) ||
    (self.mode == ScatterExpanderMode::EliminateIndeterminisitcScatters &&
     !is_deterministic(instruction))
  }

  pub fn expand_instruction(&self, _instruction: &HloInstruction) -> Result<HloInstruction, String> {
    unimplemented!()
  }
}

// Transposes the given scatter_indices such that the index_vector_dim becomes
// the most-minor dimension.
fn transpose_index_vector_dim_to_last(
  scatter_indices: &HloInstruction,
  index_vector_dim: i64) -> Result<HloInstruction, String>
{
  let scatter_indices_shape = scatter_indices.shape();
  if scatter_indices_shape.dimensions_size() == index_vector_dim as usize {
    return Ok(scatter_indices.clone());
  }

  if index_vector_dim as usize == scatter_indices_shape.dimensions_size() - 1 {
    return Ok(scatter_indices.clone());
  }

  let mut permutation = vec![];
  permutation.reserve(scatter_indices_shape.dimensions_size());
  for i in 0..scatter_indices_shape.dimensions_size() {
    if i != index_vector_dim as usize {
      permutation.push(i as i64);
    }
  }
  permutation.push(index_vector_dim);
  make_transpose_hlo(scatter_indices, &permutation)
}

// Canonicalizes the scatter_indices tensor in order to keep them uniform while
// performing the scatter operation.
fn canonicalize_scatter_indices(
  scatter_indices: &HloInstruction,
  index_vector_dim: i64) -> Result<HloInstruction, String>
{
  let transposed_scatter_indices =
    transpose_index_vector_dim_to_last(scatter_indices, index_vector_dim);
  if transposed_scatter_indices.is_err() {
    return Err(transposed_scatter_indices.err().unwrap());
  }

  if scatter_indices.shape().rank() == (index_vector_dim + 1) as usize &&
     scatter_indices.shape().dimensions(index_vector_dim as usize) == 1
  {
    // TODO
  }

  let indices_are_scalar =
    index_vector_dim as usize == scatter_indices.shape().dimensions_size();

  // The number of dimensions in scatter_indices that are index dimensions.
  let mut index_dims_in_scatter_indices = 1;
  if indices_are_scalar { index_dims_in_scatter_indices = 0; }

  // If there is only one index (i.e. scatter_indices has rank 1 and this
  // scatter is really just a dynamic update slice) add a leading degenerate
  // dimension for uniformity.  Otherwise create a "collapsed" leading dimension
  // that subsumes all of the non-index-vector dimensions.
  let shape = transposed_scatter_indices.as_ref().unwrap().shape();
  if shape.dimensions_size() == index_dims_in_scatter_indices {
    prepend_degenerate_dims(transposed_scatter_indices.as_ref().unwrap(), 1)
  } else {
    // Collapse all but the dimensions (0 or 1) in scatter_indices containing
    // the index vectors.
    collapse_first_n_dims(transposed_scatter_indices.as_ref().unwrap(),
      (shape.dimensions_size() - index_dims_in_scatter_indices) as i64)
  }
}

// Permutes the `updates` tensor such that all the scatter dims appear in the
// major dimensions and all the window dimensions appear in the minor
// dimensions.
fn permute_scatter_and_window_dims(
  updates: &HloInstruction,
  update_window_dims: &Vec<i64>) -> Result<HloInstruction, String>
{
  let mut permutation: Vec<i64> = vec![];
  let updates_rank = updates.shape().rank();
  permutation.reserve(updates_rank);

  for i in 0..updates_rank {
    let is_scatter_dim = update_window_dims.binary_search(&(i as i64));
    if is_scatter_dim.is_err() {
      permutation.push(i as i64);
    }
  }
  for window_dim in update_window_dims {
    permutation.push(*window_dim)
  }

  make_transpose_hlo(updates, &permutation)
}

// Expands or contracts the scatter indices in the updates tensor.
fn adjust_scatter_dims(
  scatter_indices_shape: &Shape,
  updates: &HloInstruction,
  index_vector_dim: i64) -> Result<HloInstruction, String>
{
  let mut num_scatter_dims = scatter_indices_shape.dimensions_size();
  if (index_vector_dim as usize) < scatter_indices_shape.dimensions_size() {
    num_scatter_dims -= 1;
  }
  if num_scatter_dims == 0 {
    // If there are no scatter dims, this must be a dynamic-update-slice kind of
    // scatter. In this case, we prepend a degenerate dimension to work
    // uniformly in the while loop.
    return prepend_degenerate_dims(updates, 1);
  }
  collapse_first_n_dims(updates, num_scatter_dims as i64)
}

// Expands an index vector from the scatter_indices tensor into a vector that
// can be used to dynamic-update-slice to perform the scatter update.
fn expand_index_vector_into_operand_space(
  index_vector: &HloInstruction,
  _dim_numbers: &ScatterDimensionNummbers,
  operand_rank: i64) -> Result<HloInstruction, String>
{
  let _computation = index_vector.parent();
  let _index_shape = index_vector.shape();

  // Scatter of a scalar. Return a zero-sized vector of indices.
  if operand_rank == 0 {
    // TODO
  }

  unimplemented!()
}

fn check_index_validity(
  computation: &HloComputation,
  index: &HloInstruction,
  operand_dims: &Vec<i64>,
  window_sizes: &Vec<i64>,
  _module: &HloModule) -> Result<HloInstruction, String>
{
  debug_assert!(operand_dims.len() == window_sizes.len());

  // Check if the index has any negative values.
  let zero_index = broadcast_zeros(
    computation,
    &index.shape().element_type(),
    index.shape().dimensions_vec());

  let negative_index_check =
    make_compare_hlo(
      &ComparisonDirection::Le,
      &zero_index,
      index,
      None,
      None);
  if negative_index_check.is_err() {
    return Err(negative_index_check.err().unwrap());
  }

  // Check if the index is OOB w.r.t. the operand dimensions and window sizes.
  let mut max_valid_index = vec![];
  for i in 0..operand_dims.len() {
    max_valid_index.insert(i, operand_dims[i] - window_sizes[i]);
  }

  let max_valid_index_constant =
    make_r1_constant_hlo::<i64>(
      computation,
      index.shape().element_type(),
      &max_valid_index);
  if max_valid_index_constant.is_err() {
    return Err(max_valid_index_constant.err().unwrap());
  }

  let oob_index_check =
    make_compare_hlo(
      &ComparisonDirection::Ge,
      max_valid_index_constant.as_ref().unwrap(),
      index,
      None,
      None);
  if oob_index_check.is_err() {
    return Err(oob_index_check.err().unwrap());
  }

  // Combine the results of the two checks above.
  let _valid_index =
    make_binary_hlo(
      HloOpcode::And,
      negative_index_check.as_ref().unwrap(),
      oob_index_check.as_ref().unwrap(),
      None,
      None);

  // TODO

  unimplemented!()
}

fn call_and_get_output(
  original: &HloComputation, _output_index: i64) -> Result<HloComputation, String>
{
  let original_root = original.root_instruction();
  if !original_root.shape().is_tuple()  {
    return Ok(original.clone());
  }

  // TODO
  unimplemented!()
}

// Body of the while loop that performs the scatter operation using other HLOs.
fn scatter_loop_body(
  _scatter: &HloInstruction,
  _induction_var: &HloInstruction,
  _loop_state: Vec<HloInstruction>) -> Result<Vec<HloInstruction>, String>
{
  unimplemented!()   
}

fn scatter_trip_count(scatter: &HloInstruction) -> i64 {
  // Compute the trip count for the while loop to be used for scatter. This
  // should be the number of indices we should scatter into the operand.
  let scatter_indices = scatter.scatter_indices();
  let scatter_indices_shape = scatter_indices.shape();
  let dim_numbers = scatter.scatter_dimension_numbers();
  let mut scatter_loop_trip_count = 1;
  for i in 0..scatter_indices_shape.dimensions_size() {
    if i != dim_numbers.index_vector_dim() as usize {
      scatter_loop_trip_count *= scatter_indices_shape.dimensions(i);
    }
  }
  scatter_loop_trip_count
}

fn is_combiner_associative(combiner: &HloComputation) -> bool {
  // Consider simple binary combiner functions only.
  if combiner.instruction_count() != 3 {
    return false;
  }
  match combiner.root_instruction().opcode() {
    // Minimum and Maximum are common associative combiners.
    HloOpcode::Minimum => return true,
    HloOpcode::Maximum => return true,
    // Other common combiners are associative at least for integer arithmetic.
    HloOpcode::Add => return combiner.root_instruction().shape().is_integer(),
    HloOpcode::Multiply => return combiner.root_instruction().shape().is_integer(),
    HloOpcode::Or => return combiner.root_instruction().shape().is_integer(),
    HloOpcode::Xor => return combiner.root_instruction().shape().is_integer(),
    _ => return false
  }
}

fn is_deterministic(scatter: &HloInstruction) -> bool {
  if scatter.unique_indices() {
    return true;
  }
  if is_combiner_associative(scatter.to_apply()) {
    return true;
  }
  false
}