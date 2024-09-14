#![allow(dead_code)]

use crate::platform::StreamPriority;

// Pointer-to-implementation object type (i.e. the Stream class delegates to
// this interface) with virtual destruction. This class exists for the
// platform-dependent code to hang any kernel data/resource info/functionality
// off of.
pub struct StreamInterface {}

impl StreamInterface {
  pub fn new() {}

  // Sets priority for a stream.
  pub fn set_priority(&self, _priority: StreamPriority) {
    unimplemented!()
  }

  // Gets priority for a stream.
  pub fn prioruty(&self) -> StreamPriority {
    StreamPriority::Default
  }

  // Returns a pointer to a platform specific stream associated with this object
  // if it exists, or nullptr otherwise. This is available via Stream public API
  // as Stream::PlatformSpecificHandle, and should not be accessed directly
  // outside of a StreamExecutor package.
  pub fn platform_specific_stream(&self) {}
}