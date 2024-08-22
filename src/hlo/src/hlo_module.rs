#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use common::{
  blitz_data::FrontendAttributes,
  printer::Printer,
  shape::Shape
};

use crate::{
  hlo_computation::HloComputation,
  hlo_input_output_alias_config::{
    HloBufferDonorConfig,
    HloInputOutputAliasConfig
  },
  hlo_instruction::HloPrintOptions,
  hlo_module_config::HloModuleConfig,
  hlo_module_metadata::HloModuleMetadata,
  hlo_schdule::HloSchedule,
  hlo_sharding::HloSharding
};

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

#[derive(PartialEq)]
pub struct CrossProgramPrefetchInfo {
  parameter: i64,
  index: usize,
  alt_memory_offset: Option<i64>
}

#[derive(PartialEq)]
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
  use_auto_spmd_partitioning: bool,
  config: HloModuleConfig,
  frontend_attributes: FrontendAttributes,
  use_auto_spmd_partition: bool,
  input_output_alias_config: HloInputOutputAliasConfig,
  buffer_donor_config: HloBufferDonorConfig,
  schedule: Option<HloSchedule>,
  spmd_parameters_shardings: Option<Vec<HloSharding>>,
  spmd_output_sharding: Option<HloSharding>,
  cross_program_prefetches: Vec<CrossProgramPrefetchInfo>,
  metadata: HloModuleMetadata,
}

impl HloModule {
  pub fn new(_name: String, _config: HloModuleConfig) -> Self {
    unimplemented!()
  }

  pub fn add_entry_computation() {}
  pub fn add_entry_computation_with_layouts() {}
  pub fn replace_entry_computation() {}

  // Adds an embedded computation to the module.
  pub fn add_embedded_computation(&mut self, _computation: HloComputation) -> &HloComputation {
    unimplemented!()
  }

  // Removes an embedded computation.
  pub fn remove_embedded_computation(
    &self,
    _to_remove: &HloComputation) -> Result<(), String>
  {
    Ok(())
  }

  pub fn remove_unused_computations() {}

  // Mark duplicate fusions with the same name to be able to group them for
  // analysis pirposes.
  pub fn mark_fusion_duplications(
    &self, _replacements: &HashMap<HloComputation, HloComputation>)
  {
    unimplemented!()
  }

