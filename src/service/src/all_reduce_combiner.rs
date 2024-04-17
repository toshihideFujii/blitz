#![allow(dead_code)]

// Combines small non-dependent AllReduce op into larger combined AllReduce ops.
pub struct AllReduceCombiner {
  combine_threshold_in_bytes: i64,
  combine_threshold_count: i64
}

impl AllReduceCombiner {
  pub fn new() {}
  pub fn name() -> String { "all-reduce-combiner".to_string() }
  pub fn run() {}
}