#![allow(dead_code)]

use common::{blitz_data::ReplicaGroup, shape::{Shape}};
use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction};

use crate::collective_ops_utils::CollectiveOpGroupMode;

pub fn create_start_indices_for_collective_decomposition(
  _group_mode: &CollectiveOpGroupMode,
  _replica_groups: &Vec<ReplicaGroup>,
  _shard_shape: &Shape,
  _shard_dimension: i64,
  _computation: &HloComputation,
  _update_layout: Option<&dyn Fn(&Shape)>) -> Result<Vec<HloInstruction>, String>
{
  unimplemented!()
}