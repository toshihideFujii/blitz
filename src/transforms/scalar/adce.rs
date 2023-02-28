#![allow(dead_code)]

// This file provides the interface for the Aggressive
// Dead Code Elimination pass.
// This pass optimistically assumes that all instructions
// are dead until proven otherwise, allowing it to eliminate
// dead computations that other DCE passes do not catch,
// particularly involving loop computations.

struct ADCEPass {}
impl ADCEPass {
  pub fn run() {}
}