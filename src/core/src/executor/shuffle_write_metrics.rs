#![allow(dead_code)]

use std::ops::{AddAssign, SubAssign};
use serde::{Serialize, Deserialize};
use crate::{
  util::accumulator_v2::LongAccumulator,
  shuffle::metrics::ShuffleWriteMetricsReporter,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShuffleWriteMetrics {
  bytes_written: LongAccumulator,
  records_written: LongAccumulator,
  write_time: LongAccumulator,
}

impl ShuffleWriteMetrics {
  pub fn bytes_written(&self) -> u64 {
    self.bytes_written.sum()
  }

  pub fn records_written(&self) -> u64 {
    self.records_written.sum()
  }

  pub fn write_time(&self) -> u64 {
    self.write_time.sum()
  }
}

impl ShuffleWriteMetricsReporter for ShuffleWriteMetrics {
  fn inc_bytes_written(&mut self, v: u64) {
    self.bytes_written.add_assign(v);
  }

  fn inc_recordes_written(&mut self, v: u64) {
    self.records_written.add_assign(v);
  }

  fn inc_write_time(&mut self, v: u64) {
    self.write_time.add_assign(v);
  }

  fn dec_bytes_written(&mut self, v: u64) {
    self.bytes_written.sub_assign(v);
  }

  fn dec_records_written(&mut self, v: u64) {
    self.records_written.sub_assign(v);
  }
}