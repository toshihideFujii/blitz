#![allow(dead_code)]

use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};

// A pass which pergorms expansion of the logistic function.
pub struct LogisticExpander {}

impl LogisticExpander {
  pub fn new() -> Self {
    LogisticExpander {  }
  }

  pub fn name() -> String {
    "logistic-expander".to_string()
  }
  
  fn instruction_matches_pattern(&self, instruction: &HloInstruction) -> bool {
    instruction.opcode() == HloOpcode::Logistic
  }

  fn expand_instruction(&self, _instruction: &HloInstruction) {
    unimplemented!()
  }
}