#![allow(dead_code)]

use std::collections::HashSet;

use crate::hlo_module::HloModule;

// Base class for HLO passes.
// These are used with the HloPassPipeline to organize a sequence of passes.
pub trait HloPassInterface {
  // Run the pass on the given HLOmodule with specified execution_threads.
  fn run(module: &HloModule, execution_threads: HashSet<String>) -> Result<(), String>;
}


pub struct HloModulePass {}

impl HloModulePass {
    
}

pub struct HloModuleGroupPass {}

impl HloModuleGroupPass {
    
}