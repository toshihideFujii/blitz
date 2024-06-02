#![allow(dead_code)]

use std::collections::HashMap;

use crate::hlo_instruction::{HloInstruction, HloPrintOptions};

#[derive(Debug, Clone, PartialEq)]
pub enum VisitState {
  NotVisited,
  Visiting,
  Visited,
}

// A postorder depth-first HloInstruction visitor.
pub struct DfsHloVisitorBase {
  visit_state: HashMap<i64, VisitState>
}

impl DfsHloVisitorBase {
  pub fn new() -> Self {
    DfsHloVisitorBase { visit_state: HashMap::new() }
  }

  pub fn get_visit_state(&self, id: i64) -> Option<&VisitState> {
    self.visit_state.get(&id)
  }

  pub fn get_visist_state_by_instruction(
    &self, instruction: &HloInstruction) -> Option<&VisitState>
  {
    self.get_visit_state(instruction.unique_id())
  }

  // Resize internal state if necessary to hold state for ids <= num.
  pub fn reserve_visit_states(&mut self, num: usize) {
    self.visit_state.reserve(num);
  }

  pub fn visit_state_capacity(&self) -> usize {
    self.visit_state.capacity()
  }

  // Useful when we want to visit the same computation more than once with the
  // same visitor.
  pub fn reset_visit_states(&mut self) {
    self.visit_state.clear();
  }

  // Useful when we want to free up the memory used by the visit state without
  // destroying the actual visitor subclass.
  pub fn destroy_visit_state(&mut self) {
    self.visit_state.clear();
  }

  pub fn set_visit_state(&mut self, id: i64, state: VisitState) {
    self.visit_state.insert(id, state);
  }

  // Sets the visitation state of the given instruction as Visiting.
  pub fn set_visiting(&mut self, instruction: &HloInstruction) {
    println!("marking HLO {:?} as visiting: ",
      instruction.to_string(&HloPrintOptions::default()));
    debug_assert!(self.not_visited(instruction));
    self.visit_state.insert(instruction.unique_id(), VisitState::Visiting);
  }

  // Sets the visitation state of the given instruction as Visited.
  pub fn set_visited(&mut self, instruction: &HloInstruction) {
    println!("marking HLO {:?} as visited: ",
      instruction.to_string(&HloPrintOptions::default()));
    debug_assert!(self.not_visited(instruction) || self.is_visiting(instruction));
    self.visit_state.insert(instruction.unique_id(), VisitState::Visited);
  }

  // Returns whether the state of the given instruction is Visiting.
  pub fn is_visiting(&self, instruction: &HloInstruction) -> bool {
    *self.get_visist_state_by_instruction(instruction).unwrap() ==
      VisitState::Visiting
  }

  // Returns whether the state of the given instruction is Visited.
  pub fn did_visit(&self, instruction: &HloInstruction) -> bool {
    *self.get_visist_state_by_instruction(instruction).unwrap() ==
      VisitState::Visited
  }

  // Returns whether the state of the given instruction is NotVisited.
  pub fn not_visited(&self, instruction: &HloInstruction) -> bool {
    *self.get_visist_state_by_instruction(instruction).unwrap() ==
      VisitState::NotVisited
  }

  // This method should be overridden by subclasses that wish to run some
  // operation on an op before its handle* visitor methos is called.
  pub fn preprocess(&self, _hlo: &HloInstruction) -> Result<(), String> {
    Ok(())
  }

  // This method should be overridden by subclasses that wish to run some
  // operation on an op after its handle* visitor methos is called.
  pub fn postprocess(&self, _hlo: &HloInstruction) -> Result<(), String> {
    Ok(())
  }
}