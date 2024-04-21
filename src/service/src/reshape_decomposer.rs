#![allow(dead_code)]

// Decomposes a reshape which does not satisfy the ReshapeIsBitcast precondition
// into a bitcast and a copy (physical transposition).
pub struct ReshapeDecomposer {}

impl ReshapeDecomposer {
  pub fn new() {}
  pub fn name() -> String { "reshape-decomposer".to_string() }
  pub fn run() {}
}