#![allow(dead_code)]

use crate::{dfs_hlo_visitor::{DfsHloVisitorBase, VisitState}, hlo_instruction::HloInstruction};

struct DfsHloVisitorWithDefaultBase {
  visitor: DfsHloVisitorBase
}

impl DfsHloVisitorWithDefaultBase {
  pub fn new() {}
}

pub struct FunctionVisitor {
  visitor: DfsHloVisitorBase,
  visitor_func: Box<dyn FnMut(&HloInstruction)->Result<(), String>>
}

impl FunctionVisitor {
  pub fn new(visitor_func: Box<dyn FnMut(&HloInstruction)->Result<(), String>>) -> Self {
    FunctionVisitor {
      visitor: DfsHloVisitorBase::new(),
      visitor_func: visitor_func
    }
  }

  pub fn default_action(&mut self, instruction: &HloInstruction) -> Result<(), String> {
    self.visitor_func.as_mut()(instruction)
  }

  pub fn handle_elementwise_unary(&mut self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_elementwise_binary() {}
  pub fn handle_batch_norm_training() {}
  pub fn handle_batch_norm_inference() {}
  pub fn handle_batch_norm_grad() {}
  pub fn handle_clamp() {}
  pub fn handle_concatenate() {}
  pub fn handle_select() {}
  pub fn handle_dot() {}
  pub fn handle_fft() {}
  pub fn handle_traiangular_solve() {}
  pub fn handle_cholsky() {}
  pub fn handle_optimization_barrier() {}

  pub fn handle_all_gather() {}
  pub fn handle_all_gather_done() {}
  pub fn handle_all_gather_start() {}
  pub fn handle_all_reduce() {}
  pub fn handle_all_reduce_done() {}
  pub fn handle_all_reduce_start() {}
  pub fn handle_all_to_all() {}
  pub fn handle_collective_broadcast() {}
  pub fn handle_collective_permute() {}
  pub fn handle_collective_permute_done() {}
  pub fn handle_collective_permute_start() {}
  pub fn handle_convolution() {}
  pub fn handle_handle_optimization_barrier() {}
  pub fn handle_partition_id() {}
  pub fn handle_reduce_scatter() {}
  pub fn handle_replica_id() {}

  pub fn get_visit_state_by_instruction(
    &self, instruction: &HloInstruction) -> Option<&VisitState>
  {
    self.visitor.get_visist_state_by_instruction(instruction)
  }

  pub fn reserve_visit_states(&mut self, num: usize) {
    self.visitor.reserve_visit_states(num)
  }

  pub fn visit_state_capacity(&self) -> usize {
    self.visitor.visit_state_capacity()
  }

  pub fn reset_visit_states(&mut self) {
    self.visitor.reset_visit_states();
  }

  pub fn destroy_visit_state(&mut self) {
    self.visitor.destroy_visit_state();
  }

  pub fn set_visit_state(&mut self, id: i64, state: VisitState) {
    self.visitor.set_visit_state(id, state);
  }

  pub fn set_visiting(&mut self, instruction: &HloInstruction) {
    self.visitor.set_visiting(instruction);
  }

  pub fn set_visited(&mut self, instruction: &HloInstruction) {
    self.visitor.set_visited(instruction)
  }

  pub fn is_visiting(&self, instruction: &HloInstruction) -> bool {
    self.visitor.is_visiting(instruction)
  }

  pub fn did_visit(&self, instruction: &HloInstruction) -> bool {
    self.visitor.did_visit(instruction)
  }

  pub fn not_visited(&self, instruction: &HloInstruction) -> bool {
    self.visitor.not_visited(instruction)
  }

  pub fn preprocess(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.visitor.preprocess(hlo)
  }

  pub fn postprocess(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.visitor.postprocess(hlo)
  }
}