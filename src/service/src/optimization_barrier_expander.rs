#![allow(dead_code)]

// This pass removes the opt-barrier operation which is functionally a no-op.
pub struct OptimizationBarrierExpander {}

impl OptimizationBarrierExpander {
  pub fn new() {}
  pub fn name() -> String { "cse-barrier-expander".to_string() }
  pub fn run() {}
}