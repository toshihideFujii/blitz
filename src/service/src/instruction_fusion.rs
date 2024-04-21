#![allow(dead_code)]

pub struct FusionDecision {}

impl FusionDecision {
  pub fn new() {}
}

// HLO pass which performs instruction fusion.
pub struct InstructionFusion {}

impl InstructionFusion {
  pub fn new() {}
  pub fn name() -> String { "fusion".to_string() }
  pub fn run() {}
}