#![allow(dead_code)]

// HLO pass that rewrites while loops to hoist loop invariant instructions in
// the while body into the computation that contains the while instruction.

use std::collections::{HashMap, HashSet};

use hlo::{hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode};

use crate::compile_time_cap::BoundNonLinearCompilerAnalysis;

// If `hoist_constants` is true then constants are always hoisted out of while
// loop bodies.  Otherwise they are only hoisted out if they enable other
// non-trivial computations to be hoisted out.
//
// Setting `hoist_constants` to false can be help if LICM is run in the mid
// level HLO pipeline because hoisting constants out of while loop bodies can
// break optimizations like constant folding.
//
// Setting `hoist_other` and `hoist_reshapes` to false can be used to hoist
// only constants. If provided, `hoist_size_inflation_ratio` will forbid
// hoisting instructions where the ratio of the size of the output(s) to the
// input(s) is larger than hoist_size_inflation_ratio. This is useful on
// platforms on which it's important to prevent blow-ups in memory size.
//
// If `hoist_reshapes` is true, then reshapes are allowed to be hoisted out of
// while loop body by themselves. Otherwise, they are only hoisted out if they
// enable other non-trivial computations to be hoisted out.
//
// Setting `hoist_reshapes` to false can be useful when LICM is run in the
// mid level HLO pipeline because the reshapes will often get fused with
// consumer instructions, and won't cost anything if not hoisted. However,
// any stand alone reshapes after fusion will benefit from hoisting.
pub struct WhileLoopInvariantCodeMotion {
  hoist_constants: bool,
  hoist_reshapes: bool,
  hoist_other: bool,
  hoist_size_inflation_ratio: Option<f64>
}

impl WhileLoopInvariantCodeMotion {
  pub fn new(
    hoist_constants: bool,
    hoist_reshapes: bool,
    hoist_other: bool,
    hoist_size_inflation_ratio: Option<f64>) -> Self
  {
    WhileLoopInvariantCodeMotion {
      hoist_constants: hoist_constants,
      hoist_reshapes: hoist_reshapes,
      hoist_other: hoist_other,
      hoist_size_inflation_ratio: hoist_size_inflation_ratio
    }
  }

  pub fn name(&self) -> String {
    "while-loop-invariant-code-motion".to_string()
  }

  pub fn run(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    println!("HLO module before WhileLoopInvariantCodeModtion: {:?}", module.to_string());

    let mut changed = false;
    let while_instrs: Vec<HloInstruction> = vec![];
    for _comp in
      module.make_computation_post_order(execution_threads, false) {
      // TODO
    }
    
    let allowance =
      BoundNonLinearCompilerAnalysis::new(module, self.name(), Some(10));
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

      if !allowance.continue_analysis() {
        break;
      }
      let result =
        self.try_hoisting_invariant_instructions_from_while_body(while_instr, &allowance);
      if result.is_err() {
        return Err(result.err().unwrap());
      }
      changed |= result.unwrap();
    }

    if changed {
      // TODO: dce
    }

    if changed {
      println!("HLO module after WhileLoopInvariantCodeMotion: {:?}", module.to_string());
    } else {
      println!("HLO module unchanged after WhileLoopInvariantCodeMotion");
    }

    Ok(changed)
  }

  // Returns true if `instruction` is worth hoisting only if it lets us hoist some
  // instruction using it. The rationale is that hoisting these instructions will
  // prevent simplification, fusion, and sharding annotation in the while body.
  fn not_worth_hoisting_individually(&self, instruction: &HloInstruction) -> bool {
    if instruction.is_custom_call("Sharding".to_string()) {
      return true;
    }
    match instruction.opcode() {
      HloOpcode::Constant => return !self.hoist_constants,
      HloOpcode::Reshape => return !self.hoist_reshapes,
      HloOpcode::Broadcast => return true,
      HloOpcode::Iota => return true,
      HloOpcode::Reverse => return true,
      HloOpcode::Slice => return true,
      HloOpcode::Transpose => return true,
      HloOpcode::Tuple => return true,
      _ => return false
    }
  }

  fn try_hoisting_invariant_instructions_from_while_body(
    &self,
    _while_instr: &HloInstruction,
    _allowance: &BoundNonLinearCompilerAnalysis) -> Result<bool, String>
  {
    unimplemented!()
  }
}

// Copies `to_hoist` to the computation containing `while_instr`, hoisting its
// operands as needed.  All of its transitive operands are expected to be either
// in `hoisted_instructions` or `unhoisted_invariant_instructions`.  This
// function hoists the operands in `unhoisted_invariant_instructions` and moves
// them into `hoisted_instructions`.
fn create_loop_invariant_copy(
  _hoisted_instructions: &HashMap<HloInstruction, HloInstruction>,
  _unhoisted_invariant_instructions: &HashSet<HloInstruction>,
  _while_instr: &HloInstruction,
  _to_hoist: &HloInstruction)
{
  unimplemented!()  
}