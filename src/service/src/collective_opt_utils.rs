#![allow(dead_code)]

use std::collections::BTreeMap;
use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};

pub struct ReduceScatterSpec {
  pub split_dim: i64,
  pub sharded_partitions: i64,
  pub sharded_replicas: i64,
  pub group_size: i64,
  pub original_split_dims: Vec<i64>,
  pub dynamic_slice: Option<HloInstruction>,
}

impl ReduceScatterSpec {
  pub fn default() -> Self {
    ReduceScatterSpec {
      split_dim: 0,
      sharded_partitions: 0,
      sharded_replicas: 0,
      group_size: 0,
      original_split_dims: Vec::new(),
      dynamic_slice: None
    }
  }
}

pub struct SplitDimSpec {
  pub split_dims: Vec<i64>,
  pub split_dim: i64,
  pub split_dim_size: i64,
}

impl SplitDimSpec {
  pub fn default() -> Self {
    SplitDimSpec { split_dims: Vec::new(), split_dim: 0, split_dim_size: 0 }
  }
}

// Represents the mapping of partition offsets to partition IDs for each replica
// group. This can be derived from either a dynamic-slice or an all-gather
// operation.
pub struct PartitionOffsetSpec {
  // A list of OffsetToIdMap, one for each replica group.
  per_replica_group_offsets: Vec<BTreeMap<i64, i64>>
}

impl PartitionOffsetSpec {
  pub fn default() -> Self {
    PartitionOffsetSpec { per_replica_group_offsets: Vec::new() }
  }
}

pub struct AllGatherDynamicSliceMatchSpec {
  permutation_pairs: Vec<(i64, i64)>
}

fn spec_from_reduce_scatter_instr(
  rs_instr: &HloInstruction,
  num_partitions: i64,
  num_replicas: i64,
  min_rank: i64,
  _is_constrain_layout: bool,
  use_global_device_ids: bool,
  is_cross_module: bool) -> Option<ReduceScatterSpec>
{
  if rs_instr.shape().dimensions_size() < min_rank as usize {
    return None;
  }
  debug_assert!(rs_instr.opcode() == HloOpcode::ReduceScatter);
  let mut spec = ReduceScatterSpec::default();
  spec.split_dim = rs_instr.dimensions()[0];
  if !is_cross_module {
    spec.sharded_replicas = num_replicas;
    if rs_instr.replica_groups().is_empty() {
      spec.group_size = num_replicas;
    } else {
      spec.group_size = rs_instr.replica_groups()[0].replica_ids().len() as i64;
    }
  } else if use_global_device_ids {
    spec.sharded_replicas = num_replicas;
    spec.sharded_partitions = num_partitions;
    spec.group_size = rs_instr.replica_groups()[0].replica_ids().len() as i64;
  } else {
    spec.sharded_partitions = num_partitions;
    spec.dynamic_slice = None;
  }
  Some(spec)
}

// Matches the given all-reduce operation to a reduce-scatter pattern.
pub fn match_reduce_scatter(
  ar: &HloInstruction,
  num_partitions: i64,
  num_replicas: i64,
  allow_multiple_split_dims: bool,
  allow_intervening_reshape: bool,
  min_rank: i64,
  match_partition_id: &dyn Fn(&HloInstruction)->bool,
  match_replica_id: &dyn Fn(&HloInstruction)->bool,
  allow_intervening_bitcast: bool) -> Option<ReduceScatterSpec>
{
  if ar.opcode() == HloOpcode::ReduceScatter {
    return spec_from_reduce_scatter_instr(
      ar, num_partitions, num_replicas, min_rank,
      ar.constrain_layout(), ar.use_global_device_ids(),
      ar.channel_id().is_some());
  }
  let is_cross_module = ar.channel_id().is_some() &&
    ar.opcode() == HloOpcode::AllReduce;
  
  match_with_dynamic_slice(
    ar, num_partitions, num_replicas, allow_multiple_split_dims,
    allow_intervening_reshape, min_rank, match_partition_id,
    match_replica_id, ar.constrain_layout(),
    ar.use_global_device_ids(), is_cross_module, allow_intervening_bitcast,
  false)
}

