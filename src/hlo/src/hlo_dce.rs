#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_opcode::HloOpcode,
  hlo_pass_interface::HloPassInterface
};

// HLO pass which removes dead instructions from each computation in
// the module and removes dead computations from the module.
// An instruction is dead if it is not reachable from the root.
// A computation is dead if it is not the entry computation of the module and
// it is not reachable from the entry computation.
// This pass does not remove dead parameter instructions, as parameter instructions
// cannot be deleted.
pub struct HloDCE {
  remove_cross_partition_collective_ops: bool
}

impl HloDCE {
  pub fn new(remove_cross_partition_collective_ops: bool) -> Self {
    HloDCE {
      remove_cross_partition_collective_ops: remove_cross_partition_collective_ops
    }
  }

  pub fn name() -> String {
    "dce".to_string()
  }

  // Run DCE on a computation.
  pub fn run_on_computation(
    &mut self,
    computation: &mut HloComputation,
    remove_cross_partition_collective_ops: bool) -> bool
  {
    let mut changed = false;
    println!("Before dce:");
    println!("{:?}", computation.to_string());

    // Remove any dead roots and their dead transitive operands.
    // Collect them into a separate list first to avoid problems with iterating
    // through the computation's instruction while simultaneously removing instructions.
    let mut dead_roots = Vec::new();
    for instruction in computation.instructions() {
      if instruction.is_dead() &&
         computation.is_safely_removable(instruction, false) &&
         (!instruction.is_custom_call("Sharding".to_string()) ||
          (!instruction.operand(0).is_root() &&
            instruction.opcode() != HloOpcode::Parameter &&
            instruction.operand(0).user_count() == 1) &&
          (!instruction.has_side_effect() ||
           (remove_cross_partition_collective_ops &&
            instruction.is_collective_instruction() && !instruction.constrain_layout()) ||
          HloDCE::is_removable_while(instruction, remove_cross_partition_collective_ops)))
      {
        dead_roots.push(instruction);
      }
    }

    for _dead_root in dead_roots {
      // TODO
      //computation.remove_instruction_and_unused_operands(dead_root);
      changed = true;
    }

    if changed {
      println!("After dce:");
      println!("{:?}", computation.to_string());
    }
    changed
  }

  // Run the pass on the given module.
  // Returns whether the module was changed (instructions were removed).
  pub fn run(&mut self, module: &HloModule, _execution_threads: HashSet<String>) -> bool {
    let mut changed = false;
    println!("Before dce:");
    println!("{:?}", module.to_string());

    // Run DCE on each computation.
    for computation in module.make_computation_post_order() {
      changed |= self.run_on_computation(computation, self.remove_cross_partition_collective_ops)
    }

    // Now DCE HloComputations.
    // Keep doing passes through the module until no more computations can be
    // eliminated. The functions removes all aubcomputations that cana be proved
    // to have no ramining live callers.
    changed |= HloDCE::recursively_remove_dead_computations(module);

    println!("After dve:");
    println!("{:?}", module.to_string());

    changed
  }

  // Finds all computations that are not called by any instruction and removes
  // them from the module. Returns whether any dead code was removed.
  fn recursively_remove_dead_computations(module: &HloModule) -> bool {
    // Tracks whether any dead code is eliminated by this pass.
    let mut module_contains_dead_code = false;

    // First, collect thecomputations that are referenced yb some remaining
    // instruction. We need to record this as a refcount map rather than a set
    // since we cannot guarantee that control flow flattening has been dine
    // and there may be multiple call sites.
    let mut live_computation_call_count: HashMap<&HloComputation, i64> = HashMap::new();
    let mut count = 1;
    let entry_computation = module.entry_computation();
    if entry_computation.is_some() {
      live_computation_call_count.insert(entry_computation.unwrap(), count);
      count += 1;
    }

    // Account for all threads' caller when counting a sub computation's live
    // call count.
    for computation in module.make_computation_post_order() {
      for instruction in computation.instructions() {
        for subcomp in instruction.called_computations() {
          live_computation_call_count.insert(subcomp, count);
          count += 1;
        }
      }
    }

    // Find dead computations.
    for computation in module.make_computation_post_order() {
      // Finds all 'top-level' dead computations not called by any instructions.
      // contains(comp) == true and live_computaiton_call_count[comp] = 0 also
      // inplies that computation is dead, but is nested in other dead computations.
      // These inner computations are ignored here since they will be removed
      // recursing through other computations.
      if !live_computation_call_count.contains_key(computation) {
        if HloDCE::recursively_remove_dead_computation(module, computation,
        &live_computation_call_count).is_err()
        {
          module_contains_dead_code = true;
        }
      }
    }

    module_contains_dead_code
  }

  // Given a dead computation, decrements the ref count of all its called
  // computations and checks if any of the subcomputations become dead after
  // the removal.
  fn recursively_remove_dead_computation(
    module: &HloModule,
    computation: &HloComputation,
    live_call_counts: &HashMap<&HloComputation, i64>) -> Result<(), String>
  {
    // First loops all the sub-instruction/sub-computations.
    for instruction in computation.instructions() {
      for subcomp in instruction.called_computations() {
        let value = live_call_counts.get(subcomp);
        if value.is_none() {
          return Err("Called computation not found in liva_call_counts table during HloDCE.".to_string());
        } else {
          // Decrements the live call count and sees if there are no more live calls
          // to this computation.
          let mut live_call_count = *value.unwrap();
          live_call_count -= 1;
          assert!(live_call_count >= 0);
          if live_call_count == 0 {
            return HloDCE::recursively_remove_dead_computation(
              module, subcomp, &live_call_counts);
          }
        }
      }
    }
    println!("Removing dead computation {:?}.", computation.name());
    // After looping called subcomputations, now safe to delete the computation.
    module.remove_embedded_computation(computation)
  }

  fn is_removable_while(
    _instruction: &HloInstruction,
    _remove_cross_partition_collective_ops: bool) -> bool
  {
    false
  }
}

impl HloPassInterface for HloDCE {
  fn run(
    _module: &HloModule,
    _execution_threads: HashSet<String>) -> Result<(), String>
  {
    Ok(())
  }
}