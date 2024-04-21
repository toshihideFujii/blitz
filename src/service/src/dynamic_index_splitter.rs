#![allow(dead_code)]

// Convert R1 index operands to DynamicSlice and DynamicUpdateSlice ops
// into separate scalars.
pub struct DynamicIndexSplitter {}

impl DynamicIndexSplitter {
  pub fn new() {}
  pub fn name() -> String { "dynamic-index-splitter".to_string() }
  pub fn run() {}
}