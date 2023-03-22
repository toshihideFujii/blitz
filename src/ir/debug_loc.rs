#![allow(dead_code)]

// This file defines a number of light weight data structures
// used to describe and track debug location information.

struct DebugLoc {}
impl DebugLoc {
  pub fn get() {}
  pub fn append_inlined_at() {}
  pub fn get_line() {}
  pub fn get_col() {}
  pub fn get_scope() {}
  pub fn get_inlined_at() {}
  pub fn get_inlined_at_scope() {}
  pub fn replace_inlined_at_subprogram() {}
  pub fn get_fn_debug_loc() {}
  pub fn get_as_md_node() {}
  pub fn is_implicit_code() {}
  pub fn set_implicit_code() {}
  pub fn dump() {}
  pub fn print() {}
}