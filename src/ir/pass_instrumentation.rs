#![allow(dead_code)]

struct PassInstrumentationCallbacks {}

struct PassInstrumentation {}

impl PassInstrumentation {
  pub fn new() {}
  pub fn run_before_pass() {}
  pub fn run_after_pass() {}
  pub fn run_after_pass_invalidated() {}
  pub fn run_before_analysis() {}
  pub fn run_after_analysis() {}
  pub fn run_analysis_invalidated() {}
  pub fn run_analyses_cleared() {}
  pub fn invalidate() {}
  pub fn push_before_non_skipped_pass_callback() {}
  pub fn pop_before_non_skipped_pass_callback() {}
  pub fn is_special_pass() {}
}