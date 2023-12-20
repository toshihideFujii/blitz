#![allow(dead_code)]

pub trait ShuffleReadMetricsReporter {
  fn inc_remote_blocks_fetched(&mut self, v: u64);
  fn inc_local_blocks_fetched(&mut self, v: u64);
  fn inc_remote_bytes_read(&mut self, v: u64);
  fn inc_remote_bytes_read_to_disk(&mut self, v: u64);
  fn inc_local_bytes_read(&mut self, v: u64);
  fn inc_corrupt_merge_block_chunks(&mut self, v: u64);
  fn inc_merged_fetch_fallback_count(&mut self, v: u64);
  fn inc_remote_merged_blocks_fetched(&mut self, v: u64);
  fn inc_local_merged_blocks_fetched(&mut self, v: u64);
  fn inc_remote_merged_chunks_fetched(&mut self, v: u64);
  fn inc_remote_merged_bytes_read(&mut self, v: u64);
  fn inc_local_merged_bytes_read(&mut self, v: u64);
  fn inc_remote_reqs_duration(&mut self, v: u64);
  fn inc_remote_merged_reqs_duration(&mut self, v: u64);
}

pub trait ShuffleWriteMetricsReporter {
  fn inc_bytes_written(&mut self, v: u64);
  fn inc_recordes_written(&mut self, v: u64);
  fn inc_write_time(&mut self, v: u64);
  fn dec_bytes_written(&mut self, v: u64);
  fn dec_records_written(&mut self, v: u64);
}