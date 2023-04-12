#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub struct DiagnositicHandler {}

impl DiagnositicHandler {
  pub fn handle_diagnostics() {}
  pub fn is_analysis_remark_enabled() {}
  pub fn is_missed_opt_remark_enabled() {}
  pub fn is_passed_opt_remark_enabled() {}
  pub fn is_any_remark_enabled() {}
}