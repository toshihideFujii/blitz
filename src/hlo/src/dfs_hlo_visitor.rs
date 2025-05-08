
use std::collections::HashMap;
use crate::hlo_instruction::{HloInstruction, HloPrintOptions};

  // 3 possible visitation states of HLO instructions. Each instruction's
  // state only flows one way: kNotVisited -> kVisiting -> kVisited.
#[derive(Debug, Clone, PartialEq)]
pub enum VisitState {
  NotVisited,
  Visiting,
  Visited,
}

// A postorder depth-first HloInstruction visitor. When Handle* is called on an
// instruction, all its operands were already visited. User code can subclass
// this to iterate over an HloInstruction DAG. The Handle* routines have
// operands / data unpacked for ease of use in the visitor subclass.
//
// No instruction will ever be visited twice; however, the root instruction will
// be reported again when the traversal is done via a call to FinishVisit.
//
// If new HloInstructions are added during the traversal (e.g. by replacing an
// instruction), they will also be visited if they are the operand of an
// instruction that has not been visited yet (i.e. the instruction is in state
// kNotVisited). If you want to avoid that a newly added instruction 'hlo' is
// visited, you can call SetVisited(hlo). This may be necessary in normalization
// passes that replace all instructions, otherwise already replaced instructions
// might be visited (and replaced) again.
//
// A subclass must override at least
// (either HandleElementwiseUnary or all the Handle methods for unary ops) and
// (either HandleElementwiseBinary or all the Handle methods for binary ops)).
// The default Handle methods for (unary, binary) ops call
// (HandleElementwiseUnary, HandleElementwiseBinary).
// The default (HandleElementwiseUnary, HandleElementwiseBinary) return an
// "unimplemented" error status.
//
// Note: this may change to an iterator in the future for flexibility purposes.
//
// Users should not use this class directly, but use the type-aliases
// DfsHloVisitor/ConstDfsHloVisitor instead.
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

  pub fn handle_elementwise_unary(&self, _hlo: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_elementwise_binary(&self, _hlo: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_maximum(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_minimum(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_convert(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_bitcast_convert(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_stochastic_convert(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_copy(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_complex(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_multiply(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_power(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_sqrt(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_rsqrt(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_cbrt(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_compare(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_add(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_divide(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_remainder(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_subtract(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_abs(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_atan_2(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_round(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_round_nearest_even(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_erf(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_logistic(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_sign(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_negate(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_exp(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_exp_m1(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_floor(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_ceil(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_log(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_clz(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_log_1p(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_cos(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_sin(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_tan(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_tanh(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_real(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_imag(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_is_finite(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_and(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_not(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handl_or(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_xor(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_population_count(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_shift_left(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_shift_right_arithmetic(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_shift_right_logical(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_binary(hlo)
  }

  pub fn handle_reduce_precision(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
  }

  pub fn handle_domain(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.handle_elementwise_unary(hlo)
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
  // operation on an op after its Handle* visitor method is called. See
  // Preprocess for more details.
  //
  // Overriding methods should call DfsHloVisitor::Postprocess after doing their
  // own postprocessing.
  pub fn postprocess(&self, _hlo: &HloInstruction) -> Result<(), String> {
    Ok(())
  }
}