#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule};

// Given a scheduled HLO module, this pass sinks the ROOT of the instruction to
// the bottom of the non-fusion computations. To avoid dependency violations of
// moving the ROOT instruction, it creates a new ROOT instruction that looks
// like the following:
//   - For tuple ROOT type:
//        new_root = tuple(gte(old_root), gte(old_root), ...)
//   - For non-tuple ROOT type:
//        new_root = bitcast(old_root)
pub struct RootInstructionSinker {}

impl RootInstructionSinker {
  pub fn new() {}

  pub fn name(&self) -> String {
    "root-instruction-sinker".to_string()
  }

  pub fn run(
    &mut self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    if !module.has_schedule() {
      return Err("Module has no schedule.".to_string());
    }

    let mut modified = false;
    for comp in module.make_nonfusion_computations(execution_threads) {
      let sequence = module.schedule().sequence(comp);
      if comp.root_instruction() == 
        &sequence.as_ref().unwrap().instructions()[sequence.as_ref().unwrap().size() - 1]
      {
        continue;
      }
      if comp.root_instruction().shape().is_tuple() {
        //sink_tuple_root(comp);
      } else {
        //sink_nontuple_root(module, comp)
      }
      modified = true;
    }

    Ok(modified)
  }
}

fn sink_tuple_root(_computation: &mut HloComputation) {
  unimplemented!()
}

// Sinks the root of the given computation for not-tuple root types.
fn sink_nontuple_root(_module: &HloModule, computation: &mut HloComputation) {
  let root = computation.root_instruction();
  assert!(!root.shape().is_tuple());
  let bitcast = HloInstruction::create_bitcast(root.shape(), root);

  let _new_root =
    computation.add_instruction(bitcast, "".to_string());

  //let sequence =
    //computation.mutable_parent().as_mut().unwrap()
    //.mutable_schedule().get_or_create_mutable_sequence(module, computation);

  //for operand in new_root.operands() {
    //sequence.push_pack(operand.clone());
  //}
  //sequence.push_pack(new_root.clone());
  //computation.set_root_instruction(new_root.clone(), false);
}