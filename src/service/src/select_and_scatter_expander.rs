#![allow(dead_code)]

use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};

// This pass rewrites select-and-scatter operations into a window reduction and
// a scatter as described in the conceptual explanation of the "select" and
// "scatter" steps of this operation.
pub struct SelectAndScatterExpander {}

impl SelectAndScatterExpander {
  pub fn new() -> Self {
    SelectAndScatterExpander {  }
  }

  pub fn name(&self) -> String {
    "select-and-scatter-expander".to_string()
  }

  pub fn instruction_matches_pattern(&self, inst: &HloInstruction) -> bool {
    inst.opcode() == HloOpcode::SelectAndScatter
  }

  pub fn expand_instruction(
    &self, _innst: &HloInstruction) -> Result<HloInstruction, String>
  {
    unimplemented!()
  }
}