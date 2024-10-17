#![allow(dead_code)]

use crate::{
  allocator_stats::AllocatorStats,
  blas::BlasSupport,
  command_buffer::{CommandBuffer, CommandBufferMode},
  device_description::DeviceDescription,
  device_memory::DeviceMemoryBase,
  dnn::DnnSupport,
  event::Event,
  fft::FftSupport,
  kernel::Kernel,
  kernel_spec::MultiKernelLoaderSpec,
  module_spec::{ModuleHandle, MultiModuleLoaderSpec},
  platform::{Platform, StreamPriority},
  stream::Stream
};


/// The StreamExecutor is a single-device abstraction for:
//
// * Loading/launching data-parallel-kernels
// * Invoking pre-canned high-performance library routines (like matrix
//   multiply)
//
// Interface which defines the method for interacting with an accelerator device
// (e.g. GPU, TPU).
//#[derive(Debug, Clone)]
pub trait StreamExecutor {
  // Returns a reference to the platform that created this executor.
  fn get_platform(&self) -> &dyn Platform;

  // Initializes the device for use.
  fn init(&self);

  // Returns the device ordinal.
  fn device_ordinal(&self) -> i64;

  // Creates and initializes a Stream.
  fn create_stream(&self, _priority: Option<(StreamPriority, i64)>) -> Result<Box<dyn Stream>, String>;

  fn create_stream_default(&self) -> Result<Box<dyn Stream>, String> {
    self.create_stream(None)
  }

  // Creates and initializes an Event.
  fn create_event(&self) -> Result<Box<dyn Event>, String>;

  // Obtains metadata about the underlying device.
  // The value is cached on first use.
  fn get_device_description(&self) -> &DeviceDescription;

  // Synchronously allocates an array on the device of type T with element_count
  // elements.
  fn allocate_array(&self, _element_count: usize, _memory_space: i64);

  // Convenience wrapper that allocates space for a single element of type T in
  // device memory.
  fn allocate_scalar(&self);

  // Loads a kernel from a MultiKernelLoaderSpec.
  //
  // Parameters:
  //   spec: The MultiKernelLoaderSpec is usually generated as a compile-time
  //    constant into an appropriate namespace.
  fn load_kernel(&self, _spec: &MultiKernelLoaderSpec) -> Result<Box<dyn Kernel>, String>;

  // Unloads the module with handle `module_handle`.
  fn unload_module(&self, _module_handle: &ModuleHandle) -> bool;

  // Loads a module for the platform this StreamExecutor is acting upon.
  //
  // `spec` describes the module to be loaded.  On success writes the handle for
  // the loaded module to `module_handle` and returns absl::OkStatus().
  // Otherwise, returns the error which has occurred.
  fn load_module(
    &self,
    _spec: &MultiModuleLoaderSpec,
    _module_handle: &ModuleHandle) -> Result<(), String>;

  // Creates a shared constant using the content provided.
  fn create_or_share_constant(
    &self, _stream: &dyn Stream, _content: &Vec<u8>) -> Result<DeviceMemoryBase, String>;

  // Synchronously allocates size bytes on the underlying platform and returns
  // a DeviceMemoryBase representing that allocation. In the case of failure,
  // nullptr is returned.
  fn allocate(&self, _size: u64, _memory_space: i64) -> DeviceMemoryBase;

  // Deallocates the DeviceMemory previously allocated via this interface.
  // Deallocation of a nullptr-representative value is permitted.
  fn deallocate(&self, _mem: &DeviceMemoryBase);

  // Allocates unified memory space of the given size, if supported.
  // See
  // https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#um-unified-memory-programming-hd
  // for more details on unified memory.
  fn unified_memory_allocate(&self, _size: i64);

  // Deallocates unified memory space previously allocated with
  // UnifiedMemoryAllocate.
  fn unified_memory_deallocate(&self);

  // Allocates collective device memory using ncclMemAlloc.
  // See
  // https://docs.nvidia.com/deeplearning/nccl/user-guide/docs/usage/bufferreg.html
  // for more details on User Buffer Registration.
  fn collective_memory_allocate(&self, _size: u64);

  // Deallocates collective device memory previously allocated with
  // CollectiveMemoryAllocate.
  fn collective_memory_deallocate(&self) -> Result<(), String>;

  // Allocates a region of host memory and registers it with the platform API.
  // Memory allocated in this manner is required for use in asynchronous memcpy
  // operations, such as Stream::Memcpy.
  fn host_memory_allocate(&self, _size: usize);

