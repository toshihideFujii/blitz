#![allow(dead_code)]

use crate::{
  event::Event,
  launch_dim::{BlockDim, ThreadDim},
  platform::StreamPriority,
  stream_executor::StreamExecutor,
  //stream_interface::StreamInterface
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
pub trait Stream {

  fn platform_specific_handle(&self) -> &PlatformSpecificHandle;

  // Returns whether any errors have occurred while entraining work for this
  // stream.
  fn ok(&self) -> bool;

  // Retrieves execution status back into the stream from the underlying
  // implementation without blocking the stream.
  //
  // Normally, Stream::BlockHostUntilDone is used to get execution status.
  // However, some devices use out-of-band mechnanisms to ensure their streams
  // have finished on-device work, without needing to block the streams. (These
  // devices should also override AllowsSyncOnCompletion to return false.) For
  // these devices, this method can be used after work is finished to retrieve
  // execution status.
  fn refresh_status(&self) -> Result<(), String>;

  // Initialize the stream. This must be performed before entraining any other
  // operations.
  fn initialize(&self) -> Result<(), String>;

  // Get or create a sub-stream from this stream. If there is any sub-stream in
  // the pool that can be reused then just return this sub-stream.  Otherwise
  // create a new sub-stream.
  fn get_or_create_sub_stream(&self) -> Result<Box<dyn Stream>, String>;

  // Return the sub-stream back to the host stream so that it can be reused
  // later. Sub-streams that are !ok() will not be reused.
  fn return_sub_stream(&self, _sub_stream: &dyn Stream);

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
  fn then_launch(
    &self,
    _thread_dims: &ThreadDim,
    _block_dims: &BlockDim) -> Result<(), String>;

  // Create a dependency for this stream's next work on the other stream
  // completing. Does not take ownership of other, and other must not be
  // null.
  //
  // Checks that a stream does not wait for itself, and it is up to the
  // user to guarantee that a stream does not come to wait on itself in a
  // cyclic manner; in that case, behavior is undefined.
  fn wait_for(&self, _other: &dyn Stream) -> Result<(), String>;

  fn wait_for_event(&self, _event: &dyn Event) -> Result<(), String>;

  // Inserts the specified event into the end of this stream. Once the stream
  // has processed all events prior to the insertion point, the event will be
  // marked as completed.
  // The stream does not take ownership of event - meaning that event's lifetime
  // must extend past the point at which it is marked complete!
  fn record_event(&self, _event: &dyn Event) -> Result<(), String>;

  // Entrain onto the stream: a memcpy to a host destination from a GPU source
  // of the given target size. host_dst must be a pointer to host memory
  // allocated by StreamExecutor::HostMemoryAllocate.
  fn memcpy(&self);

  // Entrain onto the stream: a memset of zero at a device location of size
  // bytes. The location must not be null.
  fn mem_zero(&self);

  // Returns the StreamExecutor (parent object) associated with this stream.
  fn parent(&self) -> &dyn StreamExecutor;

  //fn get_cuda_compute_capability(&self) {}

  //fn get_rotm_compute_capability(&self) {}

  // Gets priority for a stream.
  fn priority(&self) -> StreamPriority;

  // Launches a data parallel kernel with the given thread/block
  // dimensionality and already-packed args/sizes to pass to the underlying
  // platform driver.
  fn launch(&self);
}

// Platform specific handle to the underlying resources behind a stream
// implementation (e.g. it gives access to CUstream for CUDA platform).
pub struct PlatformSpecificHandle {}