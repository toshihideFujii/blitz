#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// A pass which rewrites batch norm operations into more operations. Breaking a
// big operation into smaller operations helps leverage our generic fusion
// logic.
pub struct BatchNormExpander {
  rewrite_training_op: bool,
  rewrite_inference_op: bool,
  rewrite_grad_op: bool
}

impl BatchNormExpander {
  pub fn new(
    rewrite_training_op: bool,
    rewrite_inference_op: bool,
    rewrite_grad_op: bool) -> Self
  {
    BatchNormExpander {
      rewrite_training_op: rewrite_training_op,
      rewrite_inference_op: rewrite_inference_op,
      rewrite_grad_op: rewrite_grad_op
    }
  }

  pub fn name(&self) -> String {
    "batchnorm-expander".to_string()
  }

  // Run operation expander on the given computation. Returns whether the
  // computation was changed.
  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }
}