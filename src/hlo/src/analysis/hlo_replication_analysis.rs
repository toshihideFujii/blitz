#![allow(dead_code)]

use std::{collections::{HashMap, HashSet}};

use common::{shape::Shape, shape_tree::ShapeTree, shape_util::ShapeUtil};

use crate::{hlo_computation::HloComputation, hlo_instruction::HloInstruction,
  hlo_module::HloModule, hlo_opcode::HloOpcode};

// An HLO pass that determines whether each instruction in the module outputs
// the same value across replicas or across partitions (depending on the value
// `cross_partition_spmd`). It propagates sources of replicated values to
// the rest of the module, where sources include cross-replica-sum, annotated
// entry parameters, and constants.
pub struct HloReplicationAnalysis<'module> {
  module: &'module mut HloModule,
  cross_partition_spmd: bool,
  loops_known_with_same_iterations: HashSet<HloInstruction>,
  support_partial_replication: bool,
  num_partitions: i64,
  replica_count: i64,
  hlo_replication: HashMap<HloInstruction, ShapeTree<HloReplication>>,
  replica_group_dedup_map: HashMap<HloInstruction, Option<HloReplication>>,
  replication_merge_map: HashMap<(HloReplication, HloReplication), HloReplication>,
  unique_replications: Vec<Option<HloReplication>>,
  device_sets_per_replica_map: HashMap<HashableReplicaGroupSpan, Vec<Vec<Vec<i64>>>>
}

impl<'module> HloReplicationAnalysis<'module> {
  pub fn new(
    module: &'module mut HloModule,
    cross_partition_spmd: bool,
    loops_known_with_same_iterations: HashSet<HloInstruction>,
    support_partial_replication: bool) -> Self
  {
    HloReplicationAnalysis {
      module: module,
      cross_partition_spmd: cross_partition_spmd, 
      loops_known_with_same_iterations: loops_known_with_same_iterations,
      support_partial_replication: support_partial_replication,
      num_partitions: 0,
      replica_count: 0,
      hlo_replication: HashMap::new(),
      replica_group_dedup_map: HashMap::new(),
      replication_merge_map: HashMap::new(),
      unique_replications: Vec::new(),
      device_sets_per_replica_map: HashMap::new(),
    }
  }

  // Runs the analysis on module and returns the result or an error.
  pub fn run(
    &self,
    module: &'module mut HloModule,
    cross_partition_spmd: bool) -> Result<Self, String>
  {
    let empty = HashSet::new();
    self.run_by_loops(
      module, cross_partition_spmd, empty)
  }

  // Same as above, but the caller can provide additional annotations: a set of
  // while loops that are known to have the same iteration counts across
  // replicas or partitions.
  pub fn run_by_loops(
    &self,
    module: &'module mut HloModule,
    cross_partition_spmd: bool,
    loops_known_with_same_iterations: HashSet<HloInstruction>) -> Result<Self, String>
  {
    let mut analysis = HloReplicationAnalysis::new(
      module, cross_partition_spmd,
      loops_known_with_same_iterations,
      false);
    
    analysis.build_replica_group_dedup_map();
    let result = analysis.compute_hlo_replication();
    check_error(&result);
    Ok(analysis)
  }

  // Same as above but supports finding partially replicated HLOs.
  pub fn run_with_partial_replication(
    &self,
    module: &'module mut HloModule,
    cross_partition_spmd: bool) -> Result<Self, String>
  {
    let empty = HashSet::new();
    let mut analysis = HloReplicationAnalysis::new(
      module,
      cross_partition_spmd,
      empty,
      true);
    
    analysis.build_replica_group_dedup_map();
    let result = analysis.compute_hlo_replication();
    check_error(&result);
    Ok(analysis)
  }

  // Returns if the HLO instruction outputs the same value (i.e., replicated) at
  // the given index across all replicas or partitions.
  pub fn hlo_instruction_is_replicated_at(
    &self, inst: &HloInstruction, index: &Vec<i64>) -> bool
  {
    let target = self.hlo_replication.get(inst);
    if target.is_none() { return false; }
    target.unwrap().element(index).is_replicated_on_all_devices()
  }

