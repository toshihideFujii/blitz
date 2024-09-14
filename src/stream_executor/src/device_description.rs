#![allow(dead_code)]

use crate::launch_dim::{BlockDim, ThreadDim};

pub enum CudaComputeCapabilities {
  Pascal,
  Volta,
  Ampere,
  Hopper,
}


// CUDA compute capability, as reported by the device description.
pub struct CudaComputeCapability {
  major: i64,
  minor: i64,
}

impl CudaComputeCapability {
  pub fn new() {}

  pub fn is_at_least() {}
  pub fn is_at_least_volta() {}
  pub fn is_at_least_ampere() {}
  pub fn is_at_least_hopper() {}
  pub fn get_max_resident_blocks_per_sm() {}
  pub fn get_max_resident_warps_per_sm() {}
  pub fn to_string() {}
  pub fn to_pair() {}
}

// Data that describes the execution target of the StreamExecutor, in terms of
// implrtant logical parameters.
pub struct DeviceDescription {
  device_vendor: String,
  platform_version: String,
  driver_version: String,
  runtime_version: String,
  pci_bus_id: String,
  name: String,
  model_str: String,

  thread_dim_limit: ThreadDim,
  block_dim_limit: BlockDim,

  threads_per_core_limit: i64,
  threads_per_block_limit: i64,
  threads_per_warp: i64,

  registers_per_core_limit: i64,
  registers_per_block_limit: i64,

  device_address_bits: i64,
  device_memory_size: i64,
  l2_cache_size: i64,
  memory_bandwidth: i64,

  shared_memory_per_core: i64,
  shared_memory_per_block: i64,
  shared_memory_per_block_optin: i64,

  clock_rate_ghz: f64,
  //gpu_compute_capability

  numa_node: i64,
  core_count: i64,
  fpus_per_core: i64,
  ecc_enabled: bool,
}

impl DeviceDescription {
  pub fn new() {}

  // Returns the platform being run on.
  pub fn platform_version(&self) -> String {
    self.platform_version.clone()
  }

  // Returns the driver version interfacing with the underlying platform.
  pub fn driver_version(&self) -> String {
    self.driver_version.clone()
  }

  // Returns the runtime version, if one is provided by the underlying platkform.
  pub fn runtime_version(&self) -> String {
    self.runtime_version.clone()
  }

  // Returns the name that the device reports.
  pub fn name(&self) -> String {
    self.name.clone()
  }

  // Gets a human-readable description of the device.
  pub fn model_str(&self) -> String {
    self.model_str.clone()
  }

  // Returns the PCI bus identifier for this device.
  pub fn pci_bus_id(&self) -> String {
    self.pci_bus_id.clone()
  }

  // Returns the NUMA node associated with this device. 
  pub fn numa_node(&self) -> i64 {
    self.numa_node
  }

  // Number of core.
  pub fn core_count(&self) -> i64 {
    self.core_count
  }

  // Number of floating point operations one core can execute in parallel.
  pub fn fpus_per_core(&self) -> i64 {
    self.fpus_per_core
  }

  // Returns the limit on the thread dimensionality values in each of the
  // respective dimensions.
  pub fn thread_dim_limit(&self) -> &ThreadDim {
    &self.thread_dim_limit
  }

  // Returns the limit on the block dimensionality values in each of the 
  // respective dimensions.
  pub fn block_dim_limit(&self) -> &BlockDim {
    &self.block_dim_limit
  }

  // Returns the limit on the total number of threads that can be launched in a
  // single block.
  pub fn threads_per_block_limit(&self) -> i64 {
    self.threads_per_block_limit
  }

  // Returns the limit on the total number of threads that can be simultaneously
  // launched on a given multiprocessor.
  pub fn threads_per_core_limit(&self) -> i64 {
    self.threads_per_core_limit
  }

  // Returns the number of threads per warp/wavefront.
  pub fn threads_per_warp(&self) -> i64 {
    self.threads_per_warp
  }

  // Returns the limit on the total number of registers per core.
  pub fn registers_per_core_limit(&self) -> i64 {
    self.registers_per_core_limit
  }

  // Returns the limit on the total number or registers that can be simultaneously
  // used by a block.
  pub fn registers_per_block_limit(&self) -> i64 {
    self.registers_per_block_limit
  }

  // Returns the number of address bits available to kernel code running on the
  // platform.
  pub fn device_address_bits(&self) -> i64 {
    self.device_address_bits
  }

  // Returns the device memory size in bytes.
  pub fn device_memory_size(&self) -> i64 {
    self.device_memory_size
  }

  // Returns the L2 cache size in bytes.
  pub fn l2_cache_size(&self) -> i64 {
    self.l2_cache_size
  }

  // Returns the device's memory bandwidth in bytes/sec.
  pub fn memory_bandwidth(&self) -> i64 {
    self.memory_bandwidth
  }

  // Returns the device's core clock rate in GHz.
  pub fn clock_rate_ghz(&self) -> f64 {
    self.clock_rate_ghz
  }

