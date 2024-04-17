#![allow(dead_code)]

// A pass which folds F32 <-> BF16 conversions to their operands or users,
// when it is supported by the backend.
pub struct BFloat16ConversionFolding {}

impl BFloat16ConversionFolding {
  pub fn new() {}
  pub fn name() -> String { "bfloat16-fold".to_string() }
  pub fn run() {}
}