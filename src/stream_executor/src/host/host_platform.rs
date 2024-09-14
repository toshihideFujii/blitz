#![allow(dead_code)]

use crate::{
  device_description::DeviceDescription,
  executor_cache::ExecutorCache,
  //stream_executor::StreamExecutor
};

use super::host_executor::HostExecutor;

// Host (CPU) platform plugin, registered as a singleton value via module
// initializer.
pub struct HostPlatform {
  name: String,
  executor_cache: ExecutorCache
}

impl HostPlatform {
  pub fn new() -> Self {
    HostPlatform {
      name: "Host".to_string(),
      executor_cache: ExecutorCache::new()
    }
  }

  pub fn id(&self) {
    unimplemented!()
  }

  // Device count is less clear-cut for CPUs than accelerators. This call
  // currently returns the number of thread units in the host, as reported by
  // base::NumCPUs().
  pub fn visible_device_count(&self) -> usize {
    let result = std::thread::available_parallelism();
    if result.is_ok() {
      return result.unwrap().into();
    } else {
      unreachable!("std::thread::available_parallelism() is error.");
    }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn description_for_device(&self, ordinal: i64) -> Result<DeviceDescription, String> {
    HostExecutor::create_device_description_by_ordinal(ordinal)
  }

  pub fn executor_for_device(&self, _ordinal: i64) -> Result<HostExecutor, String> {
    let _factory =
      |ordinal: i64| -> Result<HostExecutor, String>
    {
      self.get_uncached_executor(ordinal)
    };
    // TODO
    //self.executor_cache.get_or_create(ordinal, Box::new(factory))
    unimplemented!()
  }

  // Returns a device constructed with ordinal without
  // looking in or storing to the Platform's executor cache.
  // Ownership IS transferred to the caller.
  fn get_uncached_executor(&self, ordinal: i64) -> Result<HostExecutor, String> {
    let executor =
      HostExecutor::new(self, ordinal);
    let init_status = executor.init();
    if init_status.is_err() {
      let mut err_msg =
        "Failed initialising StreamExecutor for device ordinal ".to_string();
      err_msg.push_str(&ordinal.to_string());
      return Err(err_msg);
    }
    Ok(executor)
  }
}