#![allow(dead_code)]

use std::time::Duration;
use super::{
  config_entry::ConfigEntry,
  config_builder::ConfigBuilder
};

pub struct Streaming {
  streaming_dyn_allocation_enebled: ConfigEntry<bool>,
  streaming_dyn_allocation_testing: ConfigEntry<bool>,
  streaming_dyn_allocation_min_executors: ConfigEntry<i64>,
  streaming_dyn_allocation_max_executors: ConfigEntry<i64>,
  streaming_dyn_allocation_scaling_interval: ConfigEntry<Duration>,
  streaming_dyn_allocation_scaling_up_ratio: ConfigEntry<f64>, 
  streaming_dyn_allocation_scaling_down_ratio: ConfigEntry<f64>,
}

impl Streaming {
  pub fn new() -> Self {
    Streaming {
      streaming_dyn_allocation_enebled:
        ConfigBuilder::new("blits.streaming.dynamic_allocation.enabled")
          .version("3.0.0")
          .boolean_conf()
          .create_with_default(false),
      streaming_dyn_allocation_testing:
        ConfigBuilder::new("blitz.streaming.dynamic_allocation.testing")
          .version("3.0.0")
          .boolean_conf()
          .create_with_default(false),
      streaming_dyn_allocation_min_executors:
        ConfigBuilder::new("blitz.streaming.dynamic_allocation.min_executors")
          .version("3.0.0")
          .int_conf()
          .create_with_default(1), // TODO
      streaming_dyn_allocation_max_executors:
        ConfigBuilder::new("blitz.streaming.dynamic_allocation.max_executors")
          .version("3.0.0")
          .int_conf()
          .create_with_default(i64::MAX),
      streaming_dyn_allocation_scaling_interval:
        ConfigBuilder::new("blitz.streaming.dynamic_allocation.scaling_interval")
          .version("3.0.0")
          .time_conf()
          .create_with_default(Duration::from_secs(60)), // TODO
      streaming_dyn_allocation_scaling_up_ratio:
        ConfigBuilder::new("blitz.streaming.dynamic_allocation.scaling_up_ratio")
          .version("3.0.0")
          .double_conf()
          .create_with_default(0.9),
      streaming_dyn_allocation_scaling_down_ratio:
        ConfigBuilder::new("blitz.streaming.dynamic_allocation.scaling_down_ratio")
          .version("3.0.0")
          .double_conf()
          .create_with_default(0.3)
    }
  }
}