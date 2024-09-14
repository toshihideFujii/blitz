#![allow(dead_code)]

use crate::{device_memory::DeviceMemoryBase, stream_executor::StreamExecutor};

pub enum KernelCacheConfig {
  NoPreference,
  PreferShared,
  PreferL1,
  PreferEqual,
}

// KernelMetadata holds runtime-queryable attributes of a loaded kernel,
// such as registers allocated, shared memory used, etc.
pub struct KernelMetadata {
  registers_per_thread: Option<i64>,
  shared_memory_bytes: Option<i64>,
}

impl KernelMetadata {
  pub fn new() -> Self {
    KernelMetadata { registers_per_thread: None, shared_memory_bytes: None }
  }

  // Returns the number of registers used per thread executing the kernel.
  pub fn registers_per_thread(&self) -> &Option<i64> {
    &self.registers_per_thread
  }

  // Returns the amount of [static] shared memory used per block executing
  // this kernel.
  pub fn shared_memory_bytes(&self) -> &Option<i64> {
    &self.shared_memory_bytes
  }

  pub fn set_registers_per_thread(&mut self, registers_per_thread: i64) {
    self.registers_per_thread = Some(registers_per_thread);
  }

  pub fn set_shared_memory_bytes(&mut self, shared_memory_bytes: i64) {
    self.shared_memory_bytes = Some(shared_memory_bytes);
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KernelArgsKind {
  DeviceMemoryArray,
  PackedArray,
}

pub struct KernelArgs {
  device_memory_array: Option<KernelArgsDeviceMemoryArray>,
  packed_array: Option<KernelArgsPackedArray>,
}

impl KernelArgs {
  pub fn new_device_memory_array(
    device_memory_args: Vec<DeviceMemoryBase>,
    shared_memory_bytes: usize) -> Self
  {
    KernelArgs {
      device_memory_array:
        Some(KernelArgsDeviceMemoryArray::new(device_memory_args, shared_memory_bytes)),
      packed_array: None
    }
  }

  pub fn kind(&self) -> KernelArgsKind {
    if self.device_memory_array.is_some() {
      self.device_memory_array.as_ref().unwrap().kind()
    } else {
      self.packed_array.as_ref().unwrap().kind()
    }
  }

  pub fn number_of_arguments(&self) -> usize {
    if self.device_memory_array.is_some() {
      self.device_memory_array.as_ref().unwrap().number_of_arguments()
    } else {
      self.packed_array.as_ref().unwrap().number_of_arguments()
    }
  }

  pub fn number_of_shared_bytes() {}
}

pub struct KernelArgsPackedArray {
  shared_memory_bytes: usize,
  number_of_argument_addresses: usize,
}

impl KernelArgsPackedArray {
  pub fn new() {}

  pub fn kind(&self) -> KernelArgsKind {
    KernelArgsKind::PackedArray
  }

  pub fn add_argument() {}
  pub fn add_device_memory_argument() {}

  // Adds a shared memory argument to the list.
  pub fn add_shared_bytes(&mut self, number_of_bytes: usize) {
    self.shared_memory_bytes += number_of_bytes;
  }

  // Gets the number of arguments added so far, including shared memory arguments.
  pub fn number_of_arguments(&self) -> usize {
    if self.shared_memory_bytes > 0 {
      return self.number_of_argument_addresses + 1;
    }
    self.number_of_argument_addresses
  }

  // Gets the total number of shared memory bytes added so far.
  pub fn number_of_shared_bytes(&self) -> usize {
    self.shared_memory_bytes
  }
}

pub struct KernelArgsDeviceMemoryArray {
  device_memory_args: Vec<DeviceMemoryBase>,
  shared_memory_bytes: usize,
}

impl KernelArgsDeviceMemoryArray {
  pub fn new(
    device_memory_args: Vec<DeviceMemoryBase>,
    shared_memory_bytes: usize) -> Self
  {
    KernelArgsDeviceMemoryArray {
      device_memory_args: device_memory_args,
      shared_memory_bytes: shared_memory_bytes,
    }
  }

  pub fn kind(&self) -> KernelArgsKind {
    KernelArgsKind::DeviceMemoryArray
  }

  pub fn number_of_arguments(&self) -> usize {
    if self.shared_memory_bytes > 0 {
      return self.device_memory_args.len() + 1;
    }
    self.device_memory_args.len()
  }

  pub fn number_of_shared_bytes(&self) -> usize {
    self.shared_memory_bytes
  }

  pub fn device_memory_args(&self) -> &Vec<DeviceMemoryBase> {
    &self.device_memory_args
  }

  pub fn device_memory_ptr() {}

  pub fn device_memory_size(&self, index: usize) -> usize {
    self.device_memory_args[index].size()
  }
}

pub struct Kernel {
  parent: Box<dyn StreamExecutor>,
  name: String,
  demangle_name: String,
  metadata: KernelMetadata
}

impl Kernel {
  pub fn new() {}
  pub fn arity() {}
  pub fn parent() {}
  pub fn implementation() {}
  pub fn metadata() {}
  pub fn set_metadata() {}
  pub fn set_preferred_cache_config() {}
  pub fn get_preferred_cache_config() {}
  pub fn get_max_occupied_blocks_per_core() {}
  pub fn set_kernel_args_packing() {}
  pub fn kernel_args_packing() {}
  pub fn name() {}
  pub fn set_name() {}
  pub fn demangled_name() {}
}