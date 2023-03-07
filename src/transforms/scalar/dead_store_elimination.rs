#![allow(dead_code)]

// This file implements a trivial dead store elimination that
// only considers basic-block local redundant stores.
struct DSEPass {}
impl DSEPass {
  pub fn run() {}
}