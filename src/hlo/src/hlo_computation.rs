#![allow(dead_code)]

use crate::hlo_instruction::{HloInstruction, self};

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
  //instructions
  to_be_deleted: Vec<HloInstruction>,
  //param_instructions
}

impl HloComputation {
  pub fn new() {}
  pub fn add_instruction() {}
  pub fn replace_parameter() {}
  pub fn remove_parameter() {}
  pub fn remove_unused_parameters_from_fused_computation() {}
  pub fn remove_unused_parameters_from_any_computation() {}
  pub fn add_parameter() {}
  pub fn add_entry_computation_parameter() {}
  pub fn replace_entry_computation_parameter() {}
  pub fn remove_instruction() {}
  pub fn force_remove_instruction() {}
  pub fn remove_instruction_and_unused_operands() {}
  pub fn set_root_instruction() {}
  pub fn root_instruction() {}
  pub fn num_parameters() {}
  pub fn parameter_instruction() {}
  pub fn parameter_instructions() {}

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn uniquify_name() {}
  pub fn print() {}
  pub fn to_string() {}
  pub fn to_cord() {}
  pub fn to_proto() {}
  pub fn new_from_proto() {}
  pub fn absl_hash_values() {}
  pub fn instructions() {}
  pub fn make_instruction_post_order() {}
  pub fn make_instruction_post_order_from() {}
  pub fn make_instruction_post_order_with_reshape_first() {}
  pub fn for_each_instruction_post_order() {}
  pub fn instruction_count() {}
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
  pub fn parent() {}
  pub fn accept() {}
  pub fn accept_ordered() {}
  pub fn is_safely_removable() {}
  pub fn compute_channel_dependencies() {}
  pub fn has_side_effect() {}

  pub fn is_fusion_computation(&self) -> bool {
    self.is_fusion_computation
  }

  pub fn is_entry_computation() {}

  pub fn fusion_instruction(&self) -> &HloInstruction {
    &self.fusion_instruction
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

  pub fn cleanup(&mut self) {
    self.to_be_deleted.clear()
  }

  pub fn is_marked_as_dead() {}
  pub fn can_expand_into_single_instruction() {}
}