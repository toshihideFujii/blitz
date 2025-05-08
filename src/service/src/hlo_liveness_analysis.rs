#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use common::shape_tree::ShapeTree;
use hlo::{hlo_instruction::HloInstruction, hlo_module::HloModule};

use crate::call_graph::CallGraph;

// Analysis which identifies all live {HloInstruction, shapeIndex} pairs in
// an HLO module.
pub struct HloLivenessAnalysis {
  call_graph: CallGraph,
  live_index_map: HashMap<HloInstruction, ShapeTree<bool>>
}

impl HloLivenessAnalysis {
  pub fn new(module: &HloModule) -> Self {
    HloLivenessAnalysis {
      call_graph: CallGraph::build(module, &HashSet::new()),
      live_index_map: HashMap::new(),
    }
  }

  pub fn run(module: &HloModule) -> Result<HloLivenessAnalysis, String> {
    println!("HloLivenessAnalysis::run on module {:?}", module.name());
    println!("{:?}", module.to_string());

    let liveness_analysis = HloLivenessAnalysis::new(module);
    liveness_analysis.run_analysis();

    Ok(liveness_analysis)
  }

  // Returns true if output of 'instruction' at 'shape_index' is live.
  pub fn is_live(&self, instruction: &HloInstruction, _index: usize) -> bool {
    let target = self.live_index_map.get(instruction);
    if target.is_some() {
      //target.as_ref().unwrap().element(index); // TODO
      return true;
    }
    false
  }

  fn run_analysis(&self) {}
}