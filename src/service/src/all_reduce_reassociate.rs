#![allow(dead_code)]

// A pass that reassociates all-reduce feeding into compatible elementwise
// operations.
pub struct AllReduceReassociate {
  reassociate_converted_ar: bool
}

impl AllReduceReassociate {
  pub fn new() {}
  pub fn name() -> String { "all-reduce-reassociate".to_string() }
  pub fn run() {}
}