  // Returns twhether ECC is enabled.
  pub fn ecc_enabled(&self) -> bool {
    self.ecc_enabled
  }

  // Returns the device vendor string.
  pub fn device_vendor(&self) -> String {
    self.device_vendor.clone()
  }

  pub fn cuda_compute_capability() {}
  pub fn rocm_compute_capability() {}
  pub fn gpu_compute_capability() {}

  // Returns the maximum amount of shared memory present on a single core.
  pub fn shared_memory_per_core(&self) -> i64 {
    self.shared_memory_per_core
  }

  // Returns the maximum amount of static shared memory available for a  single block.
  pub fn shared_memory_per_block(&self) -> i64 {
    self.shared_memory_per_block
  }

  // Returns the maximum amount of static shared memory available for a  single block
  // including the dynamically allocated one.
  pub fn shared_memory_per_block_optin(&self) -> i64 {
    self.shared_memory_per_block_optin
  }
}

pub struct DeviceDescriptionBuilder {
  device_description: DeviceDescription
}

impl DeviceDescriptionBuilder {
  pub fn set_gpu_compute_capability() {}

  pub fn set_block_dim_limit_x(&mut self, limit: u64) {
    self.device_description.block_dim_limit.dim_3d.x = limit;
  }

  pub fn set_block_dim_limit_y(&mut self, limit: u64) {
    self.device_description.block_dim_limit.dim_3d.y = limit;
  }

  pub fn set_block_dim_limit_z(&mut self, limit: u64) {
    self.device_description.block_dim_limit.dim_3d.z = limit;
  }

  pub fn set_device_vendor(&mut self, value: String) {
    self.device_description.device_vendor = value;
  }

  pub fn set_platform_version(&mut self, value: String) {
    self.device_description.platform_version = value;
  }

  pub fn set_driver_version(&mut self, value: String) {
    self.device_description.driver_version = value;
  }

  pub fn set_runtime_version(&mut self, value: String) {
    self.device_description.runtime_version = value;
  }

  pub fn set_pci_bus_id(&mut self, value: String) {
    self.device_description.pci_bus_id = value;
  }

  pub fn set_name(&mut self, value: String) {
    self.device_description.name = value;
  }

  pub fn set_model_str(&mut self, value: String) {
    self.device_description.model_str = value;
  }

  pub fn set_thread_dim_limit(&mut self, value: ThreadDim) {
    self.device_description.thread_dim_limit = value;
  }

  pub fn set_block_dim_limit(&mut self, value: BlockDim) {
    self.device_description.block_dim_limit = value;
  }

  pub fn set_threads_per_core_limit(&mut self, value: i64) {
    self.device_description.threads_per_core_limit = value;
  }

  pub fn set_threads_per_block_limit(&mut self, value: i64) {
    self.device_description.threads_per_block_limit = value;
  }

  pub fn set_threads_per_warp(&mut self, value: i64) {
    self.device_description.threads_per_warp = value;
  }

  pub fn set_registers_per_core_limit(&mut self, value: i64) {
    self.device_description.registers_per_core_limit = value;
  }

  pub fn set_registers_per_block_limit(&mut self, value: i64) {
    self.device_description.registers_per_block_limit = value;
  }

  pub fn set_device_address_bits(&mut self, value: i64) {
    self.device_description.device_address_bits = value;
  }

  pub fn set_device_memory_size(&mut self, value: i64) {
    self.device_description.device_memory_size = value;
  }

  pub fn set_l2_cache_size(&mut self, value: i64) {
    self.device_description.l2_cache_size = value;
  }

  pub fn set_memory_bandwidth(&mut self, value: i64) {
    self.device_description.memory_bandwidth = value;
  }

  pub fn set_shared_memory_per_core(&mut self, value: i64) {
    self.device_description.shared_memory_per_core = value;
  }

  pub fn set_shared_memory_per_block(&mut self, value: i64) {
    self.device_description.shared_memory_per_block = value;
  }

  pub fn set_shared_memory_per_block_optin(&mut self, value: i64) {
    self.device_description.shared_memory_per_block_optin = value;
  }

  pub fn set_clock_rate_ghz(&mut self, value: f64) {
    self.device_description.clock_rate_ghz = value;
  }

  pub fn set_cuda_compute_capability() {}
  pub fn set_rocm_compute_capability() {}

  pub fn set_numa_node(&mut self, value: i64) {
    self.device_description.numa_node = value;
  }

  pub fn set_core_count(&mut self, value: i64) {
    self.device_description.core_count = value;
  }

  pub fn set_fpus_per_core(&mut self, value: i64) {
    self.device_description.fpus_per_core = value;
  }

  pub fn set_ecc_enabled(&mut self, value: bool) {
    self.device_description.ecc_enabled = value;
  }

  pub fn build() {}

  pub fn build_object(&self) -> &DeviceDescription {
    &self.device_description
  }
}