#![allow(dead_code)]

// This file defines various helper methods and classes
// used by BlitzContextImpl for creating and managing attributes.

use rand::Rng;

//use std::collections::HashMap;
use crate::{
  adt::{/*dense_map::DenseMap,*/ string_ref::StringRef,
    floating_point_mode::FPClassTest, folding_set::FoldingSetNodeID},
  support::{alignment::MaybeAlign,
  code_gen::UWTableKind, mod_ref::MemoryEffects}, ir::attributes::AttrIndex
};
use super::{attributes::{AttrKind, Attribute, AllocFnKind, AttributeSet,
  AttrBuilder}, type_::{Type, IntegerType, TypeID}, blits_context::BlitzContext};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttrEntryKind {
  EnumAttrEntry,
  IntAttrEntry,
  StringAttrEntry,
  TypeAttrEntry
}

// This class represents a single, uniqued attribute.
// That attribute could be a single enum, a tuple, or a string.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeImpl {
  pub id: u64,
  kind_id: AttrEntryKind,
  kind: AttrKind,
  val: u64,
  //v_type: Option<Box<dyn Type>>,
  int_t: Option<IntegerType>
}

impl AttributeImpl {
  pub fn new(kind_id: AttrEntryKind, kind: AttrKind, val: u64,
  /*v_type: Option<Box<dyn Type>>*/ t: Option<Box<dyn Type>>) -> Self
  {
    let mut rng = rand::thread_rng();
    let id: u64 = rng.gen();
    let mut attr = AttributeImpl {
      id: id,
      kind_id: kind_id,
      kind: kind,
      val: val, /*v_type: v_type*/
      int_t: None
    };
    if t.is_some() && t.as_ref().unwrap().get_type_id() == TypeID::Integer {
      let int_t = t.unwrap().as_any().
        downcast_ref::<IntegerType>().unwrap().clone();
      attr.int_t = Some(int_t);
    }
    attr
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

  pub fn profile(&self, id: &mut FoldingSetNodeID) {
    if self.is_enum_attribute() {
      debug_assert!(Attribute::is_enum_attr_kind(&self.kind), "Expected enum attribute.");
      id.add_integer_u64(self.kind.clone() as u64);
    } else if self.is_int_attribute() {
      debug_assert!(Attribute::is_int_attr_kind(&self.kind), "Expected int attribute.");
      id.add_integer_u64(self.kind.clone() as u64);
      id.add_integer_u64(self.val);
    } else if self.is_string_attribute() {
      // TODO
    }
    // TODO
  }
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
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeSetNode {
  pub id: u64,
  //available_attrs: AttributeBitSet,
  //string_attrs: DenseMap<String, Attribute>
  pub attrs: Vec<Attribute>
}

impl AttributeSetNode {
  pub fn new(attrs: Vec<Attribute>) -> Self {
    let mut rng = rand::thread_rng();
    let id: u64 = rng.gen();
    AttributeSetNode {
      id: id,
      attrs: attrs.clone()
    }
  }

  pub fn new_default() -> Self {
    let mut rng = rand::thread_rng();
    let id: u64 = rng.gen();
    AttributeSetNode {
      id: id,
      attrs: Vec::new()
    }
  }

  pub fn get_sorted(c: &mut BlitzContext, sorted_attrs: &Vec<Attribute>)
    -> Option<AttributeSetNode>
  {
    if sorted_attrs.is_empty() { return None; }
    let mut id = FoldingSetNodeID::new();

    for attr in sorted_attrs {
      attr.profile(&mut id)
    }

    let pa =
      c.p_impl.as_ref().unwrap().as_ref().attrs_set_nodes.get(&id);
    if pa.is_none() {
      let node = AttributeSetNode::new(sorted_attrs.clone());
      c.get_mut_impl().attrs_set_nodes.insert(id, node.clone());
      return Some(node);
    } else {
      return Some(pa.unwrap().clone());
    }
  }

  pub fn get_by_builder(c: &mut BlitzContext,
    b: &AttrBuilder) -> Option<AttributeSetNode>
  {
    AttributeSetNode::get_sorted(c, &b.attrs)
  }

  pub fn get_by_attrs(c: &mut BlitzContext,
    attrs: &Vec<Attribute>) -> Option<AttributeSetNode>
  {
    // TODO: sort vector
    AttributeSetNode::get_sorted(c, attrs)
  }

  // Return the number of attributes this contains.
  pub fn get_num_attributes(&self) -> usize {
    self.attrs.len()
  }

  pub fn has_attribute(&self, kind: &AttrKind) -> bool {
    for attr in &self.attrs {
      if attr.has_attribute(kind) { return true; }
    }
    false
  }

  pub fn has_attributes(&self) -> bool {
    self.attrs.len() != 0
  }

  pub fn get_attribute(&self, kind: &AttrKind) -> Option<Attribute> {
    self.find_enum_attribute(kind)
  }

  pub fn get_alignment(&self) -> Option<MaybeAlign> {
    let a =
      self.find_enum_attribute(&AttrKind::Alignment);
    if a.is_some() {
      return Some(a.as_ref().unwrap().get_alignment());
    }
    None
  }

  pub fn get_stack_alignment(&self) -> Option<MaybeAlign> {
    let a =
      self.find_enum_attribute(&AttrKind::StackAlignment);
    if a.is_some() {
      return Some(a.as_ref().unwrap().get_stack_alignment());
    }
    None
  }

  pub fn get_dereferenceable_bytes(&self) -> u64 {
    let a =
      self.find_enum_attribute(&AttrKind::Dereferenceable);
    if a.is_some() {
      return a.as_ref().unwrap().get_dereferenceable_bytes();
    }
    0
  }

  pub fn get_dereferenceable_or_null_bytes(&self) -> u64 {
    let a =
      self.find_enum_attribute(&AttrKind::DereferenceableOrNull);
    if a.is_some() {
      return a.as_ref().unwrap().get_dereferenceable_or_null_bytes();
    }
    0
  }

  pub fn get_alloc_size_args(&self) -> Option<(u64, u64)> {
    let a =
      self.find_enum_attribute(&AttrKind::AllocSize);
    if a.is_some() {
      return Some(a.as_ref().unwrap().get_alloc_size_args());
    }
    None
  }

  pub fn get_v_scale_range_min(&self) -> u64 {
    let a =
      self.find_enum_attribute(&AttrKind::VScaleRange);
    if a.is_some() {
      return a.as_ref().unwrap().get_vscale_range_min();
    }
    1
  }

  pub fn get_v_scale_range_max(&self) -> Option<u64> {
    let a =
      self.find_enum_attribute(&AttrKind::VScaleRange);
    if a.is_some() {
      return Some(a.as_ref().unwrap().get_vscale_range_max());
    }
    None
  }

  pub fn get_uw_table_kind(&self) -> UWTableKind {
    let a =
      self.find_enum_attribute(&AttrKind::UWTable);
    if a.is_some() {
      return a.as_ref().unwrap().get_uw_table_kind();
    }
    UWTableKind::None
  }

  pub fn get_alloc_kind(&self) -> AllocFnKind {
    let a =
      self.find_enum_attribute(&AttrKind::AllocKind);
    if a.is_some() {
      return a.as_ref().unwrap().get_alloc_kind();
    }
    AllocFnKind::Unknown
  }

  pub fn get_memory_effects(&self) -> MemoryEffects {
    let a =
      self.find_enum_attribute(&AttrKind::Memory);
    if a.is_some() {
      return a.as_ref().unwrap().get_memory_effects();
    }
    MemoryEffects::unknown()
  }

  pub fn get_no_fp_class(&self) -> FPClassTest {
    let a =
      self.find_enum_attribute(&AttrKind::NoFPClass);
    if a.is_some() {
      return a.as_ref().unwrap().get_no_fp_class();
    }
    FPClassTest::None
  }

  pub fn get_as_string() {}

  pub fn get_attribute_type(&self, _kind: AttrKind) -> Option<Box<dyn Type>> {
    None
  }

  pub fn profile(&self, id: &mut FoldingSetNodeID) {
    for attr in &self.attrs {
      attr.profile(id);
    }
  }

  fn find_enum_attribute(&self, kind: &AttrKind) -> Option<Attribute> {
    if !self.has_attribute(kind) { return None; }
    for attr in &self.attrs {
      if attr.has_attribute(kind) {
        return Some(attr.clone());
      }
    }
    None
  }
}

// This class represents a set of attributes that apply to the function,
// return type, and parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeListImpl {
  //pub num_attr_sets: usize,
  pub attr_sets: Vec<AttributeSet>,
  available_fn_attrs: Vec<Attribute>, //AttributeBitSet,
  available_somewhere_attrs: Vec<Attribute> //AttributeBitSet
}

impl AttributeListImpl {
  pub fn new(attr_sets: &Vec<AttributeSet>) -> Self {
    debug_assert!(!attr_sets.is_empty(), "Pointless AttributeListImpl.");

    let mut attr_list = AttributeListImpl {
      //num_attr_sets: attr_sets.len(),
      attr_sets: attr_sets.clone(),
      available_fn_attrs: Vec::new(),
      available_somewhere_attrs: Vec::new()
    };

    // Initialize available_fn_attrs and availabel_somewhere_attrs.
    let attr_set =
      attr_sets.get(AttrIndex::FunctionIndex as usize);
    println!("bbbbb {:?}", attr_set);
    if attr_set.is_some() {
      let attrs =
        attr_set.unwrap().set_node.as_ref().unwrap().attrs.clone();
      for attr in attrs {
        if !attr.is_string_attribute() {
          attr_list.available_fn_attrs.push(attr.clone());
        }
      }
    }
    for attr_set in attr_sets {
      if attr_set.set_node.is_some() {
        let attrs =
          attr_set.set_node.as_ref().unwrap().attrs.clone();
        for attr in attrs {
          if !attr.is_string_attribute() {
            attr_list.available_somewhere_attrs.push(attr.clone());
          }
        }
      }
    }

    attr_list
  }

