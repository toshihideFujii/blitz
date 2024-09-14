#![allow(dead_code)]

use crate::{
  event::Event, launch_dim::{BlockDim, ThreadDim}, platform::StreamPriority, stream_executor::StreamExecutor, stream_interface::StreamInterface
};

// Represents a stream of dependent computations on a GPU device.
//
// The operations within a stream execute linearly and asynchronously until
// BlockHostUntilDone() is invoked, which synchronously joins host code with
// the execution of the stream.
//
// If any given operation fails when entraining work for the stream, ok() will
// indicate that an error has occurred. After initialization, once a stream is
// !ok(), it will never be ok().
//
// Thread-safe post-initialization.
pub struct Stream {
  parent: Box<dyn StreamExecutor>,
  implementation: StreamInterface,
  sub_stream: Vec<(Stream, bool)>,
}

impl Stream {
  // Instantiate a stream tied to parent as a platform executor. Work
  // entrained onto this stream will be launched/managed on that
  // StreamExecutor's platform.
  pub fn new(_parent: Box<dyn StreamExecutor>) -> Self {
    unimplemented!()
  }

  pub fn platform_specific_handle(&self) -> &PlatformSpecificHandle {
    unimplemented!()
  }

  // Returns whether any errors have occurred while entraining work for this
  // stream.
  pub fn ok(&self) -> bool {
    unimplemented!()
  }

  // Retrieves execution status back into the stream from the underlying
  // implementation without blocking the stream.
  //
  // Normally, Stream::BlockHostUntilDone is used to get execution status.
  // However, some devices use out-of-band mechnanisms to ensure their streams
  // have finished on-device work, without needing to block the streams. (These
  // devices should also override AllowsSyncOnCompletion to return false.) For
  // these devices, this method can be used after work is finished to retrieve
  // execution status.
  pub fn refresh_status(&self) -> Result<(), String> {
    unimplemented!()
  }

  // Initialize the stream. This must be performed before entraining any other
  // operations.
  pub fn initialize(&self) -> Result<(), String> {
    unimplemented!()
  }

  // Get or create a sub-stream from this stream. If there is any sub-stream in
  // the pool that can be reused then just return this sub-stream.  Otherwise
  // create a new sub-stream.
  pub fn get_or_create_sub_stream(&self) -> Result<Stream, String> {
    unimplemented!()
  }

  // Return the sub-stream back to the host stream so that it can be reused
  // later. Sub-streams that are !ok() will not be reused.
  pub fn return_sub_stream(&self, _sub_stream: &Stream) {
    unimplemented!()
  }

  // Entrains onto the stream of operations: a kernel launch with the given
  // (variadic) parameters for the invocation. These arguments can be things
  // like DeviceMemory or primitive types such as int. What arguments you may
  // pass to a given kernel are noted as the template parameters to the
  // TypedKernel type that the machocc compiler generates.
  //
  // Template parameters:
  //  Params...   The type list of formal parameters that the typed kernel
  //              expects, which is matched against Args...
  //  Args...     The deduced type list for passed actual arguments
  //
  // Implementation: A compile-time compatibility check is performed that has
  // some leniency versus an exact parameter pack match -- for example,
  // `const DeviceMemory<T>` is considered "pack compatible" with a
  // `const DeviceMemory<T>&` formal parameter; in part, because we don't have
  // perfect forwarding support without rvalue references. It also attempts to
  // spit out helpful static_assert error traces with information as to the
  // argument number and types that were mismatched.
  pub fn then_launch(
    &self, _thread_dims: &ThreadDim, _block_dims: &BlockDim) -> Result<(), String>
  {
    unimplemented!()
  }

  // Create a dependency for this stream's next work on the other stream
  // completing. Does not take ownership of other, and other must not be
  // null.
  //
  // Checks that a stream does not wait for itself, and it is up to the
  // user to guarantee that a stream does not come to wait on itself in a
  // cyclic manner; in that case, behavior is undefined.
  pub fn wait_for(&self, _other: &Stream) -> Result<(), String> {
    unimplemented!()
  }

  pub fn wait_for_event(&self, _event: &Event) -> Result<(), String> {
    //self.parent.wait_for_event(self, event)
    unimplemented!()
  }

  // Inserts the specified event into the end of this stream. Once the stream
  // has processed all events prior to the insertion point, the event will be
  // marked as completed.
  // The stream does not take ownership of event - meaning that event's lifetime
  // must extend past the point at which it is marked complete!
  pub fn record_event(&self, _event: &Event) -> Result<(), String> {
    //self.parent.record_event(self, event)
    unimplemented!()
  }

  // Returns the StreamExecutor (parent object) associated with this stream.
  pub fn parent(&self) -> &dyn StreamExecutor {
    //&self.parent
    unimplemented!()
  }

  pub fn get_cuda_compute_capability() {}

  pub fn get_rotm_compute_capability() {}

  pub fn priority(&self) -> StreamPriority {
    self.implementation.prioruty()
  }
}

// Platform specific handle to the underlying resources behind a stream
// implementation (e.g. it gives access to CUstream for CUDA platform).
pub struct PlatformSpecificHandle {

}