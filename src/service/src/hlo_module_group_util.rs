#![allow(dead_code)]

use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_reachability::HloReachabilityMap};

use crate::hlo_module_group_metadata::HloModuleGroupMetadata;

// Visit state of each instruction during DFS traversal.
pub enum VisitState {
  NotVisited,
  Visiting,
  Visited,
}

// Collection of utilities for handling HloModuleGroups.
pub struct HloModuleGroupUtil {
  metadata: HloModuleGroupMetadata
}

impl HloModuleGroupUtil {
  pub fn new(metadata: HloModuleGroupMetadata) -> Self {
    HloModuleGroupUtil { metadata: metadata }
  }

  // Returns all unique predecessors of the instruction.
  pub fn global_predecessors(&self, _instruction: &HloInstruction) -> Vec<HloInstruction> {
    unimplemented!()
  }

  // Returns all unique successors of the instruction.
  pub fn global_successors(&self, _instruction: &HloInstruction) -> Vec<HloInstruction> {
    unimplemented!()
  }

  // Returns the root instructions of the computations.
  pub fn root_instructions(&self, computations: &Vec<HloComputation>) -> Vec<HloInstruction> {
    let mut roots = vec![];
    for computation in computations {
      for instruction in computation.instructions() {
        if self.global_successors(instruction).is_empty() {
          // an instruction that has no successors, e.g., an unused instruction,
          // is in roots, even though it's not the ROOT of its computation.
          roots.push(instruction.clone());
        }
      }
    }
    roots
  }

  // Given the hlo instruction as the root, recursively visits all its predecessor
  // instructions in DFS order to visit nodes in topological order.
  pub fn visit_topological_order(
    &self,
    _visit_state: &Vec<VisitState>,
    _visit_func: &Box<dyn FnMut(&HloInstruction, &Vec<HloInstruction>)-> Result<(), String>>,
    _root: &HloInstruction,
    _send_recv_as_one_group: bool) -> Result<(), String>
  {
    unimplemented!()
  }

  // Verifies that computations are well-formed (e.g., no cycles).
  pub fn verify_computations(&self, _computations: &Vec<HloComputation>) -> Result<(), String> {
    unimplemented!()
  }

  // Below reachability utils resemble in HloComputation, except that they can
  // handle instructions across multiple computations.
  pub fn compute_reachability(&self, _computations: &Vec<HloComputation>) -> HloReachabilityMap {
    let post_order = vec![];
    /*
    let visit_func =
      |_instruction: &HloInstruction, instruction_group: &Vec<HloInstruction>| -> Result<(), String>
    {
      for hlo in instruction_group {
        post_order.push(hlo.clone());
      }
      Ok(())
    };
    let visit_func_box = Box::new(visit_func);

    let visit_states = vec![];
    for root in &self.root_instructions(computations) {
      self.visit_topological_order(
        &visit_states, 
        &visit_func_box,
        root,
        false);
    }
    */
    let reachability = HloReachabilityMap::new(post_order);
    // TODO
    reachability
  }

  // Updates the reachabilityof the given instruction, taking the global
  // predecessors and successors into account.
  pub fn update_reachability_through_instruction(
    &self, instruction: &HloInstruction, reachability_map: &mut HloReachabilityMap)
  {
    let mut worklist = vec![];
    worklist.push(instruction.clone());

    while !worklist.is_empty() {
      let item = worklist.first().unwrap();
      //worklist.pop(); // ??
      if reachability_map.set_reachability_to_union(
        self.global_predecessors(item), item)
      {
        for successor in &self.global_successors(item) {
          worklist.push(successor.clone());
        }  
      }
    };
  }

  fn cycle_to_string() {}
}