  // Computes hlo_replication_.
  pub fn compute_hlo_replication(&mut self) -> Result<(), String> {
    // Add entry parameters to the above sets according to user annotation.
    // Replicated modules read from `parameter_replicated_at_leaf_buffers` whereas
    // SPMD partitioned modules read from HloSharding attributes.
    let entry = self.module.mutable_entry_computation().unwrap();
    for i in 0..entry.num_parameters() {
      let param =
        entry.mutable_parameter_instruction(i).unwrap();
      let mut shape_tree = ShapeTree::new_with_value(
        param.mutable_shape(), HloReplication::unique_on_all_devices());

      let mut sharding_tree = None;
      if self.cross_partition_spmd && param.has_sharding() {
        let result =
          param.sharding().as_shape_tree(param.shape());
        check_error(&result);
        sharding_tree = Some(result.unwrap());
      }

      let replication = param.parameter_replicated_at_leaf_byffers();
      let mut leaf_index = 0;
      let _func  =
        |_subshape: &mut Shape, index: &Vec<i64>| -> Result<(), String>
      {
        if !ShapeUtil::is_leaf_index(param.shape(), index) {
          return Ok(());
        }
        if self.cross_partition_spmd && param.has_sharding() {
          // In cross-partition spmd mode, set parameter replication status
          // based on the parameter's sharding.
          let mut value = HloReplication::unique_on_all_devices();
          if sharding_tree.unwrap().element(index).is_replicated() {
            value = HloReplication::replicated_on_all_devices();
          }
          shape_tree.set_element_value(index, value);
        }
        if replication.is_some() {
          // If parameter replication status has been set explicitly, use that instead.
          if !self.cross_partition_spmd && *replication.unwrap().get(leaf_index).unwrap() {
            // Setting parameter replication status for replicas in
            // non cross-partition spmd mode.
            shape_tree.set_element_value(
              index, HloReplication::replicated_on_all_devices());
          }
          if self.cross_partition_spmd && !replication.unwrap().get(leaf_index).unwrap() {
            // Setting paramemter replication status for partitions in
            // cross-partition spmd mode.
            shape_tree.set_element_value(
              index, HloReplication::unique_on_all_devices());
          }
          leaf_index += 1;
        }
        Ok(())
      };
      //let status =
        //ShapeUtil::for_each_mutable_subshape_with_status(param.mutable_shape(), &mut func);
      //check_error(&status);
      self.hlo_replication.insert(param.clone(), shape_tree);
    }
    //self.compute_hlo_replication_on_computation(
      //&entry, false);
    Ok(())
  }

  // A helper function to recursively compute hlo_replication on a computation.
  // Returns whether hlo_replication_ is changed.
  pub fn compute_hlo_replication_on_computation(
    &mut self,
    _computation: &HloComputation,
    _mark_everything_not_replicated: bool) -> bool
  {
    /*
    let changed = false;
    let assign_or_combine_shapetree =
      |to_combine: &ShapeTree<HloReplication>, dest: &HloInstruction| -> bool
    {
      let target = self.hlo_replication.get_mut(dest);
      if target.is_none() {
        self.hlo_replication.insert(dest.clone(), to_combine.clone());
        return true;
      }
      let updated = false;
      let mut func =
        |index: &Vec<i64>, element: &mut HloReplication|
      {
        let new_replication = self.merge_replications(
          element, to_combine.element(index).clone());
      };
      target.unwrap().for_each_mutable_element(&mut func);
      updated
    };
    for inst in computation.make_instruction_post_order() {
        
    }
    changed
    */
    unimplemented!()
  }

  // Builds the replica group dedup map that allows caching replication
  // calculations for all-reduce/all-gather that share the same replica groups.
  // This can significantly help in compile times when replica groups are very
  // large.
  pub fn build_replica_group_dedup_map(&self) {
    //let dedupable_instructions =vec![];
    for comp in
      self.module.make_nonfusion_computations(&HashSet::new())
    {
      for inst in comp.instructions() {
        if inst.opcode() == HloOpcode::AllReduce || inst.opcode() == HloOpcode::AllGather {

        }
      }
    }
    unimplemented!()
  }

  pub fn merge_replications(&self, _a: &HloReplication, _b: HloReplication) -> Self {
    // Look replication pair up in map: if not found we pass the pair to an
    // overloaded constructor of HloReplication which constructs and returns
    // a merged HloReplication.

    //let key = (a, b);
    //let target = self.replication_merge_map.
    unimplemented!()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReplicationState {
  ReplicatedOnAllDevices,
  UniqueOnAllDevices,
  PartiallyReplicated,
}

impl Default for ReplicationState {
  fn default() -> Self {
    ReplicationState::ReplicatedOnAllDevices
  }
}

// A data structure that represents how an HLO is replicated among a set of
// devices. Device ID could be either partition ID or replica ID.
// We represent partial replication by grouping devices that have the same
// value into the same set.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HloReplication {
  state: ReplicationState,
  device_set_root_per_replica: Vec<Vec<i64>>,
}

impl HloReplication {
  pub fn new(
    state: ReplicationState,
    device_set_root_per_replica: Vec<Vec<i64>>) -> Self
  {
    HloReplication {
      state: state,
      device_set_root_per_replica: device_set_root_per_replica 
    }
  }

