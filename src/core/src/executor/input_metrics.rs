#![allow(dead_code)]

use std::ops::AddAssign;

use crate::util::accumulator_v2::LongAccumulator;


pub struct InputMetrics {
  bytes_read: LongAccumulator,
  records_read: LongAccumulator,
}

impl InputMetrics {
  pub fn new() -> Self {
    InputMetrics {
      bytes_read: LongAccumulator::new(),
      records_read: LongAccumulator::new(),
    }
  }

  pub fn bytes_read(&self) -> u64 {
    self.bytes_read.sum()
  }

  pub fn records_read(&self) -> u64 {
    self.records_read.sum()
  }

  pub fn inc_bytes_read(&mut self, v: u64) {
    self.bytes_read.add_assign(v);
  }

  pub fn inc_recordes_read(&mut self, v: u64) {
    self.records_read.add_assign(v);
  }

  pub fn set_bytes_read(&mut self, v: u64) {
    self.bytes_read.set_value(v);
  }

  pub fn set_records_read(&mut self, v: u64) {
    self.records_read.set_value(v);
  }
}