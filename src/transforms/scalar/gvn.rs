#![allow(dead_code)]

// This file provides the interface for Global Value Numbering
// pass which eliminates fully redundant instructions.
// It also does somewhat ad-hoc PRE and dead load elimination.

struct GVNPass {}
impl GVNPass {
  pub fn run() {}
}