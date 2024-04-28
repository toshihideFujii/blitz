#![allow(dead_code)]

pub enum Transpose {
  NoTranspose,
  Transpose,
  ConjugateTranspose
}

pub fn transpose_string() {}

pub enum UpperLower {
  Upper,
  Lower
}

pub fn upper_lower_string() {}

pub enum Diagonal {
  Unit,
  NonUnit
}

pub fn diagonal_string() {}

pub enum Side {
  Left,
  Right
}

pub fn side_string() {}

pub enum ComputationType {
  F16,
  F32,
  F64,
  I32,
  F16AsF32,
  BF16AsF32,
  TF32AsF32
}

pub enum CallContext {
  None,
  Forward,
  BackpropInput1,
  BackpropInput2
}

pub fn computation_type_string() {}

pub fn data_type_string() {}

pub const DEFAULT_ALGORITHM: i64 = -1;
pub const DEFAULT_BLAS_GEMM: i64 = -2;
pub const DEFAULT_BLAS_GEMV: i64 = -3;
pub const NO_ALGORITHM : i64 = -4;
pub const RUNTIME_AUTO_TUNING: i64 = -5;

pub const DEFAULT_GEMM_ALGO: i64 = -1;

pub struct ProfileResult {}

impl ProfileResult {
  pub fn is_valid() {}
  pub fn set_is_valid() {}
  pub fn algorithm() {}
  pub fn set_algorithm() {}
  pub fn elapsed_timme_in_ms() {}
  pub fn set_elapsed_time_in_ms() {}
}