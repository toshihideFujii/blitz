#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

// Remove Sharding custom-call instruction by assigning its users
// to its operand. This is helpful when partition_count == 1.
pub struct ShardingRemover {}

impl ShardingRemover {
  pub fn new() -> Self {
    ShardingRemover {  }
  }

  pub fn name(&self) -> String {
    "sharding-remover".to_string()
  }

  pub fn run(
    &mut self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }
}