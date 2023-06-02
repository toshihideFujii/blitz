#![allow(dead_code)]

struct InstructionPrecedenceTracking {}
impl InstructionPrecedenceTracking {
  pub fn new() {}
  pub fn fill() {}
  pub fn validate() {}
  pub fn validate_all() {}
  pub fn get_first_special_instruction() {}
  pub fn has_special_instructions() {}
  pub fn is_preceded_by_special_instruction() {}
  pub fn is_special_instruction() {}
  pub fn insert_instruction_to() {}
  pub fn remove_instruction() {}
  pub fn remove_usesrs_of() {}
  pub fn clear() {}
}

struct ImplicitControlFlowTracking {}
impl ImplicitControlFlowTracking {
  pub fn new() {}
  pub fn get_first_icfi() {}
  pub fn has_icf() {}
  pub fn is_dominated_by_icfi_from_same_block() {}
  pub fn is_special_instruction() {}
}

struct MemoryWriteTracking {}
impl MemoryWriteTracking {
  pub fn new() {}
  pub fn get_first_memory_write() {}
  pub fn may_write_to_memory() {}
  pub fn is_dominated_by_memory_write_from_same_block() {}
  pub fn is_special_instruction() {}
}