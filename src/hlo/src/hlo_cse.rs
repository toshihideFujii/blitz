#![allow(dead_code)]

use std::collections::HashSet;

use crate::{hlo_computation::HloComputation, hlo_module::HloModule, hlo_opcode::HloOpcode};

// A pass which perform common-subexpression elimination.
// Identical constants and identical instructions with the same operands are
// commoned. The pass iterates over the instructions in topological order
// which enables the pass to find arbitrarily large common expressions.
pub struct HloCSE {
  is_layout_sensitive: bool,
  only_fusion_computations: bool,
  ignore_control_dependencies: bool,
}

impl HloCSE {
  pub fn new() -> Self {
    HloCSE {
      is_layout_sensitive: false,
      only_fusion_computations: false,
      ignore_control_dependencies: false
    }
  }
  
  pub fn name() -> String {
    "cse".to_string()
  }

  pub fn run(&self, module: &HloModule, execution_threads: &HashSet<String>) -> bool {
    let mut changed = false;
    for computation in module.computations(execution_threads) {
      if self.only_fusion_computations && !computation.is_fusion_computation() {
        continue;
      }
      changed |= HloCSE::combine_constants(computation, self.is_layout_sensitive);
      
      for instruction in computation.make_instruction_post_order() {
        // If the instruction has zero operands (constants, parameters, etc.) skip over it.
        if instruction.operand_count() == 0 &&
           instruction.opcode() != HloOpcode::PartitionId &&
           instruction.opcode() != HloOpcode::ReplicaId
        {
          continue;
        }
        // Skip instructions which have side effects.
        if instruction.has_side_effect() {
          continue;
        }
        // representatives.insert
        for i in 0..instruction.operand_count() {
          let a = instruction.mutable_operand(i);
          if a.unwrap().opcode() != HloOpcode::Iota {
            continue;
          }
          for j in i+1..instruction.operand_count() {
            let b = instruction.mutable_operand(j);

            changed = true;
            if b.unwrap().is_dead() {
              //computation.remove_instruction(b.unwrap());
            }
          }
        }
      }
    }
    changed
  }

  fn combine_constants(_computation: &HloComputation, _is_layout_sensitive: bool) -> bool {
    false
  }
}