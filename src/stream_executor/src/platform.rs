#![allow(dead_code)]

use crate::{
  device_description::DeviceDescription,
  device_options::DeviceOptions,
  stream_executor::StreamExecutor
};

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
pub trait Platform {
  // Returns a key uniquely identifying this platform.
  fn id(&self) -> i64;

  // Name of this platform.
  fn name(&self) -> &String;

  // Returns the number of devices accessible on this platform.
  //
  // Note that, though these devices are visible, if there is only one userspace
  // context allowed for the device at a time and another process is using this
  // device, a call to ExecutorForDevice may return an error status.
  fn visible_device_count(&self) -> usize;

  // Returns true iff the platform has been initialized.
  fn initialized(&self) -> bool;

  // Initializes the platform. The platform must be initialized before obtaining
  // StreamExecutor objects.
  fn initialize(&self) -> Result<(), String>;

  // Returns a populated DeviceDescription for the device at the given ordinal.
  // This should not require device initialization. Note that not all platforms
  // may support acquiring the DeviceDescription indirectly.
  //
  // Alternatively callers may call GetDeviceDescription() on the StreamExecutor
  // which returns a cached instance specific to the initialized StreamExecutor.
  fn description_for_device(&self, _ordinal: i64) -> Result<DeviceDescription, String>;

  // Returns a StreamExecutor for the given ordinal if one has already been
  // created, or an error is returned if none exists.  Does not create a new
  // context with the device.
  fn find_existing(&self, _ordinal: i64) -> Result<Box<dyn StreamExecutor>, String>;

  // Returns a device with the given ordinal on this platform with a default
  // plugin configuration or, if none can be found with the given ordinal or
  // there is an error in opening a context to communicate with the device, an
  // error status is returned.
  //
  // Ownership of the executor is NOT transferred to the caller --
  // the Platform owns the executors in a singleton-like fashion.
  fn executor_for_device(&self, _ordinall: i64) -> Result<Box<dyn StreamExecutor>, String>;
}