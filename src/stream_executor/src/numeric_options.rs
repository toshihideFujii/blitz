#![allow(dead_code)]

// Options that specify the numeric behavior of operations like matrix
// multiplications and convolutions
pub struct NumericOptions {
  require_determinism: bool,
  allow_tf32: bool,
}

impl NumericOptions {
  pub fn new(require_determinism: bool, allow_tf32: bool) -> Self {
    NumericOptions {
      require_determinism: require_determinism,
      allow_tf32: allow_tf32
    }
  }
}