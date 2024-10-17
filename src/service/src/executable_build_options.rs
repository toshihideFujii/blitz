#![allow(dead_code)]

use common::{
  //blitz_data::DebugOptions,
  shape::Shape
};

//use stream_executor::device_memory_allocator::DeviceMemoryAllocator;

use crate::compilation_environments::CompilationEnvironments;

// Class containing options for building an LocalExecutable with
// LocalClient::Compile.
#[derive(Debug, Clone)]
pub struct ExecutableBuildOptions {
  device_ordinal: i64,
  result_layout: Shape,
  result_layout_set: bool,
  comp_envs: Option<CompilationEnvironments>,
  //debug_options: Option<DebugOptions>,
  //device_allocator: Option<DeviceMemoryAllocator>,
  num_replicas: i64,
  num_partitions: i64,
  use_spmd_partitioning: bool,
  use_auto_spmd_partitioning: bool,
  auto_spmd_partitioning_mesh_shape: Vec<i64>,
  auto_spmd_partitioning_mesh_ids: Vec<i64>,
  duplicate_hlo: bool,
  broadcast_replicated_params: bool,
  alias_passthrough_params: bool,
  run_backend_only: bool,
  allow_spmd_sharding_propagation_to_parameters: Vec<bool>,
  allow_spmd_sharding_propagation_to_output: Vec<bool>,
  fdo_profile: String,
  device_memory_size: i64,
}

impl ExecutableBuildOptions {
  pub fn new() {}

  // If set, this is the device to build the computation for. Valid
  // device_ordinal values are: 0 to # of devices - 1. These values are
  // identical to the device ordinal values used by StreamExecutor. The built
  // executable will be executable on any device equivalent to the specified
  // device as determined by Backend::devices_equivalent(). A value of -1
  // indicates this option has not been set.
  pub fn set_device_ordinal(&mut self, device_ordinal: i64) {
    self.device_ordinal = device_ordinal;
  }

  pub fn device_ordinal(&self) -> i64 {
    self.device_ordinal
  }

  // If set, this specifies the layout of the result of the computation. If not
  // set, the service will chose the layout of the result. A Shape is used to
  // store the layout to accommodate tuple result shapes. A value of nullptr
  // indicates the option has not been set.
  pub fn set_result_layout(&mut self, shape_with_layout: Shape) {
    self.result_layout = shape_with_layout;
  }

  pub fn result_layout(&self) -> &Shape {
    &self.result_layout
  }

  // Expose access to the Blitz compilation environments, which will be passed to
  // the compilation process. `comp_envs()` must not be called if
  // `has_comp_envs()` returns false.
  pub fn has_comp_envs(&self) -> bool {
    self.comp_envs.is_some()
  }

  pub fn comp_envs(&self) -> &Option<CompilationEnvironments> {
    &self.comp_envs
  }

  pub fn mutable_comp_envs(&mut self) -> &mut Option<CompilationEnvironments> {
    &mut self.comp_envs
  }

  // Expose access to the XLA debug options which will be passed to the
  // compilation process. `debug_options()` must not be called if
  // `has_debug_options()` returns false.
  pub fn has_debug_options() {}
  pub fn debug_options() {}
  pub fn mutable_debug_options() {}

  // If set, this specifies an allocator that can be used to allocate temporary
  // space on the device during compilation.  For example, the compiler might
  // want to run various algorithms on the device and pick the fastest one -- it
  // might allocate buffers for use by these algorithms using this allocator.
  //
  // This does not need to be the same as the se::DeviceMemoryAllocator passed
  // when running the executable.
  pub fn set_device_allocator() {}
  pub fn device_allocator() {}

  // The number of replicas of this computation that are to be executed.
  // Defaults to 1.
  pub fn num_replicas(&self) -> i64 {
    self.num_replicas
  }

  pub fn set_num_replicas(&mut self, num_replicas: i64) {
    self.num_replicas = num_replicas;
  }

  // The number of partitions in this computation. Defaults to 1.
  pub fn num_partitions(&self) -> i64 {
    self.num_partitions
  }

