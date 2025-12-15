#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use common::{shape_tree::ShapeTree};

use crate::{dfs_hlo_visitor_with_default::DfsHloVisitorWithDefault,
  hlo_computation::HloComputation,hlo_instruction::HloInstruction, hlo_module::HloModule};

#[derive(Debug, Clone)]
pub enum WeightInfo {
  Weight,
  Tuple,
  Unknown,
}

// This analysis pass determines which HLO instructions produce/are weights.
// Parameters to the entry computation are considered weights, and this property
// is propagated through instructions that preserve it (slice, convert,
// etc).
pub struct HloDimensionAnalysis<'module> {
  info_map: HashMap<HloInstruction, ShapeTree<WeightInfo>>,
  module: &'module HloModule,
  execution_threads: HashSet<String>
}

impl<'module> HloDimensionAnalysis<'module> {
  pub fn new(module: &'module HloModule, execution_threads: HashSet<String>) -> Self {
    HloDimensionAnalysis {
      info_map: HashMap::new(),
      module: module,
      execution_threads: execution_threads
    }
  }

  // Whether the instruction has been annotated with weight info.
  pub fn has_weight_info(&self, instruction: &HloInstruction) -> bool {
    self.info_map.contains_key(instruction)
  }

  // Whether any leaf in the instruction shape is a weight.
  pub fn is_instruction_weight(&self, _instruction: &HloInstruction) -> bool {
    unimplemented!()
  }

  // Returns map of HLO instructions to their weight info.
  // If an instruction is not found in the map, it means that we have not
  // determined it is a weight.
  pub fn get_weight_info_map(
    &self) -> &HashMap<HloInstruction, ShapeTree<WeightInfo>>
  {
    &self.info_map
  }

  // Returns the weight info for the given instruction.
  pub fn get_weight_info(
    &self, instruction: &HloInstruction) -> Option<&ShapeTree<WeightInfo>>
  {
    self.info_map.get(instruction)
  }

  // Sets the instruction as a weight. This is used to annotate the entry
  // computation parameters and other instructions that are known to be
  // weights.
  fn set_instruction_as_weight(
    &self, _instruction: &HloInstruction) -> Result<(), String>
  {
    unimplemented!()
  }

  // Sets the weight info for the given target instruction.
  fn set_weight_info(
    &mut self,
    target: &HloInstruction,
    weight_annotation: ShapeTree<WeightInfo>) -> Result<(), String>
  {
    if self.has_weight_info(target) {
      return Err("Instruction already has weight info".to_string());
    }
    self.info_map.insert(target.clone(), weight_annotation);
    Ok(())
  }

  // Annotates the entry computation parameters as weights.
  fn annotate_entry_computation_parameters(
    &self, module: &HloModule) -> Result<(), String>
  {
    let params =
      module.entry_computation().unwrap().parameter_instructions();
    for instruction in params {
      let result = self.set_instruction_as_weight(instruction);
      check_error(&result);
    }
    Ok(())
  }

  pub fn run(
    &'module mut self,
    module: &'module HloModule,
    execution_threads: HashSet<String>) -> Self
  {
    let weight_analysis =
      HloDimensionAnalysis::new(module, execution_threads);

    let mut result =
      weight_analysis.annotate_entry_computation_parameters(module);
    check_error(&result);

    result = self.run_on_computation(module.entry_computation().unwrap());
    check_error(&result);
    weight_analysis
  }

  // Runs the weight analysis on the given computation.
  fn run_on_computation(
    &'module mut self, computation: &HloComputation) -> Result<(), String>
  {
    if HloInstruction::is_thread_included(
      computation.execution_thread(), &self.execution_threads)
    {
      let propagation = HloWeightPropagation::new(self);
      return propagation.run(computation);
    }
    Ok(())
  }

  fn run_on_computation_by_operands(
    &'module mut self,
    computation: &HloComputation,
    operands: &Vec<HloInstruction>) -> Result<(), String>
  {
    debug_assert_eq!(computation.num_parameters(), operands.len());
    for i in 0..computation.num_parameters() {
      let weight_info =
        self.info_map.get(&operands[i]);
      if weight_info.is_none() { continue; }
      let result = self.set_weight_info(
        &computation.parameter_instructions()[i],
        weight_info.unwrap().clone());
      check_error(&result);
    }
    self.run_on_computation(computation)
  }
}

pub struct HloWeightPropagation<'module> {
  base: DfsHloVisitorWithDefault,
  analysis: &'module mut HloDimensionAnalysis<'module>
}

impl<'module> HloWeightPropagation<'module> {
  pub fn new(analysis: &'module mut HloDimensionAnalysis<'module>) -> Self {
    HloWeightPropagation {
      base: DfsHloVisitorWithDefault::new(),
      analysis: analysis
    }
  }

  pub fn run(&self, computation: &HloComputation) -> Result<(), String> {
    computation.root_instruction().accept_visitor(&self.base);
    for instruction in computation.instructions() {
      if instruction.user_count() == 0 {
        instruction.accept_visitor(&self.base);
      }
    }
    Ok(())
  }

