#![allow(dead_code)]

pub fn find_dbg_addr_uses() {}
pub fn find_dbg_declare_uses() {}
pub fn find_debug_values() {}
pub fn find_dbg_users() {}
pub fn get_di_subprogram() {}
pub fn get_debug_value_loc() {}
pub fn strip_debug_info() {}
pub fn strip_non_line_table_debug_info() {}
pub fn update_loop_metadata_debug_locations() {}
pub fn get_debug_metadata_version_from_module() {}

// Utility to find all debug info in a module.
struct DebugInfoFinder {}
impl DebugInfoFinder {
  pub fn process_mdule() {}
  pub fn process_instruction() {}
  pub fn process_variable() {}
  pub fn process_location() {}
  pub fn process_subprogram() {}
  pub fn reset() {}

  pub fn compile_units() {}
  pub fn subprograms() {}
  pub fn global_variables() {}
  pub fn types() {}
  pub fn scopes() {}

  pub fn compile_unit_count() {}
  pub fn global_variable_count() {}
  pub fn subprogram_count() {}
  pub fn type_count() {}
  pub fn scope_count() {}
}