#![allow(dead_code)]

use std::collections::HashMap;

use common::literal::Literal;

use crate::{hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule};

// Responsible for evaluating HLO and obtain literal as the evaluation results.
// This class is not thread-safe.
pub struct HloEvaluator<T> where T: Clone + Default + PartialEq + 'static {
  arg_literals: Vec<Literal<T>>,
  max_loop_iterations: i64,
  seed: u64
}

impl<T> HloEvaluator<T> where T: Clone + Default + PartialEq + 'static {
  pub fn default() -> Self {
    HloEvaluator {
      arg_literals: Vec::new(),
      max_loop_iterations: 0,
      seed: 0
    }    
  }

  // Only evaluate up to max_loop_iterations per while-loop execution if
  // specified.
  pub fn new(max_loop_iterations: i64) -> Self {
    HloEvaluator {
      arg_literals: Vec::new(),
      max_loop_iterations: max_loop_iterations,
      seed: 0
    }
  }

  pub fn create_embedded() {}
  pub fn on_evaluate_computation() {}

  // Gets the value of running a single HLO instruction.
  //
  // This function may recursively evaluate the dependency of this instruction
  // within its parent computation until it encounters something that cannot be
  // evaluated, such as an Infeed or a Parameter instruction.
  // It makes best effort to partially evaluate a dependency if possible.
  pub fn evaluate(
    &self,
    _instruction: &HloInstruction,
    _recursively_evaluate_nonconstant_operands: bool) -> Result<Literal<T>, String>
  {
    unimplemented!()
  }

  pub fn evaluate_module(&self, _module: &HloModule) -> Result<Literal<T>, String> {
    unimplemented!()
  }

  // Evaluates an HLO computation and an array of pointers to literals.
  // Returns the evaluated result as a literal if successful.
  // Precondition: The indices of arg_literals correspond to the parameter
  // numbers of the HLO parameters in the computation. For e.g., consider the
  // following graph:
  //
  //                *
  //            /       \
  //            +     Parameter1
  //        /      \
  //       /        \
  //    Parameter0  Constant
  //
  // where Parameter0 has parameter_number 0 and Parameter1 has parameter_number
  // 1 in this computation. The input literals array will then have its first
  // literal map to Parameter0 and the second map to Parameter1.
  //
  // (Dummy template arg is to reduce the overloading priority of one overload
  // so that Evaluate(module, {}) resolves unambiguously.)
  pub fn evaluate_computation(
    &self,
    _computation: &HloComputation,
    _arg_literals: &Vec<Literal<T>>) -> Result<Literal<T>, String>
  {
    unimplemented!()    
  }

  pub fn try_evaluate() {}

  // Evaluates a single HLO instruction, substituting the given literals for
  // some of the instruction's operands.
  //
  // For example, given instruction = op(A, B, C) and the map
  // {A = x, C = y}, this evaluates op(x, B, y).
  pub fn evaluate_with_substitutions(
    &self,
    _instruction: &HloInstruction,
    _substitutions: HashMap<HloInstruction, Literal<T>>) -> Result<Literal<T>, String>
  {
    unimplemented!()
  }

  pub fn evaluate_elementwise_binary_op() {}
  pub fn evaluate_elementwise_unary_op() {}
  pub fn evaluate_elementwise_ternary_op() {}
  pub fn evaluate_elementwise_compare_op() {}

  pub fn set_dynamic_dimension_inference(&mut self) {
      
  }
  pub fn dynamic_dimension_inference() {}
  pub fn set_use_fast_path() {}
  pub fn set_cusstom_call_handler() {}
}