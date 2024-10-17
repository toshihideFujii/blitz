#![allow(dead_code)]

use crate::{command_buffer::CommandBuffer, device_memory::DeviceMemoryBase, event::Event, kernel::{Kernel, KernelArgs}, kernel_spec::MultiKernelLoaderSpec, launch_dim::{BlockDim, ThreadDim}, module_spec::{ModuleHandle, MultiModuleLoaderSpec}, platform::Platform, stream::Stream};

// Interface which defines the method for interacting with an accelerator device
// (e.g. GPU, TPU).
pub struct StreamExecutorInterface {
}

impl StreamExecutorInterface {
  // Initializes the device for use.
  pub fn init(&self) -> Result<(), String> {
    unimplemented!()
  }

  // Returns a reference to the platform that created this executor.
  pub fn get_platform(&self) -> &dyn Platform {
    unimplemented!()
  }

  // Returns the device ordinal.
  pub fn device_ordinal(&self) -> i64 {
    unimplemented!()
  }

  // Retrieves (loads) a kernel, if one exists.
  //
  // Parameters:
  //   spec: The MultiKernelLoaderSpec is usually generated as a compile-time
  //    constant into an appropriate namespace.
  //   kernel: Outparam that the kernel is loaded into. A given Kernel
  //    instantiation should not be loaded into more than once.
  pub fn get_kernel(
    &self,
    _spec: MultiKernelLoaderSpec,
    _kernel: &dyn Kernel) -> Result<(), String>
  {
    unimplemented!()
  }

  // Unloads the module with handle `module_handle`.
  pub fn unload_module(&self, _module_handle: &ModuleHandle) -> bool {
    unimplemented!()
  }

  // Loads a module for the platform this StreamExecutor is acting upon.
  //
  // `spec` describes the module to be loaded.  On success writes the handle for
  // the loaded module to `module_handle` and returns OkStatus().  Otherwise,
  // returns the error which has occurred.
  pub fn load_module(
    &self,
    _spec: &MultiModuleLoaderSpec,
    _module_handle: &ModuleHandle) -> Result<(), String>
  {
    unimplemented!()
  }

  // Creates a shared constant using the content provided.
  pub fn create_or_share_constant(
    &self, _stream: &dyn Stream, _content: &Vec<u8>) -> Result<DeviceMemoryBase, String>
  {
    unimplemented!()    
  }

  // Launches a data parallel kernel with the given thread/block
  // dimensionality and already-packed args/sizes to pass to the underlying
  // platform driver.
  pub fn launch(
    &self,
    _stream: &dyn Stream,
    _thread_dims: &ThreadDim,
    _block_dims: &BlockDim,
    _k: &dyn Kernel,
    _args: &KernelArgs) -> Result<(), String>
  {
    unimplemented!()
  }

  // Submits command buffer for execution to the underlying platform driver.
  pub fn submit(
    &self,
    _stream: &dyn Stream,
    _command_buffer: &CommandBuffer) -> Result<(), String>
  {
    unimplemented!()
  }

  // Releases any state associated with the previously loaded kernel.
  pub fn unload_kernel(&self, _kernel: &dyn Kernel) {
    unimplemented!()
  }

  // Synchronously allocates size bytes on the underlying platform and returns
  // a DeviceMemoryBase representing that allocation. In the case of failure,
  // nullptr is returned.
  pub fn allocate(&self, _size: usize, _memory_space: i64) -> &DeviceMemoryBase {
    unimplemented!()
  }

  // Deallocates the DeviceMemory previously allocated via this interface.
  // Deallocation of a nullptr-representative value is permitted.
  pub fn deallocate(&self, _mem: &DeviceMemoryBase) {
    unimplemented!()
  }

  // Allocates unified memory space of the given size, if supported.
  // See
  // https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#um-unified-memory-programming-hd
  // for more details on unified memory.
  pub fn unified_memory_allocate(&self, _size: usize) {
    unimplemented!()
  }

  // Deallocates unified memory space previously allocated with
  // UnifiedMemoryAllocate.
  pub fn unified_memory_deallocate(&self) {
    unimplemented!()
  }

  // Allocates collective device memory using ncclMemAlloc.
  // See
  // https://docs.nvidia.com/deeplearning/nccl/user-guide/docs/usage/bufferreg.html
  // for more details on User Buffer Registration.
  pub fn collective_memory_allocate(&self, _size: usize) -> Result<(), String> {
    unimplemented!()
  }

  // Deallocates collective device memory previously allocated with
  // CollectiveMemoryAllocate.
  pub fn collective_memory_deallocate(&self) -> Result<(), String> {
    unimplemented!()
  }

  // Allocates a region of host memory and registers it with the platform API.
  // Memory allocated in this manner is required for use in asynchronous memcpy
  // operations, such as Stream::Memcpy.
  pub fn host_memory_allocate(&self, _size: usize) -> Result<(), String> { // return MemoryAllocation
    unimplemented!()
  }

  // Deallocates a region of host memory allocated by HostMemoryAllocate().
  pub fn host_memory_deallocate(&self) {
    unimplemented!()
  }

