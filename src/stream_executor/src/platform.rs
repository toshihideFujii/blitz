#![allow(dead_code)]

use std::collections::HashMap;

use crate::{device_description::DeviceDescription, device_options::DeviceOptions, stream_executor::StreamExecutor};

// An enum to represent defferent levels of stream prioritues.
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

// Abstract base class for a platform registered with MultiPlatformManager.
pub struct Platform {
}

impl Platform {
  pub fn new() {}

  // Name of this platform.
  pub fn name(&self) -> String {
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

  // Initializes the platform with a custom set of options. The platform must be
  // initialized before obtaining StreamExecutor objects.  The interpretation of
  // the platform_options argument is implementation specific.  This method may
  // return an error if unrecognized options are provided.  If using
  // PlatformManager, this method will be called automatically by
  // InitializePlatformWithId/InitializePlatformWithName.
  pub fn initialize(&self, _platform_options: HashMap<String, String>) -> Result<(), String> {
    unimplemented!()
  }

  // Returns a populated DeviceDescription for the device at the given ordinal.
  // This should not require device initialization. Note that not all platforms
  // may support acquiring the DeviceDescription indirectly.
  //
  // Alternatively callers may call GetDeviceDescription() on the StreamExecutor
  // which returns a cached instance specific to the initialized StreamExecutor.
  pub fn description_for_device(&self, _ordinal: i64) -> Result<DeviceDescription, String> {
    unimplemented!()
  }

  // Returns a device with the given ordinal on this platform with a default
  // plugin configuration or, if none can be found with the given ordinal or
  // there is an error in opening a context to communicate with the device, an
  // error status is returned.
  //
  // Ownership of the executor is NOT transferred to the caller --
  // the Platform owns the executors in a singleton-like fashion.
  pub fn executor_for_device(&self, _ordinall: i64) -> Result<Box<dyn StreamExecutor>, String> {
    unimplemented!()
  }

  // Returns a device constructed with the options specified in "config".
  // Ownership of the executor is NOT transferred to the caller.
  pub fn get_executor(&self, _config: &StreamExecutorConfig) -> Result<Box<dyn StreamExecutor>, String> {
    unimplemented!()
  }

  pub fn get_uncached_executor() {}
}