#![allow(dead_code)]

// This file represents the declaration of the GlobalVariable
// class, which represents a single variable (or constant) in
// the VM.

use super::{attributes::{AttributeSet, AttrKind, Attribute},
  value::{Value, ValueType}, global_object::GlobalObject,
  global_value::GlobalValue};

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariable {
  go: GlobalObject,
  gv: GlobalValue,
  attrs: AttributeSet,
  is_constant_global: bool,
  is_externally_initialized_constant: bool
}

impl GlobalVariable {
  pub fn new() {}

  // Definitions have initializers, declarations don't.
  pub fn has_initializer(&self) -> bool {
    !self.gv.is_declaration()
  }

  // Whether the global variable has an initializer, and any other
  // instances of the global (this can happen due to weak linkage)
  // are guaranteed to have the same initializer.
  pub fn has_definitive_initializer(&self) -> bool {
    self.has_initializer() && !self.gv.is_interposable() &&
    !self.is_externally_initialized()
  }

  pub fn has_unique_initializer() {}
  pub fn get_initializer() {}
  pub fn set_initializer() {}

  // If the value is a global constant, its value is immutable
  // thoughout the runtime execution of the program. Assigning a
  // value into the constant leads to undefined behavior.
  pub fn is_constant(&self) -> bool {
    self.is_constant_global
  }

  pub fn set_constant(&mut self, val: bool) {
    self.is_constant_global = val;
  }

  pub fn is_externally_initialized(&self) -> bool {
    self.is_externally_initialized_constant
  }

  pub fn set_externally_initialized(&mut self, val: bool) {
    self.is_externally_initialized_constant = val;
  }

  pub fn copy_attributes_from() {}
  pub fn remove_from_parent() {}
  pub fn erase_from_parent() {}
  pub fn drop_all_references() {}
  pub fn add_debug_info() {}
  pub fn get_debug_info() {}
  pub fn add_attribute() {}

  // Return true if the attribute exists.
  pub fn has_attribute(&self, kind: AttrKind) -> bool {
    self.attrs.has_attribute(kind)
  }

  pub fn has_attributes() {}

  // Return the attribute object.
  pub fn get_attribute(&self, kind: AttrKind) -> Option<Attribute> {
    self.attrs.get_attribute(kind)
  }

  // Return the attribute set for this global.
  pub fn get_attributes(&self) -> &AttributeSet {
    &self.attrs
  }

  pub fn get_attributes_as_list() {}

  // Set attribute list for this global.
  pub fn set_attributes(&mut self, atts: AttributeSet) {
    self.attrs = atts;
  }

  pub fn has_implicit_section() {}

  pub fn class_of(v: Box<dyn Value>) -> bool {
    v.get_value_id() == ValueType::GlobalVariableVal
  }
}