  // Deallocates a region of host memory allocated by HostMemoryAllocate().
  fn host_memory_deallocate(&self);

  // Returns the memory space of the given pointer.
  fn get_pointer_memory_space(&self);

  // Synchronizes all activity occurring in the StreamExecutor's context.
  fn synchronize_all_activity(&self) -> bool;

  // Blocks the caller while "size" bytes are zeroed out (in POD fashion) at the
  // given location in device memory.
  fn synchronous_mem_zero(
    &self, _location: &DeviceMemoryBase, _size: usize) -> Result<(), String>;

  // Blocks the caller while "size" bytes are copied to the given location in
  // device memory.
  fn synchronous_memcpy(
    &self, _device_dst: &DeviceMemoryBase, _size: usize) -> Result<(), String>;

  // Deallocates stream resources on the underlying platform.
  fn deallocate_stream(&self, _stream: &dyn Stream);

  // Causes the host code to synchronously wait for operations enqueued
  // onto stream to complete. Effectively a join on the asynchronous device
  // operations enqueued on the stream before this program point.
  fn block_host_until_done(&self, _stream: &dyn Stream) -> Result<(), String>;

  // Enables peer access from this StreamExecutor to memory
  // allocated by other, such that launched device code, memcpies, etc may
  // access it directly.
  fn enable_peer_access_to(&self, _other: &dyn StreamExecutor) -> Result<(), String>;

  // Returns whether it's possible to enable peer access from this
  // StreamExecutor to memory allocated by another.
  fn can_enable_peer_access_to(&self, _other: &dyn StreamExecutor) -> bool;

  // Returns the underlying device memory usage information, if it is available.
  // If it is not available (false is returned), free/total may not be
  // initialized.
  fn device_memory_usage(&self, _free: &i64, _total: &i64) -> bool;

  // Retrieves device pointer and size for a symbol. To use
  // constant memory in CUDA, GetSymbol has to be used. Returns DeviceMemoryBase
  // describing the symbol in memory if symbol is found.
  //
  // If ModuleHandle is set then we search for `symbol_name` only within the
  // module corresponding to `module_handle`.  Otherwise all loaded modules are
  // searched.
  fn get_symbol(
    &self,
    _symbol_name: &String,
    _module_handle: &ModuleHandle) -> Result<DeviceMemoryBase, String>;

  // Creates a new DeviceDescription object. Ownership is transferred to the
  // caller.
  fn create_device_description(&self) -> Result<DeviceDescription, String>;

  // Gets-or-creates a BlasSupport datatype that can be used to execute BLAS
  // routines on the current platform.
  //
  // Returns null if there was an error initializing the BLAS support for the
  // underlying platform.
  fn as_blas(self) -> Box<dyn BlasSupport>;

  // Gets or creates a FftSupport datatype that can be used to execute FFT
  // routines on the current platform.
  //
  // Returns null if there was an error initializing the FFT support for the
  // underlying platform.
  fn as_fft(&self) -> &FftSupport;

  // Gets-or-creates  a DnnSupport datatype that can be used for neural network
  // routines on the current platform.
  //
  // Returns null if there was an error initializing the DNN support for the
  // underlying platform.
  fn as_dnn(&self) -> &DnnSupport;

  // Creates a new CommandBuffer object.
  fn create_command_buffer(&self, _mode: CommandBufferMode) -> Result<CommandBuffer, String>;

  // Returns allocator statistics.
  fn get_allocator_stats(self) -> Option<AllocatorStats>;

  // Clears the internal stats except for the `in_use` fields  and sets the
  // `peak_bytes_in_use` to be equal to the `bytes_in_use`. Returns true if
  // implemented.
  fn clear_allocate_stats(&self) -> bool;

  // Clears the compilation cache from volatile memory. Returns OK if no
  // compilation cache exists or if clearing the compilation cache is
  // unsupported. Caches in non-volatile storage are unaffected.
  fn flush_compilation_cache(&self) -> Result<(), String>;

  // Returns a stream allocated by this executor, or nullptr if not found.
  fn find_allocated_stream(&self) -> &dyn Stream;

  // Returns the memory limit in bytes supported by this executor.
  fn get_memory_limit_bytes(&self) -> i64;

  // Sets the argument logging mode. Returns true if 'mode' is valid.
  // The mode is a bitmask of the kLog* constants.
  fn set_argument_logging_mode(&self, _mode: u64) -> bool;
}