#![allow(dead_code)]

use crate::executor_cache::ExecutorCache;

// Cuda-specific platform plugin, registered as a singleton value via
// module initializer.
pub struct CudaPlatform {
  name: String,
  executor_cache: ExecutorCache,
  min_numa_node: i64,
  limit_numa_node: i64,
}

impl CudaPlatform {
  pub fn new() {}
  pub fn bus_count() {}
  pub fn device_to_bus() {}
  pub fn first_executor_for_bus() {}
  pub fn id() {}
  pub fn visible_device_count() {}
  pub fn name() {}
  pub fn description_for_device() {}
  pub fn executor_for_device() {}
  pub fn get_executor() {}
  pub fn get_uncached_executor() {}
}