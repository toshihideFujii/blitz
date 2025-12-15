#![allow(dead_code)]

use std::collections::HashSet;
use common::blitz_data::ReplicaGroup;
use hlo::{collective_ops_utils::{get_collective_replica_groups, is_non_fusion_collective}, hlo_computation::HloComputation,
  hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};

pub struct AbstractReplicaGroups {
  groups: Vec<HashSet<i64>>,
  index_map: Vec<i64>
}

impl AbstractReplicaGroups {
  pub fn default() -> Self {
    AbstractReplicaGroups { groups: Vec::new(), index_map: Vec::new() }
  }

  pub fn get_index(&mut self, replica_id: i64) -> i64 {
    let mut index_map_size = self.index_map.len();
    loop {    
      if index_map_size <= replica_id as usize {
        self.index_map.push(-1);
        index_map_size += 1;
      } else {
        break;
      }
    }
    *self.index_map.get(replica_id as usize).unwrap()
  }

  pub fn set_index(&mut self, replica_id: i64, index: i64) {
    let mut index_map_size = self.index_map.len();
    loop {    
      if index_map_size <= replica_id as usize {
        self.index_map.push(-1);
        index_map_size += 1;
      } else {
        break;
      }
    }
    self.index_map.insert(replica_id as usize, index);
  }

  pub fn merge_groups(&mut self, replica_id: i64, other_replica_id: i64) {
    if self.get_index(replica_id) == -1 && self.get_index(other_replica_id) == -1 {
      self.set_index(replica_id, self.groups.len() as i64);
      self.set_index(other_replica_id, self.groups.len() as i64);

      let mut set = HashSet::new();
      set.insert(replica_id);
      set.insert(other_replica_id);
      self.groups.push(set);
      return;
    }
    if self.get_index(replica_id) == self.get_index(other_replica_id) {
      return;
    }
    let mut replica_id_2 = replica_id;
    let mut other_replica_id_2 = other_replica_id;
    if self.get_index(replica_id_2) == -1 {
      let temp_replica_id = replica_id_2;
      replica_id_2 = other_replica_id_2;
      other_replica_id_2 = temp_replica_id;
    }
    debug_assert!(replica_id_2 != -1);

    if self.get_index(other_replica_id_2) == -1 {
      let value = self.get_index(replica_id_2);
      self.set_index(other_replica_id_2, value);
      let target_id = self.get_index(replica_id_2) as usize;
      self.groups[target_id].insert(other_replica_id_2);
      return;
    }
    debug_assert!(self.get_index(replica_id_2) != -1 &&
      self.get_index(other_replica_id_2) != -1 &&
      self.get_index(replica_id_2) != self.get_index(other_replica_id_2));
    
    let other_set = &self.groups[other_replica_id_2 as usize].clone();
    for replica_id_in_other_set in other_set {
      let target_id = self.get_index(replica_id_2);
      self.groups[target_id as usize].insert(*replica_id_in_other_set);
      let value = self.get_index(replica_id_2);
      self.set_index(*replica_id_in_other_set, value);
    }
    self.groups[other_replica_id_2 as usize].clear();
  }
}

pub fn is_conflicting_abstract_replica_groups(
  lhs: &mut AbstractReplicaGroups, rhs: &AbstractReplicaGroups) -> bool
{
  let mut frequency = vec![0; lhs.groups.len()];
  for rhs_g in &rhs.groups {
    for rhs_replica_id in rhs_g {
      let i = lhs.get_index(*rhs_replica_id);
      if i == -1 { continue };
      frequency[i as usize] += 1;
      if frequency[i as usize] >= 2 { return true; }
    }
  }
  false
}

pub fn get_abstract_replica_groups(
  instr: &HloInstruction, groups: &mut AbstractReplicaGroups)
{
  // Abstract from source-target pairs of collective-permute to abstract
  // replica groups.
  if instr.opcode() == HloOpcode::CollectivePermute {
    // TODO: is_collective_permute_instruction
    for (i, j) in instr.source_target_pairs() {
      groups.merge_groups(*i, *j);
    }
    return;
  }
  // Abstract from source-target pairs of send/recv to abstract replica groups.
  let mut add_replica_g = |replica_g: &ReplicaGroup| {
    let ids = replica_g.replica_ids();
    if ids.is_empty() { return; }
    let leader_id = ids[0];
    for i in 1..ids.len() {
      groups.merge_groups(leader_id, ids[i]);
    }
  };

  if instr.opcode() == HloOpcode::Send || instr.opcode() == HloOpcode::Recv {
    return;
  }

  // Convert normal replica groups to abstract replica groups.
  for replica_g in get_collective_replica_groups(instr) {
    add_replica_g(replica_g);   
  }
}

pub fn find_all_conflicting_collectives(
  computation: &HloComputation, 
  seed_collectives: &Vec<HloInstruction>) -> Vec<HloInstruction>
{
  let mut seen = HashSet::new();

  // Get the supremum of all abstract replica groups of the seed collectives
  // we're starting with.
  let mut abs_replica_g_sup = AbstractReplicaGroups::default();
  for instr in seed_collectives {
    get_abstract_replica_groups(instr, &mut abs_replica_g_sup);
    seen.insert(instr.clone());
  }

  // Try finding more and more conflicting collectives until we reach a
  // fixpoint. This is needed because we may get a coarser supremum with each
  // new conflicting collective.
  let mut conflicting_collectives = vec![];
  loop {
    let mut fixpoint_reached = true;
    // Look at each collective in the computation.
    for instr in computation.make_instruction_post_order() {
      // Skip if not a collective or already considered for the supremum.
      if !is_non_fusion_collective(instr) || seen.contains(instr) {
        continue;
      }
      // Check if this collective is already conflicting with the coarsest
      // abstract replica groups. If it does, add to the conflicting collectives
      // and update the supremum.
      let mut groups = AbstractReplicaGroups::default();
      get_abstract_replica_groups(instr, &mut groups);
      if is_conflicting_abstract_replica_groups(&mut groups, &abs_replica_g_sup) {
        conflicting_collectives.push(instr.clone());
        get_abstract_replica_groups(instr, &mut abs_replica_g_sup);
        seen.insert(instr.clone());
        fixpoint_reached = false;
      }
    }
    if fixpoint_reached { break; }
  }
  conflicting_collectives
}

pub fn find_all_conflicting_collectives_by_seed(
  seed_collective: &HloInstruction) -> Vec<HloInstruction>
{
  find_all_conflicting_collectives(seed_collective.parent().unwrap(),
    &vec![seed_collective.clone()])
}