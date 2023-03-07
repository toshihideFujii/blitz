#![allow(dead_code)]

// This pass performs various transformations related to eliminating
// memcpy calls, or transforming sets of stores into memset's.

struct MemCpyOptPass {}
impl MemCpyOptPass {
  pub fn run() {}
}