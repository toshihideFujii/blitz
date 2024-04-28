#![allow(dead_code)]

pub struct NumericOptions {
  require_determinism: bool,
  allow_tf32: bool,
}

impl NumericOptions {
  pub fn new() {}
}