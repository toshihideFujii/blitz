#![allow(dead_code)]

use crate::hlo_computation::HloComputation;

pub struct StackFrame {
  file_name: String,
  function_name: String,
  line: i64,
  column: i64,
  parent_frame_id: i64
}

impl StackFrame {
  pub fn new() {}
  pub fn empty() -> bool { false }
}

pub struct HloModule {
  name: String,
  entry_computation: Option<HloComputation>,
  computations: Vec<HloComputation>,
  next_unique_id: i64,
  unique_id: i64,
  is_dynamic: bool,
  profile_verison: i64,
  relative_speedup: f64,
  autofdo_fingerprint: String,
  use_auto_spmd_partitioning: bool
}

impl HloModule {
  pub fn new() {}
  pub fn add_entry_computation() {}
  pub fn add_entry_computation_with_layouts() {}
  pub fn replace_entry_computation() {}
  pub fn add_embedded_computation() {}
  pub fn remove_embedded_computation() {}
  pub fn remove_unused_computations() {}
  pub fn mark_fusion_deplications() {}
  pub fn replace_computations() {}
  pub fn name() {}
  pub fn set_name() {}
  pub fn move_computations_from() {}
  pub fn entry_computation() {}
  pub fn has_entry_computation() {}
  pub fn result_shape() {}
  pub fn compute_computation_layout() {}
  pub fn set_frontend_attributes() {}
  pub fn add_frontend_attributes() {}
  pub fn frontend_attributes() {}
  pub fn set_use_auto_spmd_partitioning() {}
  pub fn use_auto_spmd_partitioning() {}
  pub fn set_layout_canonicalization_callback() {}
  pub fn layout_canonicalization_callback() {}
  pub fn absl_hash_value() {}
  pub fn computations() {}
  pub fn get_computation_with_name() {}
  pub fn computation_count() {}
  pub fn instruction_count() {}
  pub fn cleanup(&mut self) {}
  pub fn make_computation_post_order() {}
  pub fn make_computation_sorted() {}
  pub fn make_nonfusion_computations() {}
  pub fn make_nonfusion_computations_sorted() {}
  pub fn config() {}
  pub fn set_config() {}
  pub fn shared_config() {}
  pub fn is_dynamic() {}
  pub fn set_is_dynamic() {}
  pub fn print() {}
  pub fn to_string() {}
  pub fn to_cord() {}
  pub fn to_proto() {}
  pub fn new_from_proto() {}
  pub fn to_proto_with_config() {}
  pub fn new_from_proto_with_config() {}
  pub fn new_module_config_from_proto() {}
  pub fn outline_expression_from_computation() {}
  pub fn random_new_64() {}
  pub fn instruction_name_uniquer() {}
  pub fn new_unique_instruction_id() {}
  pub fn input_output_alias_config() {}
  pub fn unique_id() {}
  pub fn set_schedule() {}
  pub fn clear_schedule() {}
  pub fn has_schedule() {}
  pub fn schedule() {}

  pub fn add_computation_and_unify_names_and_ids() {}
  pub fn set_and_uniquify_instr_name() {}
  pub fn check_unique_names_and_ids_for_computations_and_instructions() {}
  pub fn has_spmd_parameters_shardings() {}
  pub fn spmd_parameters_shardings() {}
  pub fn set_spmd_parameters_shardings() {}

  pub fn has_spmd_output_sharding() {}
  pub fn spmd_output_sharding() {}
  pub fn set_spmd_output_sharding() {}

  pub fn add_cross_program_prefetch() {}
  pub fn set_cross_program_prefetch_offset() {}
  pub fn cross_program_prefetches() {}

  pub fn metadata() {}
  pub fn move_metadata_to_module() {}
  pub fn profile_version() {}
  pub fn set_profile_version() {}
  pub fn add_profile_info() {}
  pub fn set_profile_info() {}
  pub fn profile_info() {}
  pub fn set_autofdo_profile_key() {}
  pub fn set_autofdo_profile_keys() {}
  pub fn autofdo_profile_keys() {}
  pub fn has_module_autofdo_profiles() {}
  pub fn set_relative_speedup() {}
  pub fn set_autofdo_fingerprint() {}
  pub fn comp_envs() {}
  pub fn get_fingerprint_128() {}
  pub fn get_stack_frame() {}
}