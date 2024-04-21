#![allow(dead_code)]

// A pass which performs map inlining.
pub struct MapInliner {}

impl MapInliner {
  pub fn new() {}
  pub fn name() -> String { "map-inline".to_string() }
  pub fn run() {}
}