  pub fn replicated_on_all_devices() -> Self {
    HloReplication::new(ReplicationState::ReplicatedOnAllDevices,
      vec![])
  }

  pub fn unique_on_all_devices() -> Self {
    HloReplication::new(ReplicationState::UniqueOnAllDevices,
      vec![])
  }

  pub fn partially_replicated(device_sets_per_replica: &Vec<Vec<Vec<i64>>>) -> Self {
    let mut device_set_root_per_replica = vec![];
    for i in 0..device_sets_per_replica.len() {
      let device_sets = &device_sets_per_replica[i];
      let mut max_device_id: i64 = 0;
      for device_set in device_sets {
        for device_id in device_set {
          max_device_id = i64::max(max_device_id, *device_id);
        }
      }
      let mut device_set_root = vec![];
      device_set_root.resize((max_device_id as usize) + 1, 0);
      for device_set in device_sets {
        let mut min_device_id: i64 = 0;
        for device_id in device_set {
          min_device_id = i64::min(min_device_id, *device_id);
        }
        for device_id in device_set {
          device_set_root[*device_id as usize] = min_device_id;
        }
      }
      device_set_root_per_replica.push(device_set_root);
    }
    HloReplication::new(ReplicationState::PartiallyReplicated,
      device_set_root_per_replica)
  }

  pub fn merge(&self, other: &HloReplication) -> Self {
    match self.state {
      ReplicationState::ReplicatedOnAllDevices => return other.clone(),
      ReplicationState::UniqueOnAllDevices => return self.clone(),
      ReplicationState::PartiallyReplicated => {
        let mut unique_on_all_devices = true;
        let mut device_sets_per_replica: Vec<Vec<Vec<i64>>> = vec![];
        debug_assert_eq!(self.device_set_root_per_replica.len(),
          other.device_set_root_per_replica.len());
        for i in 0..self.device_set_root_per_replica.len() {
          let my_device_set_root = &self.device_set_root_per_replica[i];
          let other_device_set_root = &other.device_set_root_per_replica[i];
          let mut value_to_device_set: HashMap<i64, Vec<i64>> = HashMap::new();
          let num_devices = my_device_set_root.len();

          for device_id in 0..num_devices {
            let new_value =
              my_device_set_root[device_id] * (num_devices as i64) +
              other_device_set_root[device_id];
            value_to_device_set.get_mut(&new_value).unwrap().push(device_id as i64);
          }

          let num_value_to_device_set = value_to_device_set.len();
          debug_assert!(num_value_to_device_set <= num_devices);
          let mut device_sets = vec![];
          for value_and_device_set in value_to_device_set {
            device_sets.push(value_and_device_set.1);
          }
          device_sets_per_replica.push(device_sets);
          unique_on_all_devices &= num_value_to_device_set == num_devices;
        }
        if unique_on_all_devices {
          return HloReplication::unique_on_all_devices();
        }
        return HloReplication::partially_replicated(&device_sets_per_replica);
      }
    }
  }

  pub fn is_replicated_on_all_devices(&self) -> bool {
    self.state == ReplicationState::ReplicatedOnAllDevices
  }

  pub fn is_unique_on_all_devices(&self) -> bool {
    self.state == ReplicationState::UniqueOnAllDevices
  }

  pub fn is_replicated_within_subgroup(&self, device_ids: &Vec<i64>) -> bool {
    if device_ids.is_empty() { return true; }
    for device_set_roots in &self.device_set_root_per_replica {
      let mut found = false;
      for device_id in device_ids {
        if device_set_roots[*device_id as usize] ==
          device_set_roots[*device_ids.first().unwrap() as usize]
        {
          found = true
        }
      }
      if !found { return false; }
    }
    true
  }

  pub fn to_string(&self) -> String {
    match self.state {
      ReplicationState::ReplicatedOnAllDevices =>
        return "ReplicatedOnAllDevices".to_string(),
      ReplicationState::UniqueOnAllDevices =>
        return "UniqueOnAllDevices".to_string(),
      ReplicationState::PartiallyReplicated => {
        let mut out = "PartiallyReplicated".to_string();
        for k in 0..self.device_set_root_per_replica.len() {
          if k > 0 {
            out.push_str(", ");
          }
          out.push_str("{");
          for dev in &self.device_set_root_per_replica[k] {
            out.push_str(&dev.to_string());
            out.push_str(",");
          }
          out.push_str("}");
        }
        out.push_str("}");
        return out
      }
    }
  }
}

pub struct HashableReplicaGroupSpan {}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}