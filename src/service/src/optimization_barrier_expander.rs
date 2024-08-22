#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{hlo_module::HloModule, hlo_opcode::HloOpcode};

// This pass removes the opt-barrier operation which is functionally a no-op.
pub struct OptimizationBarrierExpander {}

impl OptimizationBarrierExpander {
  pub fn new() -> Self {
    OptimizationBarrierExpander {  }
  }

  pub fn name(&self) -> String {
    "cse-barrier-expander".to_string()
  }

  pub fn run(
    &mut self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let mut barriers = vec![];
    for comp in module.make_nonfusion_computations(execution_threads) {
      let mut modified = false;
      for inst in comp.instructions() {
        if inst.opcode() == HloOpcode::OptimizationBarrier {
          barriers.push(inst.clone());
          modified = true;
        }
      }
      if modified && module.has_schedule() {
        let sequences = module.schedule().sequences();
        let target = sequences.get(&comp.unique_id());
        if target.is_some() {
          //let sequence: Vec<HloInstruction> = vec![];
          //module.mutable_schedule().set_sequence_by_instr_vec(comp, &sequence);
        }
      }
    }

    for _inst in &mut barriers {
      /*
      let arg = inst.mutable_operand(0).unwrap();
      let mut result = arg.copy_all_control_deps_from(inst);
      if result.is_err() {
        return Err(result.err().unwrap());
      }
      result = inst.replace_all_uses_with(arg, "".to_string());
      if result.is_err() {
        return Err(result.err().unwrap());
      }
      result = inst.mutable_parent().remove_instruction(&inst);
      if result.is_err() {
        return Err(result.err().unwrap());
      }
      */
    }

    Ok(!barriers.is_empty())
  }
}