// Checks whether AG(ICI) and its user DS(ICI) can be canceled out.
pub fn all_gether_dynamic_slice_cancellation(
  ag: &HloInstruction,
  num_partitions: i64,
  num_replicas: i64,
  allow_multiple_split_dims: bool,
  allow_intervening_reshape: bool,
  min_rank: i64,
  match_partition_id: &dyn Fn(&HloInstruction)->bool,
  match_replica_id: &dyn Fn(&HloInstruction)->bool,
  allow_intervention_bitcast: bool,
  allow_multiple_users: bool) -> Option<ReduceScatterSpec>
{
  let is_cross_module = ag.channel_id().is_some() &&
    ag.opcode() == HloOpcode::AllGather;

  let spec = match_with_dynamic_slice(
    ag, num_partitions, num_replicas, allow_multiple_split_dims,
    allow_intervening_reshape, min_rank, match_partition_id,
    match_replica_id, ag.constrain_layout(),
    ag.use_global_device_ids(), is_cross_module,
    allow_intervention_bitcast, allow_multiple_users);

  if spec.is_none() {
    return None;
  }
  if spec.as_ref().unwrap().dynamic_slice.is_some() &&
    spec.as_ref().unwrap().split_dim != ag.all_gather_dimension()
  {
    println!("Mismatch ag and ds");
    println!("ag:{:?}, ds:{:?}", ag.to_short_string(),
      spec.as_ref().unwrap().dynamic_slice.as_ref().unwrap().to_short_string());
    println!("ag_dim:{:?}", ag.all_gather_dimension().to_string());
    println!("ds_dim:{:?}", spec.as_ref().unwrap().split_dim);
    return None;
  }

  spec
}

// Matches an all-gather with a dynamic slice whose offset matches a permuted
// partition offset.
//
// This pattern is commonly used to implement permutations or data permuted
// across partitions. An all-gather collects data from all partitions, and then
// a dynamic-slice on each partition selects a slice from a remote partition,
// effectively permuting the data. This function identifies such patterns and
// extracts the permutation pairs (source partition, destination partition).
//
// The function matches a specific pattern:
//   - All-gather with flattened-id mode.
//   - Partitioning with `num_partitions > 1` and `num_replicas = 1`.
//   - AG Sharding and dynamic-sloce slicing on same dimension.
//
// For example, the following HLO performs a reverse permutation across 8
// partitions (partition `i` gets data from partition `7-i`):
//
// HloModule module
// ENTRY entry {
//   p = f32[32,8,128] parameter(0)
//   ag = f32[256,8,128] all-gather(p), replica_groups={{0,1,2,3,4,5,6,7}},
//     dimensions={0}, channel_id=1, use_global_device_ids=true
//   pid = u32[] partition-id()
//   permuted_index_list = s32[8]{0} constant({224,192,160,128,96,64,32,0})
//   offset = s32[1] dynamic-slice(permuted_index_list, pid),
//   dynamic_slice_sizes={1} offset_reshape = s32[] reshape(offset) zero = s32[]
//   constant(0) ROOT ds = f32[32,8,128] dynamic-slice(ag, offset_reshape, zero,
//   zero),
//     dynamic_slice_sizes={32,8,128}
// }
//
// This function would match this pattern and return permutation pairs like
// {{0,7}, {1,6}, ..., {7,0}}.
pub fn match_permuted_slice_and_partition_offset(
  _ag: &HloInstruction,
  _num_partitions: i64,
  _num_replicas: i64,
  _match_partition_id: &dyn Fn(&HloInstruction)->bool,
  _allow_multiple_users: bool) -> Option<AllGatherDynamicSliceMatchSpec>
{
  unimplemented!()
}

