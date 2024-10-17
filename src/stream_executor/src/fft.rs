#![allow(dead_code)]

use crate::{device_memory::DeviceMemory, stream::Stream};

// Specifies FFT input and output types, and the direction.
// R, D, C, and Z stand for SP real, DP real, SP complex, and DP complex.
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

  // Creates a batched FFT plan with scratch allocator.
  //
  // stream:          The GPU stream in which the FFT runs.
  // rank:            Dimensionality of the transform (1, 2, or 3).
  // elem_count:      Array of size rank, describing the size of each dimension.
  // input_embed, output_embed:
  //                  Pointer of size rank that indicates the storage dimensions
  //                  of the input/output data in memory. If set to null_ptr all
  //                  other advanced data layout parameters are ignored.
  // input_stride:    Indicates the distance (number of elements; same below)
  //                  between two successive input elements.
  // input_distance:  Indicates the distance between the first element of two
  //                  consecutive signals in a batch of the input data.
  // output_stride:   Indicates the distance between two successive output
  //                  elements.
  // output_distance: Indicates the distance between the first element of two
  //                  consecutive signals in a batch of the output data.
  pub fn create_batched_plan_with_scratch_allocator(
    &self, _stream: &dyn Stream, _rank: i64, _elem_count: &Vec<usize>,
    _input_embed: &Vec<u64>, _input_stride: u64, _input_distance: u64,
    _output_embed: &Vec<u64>, _output_stride: u64, _output_distance: u64,
    _in_place_fft: bool, _batch_count: i64) -> Plan
  {
    unimplemented!()
  }

  pub fn update_plan_with_scratch_allocator() {}

  // Computes complex-to-complex FFT in the transform direction as specified
  // by direction parameter.
  pub fn do_fft_ctoc(
    &self,
    _stream: &dyn Stream,
    _plan: &Plan,
    _input: DeviceMemory<f64>,
    _output: DeviceMemory<f64>) -> bool
  {
    unimplemented!()
  }

  // Computes real-to-complex FFT in forward direction.
  pub fn do_fft_rtoc(
    &self,
    _stream: &dyn Stream,
    _plan: &Plan,
    _input: DeviceMemory<f64>,
    _output: DeviceMemory<f64>) -> bool
  {
    unimplemented!()
  }

  // Computes complex-to-real FFT in inverse direction.
  pub fn do_fft_ctor(
    &self,
    _stream: &dyn Stream,
    _plan: &Plan,
    _input: DeviceMemory<f64>,
    _output: DeviceMemory<f64>) -> bool
  {
    unimplemented!()
  }
}