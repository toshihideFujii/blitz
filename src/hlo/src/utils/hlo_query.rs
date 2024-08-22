#![allow(dead_code)]

use crate::{hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode};


pub fn is_collective_communication_op(_op: &HloOpcode) -> bool {
  unimplemented!()
}

// Returns whether all of an instruction's operands are parameters.
pub fn all_operands_are_parameters(_instruction: &HloInstruction) -> bool {
  unimplemented!()
}

// Returns whether the module contains the given collective communication
// instructions with constrained layout.
pub fn contains_layout_constrained_collective(_module: &HloModule, _op: HloOpcode) -> bool {
  unimplemented!()
}

// Returns the next available channel id that can be used in the given module
// (for HloChannelInstructions).
pub fn next_channel_id(_module: &HloModule) -> i64 {
  unimplemented!()
}