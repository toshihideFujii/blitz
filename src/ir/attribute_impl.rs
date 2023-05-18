#![allow(dead_code)]

// This file defines various helper methods and classes
// used by BlitzContextImpl for creating and managing attributes.

use crate::adt::string_ref::StringRef;
use super::{attributes::AttrKind, type_::Type};

#[derive(Debug, PartialEq)]
pub enum AttrEntryKind {
  EnumAttrEntry,
  IntAttrEntry,
  StringAttrEntry,
  TypeAttrEntry
}

// This class represents a single, uniqued attribute.
// That attribute could be a single enum, a tuple, or a string.
#[derive(Debug)]
pub struct AttributeImpl {
  kind_id: AttrEntryKind,
  kind: AttrKind,
  val: u64,
  v_type: Option<Box<dyn Type>>
}

impl AttributeImpl {
  pub fn new(kind_id: AttrEntryKind, kind: AttrKind, val: u64, v_type: Box<dyn Type>) -> Self {
    AttributeImpl { kind_id: kind_id, kind: kind, val: val, v_type: Some(v_type) }
  }

  pub fn is_enum_attribute(&self) -> bool {
    self.kind_id == AttrEntryKind::EnumAttrEntry
  }

  pub fn is_int_attribute(&self) -> bool {
    self.kind_id == AttrEntryKind::IntAttrEntry
  }

  pub fn is_string_attribute(&self) -> bool {
    self.kind_id == AttrEntryKind::StringAttrEntry
  }

  pub fn is_type_attribute(&self) -> bool {
    self.kind_id == AttrEntryKind::TypeAttrEntry
  }

  pub fn has_attribute(&self, kind: AttrKind) -> bool {
    if self.is_string_attribute() { return false; }
    *self.get_kind_as_enum() == kind
  }

  pub fn has_attribute_string(&self, kind: StringRef) -> bool {
    if !self.is_string_attribute() { return false; }
    self.get_kind_as_string() == kind
  }

  pub fn get_kind_as_enum(&self) -> &AttrKind {
    &self.kind
  }

  pub fn get_kind_as_string(&self) -> StringRef {
    StringRef::new()
  }

  pub fn get_value_as_int(&self) -> u64 {
    self.val
  }

  pub fn get_value_as_bool(&self) -> bool {
    self.get_kind_as_string() == StringRef::new_from_string("true")
  }

  pub fn get_value_as_string(&self) -> StringRef {
    StringRef::new()
  }

  pub fn get_value_as_type(&self) -> &Option<Box<dyn Type>> {
    &self.v_type
  }

  pub fn profile() {}
}

struct EnumAttributeImpl {}
impl EnumAttributeImpl {
  pub fn new() {}
  pub fn get_enum_kind() {}
}

struct IntAttributeImpl {}
impl IntAttributeImpl {
  pub fn new() {}
  pub fn get_value() {}
}

struct StringAttributeImpl {}
impl StringAttributeImpl {
  pub fn new() {}
  pub fn get_string_kind() {}
  pub fn get_string_value() {}
}

struct TypeAttributeImpl {}
impl TypeAttributeImpl {
  pub fn new() {}
  pub fn get_type_value() {}
}

struct AttributeBitSet {
  available_attrs: Vec<u8>
}

impl AttributeBitSet {
  pub fn new() -> Self {
    AttributeBitSet { available_attrs: Vec::new() }
  }

  pub fn has_attribute(&self, kind: AttrKind) -> bool {
    let kind_val = kind as u8;
    if self.available_attrs[(kind_val / 8) as usize] & (1 << (kind_val % 8)) != 0 {
      return true;
    } else {
      return false;
    }
  }

  pub fn add_attribute(&mut self, kind: AttrKind) {
    let kind_val = kind as u8;
    self.available_attrs[(kind_val / 8) as usize] |= 1 << (kind_val % 8);
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeSetNode {}
impl AttributeSetNode {
  pub fn new() {}
  pub fn get() {}
  pub fn get_num_attributes() {}
  pub fn has_attribute() {}
  pub fn has_attributes() {}
  pub fn get_attribute() {}
  pub fn get_alignment() {}
  pub fn get_stack_alignment() {}
  pub fn get_dereferenceable_bytes() {}
  pub fn get_dereferenceable_or_null_bytes() {}
  pub fn get_v_scale_range_min() {}
  pub fn get_v_scale_range_max() {}
  pub fn get_uw_table_kind() {}
  pub fn get_alloc_kind() {}
  pub fn get_memory_effects() {}
  pub fn get_as_string() {}
  pub fn get_attribute_type() {}
  pub fn profile() {}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeListImpl {}
impl AttributeListImpl {
  pub fn new() {}
  pub fn has_fn_attribute() {}
  pub fn has_attr_somewhere() {}
  pub fn profile() {}
  pub fn dump() {}
}