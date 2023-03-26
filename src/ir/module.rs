#![allow(dead_code)]

// This file contains the declarations for the Module class.
use super::blits_context::*;

enum ModFlagBehavior {
  Error,
  Warning,
  Require,
  Override,
  Append,
  AppendUnique,
  Max,
  Min
}

pub fn is_valid_mod_flag_behavior() {}
pub fn is_valid_module_flag() {}

struct ModuleFlagEntry {}

struct Module {
  context: BlitzContext
}

impl Module {
  pub fn new() {}
  pub fn get_module_identifier() {}
  pub fn get_instruction_count() {}
  pub fn get_source_file_name() {}
  pub fn get_name() {}
  pub fn get_data_layout_str() {}
  pub fn get_data_layout() {}
  pub fn get_target_triple() {}

  // Get the global data context.
  pub fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  pub fn get_module_inline_asm() {}
  pub fn create_rng() {}
  pub fn should_emit_instr_count_changed_remark() {}

  pub fn set_module_identifier() {}
  pub fn set_source_file_name() {}
  pub fn set_data_layout() {}
  pub fn set_target_triple() {}
  pub fn set_module_inline_asm() {}
  pub fn append_module_inline_asm() {}
}