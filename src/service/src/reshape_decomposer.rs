#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{
  dfs_hlo_visitor_with_default::DfsHloRewriteVisitor,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule
};

// Decomposes a reshape which does not satisfy the ReshapeIsBitcast precondition
// into a bitcast and a copy (physical transposition).
pub struct ReshapeDecomposer {}

impl ReshapeDecomposer {
  pub fn new() -> Self {
    ReshapeDecomposer {  }
  }

  pub fn name(&self) -> String {
    "reshape-decomposer".to_string()
  }

  pub fn run(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    ReshapeDecomposerVisitor::new().run_on_module(module, execution_threads)
  }
}

struct ReshapeDecomposerVisitor {
  changed: bool
}

impl DfsHloRewriteVisitor for ReshapeDecomposerVisitor {
  fn run_on_module(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    for computation in module.make_nonfusion_computations(execution_threads) {
      let status = computation.accept_rewrite_visitor(self);
      if status.is_err() { return Err(status.err().unwrap()); }
    }

    Ok(self.changed)
  }
}

impl ReshapeDecomposerVisitor {
  pub fn new() -> Self {
    ReshapeDecomposerVisitor { changed: false }
  }

  pub fn changed(&self) -> bool {
    self.changed
  }

  pub fn handle_reshape(_reshape: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }
}