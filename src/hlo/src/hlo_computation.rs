#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap, HashSet};

use common::{blitz_data::OpMetadata, shape::ProgramShape};

use crate::{
  dfs_hlo_visitor_with_default::{DfsHloRewriteVisitor, FunctionVisitor},
  hlo_clone_context::HloCloneContext,
  hlo_instruction::{self, HloInstruction, HloPrintOptions, print_name},
  hlo_module::HloModule, hlo_opcode::HloOpcode, name_uniquer::NameUniquer
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HloComputation {
  name: String,
  unique_id: i64,
  root_instruction: HloInstruction,
  fusion_instruction: HloInstruction,
  is_fusion_computation: bool,
  custom_call_instruction: HloInstruction,
  is_custom_call_computation: bool,
  collective_call_instruction: HloInstruction,
  is_collective_call_instruction: bool,
  while_call_instruction: HloInstruction,
  is_while_call_body_computation: bool,
  async_instructions: Vec<HloInstruction>,
  execution_thread: String,
  //parent
  instructions: Vec<HloInstruction>,
  to_be_deleted: Vec<HloInstruction>,
  param_instructions: Vec<HloInstruction>,

  // Callers and callees of this computation.
  // * These include all computations that have a caller/callee relationship
  //   with this computation, even those that may not belong to a module. For
  //   example, a computation that has been created and is in the process of
  //   being constructed but has not been added to a module yet may appear here.
  // * These are ordered maps, ordered by (unique ID, computation pointer). The
  //   unique ID is used to ensure determinism, whereas the computation pointer
  //   is used to disambiguate computations that do not belong to any module and
  //   therefore have a unique ID of -1. We assume that determinism only matters
  //   for computations that belong to a module (i.e, unique_id != -1), since
  //   the primary use case for this data structure is to topologically sort
  //   computations in a module.
  // * The values of the maps are the number of times the computation is
  //   referenced. In a graph sense, this is the number of parallel edges.
  pub callee_computations: BTreeMap<HloComputation, i64>,
  pub caller_computations: BTreeMap<HloComputation, i64>
}

impl HloComputation {
  pub fn default() -> Self {
    HloComputation {
      name: String::new(),
      unique_id: -1,
      root_instruction: HloInstruction::default(),
      fusion_instruction: HloInstruction::default(),
      is_fusion_computation: false,
      custom_call_instruction: HloInstruction::default(),
      is_custom_call_computation: false,
      collective_call_instruction: HloInstruction::default(),
      is_collective_call_instruction: false,
      while_call_instruction: HloInstruction::default(),
      is_while_call_body_computation: false,
      async_instructions: Vec::new(),
      execution_thread: String::new(),
      instructions: Vec::new(),
      to_be_deleted: Vec::new(),
      param_instructions: Vec::new(),
      callee_computations: BTreeMap::new(),
      caller_computations: BTreeMap::new()
    }
  }

  pub fn new(
    name: &String,
    parameter_count: usize,
    instructions: &Vec<HloInstruction>,
    root_instruction: HloInstruction,
    preserve_instruction_ids: bool) -> Self
  {
    let mut instance = HloComputation {
      name: NameUniquer::get_sanitized_name(name),
      unique_id: -1,
      root_instruction: root_instruction,
      fusion_instruction: HloInstruction::default(),
      is_fusion_computation: false,
      custom_call_instruction: HloInstruction::default(),
      is_custom_call_computation: false,
      collective_call_instruction: HloInstruction::default(),
      is_collective_call_instruction: false,
      while_call_instruction: HloInstruction::default(),
      is_while_call_body_computation: false,
      async_instructions: Vec::new(),
      execution_thread: String::new(),
      instructions: Vec::new(),
      to_be_deleted: Vec::new(),
      param_instructions: Vec::new(),
      callee_computations: BTreeMap::new(),
      caller_computations: BTreeMap::new()
    };
    instance.param_instructions.resize(
      parameter_count, HloInstruction::default());
    let mut root_found = false;

    if preserve_instruction_ids {
      // Pre-allocate all instructions in the vector since it state should be
      // identical.
      let mut max_instruction_local_id = 0;
      for instr in instructions {
        max_instruction_local_id =
          usize::max(max_instruction_local_id, instr.local_id() as usize);
      }
      instance.instructions.resize(max_instruction_local_id + 1,
        HloInstruction::default());
    }
    for instr in instructions {
      if instr.opcode() == HloOpcode::Parameter {
        let param_no = instr.parameter_number();
        assert!(param_no >= 0 && (param_no as usize) < parameter_count);

        instance.param_instructions.insert(
          param_no as usize, instr.clone());
      }
      root_found |= instr == &instance.root_instruction;
      instance.add_instruction_internal(instr.clone(),
        preserve_instruction_ids);
    }
    assert!(root_found);
    instance.root_instruction.mark_as_root();
    instance
  }

  // Add an instruction to the computation.
  // The computation takes ownership of the instruction.
  pub fn add_instruction(
    &mut self,
    mut instruction: HloInstruction,
    name: &String) -> &mut HloInstruction
  {
    assert!(instruction.opcode() != HloOpcode::Parameter,
      "Parameter insstructions cannot be added to a computation after it has been built.");
    if !name.is_empty() { instruction.set_and_sanitize_name(name); }
    self.add_instruction_internal(instruction, false);

    // TODO
    unimplemented!()
  }

  pub fn add_instruction_by_metadata(
    &mut self,
    mut instruction: HloInstruction,
    metadata: OpMetadata) -> &mut HloInstruction
  {
    instruction.set_metadata(metadata);
    self.add_instruction(instruction, &"".to_string())
  }

  fn add_instruction_internal(
    &mut self,
    _instruction: HloInstruction,
    _preserve_unique_id: bool)
  {
    unimplemented!();
  }

  pub fn replace_parameter() {}
  pub fn remove_parameter() {}
  pub fn remove_unused_parameters_from_fused_computation() {}
  pub fn remove_unused_parameters_from_any_computation() {}

  // Adds a new parameter instruction to a fusion computation.
  pub fn add_parameter(&mut self, instruction: HloInstruction) {
    assert!(instruction.opcode() == HloOpcode::Parameter);
    assert!(!self.is_fusion_computation() ||
      self.fusion_instruction().as_ref().unwrap().operand_count() ==
      self.param_instructions.len());
    
    // TODO
    //instruction.set_parent(self);
    //self.param_instructions.push(instruction);
    self.add_instruction_internal(instruction, false);
  }

  pub fn add_entry_computation_parameter() {}
  pub fn replace_entry_computation_parameter() {}

  pub fn remove_instruction(&mut self, _instruction: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn force_remove_instruction() {}

  pub fn remove_instruction_and_unused_operands(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    unimplemented!()
  }

  // Set the root of the computation to the given instruction. The instruction
  // must have already been added to the computation.
  pub fn set_root_instruction(
    &mut self,
    _root_instruction: HloInstruction,
    _accept_different_shape: bool)
  {

  }

  // Return the root instruction of the computation. The root instruction is the
  // instruction which produces the output of the computation.
  pub fn root_instruction(&self) -> &HloInstruction {
    &self.root_instruction
  }

  pub fn mutable_root_instruction(&mut self) -> &mut HloInstruction {
    &mut self.root_instruction
  }

  // Returns the number of parameters for this computation.
  pub fn num_parameters(&self) -> usize {
    self.param_instructions.len()
  }

  // Returns the parameter instruction for the given parameter number.
  pub fn parameter_instruction(&self, param_no: usize) -> Option<&HloInstruction> {
    assert!(param_no < self.param_instructions.len());
    self.param_instructions.get(param_no)
  }

  pub fn mutable_parameter_instruction(
    &mut self, param_no: usize) -> Option<&mut HloInstruction>
  {
    assert!(param_no < self.param_instructions.len());
    self.param_instructions.get_mut(param_no)
  }

  pub fn parameter_instructions(&self) -> &Vec<HloInstruction> {
    &self.param_instructions
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }

  // Use the given NameUniquer to select a unique name for the computation based
  // on the computation's existing name.
  //
  // See also HloModule::SetAndUniquifyComputationName(), which does this plus
  // SetAndSanitizeName().
  pub fn uniquify_name(&mut self, name_uniquer: &mut NameUniquer) {
    self.name = name_uniquer.get_unique_name(&self.name);
  }

  pub fn print() {}

  pub fn to_string(&self) -> String { "".to_string() }
  pub fn to_string_with_options(&self, _options: &HloPrintOptions) -> String {
    unimplemented!()
  }

  pub fn to_cord() {}
  pub fn to_proto() {}
  pub fn new_from_proto() {}
  pub fn absl_hash_values() {}

  pub fn instructions(&self) -> &Vec<HloInstruction> {
    &self.instructions
  }

  pub fn mutable_instructions(&mut self) -> &mut Vec<HloInstruction> {
    &mut self.instructions
  }

  pub fn make_instruction_post_order(&self) -> &Vec<HloInstruction> {
    unimplemented!()
  }

  pub fn mutable_make_instruction_post_order(&mut self) -> &mut Vec<HloInstruction> {
    unimplemented!()
  }

  pub fn make_instruction_post_order_from() {}
  pub fn make_instruction_post_order_with_reshape_first() {}
  pub fn for_each_instruction_post_order() {}

  pub fn instruction_count(&self) -> usize {
    self.instructions.len()
  }

  pub fn make_embedded_computations_list() {}
  pub fn create_fusion_instruction() {}
  pub fn create_async_instructions() {}
  pub fn deep_copy_instruction() {}
  pub fn deep_copy_instruction_with_custom_copier() {}

  // Computes and returns the ProgramShape of this computation (shape of
  // parameters and result with layout).
  pub fn compute_program_shape(&self, include_ids: bool) -> ProgramShape {
    let mut program_shape = ProgramShape::default();
    for param_instr in &self.param_instructions {
      program_shape.add_parameter(
        param_instr.shape().clone(),
        print_name(param_instr.name(), include_ids));
    }
    *program_shape.mutable_result() = self.root_instruction.shape().clone();
    program_shape
  }

  pub fn replace_with_new_instruction(
    &self,
    _old: &HloInstruction,
    _new: &HloInstruction) -> Result<(), String>
  {
    unimplemented!()
  }

  pub fn replace_with_entry_computation_parameter() {}

  pub fn replace_instruction(
    &self,
    _old_instruction: &HloInstruction,
    _new_instruction: &HloInstruction,
    _preserve_sharding: bool,
    _relay_control_dependency: bool,
    _remove_unused_operands: bool) -> Result<bool, String>
  {
    unimplemented!()
  }

  pub fn replace_instruction_with_defferent_shape() {}

  pub fn set_parent(&mut self, _parent: &HloModule) {
    unimplemented!()
  }

  pub fn parent(&self) -> Option<&HloModule> {
    unimplemented!()
  }

  pub fn mutable_parent(&mut self) -> Option<&mut HloModule> {
    unimplemented!()
  }

  pub fn accept(&self, _visitor: &FunctionVisitor) -> Result<(), String> {
    unimplemented!()
  }

  pub fn accept_rewrite_visitor(&self, _visitor: &DfsHloRewriteVisitor) -> Result<(), String> {
    unimplemented!()
  }

  pub fn accept_ordered() {}

  // Returns true if the given instruction can be removed from the computation.
  // Paarameter instructions cannot ne removed without violating invariants of
  // the HLO computation with the exception of fusion computation.
  pub fn is_safely_removable(
    &self,
    instruction: &HloInstruction,
    ignore_control_dependency: bool) -> bool
  {
    if !ignore_control_dependency && instruction.has_control_dependencies() {
      return false;
    }
    if instruction.opcode() == HloOpcode::Parameter && !self.is_fusion_computation() {
      return false;
    }
    true
  }

  pub fn compute_channel_dependencies() {}

  // Returns true if this computation has a side effect.
  // A computation has a side effect if it contains one or more instruction with
  // a side effect.
  pub fn has_side_effect(&self) -> bool {
    for instruction in &self.instructions {
      if instruction.has_side_effect() { return true; }
    }
    false
  }

  // Returns if this computation is a fusion computation.
  pub fn is_fusion_computation(&self) -> bool {
    false //self.is_fusion_computation
  }

  // Returns if the computation is the entry computation of the module.
  pub fn is_entry_computation(&self) -> bool {
    self.parent().as_ref().unwrap().entry_computation().unwrap() == self
  }

  // Returns the owning fusion instruction, or nullptr if this is not a fusion
  // computation.
  pub fn fusion_instruction(&self) -> Option<&HloInstruction> {
    Some(&self.fusion_instruction)
  }

  pub fn set_fusion_instruction() {}

  pub fn is_custom_call_computation(&self) -> bool {
    self.is_custom_call_computation
  }

  pub fn custom_call_instruction(&self) -> &HloInstruction {
    &self.custom_call_instruction
  }

  pub fn set_custom_call_instruction() {}

  pub fn is_collective_called_computation(&self) -> bool {
    self.is_collective_call_instruction
  }

  pub fn collective_call_instruction(&self) -> &HloInstruction {
    &self.collective_call_instruction
  }

  pub fn set_collective_call_instruction() {}

  pub fn is_while_body_computation(&self) -> bool {
    self.is_while_call_body_computation
  }

  pub fn while_call_instruction(&self) -> &HloInstruction {
    &self.while_call_instruction
  }

  pub fn set_while_call_instruction(&mut self, _while_call_instruction: &HloInstruction) {
    unimplemented!()
  }

  pub fn is_async_computation(&self) -> bool {
    !self.async_instructions.is_empty()
  }

  pub fn async_instructions(&self) -> &Vec<HloInstruction> {
    &self.async_instructions
  }

  pub fn add_async_instruction() {}
  pub fn remove_async_instruction() {}

  pub fn is_called_computation(&self) -> bool {
    self.is_fusion_computation() || self.is_custom_call_computation()
  }

  // Clear the unique ID of the computation so that it can be re-assigned, such
  // as for the purpose of compacting the unique IDs.
  pub fn clear_unique_id_internal(&mut self) {
    self.unique_id = -1;
  }

  pub fn set_unique_id(&mut self, id: i64) {
    assert!(self.unique_id == -1);
    assert!(id >= 0);
    self.unique_id = id;
  }

  pub fn get_instruction_with_name() {}

  pub fn unique_id(&self) -> i64 {
    self.unique_id
  }

  pub fn set_execution_thread(&mut self, execution_thread: String) {
    self.execution_thread = execution_thread;
  }

  pub fn execution_thread(&self) -> String {
    self.execution_thread.clone()
  }

  pub fn is_main_thread(&self) -> bool {
    self.execution_thread.as_str() == hlo_instruction::MAIN_EXECUTION_THREAD
  }

  // Deallocate instructions that are marked by 'remove_instruction'.
  pub fn cleanup(&mut self) {
    self.to_be_deleted.clear()
  }

  // Returns true if a given instruction is marked dead in this computation.
  pub fn is_marked_as_dead(&self, _inst: &HloInstruction) -> bool {
    false
  }

  pub fn can_expand_into_single_instruction() {}

  // Like Clone(), but if an instruction is present in replacement_map, we use
  // the map's value to replace that instruction in the cloned computation.
  //
  // If replacements is nullptr, don't perform replacement.
  // If replacements maps a key to nullptr, we remove that instruction from the
  // new computation.  If an element of `replacements` references an instruction
  // that's not already in the computation, it's cloned and added to the new
  // computation.
  //
  // 'extra_parameters' allows to specify additional parameters that should be
  // added to the computation.
  //
  // All relevant instructions are cloned, *including* unique_ptr in the
  // `replacements` map.
  pub fn clone_with_replacements(
    &self,
    _replacements: &HashMap<HloInstruction, HloInstruction>,
    _extra_parameeters: &Vec<HloInstruction>,
    _context: Option<HloCloneContext>,
    _suffix: String,
    _new_root: Option<HloInstruction>) -> HloComputation
  {
    unimplemented!()    
  }
}

pub struct HloComputationBuilder {
  name: String,
  instructions: Vec<HloInstruction>,
  parameter_numbers: HashSet<i64>,
}

impl HloComputationBuilder {
  pub fn new(name: String) -> Self {
    HloComputationBuilder {
      name: name,
      instructions: Vec::new(),
      parameter_numbers: HashSet::new()
    }
  }

  // Build and return an HloComputation. The parameter root_instruction
  // specifies the already-added instruction to use as the root. If
  // root_instruction is nullptr then use the last added instruction as the
  // root.
  pub fn build(
    &self, root_instruction: Option<&HloInstruction>) -> HloComputation
  {
    let mut parameter_count = 0;
    for instr in &self.instructions {
      if instr.opcode() == HloOpcode::Parameter {
        parameter_count += 1;
      }
    }
    // If root_instruction is not specified use the last added instruction.
    let mut root = self.last_added_instruction();
    if root_instruction.is_some() {
      root = root_instruction;
    }
    assert!(root.is_some());
    HloComputation::new(&self.name, parameter_count, &self.instructions,
      root.unwrap().clone(), false)
  }

  // Add the instruction to be part of this computation.
  // If the new instruction is derived from another one, you probably want to do
  // `original_inst->AddInstruction(new_inst)` instead.
  pub fn add_instruction(&mut self, instruction: HloInstruction) -> &HloInstruction {
    self.instructions.push(instruction);
    self.instructions.last().unwrap()
  }

  pub fn add_parameter(
    &mut self, parameter: HloInstruction) -> Result<&HloInstruction, String>
  {
    if !self.parameter_numbers.insert(parameter.parameter_number()) {
      let mut err_msg = "duplicate parameter number ".to_string();
      err_msg.push_str(&parameter.parameter_number().to_string());
      return Err(err_msg);
    }
    Ok(self.add_instruction(parameter))
  }

  pub fn last_added_instruction(&self) -> Option<&HloInstruction> {
    if !self.instructions.is_empty() {
      return self.instructions.last();
    }
    None
  }
}