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
#[derive(Debug, Clone)]
pub struct StreamExecutor {}

impl StreamExecutor {
  pub fn new(_device_ordinal: i64) -> Self {
    StreamExecutor {  }
  }

  // Returns a reference to the platform that created this executor.
  pub fn get_platform(&self) -> &Platform {
    unimplemented!()
  }

  // Initializes the device for use.
  pub fn init(&self) {
    unimplemented!()
  }

  // Returns the device ordinal.
  pub fn device_ordinal(&self) -> i64 {
    unimplemented!()
  }

  // Creates and initializes a Stream.
  pub fn create_stream(
    &self,
    _priority: Option<(StreamPriority, i64)>) -> Result<Stream, String>
  {
    unimplemented!()
  }

  pub fn create_stream_default(&self) -> Result<Stream, String> {
    self.create_stream(None)
  }

  // Creates and initializes an Event.
  pub fn create_event(&self) -> Result<Box<dyn Event>, String> {
    unimplemented!()
  }

  // Obtains metadata about the underlying device.
  // The value is cached on first use.
  pub fn get_device_description(&self) -> &DeviceDescription {
    unimplemented!()
  }

  // Synchronously allocates an array on the device of type T with element_count
  // elements.
  pub fn allocate_array(&self, _element_count: usize, _memory_space: i64) {
    unimplemented!()
  }

  // Convenience wrapper that allocates space for a single element of type T in
  // device memory.
  pub fn allocate_scalar(&self) {
    unimplemented!()
  }

  // Loads a kernel from a MultiKernelLoaderSpec.
  //
  // Parameters:
  //   spec: The MultiKernelLoaderSpec is usually generated as a compile-time
  //    constant into an appropriate namespace.
  pub fn load_kernel(
    &self,
    _spec: &MultiKernelLoaderSpec) -> Result<Box<dyn Kernel>, String>
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
  // the loaded module to `module_handle` and returns absl::OkStatus().
  // Otherwise, returns the error which has occurred.
  pub fn load_module(
    &self,
    _spec: &MultiModuleLoaderSpec,
    _module_handle: &ModuleHandle) -> Result<(), String>
  {
    unimplemented!()
  }

  // Creates a shared constant using the content provided.
  pub fn create_or_share_constant(
    &self,
    _stream: &Stream,
    _content: &Vec<u8>) -> Result<DeviceMemoryBase, String>
  {
    unimplemented!()
  }

