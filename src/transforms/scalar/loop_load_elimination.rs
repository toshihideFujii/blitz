#![allow(dead_code)]

// This file defines the LoopLoadEliminationPass object.
// This pass forwards loaded values around loop backedges
// to allow their use in subsequent iterations.

struct LoopLoadEliminationPass {}
impl LoopLoadEliminationPass {
  pub fn run() {}
}