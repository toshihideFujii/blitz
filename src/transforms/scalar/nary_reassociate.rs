#![allow(dead_code)]

// This pass reassociates n-ary add expressions and eliminates
// the redundancy exposed by the reassociation.

struct NaryReassociatePass {}
impl NaryReassociatePass {
  pub fn run() {}
}