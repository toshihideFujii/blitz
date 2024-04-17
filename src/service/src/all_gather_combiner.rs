#![allow(dead_code)]

// Combines small non-dependent AllGather ops into larger combined
// AllGather ops.
pub struct AllGatherCombiner {
  combine_threshold_in_bytes: i64,
  combine_threshold_count: i64,
  combine_by_dim: bool
}

impl AllGatherCombiner {
  pub fn new() {}
  pub fn name() -> String { "all-gather-combiner".to_string() }
  pub fn run() {}
}