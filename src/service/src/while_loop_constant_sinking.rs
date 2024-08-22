#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode};

// Sinks while loop invariant values that happen to be constants into the while
// loop body and conditional. This is probably not a win in isolation but may
// unlock further optimizations like constant folding.
//
//   state = (..., const, ...)
//   while (pred(state)) {
//     (..., v, ...) = state
//     use(v)
//     state = (..., v, ...)
//   }
//
// =>
//
//   state = (..., const, ...)
//   while (pred(state)) {
//     (..., v, ...) = state
//     use(const)
//     state = (..., v, ...)
//   }
//
// Note that it leaves the `v` in place to keep that component of the state
// tuple trivially loop invariant.  WhileLoopSimplifier will later get rid of
// `v`.
//
pub struct WhileLoopConstantSinking {
  sink_broadcast_of_constants: bool,
  sink_only_scalar_constants: bool,
}

impl WhileLoopConstantSinking {
  pub fn new(sink_broadcast_of_constants: bool, sink_only_scalar_constants: bool) -> Self {
    WhileLoopConstantSinking {
      sink_broadcast_of_constants: sink_broadcast_of_constants,
      sink_only_scalar_constants: sink_only_scalar_constants
    }
  }

  pub fn name(&self) -> String {
    "while-loop-constant-sinking".to_string()
  }

  pub fn run(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    println!("HLO module before WhileLoopConstantSinking: {:?}", module.to_string());

    let mut changed = false;
    let while_instrs: Vec<HloInstruction> = vec![];
    for _comp in module.make_nonfusion_computations(execution_threads) {
      // TODO
    }

    for while_instr in &while_instrs {
      let result =
        self.try_sinking_constants_into_while_loop(while_instr);
      if result.is_err() {
        return Err(result.err().unwrap());
      }
      changed |= result.unwrap();
    }

    if changed {
      println!("HLO module after WhileLoopConstantSinking: {:?}", module.to_string());
    } else {
      println!("HLO module unchanged after WhileLoopConstantSinking");
    }

    Ok(changed)
  }

  fn try_sinking_constants_into_while_loop(
    &self, _while_instr: &HloInstruction) -> Result<bool, String>
  {
    unimplemented!()    
  }
}

// Replaces all uses of old_instr with new_instr except the use at
// `while_body_root` (which must be a tuple instruction) at index `tuple_index`.
// This utility helps us replace an instruction in the while body with a
// constant while still keeping it trivially loop invariant.
fn replace_uses_while_keeping_loop_invariance(
  old_instr: &HloInstruction,
  new_instr: &HloInstruction,
  while_body_root: &HloInstruction,
  tuple_index: usize) -> Result<(), String>
{
  assert_eq!(while_body_root.opcode(), HloOpcode::Tuple);

  let mut users: Vec<HloInstruction> = vec![];
  users.reserve(old_instr.user_count());
  users.clone_from(old_instr.users());

  for user in &mut users {
    for i in 0..user.operand_count() {
      if user.operand(i) == old_instr && !(user == while_body_root && i == tuple_index) {
        let result =
          user.replace_operand_with(i as i64, new_instr.clone());
        if result.is_err() {
          return Err(result.err().unwrap());
        }
      }
    }
  }
  Ok(())
}

fn clone_helper(
  _instruction: &HloInstruction,
  _computation: &HloComputation) -> HloInstruction
{
  unimplemented!()    
}