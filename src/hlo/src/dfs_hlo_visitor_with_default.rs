#![allow(dead_code)]

use std::collections::HashSet;
use crate::{
  dfs_hlo_visitor::{DfsHloVisitorBase, VisitState}, hlo_instruction::HloInstruction,
  hlo_module::HloModule
};

// DfsHloVisitor with default action based on the HloInstruction being visited.
// Users should not use this class directly, but use the type aliases
// DfsHloVisitorWithDefault/ConstDfsHloVisitorWithDefault instead.
//
// Do *not* add an override to this class if the opcode is covered by
// HandleElementwiseUnary/Binary. These opcode handlers dispatch to
// HandleElementwiseUnary/Binary in DfsHloVisitorBase. Adding such a handler
// here will break passes which rely on the HandleElementwiseUnary/Binary
// handling these opcodes.
pub trait DfsHloVisitor {
  fn default_action(&self, _instruction: &HloInstruction) -> Result<(), String> {
    Ok(())
  }
}

struct DfsHloVisitorWithDefaultBase {
  visitor: DfsHloVisitorBase
}

impl DfsHloVisitorWithDefaultBase {
  pub fn new() -> Self {
    DfsHloVisitorWithDefaultBase {
      visitor: DfsHloVisitorBase::new()
    }
  }

  pub fn handle_elementwise_unary(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_elementwise_binary(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_batch_norm_training(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_batch_norm_inference(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_batch_norm_grad(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_clamp(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_concatenate(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_selsect(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_dot(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_ragged_dot(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_convolution(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_fft(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_triangular_solve(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_cholsky(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_optimization_barrier(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_all_gather(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_all_gather_start(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_all_gather_done(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_all_reduce(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_reduce_scatter(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_all_reduce_start(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_all_reduce_done(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_all_to_all(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_ragged_all_to_all(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_collective_broadcast(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_collective_permute(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_collective_permute_start(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_collective_permute_done(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_replica_id(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_partition_id(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_rng(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_rng_bit_generator(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_rng_get_and_update_state(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_infeed(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_outfeed(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_reverse(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_sort(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_constant(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_iota(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_get_tuple_element(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_parameter(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_fusion(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_call(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_custom_call(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_slice(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_dynamic_slice(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_dynamic_update_slice(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_tuple(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_map(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_reduce(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_reduce_window(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handleselect_and_scatter(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_bitcast(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_broadcast(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_pad(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_dynamic_reshape(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_reshape(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_transpose(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_while(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_conditional(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_async_start(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_async_update(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_async_done(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_copy_start(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_copy_done(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_recv(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_recv_done(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_send(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_top_k(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_send_done(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_gather(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_scatter(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_after_all(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_get_dimension_size(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_set_dimension_size(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  pub fn handle_add_dependency(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.default_action(hlo)
  }

  // Invoked to inform the visitor that the traversal has completed, and that
  // the root was "root".
  pub fn finish_visit(&self, _hlo: &HloInstruction) -> Result<(), String> {
    Ok(())
  }

  fn default_action(&self, _instruction: &HloInstruction) -> Result<(), String> {
    Ok(())
  }
}

// A common base class for visitors performing rewriting operation.
//
// Subclasses call ReplaceWithNewInstruction and ReplaceInstruction while
// visiting.
pub struct DfsHloRewriteVisitor {
  base: DfsHloVisitorWithDefaultBase,
  changed: bool,
}

impl DfsHloRewriteVisitor {
  pub fn new() -> Self {
    DfsHloRewriteVisitor {
      base: DfsHloVisitorWithDefaultBase::new(),
      changed: false,
    }
  }

  // Runs a visitor on the module and returns whether the module has changed.
  pub fn run_on_module(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    for comp in module.make_nonfusion_computations(execution_threads) {
      let status = comp.accept_rewrite_visitor(&self);
      if status.is_err() { return Err(status.err().unwrap()); }
    }
    Ok(self.changed())
  }

  // Default visitor action is to do nothing and return OK.
  pub fn default_action(_hlo: &HloInstruction) -> Result<(), String> {
    Ok(())
  }

  pub fn changed(&self) -> bool {
    self.changed
  }

  // Replaces the existing HLO instruction old_instruction, with
  // new_instruction, and marks the optimizer status as changed.
  // Returns the absl::Status representing the result of the replace operation.
  pub fn replace_with_new_instruction(
    &mut self,
    old: &HloInstruction,
    new: &HloInstruction) -> Result<(), String>
  {
    let status =
      old.parent().replace_with_new_instruction(old, new);
    if status.is_ok() {
      self.changed = true;
    }
    status
  }

  // Replaces the existing HLO instruction old_instruction, with
  // new_instruction, and marks the optimizer status as changed.
  // Returns the absl::Status representing the result of the replace operation.
  pub fn replace_instruction(
    &mut self,
    old: &HloInstruction,
    new: &HloInstruction,
    preserve_sharding: bool) -> Result<bool, String>
  {
    let changed_or = old.parent().replace_instruction(
      old, new, preserve_sharding, true, false);
    if changed_or.is_ok() {
      self.changed |= changed_or.as_ref().unwrap();
    }
    Ok(*changed_or.as_ref().unwrap())
  }

  // Mark the computation as having changed.
  pub fn mark_as_changed(&mut self) {
    self.changed = true;
  }

  pub fn mark_as_maybe_changed(&mut self, changed: bool) {
    self.changed |= changed;
  }
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