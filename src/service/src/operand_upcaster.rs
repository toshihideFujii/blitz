#![allow(dead_code)]

use common::{blitz_data::Precision, shape::Shape, shape_util::ShapeUtil};
use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};

use crate::shape_inference::ShapeInference;

// Inserts Convert to operands of instructions that allows result accumulation
// as wider integral types.
pub struct OperandUpcaster {}

impl OperandUpcaster {
  pub fn new() {}

  pub fn name(&self) -> String {
    "operand-upcaster".to_string()
  }

  pub fn instruction_matches_pattern(instruction: &HloInstruction) -> bool {
    let inferred_shape = maybe_infer_shape(instruction);
    if inferred_shape.is_err() {
      return false;
    }

    // Always expand packed nibble precision mode.
    let mut operand_count = 0;
    for precision in instruction.precision_config().operand_precision() {
      if *precision == Precision::PackedNibble {
        operand_count += 1;
      }
    }
    if operand_count == 2 {
      return true;
    }

    let inferred_type = inferred_shape.as_ref().unwrap().element_type();
    if instruction.shape().element_type() == inferred_type &&
       instruction.operand(0).shape().element_type() == inferred_type &&
       instruction.operand(1).shape().element_type() == inferred_type
    {
      return false;
    }
    
    ShapeUtil::element_can_upcast(inferred_shape.as_ref().unwrap(), instruction.shape())
  }

  pub fn expand_instruction(
    &self, _instruction: &HloInstruction) -> Result<HloInstruction, String>
  {
    unimplemented!()
  }
}

fn maybe_infer_shape(instruction: &HloInstruction) -> Result<Shape, String> {
  match instruction.opcode() {
    HloOpcode::Dot => {
      return ShapeInference::infer_dot_op_shape(
        instruction.operand(0).shape(),
        instruction.operand(1).shape(),
        instruction.dot_dimension_numbers(),
        None,
        instruction.sparsity());
    },
    HloOpcode::Convolution => {
      return ShapeInference::infer_convolve_shape(
        instruction.operand(0).shape(),
        instruction.operand(1).shape(),
        instruction.feature_group_count(),
        instruction.batch_group_count(),
        instruction.window(),
        instruction.convolution_dimension_numberes(),
        None);
    },
    _ => return Err("Unsupported opcode.".to_string())
  }
}