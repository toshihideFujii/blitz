#![allow(dead_code)]

// HLO pass which reduces the precision of some HLO instructions to BF16
// according to the backend-specific FloatSupport rule provided by the caller.
pub struct BFloat16Propagation {}

impl BFloat16Propagation {
  pub fn new() {}

  pub fn name() -> String { "bfloat16-propagation".to_string() }

  pub fn run() {}

  pub fn should_keep_precision_unchanged() {}

  pub fn instruction_is_candidate_for_bf16_output() {}
}