#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use hlo::hlo_module::HloModule;

use crate::graphcycles::graphcycles::GraphCycles;

struct ComputationInstructionOrdering {
  node_id_to_graph_id: HashMap<i64, i64>,
  graph_cycles: GraphCycles,
}

impl ComputationInstructionOrdering {
  pub fn new() {}
  pub fn node_id_for_instruction() {}
  pub fn insert_edge() {}
}


// Adds control dependency edges from instructions which "write" values inside
// the loop, to instructions which "read" those same values, in order to avoid
// extraneous copies.
pub struct LoopScheduleLinearizer {}

impl LoopScheduleLinearizer {
  pub fn new() {}

  pub fn name() -> String {
    "loop-schedule-linearizer".to_string()
  }

  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }
}