
#![allow(dead_code)]

use std::collections::HashMap;
use common::blitz_data::Precision;

#[derive(Clone, PartialEq)]
pub enum FusionConfigCollection {
  Off,
  PerEdge,
  PerNode,
}

pub struct HloModuleConfig {
  seed: u64,
  launch_id: i32,
  replica_count: i64,
  num_partitions: i64,
  param_requires_broadcast_via_collectives: Vec<bool>,
  use_spmd_partitioning: bool,
  use_auto_spmd_partitioning: bool,
  auto_spmd_partitioning_mesh_shape: Vec<i64>,
  auto_spmd_partitioning_mesh_ids: Vec<i64>,
  deduplicate_hlo: bool,
  intra_op_parallelism_threads: i64,
  device_type: String,
  allow_separate_sharding_programs: bool,
  alias_passthrough_params: bool,
  content_aware_computation_sorting: bool,
  fusion_config: Vec<Vec<bool>>,
  layout_config: Vec<Vec<Vec<i64>>>,
  memory_space_assignment_config: Vec<u64>,
  phase_ordering_config: Vec<Vec<bool>>,
  phase_index: i64,
  analysis_allowance_map: HashMap<String, i64>,
  matrix_unit_operand_precision: Precision,
  fdo_profile: String,
  device_memory_size: i64,
}

impl HloModuleConfig {
  pub fn new() {}
  pub fn to_proto() {}
  pub fn assign_proto_shardable_value_update_pairs() {}
  pub fn has_entry_computation_layout() {}
  pub fn set_default_computation_layout() {}
  pub fn set_computation_layout_if_exists() {}
  pub fn entry_computation_layout() {}
  pub fn clear_entry_computation_layout() {}
  pub fn hlo_profiling_enabled() {}
  pub fn cpu_traceme_enabled() {}

  pub fn set_seed(&mut self, seed: u64) {
    self.seed = seed;
  }

  pub fn seed(&self) -> u64 {
    self.seed
  }

  pub fn set_launch_id(&mut self, launch_id: i32) {
    self.launch_id = launch_id;
  }

  pub fn launch_id(&self) -> i32 {
    self.launch_id
  }

  pub fn set_replica_count(&mut self, replica_count: i64) {
    self.replica_count = replica_count;
  }

  pub fn replica_count(&self) -> i64 {
    self.replica_count
  }

  pub fn set_num_partitions(&mut self, num_partitions: i64) {
    self.num_partitions = num_partitions;
  }

  pub fn num_partitions(&self) -> i64 {
    self.num_partitions
  }

  pub fn param_requires_broadcast_via_collectives(&self) -> &Vec<bool> {
    &self.param_requires_broadcast_via_collectives
  }

  pub fn set_param_requires_broadcast_via_collectives(&mut self, require_broadcast: Vec<bool>) {
    self.param_requires_broadcast_via_collectives = require_broadcast;
  }

  pub fn set_use_spmd_partitioning(&mut self, use_spmd_partitioning: bool) {
    self.use_spmd_partitioning = use_spmd_partitioning;
  }

  pub fn use_spmd_partitioning(&self) -> bool {
    self.use_spmd_partitioning
  }

  pub fn set_use_auto_spmd_partitioning(&mut self, use_auto_spmd_partitioning: bool) {
    self.use_auto_spmd_partitioning = use_auto_spmd_partitioning;
  }

  pub fn use_auto_spmd_partitioning(&self) -> bool {
    self.use_auto_spmd_partitioning
  }

  pub fn set_auto_spmd_partitioning_mesh_shape(&mut self, mesh_ahape: Vec<i64>) {
    self.auto_spmd_partitioning_mesh_shape = mesh_ahape;
  }

  pub fn auto_spmd_partitioning_mesh_shape(&self) -> &Vec<i64> {
    &self.auto_spmd_partitioning_mesh_shape
  }

  pub fn set_auto_spmd_partitioning_mesh_ids(&mut self, mesh_ids: Vec<i64>) {
    self.auto_spmd_partitioning_mesh_ids = mesh_ids;
  }

