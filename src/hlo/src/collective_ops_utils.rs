#![allow(dead_code)]

use common::{
  blitz_data::{PrimitiveType, ReplicaGroup},
  computation_placer::DeviceAssignment,
  literal::Literal, literal_util::LiteralUtil, primitive_util::is_complex_type
};
use crate::{
  collective_device_list::CollectiveDeviceList, hlo_computation::HloComputation,
  hlo_instruction::HloInstruction, hlo_opcode::HloOpcode
};

#[derive(Debug, Clone, PartialEq)]
pub enum ReductionKind {
  Sum,
  Product,
  Min,
  Max,
}

// Attempts to match instruction to one of the possible cases for ReductionKind.
pub fn match_reduction_instruction(hlo: &HloInstruction) -> Option<ReductionKind>
{
  match hlo.opcode() {
    HloOpcode::Add => Some(ReductionKind::Sum),
    HloOpcode::Multiply => Some(ReductionKind::Product),
    HloOpcode::Minimum => Some(ReductionKind::Min),
    HloOpcode::Maximum => Some(ReductionKind::Max),
    HloOpcode::And => {
      let t = hlo.shape().element_type();
      if t == PrimitiveType::Pred {
        return Some(ReductionKind::Min);
      } else {
        return None;
      }
    }
    HloOpcode::Or => {
      let t = hlo.shape().element_type();
      if t == PrimitiveType::Pred {
        return Some(ReductionKind::Max);
      } else {
        return None;
      }
    }
    _ => None
  }
}

// Attempts to match computation to one of the possible cases in ReductionKind.
pub fn match_reduction_computation(
  computation: &HloComputation) -> Option<ReductionKind>
{
  let root = computation.root_instruction();
  let kind = match_reduction_instruction(root);
  if kind.is_some() {
    // TODO
  }
  kind
}

