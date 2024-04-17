#![allow(dead_code)]

// Canonicalize output of conditionals, make non-tuple outputs into tuple
// with single element output.
pub struct ConditionalCanonicalizer {}

impl ConditionalCanonicalizer {
  pub fn new() {}
  pub fn name() -> String { "conditional-canonicalizer".to_string() }
  pub fn run() {}
}