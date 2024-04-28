#![allow(dead_code)]

// Represents a stream of dependent computations on a gpu device.
pub struct Stream {}

impl Stream {
  pub fn new() {}
  pub fn platform_specific_handle() {}
  pub fn ok() {}
  pub fn refresh_status() {}
  pub fn init() {}
  pub fn get_or_create_sub_stream() {}
  pub fn return_sub_stream() {}
  pub fn then_launch() {}
  pub fn then_wait_for() {}
  pub fn then_record_event() {}
  pub fn convolve_with_algorithm() {}
  pub fn fused_convolve_with_algorithm() {}
  pub fn cudnn_reorder_convolution_filter_and_bias() {}
  pub fn then_pool_forward() {}
  pub fn then_pool_backward() {}
  pub fn then_blas_gemm() {}
  pub fn then_blas_gemm_with_alogorithm() {}
  pub fn then_blas_gemm_strided_batched_with_algorithm() {}
  pub fn then_blas_gemm_strided_batched() {}
  pub fn then_memcpy() {}
  pub fn then_memcpy_d2h() {}
  pub fn then_memcpy_h2d() {}
  pub fn then_memcpy_d2d() {}
  pub fn then_mem_zero() {}
}