#![allow(dead_code)]

// A pass which sinks constants into fusion computations.
pub struct FusionConstantSinking {}

impl FusionConstantSinking {
  pub fn new() {}
  pub fn name() -> String { "fusion-constant-sinking".to_string() }
  pub fn run() {}
}