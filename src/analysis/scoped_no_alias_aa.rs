#![allow(dead_code)]

// This is the interface for a metadata-based scoped no-alias analysis.

struct ScopedNoAliasAAResult {}
impl ScopedNoAliasAAResult {
  pub fn new() {}
  pub fn invalidate() {}
  pub fn alias() {}
  pub fn get_mod_ref_info() {}
}

struct ScopedNoAliasAA {}
impl ScopedNoAliasAA {
  pub fn new() {}
  pub fn run() {}
}

struct ScopedNoAliasAAWrapperPass {}
impl ScopedNoAliasAAWrapperPass {
  pub fn new() {}
  pub fn get_result() {}
  pub fn do_initialization() {}
  pub fn do_finalization() {}
  pub fn get_analysis_usage() {}
}

pub fn create_scoped_no_alias_aa_wrapper_pass() {}