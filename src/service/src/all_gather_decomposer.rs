#![allow(dead_code)]

// AllGatherDecomposer is a pass which converts unsupported all-gathers into
// dynamic-update-slices and all-reduces.
pub struct AllGatherDecomposer {}

impl AllGatherDecomposer {
  pub fn new() {}
  pub fn name() -> String { "all_gather_decomposesr".to_string() }
  pub fn run() {}
}