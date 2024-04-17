#![allow(dead_code)]

// Concatenates all-reduce operands together, so the all-reduce is performed
// over a single, contiguous buffer.
pub struct AllReduceContiguous {}

impl AllReduceContiguous {
  pub fn new() {}
  pub fn name() -> String { "all-reduce-contiguous".to_string() }
  pub fn run() {}
}