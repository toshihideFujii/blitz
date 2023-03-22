#![allow(dead_code)]

// This file defines a DIBulder that is useful for creating
// debugging information entries in IR form.

struct DIBulder {}
impl DIBulder {
  pub fn new() {}
  pub fn track_if_unresolved() {}
  pub fn insert_declare() {}
  pub fn insert_label() {}
  pub fn insert_dbg_intrinsic() {}
  pub fn insert_dbg_value_intrinsic() {}
  pub fn finalize() {}
  pub fn finalize_subprogram() {}
  pub fn create_compile_unit() {}
  pub fn create_file() {}
  pub fn create_macro() {}
  pub fn create_temp_macro_file() {}
  pub fn create_enumarator() {}
  pub fn create_unspecified_type() {}
  pub fn create_null_ptr_type() {}
}