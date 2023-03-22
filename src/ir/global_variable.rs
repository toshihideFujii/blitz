#![allow(dead_code)]

// This file represents the declaration of the GlobalVariable
// class, which represents a single variable (or constant) in
// the VM.

struct GlobalVariable {}
impl GlobalVariable {
  pub fn new() {}
  pub fn has_initializer() {}
  pub fn has_definitive_initializer() {}
  pub fn has_unique_initializer() {}
  pub fn get_initializer() {}
  pub fn set_initializer() {}
  pub fn is_constant() {}
  pub fn set_constant() {}
  pub fn is_externally_initialized() {}
  pub fn set_externally_initialized() {}
  pub fn copy_attributes_from() {}
  pub fn remove_from_parent() {}
  pub fn erase_from_parent() {}
  pub fn drop_all_references() {}
  pub fn add_debug_info() {}
  pub fn get_debug_info() {}
  pub fn add_attributee() {}
  pub fn has_attribute() {}
  pub fn get_attribute() {}
}