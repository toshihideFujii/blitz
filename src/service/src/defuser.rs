#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

//use crate::call_graph::CallGraph;

// A pass which replaces all jfusion instructions with the equivalent
// un-fused instructions.
pub struct Defuser {}

impl Defuser {
  pub fn new() {}
  pub fn name() -> String { "defuser".to_string() }

  pub fn run(
    &self,
    module: &HloModule,
    _execution_threads: HashSet<String>) -> Result<bool, String>
  {
    println!("Defusing module {:?}", module.name());
    println!("Before defusion:");
    println!("{:?}", module.to_string());

    let changed = false;
    //let call_graph = CallGraph::build(module, None);


    println!("After defusion:");
    println!("{:?}", module.to_string());
    Ok(changed)
  }
}