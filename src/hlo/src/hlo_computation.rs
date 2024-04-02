#![allow(dead_code)]

use crate::{hlo_instruction::{self, HloInstruction}, hlo_module::HloModule, hlo_opcode::HloOpcode};

#[derive(PartialEq, Eq, Hash)]
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
}

impl HloComputation {
  pub fn new() {}

  // Add an instruction to the computation.
  // The computation takes ownership of the instruction.
  pub fn add_instruction(&mut self, mut instruction: HloInstruction, name: String) {
    assert!(instruction.opcode() != HloOpcode::Parameter,
      "Parameter insstructions cannot be added to a computation after it has been built.");
    if !name.is_empty() { instruction.set_and_sanitize_name(name); }
    self.add_instruction_internal(instruction);
  }

  fn add_instruction_internal(&mut self, _instruction: HloInstruction) {}

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
    self.add_instruction_internal(instruction);
  }

  pub fn add_entry_computation_parameter() {}
  pub fn replace_entry_computation_parameter() {}
  pub fn remove_instruction() {}
  pub fn force_remove_instruction() {}
  pub fn remove_instruction_and_unused_operands() {}

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

  // Returns the number of parameters for this computation.
  pub fn num_parameters(&self) -> usize {
    self.param_instructions.len()
  }

  // Returns the parameter instruction for the given parameter number.
  pub fn parameter_instruction(&self, param_no: usize) -> Option<&HloInstruction> {
    assert!(param_no < self.param_instructions.len());
    self.param_instructions.get(param_no)
  }

  pub fn parameter_instructions(&self) -> &Vec<HloInstruction> {
    &self.param_instructions
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn uniquify_name() {}
  pub fn print() {}

  pub fn to_string(&self) -> String { "".to_string() }

  pub fn to_cord() {}
  pub fn to_proto() {}
  pub fn new_from_proto() {}
  pub fn absl_hash_values() {}

  pub fn instructions(&self) -> &Vec<HloInstruction> {
    &self.instructions
  }

  pub fn make_instruction_post_order() {}
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
  pub fn compute_program_shape() {}
  pub fn replace_with_new_instruction() {}
  pub fn replace_with_entry_computation_parameter() {}
  pub fn replace_instruction() {}
  pub fn replace_instruction_with_defferent_shape() {}
  pub fn set_parent() {}

  pub fn parent(&self) -> &Option<HloModule> {
    unimplemented!()
  }

  pub fn accept() {}
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

  pub fn is_entry_computation() {}

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

  pub fn set_while_call_instruction() {}

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
  pub fn is_marked_as_dead(_inst: &HloInstruction) -> bool {
    false
  }

  pub fn can_expand_into_single_instruction() {}
}