#![allow(dead_code)]

// This pass tries to convert loop variant range checks to
// loop invariant by widening checks across loop iterations.

struct LoopPredicationPass {}
impl LoopPredicationPass {
  pub fn run() {}
}