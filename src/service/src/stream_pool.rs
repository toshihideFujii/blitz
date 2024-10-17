#![allow(dead_code)]

// Pool of stream_executor::Streams, which are created as needed and
// destroyed when the pool is destroyed.
pub struct StreamPool {}

impl StreamPool {
  pub fn new() {}
}