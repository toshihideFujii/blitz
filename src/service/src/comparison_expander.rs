#![allow(dead_code)]

// A pass which performs expansion of the comparison operator to support
// total order comparison of floating point numbers.
pub struct ComparisonExpander {}

impl ComparisonExpander {
  pub fn new() {}
  pub fn name() -> String { "comparison-expander".to_string() }
  fn expand_instruction() {}
}