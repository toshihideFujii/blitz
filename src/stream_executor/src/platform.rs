#![allow(dead_code)]

use crate::{
  device_description::DeviceDescription,
  device_options::DeviceOptions,
  stream_executor::StreamExecutor
};

// An enum to represent different levels of stream priorities.
// This is to avoid platform-specific representations in abstractions.
#[derive(Debug, Clone, PartialEq)]
pub enum StreamPriority {
  Default,
  Lowest,
  Highest
}

// Returns a printable description of StreamPriority.
pub fn stream_priority_to_string(_priority: StreamPriority) -> String {
  unimplemented!()
}

// StreamExecutorConfig encapsulates the sest of options for construction a
// StreamExecutor for a given platform.
pub struct StreamExecutorConfig {
  ordinal: i64,
  device_options: DeviceOptions,
}

#[derive(Debug, Clone)]
pub struct Platform {}

// Abstract base class for a platform registered with MultiPlatformManager.
impl Platform {
  // Returns a key uniquely identifying this platform.
  pub fn id(&self) -> i64 {
    unimplemented!()
  }

  // Name of this platform.
  pub fn name(&self) -> &String {
    unimplemented!()
  }

  // Returns the number of devices accessible on this platform.
  //
  // Note that, though these devices are visible, if there is only one userspace
  // context allowed for the device at a time and another process is using this
  // device, a call to ExecutorForDevice may return an error status.
  pub fn visible_device_count(&self) -> usize {
    unimplemented!()
  }

  // Returns true iff the platform has been initialized.
  pub fn initialized(&self) -> bool {
    unimplemented!()
  }

  // Initializes the platform. The platform must be initialized before obtaining
  // StreamExecutor objects.
  pub fn initialize(&self) -> Result<(), String> {
    unimplemented!()
  }

  // Returns a populated DeviceDescription for the device at the given ordinal.
  // This should not require device initialization. Note that not all platforms
  // may support acquiring the DeviceDescription indirectly.
  //
  // Alternatively callers may call GetDeviceDescription() on the StreamExecutor
  // which returns a cached instance specific to the initialized StreamExecutor.
  pub fn description_for_device(
    &self, _ordinal: i64) -> Result<DeviceDescription, String>
  {
    unimplemented!()
  }

  // Returns a StreamExecutor for the given ordinal if one has already been
  // created, or an error is returned if none exists.  Does not create a new
  // context with the device.
  pub fn find_existing(
    &self, _ordinal: i64) -> Result<StreamExecutor, String> {
    unimplemented!()
  }

  // Returns a device with the given ordinal on this platform with a default
  // plugin configuration or, if none can be found with the given ordinal or
  // there is an error in opening a context to communicate with the device, an
  // error status is returned.
  //
  // Ownership of the executor is NOT transferred to the caller --
  // the Platform owns the executors in a singleton-like fashion.
  pub fn executor_for_device(
    &self, _ordinall: i64) -> Result<StreamExecutor, String> {
    unimplemented!()
  }
}