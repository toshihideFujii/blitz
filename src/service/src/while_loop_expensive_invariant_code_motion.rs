#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{hlo_instruction::HloInstruction, hlo_module::HloModule};

// HLO pass that rewrites while loops to hoist expensive and non-size-inflating
// groups of loop invariant instructions in the while body into the computation
// that contains the while instruction.
// Users can specify worth_hoisting_individually, and only the groups
// instructions with a root that returns true with it will be hoisted out.
pub struct WhileLoopExpensiveInvariantCodeMotion {}

impl WhileLoopExpensiveInvariantCodeMotion {
  pub fn new() {}
  
  pub fn name(&self) -> String {
    "while-loop-expensive-invariant-code-motion".to_string()
  }

  pub fn run(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    println!("HLO module before WhileLoopExpensiveInvariantCodeMotion: {:?}",
      module.to_string());
    
    let mut changed = false;
    let while_instrs: Vec<HloInstruction> = vec![];
    for _comp in module.computations_by_exec_threads(execution_threads) {
      // TODO
    }
    
    for while_instr in &while_instrs {
      // Right now we only hoist computations from the while body, but
      // TryHoistingInvariantInstructionsFromWhileBody can be generalized to
      // optimize the condition computation too, if needed.
      //
      // The transform we do here is a pessimization for while loops that execute
      // zero times*, but at this time we expect those to be rare.  If this
      // becomes a problem we can consider using the conditional HLO to avoid
      // doing extra work for while loops with zero trip count.
      //
      // * We delete while loops that have a zero trip count, so this would have
      //   to be a while loop with a somewhat opaque condition expression.
      let result =
        self.try_hoisting_invariant_instructions_from_while_body(while_instr);
      if result.is_err() {
        return Err(result.err().unwrap());
      }
      changed |= result.unwrap();
    }

    if changed {
      println!("HLO module after WhileLoopExpensiveInvariantCodeMotion: {:?}",
        module.to_string());
    } else {
      println!("HLO module unchanged after WhileLoopExpensiveInvariantCodeMotion");
    }

    Ok(changed)
  }

  fn try_hoisting_invariant_instructions_from_while_body(
    &self, _while_instr: &HloInstruction) -> Result<bool, String>
  {
    unimplemented!()    
  }
}

struct InvariantInfo {}