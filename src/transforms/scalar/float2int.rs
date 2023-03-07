#![allow(dead_code)]

// This file provides the Float2Int pass, which aims to demote
// floating point operations to work on integers, where that is
// losslessly possible.

struct Float2IntPass {}
impl Float2IntPass {
  pub fn run() {}
}