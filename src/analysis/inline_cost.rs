#![allow(dead_code)]

// This file implements heuristics for inlining decisions.

const OPT_SIZE_THRESHOLD: i32 = 50;


struct CostBenefitPair {}
impl CostBenefitPair {
  pub fn new() {}
  pub fn get_cost() {}
  pub fn get_benefit() {}
}

struct InlineCost {}
impl InlineCost {
  pub fn new() {}
  pub fn get() {}
  pub fn get_always() {}
  pub fn get_never() {}
  pub fn is_always() {}
  pub fn is_never() {}
  pub fn is_available() {}
  pub fn get_cost() {}
  pub fn get_threshold() {}
  pub fn get_static_bonus_applied() {}
  pub fn get_cost_benefit() {}
  pub fn get_reason() {}
  pub fn get_cost_delta() {}
}

struct InlineResult {}
impl InlineResult {
  pub fn new() {}
  pub fn success() {}
  pub fn failure() {}
  pub fn is_success() {}
  pub fn get_failure_reason() {}
}

struct InlineParams {}

pub fn get_string_fn_attr_as_int() {}
pub fn get_inline_parans() {}
pub fn get_callsite_cost() {}
pub fn get_inline_cost() {}
pub fn get_attribute_based_inlining_decision() {}
pub fn get_inlining_cost_estimate() {}
pub fn get_inlining_cost_features() {}
pub fn is_inline_viable() {}
struct InlineCostAnnotationPrinterPass {}
impl InlineCostAnnotationPrinterPass {
  pub fn new() {}
  pub fn run() {}    
}