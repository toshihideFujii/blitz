#![allow(dead_code)]

pub struct FusionDecision {}

impl FusionDecision {
  pub fn new() {}
  pub fn can_fuse() {}
  pub fn or() {}
  pub fn and() {}
  pub fn explain() {}
}

// HLO pass which performs instruction fusion.
pub struct InstructionFusion {}

impl InstructionFusion {
  pub fn new() {}
  pub fn name() -> String { "fusion".to_string() }
  pub fn run() {}
  pub fn is_expensive() {}
  pub fn should_fuse_in_place_op() {}

  fn get_non_fusion_computations() {}
  fn get_fusion_queue() {}
  fn should_fuse() {}
  fn should_fuse_into_multi_output() {}
  fn choose_kind() {}
  fn fuse_instruction() {}
  fn fuse() {}
  fn fuse_into_multi_output() {}
  fn effective_at_most_unary() {}
  fn fusion_would_duplicate() {}
}