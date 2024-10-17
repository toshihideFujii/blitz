#![allow(dead_code)]

use crate::{
  allocator_stats::AllocatorStats,
  blas::BlasSupport,
  command_buffer::{CommandBuffer, CommandBufferMode},
  device_description::DeviceDescription,
  device_memory::DeviceMemoryBase,
  kernel::Kernel,
  kernel_spec::MultiKernelLoaderSpec,
  module_spec::{ModuleHandle, MultiModuleLoaderSpec},
  platform::{Platform, StreamPriority},
  stream::Stream,
  stream_executor::StreamExecutor
};

use super::host_platform::HostPlatform;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostExecutor {
  device_ordinal: i64
}

impl HostExecutor {
  pub fn default(device_ordinal: i64) -> Self {
    HostExecutor { device_ordinal: device_ordinal }
  }

  pub fn new(_platform: &HostPlatform, _device_ordinal: i64) -> Self {
    unimplemented!()
  }

  pub fn init(&self) -> Result<(), String> {
    unimplemented!()
  }

  pub fn create_device_description(&self) -> Result<DeviceDescription, String> {
    HostExecutor::create_device_description_by_ordinal(0)
  }

  pub fn create_device_description_by_ordinal(
    _device_ordinal: i64) -> Result<DeviceDescription, String>
  {
    unimplemented!()
  }
}

impl StreamExecutor for HostExecutor {
  fn get_platform(&self) -> &dyn Platform {
    unimplemented!()
  }

  fn device_ordinal(&self) -> i64 {
    self.device_ordinal
  }

  fn allocate(
    &self,
    _size: u64,
    _memory_space: i64) -> DeviceMemoryBase
  {
    unimplemented!()
  }

  fn allocate_array(&self, _element_count: usize, _memory_space: i64) {
    unimplemented!()
  }

  fn allocate_scalar(&self) {
    unimplemented!()
  }

  fn as_blas(self) -> Box<dyn BlasSupport> {
    unimplemented!()
  }

  fn as_dnn(&self) -> &crate::dnn::DnnSupport {
    unimplemented!()
  }

  fn as_fft(&self) -> &crate::fft::FftSupport {
    unimplemented!()
  }

  fn block_host_until_done(
    &self,
    _stream: &dyn crate::stream::Stream) -> Result<(), String>
  {
    unimplemented!()
  }

  fn can_enable_peer_access_to(&self, _other: &dyn StreamExecutor) -> bool {
    unimplemented!()
  }

  fn clear_allocate_stats(&self) -> bool {
    unimplemented!()
  }

  fn collective_memory_allocate(&self, _size: u64) {
    unimplemented!()
  }

  fn collective_memory_deallocate(&self) -> Result<(), String> {
    unimplemented!()
  }

  fn create_command_buffer(
    &self,
    _mode: CommandBufferMode) -> Result<CommandBuffer, String>
  {
    unimplemented!()
  }

  fn create_device_description(&self) -> Result<DeviceDescription, String> {
    unimplemented!()
  }

  fn create_event(&self) -> Result<Box<dyn crate::event::Event>, String> {
    unimplemented!()
  }

  fn create_or_share_constant(
    &self,
    _stream: &dyn Stream,
    _content: &Vec<u8>) -> Result<DeviceMemoryBase, String>
  {
     unimplemented!()
  }

  fn create_stream(
    &self,
    _priority: Option<(StreamPriority, i64)>) -> Result<Box<dyn Stream>, String>
  {
    unimplemented!()
  }

  fn create_stream_default(&self) -> Result<Box<dyn Stream>, String> {
    unimplemented!()
  }

  fn deallocate(&self, _mem: &DeviceMemoryBase) {
    unimplemented!()
  }

  fn deallocate_stream(&self, _stream: &dyn Stream) {
    unimplemented!()
  }

  fn device_memory_usage(&self, _free: &i64, _total: &i64) -> bool {
    unimplemented!()
  }

  fn enable_peer_access_to(&self, _other: &dyn StreamExecutor) -> Result<(), String> {
    unimplemented!()
  }

  fn find_allocated_stream(&self) -> &dyn Stream {
    unimplemented!()
  }

  fn flush_compilation_cache(&self) -> Result<(), String> {
    unimplemented!()
  }

  fn get_allocator_stats(self) -> Option<AllocatorStats> {
    unimplemented!()
  }

  fn get_device_description(&self) -> &DeviceDescription {
    unimplemented!()
  }

  fn get_memory_limit_bytes(&self) -> i64 {
    unimplemented!()
  }

  fn get_pointer_memory_space(&self) {
    unimplemented!()
  }

  fn get_symbol(
    &self,
    _symbol_name: &String,
    _module_handle: &ModuleHandle) -> Result<DeviceMemoryBase, String>
  {
    unimplemented!()
  }

  fn host_memory_allocate(&self, _size: usize) {
    unimplemented!()
  }

  fn host_memory_deallocate(&self) {
    unimplemented!()
  }

  fn init(&self) {
    unimplemented!()
  }

  fn load_kernel(&self, _spec: &MultiKernelLoaderSpec) -> Result<Box<dyn Kernel>, String> {
    unimplemented!()
  }

  fn load_module(
    &self,
    _spec: &MultiModuleLoaderSpec,
    _module_handle: &ModuleHandle) -> Result<(), String>
  {
    unimplemented!()
  }

  fn set_argument_logging_mode(&self, _mode: u64) -> bool {
    unimplemented!()
  }

  fn synchronize_all_activity(&self) -> bool {
    unimplemented!()
  }

  fn synchronous_mem_zero(
    &self, _location: &DeviceMemoryBase, _size: usize) -> Result<(), String>
  {
    unimplemented!()
  }

  fn synchronous_memcpy(
    &self, _device_dst: &DeviceMemoryBase, _size: usize) -> Result<(), String>
  {
    unimplemented!()
  }

  fn unified_memory_allocate(&self, _size: i64) {
    unimplemented!()
  }

  fn unified_memory_deallocate(&self) {
    unimplemented!()
  }

  fn unload_module(&self, _module_handle: &ModuleHandle) -> bool {
    unimplemented!()
  }
}