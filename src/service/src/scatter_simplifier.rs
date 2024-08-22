#![allow(dead_code)]

use common::permutation_util::is_identity_permutation;
use hlo::hlo_instruction::HloInstruction;

use crate::hlo_creation_utils::insert_degenerate_dims;

// This pass rewrites scatter operations into a combination of transposes,
// reshapes and a simpler scatter.
//
// It implements the first two steps of the algorithm decribed in
// ScatterExpander::ExpandInstruction (scatter_expander.cc). Additionally, it
// transposes updates and operands to transform scatter_dims_to_operand_dims
// into the identity mapping. This is different from the algorithm in
// ScatterExpander, which instead applies the mapping in scatter_indices.
//
// The output scatter's attributes will have the following characteristics:
// - scatter_indices is a two-dimensional tensor
// - index_vector_dim is 1
// - inserted_window_dims is []
// - update_window_dims is [0, 1, ...]
// - scatter_dims_to_operand_dims is [0, 1, ...]
//
// The purpose of this pass is to check whether this transformation has any
// performance implications.
pub struct ScatterSimplifier {}

impl ScatterSimplifier {
  pub fn new() -> Self {
    ScatterSimplifier {  }
  }

  pub fn name(&self) -> String {
    "scatter-simplifier".to_string()
  }

  pub fn is_simplified_scatter(scatter: &HloInstruction) -> bool {
    let dims = scatter.scatter_dimension_numbers();

    let nonstandard_index_vector_dim =
      dims.index_vector_dim() != (scatter.scatter_indices().shape().rank() - 1) as i64;

    let num_scatter_dims =
      scatter.scatter_updates().first().unwrap().shape().rank() - dims.update_window_dims().len();

    let scatter_indices_reordered =
      !is_identity_permutation(dims.scatter_dims_to_operand_dims());

    let mut scatter_dim_not_first = false;
    for i in  dims.update_window_dims() {
      if *i == 0 { scatter_dim_not_first = true; }
    }
    
    !(nonstandard_index_vector_dim || num_scatter_dims > 1 ||
      scatter_indices_reordered || scatter_dim_not_first ||
      !dims.inserted_window_dims().is_empty())
  }

  pub fn instruction_matches_pattern(inst: &HloInstruction) -> bool {
    !ScatterSimplifier::is_simplified_scatter(inst)
  }

  pub fn expand_instruction(_inst: &HloInstruction) -> Result<HloInstruction, String> {
    unimplemented!()
  }
}

fn flatten_and_transpose_updates(
  updates: &mut HloInstruction,
  update_window_dims: &Vec<i64>,
  inserted_window_dims: &Vec<i64>,
  _scatter_indices_size: i64) -> Result<HloInstruction, String>
{
  let updates_rank = updates.shape().rank();

  let mut permutation = vec![];
  let num_scatter_dims = updates_rank - update_window_dims.len();
  permutation.reserve(updates_rank);

  // Move the scatter dimensions to the front.
  for i in 0..updates_rank {
    // update_window_dims is small, so linear search is acceptable.
    if update_window_dims.binary_search(&(i as i64)).is_err() {
      permutation.push(i);
    }
  }

  // Followed by the update_window_dims.
  // TODO

  // Collapse scatter dimensions to one.
  if num_scatter_dims > 1 {
    //updates = collapse_first_n_dims(
      //&updates, num_scatter_dims as i64).as_mut().unwrap();
  } else if num_scatter_dims == 0 {
    // TODO
  }
  
  // Insert size 1 dimensions.
  if !inserted_window_dims.is_empty() {
    let mut new_dims = vec![];
    new_dims.reserve(inserted_window_dims.len());
    for i in inserted_window_dims {
      new_dims.push(i + 1);
    }
    return insert_degenerate_dims(&updates, &new_dims);
  }

  Ok(updates.clone())
}