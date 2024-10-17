#![allow(dead_code)]

use crate::stream::Stream;

// BLAS plugin for CUDA platform via cuBLAS library.
pub struct CUDABlas {}

impl CUDABlas {
  pub fn new() {}
  pub fn init() {}

  pub fn do_blas_axpy(
    _stream: Box<dyn Stream>,
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

  fn set_stream() {}
  fn cuda_stream() {}
  fn do_blas_internal_impl() {}
  fn do_blas_internal() {}
  fn do_blas_gemm_batched_internal() {}
}