#![allow(dead_code)]

// This file containes the declaration of the GlobalVariable class, 
// which represents a single variable (or constant) in the VM.
// Global variables are constant pointers that refer to hunks of space
// that are allocated by either the VM, or by the linker in a static compiler.
// A global variable may have an initial value, which is copied into the
// executables .data area. Global Constants are required to have initializers

//use crate::adt::twine::Twine;

use super::{
  attributes::{AttributeSet, AttrKind, Attribute},
  value::{Value, ValueType}, global_object::GlobalObject,
  //global_value::{GlobalValue, LinkageTypes}, constant::Constant,
};

#[derive(Debug)]
pub struct GlobalVariable {
  go: Box<dyn GlobalObject>,
  attrs: AttributeSet,
  is_constant_global: bool,
  is_externally_initialized_constant: bool
}

impl GlobalVariable {
  //pub fn new(t: Box<dyn Type>, is_constant: bool,
    //linkage: LinkageTypes, name: Twine) -> Self
  //{
    //GlobalVariable { go: (), attrs: (), is_constant_global: (), is_externally_initialized_constant: () }
  //}

  // Definitions have initializers, declarations don't.
  pub fn has_initializer(&self) -> bool {
    !self.go.as_ref().is_declaration()
  }

  // Whether the global variable has an initializer, and any other
  // instances of the global (this can happen due to weak linkage)
  // are guaranteed to have the same initializer.
  pub fn has_definitive_initializer(&self) -> bool {
    self.has_initializer() && !self.go.is_interposable() &&
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
  pub fn has_attribute(&self, kind: &AttrKind) -> bool {
    self.attrs.has_attribute(kind)
  }

  pub fn has_attributes() {}

  // Return the attribute object.
  pub fn get_attribute(&self, kind: &AttrKind) -> Option<Attribute> {
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

/*
impl Value for GlobalVariable {}
impl Constant for GlobalVariable {}
impl GlobalValue for GlobalVariable {}
impl GlobalObject for GlobalVariable {}
*/