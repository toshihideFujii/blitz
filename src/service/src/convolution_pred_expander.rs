#![allow(dead_code)]

// A pass that rewrites boolean convolutions to floating point and converts
// the result back to boolean.
pub struct ConvolutionPredExpander {}

impl ConvolutionPredExpander {
  pub fn new() {}
  pub fn name() -> String { "convolution-pred-expander".to_string() }
  pub fn instruction_matches_pattern() {}
  pub fn expand_instruction() {}
}