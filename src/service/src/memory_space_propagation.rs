#![allow(dead_code)]

use std::collections::HashSet;

use common::{shape::Shape, shape_util::ShapeUtil};
use hlo::{hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode};

use crate::hlo_dataflow_analysis::HloDataflowAnalysis;

// This is a legalization pass that propagates the memory space in the
// layout to the fusion computations.
pub struct MemorySpacePropagation {
  dataflow_analysis: Option<HloDataflowAnalysis>
}

impl MemorySpacePropagation {
  pub fn new() -> Self {
    MemorySpacePropagation {
      dataflow_analysis: None
    }
  }

  pub fn name() -> String {
    "memory-space-propagation".to_string()
  }

  pub fn run(
    &self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let mut modified = true;
    let _dataflow_analysis = HloDataflowAnalysis::run(
      module, false, true, None,
      None, execution_threads).unwrap();

    for computation in
      module.make_nonfusion_computations_by_exec_threads(execution_threads) {
      for instruction in computation.instructions() {
        if instruction.opcode() == HloOpcode::Fusion {
          // Propagate the operand subshapes.
          for operand_idx in 0..instruction.operand_count() {
            let mut operand_func =
              |subshape: &Shape, index_vec: &Vec<i64>|
            {
              let memory_space = subshape.layout().as_ref().unwrap().memory_space();
              modified |= self.propagate(
                index_vec,
                instruction.fused_parameter(
                  operand_idx as i64),
                memory_space);
            };
            ShapeUtil::for_each_mutable_leaf_shape(
              instruction.operand(operand_idx).shape(), &mut operand_func);
          }
          // Propagate output subshapes.
          let mut output_func =
            |subshape: &Shape, index_vec: &Vec<i64>|
          {
            let memory_space = subshape.layout().as_ref().unwrap().memory_space();
            modified |= self.propagate(
              index_vec,
              instruction.fused_expression_root(),
              memory_space);
          };
          ShapeUtil::for_each_mutable_leaf_shape(instruction.shape(), &mut output_func);
        }
      }
    }
    Ok(modified)
  }

  fn propagate(
    &self,
    index_vec: &Vec<i64>,
    callee_instruction: &HloInstruction,
    _memory_space: i64) -> bool
  {
    let modified = false;
    let value = self.dataflow_analysis.as_ref().unwrap()
      .get_unique_value_at(callee_instruction, index_vec);

    for _pos in value.positions() {
      
    }

    for _use_ in value.get_uses() {
      // For fusion uses, propagate the memory space to the fusion parameter.
      //modified |= self.propagate(use_., callee_instruction, memory_space)
    }

    modified
  }
}