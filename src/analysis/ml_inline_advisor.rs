#![allow(dead_code)]

struct MLInlineAdvisor {}
impl MLInlineAdvisor {
  pub fn new() {}
  pub fn on_pass_entry() {}
  pub fn on_pass_exit() {}
  pub fn get_ir_size() {}
  pub fn on_successful_inlining() {}
  pub fn is_forced_to_stop() {}
  pub fn get_local_calls() {}
  pub fn get_model_runner() {}
  pub fn get_cached_fpi() {}
  pub fn get_advice_impl() {}
  pub fn get_mandatory_advice() {}
  pub fn get_advice_form_model() {}
  pub fn get_initial_function_level() {}
}

struct MLInlineAdvice {}
impl MLInlineAdvice {
  pub fn new() {}
  pub fn record_inlining_impl() {}
  pub fn record_inlining_with_callee_deleted_impl() {}
  pub fn record_unsuccessful_inlining_impl() {}
  pub fn record_unattempted_inlining_impl() {}
  pub fn get_caller() {}
  pub fn get_callee() {}
  pub fn update_cached_caller_fpi() {}
}