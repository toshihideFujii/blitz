#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// HLO pass that rewrites while loops to sink all-reduces that are only
// accumulated into a buffer and not otherwise used in the loop body.
// An all-reduce instruction can be sinked if its result is only added
// to a number of accumulation buffers, and the accumulation buffers are not
// used inside the loop.
//
// Pattern before this pass:
// a = ...
// while:
//   b = ...
//   c = all-reduce(b)
//   a += c
// Pattern after this pass:
// a = ...
// d = 0
// while:
//   b = ...
//   d += b
// e = all-reduce(d)
// a += e
pub struct WhileLoopAllReduceCodeMotion {
  enable_reduce_scatter: bool
}

impl WhileLoopAllReduceCodeMotion {
  pub fn new(enable_reduce_scatter: bool) -> Self {
    WhileLoopAllReduceCodeMotion {
      enable_reduce_scatter: enable_reduce_scatter
    }
  }

  pub fn name(&self) -> String {
    "while-loop-all-reduce-code-motion".to_string()
  }

  pub fn run(
    &mut self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }
}