#![allow(dead_code)]

use crate::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};


pub fn is_collective_communication_op(_op: &HloOpcode) -> bool {
  unimplemented!()
}

// Returns whether all of an instruction's operands are parameters.
pub fn all_operands_are_parameters(_instruction: &HloInstruction) -> bool {
  unimplemented!()
}