  pub fn auto_spmd_partitioning_mesh_ids(&self) -> &Vec<i64> {
    &self.auto_spmd_partitioning_mesh_ids
  }

  pub fn set_deduplicate_hlo(&mut self, deduplicate_hlo: bool) {
    self.deduplicate_hlo = deduplicate_hlo;
  }

  pub fn deduplicate_hlo(&self) -> bool {
    self.deduplicate_hlo
  }

  pub fn set_device_type(&mut self, device_tytpe: String) {
    self.device_type = device_tytpe;
  }

  pub fn device_type(&self) -> String {
    self.device_type.clone()
  }

  pub fn compilation_cache_key() {}
  pub fn debug_options() {}
  pub fn set_debug_options() {}

  pub fn set_intra_op_parallelism_threads(&mut self, intra_op_parallelism_threads: i64) {
    self.intra_op_parallelism_threads = intra_op_parallelism_threads;
  }

  pub fn intra_op_parallelism_threads(&self) -> i64 {
    self.intra_op_parallelism_threads
  }

  pub fn has_static_device_assignment() {}
  pub fn static_device_assignment() {}
  pub fn set_static_device_assignment() {}

  pub fn allow_separate_sharding_programs(&self) -> bool {
    self.allow_separate_sharding_programs
  }

  pub fn set_allow_separate_sharding_programs(&mut self, allow_separate_sharding_programs: bool) {
    self.allow_separate_sharding_programs = allow_separate_sharding_programs;
  }

  pub fn shardable_value_update_pairs() {}
  pub fn set_shardable_value_update_pairs() {}

  pub fn alias_passthrough_params(&self) -> bool {
    self.alias_passthrough_params
  }

  pub fn set_alias_passthrough_params(&mut self, alias_passthrough_params: bool) {
    self.alias_passthrough_params = alias_passthrough_params;
  }

  pub fn content_aware_computation_sorting(&self) -> bool {
    self.content_aware_computation_sorting
  }

  pub fn set_content_aware_computation_sorting(&mut self, content_aware_aomputation_sorting: bool) {
    self.content_aware_computation_sorting = content_aware_aomputation_sorting;
  }

  pub fn fusion_config_collection() {}
  pub fn set_fusion_config_collection() {}

  pub fn fusion_config(&self) -> &Vec<Vec<bool>> {
    &self.fusion_config
  }

  pub fn dot_config() {}

  pub fn layout_config(&self) -> &Vec<Vec<Vec<i64>>> {
    &self.layout_config
  }

  pub fn phase_ordering_config(&self) -> &Vec<Vec<bool>> {
    &self.phase_ordering_config
  }

  pub fn phase_index(&self) -> i64 {
    self.phase_index
  }

  pub fn set_phase_index(&mut self, phase_index: i64) {
    self.phase_index = phase_index;
  }

  pub fn allow_spmd_sharding_propagation_to_output() {}
  pub fn set_allow_spmd_sharding_propagation_to_output() {}

  pub fn memory_space_assignment_config(&self) -> &Vec<u64> {
    &self.memory_space_assignment_config
  }

  pub fn get_analysis_allowance(&self, pass_name: String) -> Option<&i64> {
    self.analysis_allowance_map.get(&pass_name)
  }

  pub fn set_analysis_allowance(&mut self, pass_name: String, allowance: i64) {
    self.analysis_allowance_map.insert(pass_name, allowance);
  }

  pub fn matrix_unit_operand_precision(&self) -> Precision {
    self.matrix_unit_operand_precision.clone()
  }

  pub fn set_matrix_unit_operand_precision(&mut self, matrix_unit_operand_precision: Precision) {
    self.matrix_unit_operand_precision = matrix_unit_operand_precision;
  }

  pub fn fdo_profile(&self) -> String {
    self.fdo_profile.clone()
  }

  pub fn device_memory_size(&self) -> i64 {
    self.device_memory_size
  }

  pub fn set_device_memory_size(&mut self, device_memory_size: i64) {
    self.device_memory_size = device_memory_size;
  }
}
