#![allow(dead_code)]

use common::executable_run_options::ExecutableRunOptions;
use stream_executor::{device_memory_allocator::DeviceMemoryAllocator, stream::Stream};

// Class containing options for running a LocalExecutable and other auxiliary
// data.
pub struct ServiceExecutableRunOptions {
  run_options: ExecutableRunOptions
}

impl ServiceExecutableRunOptions {
  pub fn new() -> Self {
    ServiceExecutableRunOptions {
      run_options: ExecutableRunOptions::new()
    }
  }

  // Returns reference or pointer to `ExecutableRunOptions` member.
  pub fn run_options(&self) -> &ExecutableRunOptions {
    &self.run_options
  }

  pub fn mutable_run_options(&mut self) -> &mut ExecutableRunOptions {
    &mut self.run_options
  }

  pub fn stream(&self) -> &Option<Stream> {
    self.run_options.stream()
  }

  pub fn allocator(&self) -> &DeviceMemoryAllocator {
    self.run_options.allocator()
  }

  pub fn device_ordinal(&self) -> i64 {
    self.run_options.device_ordinal()
  }

  pub fn borrow_stream() {}
  pub fn borrow_streams() {}
}