// Returns the reduction identity value for a certain ReductionKind and
// PrimitiveType.
pub fn get_reduction_identity<T>(
  kind: &ReductionKind, t: &PrimitiveType) -> Option<Literal>
  where  T: Clone
{
  match kind {
    ReductionKind::Sum => Some(LiteralUtil::zero::<T>(t)),
    ReductionKind::Product => Some(LiteralUtil::one::<T>(t)),
    ReductionKind::Min => {
      if is_complex_type(t) {
        return None;
      }
      return Some(LiteralUtil::max_value::<T>(t));
    }
    ReductionKind::Max => {
      if is_complex_type(t) {
        return None;
      }
      return Some(LiteralUtil::min_value::<T>(t));
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollectiveOpGroupMode {
  CrossReplica,
  CrossPartition,
  CrossReplicaAndPartition,
  FlattenedID,
}

// Figures out which IDs are participating in the collective subgroup.
// An empty `groups` indicates that all [0, total_participant_count) IDs
// are participating. Note that for CollectiveOpGroupMode::kFlattenedID,
// groups cannot be empty, so `total_participant_count` is an optional.
pub fn get_participating_ids(
  _g_mode: CollectiveOpGroupMode,
  current_id: i64,
  total_participant_count: Option<i64>,
  groups: &Vec<ReplicaGroup>) -> Result<Vec<i64>, String>
{
  // Empty replica_groups() means that all replicas participate.
  if groups.is_empty() {
    let all_participants =
      vec![0; total_participant_count.unwrap() as usize];
    return Ok(all_participants);
  }
  // Formatter for printing replica groups in StrJoin.
  /*
  let g_formatter =
    |out: &mut String, group: &ReplicaGroup|
  {
    out.push_str("[");
    let mut count = 0;
    for id in group.replica_ids() {
      out.push_str(&id.to_string());
      if count != group.replica_ids().len() {
        out.push_str(", ");
      }
      count += 1;
    }
    out.push_str("]");
  };
  */
  // Figure out the other replicas that go together with this one.
  let group: Option<ReplicaGroup> = None;
  for g in groups {
    for rep_id in g.replica_ids() {
      if *rep_id == current_id {
        let mut err_msg = "Replica ID ".to_string();
        err_msg.push_str(&current_id.to_string());
        err_msg.push_str(" appears twice in replica groups");
        return Err(err_msg);
      }
    }
  }
  if group.is_none() {
    let mut err_msg = "Replica ID ".to_string();
    err_msg.push_str(&current_id.to_string());
    err_msg.push_str(" doesn't appear in replica groups");
    return Err(err_msg);
  }
  let mut result = vec![];
  result.clone_from(group.unwrap().replica_ids());
  Ok(result)
}

pub fn collective_op_group_mode_to_string(
  group_mode: &CollectiveOpGroupMode) -> String
{
  match group_mode {
    CollectiveOpGroupMode::CrossReplica => "cross_replica".to_string(),
    CollectiveOpGroupMode::CrossPartition => "cross_partition".to_string(),
    CollectiveOpGroupMode::CrossReplicaAndPartition => "cross_replica_and_partition".to_string(),
    CollectiveOpGroupMode::FlattenedID => "flattened_id".to_string()
  }    
}

pub fn string_to_collective_op_group_mode(
  name: String) -> Result<CollectiveOpGroupMode, String>
{
  if name.as_str() == "cross_replica" {
    return Ok(CollectiveOpGroupMode::CrossReplica);
  } else if name.as_str() == "cross_partition" {
    return Ok(CollectiveOpGroupMode::CrossPartition);
  } else if name.as_str() == "cross_replica_and_partition" {
    return Ok(CollectiveOpGroupMode::CrossReplicaAndPartition);
  } else if name.as_str() == "flattened_id" {
    return Ok(CollectiveOpGroupMode::FlattenedID);
  }
  Err(name + "is not exist in CollectiveGroupMode")
}

// Returns the group formation mode implied by (a) whether the operation has
// channel_id and (b) if it has use_global_device_ids and if yes, its value.
pub fn get_collective_op_group_mode(
  has_channel_id: bool,
  use_global_device_ids: Option<bool>) -> Result<CollectiveOpGroupMode, String>
{
  if !has_channel_id {
    if use_global_device_ids.is_some() &&
      *use_global_device_ids.as_ref().unwrap() {
      let err_msg =
        "Cannot have use_global_device_ids=true without channel_id".to_string();
      return Err(err_msg);
    }
    return Ok(CollectiveOpGroupMode::CrossReplica);
  }
  if !use_global_device_ids.is_none() {
    return Ok(CollectiveOpGroupMode::CrossPartition);
  }
  if !*use_global_device_ids.as_ref().unwrap() {
    return Ok(CollectiveOpGroupMode::CrossReplicaAndPartition);
  }
  Ok(CollectiveOpGroupMode::FlattenedID)
}

pub fn get_collective_op_group_mode_by_instruction(
  _instr: &HloInstruction) -> Result<CollectiveOpGroupMode, String>
{
  unimplemented!()    
}

pub fn get_collective_replica_groups(instr: &HloInstruction) -> &Vec<ReplicaGroup> {
  instr.replica_groups()
}

// Figures out subgroups of participating devices from given replica_groups and
// group_mode.
//
// Returns list of participants, where each participant is a list of
// GlobalDeviceIds.
//
// For example:
//   device_assignment={{33, 34}, {44, 45}, {55, 56}}  3 replicas 2 partitions
//   replica_groups={{0}, {1, 2}}
//   group_mode=CollectiveOpGroupMode::kCrossReplica
//
//   This functions returns {{33}, {34}, {44, 45}, {55, 56}}.
//   Partition 0 has 2 subgroups of participating devices {33}, {44, 55} and
//   partition 1 has 2 subgroups of participating devices {34}, {45, 56}.
//
// Another example:
//   device_assignment={{33, 34}, {44, 45}, {55, 56}}  3 replicas 2 partitions
//   replica_groups={{0}, {1, 2}, {3, 4, 5}}
//   group_mode=CollectiveOpGroupMode::kFlattenedID
//
//   This functions returns {{33}, {34, 44}, {45, 55, 56}}. The replica_ids map
//   into a flattened version of device_assignment.
pub fn get_participating_devices_groups(
  device_assignment: &DeviceAssignment,
  replica_groups: &Vec<ReplicaGroup>,
  group_mode: CollectiveOpGroupMode) -> Result<Vec<Vec<i64>>, String>
{
  let replica_count = device_assignment.replica_count();
  let partition_count = device_assignment.computation_count();

  // If replica groups are empty, assume a group with all replicas.
  if replica_groups.is_empty() {
  }

  let mut groups = vec![];
    match group_mode {
      CollectiveOpGroupMode::CrossReplica => {
        for rg in replica_groups {
          // replica_group contains replica id, participants contains all
          // replica_group's replica_ids for the current partition.
          for partition_id in 0..partition_count {
            let mut participants = vec![];
            for replica_id in rg.replica_ids() {
              participants.push(device_assignment.device_id(
                *replica_id, partition_id));
            }
            groups.push(participants);
          }
        }
        return Ok(groups);
      }
      CollectiveOpGroupMode::CrossPartition => {
        for rg in replica_groups {
          // replica_group contains partition id, participants contains all
          // replica_group's partition_ids for the current replica_id.
          for replica_id in 0..replica_count {
            let mut participants = vec![];
            for partition_id in rg.replica_ids() {
              participants.push(device_assignment.device_id(
                replica_id, *partition_id));
            }
            groups.push(participants);
          }
        }
        return Ok(groups);
      }
      CollectiveOpGroupMode::CrossReplicaAndPartition => {
        for rg in replica_groups {
          let mut participants = vec![];
          // replica_group contains replica id, participants contains all
          // replica_group's replica_ids for all partitions.
          for replica_id in rg.replica_ids() {
            for partition_id in 0..partition_count {
              participants.push(device_assignment.device_id(
                *replica_id, partition_id));
            }
          }
          groups.push(participants);
        }
        return Ok(groups);
      }
      CollectiveOpGroupMode::FlattenedID => {
        for rg in replica_groups {
          let mut participants = vec![];
          for flattened_id in rg.replica_ids() {
            // Map from flattened id back to replica_id, partition_id.
            let replica_id = flattened_id / partition_count;
            let partition_id = flattened_id % partition_count;
            participants.push(device_assignment.device_id(
              replica_id, partition_id));
          }
          groups.push(participants);
        }
        return Ok(groups);
      }
    }
}

// Same as above, except taking an HloInstruction instead.
pub fn get_participating_devices_groups_by_instruction(
  collective: &HloInstruction) -> Result<Vec<Vec<i64>>, String>
{
  debug_assert!(collective.get_module().as_ref().unwrap()
    .config().has_static_device_assignment());
  let device_assignment = collective.get_module()
    .as_ref().unwrap().config().static_device_assignment();
  let mode =
    get_collective_op_group_mode_by_instruction(collective);
  if mode.is_err() {
    return Err(mode.err().unwrap());
  }
  get_participating_devices_groups(device_assignment,
    get_collective_replica_groups(collective), mode.unwrap())
}

// Same as above, except that it returns the flattened id in the replica groups
// instead of device id.
pub fn get_participating_flattened_id_groups(
  device_assignment: &DeviceAssignment,
  collective_device_list: &CollectiveDeviceList,
  group_mode: CollectiveOpGroupMode) -> Result<CollectiveDeviceList, String>
{
  get_participating_flattened_id_groups_by_r_and_p(
    collective_device_list,
    group_mode,
    device_assignment.replica_count(),
    device_assignment.computation_count())
}

// Same as above, but take replica/partition count instead of device assignment.
pub fn get_participating_flattened_id_groups_by_r_and_p(
  collective_device_list: &CollectiveDeviceList,
  group_mode: CollectiveOpGroupMode,
  replica_count: i64,
  partition_count: i64) -> Result<CollectiveDeviceList, String>
{
  if group_mode == CollectiveOpGroupMode::FlattenedID {
    return Ok(collective_device_list.clone());
  }
  let filled_empty_replica_group: Vec<ReplicaGroup> = vec![];
  let mut original_replica_groups = vec![];
  original_replica_groups.clone_from(
    collective_device_list.replica_groups().as_ref().unwrap());
  let mut flattened_replica_groups: Vec<ReplicaGroup> = vec![];

  if collective_device_list.replica_groups().as_ref().unwrap().is_empty() {
    let mut id_count = replica_count;
    if group_mode == CollectiveOpGroupMode::CrossPartition {
      id_count = partition_count;
    }
    for _i in  0..id_count{
      //filled_empty_replica_group.last().as_mut().unwrap()
        //.add_replica_ids(i);
    }
    original_replica_groups.clone_from(&filled_empty_replica_group);
  }

  match group_mode {
    CollectiveOpGroupMode::CrossReplica => {
      let mut curr_g_offset = 0;
      for i in 0..original_replica_groups.len() {
        for replica_id in original_replica_groups[i].replica_ids() {
          for partition_id in 0..partition_count {
            let flattend_id = replica_id * partition_count + partition_id;
            flattened_replica_groups[(curr_g_offset + partition_id) as usize]
              .add_replica_ids(flattend_id);
          }
        }
        curr_g_offset += partition_count;
      }
    }
    CollectiveOpGroupMode::CrossPartition => {
      let mut curr_g_offset = 0;
      for i in 0..original_replica_groups.len() {
        for partition_id in original_replica_groups[i].replica_ids() {
          for replica_id in 0..replica_count {
            let flattened_id = replica_id + partition_count + partition_id;
            flattened_replica_groups[(curr_g_offset + replica_id) as usize]
              .add_replica_ids(flattened_id);
          }
        }
        curr_g_offset += replica_count;   
      }
    }
    CollectiveOpGroupMode::CrossReplicaAndPartition => {
      for i in 0..original_replica_groups.len() {
        for replica_id in original_replica_groups[i].replica_ids() {
          for partition_id in 0..partition_count {
            let flattened_id = replica_id * partition_count + partition_id;
            flattened_replica_groups[i].add_replica_ids(flattened_id);
          }
        }
      }
    }
    _ => {
      unimplemented!()
    }
  }
  Ok(CollectiveDeviceList::new(flattened_replica_groups))
}

pub fn get_participating_devices(
  device_id: i64,
  device_assignment: &DeviceAssignment,
  replica_groups: &Vec<ReplicaGroup>,
  group_mode: CollectiveOpGroupMode) -> Result<Vec<i64>, String>
{
  let replica_count = device_assignment.replica_count();
  let partition_count = device_assignment.computation_count();
  let logical_id_wrapper =
    device_assignment.logical_id_for_device(device_id);
  if logical_id_wrapper.is_err() {
    return Err(logical_id_wrapper.err().unwrap());
  }

  let logical_id = logical_id_wrapper.unwrap();
  let current_replica_id = logical_id.replica_id;
  let current_partition_id = logical_id.computation_id;
  debug_assert!(current_replica_id >= 0 && current_replica_id < replica_count);
  debug_assert!(current_partition_id >= 0 && current_partition_id < partition_count);

  let mut participants = vec![];
  match group_mode {
    CollectiveOpGroupMode::CrossReplica => {
      // This is a cross replica operation. replica group contains replica id.
      // use current replica id to find the set of participating replicas. If
      // replica groups are empty, assume a group with all replicas.
      let participating_replicas_wrapper =
        get_participating_ids(group_mode, current_partition_id,
        Some(replica_count), replica_groups);
      if participating_replicas_wrapper.is_err() {
        return Err(participating_replicas_wrapper.err().unwrap());
      }
      let participating_replicas = participating_replicas_wrapper.unwrap();
      for replica_id in &participating_replicas {
        debug_assert!(*replica_id >= 0 && *replica_id < replica_count);
        participants.push(device_assignment.device_id(
          *replica_id, current_partition_id));
      }
      return Ok(participants);
    }
    CollectiveOpGroupMode::CrossPartition => {
      // replica_groups contain partition_id, group contains all partitions for
      // the current replica.
      let participating_partitions_wrapper =
        get_participating_ids(group_mode, current_partition_id,
        Some(partition_count), replica_groups);
      if participating_partitions_wrapper.is_err() {
        return Err(participating_partitions_wrapper.err().unwrap());
      }
      let participating_partitions = participating_partitions_wrapper.unwrap();
      for partition_id in &participating_partitions {
        debug_assert!(*partition_id >= 0 && *partition_id < partition_count);
        participants.push(device_assignment.device_id(
          current_replica_id, *partition_id));
      }
      return Ok(participants);
    }
    CollectiveOpGroupMode::CrossReplicaAndPartition => {
      // replica_groups contain replica_ids. Group contains replicas for all
      // partitions.
      let participating_replicas_wrapper =
        get_participating_ids(group_mode, current_replica_id,
        Some(replica_count), replica_groups);
      if participating_replicas_wrapper.is_err() {
        return Err(participating_replicas_wrapper.err().unwrap());
      }
      let participating_replicas = participating_replicas_wrapper.unwrap();
      for replica_id in &participating_replicas {
        debug_assert!(*replica_id >= 0 && *replica_id < replica_count);
        for partition_id in 0..partition_count {
          participants.push(device_assignment.device_id(
            *replica_id, partition_id));
        }
      }
      return Ok(participants);
    }
    CollectiveOpGroupMode::FlattenedID => {
      // replica groups contain flattened-ids and cannot be empty.
      if replica_groups.is_empty() {
        let err_msg =
          "replica groups cannot be empty for FlattendID mode".to_string();
        return Err(err_msg);
      }
      let current_flattened_id =
        current_replica_id * partition_count + current_partition_id;
      // Find participants based on flattened id. replica_groups cannot be empty
      // so no need to pass in total_participant_count.
      let participating_flattened_ids_wrapper =
        get_participating_ids(group_mode, current_flattened_id,
        None, replica_groups);
      if participating_flattened_ids_wrapper.is_err() {
        return Err(participating_flattened_ids_wrapper.err().unwrap());
      }
      let participating_flattened_ids =
        participating_flattened_ids_wrapper.unwrap();
      for flattened_id in &participating_flattened_ids {
        // Map from flattened id back to replica_id, partition_id.
        let replica_id = flattened_id / partition_count;
        debug_assert!(replica_id >= 0 && replica_id < replica_count);
        let partition_id = flattened_id % partition_count;
        participants.push(device_assignment.device_id(
          replica_id, partition_id));
      }
      return Ok(participants);
    }
  }  
}

pub fn get_participant_counts_for_replica_groups(
  num_replicas: i64,
  num_partitions: i64,
  replica_groups: &mut Vec<ReplicaGroup>,
  group_mode: CollectiveOpGroupMode) -> Result<Vec<i64>, String>
{
  let mut participant_counts = vec![];

  // If replica groups are empty, assume a group with all replicas.
  let mut all_replica_groups = ReplicaGroup::new();
  if replica_groups.is_empty() {
    if group_mode == CollectiveOpGroupMode::FlattenedID {
      // replica groups contain flattened-ids and cannot be empty.
    }
    #[allow(unused_assignments)]
    let mut total_partition_count = 0;
    if group_mode == CollectiveOpGroupMode::CrossPartition {
      // replica group are partition ids.
      total_partition_count = num_partitions;
    } else {
      // replica group are replica ids.
      total_partition_count = num_replicas;
    }
    for id in 0..total_partition_count {
      all_replica_groups.add_replica_ids(id);
    }
    replica_groups.clone_from(&vec![all_replica_groups]);
  }
  match group_mode {
    CollectiveOpGroupMode::CrossReplica => {
      for rg in replica_groups {
        for _partition_id in 0..num_partitions {
          participant_counts.push(rg.replica_ids().len() as i64);
        }
      }
      return Ok(participant_counts);
    }
    CollectiveOpGroupMode::CrossPartition => {
      for rg in replica_groups {
        participant_counts.push(rg.replica_ids().len() as i64);
      }
      return Ok(participant_counts);
    }
    CollectiveOpGroupMode::CrossReplicaAndPartition => {
      for rg in replica_groups {
        let participant = (rg.replica_ids().len() as i64) * num_partitions;
        participant_counts.push(participant);
      }
      return Ok(participant_counts);
    }
    CollectiveOpGroupMode::FlattenedID => {
      for rg in replica_groups {
        participant_counts.push(rg.replica_ids().len() as i64);
      }
      return Ok(participant_counts);
    }
  }
}

// Returns true if the two replica group are orthogonal.
pub fn replica_groups_orthogonal(
  first: &Vec<ReplicaGroup>, second: &Vec<ReplicaGroup>) -> bool
{
  if first.len() != second[0].replica_ids().len() {
    return false;
  }
  if first[0].replica_ids().len() != second.len() {
    return false;
  }
  for i in 0..first.len() {
    for j in 0..first[i].replica_ids().len() {
      if first[i].replica_ids()[j] != second[j].replica_ids()[i] {
        return false;
      }
    }
  }
  true
}

// Returns true if the two replica group are Equal.
pub fn replica_groups_equal(
  first: &Vec<ReplicaGroup>, second: &Vec<ReplicaGroup>) -> bool
{
  if first.len() != second.len() {
    return false;
  }
  for i in 0..first.len() {
    if first[i].replica_ids().len() != second[i].replica_ids().len() {
      return false;
    }
    for j in 0..first[i].replica_ids().len() {
      if first[i].replica_ids()[j] != second[i].replica_ids()[j] {
        return false;
      }
    }
  }
  true
}

// Returns true if all subgroups in replica_groups are exclusively cross-module.
pub fn is_exclusive_cross_module(
  replica_groups: &Vec<ReplicaGroup>,
  use_global_ids: bool,
  has_channel_id: bool,
  device_assignment: &DeviceAssignment) -> bool
{
  if !has_channel_id {
    return false;
  }
  if !use_global_ids {
    // Each id in a replica group is a replica id. If any group
    // has more than one id then this is not exclusively cross module.
    for rg in replica_groups {
      if rg.replica_ids().len() != 1 { return false; }
      return true;
    }
  }
  // Each id in a replica group is a global id. Check if all replica groups are
  // exclusively cross module (all participants in a group have the same replica
  // id).
  let partition_count = device_assignment.computation_count();
  for rg in replica_groups {
    let mut first_replica_id = None;
    for global_id in rg.replica_ids() {
      let replica_id = global_id / partition_count;
      if first_replica_id.is_none() {
        first_replica_id = Some(replica_id);
      } else if Some(replica_id) != first_replica_id {
        return false;
      }
    }
  }
  true
}

pub fn is_exclusive_cross_replica(
  replica_groups: &Vec<ReplicaGroup>,
  use_global_ids: bool,
  has_channel_id: bool,
  device_assignment: &DeviceAssignment) -> bool
{
  if !has_channel_id {
    return true;
  }
  let partition_count = device_assignment.computation_count();
  if !use_global_ids {
    // Each id in a replica group is a replica id and we will perform the
    // collective between all devices with that replica id. If partition count
    // is > 1, then this is not exclusively cross replica.
    return partition_count == 1;
  }
  // Each id in a replica group is a global id. Check if all replica groups are
  // exclusively cross replica (all participants in a group have the same
  // partition id).
  for rg in replica_groups {
    let mut first_partition_id = None;
    for global_id in rg.replica_ids() {
      let partition_id = global_id % partition_count;
      if first_partition_id.is_none() {
        first_partition_id = Some(partition_id);
      } else if partition_id != first_partition_id.unwrap() {
        return false;
      }
    }
  }
  true
}

pub fn has_duplicate_sources_or_targets() {
    
}

// Returns true if instruction is a collective op that is not a collective
// fusion.
pub fn is_non_fusion_collective(instruction: &HloInstruction) -> bool {
  match instruction.opcode() {
    HloOpcode::AllReduce => true,
    HloOpcode::AllReduceStart => true,
    HloOpcode::AllReduceDone => true,
    HloOpcode::AllGather => true,
    HloOpcode::AllGatherStart => true,
    HloOpcode::AllGatherDone => true,
    HloOpcode::AllToAll => true,
    HloOpcode::CollectiveBroadcast => true,
    HloOpcode::CollectivePermute => true,
    HloOpcode::CollectivePermuteStart => true,
    HloOpcode::CollectivePermuteDone => true,
    HloOpcode::RaggedAllToAll => true,
    HloOpcode::ReduceScatter => true,
    HloOpcode::AsyncStart => is_non_fusion_collective(instruction.async_wrapped_instruction()),
    HloOpcode::AsyncUpdate => is_non_fusion_collective(instruction.async_wrapped_instruction()),
    HloOpcode::AsyncDone => is_non_fusion_collective(instruction.async_wrapped_instruction()),
    HloOpcode::Send => !instruction.is_host_transfer(),
    HloOpcode::Recv => !instruction.is_host_transfer(),
    _ => false
  }
}

// Returns true if instruction is a collective op or a collective fusion.
pub fn is_collective(instruction: &HloInstruction) -> bool {
  if is_non_fusion_collective(instruction) {
    return true;
  }
  if instruction.opcode() == HloOpcode::Fusion &&
    instruction.is_custom_fusion()
  {
    for inner_inst in instruction.fused_instructions() {
      if is_collective(inner_inst) { return true; }
    }
  }
  false
}

// Returns true if instruction is an async collective op.
pub fn is_async_collective(
  instruction: &HloInstruction) -> Result<bool, String>
{
  if !is_non_fusion_collective(instruction) {
    return Ok(false);
  }
  if instruction.is_asynchronous() {
    match instruction.async_wrapped_opcode() {
      HloOpcode::AllGather => return Ok(true),
      HloOpcode::AllReduce => return Ok(true),
      HloOpcode::AllToAll => return Ok(true),
      HloOpcode::CollectiveBroadcast => return Ok(true),
      HloOpcode::CollectivePermute => return Ok(true),
      HloOpcode::RaggedAllToAll => return Ok(true),
      HloOpcode::ReduceScatter => return Ok(true),
      _ => {
        let mut err_msg = "Async instruction ".to_string();
        err_msg.push_str(&instruction.to_string_default());
        err_msg.push_str(" is not a collective.");
        return Err(err_msg);
      }
    }
  }
  match instruction.opcode() {
    HloOpcode::AllGatherStart => return Ok(true),
    HloOpcode::AllGatherDone => return Ok(true),
    HloOpcode::AllReduceStart => return Ok(true),
    HloOpcode::AllReduceDone => return Ok(true),
    HloOpcode::CollectivePermuteStart => return Ok(true),
    HloOpcode::CollectivePermuteDone => return Ok(true),
    HloOpcode::Send => Ok(!instruction.is_host_transfer()),
    HloOpcode::Recv => Ok(!instruction.is_host_transfer()),
    HloOpcode::AllGather => return Ok(false),
    HloOpcode::AllReduce => return Ok(false),
    HloOpcode::AllToAll => return Ok(false),
    HloOpcode::CollectiveBroadcast => return Ok(false),
    HloOpcode::CollectivePermute => return Ok(false),
    HloOpcode::RaggedAllToAll => return Ok(false),
    HloOpcode::ReduceScatter => return Ok(false),
    _ => {
      let mut err_msg = "Async instruction ".to_string();
      err_msg.push_str(&instruction.to_string_default());
      err_msg.push_str(" is not an async collective.");
      return Err(err_msg);
    }
  }
}

pub fn is_or_has_collective_with_channel_id(
  instruction: &HloInstruction) -> Option<&HloInstruction>
{
  if instruction.opcode() == HloOpcode::Fusion {
    for inner_inst in instruction.fused_instructions() {
      if is_or_has_collective_with_channel_id(inner_inst).is_some() {
        return Some(inner_inst);
      }
    }
  }
  if !instruction.is_channel_instruction() {
    return None;
  }
  if is_collective(instruction) && instruction.channel_id().is_some() {
    return Some(instruction);
  }
  None
}