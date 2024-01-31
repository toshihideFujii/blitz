#![allow(dead_code)]

use common::shape::Shape;

pub struct ExecutableBuildOptions {
  device_ordinal: i64,
  result_layout: Shape,
  result_layout_set: bool,
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
  fdo_profile: String,
  device_memory_size: i64,
}

impl ExecutableBuildOptions {
  pub fn new() {}

  pub fn set_device_ordinal(&mut self, device_ordinal: i64) {
    self.device_ordinal = device_ordinal;
  }

  pub fn device_ordinal(&self) -> i64 {
    self.device_ordinal
  }

  pub fn set_result_layout(&mut self, shape_with_layout: Shape) {
    self.result_layout = shape_with_layout;
  }

  pub fn result_layout(&self) -> &Shape {
    &self.result_layout
  }

  pub fn has_comp_envs() {}
  pub fn comp_envs() {}
  pub fn has_debug_options() {}
  pub fn debug_options() {}
  pub fn set_device_allocator() {}
  pub fn device_allocator() {}

  pub fn num_replicas(&self) -> i64 {
    self.num_replicas
  }

  pub fn set_num_replicas(&mut self, num_replicas: i64) {
    self.num_replicas = num_replicas;
  }

  pub fn num_partitions(&self) -> i64 {
    self.num_partitions
  }

  pub fn set_num_partitions(&mut self, num_partitions: i64) {
    self.num_partitions = num_partitions;
  }

  pub fn use_spmd_partitioning(&self) -> bool {
    self.use_spmd_partitioning
  }

  pub fn set_use_spmd_partitioning(&mut self, use_spmd_partitioning: bool) {
    self.use_spmd_partitioning = use_spmd_partitioning;
  }

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

  pub fn allow_spmd_sharding_propagation_to_output() {}
  pub fn any_allow_spmd_sharding_propagation_to_output() {}
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