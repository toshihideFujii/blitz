#![allow(dead_code)]

// Eit warnings if forced code transsormations have not performed.

struct WarnMissedTransformationPass {}
impl WarnMissedTransformationPass {
  pub fn run() {}
}