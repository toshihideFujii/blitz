#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use common::shape_util::ShapeUtil;
use hlo::{
  hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode, hlo_reachability::HloReachabilityMap
};

use crate::hlo_dataflow_analysis::HloDataflowAnalysis;

// An internal data structure for each instruction in current computation.
// When an instruction is removed, member 'hlo' is set to nullptr.
struct FusionCandidate {
  hlo: Option<HloInstruction>,
  fusible: Vec<(HloInstruction, i64)>
}

impl FusionCandidate {
  pub fn new(hlo: HloInstruction) -> Self {
    FusionCandidate {
      hlo: Some(hlo),
      fusible: Vec::new()
    }
  }
}

// The pair of candidates to be fused and the profit score.
struct ToBeFused {
  instr1: HloInstruction,
  instr2: HloInstruction,
  score: i64,
  timestamp: i64
}

impl ToBeFused {
  pub fn new(
    instr1: HloInstruction,
    instr2: HloInstruction,
    score: i64,
    timestamp: i64) -> Self
  {
    ToBeFused {
      instr1: instr1,
      instr2: instr2,
      score: score,
      timestamp: timestamp
    }    
  }
}

struct WorkList {
  worklist: Vec<ToBeFused>,
  timestamp: i64
}

impl WorkList {
  pub fn new() -> Self {
    WorkList {
      worklist: Vec::new(),
      timestamp: 0
    }
  }

  pub fn empty(&self) -> bool {
    self.worklist.is_empty()
  }

  pub fn pop(&mut self) -> Option<ToBeFused> {
    self.worklist.pop()
  }

  pub fn emplace(&self) {}
}

pub struct MultiOutputFusion {
  //candidates: Vec<FusionCandidate>,
  candidates: [Option<FusionCandidate>; 0],
  worklist: WorkList,
  candidats_index: HashMap<HloInstruction, i64>,
  reachability: HloReachabilityMap,
  all_fusion_candidates: Vec<(HloInstruction, usize)>,
  computation: HloComputation
}

impl MultiOutputFusion {
  pub fn new() {}

  pub fn name() -> String {
    "multi-output-fusion".to_string()
  }

  pub fn run(
    &mut self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }

  // Main entry for the optimization. Returns true if the optimization happens.
  pub fn perform(&self) -> bool {
    unimplemented!()
  }

  // Whether fusing the instruction can reduce memory reads.
  pub fn is_profitable_operand(&self, instr: &HloInstruction) -> bool {
    // Constant instruction will not have memory reads, so it won't be a profit
    // source. Skip them.
    if instr.opcode() == HloOpcode::Constant && ShapeUtil::is_effective_scalar(instr.shape()) {
      return false;
    }
    // We don't target to fuse producer/consumer instructions -- this should
    // be taken care of by the instruction_fusion pass. If instr has only
    // one user, it will not have sibling instructions. We won't consider it.
    if instr.user_count() < 2 {
      return false;
    }
    true
  }

  // Test if it's legal to fuse instr1 and instr2 into one fusion instruction.
  pub fn legal_to_fuse(&self, instr1: &HloInstruction, instr2: &HloInstruction) -> bool {
    if instr1.opcode() != HloOpcode::Fusion { return false; }
    self.legal_to_fuse_main_constraints(instr1, instr2)
  }

  // Test if it's legal to fuse instr1 and instr2 into one fusion instruction
  // using main constraints.
  pub fn legal_to_fuse_main_constraints(
    &self, instr1: &HloInstruction, instr2: &HloInstruction) -> bool
  {
    if instr1 == instr2 { return false; }

    // Fusing nodes with 0 users makes no sense and the rest of the implementation
    // doesn't support it either.
    if instr1.is_dead() || instr2.is_dead() { return false; }

    // Check if the users of multioutput fusion is not a get-tuple-element.
    // If this is the case, we bail out because the transformation assumes
    // the users are get-tuple-element.
    let multioutput_user_is_not_gte = |instr: &HloInstruction| -> bool {
      if instr.is_multi_output_fusion() { return false; }
      for user in instr.users() {
        if user.opcode() != HloOpcode::GetTupleElement { return true; }
      }
      false
    };

    if multioutput_user_is_not_gte(instr1) ||
       multioutput_user_is_not_gte(instr2)
    {
      return false;
    }

    if self.is_connected(instr1, instr2) { return false; }

    // If both nodes are in-place operations and they use a common in-place
    // operand, we can't fuse these two.
    for operand_and_output_index1 in
      HloDataflowAnalysis::get_in_place_input_output_pairs(instr1)
    {
      let operand1 =
        instr1.operand(operand_and_output_index1.0.operand_number as usize);
      for operand_and_output_index2 in
        HloDataflowAnalysis::get_in_place_input_output_pairs(instr2)
      {
        let operand2 =
          instr2.operand(operand_and_output_index2.0.operand_number as usize);
        if operand1 == operand2 {
          return false;
        }
      }
    }

    true
  }

