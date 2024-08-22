#![allow(dead_code)]

use common::shape::Shape;
use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};

use crate::shape_inference::ShapeInference;

// Inserts Convert to result of instructions to the preferred element type
// specified by the instructions when direct accumulation of that type isn't
// supported by the backend. This pass should run after OperandUpcaster.
pub struct ResultCaster {}

impl ResultCaster {
  pub fn new() -> Self {
    ResultCaster {  }
  }

  pub fn name(&self) -> String {
    "result-caster".to_string()
  }

  pub fn instruction_matches_pattern(instruction: &HloInstruction) -> bool {
    let inferred_shape = maybe_infer_shape(instruction);
    if inferred_shape.is_err() { return false; }
    inferred_shape.unwrap().element_type() != instruction.shape().element_type()
  }

  pub fn expand_instruction(_instruction: &HloInstruction) -> Result<HloInstruction, String> {
    //let computation = instruction.parent();
    //let inferred_shape = maybe_infer_shape(instruction);
    unimplemented!()
  }
}

fn maybe_infer_shape(instruction: &HloInstruction) -> Result<Shape, String> {
  match instruction.opcode() {
    HloOpcode::Dot => return ShapeInference::infer_dot_op_shape(
      instruction.operand(0).shape(),
      instruction.operand(1).shape(),
      instruction.dot_dimension_numbers(),
      None,
      instruction.sparsity()),
    HloOpcode::Convolution => return ShapeInference::infer_convolve_shape(
      instruction.operand(9).shape(),
      instruction.operand(1).shape(),
      instruction.feature_group_count(),
      instruction.batch_group_count(),
      instruction.window(),
      instruction.convolution_dimension_numberes(),
      None),
    _ => return Err("Unsupported opcode.".to_string())
  }
}