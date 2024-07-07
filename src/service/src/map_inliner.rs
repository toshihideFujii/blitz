#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{dfs_hlo_visitor_with_default::DfsHloVisitor, hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode, utils::hlo_query::all_operands_are_parameters};

struct MapInlinerVisitor {
  computation: Option<HloComputation>,
  changed: bool
}

impl MapInlinerVisitor {
  pub fn new(computation: Option<HloComputation>) -> Self {
    MapInlinerVisitor {
      computation: computation,
      changed: false
    }
  }

  pub fn default_action(&self, _instruction: &HloInstruction) -> Result<(), String> {
    Ok(())
  }

  pub fn handle_map(&mut self, map: &mut HloInstruction) -> Result<(), String> {
    let function = map.to_apply();
    let root = function.root_instruction();

    if all_operands_are_parameters(root) {
      if root.opcode() == HloOpcode::Fusion {
        // Cloning not supported for these instructions.
        return Ok(());
      }
      println!("inlining map(X ... Y, op) => : op(X ... Y) with function {:?}",
        root.to_short_string());
      if root.opcode() == HloOpcode::Parameter {
        // If the root is a parameter, then use the corresponding operand as the
        // result of the computation.
        let mut new_producer =
          map.mutable_operand(root.parameter_number() as usize).unwrap().clone();
        let result = map.replace_all_uses_with(
          &mut new_producer, "".to_string());
        if result.is_err() {
          return Err(result.err().unwrap());
        }
      } else if root.opcode() == HloOpcode::Constant {
        /*
        let constant =
          self.computation.as_mut().unwrap().add_instruction(root.clone(), "".to_string());
        let placed_instruction =
          self.computation.as_mut().unwrap().add_instruction(
            HloInstruction::create_broadcast(
              map.shape(), constant.clone(), vec![]), "".to_string());
        let result =
          self.computation.as_mut().unwrap().replace_instruction(
            &map, &placed_instruction, false, false, true);
        if result.is_err() {
          return Err(result.err().unwrap());
        }
        */
      } else {
        // TODO
      }

      self.changed = true;
      return Ok(());
    }

    Ok(())
  }

  // Runs the visitor on a computation.
  pub fn run(&mut self, computation: &HloComputation) -> Result<bool, String> {
    self.changed = false;
    self.computation = Some(computation.clone());
    let result =
      self.computation.as_ref().unwrap().root_instruction()
        .accept(self, true, false, false);
    if result.is_err() { return Err(result.err().unwrap()); }
    Ok(true)
  }
}

impl DfsHloVisitor for MapInlinerVisitor {}

// A pass which performs map inlining.
pub struct MapInliner {}

impl MapInliner {
  pub fn new() -> Self {
    MapInliner {  }
  }

  pub fn name(&self) -> String {
    "map-inline".to_string()
  }

  pub fn run(
    &self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let mut visitor = MapInlinerVisitor::new(None);
    let mut changed = false;
    
    for computation in module.computations_by_exec_threads(execution_threads) {
      let computation_changed = visitor.run(computation);
      if computation_changed.is_err() {
        return Err(computation_changed.err().unwrap());
      }
      changed |= computation_changed.ok().unwrap();
    }

    Ok(changed)
  }
}