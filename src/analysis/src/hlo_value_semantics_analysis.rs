#![allow(dead_code)]

use hlo::hlo_value::HloPosition;

struct SendRecvGroup {}

struct SendRecvGroupMap {}

pub struct HloPreOrderDFS {}

impl HloPreOrderDFS {
  pub fn new() {}
  pub fn run() {}
  fn is_ready() {}
}

pub struct EinsumHeightAnalysis {}

impl EinsumHeightAnalysis {
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum HloValueSemanticLabel {
  Static,
  Random,
  Weight,
  Activation,
  ActivationGradient,
  WeightGradient,
  TupleOrToken,
}

pub fn hlo_value_semantic_label_to_string() {}

pub struct HloValueSemantics {
  id: i64,
  label: HloValueSemanticLabel,
  origin: HloPosition
}

impl HloValueSemantics {
  pub fn new() {}

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn label(&self) -> HloValueSemanticLabel {
    self.label.clone()
  }

  pub fn origin(&self) -> &HloPosition {
    &self.origin
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }
}

pub struct HloValueSemanticsAnalysis {

}

impl HloValueSemanticsAnalysis {
  pub fn new() {}
  pub fn run() {}
  pub fn get_semantics() {}
  pub fn get_semantics_map() {}
  pub fn get_einsum_depth_map() {}
  pub fn get_einsum_height_map() {}
  pub fn get_depth() {}
  pub fn get_height() {}
  pub fn get_send_recv_group_map() {}
  pub fn get_matching_send_or_recv() {}
}