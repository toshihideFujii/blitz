#![allow(dead_code)]

// This file defines the DominatorTree class, which provides
// fast and efficient dominance queries.

use super::basic_block::BasicBlock;

pub fn calculate() {}
pub fn calculate_with_updates() {}
pub fn insert_edge() {}
pub fn delete_edge() {}
pub fn apply_updates() {}
pub fn verify() {}

struct BasicBlockEdge {
  start: BasicBlock,
  end: BasicBlock
}

impl BasicBlockEdge {
  pub fn new(start: BasicBlock, end: BasicBlock) -> Self {
    BasicBlockEdge { start: start, end: end }
  }

  pub fn get_start(&self) -> &BasicBlock {
    &self.start
  }

  pub fn get_end(&self) -> &BasicBlock {
    &self.end
  }

  // Check if this is the only edge between start and end.
  pub fn is_single_edge(&self) -> bool {
    let mut num_edges_to_end = 0;
    let ti = self.start.get_terminator();
    for i in 0..ti.unwrap().get_num_successors() {
      if ti.unwrap().get_successor(i).is_none() {
        num_edges_to_end += 1;
      }
      if num_edges_to_end >= 2 {
        return false;
      }
    }
    debug_assert!(num_edges_to_end == 1);
    true
  }
}

struct DominatorTree {}

impl DominatorTree {
  pub fn new() {}

  pub fn invalidate() {}
  pub fn dominates() {}
  pub fn is_reachable_from_entry() {}
  pub fn find_nearest_common_dominator() {}
  pub fn view_graph() {}
}

struct DominatorTreeAnalysis {}
impl DominatorTreeAnalysis {
  pub fn run() {}
}

struct DominatorTreePrinterPass {}
impl DominatorTreePrinterPass {
  pub fn run() {}
}

struct DominatorTreeVerifierPass {}
impl DominatorTreeVerifierPass {
  pub fn run() {}
}

struct DominatorTreeWrapperPass {}
impl DominatorTreeWrapperPass {
  pub fn get_dom_tree() {}
  pub fn run_on_function() {}
  pub fn verify_analysis() {}
  pub fn get_analysis_usage() {}
  pub fn release_memory() {}
  pub fn print() {}
}