#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// HLO pass that makes the following transformations on while loops:
//  - A while loop with static trip count 0 is deleted.
//
//  - A while loop with static trip count 1 is replaced by its body
//    (sans loop).
//
//  - Elements of a while loop 's tuple that the loop doesn't use are
//    removed from the tuple.
//
//  - If the while loop's parameter is a nested tuple, it's flattened
//    to a single-level tuple.
//
//  - Removing trivial compare instructions inside while bodies.
pub struct WhileLoopSimplifier {
  simplify_compare_instrs: bool
}

impl WhileLoopSimplifier {
  pub fn new(simplify_compare_instrs: bool) -> Self {
    WhileLoopSimplifier { simplify_compare_instrs: simplify_compare_instrs }
  }

  pub fn name() -> String {
    "simplify-while-loop".to_string()
  }

  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: HashSet<String>) -> Result<bool, String>
  {
    Ok(true)
  }
}