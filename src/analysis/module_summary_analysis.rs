#![allow(dead_code)]

// This is the interface to build a ModuleSummaryIndex for a module.

pub fn build_module_summary_index() {}

struct ModuleSummaryIndexAnalysis {}
impl ModuleSummaryIndexAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct  ModuleSummaryIndexWrapperPass {}
impl ModuleSummaryIndexWrapperPass {
  pub fn new() {}
  pub fn get_index() {}
  pub fn run_on_module() {}
  pub fn do_finalization() {}
  pub fn get_analysis_usage() {}
}

pub fn create_module_summary_index_wrapper_pass() {}

struct ImmutableModuleSummaryIndexWrapperPass {}
impl ImmutableModuleSummaryIndexWrapperPass {
  pub fn new() {}
  pub fn get_index() {}
  pub fn get_analysis_usage() {}
}

pub fn create_immutable_module_summary_index_wrapper_pass() {}