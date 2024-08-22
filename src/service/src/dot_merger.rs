#![allow(dead_code)]

use std::collections::HashSet;

use hlo::{
  hlo_computation::HloComputation,
  hlo_module::HloModule
};

// Merges dots that share an operand. Transforms
//     x = dot(a, b)
//     y = dot(a, c)
// into
//     z = dot(a, concat(b, c))
//     x = slice(z)
//     y = slice(z).
pub struct DotMerger {
  max_size_to_merge: i64
}

impl DotMerger {
  pub fn new(maz_size_to_merge: i64) -> Self {
    DotMerger { max_size_to_merge: maz_size_to_merge }
  }

  pub fn name() -> String { "dot-merger".to_string() }

  pub fn run(
    &self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let mut changed = false;
    for comp in module.make_nonfusion_computations(execution_threads) {
      let result = self.merge_dots(comp);
      if result.is_err() { return Err(result.err().unwrap()); }
      changed |= result.ok().unwrap();
    }
    Ok(changed)
  }

  fn merge_dots(&self, _comp: &HloComputation) -> Result<bool, String> {
    unimplemented!()
  }
}