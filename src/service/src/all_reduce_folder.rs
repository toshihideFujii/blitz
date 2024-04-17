#![allow(dead_code)]

// A pass that folds an all-reduce feeding into another all-reduce by
// expanding the replica groups.
pub struct AllReduceFolder {}

impl AllReduceFolder {
  pub fn new() {}
  pub fn name() -> String { "all-reduce-folder".to_string() }
  pub fn run() {}
}