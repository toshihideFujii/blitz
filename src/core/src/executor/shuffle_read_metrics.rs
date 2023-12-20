#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use crate::util::accumulator_v2::LongAccumulator;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShuffleReadMtrics {
  remote_blocks_fetched: LongAccumulator,
  local_blocks_fetched: LongAccumulator,
  remote_bytes_read: LongAccumulator,
  remote_bytes_read_to_disk: LongAccumulator,
  local_bytes_read: LongAccumulator,
  fetch_wait_time: LongAccumulator,
  records_read: LongAccumulator,
  corrupt_merged_block_chunks: LongAccumulator,
  merged_fetch_fallback_count: LongAccumulator,
  remote_merged_blocks_fetched: LongAccumulator,
  local_merged_blocks_fetched: LongAccumulator,
  remote_merged_chunks_fetched: LongAccumulator,
  local_merged_chunks_fetched: LongAccumulator,
  remote_merged_bytes_read: LongAccumulator,
  local_merged_bytes_read: LongAccumulator,
  remote_reqs_duration: LongAccumulator,
  remote_merged_reqs_duration: LongAccumulator,
}

impl ShuffleReadMtrics {
  pub fn remote_blocks_fetched(&self) -> u64 {
    self.remote_blocks_fetched.sum()
  }

  pub fn local_blocks_fetched(&self) -> u64 {
    self.local_blocks_fetched.sum()
  }

  pub fn remote_bytes_read(&self) -> u64 {
    self.remote_bytes_read.sum()
  }

  pub fn remote_bytes_read_to_disk(&self) -> u64 {
    self.remote_bytes_read_to_disk.sum()
  }

  pub fn local_bytes_read(&self) -> u64 {
    self.local_bytes_read.sum()
  }

  pub fn fetch_wait_time(&self) -> u64 {
    self.fetch_wait_time.sum()
  }

  pub fn records_read(&self) -> u64 {
    self.records_read.sum()
  }

  pub fn total_bytes_read(&self) -> u64 {
    self.remote_bytes_read() + self.local_bytes_read()
  }

  pub fn total_blocks_fetched(&self) -> u64 {
    self.remote_blocks_fetched() + self.local_blocks_fetched()
  }

  pub fn corrupt_merged_block_chunks(&self) -> u64 {
    self.corrupt_merged_block_chunks.sum()
  }

  pub fn merged_fetch_fallback_count(&self) -> u64 {
    self.merged_fetch_fallback_count.sum()
  }

  pub fn remote_merged_blocks_fetched(&self) -> u64 {
    self.remote_merged_blocks_fetched.sum()
  }

  pub fn local_merged_blocks_fetched(&self) -> u64 {
    self.local_merged_blocks_fetched.sum()
  }

  pub fn remote_merged_chunks_fetched(&self) -> u64 {
    self.remote_merged_chunks_fetched.sum()
  }

  pub fn local_merged_chunks_fetched(&self) -> u64 {
    self.local_merged_chunks_fetched.sum()
  }

  pub fn remote_merged_bytes_read(&self) -> u64 {
    self.remote_merged_bytes_read.sum()
  }

  pub fn local_merged_bytes_read(&self) -> u64 {
    self.local_merged_bytes_read.sum()
  }

  pub fn remote_reqs_duration(&self) -> u64 {
    self.remote_reqs_duration.sum()
  }

  pub fn remote_merged_reqs_duration(&self) -> u64 {
    self.remote_merged_reqs_duration.sum()
  }

  pub fn set_remote_blocks_fetched(&mut self, v: u64) {
    self.remote_blocks_fetched.set_value(v);
  }

  pub fn set_local_blocks_fetched(&mut self, v: u64) {
    self.local_blocks_fetched.set_value(v);
  }

  pub fn set_remote_bytes_read(&mut self, v: u64) {
    self.remote_bytes_read.set_value(v);
  }

  pub fn set_remote_bytes_read_to_disk(&mut self, v: u64) {
    self.remote_bytes_read_to_disk.set_value(v);
  }

  pub fn set_local_bytes_read(&mut self, v: u64) {
    self.local_bytes_read.set_value(v);
  }

  pub fn set_fetch_wait_time(&mut self, v: u64) {
    self.fetch_wait_time.set_value(v);
  }

  pub fn set_records_read(&mut self, v: u64) {
    self.records_read.set_value(v);
  }

  pub fn set_corrupt_merged_block_chunls(&mut self, v: u64) {
    self.corrupt_merged_block_chunks.set_value(v);
  }

  pub fn set_merged_fetch_fallback_count(&mut self, v: u64) {
    self.merged_fetch_fallback_count.set_value(v);
  }

  pub fn set_remote_merged_blocks_fetched(&mut self, v: u64) {
    self.remote_merged_blocks_fetched.set_value(v);
  }

  pub fn set_local_merged_blocks_fetched(&mut self, v: u64) {
    self.local_merged_blocks_fetched.set_value(v);
  }

  pub fn set_remote_merged_chunks_fetched(&mut self, v: u64) {
    self.remote_merged_chunks_fetched.set_value(v);
  }

  pub fn set_local_merged_chunks_fetched(&mut self, v: u64) {
    self.local_merged_chunks_fetched.set_value(v);
  }

  pub fn set_remote_merged_bytes_read(&mut self, v: u64) {
    self.remote_merged_bytes_read.set_value(v);
  }

  pub fn set_locak_merged_bytes_read(&mut self, v: u64) {
    self.local_merged_bytes_read.set_value(v);
  }

  pub fn set_remote_reqs_duration(&mut self, v: u64) {
    self.remote_reqs_duration.set_value(v);
  }

  pub fn set_remote_merged_reqs_duration(&mut self, v: u64) {
    self.remote_merged_reqs_duration.set_value(v);
  }

  pub fn set_nerge_values(&mut self) {
      
  }
}