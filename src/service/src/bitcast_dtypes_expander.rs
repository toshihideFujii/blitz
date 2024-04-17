#![allow(dead_code)]

use std::collections::HashMap;

use common::primitive_util::bit_width;
use hlo::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  hlo_opcode::HloOpcode
};

// A pass which expands bitcast-convert between differently sized dtypes
// to a reduction.
pub struct BitcastDtypesExpander {
  computation_chache: HashMap<String, HloComputation>
}

impl BitcastDtypesExpander {
  pub fn new() -> Self {
    BitcastDtypesExpander { computation_chache: HashMap::new() }
  }

  pub fn name() -> String { "bitcast_dtypes_expander".to_string() }

  pub fn instruction_matches_pattern(instruction: &HloInstruction) -> bool {
    instruction.opcode() == HloOpcode::BitcastConvert &&
    bit_width(&instruction.shape().element_type()) !=
    bit_width(&instruction.operand(0).shape().element_type())
  }

  pub fn expand_instruction(&self) {}
}