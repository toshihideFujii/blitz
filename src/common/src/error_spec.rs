#![allow(dead_code)]

pub struct ErrorSpec {
  abs: f64,
  rel: f64,
  relaxed_nans: bool,
  all_nans_are_equivalent: bool,
  fewer_infs_ok: bool,
}

impl ErrorSpec {
  pub fn new(abs: f64, rel: f64, relaxed_nans: bool) -> Self {
    ErrorSpec {
      abs: abs,
      rel: rel,
      relaxed_nans: relaxed_nans,
      all_nans_are_equivalent: true,
      fewer_infs_ok: false
    }
  }
}