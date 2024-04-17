#![allow(dead_code)]

// A pass that detects all-reduces whose inputs are already the same across
// replicas using the replication analysis, then replicas those all-reduces
// with local computations.
pub struct AllReduceSimplifier {
  replica_count: i64
}

impl AllReduceSimplifier {
  pub fn new() {}
  pub fn name() -> String { "all-reduce-simp".to_string() }
  pub fn run() {}
}