#![allow(dead_code)]

use crate::device_options::DeviceOptions;

// An enum to represent defferent levels of stream prioritues.
pub enum StreamPriority {
  Default,
  Lowest,
  Highest
}

pub fn stream_priority_to_string(_priority: StreamPriority) -> String {
  "".to_string()
}

// StreamExecutorConfig encapsulates the sest of options for construction a
// StreamExecutor for a given platform.
pub struct StreamExecutorConfig {
  ordinal: i64,
  device_options: DeviceOptions,
}

// Abstract base class for a platform registered with MultiPlatformManager.
pub struct Platform {
}

impl Platform {
  pub fn new() {}
  pub fn name() {}
  pub fn visible_device_count() {}
  pub fn initialized() {}
  pub fn initialize() {}
  pub fn description_for_device() {}
  pub fn executor_for_device() {}
  pub fn get_executor() {}
  pub fn get_uncached_executor() {}
}