  pub fn new_default() -> Self {
    AttributeListImpl {
      attr_sets: Vec::new(),
      available_fn_attrs: Vec::new(),
      available_somewhere_attrs: Vec::new()
    }
  }

  // Return true if the AttributeSet or the FunctionIndex has an enum
  // attribute of the given kind. 
  pub fn has_fn_attribute(&self, kind: &AttrKind) -> bool {
    for attr in &self.available_fn_attrs {
      if attr.has_attribute(kind) { return true; }
    }
    false
  }

  // Return true if the specified attribute is set for at least one
  // parameter or for the return value.
  // If index is not None, the index of a parameter with the specified 
  // attribute is provided.
  pub fn has_attr_somewhere(&self, kind: &AttrKind, index: Option<usize>) -> bool {
    if index.is_some() {
      let attr =
        self.available_somewhere_attrs.get(index.unwrap());
      if attr.is_some() && attr.unwrap().has_attribute(kind) {
        return true;
      }
    }
    for attr in &self.available_somewhere_attrs {
      if attr.has_attribute(kind) {
        return true;
      }
    }
    false
  }

  pub fn profile(id: &mut FoldingSetNodeID, attr_sets: &Vec<AttributeSet>) {
    for attr_set in attr_sets {
      id.add_attr_set_node_id(attr_set.set_node.as_ref().unwrap());
    }
  }

  pub fn dump() {}

  pub fn num_trailing_objects(&self) -> usize {
    self.attr_sets.len()
  }
}