#![allow(dead_code)]

// This file contains a pass that provides access to profile
// summary information.

struct ProfileSummaryInfo {}
impl ProfileSummaryInfo {
  pub fn new() {}
  pub fn refresh() {}
  pub fn has_profile_summary() {}
  pub fn has_sample_profile() {}
  pub fn has_instrumentation_profile() {}
  pub fn invalidate() {}
  pub fn get_profile_count() {}
  pub fn has_partial_sample_profile() {}
  pub fn has_huge_working_set_size() {}
  pub fn has_large_working_set_size() {}
  pub fn is_function_entry_hot() {}
  pub fn is_function_hot_in_call_graph() {}
  pub fn is_function_entry_cold() {}
  pub fn is_function_cold_in_call_graph() {}
  pub fn is_function_hotness_unknown() {}
  pub fn is_function_hot_in_call_graph_nth_percentile() {}
  pub fn is_function_cold_in_call_graph_nth_percentile() {}
  pub fn is_hot_count() {}
  pub fn is_cold_count() {}
  pub fn is_hot_count_nth_percentile() {}
  pub fn is_cold_count_nth_percentile() {}
  pub fn is_hot_block() {}
  pub fn is_cold_block() {}
  pub fn is_hot_block_nth_percentile() {}
  pub fn is_cold_block_nth_percentile() {}
  pub fn is_hot_call_site() {}
  pub fn is_cold_call_site() {}
  pub fn get_or_comp_hot_count_threshold() {}
  pub fn get_or_comp_cold_count_threshold() {}
  pub fn get_hot_count_threshold() {}
  pub fn get_cold_count_threshold() {}
}

struct ProfileSummaryInfoWrapperPass {}
impl ProfileSummaryInfoWrapperPass {
  pub fn new() {}
  pub fn get_psi() {}
  pub fn do_initialization() {}
  pub fn do_finalization() {}
  pub fn get_analysis_usage() {}
}

struct ProfileSummaryAnalysis {}
impl ProfileSummaryAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct ProfileSummaryPrinterPass {}
impl ProfileSummaryPrinterPass {
  pub fn new() {}
  pub fn run() {}
}