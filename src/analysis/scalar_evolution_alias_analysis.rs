#![allow(dead_code)]

// This is the interface for a SCEV-based alias analysis.

struct SCEVAAResult {}
impl SCEVAAResult {
  pub fn new() {}
  pub fn alias() {}
  pub fn invalidate() {}
}

struct SCEVAA {}
impl SCEVAA {
  pub fn new() {}
  pub fn run() {}
}

struct SCEVAAWrapperPass {}
impl SCEVAAWrapperPass {
  pub fn new() {}
  pub fn get_result() {}
  pub fn run_on_function() {}
  pub fn get_analysis_usage() {}
}

pub fn create_scevaa_wrapper_pass() {}