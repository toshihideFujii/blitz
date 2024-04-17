#![allow(dead_code)]

// Simplifies batch dot operations.
pub struct BatchDotSimplification {}

impl BatchDotSimplification {
  pub fn new() {}
  pub fn name() -> String { "batch-dot-simplification".to_string() }
  pub fn run() {}

  fn elide_degenerate_batch_dimension_from_batch_dot() {}
}