  pub fn set_num_partitions(&mut self, num_partitions: i64) {
    self.num_partitions = num_partitions;
  }

  // Indicates whether to use SPMD (true) or MPMD (false) partitioning when
  // num_partitions > 1 and XLA is requested to partition the input program.
  pub fn use_spmd_partitioning(&self) -> bool {
    self.use_spmd_partitioning
  }

  pub fn set_use_spmd_partitioning(&mut self, use_spmd_partitioning: bool) {
    self.use_spmd_partitioning = use_spmd_partitioning;
  }

  // Whether to automatically generate XLA shardings for SPMD partitioner.
  pub fn use_auto_spmd_partitioning(&self) -> bool {
    self.use_auto_spmd_partitioning
  }

  pub fn set_use_auto_spmd_partitioing(&mut self, use_auto_spmd_partitioning: bool) {
    self.use_auto_spmd_partitioning = use_auto_spmd_partitioning;
  }

  pub fn auto_spmd_partitioning_mesh_shape(&self) -> &Vec<i64> {
    &self.auto_spmd_partitioning_mesh_shape
  }

  pub fn set_auto_spmd_partitioning_mesh_shape(&mut self, mesh_shape: Vec<i64>) {
    self.auto_spmd_partitioning_mesh_shape = mesh_shape;
  }

  pub fn auto_spmd_partitioning_mesh_ids(&self) -> &Vec<i64> {
    &self.auto_spmd_partitioning_mesh_ids
  }

  pub fn set_auto_spmd_partitioning_mesh_ids(&mut self, mesh_ids: Vec<i64>) {
    self.auto_spmd_partitioning_mesh_ids = mesh_ids;
  }

  pub fn duplicate_hlo(&self) -> bool {
    self.duplicate_hlo
  }

  pub fn set_duplicate_hlo(&mut self, duplicate_hlo: bool) {
    self.duplicate_hlo = duplicate_hlo;
  }

  pub fn has_device_assignment() {}

  pub fn alias_passthrough_params(&self) -> bool {
    self.alias_passthrough_params
  }

  pub fn set_alias_passthrough_params(&mut self, alias_passthrough_params: bool) {
    self.alias_passthrough_params = alias_passthrough_params;
  }

  pub fn run_backend_only(&self) -> bool {
    self.run_backend_only
  }

  pub fn set_run_backend_only(&mut self, run_backend_only: bool) {
    self.run_backend_only = run_backend_only;
  }

  pub fn allow_spmd_sharding_propagation_to_parameters(&self) -> &Vec<bool> {
    &self.allow_spmd_sharding_propagation_to_parameters
  }

  pub fn allow_spmd_sharding_propagation_to_output(&self) -> &Vec<bool> {
    &self.allow_spmd_sharding_propagation_to_output
  }

  pub fn any_allow_spmd_sharding_propagation_to_parameters(&self) -> bool {
    for param in &self.allow_spmd_sharding_propagation_to_parameters {
      if *param == true { return true; }
    }
    false
  }

  pub fn any_allow_spmd_sharding_propagation_to_output(&self) -> bool {
    for output in &self.allow_spmd_sharding_propagation_to_output {
      if *output == true { return true; }
    }
    false
  }
  pub fn set_allow_spmd_sharding_propagation_to_output() {}
  pub fn compile_thread_pool() {}
  pub fn set_compile_thread_pool() {}
  pub fn set_layout_cannonicalization_callback() {}

  pub fn fdo_profile(&self) -> String {
    self.fdo_profile.clone()
  }

  pub fn set_fdo_profile(&mut self, fdo_profile: String) {
    self.fdo_profile = fdo_profile;
  }

  pub fn device_memory_size(&self) -> i64 {
    self.device_memory_size
  }

  pub fn set_device_memory_size(&mut self, device_memory_size: i64) {
    self.device_memory_size = device_memory_size;
  }
  
  pub fn to_string() {}
  pub fn to_proto() {}
}