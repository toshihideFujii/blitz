#![allow(dead_code)]

use crate::{
  device_memory::{DeviceMemoryBase, DeviceMemory},
  platform::Platform,
  stream::Stream,
  stream_executor::StreamExecutor
};

// Owning pointer for memory on a device.
//
// ScopedDeviceMemory is an owning pointer like std::unique_ptr, but it can
// point to memory that resides on a "device" (e.g. a GPU).  When a
// ScopedDeviceMemory goes out of scope, it frees the memory it owns.
//
// We say that an instance of ScopedDeviceMemory is "active" if it currently
// owns a (possibly empty) slice of memory on the device.  Moving,
// Release()'ing, Free()'ing, and other actions can deactivate an active object.
pub struct ScopedDeviceMemory<T> {
  wrapped: DeviceMemory<T>
}

impl<T> ScopedDeviceMemory<T> {
  // Construct a ScopedDeviceMemory from a custom allocator.
  //
  // Parameters:
  //  mem: Already-allocated device memory value for this scoped mechanism to
  //       deallocate. This memory must have been allocated by parent.
  //  device_ordinal: Device on which the memory was allocated.
  //  allocator: Allocator used to deallocate memory when this instance goes
  //             out of scope.
  pub fn new(
    _mem: DeviceMemoryBase,
    _device_ordinal: i64,
    _allocator: DeviceMemoryAllocator) -> Self
  {
    //ScopedDeviceMemory {
      //wrapped: DeviceMemory::default(t)
    //}
    unimplemented!()    
  }

  // Frees the existing memory, resets the wrapped memory to null.
  pub fn free(&self) -> Result<(), String> {
    unimplemented!()
  }
}

// Type alias for compatibility with the previous managed memory implementation.
pub type OwningDeviceMemory = ScopedDeviceMemory<u8>;

// Memory allocator interface for the device.
//
// Intended usage is through Allocate() functions which return an owning smart
// pointer.
pub struct DeviceMemoryAllocator {
  platform: Box<dyn Platform>
}

impl DeviceMemoryAllocator {
  // Parameter platform indicates which platform the allocator allocates memory
  // on. Must be non-null.
  pub fn new(platform: Box<dyn Platform>) -> Self {
    DeviceMemoryAllocator { platform: platform }
  }

  // Typed version of the allocation, returning typed memory.
  pub fn allocate<T>(
    &self,
    _device_ordinal: i64,
    _size: i64,
    _retry_on_failure: bool,
    _memory_space: i64) -> Result<DeviceMemory<T>, String>
  {
    unimplemented!()
  }

  // Must be a nop for null pointers. Should not be used.
  pub fn deallocate(
    &self, _device_ordinal: i64, _mem: &DeviceMemoryBase) -> Result<(), String>
  {
    unimplemented!()
  }

  // Return the platform that the allocator allocates memory on.
  pub fn platform(&self) -> &dyn Platform {
    self.platform.as_ref()
  }

  // Can we call Deallocate() as soon as a computation has been scheduled on
  // a stream, or do we have to wait for the computation to complete first?
  pub fn allows_asynchronous_deallocation(&self) -> bool {
    false
  }

  // Returns a stream pointer on which it is always safe to access memory
  // allocated by this allocator. It is not necessary to use the returned stream
  // though, as clients may have additional information letting them safely use
  // a different stream.
  pub fn get_stream(&self) -> Result<Box<dyn Stream>, String> {
    unimplemented!()
  }
}

// Default memory allocator for a platform which uses
// StreamExecutor::allocate/deallocate.
pub struct StreamExecutorMemoryAllocator {
  stram_executors: Vec<Box<dyn StreamExecutor>>
}

impl StreamExecutorMemoryAllocator {
  // Create an allocator supporting a single device, corresponding to the passed
  // executor.
  pub fn new(_executor: &dyn StreamExecutor) -> Self {
    unimplemented!()
  }

  pub fn allocate(
    &self,
    _device_ordinal: i64,
    _size: u64,
    _retry_on_failure: bool,
    _memory_space: i64) -> Result<DeviceMemory<u8>, String>
  {
    unimplemented!()
  }

  pub fn deallocate(&self, _device_ordinal: i64, _mem: &DeviceMemoryBase) -> Result<(), String>
  {
    unimplemented!()
  }

  pub fn allows_asynchronous_deallocation(&self) -> bool {
    false
  }
  
  // Gets-or-creates a stream for a given `device_ordinal` from an appropriate
  // stream executor.
  pub fn get_stream(&self, _device_ordinal: i64) -> Result<Box<dyn Stream>, String> {
    unimplemented!()
  }

  // Gets the stream executor for given device ordinal.
  pub fn get_stream_executor(&self, _device_ordinal: i64) -> Result<Box<dyn StreamExecutor>, String>
  {
    unimplemented!()
  }
}