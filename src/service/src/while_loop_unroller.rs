#![allow(dead_code)]

use std::collections::HashSet;

use common::shape::Shape;
use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule};

// Config for unrollable while loops.
struct WhileLoopConfig {
  init: i64,
  trip_count: i64,
  induction_var_idx: i64
}

// Runs a sequence of passes that are necessary to prepare loops for unrolling.
// Failure to run these passes will prevent unroller from unrolling loops that
// would have been otherwise unrollable.
fn prepare_module_for_unrolling(
  _module: &HloModule,
  _execution_threads: &HashSet<String>) -> Result<bool, String>
{
  unimplemented!()  
}

// Unrolls the given while loop with the default behaviour set to full unroll.
// If wrap_in_trivial_loop is set, the unrolled body of the loop will be wrapped
// in a loop with trip count of one.
pub fn unroll(
  _while_op: &HloInstruction,
  _unroll_factor: i64,
  _wrap_in_trivial_loop: bool) -> Result<bool, String>
{
  unimplemented!()    
}

// Parameters for the unroller that can be adjusted.
const UNROLL_TRIP_COUNT_THRESHOLD: usize = 64;
const UNROLL_INSTRUCTION_COUNT_THRESHOLD: usize = 800;
const UNROLL_EXPAND_FACTOR_THRESHOLD: usize = 10000;

// This pass unrolls while loops with the given unrolling factor. The value of
// unroll_factor = -1 will fully unroll the loop.
//
// TODO(b/288130138): Currently, we `only` support full unrolling.
//
// The trip count for loops is calculated based on
// `MatchTrivialLoopTripCount` function in
// tensorflow/compiler/xla/service/while_loop_analysis.h`
//
// TODO(b/301472793): Add utility functions to unroll specific loops.
pub struct WhileLoopUnroller {
  unroll_factor: i64,
  wrap_in_trivial_loop: bool
}

impl WhileLoopUnroller {
  pub fn new(unroll_factor: i64, wrap_in_trivial_loop: bool) -> Self {
    WhileLoopUnroller {
      unroll_factor: unroll_factor,
      wrap_in_trivial_loop: wrap_in_trivial_loop 
    }
  }

  pub fn name(&self) -> String {
    "while-loop-unroller".to_string()
  }

  #[allow(unused_assignments)]
  pub fn run(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    if self.unroll_factor != -1 {
      return Ok(false);
    }
    println!("WhileLoopUnroller::run(), before: {:?}", module.to_string());
    let mut changed = false;

    // Make sure all the necessary passes are executed before unrolling in order
    // to unroll every possible loop.
    let result = prepare_module_for_unrolling(module, execution_threads);
    if result.is_err() {
      return Err(result.err().unwrap());
    }
    changed = result.unwrap();

    // Processing the while loops in the reverse of topological order. If the body
    // of while loop A calls while loop B, B comes before A.
    for _comp in
      module.make_computation_post_order(execution_threads, false) {
      // TODO
    }

    // Gather a preliminary vector of all the while ops that we think we can
    // unroll. We do this ahead of time so we don't have to worry about mutating
    // the lists of computations or instructions while we iterate.
    let unrollable_while_ops =
      get_unrollable_loops(module, execution_threads);

    println!("Number of while instructions in the module to unroll: {:?}",
      unrollable_while_ops.len());

    for unrollable_while_op in &unrollable_while_ops {
      let mut result: Result<bool, String> = Ok(false);
      if self.wrap_in_trivial_loop {
        result = unroll_internal_wrapped(
          &unrollable_while_op.0, &unrollable_while_op.1, self.unroll_factor);
      } else {
        result = unroll_internal(
          &unrollable_while_op.0, &unrollable_while_op.1, self.unroll_factor);
      }
      if result.is_err() { return Err(result.err().unwrap()); }
      changed |= result.unwrap()
    }
    
    // We need to inline the calls created for unrolling since later passes rely
    // on the calls to be inlined.
    if changed {
      // TODO
    }

    println!("WhileLoopUnroller::run(), after: {:?}", module.to_string());
    Ok(changed)
  }
}

// A utility function that decides whether a loop is unrollable or not.
fn is_loop_unrollable(_while_op: &HloInstruction) -> Option<WhileLoopConfig> {
  unimplemented!()    
}

fn get_constant_with_shape(_shape: &Shape, _value: i64) -> HloInstruction {
  unimplemented!()
}

// Helper function to create a condition for a single iteration while loop in
// the form of 'i <= init_value' where i is the induction variable.
fn make_trivial_loop_condition(
  _while_op: &HloInstruction,
  _name: String,
  _induction_idx: i64,
  _init_value: i64) -> HloComputation
{
  unimplemented!()    
}

// Helper function that replaces a single iteration of a while loop with
// induction variable equal to induction_value.
fn unroll_single_iteration_of_trivial_loop(
  _while_op: &HloInstruction,
  _config: &WhileLoopConfig,
  _induction_value: i64) -> Result<HloComputation, String>
{
  unimplemented!()    
}

fn initial_feasibility_check(
  _while_op: &HloInstruction,
  _config: &WhileLoopConfig,
  _unroll_factor: i64) -> Result<(), String>
{
  unimplemented!()    
}

fn unroll_internal(
  _while_op: &HloInstruction,
  _config: &WhileLoopConfig,
  _unroll_factor: i64) -> Result<bool, String>
{
  unimplemented!()    
}

fn unroll_internal_wrapped(
  _while_op: &HloInstruction,
  _config: &WhileLoopConfig,
  _unroll_factor: i64) -> Result<bool, String>
{
  unimplemented!()    
}

fn get_unrollable_loops(
  _module: &HloModule,
  _execution_threads: &HashSet<String>) -> Vec<(HloInstruction, WhileLoopConfig)>
{
  unimplemented!()    
}