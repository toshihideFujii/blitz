#![allow(dead_code)]

// This represents an independent object.
// That is a function, or a global variable, but not an alias.

enum VCallVisibility {
  Public,
  LinkageUnit,
  TranslationUnit
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalObject {}

impl GlobalObject {
  pub fn new() {}
  pub fn get_alignment() {}
  pub fn get_align() {}
  pub fn set_alignment() {}
  pub fn get_global_object_sub_class_data() {}
  pub fn set_global_object_sub_class_data() {}
  pub fn has_section() {}
  pub fn get_section() {}
  pub fn set_section() {}
  pub fn has_combat() {}
  pub fn get_combat() {}
  pub fn set_combat() {}
  pub fn copy_metadata() {}
  pub fn add_type_metadata() {}
  pub fn set_vcall_visibility_metadata() {}
  pub fn get_vcall_visibility() {}
  pub fn can_increase_alignment() {}

  fn set_global_object_flag() {}
}