  pub fn default_action(&self, _instruction: &HloInstruction) -> Result<(), String> {
    Ok(())
  }

  pub fn handle_tuple(&self, _tuple: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_get_tuple_element(
    &self, get_tuple_element: &HloInstruction) -> Result<(), String>
  {
    let operand = get_tuple_element.operand(0);
    if self.analysis.is_instruction_weight(operand) {
      // TODO
      //let weight_tree = ShapeTree::new(shape)
    }
    Ok(())
  }

  pub fn handle_call(&'module mut self, call: &HloInstruction) -> Result<(), String> {
    if self.return_if_already_propagated(call).is_err() {
      return Err("".to_string());
    }
    let computation = &call.called_computations()[0];
    let result = self.analysis.run_on_computation_by_operands(
      computation, call.operands());
    check_error(&result);

    //if self.analysis.is_instruction_weight(computation.root_instruction()) {
      //self.analysis.set_weight_info(target);
    //}
    Ok(())
  }

  pub fn handle_while(&'module mut self, _blitz_while: &HloInstruction) -> Result<(), String> {
    /*
    let mut result = self.analysis.run_on_computation_by_operands(
      blitz_while.while_condition(), blitz_while.operands());
    check_error(&result);
    
    let computation = blitz_while.while_body();
    result = self.analysis.run_on_computation_by_operands(computation, blitz_while.operands());
    check_error(&result);

    if self.analysis.is_instruction_weight(computation.root_instruction()) {
      result = self.analysis.set_weight_info(
        blitz_while,
        self.analysis.get_weight_info(
          computation.root_instruction()).unwrap().clone());
      check_error(&result);
    }
    */
    Ok(())
  }

  pub fn handle_simple_op(&self, op: &HloInstruction) -> Result<(), String> {
    let result =
      self.return_if_already_propagated(op);
    check_error(&result);
    let operand = op.operand(0);
    if self.analysis.is_instruction_weight(operand) {
      let result =
        self.analysis.set_instruction_as_weight(op);
      check_error(&result);
    }
    Ok(())
  }

  pub fn handle_dynamic_slice(
    &self, dynamic_slice: &HloInstruction) -> Result<(), String>
  {
    self.handle_simple_op(dynamic_slice)
  }

  pub fn handle_dynamic_update_slice(
    &self, dynamic_update_slice: &HloInstruction) -> Result<(), String>
  {
    let result = self.return_if_already_propagated(
      dynamic_update_slice);
    check_error(&result);

    // If either the operand or the update is a weight, we consider the output to
    // be a weight.
    let operand = dynamic_update_slice.operand(0);
    let update = dynamic_update_slice.operand(1);
    if self.analysis.is_instruction_weight(operand) ||
      self.analysis.is_instruction_weight(update)
    {
      let result =
        self.analysis.set_instruction_as_weight(dynamic_update_slice);
      check_error(&result);
    }
    Ok(())
  }

  pub fn handle_slice(&self, slice: &HloInstruction) -> Result<(), String> {
    self.handle_simple_op(slice)
  }

  pub fn handle_convert(&self, convert: &HloInstruction) -> Result<(), String> {
    self.handle_simple_op(convert)
  }

  pub fn handle_reshape(&self, reshape: &HloInstruction) -> Result<(), String> {
    self.handle_simple_op(reshape)
  }

  pub fn handle_bitcast(&self, bitcast: &HloInstruction) -> Result<(), String> {
    self.handle_simple_op(bitcast)
  }

  pub fn handle_transpose(&self, transpose: &HloInstruction) -> Result<(), String> {
    self.handle_simple_op(transpose)
  }

  pub fn handle_copy(&self, copy: &HloInstruction) -> Result<(), String> {
    self.handle_simple_op(copy)
  }

  pub fn handle_bitcast_convert(
    &self, bitcast_convert: &HloInstruction) -> Result<(), String>
  {
    self.handle_simple_op(bitcast_convert)
  }

  pub fn handle_optimization_barrier(
    &mut self, optimization_barrier: &HloInstruction) -> Result<(), String>
  {
    let result = self.return_if_already_propagated(
      optimization_barrier);
    check_error(&result);

    debug_assert_eq!(optimization_barrier.operand_count(), 1);
    let optimization_barrier_operand = optimization_barrier.operand(0);
    if self.analysis.is_instruction_weight(optimization_barrier_operand) {
      let result = self.analysis.set_weight_info(
        optimization_barrier,
        self.analysis.get_weight_info(
          optimization_barrier_operand).unwrap().clone());
      check_error(&result);
    }
    Ok(())
  }

  fn return_if_already_propagated(
    &self, instruction: &HloInstruction) -> Result<(), String>
  {
    if self.analysis.has_weight_info(instruction) {
      return Ok(());
    }
    Err("instruction is not propagated".to_string())
  }
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}