#![allow(dead_code)]

// This class is used to collect information about HLO passes
// and print some statistics at the end of compilation.
struct CompilationStats {}

impl CompilationStats {
  pub fn make_noop_stats() {}

  pub fn make_stats() {}

  pub fn start_pass() {}

  pub fn end_pass() {}

  pub fn compilation_report() {}

  pub fn get_passes_size() {}

  pub fn record_pass_error() {}
}