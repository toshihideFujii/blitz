#![allow(dead_code)]

// This file defines the function verifier interface, that can be
// used for validation checking of input to the system, and for checking
// that transformations haven't done something bad.

#[derive(Debug, Clone)]
struct VerifierSupport {}

// Verufy that the TBAA metadatas are valid.
#[derive(Debug, Clone)]
pub struct TBAAVerifier {
  diagnostic: VerifierSupport
}

impl TBAAVerifier {
  pub fn new() {}
  pub fn visit_tbaa_metadata() {}
  pub fn get_field_node_from_tbaa_base_node() {}
  pub fn verify_tbaa_base_node() {}
  pub fn is_valid_scalar_tbaa_node() {}
}

// Check a function for errors, useful for use when debugging a pass.
pub fn verify_function() {}

// Check a module for errors.
pub fn verify_module() {}

pub fn create_verifier_pass() {}

// Check a module for errors, and report separate error states for IR
// and debug info errors.
struct VerifierAnalysis {}
impl VerifierAnalysis {
  pub fn new() {}
  pub fn run() {}
  pub fn is_requred() {}
}

// Create a verifier pass.
struct VerifierPass {}
impl VerifierPass {
  pub fn new() {}
  pub fn run() {}
  pub fn is_required() {}
}