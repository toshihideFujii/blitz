#![allow(dead_code)]

// This file implements the Bit-Tracking Dead Code Elimination
// pass. Some instructions (shifts, some ands, ors, etc.) kill
// some of their input bits. We track these dead bits and remove
// instructions that compute only these dead bits.

struct BDCEPass {}
impl BDCEPass {
  pub fn run() {}
}