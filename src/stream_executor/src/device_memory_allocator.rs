#![allow(dead_code)]

use crate::{device_memory::{DeviceMemoryBase, Devicememory}, platform::Platform, stream::Stream, stream_executor::StreamExecutor};

// Memory allocator interface for the device.
//
// Intended usage is through Allocate() functions which return an owning smart
// pointer.
pub struct DeviceMemoryAllocator {
  platform: Platform
}

impl DeviceMemoryAllocator {
  // Parameter platform indicates which platform the allocator allocates memory
  // on. Must be non-null.
  pub fn new(platform: Platform) -> Self {
    DeviceMemoryAllocator { platform: platform }
  }

  // Typed version of the allocation, returning typed memory.
  pub fn allocate<T>(
    &self,
    _device_ordinal: i64,
    _size: i64,
    _retry_on_failure: bool,
    _memory_space: i64) -> Result<Devicememory<T>, String>
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
  pub fn platform(&self) -> &Platform {
    &self.platform
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
  pub fn get_stream(&self) -> Result<Stream, String> {
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
    _memory_space: i64) -> Result<Devicememory<u8>, String>
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
  pub fn get_stream(&self, _device_ordinal: i64) -> Result<Stream, String> {
    unimplemented!()
  }

  // Gets the stream executor for given device ordinal.
  pub fn get_stream_executor(&self, _device_ordinal: i64) -> Result<Box<dyn StreamExecutor>, String>
  {
    unimplemented!()
  }
}