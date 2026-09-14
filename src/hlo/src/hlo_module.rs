#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use common::{
  blitz_data::{FrontendAttributes, StackFrameIndex},
  printer::Printer,
  shape::Shape
};

use crate::{
  compilation_environments::CompilationEnvironments, hlo_computation::HloComputation,
  hlo_input_output_alias_config::{
    HloBufferDonorConfig,
    HloInputOutputAliasConfig
  },
  hlo_instruction::HloPrintOptions, hlo_module_config::HloModuleConfig,
  hlo_module_metadata::HloModuleMetadata, hlo_proto::HloModuleProto,
  hlo_schdule::HloSchedule, hlo_sharding::HloSharding, name_uniquer::NameUniquer
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

#[derive(Clone, PartialEq)]
pub struct CrossProgramPrefetchInfo {
  parameter: i64,
  index: usize,
  alt_memory_offset: Option<i64>
}

#[derive(Clone, PartialEq)]
pub struct HloModule {
  name: String,
  entry_computation: Option<HloComputation>,
  computations: Vec<HloComputation>,
  next_unique_id: i64,
  // Used to keep track of the next unique computation id that should be
  // assigned to computations in this module.
  next_unique_computation_id: i64,
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
  //schedule: Option<HloSchedule>,
  spmd_parameters_shardings: Option<Vec<HloSharding>>,
  spmd_output_sharding: Option<HloSharding>,
  cross_program_prefetches: Vec<CrossProgramPrefetchInfo>,
  metadata: HloModuleMetadata,
  stack_frame_index: StackFrameIndex,
  // Unique name generator for computation and instruction names, which are
  // unique per module. Will be reset to nullopt when Finalize() is called.
  computation_name_uniquer: Option<NameUniquer>,
  instruction_name_uniquer: Option<NameUniquer>,
}

impl HloModule {
  pub fn new(name: String, config: HloModuleConfig) -> Self {
    HloModule {
      name: name,
      entry_computation: None,
      computations: Vec::new(),
      next_unique_id: 0,
      next_unique_computation_id: 0,
      unique_id: 0,
      is_dynamic: false,
      profile_verison: 0,
      relative_speedup: 0.0,
      autofdo_fingerprint: "".to_string(),
      use_auto_spmd_partitioning: false,
      config: config,
      frontend_attributes: FrontendAttributes::default(),
      use_auto_spmd_partition: false,
      input_output_alias_config: HloInputOutputAliasConfig::default(),
      buffer_donor_config: HloBufferDonorConfig::default(),
      spmd_parameters_shardings: None,
      spmd_output_sharding: None,
      cross_program_prefetches: Vec::new(),
      metadata: HloModuleMetadata::default(),
      stack_frame_index: StackFrameIndex::default(),
      computation_name_uniquer: Some(NameUniquer::new(".".to_string())),
      instruction_name_uniquer: Some(NameUniquer::new(".".to_string())),
    }
  }

  // Convert an HloModule to or from a proto.
  pub fn create_from_proto(
    _proto: &HloModuleProto,
    _module_config: &HloModuleConfig,
    _prohibit_empty_literal: bool,
    _comp_env: Option<&CompilationEnvironments>) -> HloModule
  {
     unimplemented!() 
  }

  // Adds an entry computation to the module. A module can only have one entry
  // computation. Returns a pointer to the newly added computation.
  pub fn add_entry_computation(
    &mut self, computation: HloComputation) -> &HloComputation
  {
    self.add_computation_internal(computation, true,
      true, false)
  }

  // Same as the AddEntryComputation function above but the module's
  // entry_computation_layout is updated to match the layout of the new entry
  // computation.
  pub fn add_entry_computation_with_layouts(
    &mut self, computation: HloComputation) -> &HloComputation
  {
    self.add_computation_internal(computation, true,
      true, true)
  }

  fn add_computation_internal(
    &mut self,
    mut computation: HloComputation,
    is_entry: bool,
    uniquify_identifiers: bool,
    preserve_entry_layouts: bool) -> &HloComputation
  {
    if is_entry {
      assert!(self.entry_computation.is_none());
      self.entry_computation = Some(computation.clone());

      if preserve_entry_layouts {
        let program_shape = self.entry_computation
          .as_ref().unwrap().compute_program_shape(true);
        self.mutable_config().set_computation_layout_if_exists(program_shape);
      } else if self.config().has_entry_computation_layout() {
        // If the module configuration has no entry layout computation set, create
        // a default one based on the program shape.
        let program_shape = self.entry_computation
          .as_ref().unwrap().compute_program_shape(true);
        self.mutable_config().set_default_computation_layout(program_shape);
      }
      self.input_output_alias_config = HloInputOutputAliasConfig::new(
        self.entry_computation.as_ref().unwrap()
        .root_instruction().shape().clone());
      self.buffer_donor_config = HloBufferDonorConfig::default();
    }

    if uniquify_identifiers {
      computation.uniquify_name(self.computation_name_uniquer());
      for instruction in computation.mutable_instructions() {
        instruction.uniquify_name(self.instruction_name_uniquer());
      }
      // Set unique id to this computation.
      computation.clear_unique_id_internal();
      computation.set_unique_id(self.read_and_increment_next_unique_computation_id());
      // Computation sets unique ID internally in sequence
      // Recompacts the instructions vector to remove nullptr entries.
      computation.cleanup();
    } else {
      // Don't uniquify the names of the computation or instruction, but we must
      // run the names through the uniquifiers to prevent future name collisions
      // for computations and instructions created later.
      self.resync_next_unique_computation_id(
        computation.unique_id());
      let _ = self.computation_name_uniquer().get_unique_name(
        &computation.name());
      for instruction in computation.mutable_instructions() {
        self.instruction_name_uniquer().get_unique_name(&instruction.name());
      }
    }

    computation.set_parent(self);
    // TODO
    for (caller, _count) in &computation.caller_computations {
      if caller.parent().unwrap() == self {
        // TODO
      }
    }
    for (callee, _count) in &computation.callee_computations {
      if callee.parent().unwrap() == self {
        // TODO
      }
    }
    self.computations.push(computation);
    self.computations.last().unwrap()
  }

  pub fn replace_entry_computation() {}

  // Adds an embedded computation to the module.
  pub fn add_embedded_computation(
    &mut self, _computation: HloComputation) -> &HloComputation
  {
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

  pub fn mutable_name(&mut self) -> &mut String {
    &mut self.name
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

  pub fn mutable_entry_computation(&mut self) -> Option<&mut HloComputation> {
    self.entry_computation.as_mut()
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

  // Based on module's entry_computation sharded shapes,
  // layout_canonicalization_callback_ computes and
  // returns <argument_layouts, result_layout> for module's entry computation.
  // argument_layouts is std::vector<Shape> and results_layout is Shape.
  // layout_canonicalization_callback_ is used only when
  // use_auto_spmd_partitioning_ = true.
  pub fn set_layout_canonicalization_callback(&self) {}

  pub fn layout_canonicalization_callback() {}
  pub fn absl_hash_value() {}

  // Gets the computations in this module.
  //
  // Returns a view of HloComputation*s, so you can iterate over this in the
  // natural way:
  //
  //   for (HloComputation* c : module->computations()) { ... }
  pub fn computations(&self) -> &Vec<HloComputation> {
    unimplemented!()
  }

  pub fn computations_with_cb<F>(&self, _callback: F) -> &Vec<HloComputation>
    where F: Fn(&HloModule) -> Result<(Vec<Shape>, Shape), String>
  {
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
    &self,
     _execution_threads: &HashSet<String>,
     _dfs_post_order: bool) -> Vec<&mut HloComputation>
  {
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

  pub fn mutable_make_nonfusion_computations(
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

  // Returns the NameUniquer for uniquing instruction names in this module.
  pub fn instruction_name_uniquer(&mut self) -> &mut NameUniquer {
    debug_assert!(self.computation_name_uniquer.is_some(),
      "Can't get instruction name uniquer after HloModule was finalized");
    self.instruction_name_uniquer.as_mut().unwrap()
  }

  // Returns the NameUniquer for uniquing computation names in this module.
  pub fn computation_name_uniquer(&mut self) -> &mut NameUniquer {
    debug_assert!(self.computation_name_uniquer.is_some(),
      "Can't get computation name uniquer after HloModule was finalized");
    self.computation_name_uniquer.as_mut().unwrap()
  }

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
  pub fn set_schedule(&mut self, _schedule: HloSchedule) {
    //self.schedule = Some(schedule);
    unimplemented!()
  }

  // Clears the schedule of the module.
  pub fn clear_schedule(&mut self) {
    //self.schedule = None;
    unimplemented!()
  }

  // Returns true if the module has a schedule set.
  pub fn has_schedule(&self) -> bool {
    //self.schedule.is_some()
    unimplemented!()
  }

  // Returns the schedule of the module.
  pub fn schedule(&self) -> &HloSchedule<'_> {
    //assert!(self.has_schedule());
    //&self.schedule.as_ref().unwrap()
    unimplemented!()
  }

  pub fn mutable_schedule(&mut self) -> &mut HloSchedule<'_> {
    //assert!(self.has_schedule());
    //self.schedule.as_mut().unwrap()
    unimplemented!()
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

  // Setter for the stack frame index.
  pub fn set_stack_frame_index(&mut self, stack_frame_index: StackFrameIndex) {
    self.stack_frame_index = stack_frame_index;
  }

  fn resync_next_unique_computation_id(&mut self, last_assigned_unique_id: i64) {
    // TODO
    self.next_unique_computation_id =
      i64::max(self.next_unique_computation_id, last_assigned_unique_id + 1)
  }

  fn read_and_increment_next_unique_computation_id(&mut self) -> i64 {
    // TODO
    let id = self.next_unique_computation_id;
    self.next_unique_computation_id += 1;
    id
  }
}