  pub fn fuse() {}

  // Recompute reachability for the current computation.
  pub fn recompute_reachability(&mut self) {
    self.reachability.reset();
    self.reachability = HloReachabilityMap::build(&self.computation);
  }

  // Returns the reachability map for the current computation.
  pub fn reachability(&self) -> &HloReachabilityMap {
    &self.reachability
  }

  // Returns the computation for the pass.
  pub fn computation(&self) -> &HloComputation {
    &self.computation
  }

  // Update the reachability map after fusing instr1 and instr2.
  pub fn update_reachability(
    &mut self,
    instr1: &HloInstruction,
    instr2: &HloInstruction,
    instrs_to_update: &Vec<(HloInstruction, usize)>,
    skip: Box<dyn Fn(&HloInstruction)->bool>)
  {
    let instr1_i = self.reachability.get_index(instr1);
    let instr2_i = self.reachability.get_index(instr2);
    
    for instr_and_index in instrs_to_update {
      let instr = &instr_and_index.0;
      if skip(instr) { continue; }

      let instr_i = instr_and_index.1;
      let instr2_instr = self.reachability.is_reachable_by_index(instr2_i, instr_i);
      let instr1_instr = self.reachability.is_reachable_by_index(instr1_i, instr_i);
      if instr2_instr && instr1_instr {
        // If a candidate was already reachable by both, no update needed.
        continue;
      }

      if instr2_instr {
        self.reachability.fast_set_rachability_to_union_by_index(
          &vec![instr1_i, instr1_i], instr_i);
      }
      if self.reachability.is_reachable_by_index(instr1_i, instr_i) {
        self.reachability.fast_set_rachability_to_union_by_index(
          &vec![instr_i, instr2_i], instr_i);
      }
    }
  }

  // Hook for multi-output fusion along producer-consumer edges.
  // Returns whether any instructions were fused.
  pub fn do_producer_consumer_multi_output_fusion(&self) -> bool {
    false
  }

  pub fn get_new_fusibles() {}
  pub fn create_fusion() {}

  // Update the internal data structures before instr1 and instr2 are fused into
  // one fusion instruction.
  fn update_before_fuse(&mut self, instr1: &HloInstruction, instr2: &HloInstruction) {
    let mut fusion = instr1;
    let mut _fused = instr2;
    if self.is_fused(instr1) {
      fusion = instr2;
      _fused = instr1;
    }

    // Insert the newly created instruction (if any), to candidates_.
    for use_ in fusion.users() {
      if self.candidats_index.get(use_).is_none() {
        // TODO
      }
    }

    // Update the reachability graph.
    let _skip = |instruction: &HloInstruction| -> bool {
      self.is_fused(instruction)
    };
    //self.update_reachability(fusion, fused,
      //&self.all_fusion_candidates, Box::new(skip));
  }

  // Update the internal data structures after instructions are fused into
  // one fusion instruction.
  fn update_after_fuse(
    &mut self,
    fusion: &HloInstruction,
    new_fusibles: &Vec<(HloInstruction, usize)>,
    new_fusion_node: bool)
  {
    let index = (*self.candidats_index.get(fusion).unwrap()) as usize;
    let _candidate_node = &self.candidates[index];

    for _it in new_fusibles {
      // TODO
      if new_fusion_node {

      } else {
          
      }
    }
  }

  fn get_candidate_id(&self, _instr: &HloInstruction) -> i64 {
    unimplemented!()
  }

  fn is_fused(&self, instr: &HloInstruction) -> bool {
    self.candidates.get(self.get_candidate_id(instr) as usize).is_none()
  }

  fn set_is_fused(&mut self, instr: &HloInstruction) {
    self.candidates[self.get_candidate_id(instr) as usize] = None;
  }

  fn is_connected(&self, instr1: &HloInstruction, instr2: &HloInstruction) -> bool {
    self.reachability.is_connected(instr1, instr2)
  }

}