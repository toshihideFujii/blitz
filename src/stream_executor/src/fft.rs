#![allow(dead_code)]

// Specifies FFT input and out types, and the direction.
pub enum FftType {
  Invalid,
  C2CForward,
  C2CInverse,
  C2R,
  R2C,
  Z2ZForward,
  Z2ZInverse,
  Z2D,
  D2Z,
}

pub struct Plan {}

pub struct FftSupport {}

impl FftSupport {
  pub fn new() {}
  pub fn create_batched_plan_with_scratch_allocator() {}
  pub fn update_plan_with_scratch_allocator() {}
  pub fn do_fft() {}
}