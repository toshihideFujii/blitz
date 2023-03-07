#![allow(dead_code)]

// Loop unroll pass that only does full loop unrolling and peeling.
struct LoopFullUnrollPass {}
impl LoopFullUnrollPass {
  pub fn run() {}
}

// Loop unroll pass that will support both full and partial unrolling.
struct LoopUnrollPass {}
impl LoopUnrollPass {
  pub fn run() {}
}