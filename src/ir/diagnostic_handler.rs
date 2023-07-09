#![allow(dead_code)]

// Base DiagnosticHandler class declaration.
// Derive from this class to provide custom diagnostic reporting.

use crate::adt::string_ref::StringRef;

#[derive(Debug, Clone, PartialEq)]
pub struct DiagnositicHandler {}

impl DiagnositicHandler {
  pub fn new() -> Self {
    DiagnositicHandler {}
  }

  pub fn handle_diagnostics() {}

  // Return true if analysis remarks are enebled, override to provide
  // deifferent implementation.
  pub fn is_analysis_remark_enabled(&self, _pass_name: &StringRef) -> bool {
    false
  }

  // Return true if missed optimization remarks are enabled, override
  // to provide different implementation.
  pub fn is_missed_opt_remark_enabled(&self, _pass_name: &StringRef) -> bool {
    false
  }

  // Return true if passed optimization remarks are enabled, override
  // to provide different implementation.
  pub fn is_passed_opt_remark_enabled(&self, _pass_name: &StringRef) -> bool {
    false
  }

  // Return true if any type of remarks are enabled for this pass.
  pub fn is_any_remark_enabled(&self, pass_name: &StringRef) -> bool {
    self.is_missed_opt_remark_enabled(pass_name) ||
    self.is_passed_opt_remark_enabled(pass_name) ||
    self.is_analysis_remark_enabled(pass_name)
  }
}