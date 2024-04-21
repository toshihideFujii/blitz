#![allow(dead_code)]

use common::blitz_data::PrimitiveType;
use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};

// This class has methods to query if a certain low-precision floating-point
// type, such as bfloat16, is supported in certain instructions on a given
// backend.
pub struct FloatSupport {
  low_precision_type: PrimitiveType,
  high_precision_type: PrimitiveType
}

impl FloatSupport {
  pub fn new(
    low_precision_type: PrimitiveType,
    high_precision_type: PrimitiveType) -> Self
  {
    FloatSupport {
      low_precision_type: low_precision_type,
      high_precision_type: high_precision_type
    }
  }

  pub fn low_precision_type(&self) -> &PrimitiveType {
    &self.low_precision_type
  }

  pub fn high_precision_type(&self) -> &PrimitiveType {
    &self.high_precision_type
  }

  pub fn supports_low_precision_operand(
    &self, hlo: &HloInstruction, operand_index: usize) -> bool
  {
    match hlo.opcode() {
      HloOpcode::Call => return true,
      HloOpcode::Conditional => return true,
      HloOpcode::CustomCall => return true,
      HloOpcode::Domain => return true,
      HloOpcode::GetTupleElement => return true,
      HloOpcode::Tuple => return true,
      HloOpcode::While => return true,
      HloOpcode::OptimizationBarrier => return true,
      HloOpcode::Convert => {
        debug_assert_eq!(operand_index, 0);
        return hlo.operand(0).shape().element_type() == self.low_precision_type
      },
      _ => return false
    }
  }

  pub fn supports_low_precision_output(&self, hlo: &HloInstruction) -> bool {
    match hlo.opcode() {
      HloOpcode::Call => return true,
      HloOpcode::Conditional => return true,
      HloOpcode::CustomCall => return true,
      HloOpcode::Domain => return true,
      HloOpcode::GetTupleElement => return true,
      HloOpcode::Tuple => return true,
      HloOpcode::While => return true,
      HloOpcode::OptimizationBarrier => return true,
      HloOpcode::Convert => return hlo.shape().element_type() == self.low_precision_type,
      _ => return false
    }
  }

  pub fn supports_mixed_presicion(&self, hlo: &HloInstruction) -> bool {
    match hlo.opcode() {
      HloOpcode::Call => return true,
      HloOpcode::Conditional => return true,
      HloOpcode::Convert => return true,
      HloOpcode::CustomCall => return true,
      HloOpcode::GetTupleElement => return true,
      HloOpcode::Tuple => return true,
      HloOpcode::While => return true,
      HloOpcode::OptimizationBarrier => return true,
      _ => return false
    }
  }

  pub fn effective_operand_precision_is_output_precision() {}

  pub fn effective_operand_precision_is_low_precision(&self) -> bool {
    false
  }
}