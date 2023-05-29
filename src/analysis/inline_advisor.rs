#![allow(dead_code)]

struct InlineAdvice {}
impl InlineAdvice {
  pub fn new() {}
  pub fn record_inlining() {}
  pub fn record_inlining_with_callee_deleted() {}
  pub fn record_unsuccessful_inlining() {}
  pub fn record_unattempted_inlining() {}
  pub fn is_inlining_recommended() {}
  pub fn get_original_call_site_debug_loc() {}
  pub fn get_original_call_site_basic_block() {}

  pub fn record_inlining_impl() {}
  pub fn record_inlining_with_callee_deleted_impl() {}
  pub fn record_unsuccessful_inlining_impl() {}
  pub fn record_unattempted_inlining_impl() {}

  fn mark_recorded() {}
  fn record_inline_stats_if_needed() {}
}

struct DefaultInlineAdvice {}
impl DefaultInlineAdvice {
  pub fn new() {}
  fn record_unsuccessful_inlining_impl() {}
  fn record_inlining_with_callee_deleted_impl() {}
  fn record_inlining_impl() {}
}

struct InlineAdvisor {}
impl InlineAdvisor {
  pub fn new() {}
  pub fn get_advice() {}
  pub fn on_pass_entry() {}
  pub fn on_pass_exit() {}
  pub fn print() {}
  pub fn get_annotated_inline_pass_name() {}
  pub fn get_mandatory_kind() {}
  pub fn get_caller_ore() {}
}

struct DefaultInlineAdvisor {}
impl DefaultInlineAdvisor {
  pub fn new() {}
  pub fn get_advice_impl() {}
}

struct PluginInlineAdvisorAnalysis {}
impl PluginInlineAdvisorAnalysis {
  pub fn new() {}
  pub fn run() {}
  pub fn get_result() {}
}

struct InlineAdvisorAnalysis {}
impl InlineAdvisorAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct InlineAdvisorAnalysisPrinterPass {}
impl InlineAdvisorAnalysisPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

pub fn get_release_mode_advisor() {}
pub fn get_develop_mode_advisor() {}
pub fn should_inline() {}
pub fn emit_inlined_into() {}
pub fn emit_inlined_into_nesed_on_cost() {}
pub fn add_location_to_remark() {}
pub fn set_inline_remark() {}
pub fn inline_cost_str() {}