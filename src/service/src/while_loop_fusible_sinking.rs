#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode};

// Sinks while loop invariant values that happen to be fusibles into the while
// loop body and conditional. This is probably not a win in isolation but may
// unlock further optimizations like fusible folding.
//
//   state = (..., fusible_graph, ...)
//   while (pred(state)) {
//     (..., v, ...) = state
//     use(v)
//     state = (..., v, ...)
//   }
//
// =>
//
//   state = (..., fusbile_graph, ..., fusible_graph_operands)
//   while (pred(state)) {
//     (..., v, ...) = state
//     use(fusibile_graph)
//     state = (..., v, ...)
//   }
//
// Note that it leaves the `v` in place to keep that component of the state
// tuple trivially loop invariant.  WhileLoopSimplifier will later get rid of
// `v`.
//
pub struct WhileLoopFusibleSinking {
  call_counts: HashMap<HloComputation, i64>
}

impl WhileLoopFusibleSinking {
  pub fn new() -> Self {
    WhileLoopFusibleSinking { call_counts: HashMap::new() }
  }

  pub fn name(&self) -> String {
    "while-loop-fusible-sinking".to_string()
  }

  pub fn run(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    self.call_counts.clear();
    let mut changed = false;

    let while_instrs: Vec<HloInstruction> = vec![];
    for _comp in module.make_nonfusion_computations(execution_threads) {
      // TODO
    }
    
    for while_instr in &while_instrs {
      let mut while_body_val =
        self.call_counts.get(while_instr.while_body()).unwrap().clone();
      while_body_val += 1;
      self.call_counts.insert(while_instr.while_body().clone(), while_body_val);

      let mut while_cond_val =
        self.call_counts.get(while_instr.while_condition()).unwrap().clone();
      while_cond_val += 1;
      self.call_counts.insert(while_instr.while_condition().clone(), while_cond_val);
    }

    for while_instr in &while_instrs {
      let result = self.try_sinking_fusibles_into_while_loop(while_instr);
      if result.is_err() {
        return Err(result.err().unwrap());
      }
      changed |= result.unwrap();
    }

    Ok(changed)
  }

  // Sink a fusible subgraph into a while loop.
  fn try_sinking_fusibles_into_while_loop(
    &self, _while_instr: &HloInstruction) -> Result<bool, String>
  {
    unimplemented!()    
  }

  // Creates a loop fusion instruction containing the computation to move into
  // the while loop to avoid conflicts with actual instruction fusion, the loop
  // fusion will be defused.
  fn is_sinkable_fusion(&self, while_operand: &HloInstruction) -> bool {
    let mut worklist = vec![];
    let mut visited: HashSet<i64> = HashSet::new();
    worklist.push(while_operand.clone());

    while !worklist.is_empty() {
      let to_process = worklist.pop().unwrap();
      if !to_process.is_fusible() {
        return false;
      }
      if !visited.insert(to_process.unique_id()) {
        // Do not sink extremely large subgraphs as they will be expensive to
        // recompute in the loop.
        if visited.len() > 100 {
          return false;
        }
        continue;
      }
      if is_purely_expanding(&to_process) {
        continue;
      }
      if is_fusion_candidate(&to_process) {
        for op in to_process.operands() {
          worklist.push(op.clone());
        }
        continue;
      }
      return false;
    }
    true
  }

  fn create_sinkable_fusion(&self, _while_operand: &HloInstruction) -> HloInstruction {
    unimplemented!()
  }
}

// Constant and Iota have no operands and an output and broadcasts add
// dimensions to the output so we are looking fusions that have much smaller
// operand sizes compared to output sizes to avoid materialization
fn is_purely_expanding(instr: &HloInstruction) -> bool {
  instr.opcode() == HloOpcode::Broadcast ||
 (instr.opcode() == HloOpcode::Constant && instr.shape().rank() == 0) ||
  instr.opcode() == HloOpcode::Iota
}

fn is_fusion_candidate(instr: &HloInstruction) -> bool {
  instr.opcode() != HloOpcode::Rng &&
 (instr.is_elementwise() || instr.opcode() == HloOpcode::Reshape ||
  instr.opcode() == HloOpcode::Transpose)
}