  // Replaces all uses of computations that are keys of 'replacements' with
  // the corresponding values in 'replacements'.
  pub fn replace_computations(
    &self, _replacements: &HashMap<HloComputation, HloComputation>)
  {
    unimplemented!()
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn move_computations_from() {}

  // Return a pointer to the entry computation of the module.
  pub fn entry_computation(&self) -> Option<&HloComputation> {
    //assert!(self.has_entry_computation());
    //self.entry_computation.as_ref().unwrap()
    self.entry_computation.as_ref()
  }

  pub fn has_entry_computation(&self) -> bool {
    self.entry_computation.is_some()
  }

  // Returns the root instruction shape of entry computation.
  pub fn result_shape(&self) -> &Shape {
    assert!(self.has_entry_computation());
    self.entry_computation().unwrap().root_instruction().shape()
  }

  pub fn compute_computation_layout() {}
  pub fn mutable_entry_computation_layout() {}
  pub fn entry_computation_layout() {}

  pub fn set_frontend_attributes(&mut self, frontend_attributes: FrontendAttributes) {
    self.frontend_attributes = frontend_attributes;
  }

  pub fn add_frontend_attributes(&mut self, _frontend_attributes: FrontendAttributes) {
    //for (k, v) in frontend_attributes.map().iter() {    
      //self.frontend_attributes.mutable_map()
        //.insert(k.clone(), v.clone());
    //}
  }

  pub fn frontend_attributes(&self) -> &FrontendAttributes {
    &self.frontend_attributes
  }

  pub fn set_use_auto_spmd_partitioning(&mut self, use_auto_spmd_partition: bool) {
    self.use_auto_spmd_partition = use_auto_spmd_partition;
  }

  pub fn use_auto_spmd_partitioning(&self) -> bool {
    self.use_auto_spmd_partition
  }

  pub fn set_layout_canonicalization_callback() {}
  pub fn layout_canonicalization_callback() {}
  pub fn absl_hash_value() {}

  pub fn computations(&self) -> &Vec<HloComputation> {
    unimplemented!()
  }

  pub fn mutable_computations(&mut self) -> &mut Vec<HloComputation> {
    unimplemented!()
  }

  pub fn computations_by_exec_threads(
    &self, _execution_threads: &HashSet<String>) -> &Vec<HloComputation>
  {
    unimplemented!()
  }

  pub fn mutable_computations_by_exec_threads(
    &mut self, _execution_threads: &HashSet<String>) -> &mut Vec<HloComputation>
  {
    unimplemented!()
  }

  pub fn get_computation_with_name() {}

  // Gets the number of computations in this module.
  pub fn computation_count(&self) -> usize {
    self.computations.len()
  }

  // Gets the number of instructions in this module.
  pub fn instruction_count(&self) -> usize {
    let mut n = 0;
    for computation in &self.computations {
      n += computation.instruction_count();
    }
    n
  }

  // Deallocate removed instructions in this module.
  pub fn cleanup(&mut self) {
    for computation in &mut self.computations {
      computation.cleanup();
    }
  }

  pub fn make_computation_post_order(
    &self, _execution_threads: &HashSet<String>) -> Vec<&mut HloComputation> {
    unimplemented!()
  }

  pub fn make_computation_sorted() {}

  // Gets the computation in this module which aren't for fusion nodes.
  pub fn make_nonfusion_computations_default(&self) -> &Vec<HloComputation> {
    unimplemented!()
  }

  pub fn make_nonfusion_computations(
    &self, _execution_threads: &HashSet<String>) -> &Vec<HloComputation>
  {
    unimplemented!()
  }

  pub fn make_mutable_nonfusion_computations(
    &mut self, _execution_threads: &HashSet<String>) -> &mut Vec<HloComputation>
  {
    unimplemented!()
  }

  pub fn make_nonfusion_computations_sorted() {}

  pub fn config(&self) -> &HloModuleConfig {
    &self.config
  }

  pub fn mutable_config(&mut self) -> &mut HloModuleConfig {
    &mut self.config
  }

  pub fn set_config(&mut self, config: HloModuleConfig) {
    self.config = config;
  }

  pub fn shared_config() {}

  pub fn is_dynamic(&self) -> bool {
    self.is_dynamic
  }

  pub fn set_is_dynamic(&mut self, is_dynamic: bool) {
    self.is_dynamic = is_dynamic;
  }

  // Prints a string representation of the module.
  pub fn print_default(printer: &dyn Printer) {
    HloModule::print(printer, HloPrintOptions::default())
  }

  pub fn print(_printer: &dyn Printer, _options: HloPrintOptions) {}

  pub fn to_string(&self) -> String { "".to_string() }
  
  pub fn to_cord() {}
  pub fn to_proto() {}
  pub fn new_from_proto() {}
  pub fn to_proto_with_config() {}
  pub fn new_from_proto_with_config() {}
  pub fn new_module_config_from_proto() {}
  pub fn outline_expression_from_computation() {}
  pub fn random_new_64() {}
  pub fn instruction_name_uniquer() {}

  // Assign a new unique dense id for an instruction.
  pub fn new_unique_instruction_id(&mut self) -> i64 {
    let result = self.next_unique_id;
    self.next_unique_id += 1;
    result
  }

  // input_output_alias_config indicates the list of aliased buffers that are
  // expected from the module.
  pub fn input_output_alias_config(&self) -> &HloInputOutputAliasConfig {
    &self.input_output_alias_config
  }

  // buffer_donor_config indicates the set of input buffer donors that are
  // expected from the module.
  pub fn buffer_donor_config(&self) -> &HloBufferDonorConfig {
    &self.buffer_donor_config
  }

  // Returns an id that is unique to this module across all modules created over
  // the lifetime of this process.
  pub fn unique_id(&self) -> i64 {
    self.unique_id
  }

  // Sets the schedule of the module to the given schedule.
  pub fn set_schedule(&mut self, schedule: HloSchedule) {
    self.schedule = Some(schedule);
  }

  // Clears the schedule of the module.
  pub fn clear_schedule(&mut self) {
    self.schedule = None;
  }

  // Returns true if the module has a schedule set.
  pub fn has_schedule(&self) -> bool {
    self.schedule.is_some()
  }

  // Returns the schedule of the module.
  pub fn schedule(&self) -> &HloSchedule {
    assert!(self.has_schedule());
    &self.schedule.as_ref().unwrap()
  }

  pub fn mutable_schedule(&mut self) -> &mut HloSchedule {
    assert!(self.has_schedule());
    self.schedule.as_mut().unwrap()
  }

  pub fn add_computation_and_unify_names_and_ids() {}
  pub fn set_and_uniquify_instr_name() {}
  pub fn check_unique_names_and_ids_for_computations_and_instructions() {}

  // Checks if this config has a list of entry parameter's HLO shardings for
  // SPMD.
  pub fn has_spmd_parameters_shardings(&self) -> bool {
    self.spmd_parameters_shardings.is_some()
  }

  // Getter and setter for the list of entry parameter's HLO shardings for SPMD.
  pub fn spmd_parameters_shardings(&self) -> &Vec<HloSharding> {
    assert!(self.has_spmd_parameters_shardings());
    self.spmd_parameters_shardings.as_ref().unwrap()
  }

  pub fn set_spmd_parameters_shardings(&mut self, shardings: Vec<HloSharding>) {
    self.spmd_parameters_shardings = Some(shardings);
  }

  // Checks if this config has the entry computation output's HLO sharding for
  // SPMD.
  pub fn has_spmd_output_sharding(&self) -> bool {
    self.spmd_output_sharding.is_some()
  }

  // Getter and setter for the entry computation output's HLO shardings for
  // SPMD.
  pub fn spmd_output_sharding(&self) -> &HloSharding {
    assert!(self.has_spmd_output_sharding());
    self.spmd_output_sharding.as_ref().unwrap()
  }

  pub fn set_spmd_output_sharding(&mut self, sharding: HloSharding) {
    self.spmd_output_sharding = Some(sharding);
  }

  // Add a program argument to be prefetched across programs.
  pub fn add_cross_program_prefetch(
    &mut self, parameter: i64, index: usize, alt_memory_offset: Option<i64>)
  {
    let info = CrossProgramPrefetchInfo {
      parameter: parameter, index: index, alt_memory_offset: alt_memory_offset
    };
    self.cross_program_prefetches.push(info);
  }

  pub fn set_cross_program_prefetch_offset() {}

  // Getthe list of program arguments to be prefetch across programs.
  pub fn cross_program_prefetches(&self) -> &Vec<CrossProgramPrefetchInfo> {
    &self.cross_program_prefetches
  }

  pub fn metadata(&self) -> &HloModuleMetadata {
    &self.metadata
  }

  // Moves (not copies) metadata from this HloModule to 'module'.
  pub fn move_metadata_to_module(&mut self, _module: &mut HloModule) {
    //module.metadata = self.metadata;
  }

  pub fn profile_version(&self) -> i64 {
    self.profile_verison
  }

  pub fn set_profile_version(&mut self, profile_version: i64) {
    self.profile_verison = profile_version;
  }

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