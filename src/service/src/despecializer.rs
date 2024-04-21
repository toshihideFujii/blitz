#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// Creates an HloPassPipeline containing mutiple HloPasses that can
// despecialize an optimized HloModule.
pub struct Despecializer {}

impl Despecializer {
  pub fn new() {}
  pub fn name() -> String { "despecializer".to_string() }
  pub fn run() {}
}

pub struct DeconstructReduceWindowToReduceBroadcast {}

impl DeconstructReduceWindowToReduceBroadcast {
  pub fn new() {}
  pub fn name() -> String {
    "reduce-window-to-reduce-and-broadcast".to_string()
  }
  pub fn run() {}
}

// Pass which strips control dependencies from all instructions in the module.
pub struct ControlDepRemover {}

impl ControlDepRemover {
  pub fn new() -> Self { ControlDepRemover {  } }

  pub fn name() -> String { "control-dep-remover".to_string() }

  pub fn run(
    module: &mut HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let mut changed = false;
    for computation in module.mutable_computations() {
      for instruction in computation.mutable_instructions() {
        changed |= !instruction.control_successors().is_empty();
        let result = instruction.drop_all_control_deps();
        if result.is_err() { return Err(result.err().unwrap()); }
      }
    }
    Ok(changed)
  }
}