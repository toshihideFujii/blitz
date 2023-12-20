#![allow(dead_code)]

use std::ops::AddAssign;
use serde::{Serialize, Deserialize};
use crate::util::accumulator_v2::LongAccumulator;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskMetrics {
  executor_deserialize_time: LongAccumulator,
  executor_deserialize_cpu_time: LongAccumulator,
  executor_run_time: LongAccumulator,
  executor_cpu_time: LongAccumulator,
  result_size: LongAccumulator,
  gc_time: LongAccumulator,
  resule_serialization_time: LongAccumulator,
  memory_bytes_spilled: LongAccumulator,
  disk_bytes_spilled: LongAccumulator,
  peak_execution_memory: LongAccumulator,
  //updated_block_statuses: 
}

impl TaskMetrics {
  pub fn executor_deserialize_time(&self) -> u64 {
    self.executor_deserialize_time.sum()
  }

  pub fn executor_deserialize_cpu_time(&self) -> u64 {
    self.executor_deserialize_cpu_time.sum()
  }

  pub fn executor_run_time(&self) -> u64 {
    self.executor_run_time.sum()
  }

  pub fn executor_cpu_time(&self) -> u64 {
    self.executor_cpu_time.sum()
  }

  pub fn result_size(&self) -> u64 {
    self.result_size.sum()
  }

  pub fn gc_time(&self) -> u64 {
    self.gc_time.sum()
  }

  pub fn resule_serialization_time(&self) -> u64 {
    self.resule_serialization_time.sum()
  }

  pub fn memory_bytes_spilled(&self) -> u64 {
    self.memory_bytes_spilled.sum()
  }

  pub fn disk_bytes_spilled(&self) -> u64 {
    self.disk_bytes_spilled.sum()
  }

  pub fn peak_execution_memory(&self) -> u64 {
    self.peak_execution_memory.sum()
  }

  pub fn set_executor_deserialize_time(&mut self, v: u64) {
    self.executor_deserialize_time.set_value(v);
  }

  pub fn set_executor_deserialize_cpu_time(&mut self, v: u64) {
    self.executor_deserialize_cpu_time.set_value(v);
  }

  pub fn set_executor_run_time(&mut self, v: u64) {
    self.executor_run_time.set_value(v);
  }

  pub fn set_executor_cpu_time(&mut self, v: u64) {
    self.executor_cpu_time.set_value(v);
  }

  pub fn set_result_size(&mut self, v: u64) {
    self.result_size.set_value(v);
  }

  pub fn set_gc_time(&mut self, v: u64) {
    self.gc_time.set_value(v);
  }

  pub fn set_result_serialization_time(&mut self, v: u64) {
    self.resule_serialization_time.set_value(v);
  }

  pub fn set_peak_execution_memory(&mut self, v: u64) {
    self.peak_execution_memory.set_value(v);
  }

  pub fn inc_memory_bytes_spilled(&mut self, v: u64) {
    self.memory_bytes_spilled.add_assign(v);
  }
}