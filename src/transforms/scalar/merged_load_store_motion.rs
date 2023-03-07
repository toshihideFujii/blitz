#![allow(dead_code)]

// This pass performs of loads and stores on both sides of
// a diamond. It hoists the loads and sinks the stores.

struct MergedLoadStoreMotionPass {}
impl MergedLoadStoreMotionPass {
  pub fn run() {}
}