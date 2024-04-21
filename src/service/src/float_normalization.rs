#![allow(dead_code)]

pub struct FloatNormalization {}

impl FloatNormalization {
  pub fn new() {}
  pub fn name() -> String { "float-normalization".to_string() }
  pub fn run() {}
}

pub struct BFloat16MixedPrecisionRemoval {}

impl BFloat16MixedPrecisionRemoval {
  pub fn new() {}
  pub fn name() -> String { "bf16-mixed-precision-removal".to_string() }
  pub fn run() {}
}