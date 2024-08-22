#![allow(dead_code)]

use std::collections::HashSet;

use common::blitz_data::WhileLoopBackendConfig;
use hlo::{hlo_module::HloModule, hlo_opcode::HloOpcode};

use crate::while_loop_analysis::compute_while_loop_trip_count;

// Pass that annotates `while` loops with known trip counts.
//
// The annotation is stored as a backend-config on the while loop node.
//
// This pass should run after all passes that might semantically modify a while
// loop, e.g. by unrolling it.  Otherwise, a loop could end up with a
// backend-config that doesn't match its true trip-count.
//
// This pass does some pattern-matching on loop bodies and conditions, so it
// should run after most HLO simplifications and before fusion and layout
// assignment, which make pattern matching much more difficult by e.g.
// introducing `copy` nodes.
pub struct WhileLoopTripCountAnnotator {}

impl WhileLoopTripCountAnnotator {
  pub fn new() -> Self {
    WhileLoopTripCountAnnotator { }
  }

  pub fn name(&self) -> String {
    "while-loop-trip-count-annotator".to_string()
  }

  pub fn run(
    &mut self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let mut changed = false;
    for comp in module.mutable_computations_by_exec_threads(execution_threads) {
      for instr in comp.mutable_instructions() {
        if instr.opcode() != HloOpcode::While {
          continue;
        }
        let trip_count =
          compute_while_loop_trip_count(instr, 128);
        if trip_count.unwrap() > 0 {
          let mut config = WhileLoopBackendConfig::new();
          config.set_known_trip_count(trip_count.unwrap() as usize);
          instr.set_backend_config(config);
          changed = true;
        }
      }
    }
    Ok(changed)
  }
}