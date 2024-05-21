#![allow(dead_code)]

use std::i64;

use crate::stream::Stream;

// Specifies whether the input matrix will be transposed or
// transposed+conjugated befoere any BLAS operations.
pub enum Transpose {
  NoTranspose,
  Transpose,
  ConjugateTranspose
}

// Returns a name for t.
pub fn transpose_string(t: &Transpose) -> String {
  match t {
    Transpose::NoTranspose => return "NoTranspose".to_string(),
    Transpose::Transpose => return "Transpose".to_string(),
    Transpose::ConjugateTranspose => "ConjugateTranspose".to_string(),
  }
}

// Specifies whether the upper or lower triangular part of a 
// symmetric/Hermitian matrix is used.
pub enum UpperLower {
  Upper,
  Lower
}

// Returns a name for ul.
pub fn upper_lower_string(ul: &UpperLower) -> String {
  match ul {
    UpperLower::Upper => return "Upper".to_string(),
    UpperLower::Lower => return "Lower".to_string(),
  }
}

// Specifies whether a matrix is unit triangular.
pub enum Diagonal {
  Unit,
  NonUnit
}

// Returns a name for d.
pub fn diagonal_string(d: &Diagonal) -> String {
  match d {
    Diagonal::Unit => return "Unit".to_string(),
    Diagonal::NonUnit => return "NoUnit".to_string(),
  }
}

// Specifies whether a Hermitian matrix appears on the left or
// right in operation.
pub enum Side {
  Left,
  Right
}

// Returns a name for s
pub fn side_string(s: &Side) -> String {
  match s {
    Side::Left => return "Left".to_string(),
    Side::Right => return "Right".to_string(),
  }
}

// Type with which intermediate computations of a blas routine are performed.
pub enum ComputationType {
  F16,
  F32,
  F64,
  I32,
  F16AsF32,
  BF16AsF32,
  TF32AsF32
}

// Call context information for GEMM API calls.
pub enum CallContext {
  None,
  Forward,
  BackpropInput1,
  BackpropInput2
}

// Converts a ComputationType to a string.
pub fn computation_type_string(t: &ComputationType) -> String {
  match t {
    ComputationType::F16 => return "f16".to_string(),
    ComputationType::F32 => return "f32".to_string(),
    ComputationType::F64 => return "f64".to_string(),
    ComputationType::I32 => return "i32".to_string(),
    ComputationType::F16AsF32 => return "f16 (w/ f32 accumulation)".to_string(),
    ComputationType::BF16AsF32 => return "bf16 (w/ f32 accumulation)".to_string(),
    ComputationType::TF32AsF32 => return "tf32 (w/ f32 accumulation)".to_string(),
  }
}

pub fn data_type_string() {}

// Opaque identifie for an 'algorithm" used by a blas routine.
type AlgorithmType: = i64;
pub const DEFAULT_ALGORITHM: AlgorithmType = -1;
pub const DEFAULT_BLAS_GEMM: AlgorithmType = -2;
pub const DEFAULT_BLAS_GEMV: AlgorithmType = -3;
pub const NO_ALGORITHM : AlgorithmType = -4;
pub const RUNTIME_AUTO_TUNING: AlgorithmType = -5;

// blas uses -1 to represent the default algorithm.s
pub const DEFAULT_GEMM_ALGO: AlgorithmType = -1;

// Descibes the result of a performance experiment, usually timing the speed of
// a particular AlgorithmType.
pub struct ProfileResult {
  is_valid: bool,
  algorithm: AlgorithmType,
  elapsed_time_in_ms: f64,
}

impl ProfileResult {
  pub fn is_valid(&self) -> bool {
    self.is_valid
  }

  pub fn set_is_valid(&mut self, val: bool) {
    self.is_valid = val;
  }

  pub fn algorithm(&self) -> AlgorithmType {
    self.algorithm
  }

  pub fn set_algorithm(&mut self, val: AlgorithmType) {
    self.algorithm = val;
  }

  pub fn elapsed_timme_in_ms(&self) -> f64 {
    self.elapsed_time_in_ms
  }

  pub fn set_elapsed_time_in_ms(&mut self, val: f64) {
    self.elapsed_time_in_ms = val;
  }
}

pub struct AlgorithmConfig {
  algorithm: AlgorithmType
}

impl AlgorithmConfig {
  pub fn new(algorithm: AlgorithmType) -> Self {
    AlgorithmConfig { algorithm: algorithm }
  }

  pub fn algorithm(&self) -> AlgorithmType {
    self.algorithm
  }

  pub fn set_algorithm(&mut self, val: AlgorithmType) {
    self.algorithm = val;
  }

  pub fn to_string(&self) -> String {
    self.algorithm.to_string()
  }
}

// BLAS support interfaxe.
// This can be derived from a GPU executor when the underlying platform
// has an BLAS library implementation available.
pub struct BlasSupport {}

impl BlasSupport {
  pub fn new() {}

  pub fn do_blas_axpy(
    _stream: Stream,
    _elem_count: u64,
    _alpha: f64,
    _incx: i64,
    _incy: i64) {}

  pub fn do_blas_copy() {}
  pub fn do_blas_scal() {}
  pub fn do_blas_gemv() {}
  pub fn do_blas_sbmv() {}
  pub fn do_blas_gemm() {}
  pub fn get_blas_gemm_algorithms() {}
  pub fn do_blas_gemm_with_algorithm() {}
  pub fn do_blas_gemm_stride_batched_with_algorithm() {}
  pub fn do_blas_gemm_batched() {}
  pub fn do_blas_gemm_strided_batched() {}
  pub fn do_blas_trsm() {}
  pub fn do_blas_trsm_batched() {}
}