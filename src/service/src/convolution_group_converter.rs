#![allow(dead_code)]

// A pass which rewrites convolutions with feature_group_count > 1 into
// convolutions with feature_group_count = 1.
pub struct ConvolutionGroupConverter {}

impl ConvolutionGroupConverter {
  pub fn new() {}
  pub fn name() -> String { "convolution-group-converter".to_string() }
  pub fn run() {}
}