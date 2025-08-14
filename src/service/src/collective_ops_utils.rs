#![allow(dead_code)]

use common::{blitz_data::PrimitiveType, literal::Literal};
use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction};

#[derive(Debug, Clone, PartialEq)]
pub enum ReductionKind {
  Sum,
  Product,
  Min,
  Max,
}

// Attempts to match instruction to one of the possible cases for ReductionKind.
pub fn match_reduction_instruction(_hlo: &HloInstruction) -> Option<ReductionKind>
{
  unimplemented!()
}

// Attempts to match computation to one of the possible cases in ReductionKind.
pub fn match_reduction_computation(
  _computation: &HloComputation) -> Option<ReductionKind>
{
  unimplemented!()
}

// Returns the reduction identity value for a certain ReductionKind and
// PrimitiveType.
pub fn get_reduction_identity<T>(
  _kind: &ReductionKind, _t: &PrimitiveType) -> Option<Literal<T>>
  where  T: Default + Clone + PartialEq
{
  unimplemented!()
}

pub enum CollectiveOpGroupMode {
  CrossReplica,
  CrossPartition,
  CrossReplicaAndPartition,
  FlattenedID,
}

pub fn get_participating_ids() {}

pub fn collective_op_group_mode_to_string() {}

// Returns the group formation mode implied by (a) whether the operation has
// channel_id and (b) if it has use_global_device_ids and if yes, its value.
pub fn get_collective_op_group_mode(
  _has_channel_id: bool,
  _use_global_device_ids: Option<bool>) -> Result<CollectiveOpGroupMode, String>
{
  unimplemented!()
}

pub fn get_participating_device_groups() {}

pub fn get_participating_flattened_id_groups() {}

pub fn get_participating_devices() {}

pub fn get_participant_counts_for_replica_groups() {}

pub fn replica_groups_orthogonal() {}

pub fn replica_groups_equal() {}

pub fn is_collective() {}

pub fn is_sync_collective() {}