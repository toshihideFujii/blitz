#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{hlo_computation::HloComputation, hlo_module::HloModule};

// Struct that hold states of pass runs across mutiple iterations.
pub struct RunState {
  iteration: usize,
  changed: HashSet<HloComputation>,
  changed_last_iteration: HashSet<HloComputation>,
  changed_this_iteration: HashSet<HloComputation>
}

impl RunState {
  pub fn new() {}
  pub fn increment_iteration(&mut self) {
    // TODO
    self.changed_this_iteration.clear();
    self.iteration += 1;
  }
}

// Base class for HLO passes.
// These are used with the HloPassPipeline to organize a sequence of passes.
pub trait  HloPassInterface{
  fn run(module: &HloModule, execution_threads: &HashSet<String>) -> Result<bool, String>;
}


pub struct HloModulePass {}

impl HloModulePass {
    
}

pub struct HloModuleGroupPass {}

impl HloModuleGroupPass {
    
}