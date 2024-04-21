#![allow(dead_code)]

// Merge consecutive batch dimensions of a dot() by inserting reshapes.
pub struct DotDimensionMerger {}

impl DotDimensionMerger {
  pub fn new() {}
  pub fn name() -> String { "dot-dimension-merger".to_string() }
  pub fn run() {}
}