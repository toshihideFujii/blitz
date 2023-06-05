#![allow(dead_code)]

// The ScalarEvolution class is an Blitz pass which can be used to
// analyze and categorize scalar expressions in loops.

struct SCEV {}
impl SCEV {
  pub fn new() {}
  pub fn get_scev_type() {}
  pub fn get_type() {}
  pub fn operands() {}
  pub fn is_zero() {}
  pub fn is_one() {}
  pub fn is_all_ones_value() {}
  pub fn is_non_constant_negative() {}
  pub fn get_expression_size() {}
  pub fn print() {}
  pub fn dump() {}
}

struct SCEVPredicate {}
impl SCEVPredicate {
  pub fn new() {}
  pub fn get_kind() {}
  pub fn get_complexity() {}
  pub fn is_always_true() {}
  pub fn implies() {}
  pub fn print() {}
}