  // Synchronizes all activity occurring in the StreamExecutor's context.
  pub fn synchronize_all_activity(&self) -> bool {
    unimplemented!()
  }

  // Blocks the caller while "size" bytes are zeroed out (in POD fashion) at the
  // given location in device memory.
  pub fn synchronous_mem_zero(
    &self, _location: &DeviceMemoryBase, _size: u64) -> Result<(), String>
  {
    unimplemented!()
  }
  
  // Blocks the caller while "size" bytes are copied to the given location in
  // device memory.
  pub fn synchronous_memcpy(
    &self, _device_host:&DeviceMemoryBase, /* host_src, */ _size: u64) -> Result<(), String>
  {
    unimplemented!()
  }

  // Enqueues an operation onto stream to zero out size bytes at the given
  // device memory location. Neither stream nor location may be null. Returns
  // whether the operation was successfully enqueued onto the stream.
  pub fn mem_zero(
    &self, _stream: &dyn Stream, _location: &DeviceMemoryBase, _size: u64) -> Result<(), String>
  {
    unimplemented!()
  }

  // Enqueues an operation onto stream to set 8-bit patterns starting at
  // location, for byte count given by size.  Returns whether the operation was
  // successfully enqueued onto the stream.
  pub fn memset(
    &self,
    _stream: &dyn Stream,
    _location: &DeviceMemoryBase,
    _pattern: u8,
    _size: u64) -> Result<(), String>
  {
    unimplemented!()
  }

  // Enqueues a memcpy operation onto stream, with a host destination location
  // host_dst and a device memory source, with target size size.
  pub fn memcpy(
    &self, _stream: &dyn Stream, _device_src: &DeviceMemoryBase, _size: u64) -> Result<(), String>
  {
    unimplemented!()
  }

  // Enqueues a memcpy operation onto stream, with a device destination location
  // and a device source location, with target size size. Peer access should
  // have been enabled between the StreamExecutors owning the device memory
  // regions.
  pub fn memcpy_device_to_device(
    &self,
    _stream: &dyn Stream,
    _device_dst: &DeviceMemoryBase,
    _device_src: &DeviceMemoryBase,
    _size: u64) -> bool
  {
    unimplemented!()    
  }

  // Enqueues on a stream a user-specified function to be run on the host.
  pub fn host_callback(&self, _stream: &dyn Stream /* callback */) -> bool {
    unimplemented!()
  }

  // Performs platform-specific allocation and initialization of an event.
  pub fn allocate_event(&self, _event: &dyn Event) -> Result<(), String> {
    unimplemented!()
  }

  // Performs platform-specific deallocation and cleanup of an event.
  pub fn deallocate_event(&self, _event: &dyn Event) -> Result<(), String> {
    unimplemented!()
  }

  // Inserts the specified event at the end of the specified stream.
  pub fn record_event(&self, _stream: &dyn Stream, _event: &dyn Event) -> Result<(), String> {
    unimplemented!()
  }

  // Waits for the specified event at the end of the specified stream.
  pub fn wait_for_event(&self, _stream: &dyn Stream, _event: &dyn Event) -> Result<(), String> {
    unimplemented!()
  }

  // Requests the current status of the event from the underlying platform.
  pub fn poll_for_event_status(&self, _event: &dyn Event) -> Result<(), String> {
    unimplemented!()
  }

  // Allocates stream resources on the underlying platform and initializes its
  // internals.
  pub fn allocate_stream(&self, _stream: &dyn Stream) -> bool {
    unimplemented!()
  }

  // Deallocates stream resources on the underlying platform.
  pub fn deallocate_stream(&self, _stream: &dyn Stream) {
    unimplemented!()
  }

  // Causes dependent to not begin execution until other has finished its
  // last-enqueued work.
  pub fn create_stream_dependency(&self, _dependent: &dyn Stream, _other: &dyn Stream) -> bool {
    unimplemented!()
  }

  // Causes the host code to synchronously wait for operations enqueued
  // onto stream to complete. Effectively a join on the asynchronous device
  // operations enqueued on the stream before this program point.
  pub fn block_host_until_done(&self, _stream: &dyn Stream) -> Result<(), String> {
    unimplemented!()
  }

  // Without blocking the device, retrieve the current stream status.
  pub fn get_status(&self, _stream: &dyn Stream) -> Result<(), String> {
    unimplemented!()
  }

  // Enables peer access from this StreamExecutor to memory
  // allocated by other, such that launched device code, memcpies, etc may
  // access it directly.
  pub fn enable_peer_access_to(&self, _other: &StreamExecutorInterface) -> Result<(), String> {
    unimplemented!()
  }

  // Returns whether it's possible to enable peer access from this
  // StreamExecutor to memory allocated by another.
  pub fn can_enable_peer_access_to(&self, _other: &StreamExecutorInterface) -> bool {
    unimplemented!()
  }

  // Returns the underlying device memory usage information, if it is available.
  // If it is not available (false is returned), free/total may not be
  // initialized.
  pub fn device_memory_usage(&self, _free: &i64, _total: &i64) -> bool {
    unimplemented!()
  }
}