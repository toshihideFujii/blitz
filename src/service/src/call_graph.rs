#![allow(dead_code)]

#[derive(Clone, PartialEq)]
pub enum CallContext {
  Embedded,
  ControlFlow,
  Both,
  None,
}

pub fn call_context_to_string() {}

pub fn get_instruction_call_context() {}

pub struct CallSite {}
impl CallSite {
  pub fn new() {}
  pub fn instruction() {}
  pub fn called_computations() {}
  pub fn context() {}
  pub fn to_string() {}
}

pub struct CallGrapgNode {}
impl CallGrapgNode {
  pub fn new() {}
  pub fn computation() {}
  pub fn callsites() {}
  pub fn get_call_site() {}
  pub fn callees() {}
  pub fn caller_callsites() {}
  pub fn callers() {}
  pub fn context() {}
  pub fn depth() {}
  pub fn to_string() {}
}

pub struct CallGraph {}
impl CallGraph {
  pub fn new() {}
  pub fn get_node() {}
  pub fn nodes() {}
  pub fn visit_nodes() {}
  pub fn dominates() {}
  pub fn can_reach() {}
  pub fn instruction_is_nested_in() {}
  pub fn nearest_ancestors_in_same_computation() {}
  pub fn nearest_common_ancestor_instructions() {}
  pub fn nearest_common_ancestor_computations() {}
  pub fn nearest_common_ancestors_helper() {}
  pub fn is_flattened() {}
  pub fn get_computation_callers() {}
  pub fn to_string() {}
}