#![allow(dead_code)]

//use crate::hlo_pass_interface::HloPassInterface;

use crate::compilation_stats::CompilationStats;

// Pipeline of HLO passes.
pub struct HloPassPipeline {
  name: String,
  //passes: Vec<Box<dyn HloPassInterface>>
  run_called: bool,
  compilation_stats: CompilationStats
}

impl HloPassPipeline {
  pub fn new(_name: String, _compilation_stats: Option<CompilationStats>) -> Self {
    unimplemented!()
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  // Add a pass to the pipeline. It should be called with the arguments for the
  // pass constructor:
  //
  //   pipeline.AddPass<FooPass>(constructor_arg1, constructor_arg2);
  //
  // Returns a reference to the added pass.
  pub fn add_pass<Arg>(&mut self, _arg: Option<Arg>) {
    assert!(!self.run_called, "add_pass cannot be called after run.");
    unimplemented!()
  }

  pub fn add_invariant_checker() {}
  pub fn run() {}
  pub fn is_pass_pipeline() {}
  pub fn passes_size() {}
  pub fn get_pass() {}
}