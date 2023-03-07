#![allow(dead_code)]

// This pass pergorms loop invariant code motion, attempting
// to remove as much code from the body of a loop as possible.

struct LICMPass {}
impl LICMPass {
  pub fn run() {}
}

// Perform Loop Nest Invariant Code Motion Pass.
struct LNICMPass {}
impl LNICMPass {
  pub fn run() {}
}