#![allow(dead_code)]

// This is a legalization pass that propagates the memory space in the
// layout to the fusion computations.
pub struct MemorySpacePropagation {}

impl MemorySpacePropagation {
  pub fn new() {}
  pub fn name() -> String { "memory-space-propagation".to_string() }
  pub fn run() {}
}