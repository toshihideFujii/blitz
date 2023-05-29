#![allow(dead_code)]

// This is the interface for a simple mod/ref and alias analysis
// over globals.

struct GlobalsAAResult {}
impl GlobalsAAResult {
  pub fn new() {}
  pub fn invalidate() {}
  pub fn analyze_module() {}
  pub fn alias() {}
  pub fn get_mod_ref_info() {}
  pub fn get_memory_effects() {}

  fn get_function_info() {}
  fn analyze_globals() {}
  fn analyze_call_graph() {}
  fn analyze_uses_of_pointer() {}
  fn analyze_indirect_global_memory() {}
  fn collect_sccm_membership() {}
  fn is_non_escaping_global_no_alias() {}
  fn get_mod_ref_info_for_arjgument() {}
}

struct GlobalsAA {}
impl GlobalsAA {
  pub fn new() {}
  pub fn run() {}
}

struct RecomputeGlobalsAAPass {}
impl RecomputeGlobalsAAPass {
  pub fn new() {}
  pub fn run() {}
}

struct GlobalsAAWrapperPass {}
impl GlobalsAAWrapperPass {
  pub fn new() {}
  pub fn get_result() {}
  pub fn run_on_module() {}
  pub fn do_finalization() {}
  pub fn get_analysis_usage() {}
}

pub fn create_globals_aa_wrapper_pass() {}