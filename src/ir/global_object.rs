#![allow(dead_code)]

// This represents an independent object.
// That is a function, or a global variable, but not an alias.

//use super::comdat::Comdat;

use super::{comdat::Comdat, value::{Value, ValueType}};

enum VCallVisibility {
  Public = 0,
  LinkageUnit = 1,
  TranslationUnit = 2
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalObject {
  //obj_comdata: Box<Option<Comdat>>
}

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

  pub fn has_combat(&self) -> bool {
    self.get_combat().is_some()
  }

  pub fn get_combat(&self) -> Option<Comdat> {
    None // TODO
  }

  pub fn set_combat() {}
  pub fn copy_metadata() {}
  pub fn add_type_metadata() {}
  pub fn set_vcall_visibility_metadata() {}
  pub fn get_vcall_visibility() {}
  pub fn can_increase_alignment(&self) -> bool { false }

  pub fn class_of(v: Box<dyn Value>) -> bool {
    v.get_value_id() == ValueType::FunctionVal ||
    v.get_value_id() == ValueType::GlobalVariableVal ||
    v.get_value_id() == ValueType::GlobalIFuncVal
  }

  fn set_global_object_flag() {}
}