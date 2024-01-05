#![allow(dead_code)]

pub struct Diagnostic {}

impl Diagnostic {
  pub fn new() {}
  pub fn status() {}
}

pub struct InFlightDiagnostic {}

impl InFlightDiagnostic {
  pub fn new() {}
}

pub struct DiagnosticEngine {}

impl DiagnosticEngine {
  pub fn new() {}
  pub fn emit_error() {}
  pub fn add_handler() {}
  pub fn emit() {}
}