#![allow(dead_code)]

// HLO pass that hoists parameters and constants to increase opportunities
// for prefetching.
pub struct InstructionHoister {}

impl InstructionHoister {
  pub fn new() {}
  pub fn name() -> String { "instruction-hoister".to_string() }
  pub fn run() {}
}