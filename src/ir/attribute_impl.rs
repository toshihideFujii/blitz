#![allow(dead_code)]

// This file defines various helper methods and classes
// used by BlitzContextImpl for creating and managing attributes.

use rand::Rng;

//use std::collections::HashMap;
use crate::{
  adt::{/*dense_map::DenseMap,*/ string_ref::StringRef,
    floating_point_mode::FPClassTest, folding_set::FoldingSetNodeID},
  support::{alignment::MaybeAlign,
  code_gen::UWTableKind, mod_ref::MemoryEffects}
};
use super::{attributes::{AttrKind, Attribute, AllocFnKind, AttributeSet,
  AttrBuilder}, type_::Type, blits_context::BlitzContext};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttrEntryKind {
  EnumAttrEntry,
  IntAttrEntry,
  StringAttrEntry,
  TypeAttrEntry
}

// This class represents a single, uniqued attribute.
// That attribute could be a single enum, a tuple, or a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeImpl {
  kind_id: AttrEntryKind,
  kind: AttrKind,
  val: u64,
  //v_type: Option<Box<dyn Type>>
}

impl AttributeImpl {
  pub fn new(kind_id: AttrEntryKind, kind: AttrKind, val: u64,
  /*v_type: Option<Box<dyn Type>>*/) -> Self
  {
    AttributeImpl { kind_id: kind_id, kind: kind, val: val, /*v_type: v_type*/ }
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

  pub fn get_value_as_type(&self) -> Option<Box<dyn Type>> {
    //&self.v_type
    None
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeBitSet {
  // Bitset with a bit for each available attribute AttrKind.
  available_attrs: Vec<u8>
}

impl AttributeBitSet {
  pub fn new() -> Self {
    AttributeBitSet { available_attrs: Vec::new() }
  }

  pub fn has_attribute(&self, kind: &AttrKind) -> bool {
    let kind_val = kind.clone() as u8;
    if self.available_attrs[(kind_val / 8) as usize] & (1 << (kind_val % 8)) != 0 {
      return true;
    } else {
      return false;
    }
  }

  pub fn add_attribute(&mut self, kind: &AttrKind) {
    let kind_val = kind.clone() as u8;
    self.available_attrs[(kind_val / 8) as usize] |= 1 << (kind_val % 8);
  }
}

// This class represents a group of attributes that apply to one element:
// function, return type, or parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeSetNode {
  pub id: u64,
  num_attrs: usize,
  available_attrs: AttributeBitSet,
  //string_attrs: DenseMap<String, Attribute>
  pub attrs: Vec<Attribute>
}

impl AttributeSetNode {
  pub fn new(attrs: Vec<Attribute>) -> Self {
    let mut available_attrs = AttributeBitSet::new();
    let attrs_clone = attrs.clone();
    let num_attrs = attrs.len();
    for attr in attrs {
      if attr.is_string_attribute() {
        // TODO
      } else {
        available_attrs.add_attribute(&attr.get_kind_as_enum());
      }
    }
    let mut rng = rand::thread_rng();
    let id: u64 = rng.gen();
    AttributeSetNode {
      id: id, num_attrs: num_attrs,
      available_attrs: available_attrs, attrs: attrs_clone
    }
  }

  pub fn get_sorted(_c: &BlitzContext, _attrs: &Vec<Attribute>)
    -> Option<AttributeSetNode>
  {
    None
  }

  pub fn get_by_builder(c: &BlitzContext, b: &AttrBuilder) -> Option<AttributeSetNode> {
    AttributeSetNode::get_sorted(c, &b.attrs)
  }

  pub fn get_by_attrs(c: &BlitzContext, attrs: &Vec<Attribute>) -> Option<AttributeSetNode> {
    // TODO: sort vector
    AttributeSetNode::get_sorted(c, attrs)
  }

  // Return the number of attributes this contains.
  pub fn get_num_attributes(&self) -> usize {
    self.num_attrs
  }

  pub fn has_attribute(&self, kind: &AttrKind) -> bool {
    self.available_attrs.has_attribute(kind)
  }

  pub fn has_attributes(&self) -> bool {
    self.num_attrs != 0
  }

  pub fn get_attribute(&self, kind: AttrKind) -> Option<Attribute> {
    self.find_enum_attribute(kind)
  }

  pub fn get_alignment(&self) -> Option<MaybeAlign> {
    let a = self.find_enum_attribute(AttrKind::Alignment);
    if a.is_some() {
      return Some(a.as_ref().unwrap().get_alignment());
    }
    None
  }

  pub fn get_stack_alignment(&self) -> Option<MaybeAlign> {
    let a = self.find_enum_attribute(AttrKind::StackAlignment);
    if a.is_some() {
      return Some(a.as_ref().unwrap().get_stack_alignment());
    }
    None
  }

  pub fn get_dereferenceable_bytes(&self) -> u64 {
    let a = self.find_enum_attribute(AttrKind::Dereferenceable);
    if a.is_some() {
      return a.as_ref().unwrap().get_dereferenceable_bytes();
    }
    0
  }

  pub fn get_dereferenceable_or_null_bytes(&self) -> u64 {
    let a = self.find_enum_attribute(AttrKind::DereferenceableOrNull);
    if a.is_some() {
      return a.as_ref().unwrap().get_dereferenceable_or_null_bytes();
    }
    0
  }

  pub fn get_alloc_size_args(&self) -> Option<(u64, u64)> {
    let a = self.find_enum_attribute(AttrKind::AllocSize);
    if a.is_some() {
      return Some(a.as_ref().unwrap().get_alloc_size_args());
    }
    None
  }

  pub fn get_v_scale_range_min(&self) -> u64 {
    let a = self.find_enum_attribute(AttrKind::VScaleRange);
    if a.is_some() {
      return a.as_ref().unwrap().get_vscale_range_min();
    }
    1
  }

  pub fn get_v_scale_range_max(&self) -> Option<u64> {
    let a = self.find_enum_attribute(AttrKind::VScaleRange);
    if a.is_some() {
      return Some(a.as_ref().unwrap().get_vscale_range_max());
    }
    None
  }

  pub fn get_uw_table_kind(&self) -> UWTableKind {
    let a = self.find_enum_attribute(AttrKind::UWTable);
    if a.is_some() {
      return a.as_ref().unwrap().get_uw_table_kind();
    }
    UWTableKind::None
  }

  pub fn get_alloc_kind(&self) -> AllocFnKind {
    let a = self.find_enum_attribute(AttrKind::AllocKind);
    if a.is_some() {
      return a.as_ref().unwrap().get_alloc_kind();
    }
    AllocFnKind::Unknown
  }

  pub fn get_memory_effects(&self) -> MemoryEffects {
    let a = self.find_enum_attribute(AttrKind::Memory);
    if a.is_some() {
      return a.as_ref().unwrap().get_memory_effects();
    }
    MemoryEffects::unknown()
  }

  pub fn get_no_fp_class(&self) -> FPClassTest {
    let a = self.find_enum_attribute(AttrKind::NoFPClass);
    if a.is_some() {
      return a.as_ref().unwrap().get_no_fp_class();
    }
    FPClassTest::None
  }

  pub fn get_as_string() {}

  pub fn get_attribute_type(&self, _kind: AttrKind) -> Option<Box<dyn Type>> {
    None
  }

  pub fn profile() {}

  fn find_enum_attribute(&self, _kind: AttrKind) -> Option<Attribute> {
    None
  }
}

// This class represents a set of attributes that apply to the function,
// return type, and parameters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeListImpl {
  // Number of entries in this set.
  num_attr_sets: usize,
  // Available enum function attributes.
  available_function_attrs: AttributeBitSet,
  // Union of enum attributes available at any index.
  available_somewhere_attrs: AttributeBitSet
}

impl AttributeListImpl {
  pub fn new(attr_sets: &Vec<AttributeSet>) -> Self {
    debug_assert!(attr_sets.is_empty(), "Pointless AttributeListImpl.");

    // TODO
    let attr_list = AttributeListImpl {
      num_attr_sets: attr_sets.len(),
      available_function_attrs: AttributeBitSet::new(),
      available_somewhere_attrs: AttributeBitSet::new()
    };

    //for attr_set in attr_sets {
      //for attr in attr_set {
      //}
    //}

    attr_list
  }

  pub fn has_fn_attribute() {}
  pub fn has_attr_somewhere() {}

  pub fn profile(id: &mut FoldingSetNodeID, attr_sets: &Vec<AttributeSet>) {
    for attr_set in attr_sets {
      id.add_attr_set_node_id(attr_set.set_node.as_ref().unwrap());
    }
  }

  pub fn dump() {}

  fn num_trailing_objects(&self) -> usize {
    self.num_attr_sets
  }
}