  // Synchronously allocates size bytes on the underlying platform and returns
  // a DeviceMemoryBase representing that allocation. In the case of failure,
  // nullptr is returned.
  pub fn allocate(&self, _size: u64, _memory_space: i64) -> DeviceMemoryBase {
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
  pub fn unified_memory_allocate(&self, _size: i64) {
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
  pub fn collective_memory_allocate(&self, _size: u64) {
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
  pub fn host_memory_allocate(&self, _size: usize) {
    unimplemented!()
  }

  // Deallocates a region of host memory allocated by HostMemoryAllocate().
  pub fn host_memory_deallocate(&self) {
    unimplemented!()
  }

  // Returns the memory space of the given pointer.
  pub fn get_pointer_memory_space(&self) {
    unimplemented!()
  }

  // Synchronizes all activity occurring in the StreamExecutor's context.
  pub fn synchronize_all_activity(&self) -> bool {
    unimplemented!()
  }

  // Blocks the caller while "size" bytes are zeroed out (in POD fashion) at the
  // given location in device memory.
  pub fn synchronous_mem_zero(
    &self,
    _location: &DeviceMemoryBase,
    _size: usize) -> Result<(), String>
  {
    unimplemented!()
  }

  // Blocks the caller while "size" bytes are copied to the given location in
  // device memory.
  pub fn synchronous_memcpy(
    &self,
    _device_dst: &DeviceMemoryBase,
    _size: usize) -> Result<(), String>
  {
    unimplemented!()
  }

  // Deallocates stream resources on the underlying platform.
  pub fn deallocate_stream(&self, _stream: &Stream) {
    unimplemented!()
  }

  // Causes the host code to synchronously wait for operations enqueued
  // onto stream to complete. Effectively a join on the asynchronous device
  // operations enqueued on the stream before this program point.
  pub fn block_host_until_done(
    &self, _stream: &Stream) -> Result<(), String>
  {
    unimplemented!()
  }

  // Enables peer access from this StreamExecutor to memory
  // allocated by other, such that launched device code, memcpies, etc may
  // access it directly.
  pub fn enable_peer_access_to(
    &self, _other: &StreamExecutor) -> Result<(), String>
  {
    unimplemented!()
  }

  // Returns whether it's possible to enable peer access from this
  // StreamExecutor to memory allocated by another.
  pub fn can_enable_peer_access_to(
    &self, _other: &StreamExecutor) -> bool
  {
    unimplemented!()
  }

  // Returns the underlying device memory usage information, if it is available.
  // If it is not available (false is returned), free/total may not be
  // initialized.
  pub fn device_memory_usage(&self, _free: &i64, _total: &i64) -> bool {
    unimplemented!()
  }

  // Retrieves device pointer and size for a symbol. To use
  // constant memory in CUDA, GetSymbol has to be used. Returns DeviceMemoryBase
  // describing the symbol in memory if symbol is found.
  //
  // If ModuleHandle is set then we search for `symbol_name` only within the
  // module corresponding to `module_handle`.  Otherwise all loaded modules are
  // searched.
  pub fn get_symbol(
    &self,
    _symbol_name: &String,
    _module_handle: &ModuleHandle) -> Result<DeviceMemoryBase, String>
  {
    unimplemented!()
  }

  // Creates a new DeviceDescription object. Ownership is transferred to the
  // caller.
  pub fn create_device_description(&self) -> Result<DeviceDescription, String> {
    unimplemented!()
  }

  // Gets-or-creates a BlasSupport datatype that can be used to execute BLAS
  // routines on the current platform.
  //
  // Returns null if there was an error initializing the BLAS support for the
  // underlying platform.
  pub fn as_blas(self) -> Box<dyn BlasSupport> {
    unimplemented!()
  }

  // Gets or creates a FftSupport datatype that can be used to execute FFT
  // routines on the current platform.
  //
  // Returns null if there was an error initializing the FFT support for the
  // underlying platform.
  pub fn as_fft(&self) -> &FftSupport {
    unimplemented!()
  }

  // Gets-or-creates  a DnnSupport datatype that can be used for neural network
  // routines on the current platform.
  //
  // Returns null if there was an error initializing the DNN support for the
  // underlying platform.
  pub fn as_dnn(&self) -> &DnnSupport {
    unimplemented!()
  }

  // Creates a new CommandBuffer object.
  pub fn create_command_buffer(
    &self, _mode: CommandBufferMode) -> Result<CommandBuffer, String>
  {
    unimplemented!()
  }

  // Returns allocator statistics.
  pub fn get_allocator_stats(self) -> Option<AllocatorStats> {
    unimplemented!()
  }

  // Clears the internal stats except for the `in_use` fields  and sets the
  // `peak_bytes_in_use` to be equal to the `bytes_in_use`. Returns true if
  // implemented.
  pub fn clear_allocate_stats(&self) -> bool {
    unimplemented!()
  }

  // Clears the compilation cache from volatile memory. Returns OK if no
  // compilation cache exists or if clearing the compilation cache is
  // unsupported. Caches in non-volatile storage are unaffected.
  pub fn flush_compilation_cache(&self) -> Result<(), String> {
    unimplemented!()
  }

  // Returns a stream allocated by this executor, or nullptr if not found.
  pub fn find_allocated_stream(&self) -> &Stream {
    unimplemented!()
  }

  // Returns the memory limit in bytes supported by this executor.
  pub fn get_memory_limit_bytes(&self) -> i64 {
    unimplemented!()
  }

  // Sets the argument logging mode. Returns true if 'mode' is valid.
  // The mode is a bitmask of the kLog* constants.
  pub fn set_argument_logging_mode(&self, _mode: u64) -> bool {
    unimplemented!()
  }
}