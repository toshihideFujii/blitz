#![allow(dead_code)]

use std::collections::HashMap;

use crate::{hlo_computation::HloComputation, hlo_instruction::HloInstruction};

// A simple DFS-based reachability analysis for HLO instructions.
//
// When the class is created, the instructions are ordered in a defs-before-uses
// topological order.
// The reachability query runs a DFS from the destination node (going up through
// operands / control predecessors), and stops when the instruction's index in
// the defs-before-uses list is before the source node. As the reachability is
// tested for nodes that are close to each other, this optimization works well,
// and the time is dominated by the post-order sort.
pub struct HloDfsReachability {
  instruction_to_idx: HashMap<HloInstruction, usize>
}

impl HloDfsReachability {
  pub fn build(computation: &HloComputation) -> Self {
    let mut instance = HloDfsReachability {
      instruction_to_idx: HashMap::new()
    };
    let instructions =
      computation.make_instruction_post_order();
    for i in 0..instructions.len() {
      instance.instruction_to_idx.insert(instructions[i].clone(), i);
    }
    instance
  }

  // Returns true iff the instruction was present in the computation passed to
  // Build() or it was added via OnInstructionReplaced(). The calling code may
  // want to still use the class after the computation is modified, if it's
  // known that the def-before-use order is still preserved.
  pub fn is_present(&self, instruction: &HloInstruction) -> bool {
    self.instruction_to_idx.contains_key(instruction)
  }

  // Returns true iff there is a path (with edges being users and control
  // successors) from 'from' to 'to'. (i.e. path from definitions to uses; from
  // producers to consumers)  
  pub fn is_reachable(
    &self, _from: &HloInstruction, _to: &HloInstruction) -> bool
  {
    /*
    if from == to {
      return true;
    }
    if to.operand_count() == 0 && from.control_predecessors().is_empty() {
      return false;
    }
    let target_node_idx = self.instruction_to_idx.get(from).unwrap();
    let dfs_root_idx = self.instruction_to_idx.get(to).unwrap();

    // Note that the DFS goes from the "uses" root towards the "defs", i.e. from
    // `to` node to `from` node, so the node indices are decreasing.
    if dfs_root_idx < target_node_idx {
      return false;
    }

    // We use LLVM support library here because it has stack-allocated bit vector
    // which significantly improves performance by avoiding heap allocations when
    // instructions are reachable via a short chain.
    let mut stack = vec![];

    // We will visit instructions in the [target_node_idx, dfs_root_idx] range, so
    // we can construct a smaller bit vector.
    let mut visited_idxs = vec![];

    let mut check_and_enqueue =
      |instr: &HloInstruction| -> bool
    {
      if instr == from {
        return true;
      }
      let instr_idx = self.instruction_to_idx.get(instr).unwrap();
      if instr_idx < target_node_idx {
        return false;
      }
      let visited_idx = instr_idx - target_node_idx;
      if visited_idxs.contains(&visited_idx) {
        return false;
      }
      visited_idxs.push(visited_idx);
      stack.push(instr.clone());
      false
    };

    while !stack.is_empty() {
      let instr = stack.pop().unwrap();
      for operand in instr.operands() {
        if check_and_enqueue(operand) {
          return true;
        }
      }
      for predecessor in instr.control_successors() {
        if check_and_enqueue(predecessor) {
          return true;
        }
      }
    }
    */
    false
  }

  // Returns true iff either `a` is reachable from `b` or `b` is reachable from
  // `a`.
  pub fn is_connected(&self, a: &HloInstruction, b: &HloInstruction) -> bool {
    self.is_reachable(a, b) || self.is_reachable(b, a)
  }

  // Updates the internal data structure when instruction `previous` was
  // replaced with instruction `now` in the computation. Requires
  // IsPresent(previous) returns true and IsPresent(now) returns false.
  // Postconditon: IsPresent(previous) returns false and IsPresent(now) returns
  // true.
  pub fn on_instruction_replaced(
    &mut self, previous: &HloInstruction, now: &HloInstruction)
  {
    let index = self.instruction_to_idx.remove(previous);
    debug_assert!(index.is_some());

    let inserted =
      self.instruction_to_idx.insert(now.clone(), index.unwrap());
    debug_assert!(inserted.is_some());
  }
}