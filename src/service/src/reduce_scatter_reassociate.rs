#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_opcode::HloOpcode,
  utils::hlo_query
};

use crate::collective_ops_utils::{match_reduction_instruction, ReductionKind};

// A pass that reassociates reduce-scatter feeding into compatible elementwise
// operations. As an example: add(reduce-scatter(x), reduce-scatter(y)) will be
// replaced with reduce_scatter(add(x,y)).
//
//  i.e., reassociating the reduce-scatter operation.
pub struct ReduceScatterReassociate {}

impl ReduceScatterReassociate {
  pub fn new() -> Self {
    ReduceScatterReassociate {  }
  }

  pub fn name(&self) -> String {
    "reduce-scatter-reassociate".to_string()
  }

  pub fn run(
    &mut self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    if hlo_query::contains_layout_constrained_collective(module, HloOpcode::ReduceScatter) {
      println!("Skip ReduceScatterReassociate because the module contains
        reduce-scatter with constrained layouts.");
      return Ok(false);
    }

    let _next_channel_id = hlo_query::next_channel_id(module);
    let mut changed = false;

    for computation in
      module.mutable_computations_by_exec_threads(execution_threads)
    {
      for instruction in computation.mutable_make_instruction_post_order() {
        let kind = match_reduction_instruction(instruction);
        if kind.is_none() ||
           instruction.operand(0).opcode() != HloOpcode::ReduceScatter ||
           instruction.operand(1).opcode() != HloOpcode::ReduceScatter ||
          !instruction.shape().is_array()  
        {
          continue;
        }

        let rs0 = instruction.operand(0);
        let rs1 = instruction.operand(1);
        if !are_compatible(rs0, rs1, &kind.unwrap()) {
          println!("Reduce-Scatter operations are not compatible, skipping.");
          continue;
        }

        if rs0.user_count() != 1 || rs1.user_count() != 1 {
          println!("Reduce-Scatter operations have > 1 users.");
          continue;
        }

        // TODO
        
        //let result = computation.remove_instruction(instruction);
        //if result.is_err() { return Err(result.err().unwrap()); }

        changed = true;
      }
    }

    Ok(changed)
  }
}

pub fn are_compatible(
  _rs0: &HloInstruction,
  _rs1: &HloInstruction,
  _op_kind: &ReductionKind) -> bool
{
  unimplemented!()
}