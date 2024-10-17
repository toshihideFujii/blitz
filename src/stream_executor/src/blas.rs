#![allow(dead_code)]

use std::i64;

use crate::{
  device_memory::{DeviceMemory, DeviceMemoryBase},
  dnn::DataType,
  numeric_options::NumericOptions,
  stream::Stream
};

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

pub fn data_type_string(t: &DataType) -> String {
  match t {
    DataType::BF16 => return "bf16".to_string(),
    DataType::Half => return "f16".to_string(),
    DataType::Float => return "f32".to_string(),
    DataType::Double => return "f64".to_string(),
    DataType::Int8 => return "i8".to_string(),
    DataType::Int32 => return "i32".to_string(),
    DataType::ComplexFloat => return "complex f32".to_string(),
    DataType::ComplexDouble => return "complex f64".to_string(),
    _ => "Unknown DataType".to_string()
  }
}

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

// BLAS support interface -- this can be derived from a GPU executor when the
// underlying platform has an BLAS library implementation available. See
// StreamExecutor::AsBlas().
//
// Thread-hostile: CUDA associates a CUDA-context with a particular thread in
// the system. Any operation that a user attempts to perform by enqueueing BLAS
// operations on a thread not-associated with the CUDA-context has unknown
// behavior at the current time; see b/13176597
pub trait BlasSupport {

  fn get_blas_lt(&self);

  // Computes the product of a vector by a scalar: x <- a*x.
  fn do_blas_scal(
    &self,
    _stream: &dyn Stream,
    _elem_count: usize,
    _alpha: f64,
    _x: &DeviceMemory<f64>,
    _incx: i64) -> bool;

  // Computes a matrix-vector product using a general matrix.
  //
  //     y <- alpha * a * x + beta * y,
  // or
  //     y <- alpha * a' * x + beta * y,
  // or
  //     y <- alpha * conj(a') * x + beta * y,
  //
  // alpha and beta are scalars; a is an m-by-n general matrix; x is a vector
  // with n(trans==kNoTranspose)/m(otherwise) elements;
  // y is a vector with m(trans==kNoTranspose)/n(otherwise) elements.
  fn do_blas_gemv(
    &self,
    _stream: &dyn Stream,
    _trans: Transpose,
    _m: u64, _n: u64,
    _alpha: f64,
    _a: &DeviceMemory<f64>,
    _lda: i64,
    _x: &DeviceMemory<f64>, _incx: i64,
    _beta: f64,
    _y: &DeviceMemory<f64>, _incy: i64) -> bool;

  // Computes a matrix-matrix product with general matrices:
  //
  //     c <- alpha * op(a) * op(b) + beta * c,
  //
  // op(X) is one of op(X) = X, or op(X) = X', or op(X) = conj(X'); alpha and
  // beta are scalars; a, b, and c are matrices; op(a) is an m-by-k matrix;
  // op(b) is a k-by-n matrix; c is an m-by-n matrix.
  //
  // Note: The half interface uses float precision internally; the version
  // that uses half precision internally is not yet supported. There is no
  // batched version of the half-precision interface.
  //
  // Alpha/beta type matches `dtype`, unless `dtype` is `Eigen::half`, in that
  // case the expected alpha/beta type is `float`.
  fn do_blas_gemm(
    &self,
    _stream: &dyn Stream,
    _transa: Transpose, _transb: Transpose,
    _m: u64, _n: u64, _k: u64,
    _dtype: DataType,
    _alpha: f64, _a: &DeviceMemoryBase, _lda: i64,
    _b: &DeviceMemoryBase, _ldb: i64, _beta: f64,
    _c: &DeviceMemoryBase, _ldc: i64,
    _nummeric_options: &NumericOptions, _context: CallContext) -> Result<(), String>;

  // Gets a list of supported algorithms for DoBlasGemmWithAlgorithm.
  fn get_blas_gemm_algorithms(
    &self,
    _stream: &dyn Stream,
    _alpha: f64, _beta: f64,
    _out_algorithm: &Vec<AlgorithmType>) -> Result<(), String>;

  // Like DoBlasGemm, but accepts an algorithm and an compute type.
  //
  // The compute type lets you say (e.g.) that the inputs and outputs are
  // Eigen::halfs, but you want the internal computations to be done with
  // float32 precision.
  //
  // If output_profile_result is not null, a failure here does not put the
  // stream in a failure state.  Instead, success/failure is indicated by
  // output_profile_result->is_valid().  This lets you use this function for
  // choosing the best algorithm among many (some of which may fail) without
  // creating a new Stream for each attempt.
  fn do_blas_gemm_with_algorithm(
    &self,
    _stream: &dyn Stream,
    _transa: Transpose, _transb: Transpose,
    _m: u64, _n: u64, _k: u64, _alpha: f64,
    _a: &DeviceMemoryBase, _type_a: DataType, _lda: i64,
    _b: &DeviceMemoryBase, _type_b: DataType, _ldb: i64, _beta: f64,
    _c: &DeviceMemoryBase, _type_c: DataType, _ldc: i64,
    _computation_type: ComputationType, _algorithm: AlgorithmType,
    _nummeric_options: &NumericOptions,
    _output_profile_result: &ProfileResult, _context: CallContext) -> Result<(), String>;

  fn do_blas_gemm_stride_batched_with_algorithm(&self) {}

  // Computes a batch of matrix-matrix product with general matrices.
  // This is a batched version of DoBlasGemm.
  // The batched GEMM computes matrix product for each input/output in a, b,
  // and c, which contain batch_count DeviceMemory objects.
  fn do_blas_gemm_batched(
    &self,
    _stream: &dyn Stream,
    _transa: Transpose, _transb: Transpose,
    _m: u64, _n: u64, _k: u64, _alpha: f64,
    _lda: i64, _ldb: i64, _beta: f64, _ldc: i64,
    _batch_count: usize,
    _nummeric_options: &NumericOptions, _context: CallContext) -> bool;

  fn do_blas_gemm_strided_batched(&self) {}

  // Solves a triangular matrix equation.
  //
  //     op(a) * x = alpha * b,
  // or
  //     x * op(a) = alpha * b
  //
  // alpha is a scalar; x and b are m-by-n matrices; a is a unit, or non-unit,
  // upper or lower triangular matrix; op(a) is one of op(a) = a, or op(a) = a',
  // or op(a) = conj(a').
  fn do_blas_trsm(
    &self,
    _stream: &dyn Stream,
    _side: Side, _uplo: UpperLower, _transa: Transpose, _diag: Diagonal,
    _m: u64, _n: u64,
    _alpha: f64, _a: &DeviceMemory<f64>,
    _lda: i64, _b: &DeviceMemory<f64>, _ldb: i64) -> bool;

  // Same as DoBlasTrsm, but operates over a list of a's and b's.  The lists
  // `as` and `bs` must have the same length.
  fn do_blas_trsm_batched(
    &self,
    _stream: &dyn Stream,
    _side: Side, _uplo: UpperLower, _transa: Transpose, _diag: Diagonal,
    _m: u64, _n: u64, _alpha: f64, _as: &DeviceMemory<f64>, _lda: i64,
    _bs: &DeviceMemory<f64>, _ldb: i64, _batch_count: usize) -> bool;
}