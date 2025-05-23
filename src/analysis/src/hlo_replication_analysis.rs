#![allow(dead_code)]

enum State {
  ReplicatedOnAllDevices,
  UniqueOnAllDevices,
  PartiallyReplicated,
}

struct HloReplication {
  state: State,
  device_set_root: Vec<i64>
}

impl HloReplication {
    
}

// An HLO pass that determines whether each instruction in the module outputs
// the same value across replicas or across partitions (depending in the value
// 'cross_partition_spmd').
pub struct HloReplicationAnalysis {}

impl HloReplicationAnalysis {
  pub fn new() {}
  pub fn run() {}
}