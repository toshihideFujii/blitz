#![allow(dead_code)]

use crate::util::accumulator_v2::LongAccumulator;

pub struct OutputMetrics {
  bytes_written: LongAccumulator,
  records_written: LongAccumulator,
}

impl OutputMetrics {
  pub fn new() -> Self {
    OutputMetrics {
      bytes_written: LongAccumulator::new(),
      records_written: LongAccumulator::new(),
    }
  }

  pub fn bytes_written(&self) -> u64 {
    self.bytes_written.sum()
  }

  pub fn records_written(&self) -> u64 {
    self.records_written.sum()
  }

  pub fn set_bytes_written(&mut self, v: u64) {
    self.bytes_written.set_value(v);      
  }

  pub fn set_records_written(&mut self, v: u64) {
    self.records_written.set_value(v);
  }
}