// Checks whether the replica groups in the given channel instruction are
// of the same size.
pub fn check_uniform_replica_groups(instruction: &HloInstruction) -> bool {
  if instruction.replica_groups().len() <= 1 {
    return true;
  } 
  let size = instruction.replica_groups()[0].replica_ids().len();
  for rg in instruction.replica_groups() {
    if rg.replica_ids().len() != size { return false; }
  }
  true
}

pub struct CollectiveUsers {
  pub dynamic_slice: Option<HloInstruction>,
  pub bitcast: Option<HloInstruction>,
  pub reshape: Option<HloInstruction>,
}

impl CollectiveUsers {
  pub fn default() -> Self {
    CollectiveUsers { dynamic_slice: None, bitcast: None, reshape: None }
  }
}

// Extracts the dynamic-slice user from a collective instruction, potentially
// looking through reshapes and bitcasts.
pub fn find_unique_dynamic_slice_usesr_from_collective(
  instruction: &HloInstruction,
  allow_multiple_users: bool,
  allow_intervening_reshape: bool,
  allow_intervening_bitcast: bool) -> Option<CollectiveUsers>
{
  if instruction.user_count() == 0 {
    return None;
  }
  let mut user = &instruction.users()[0];
  if allow_multiple_users {
    for some_user in instruction.users() {
      if (allow_intervening_reshape &&
        some_user.opcode() == HloOpcode::Reshape) ||
        some_user.opcode() == HloOpcode::DynamicSlice
      {
        user = some_user;
        break;
      }
    }
  }
  let mut result = CollectiveUsers::default();
  if allow_intervening_reshape {
    if user.opcode() == HloOpcode::Reshape {
      if user.user_count() != 1 {
        println!("Reshape user count > 1 for {:?}", user.to_short_string());
        return None;
      }
    }
    result.reshape = Some(user.clone());
    user = &user.users()[0];
  }
  if allow_intervening_bitcast {
    if user.opcode() == HloOpcode::Bitcast {
      if user.user_count() != 1 {
        println!("Bitcast user count > 1 for {:?}", user.to_short_string());
        return None;
      }
      result.bitcast = Some(user.clone());
      user = &user.users()[0];
    }
  }
  if user.opcode() == HloOpcode::DynamicSlice {
    result.dynamic_slice = Some(user.clone());
    return Some(result);
  }
  None
}

// Check if a given instruction (AllReduce or AllGather) matches a DynamicSlice;
// the DynamicSlice has to be the user of the given instruction.
pub fn match_with_dynamic_slice(
  _ag: &HloInstruction,
  _num_partitions: i64,
  _num_replicas: i64,
  _allow_multiple_split_dims: bool,
  _allow_intervening_reshape: bool,
  _min_rank: i64,
  _match_partition_id: &dyn Fn(&HloInstruction)->bool,
  _match_replica_id: &dyn Fn(&HloInstruction)->bool,
  _is_constrain_layout: bool,
  _use_global_device_ids: bool,
  _is_cross_module: bool,
  _allow_intervening_bitcast: bool,
  _allow_multiple_users: bool) -> Option<ReduceScatterSpec>
{
  unimplemented!()
}

