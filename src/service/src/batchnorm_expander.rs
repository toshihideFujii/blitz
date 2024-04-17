#![allow(dead_code)]

// A pass which rewrites batch norm operations into more operations.
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

  pub fn name() -> String { "batchnorm-expander".to_string() }
  pub fn run() {}
}