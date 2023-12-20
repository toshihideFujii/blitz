#![allow(dead_code)]

pub trait ShuffleWriter {
  fn write(&self) {}
  fn stop(&self) {}
  fn get_partition_lengths(&self) {}
}