// Extracts the split dimension spec from a `DynamicSlice` instruction. This
// spec identifies the dimension(s) being operated on by a collective
// operation that is fused with the slice.
//
// The function first attempts a fast path by finding a single dimension where
// the input and output shapes of the `DynamicSlice` differ.
//
// If more than one dimension differs, it re-computes the split dimension by
// examining the slice's offsets. It identifies non-trivial dimensions being
// sliced. A dimension is considered trivial and skipped if its size is 1, or
// if the slice offset along it is a constant zero. This prevents
// misidentifying a dimension that isn't actually being scattered as the split
// dimension.
pub fn extract_split_dim_spec(
  dynamic_slice: &HloInstruction,
  allow_multiple_split_dims: bool) -> Option<SplitDimSpec>
{
  let mut spec = SplitDimSpec::default();

  // First find a single dimension where the input and output of dynamic slice
  // differ.
  let mut num_dims = 0;
  for dim in 0..dynamic_slice.operand(0).shape().dimensions_size() {
    if dynamic_slice.operand(0).shape().dimensions(dim) ==
      dynamic_slice.shape().dimensions(dim)
    {
      continue;
    }
    num_dims += 1;
    spec.split_dim = dim as i64;
    spec.split_dim_size = dynamic_slice.dynamic_slice_sizes()[dim];
  }
  if spec.split_dim != -1 && num_dims == 1 {
    // No recomputation needed if dynamic-slice has unique dimension to slice.
    spec.split_dims.push(spec.split_dim);
    return Some(spec);
  }
  // Recompute split dim if dynamic-slice has multiple dimensions to slice.
  spec.split_dim = -1;
  let shape = dynamic_slice.operand(0).shape();
  for dim in 0..shape.dimensions_size() {
    let offset = dynamic_slice.operand(dim+1);
    // Skip trivial (1) dimensions or if the index is a constant 0.
    if shape.dimensions(dim) == 1 ||
      (offset.opcode() == HloOpcode::Constant &&
      offset.literal::<i64>().is_zero_i64(&vec![]))
    {
      continue;
    }
    spec.split_dims.push(dim as i64);
    if spec.split_dim != -1 {
      if !allow_multiple_split_dims || spec.split_dim != (dim - 1) as i64 {
        println!("Only support split on consecutive dims {:?}",
          dynamic_slice.to_short_string());
        return None;
      }
    }
  }
  Some(spec)
}

// Extracts the PartitionOffsetSpec from an all-gather instruction.
pub fn extract_partition_offset_spec(
  ag: HloInstruction, num_partitions: i64) -> Option<PartitionOffsetSpec>
{
  println!("Extracting partition offset spec for: {:?}
     with num_partitions={:?}", ag.to_short_string(), num_partitions);

  let mut spec = PartitionOffsetSpec::default();
  let all_gather_shard_size = ag.operand(0).shape()
    .dimensions(ag.all_gather_dimension() as usize);
  println!("ag: {:?}, num_partitions: {:?}, all_gather_shard_size: {:?}",
    ag.to_short_string(), num_partitions, all_gather_shard_size);
  if all_gather_shard_size <= 0 {
    println!("ag does not have valid all gather shard size");
    return None;
  }
  if ag.replica_groups().is_empty() {
    println!("Ag has no replica groups, assuming iota.");
    let offset_map = &mut spec.per_replica_group_offsets[0];
    for i in 0..num_partitions {
      let offset = i * all_gather_shard_size;
      let partition_id = i;
      if offset_map.insert(offset, partition_id).is_none() {
        println!("Duplicate offset {:?} in replica group 0 for partition {:?}",
          offset, partition_id);
        return None;
      }
    }
    println!("Successfully extracted partition offset spec for {:?}",
      ag.to_short_string());
    return Some(spec);
  }

  for g_idx in 0..ag.replica_groups().len() {
    println!("Processing replica group: {:?}", g_idx);
    let group = &ag.replica_groups()[g_idx];
    for replica_idx in 0..group.replica_ids().len() {
      let offset = replica_idx * (all_gather_shard_size as usize);
      let partition_id = group.replica_ids()[replica_idx];

      if spec.per_replica_group_offsets
        .get(g_idx).as_ref().unwrap().contains_key(&(offset as i64))
      {
        println!("Duplicate offset {:?} in replica groups {:?}",
          offset, g_idx);
        return None;
      }
      let table =
        spec.per_replica_group_offsets.get_mut(g_idx).unwrap();
      table.insert(offset as i64, partition_id);
    }
  }
  println!("Successfully extracted partition offset spec.");
  Some(spec)
}

// Extracts pattern dynamic-slice(pad(all-gather)).
// Returns true if the pattern is found, and set pad_hlo and ag_hlo.
// Otherwise, returns false.
pub fn match_ds_pad_all_gather(
  _ds_hlo: &HloInstruction,
  _pad_hlo: &HloInstruction,
  _ag_hlo: &HloInstruction) -> bool
{
  unimplemented!()
}