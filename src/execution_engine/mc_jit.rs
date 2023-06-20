#![allow(dead_code)]

// This file forces the MCJIT to link in on certain operating systems.

struct ForceMcJitLinking {}

impl ForceMcJitLinking {
  pub fn new() {}
}

pub fn blitz_link_in_mc_jit() {}