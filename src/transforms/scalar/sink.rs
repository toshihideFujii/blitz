#![allow(dead_code)]

// This pass moves instructions into successor blocks, when possible,
// so that they aren't executed on paths where their results aren't needed.

struct SinkingPass {}
impl SinkingPass {
  pub fn run() {}
}