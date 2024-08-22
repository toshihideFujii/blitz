#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// HLO pass that makes the following transformations on while loops:
//
//  - A while loop with static trip count of 0 is deleted.
//
//  - A while loop with static trip count of 1 is replaced by its body (sans
//    loop).
//
//  - Elements of a while loop's tuple that the loop doesn't use are removed
//    from the tuple.
//
//  - If the while loop's parameter is a nested tuple, it's flattened to a
//    single-level tuple.  This is good because it usually reduces the number of
//    kTuple instructions, but also because it unlocks additional optimizations
//    (e.g. removing unused loop parameters).
//
//  - Removing trivial compare instructions inside while bodies. Assuming a
//    while loop with known trip count, k, loop induction variable i, and the
//    initial loop induction value c, a compare(i,x) instruction is trivial if:
//      1) x is a constant and x >= k + c.
//      2) x is a constant x <= c.
//
// Flattening nested while loop tuples adds a whole mess of likely unnecessary
// kGetTupleElement and kTuple operations to the graph.  We expect that tuple
// simplifier will be run afterwards.
pub struct WhileLoopSimplifier {
  simplify_compare_instrs: bool
}

impl WhileLoopSimplifier {
  pub fn new(simplify_compare_instrs: bool) -> Self {
    WhileLoopSimplifier { simplify_compare_instrs: simplify_compare_instrs }
  }

  pub fn name(&self) -> String {
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