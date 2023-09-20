#![allow(dead_code)]

// This file contains the simple types necessary to represent the
// attributes associated with functions and their calls.

use crate::{
  adt::{
    string_ref::StringRef, folding_set::FoldingSetNodeID,
    floating_point_mode::FPClassTest,
  },
  ir::{
    blits_context::BlitzContext,
    attribute_impl::{AttributeImpl, AttributeSetNode, AttrEntryKind,
      AttributeListImpl},
    type_::Type, value
  },
  support::{
    alignment::{MaybeAlign, Align}, code_gen::UWTableKind,
    mod_ref::MemoryEffects
  },
};

use super::blits_context::blits_context_mut;

#[derive(Debug, Clone)]
pub enum AllocFnKind {
  Unknown = 0,
  Alloc = 1 << 0, // Allocator function returns a new allocation
  Realloc = 1 << 1, // Allocator function resizes the 'allocptr' argument
  Free = 1 << 2, // Allocator function frees the 'allocptr' argument
  Uninitialized = 1 << 3, // Allocator function returns uninitialized memory
  Zeroed = 1 << 4, // Allocator function returns zeroed memory
  Aligned = 1 << 5 // Allocator function aligns allocations per the 'allocalign' argument
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum AttrKind {
  None,
  // IntAttr
  Alignment,
  AllocKind,
  AllocSize,
  Dereferenceable,
  DereferenceableOrNull,
  Memory,
  NoFPClass,
  StackAlignment,
  UWTable,
  VScaleRange,

  // EnumAttr
  InReg,
  Nest,
  NoAlias,
  NoCapture,
  NoFree,
  NonNull,
  NoUndef,
  Returned,
  SExt,
  ZExt,
  PresplitCoroutine,
  NoReturn,
  NoCfCheck,
  NoUnwind,
  NoDuplicate,
  Convergent,
  Speculatable,
  NoSync,
  NoRecurse,
  MustProgress,
  WillReturn,
  OptimizeNone,
  MinSize,
  OptimizeForSize,
  ReadNone,
  ReadOnly,
  WriteOnly,
  NoBuiltin,
  Builtin,
  StrictFP,
  NoInline,
  NoMerge,
  ReturnsTwice,
  AlwaysInline,

  // TypeAttr
  ByVal,
  ByRef,
  ElementType,
  InAlloca,
  Preallocated,
  StructRet,

  // Others
  EndAttrKinds,
  EmptyKey,
  TombstoneKey,
}

pub enum AttributeProperty {
  FnAttr = (1 << 0),
  ParamAttr = (1 << 1),
  RetAttr = (1 << 2)
}

// Functions, function parameters, and return types can have attributes
// to indicate how they should be treated by optimizations and code generation.
// This class represents one of those attributes.
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
  pimpl: Option<AttributeImpl>
}

impl Attribute {
  pub fn new(pimpl: Option<AttributeImpl>) -> Self {
    Attribute { pimpl: pimpl }
  }

  pub fn is_enum_attr_kind(kind: &AttrKind) -> bool {
    AttrKind::InReg <= *kind && *kind <= AttrKind::AlwaysInline
  }

  pub fn is_int_attr_kind(kind: &AttrKind) -> bool {
    AttrKind::Alignment <= *kind && *kind <= AttrKind::VScaleRange
  }

  pub fn is_type_attr_kind(kind: &AttrKind) -> bool {
    AttrKind::ByVal <= *kind && *kind <= AttrKind::StructRet
  }

  pub fn can_use_as_fn_attr(kind: AttrKind) -> bool {
    Attribute::has_attribute_property(kind, AttributeProperty::FnAttr)
  }

  pub fn can_use_as_param_attr(kind: AttrKind) -> bool {
    Attribute::has_attribute_property(kind, AttributeProperty::ParamAttr)
  }

  pub fn can_use_as_ret_attr(kind: AttrKind) -> bool {
    Attribute::has_attribute_property(kind, AttributeProperty::RetAttr)
  }

  fn has_attribute_property(_kind: AttrKind, _prop: AttributeProperty) -> bool {
    false
  }

  // Return a uniquified Attribute object.
  pub fn get_by_int(c: &mut BlitzContext, kind: AttrKind, val: u64) -> Self {
    let is_int_attr = Attribute::is_int_attr_kind(&kind);
    debug_assert!(is_int_attr || Attribute::is_enum_attr_kind(&kind),
      "Not an enum or int attribute.");
    
    let mut id = FoldingSetNodeID::new();
    id.add_integer_u32(kind.clone() as u32);
    if is_int_attr {
      id.add_integer_u64(val);
    } else {
      debug_assert!(val == 0, "Value must be zero for enum attributes.");
    }

    let attr_impl = c.attrs_set.get(&id);
    if attr_impl.is_none() {
      let mut kind_id = AttrEntryKind::EnumAttrEntry;
      if is_int_attr { kind_id = AttrEntryKind::IntAttrEntry }
      let v = AttributeImpl::new(kind_id, kind.clone(), val, None);
      c.attrs_set.insert(id, v.clone());
      return Attribute::new(Some(v));
    } else {
      return Attribute::new(Some(attr_impl.unwrap().clone()));
    }
  }

  pub fn get_by_string(_c: &BlitzContext, _kind: AttrKind, _val: StringRef) -> Self {
    Attribute { pimpl: None }
  }

  // TODO
  pub fn get_by_type(c: &mut BlitzContext, kind: AttrKind, _t: Box<dyn Type>) -> Self {
    debug_assert!(Attribute::is_type_attr_kind(&kind), "Not a type attribute.");
    let mut id = FoldingSetNodeID::new();
    id.add_integer_u32(kind.clone() as u32);
    // id.add(t)

    let attr_impl = c.attrs_set.get(&id);
    if attr_impl.is_none() {
      return Attribute::new(Some(attr_impl.unwrap().clone()));
    } else {
      return Attribute::new(Some(attr_impl.unwrap().clone()));
    }
  }

  pub fn get_by_alignment(context: &mut BlitzContext, align: Align) -> Self {
    debug_assert!(align.value() <= value::MAXIMUM_ALIGNMENT, "Alignment too large.");
    Attribute::get_by_int(context, AttrKind::Alignment, align.value())
  }

  pub fn get_by_stack_alignment(context: &mut BlitzContext, align: Align) -> Self {
    debug_assert!(align.value() <= 0x100, "Alignment too large.");
    Attribute::get_by_int(context, AttrKind::StackAlignment, align.value())
  }

  pub fn get_by_dereferenceable_bytes(context: &mut BlitzContext, bytes: u64) -> Self {
    debug_assert!(bytes != 0, "Bytes must be non-zero.");
    Attribute::get_by_int(context, AttrKind::Dereferenceable, bytes)
  }

  pub fn get_by_dereferenceable_or_null_bytes(context: &mut BlitzContext, bytes: u64) -> Self {
    debug_assert!(bytes != 0, "Bytes must be non-zero.");
    Attribute::get_by_int(context, AttrKind::DereferenceableOrNull, bytes)
  }

  pub fn get_by_alloc_size_args(context: &mut BlitzContext, elem_size_arg: u64,
    num_elems_arg: u64) -> Self
  {
    debug_assert!(!(elem_size_arg == 0 && num_elems_arg == 0),
      "Invalid allocsize arguments -- given allocsize(0, 0)");
    Attribute::get_by_int(context, AttrKind::AllocSize,
      Attribute::pack_alloc_size_args(elem_size_arg, num_elems_arg))
  }

  pub fn gete_by_v_scale_range_args(context: &mut BlitzContext, min_value: u64,
    max_value: u64) -> Self
  {
    Attribute::get_by_int(context, AttrKind::VScaleRange,
      Attribute::pack_vscale_range_args(min_value, max_value))
  }

  pub fn get_by_by_val_type(c: &mut BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_by_type(c, AttrKind::ByVal, t)
  }

  pub fn get_by_struct_ret_type(c: &mut BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_by_type(c, AttrKind::StructRet, t)
  }

  pub fn get_by_by_ref_type(c: &mut BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_by_type(c, AttrKind::ByRef, t)
  }

  pub fn get_by_preallocated_type(c: &mut BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_by_type(c, AttrKind::Preallocated, t)
  }

  pub fn get_by_in_alloca_type(c: &mut BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_by_type(c, AttrKind::InAlloca, t)
  }

  pub fn get_by_uw_table_kind(context: &mut BlitzContext,
    kind: UWTableKind) -> Self
  {
    Attribute::get_by_int(context, AttrKind::UWTable, kind as u64)
  }

  pub fn get_by_memory_effects(context: &mut BlitzContext,
    me: MemoryEffects) -> Self
  {
    Attribute::get_by_int(context, AttrKind::Memory,
      me.to_int_value() as u64)
  }

  pub fn get_by_no_fp_class(context: &mut BlitzContext,
    class_mask: FPClassTest) -> Self
  {
    Attribute::get_by_int(context, AttrKind::NoFPClass,
      class_mask as u64)
  }

  pub fn get_attr_kind_from_name() {}
  pub fn get_name_from_attr_kind() {}
  pub fn is_existing_attribute() {}

  // Return true if the attribute is an AttrKind type.
  pub fn is_enum_attribute(&self) -> bool {
    self.pimpl.is_some() && self.pimpl.as_ref().unwrap().is_enum_attribute()
  }

  // Return true if the attribute is an integer attribute.
  pub fn is_int_attribute(&self) -> bool {
    self.pimpl.is_some() && self.pimpl.as_ref().unwrap().is_int_attribute()
  }

  // Return true if the attribute is a string attribute.
  pub fn is_string_attribute(&self) -> bool {
    self.pimpl.is_some() && self.pimpl.as_ref().unwrap().is_string_attribute()
  }

  // Return true if the attribute is a type attribute.
  pub fn is_type_attribute(&self) -> bool {
    self.pimpl.is_some() && self.pimpl.as_ref().unwrap().is_type_attribute()
  }

  // Return true if the attribute is any kind of attribute.
  pub fn is_valid(&self) -> bool {
    self.pimpl.is_some()
  }

  // Return true if the attribute is present.
  pub fn has_attribute(&self, kind: &AttrKind) -> bool {
    if self.pimpl.is_some() {
      return self.pimpl.as_ref().unwrap().has_attribute(kind.clone());
    } else {
      return kind == &AttrKind::None;
    }
  }

  // Return the attribute's kind as an enum (AttrKind).
  pub fn get_kind_as_enum(&self) -> &AttrKind {
    if self.pimpl.is_some() {
      return self.pimpl.as_ref().unwrap().get_kind_as_enum();
    } else {
      return &AttrKind::None;
    }
  }

  // Return the attribute's value as an integer.
  pub fn get_value_as_int(&self) -> u64 {
    if self.pimpl.is_some() {
      return self.pimpl.as_ref().unwrap().get_value_as_int();
    } else {
      return 0;
    }
  }

  // Return the attribute's value as a boolean.
  pub fn get_value_as_bool(&self) -> bool {
    if self.pimpl.is_some() {
      return self.pimpl.as_ref().unwrap().get_value_as_bool();
    } else {
      return false;
    }
  }

  // Return the attribute's kind as a string.
  pub fn get_kind_as_string(&self) -> StringRef {
    if self.pimpl.is_some() {
      return self.pimpl.as_ref().unwrap().get_kind_as_string();
    } else {
      return StringRef::new();
    }
  }

  // Return the attribute's value as a string.
  pub fn get_value_as_string(&self) -> StringRef {
    if self.pimpl.is_some() {
      return self.pimpl.as_ref().unwrap().get_value_as_string();
    } else {
      return StringRef::new();
    }
  }

  // Return the attribute's value as a Type.
  pub fn get_value_as_type(&self) -> Option<Box<dyn Type>> {
    if self.pimpl.is_some() {
      self.pimpl.as_ref().unwrap().get_value_as_type()
    } else {
      return None;
    }
  }

  // Returns the alignment field of an attribute as a byte alignment
  // value.
  pub fn get_alignment(&self) -> MaybeAlign {
    debug_assert!(self.has_attribute(&AttrKind::Alignment),
      "Trying to get alignment from non-alignment attribute.");
    MaybeAlign::new(self.pimpl.as_ref().unwrap().get_value_as_int())
  }

  // Returns the stack alignment field of an attribute as a byte
  // alignment value.
  pub fn get_stack_alignment(&self) -> MaybeAlign {
    debug_assert!(self.has_attribute(&AttrKind::StackAlignment),
      "Trying to get alignment from non-alignment attribute.");
    MaybeAlign::new(self.pimpl.as_ref().unwrap().get_value_as_int())
  }

  // Returns the number of dereferenceable bytes from the 
  // dereferenceable attribute.
  pub fn get_dereferenceable_bytes(&self) -> u64 {
    debug_assert!(self.has_attribute(&AttrKind::Dereferenceable),
      "Trying to get dereferenceable bytes from non-dereferenceable attribute.");
    self.pimpl.as_ref().unwrap().get_value_as_int()
  }

  // Returns the number of dereferenceable_or_null bytes from the
  // dereferenceable_or_null attribute.
  pub fn get_dereferenceable_or_null_bytes(&self) -> u64 {
    debug_assert!(self.has_attribute(&AttrKind::DereferenceableOrNull),
      "Trying to get dereferenceable bytes from non-dereferenceable attribute.");
    self.pimpl.as_ref().unwrap().get_value_as_int()
  }

  // Returns the argument numbers for the allocsize attribute.
  pub fn get_alloc_size_args(&self) -> (u64, u64) {
    debug_assert!(self.has_attribute(&AttrKind::AllocSize),
      "Trying to get allocsize args from non-allocsize attribute.");
    Attribute::unpack_alloc_size_args(self.pimpl.as_ref().unwrap().get_value_as_int())
  }

  // Returns the minimum value for the vscale_range attribute.
  pub fn get_vscale_range_min(&self) -> u64 {
    debug_assert!(self.has_attribute(&AttrKind::VScaleRange),
      "Trying to get vscale args from non-vscale attribute.");
    Attribute::unpack_vscale_range_args(self.pimpl.as_ref().unwrap().get_value_as_int()).0
  }

  // Returns the maximum value for the vscale_range attribute.
  pub fn get_vscale_range_max(&self) -> u64 {
    debug_assert!(self.has_attribute(&AttrKind::VScaleRange),
      "Trying to get vscale args from non-vscale attribute.");
    Attribute::unpack_vscale_range_args(self.pimpl.as_ref().unwrap().get_value_as_int()).1
  }

  // Returns the unwind table kind.
  pub fn get_uw_table_kind(&self) -> UWTableKind {
    debug_assert!(self.has_attribute(&AttrKind::UWTable),
      "Trying to get unwind table kind from non-uwtable attribute.");
    let val = self.pimpl.as_ref().unwrap().get_value_as_int();
    match val {
      0 => return UWTableKind::None,
      1 => return UWTableKind::Sync,
      2 => return UWTableKind::Async,
      _ => panic!("Unsupprted valud of UWTableKind.")
    };
  }

  // Returns the allocator function kind.
  pub fn get_alloc_kind(&self) -> AllocFnKind {
    debug_assert!(self.has_attribute(&AttrKind::AllocKind),
      "Trying to get allockind value from non-allockind attribute.");
    let val = self.pimpl.as_ref().unwrap().get_value_as_int();
    match val {
      0 => return AllocFnKind::Unknown,
      1 => return AllocFnKind::Alloc,
      2 => return AllocFnKind::Realloc,
      3 => return AllocFnKind::Free,
      4 => return AllocFnKind::Uninitialized,
      5 => return AllocFnKind::Zeroed,
      6 => return AllocFnKind::Aligned,
      _ => panic!("Unsupprted valud of AllocFnKind.")
    };
  }

  // Returns memory effects.
  pub fn get_memory_effects(&self) -> MemoryEffects {
    debug_assert!(self.has_attribute(&AttrKind::Memory),
      "Can only call get_memory_effects() on memory attribute.");
    MemoryEffects::create_from_int_value(
      self.pimpl.as_ref().unwrap().get_value_as_int() as u32)
  }

  pub fn get_no_fp_class(&self) -> FPClassTest {
    debug_assert!(self.has_attribute(&AttrKind::NoFPClass),
      "Can only call get_no_fp_class() on nofpclass attribute.");
    let val = self.pimpl.as_ref().unwrap().get_value_as_int();
    match val {
      0 => return FPClassTest::None,
      0x0001 => return FPClassTest::SNan,
      0x0002 => return FPClassTest::QNan,
      0x0004 => return FPClassTest::NegInf,
      0x0008 => return FPClassTest::NegNormal,
      0x0010 => return FPClassTest::NegSubnormal,
      0x0020 => return FPClassTest::NegZero,
      0x0040 => return FPClassTest::PosZero,
      0x0080 => return FPClassTest::PosSubnormal,
      0x0100 => return FPClassTest::PosNormal,
      0x0200 => return FPClassTest::PosInf,
       _ => panic!("Unsupprted valud of FPClassTest.")
    };
  }

  pub fn get_as_string() {}

  // Return true if this attribute belongs to the BlitzContext.
  pub fn has_parent_context(&self, c: &mut BlitzContext) -> bool {
    debug_assert!(self.is_valid(), "Invalid attribute doesn't refer to any context.");
    let mut id = FoldingSetNodeID::new();
    self.pimpl.as_ref().unwrap().profile(&mut id);
    c.attrs_set.get(&id) == self.pimpl.as_ref()
  }

  pub fn profile(&self, id: &mut FoldingSetNodeID) {
    id.add_integer_u64(self.pimpl.as_ref().unwrap().id);
  }

  // Return a raw pointer that uniquely identifies this attribute.
  pub fn get_raw_pointer(&self) -> &Option<AttributeImpl> {
    &self.pimpl
  }

  // Get an attribute from a raw pointer created by get_raw_pointer.
  pub fn from_raw_pointer(raw_ptr: Option<AttributeImpl>) -> Attribute {
    Attribute { pimpl: raw_ptr }
  }

  fn pack_alloc_size_args(elem_size_arg: u64, num_elems_arg: u64) -> u64 {
    debug_assert!(num_elems_arg == 0, "Attempting to pack a reserved value.");
    elem_size_arg << 32 | num_elems_arg
  }

  fn unpack_alloc_size_args(num: u64) -> (u64, u64) {
    let num_elems = num & u64::MAX;
    let elem_size_arg = num >> 32;
    let mut num_elems_arg = 0;
    if num_elems as i64 != -1 {
      num_elems_arg = num.clone();
    }
    (elem_size_arg, num_elems_arg)
  }

  fn pack_vscale_range_args(min_value: u64, max_value: u64) -> u64 {
    max_value << 32 | min_value
  }

  fn unpack_vscale_range_args(value: u64) -> (u64, u64) {
    let max_value = value & u64::MAX;
    let min_value = value >> 32;
    (min_value, max_value)
  }
}

// This class holds the attributes for a particular argument, parameter,
// function, or return value.
// It is an immutable value type that is cheap to copy.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeSet {
  pub set_node: Option<AttributeSetNode>
}

impl AttributeSet {
  pub fn new(asn: Option<AttributeSetNode>) -> Self {
    AttributeSet { set_node: asn }
  }

  pub fn new_default() -> Self {
    AttributeSet { set_node: Some(AttributeSetNode::new_default()) }
  }

  pub fn get_by_builder(c: &mut BlitzContext, b: &AttrBuilder) -> Self {
    AttributeSet::new(AttributeSetNode::get_by_builder(c, b))
  }

  pub fn get_by_attrs(c: &mut BlitzContext, attrs: &Vec<Attribute>) -> Self {
    AttributeSet::new(AttributeSetNode::get_by_attrs(c, attrs))
  }

  // Add an argument attribute.
  // Returns a new set because attribute sets are immutable.
  pub fn add_attribute_by_kind(&self, c: &BlitzContext, kind: &AttrKind) {
    if self.has_attribute(kind) {
      //return self;
    }
    let mut b = AttrBuilder::new(c);
    b.add_attribute_by_kind(kind);
  }

  // Add attributes to the attribute set.
  // Returns a new set because attribute sets are immutable.
  pub fn add_attributes(&self, c: &mut BlitzContext, attrs: AttributeSet) -> Self {
    if !self.has_attributes() {
      return attrs;
    }
    if !attrs.has_attributes() {
      return self.clone();
    }
    let mut b = AttrBuilder::new_from_attr_set(c, self);
    b.merge(&AttrBuilder::new_from_attr_set(c, &attrs));
    AttributeSet::get_by_builder(c, &b)
  }

  // Remove the specified attribute from this set.
  // Returns a new set because attribute sets are immutable.
  pub fn remove_attribute_by_kind(&self, c: &mut BlitzContext, kind: &AttrKind) -> Self {
    if !self.has_attribute(kind) {
      return  self.clone();
    }
    let mut b = AttrBuilder::new_from_attr_set(c, self);
    b.remove_attribute_by_kind(kind);
    AttributeSet::get_by_builder(c, &b)
  }

  // Remove the specified attributes from this set.
  // Returns a new set because attribute sets are immutable.
  pub fn remove_attributes(&self, c: &mut BlitzContext, mask: &AttributeMask) -> Self{
    let mut b = AttrBuilder::new_from_attr_set(c, self);
    if !b.overlaps(mask) { return self.clone(); }
    b.remove(mask);
    AttributeSet::get_by_builder(c, &b)
  }

  pub fn get_num_attributes() {}

  // Return true if attributes exist in this set.
  pub fn has_attributes(&self) -> bool {
    self.set_node.is_some()
  }

  // Return true if the attribute exists in this set.s
  pub fn has_attribute(&self, kind: &AttrKind) -> bool {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().has_attribute(kind);
    }
    false
  }

  // Return the attribute object.
  pub fn get_attribute(&self, kind: &AttrKind) -> Option<Attribute> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_attribute(kind);
    }
    None
  }

  pub fn get_alignment(&self) -> Option<MaybeAlign> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_alignment();
    }
    None
  }

  pub fn get_stack_alignment(&self) -> Option<MaybeAlign> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_stack_alignment();
    }
    None
  }

  pub fn get_dereferenceable_bytes(&self) -> u64 {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_dereferenceable_bytes();
    }
    0
  }

  pub fn get_dereferenceable_or_null_bytes(&self) -> u64 {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_dereferenceable_or_null_bytes();
    }
    0
  }

  pub fn get_by_val_type(&self) -> Option<Box<dyn Type>> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_attribute_type(AttrKind::ByVal);
    }
    None
  }

  pub fn get_struct_ret_type(&self) -> Option<Box<dyn Type>> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_attribute_type(AttrKind::StructRet);
    }
    None
  }

  pub fn get_by_ref_type(&self) -> Option<Box<dyn Type>> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_attribute_type(AttrKind::ByRef);
    }
    None
  }

  pub fn get_preallocated_type(&self) -> Option<Box<dyn Type>> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_attribute_type(AttrKind::Preallocated);
    }
    None
  }

  pub fn get_in_alloca_type(&self) -> Option<Box<dyn Type>> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_attribute_type(AttrKind::InAlloca);
    }
    None
  }

  pub fn get_element_type(&self) -> Option<Box<dyn Type>> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_attribute_type(AttrKind::ElementType);
    }
    None
  }

  pub fn get_alloc_size_args(&self) -> Option<(u64, u64)> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_alloc_size_args();
    }
    None
  }

  pub fn get_vscale_range_min(&self) -> u64 {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_v_scale_range_min();
    }
    1
  }

  pub fn get_vscale_range_max(&self) -> Option<u64> {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_v_scale_range_max();
    }
    None
  }

  pub fn get_uw_table_kind(&self) -> UWTableKind {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_uw_table_kind();
    }
    UWTableKind::None
  }

  pub fn get_alloc_kind(&self) -> AllocFnKind {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_alloc_kind()
    }
    AllocFnKind::Unknown
  }

  pub fn get_memory_effects(&self) -> MemoryEffects {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_memory_effects();
    }
    MemoryEffects::unknown()
  }

  pub fn get_no_fp_class(&self) -> FPClassTest {
    if self.set_node.is_some() {
      return self.set_node.as_ref().unwrap().get_no_fp_class();
    }
    FPClassTest::None
  }

  pub fn get_as_string() {}

  // Return true if this attribute set belongs to the BlitzContext.
  pub fn has_parent_context(&self, _c: &BlitzContext) -> bool {
    debug_assert!(self.has_attributes(), "Empty AttributeSet doesn't refer to any context.");
    //let id = FoldingSetNodeID::new();
    //self.set_node.as_ref().unwrap().p
    false
  }
}


pub enum AttrIndex {
  ReturnIndex = 20000, // TODO: 0
  FunctionIndex = 10000, // TODO:  !0,
  FirstArgIndex = 1
}

// This class holds the attributes for a function, its return value, and
// its parameters. You access the the attributes for each of them via an
// index into the AttributeList object.
#[derive(Debug)]
pub struct AttributeList {
  pimpl: Option<AttributeListImpl>
}

impl AttributeList {
  pub fn new(pimpl: Option<AttributeListImpl>) -> Self {
    AttributeList { pimpl: pimpl }
  }

  pub fn new_default() -> Self {
    AttributeList { pimpl: Some(AttributeListImpl::new_default()) }
  }

  fn get_impl(c: &mut BlitzContext, attr_sets: Vec<AttributeSet>) -> AttributeList {
    debug_assert!(!attr_sets.is_empty(), "Pointless AttributeListImpl.");
    let mut id = FoldingSetNodeID::new();
    AttributeListImpl::profile(&mut id, &attr_sets);

    let attr_list_impl = c.attrs_lists.get(&id);
    if attr_list_impl.is_none() {
      let list = AttributeListImpl::new(&attr_sets);
      c.attrs_lists.insert(id, list.clone());
      return AttributeList::new(Some(list));
    } else {
      return AttributeList::new(Some(attr_list_impl.unwrap().clone()))
    }
  }

  fn set_attributes_at_index(&self, c: &mut BlitzContext, i: usize,
    attrs: AttributeSet) -> Self
  {
    let pimpl = self.pimpl.clone();
    let mut attr_sets = pimpl.unwrap().attr_sets.clone();
    if i > attr_sets.len() {
      attr_sets.resize(i + 1, attrs.clone());
    }
    //attr_sets.insert(i, attrs);
    attr_sets.push(attrs);
    attr_sets.swap_remove(i); 
    AttributeList::get_impl(c, attr_sets)
  }

  // Add an attribute to the attribute set at the given index.
  // Returns a new list because attribute lists are immutable.
  pub fn add_attribute_at_index_by_kind(&self, c: &mut BlitzContext,
    i: usize, kind: &AttrKind) -> Self
  {
    let attrs = self.get_attributes(i);
    if attrs.has_attribute(kind) {
      return AttributeList::new(self.pimpl.clone());
    }
    let mut new_attrs = Vec::new();
    new_attrs.clone_from(&attrs.set_node.unwrap().attrs);
    new_attrs.push(Attribute::get_by_int(c, kind.clone(), 0));

    let new_attr_set = AttributeSet::get_by_attrs(c, &new_attrs);
    self.set_attributes_at_index(c, i, new_attr_set)
  }

  pub fn add_attribute_at_index_by_string(&self) {}

  // Add an attribute to the attribute set at the given index.
  // Returns a new list because attribute lists are immutable.
  pub fn add_attribute_at_index(&self, c: &mut BlitzContext,
    _i: usize, attr: &Attribute) -> Self
  {
    let mut b = AttrBuilder::new(c);
    b.add_attribute(attr);

    AttributeList::new(None)
  }

  // Add an attributes to the attribute set at the given index.
  // Returns a new list because attribute lists are immutable.
  pub fn add_attributes_at_index(&self, c: &mut BlitzContext,
    i: usize, b: &AttrBuilder) -> Self
  {
    if !b.has_attributes() { 
      return AttributeList::new(self.pimpl.clone());
    }
    if self.pimpl.is_none() {
      // TODO
    }
    let mut merged = AttrBuilder::new_from_attr_set(c,
      &self.get_attributes(i));
    merged.merge(b);

    let attrs = AttributeSet::get_by_builder(c, &merged);
    self.set_attributes_at_index(c, i, attrs)
  }

  // Add a function attribute to the list.
  // Returns a new list because attribute lists are immutable.
  pub fn add_fn_attribute_by_kind(&self, c: &mut BlitzContext, kind: &AttrKind) -> Self {
    self.add_attribute_at_index_by_kind(c, AttrIndex::FunctionIndex as usize, kind)
  }

  pub fn add_fn_attribute_by_string() {}

  // Add a function attribute to the list.
  // Returns a new list because attribute lists are immutable.
  pub fn add_fn_attribute(&self, c: &mut BlitzContext, attr: &Attribute) -> Self {
    self.add_attribute_at_index(c, AttrIndex::FunctionIndex as usize, attr)
  }

  // Add a function attributes to the list.
  // Returns a new list because attribute lists are immutable.
  pub fn add_fn_attributes(&self, c: &mut BlitzContext, b: &AttrBuilder) -> Self {
    self.add_attributes_at_index(c, AttrIndex::FunctionIndex as usize, b)
  }

  // Add a return value attribute to the list.
  // Returns a new list because attribute lists are immutable.
  pub fn add_ret_attribute_by_kind(&self, c: &mut BlitzContext, kind: &AttrKind) -> Self {
    self.add_attribute_at_index_by_kind(c, AttrIndex::ReturnIndex as usize, kind)
  }

  // Add a return value attribute to the list.
  // Returns a new list because attribute lists are immutable.
  pub fn add_ret_attribute(&self, c: &mut BlitzContext, attr: &Attribute) -> Self {
    self.add_attribute_at_index(c, AttrIndex::ReturnIndex as usize, attr)
  }

  // Add a return value attributes to the list.
  // Returns a new list because attribute lists are immutable.
  pub fn add_ret_attributes(&self, c: &mut BlitzContext, b: &AttrBuilder) -> Self {
    self.add_attributes_at_index(c, AttrIndex::ReturnIndex as usize, b)
  }

  // Add an argument attribute to the list.
  // Returns a new list because attribute lists are immutable.
  pub fn add_param_attribute_by_kind(&self, c: &mut BlitzContext,
    arg_no: usize, kind: &AttrKind) -> Self
  {
    self.add_attribute_at_index_by_kind(c,
      arg_no + AttrIndex::FirstArgIndex as usize, kind)
  }

  pub fn add_param_attribute_by_string() {}

  // Add an attribute to the attribute list at the given arg indices,
  // Returns a new list because attribute lists are immutable.
  pub fn add_param_attribute(&self, c: &mut BlitzContext,
    mut arg_nos: Vec<usize>, attr: &Attribute) -> Self
  {
    arg_nos.sort();
    let max_index = arg_nos.last().unwrap().clone() +
      AttrIndex::FirstArgIndex as usize;
    
    let mut attr_sets =
      self.pimpl.as_ref().unwrap().attr_sets.clone();
    if attr_sets.len() < max_index {
      attr_sets.resize(max_index + 1,
        AttributeSet::new_default());
    }

    for arg_no in arg_nos {
      let index = arg_no + AttrIndex::FirstArgIndex as usize;
      let attr_set = attr_sets.get(index);
      if attr_set.is_some() {
        let mut b =
          AttrBuilder::new_from_attr_set(c, attr_set.unwrap());
        b.add_attribute(attr);
        let attrs = AttributeSet::get_by_builder(c, &b);
        attr_sets.insert(index, attrs);
      } else {
        let mut b = AttrBuilder::new(c);
        b.add_attribute(attr);
        let attrs = AttributeSet::get_by_builder(c, &b);
        attr_sets.insert(index, attrs);
      }
    }

    AttributeList::get_impl(c, attr_sets)
  }

  // Add an argument attribute to the list.
  // Returns a new list because attribute lists are immutable.
  pub fn add_param_attributes(&self, c: &mut BlitzContext,
    arg_no: usize, b: &AttrBuilder) -> Self
  {
    self.add_attributes_at_index(c, 
      arg_no + AttrIndex::FirstArgIndex as usize, b)
  }

  // Remove the specified attribute at the specified index from this
  // attribute list. Returns a new list because attribute lists are immutable.
  pub fn remove_attribute_at_index_by_kind(&self, c: &mut BlitzContext,
    i: usize, kind: &AttrKind) -> Self
  {
    let attrs = self.get_attributes(i);
    let new_attrs = attrs.remove_attribute_by_kind(c, kind);
    if attrs == new_attrs { 
      return AttributeList::new(self.pimpl.clone());
    }
    self.set_attributes_at_index(c, i, new_attrs)
  }

  pub fn remove_attribute_at_index_by_string() {}

  // Remove the specified attributes at the specified index from this attribute list.
  // Returns a new list because attribute lists are immutable.
  pub fn remove_attributes_at_index_by_mask(&self, c: &mut BlitzContext,
    i: usize, mask: &AttributeMask) -> Self
  {
    let attrs = self.get_attributes(i);
    let new_attrs = attrs.remove_attributes(c, mask);
    if attrs == new_attrs { 
      return AttributeList::new(self.pimpl.clone());
    }
    self.set_attributes_at_index(c, i, new_attrs)
  }

  // Remove all attributes at the specified index from this attribute list.
  // Returns a new list because attribute lists are immutable.
  pub fn remove_attributes_at_index(&self, c: &mut BlitzContext, i: usize) -> Self {
    if self.pimpl.is_none() { return AttributeList::new(None); }
    if i > self.get_num_attr_sets() {
      return AttributeList::new(self.pimpl.clone());
    }
    self.set_attributes_at_index(c, i, AttributeSet::new(None))
  }

  // Remove the specified attribute at the function index from this
  // attribute list. Returns a new list because attribute lists are immutable.
  pub fn remove_fn_attribute(&self, c: &mut BlitzContext, kind: &AttrKind) -> Self {
    self.remove_attribute_at_index_by_kind(c, AttrIndex::FunctionIndex as usize, kind)
  }

  pub fn remove_fn_attribute_by_string() {}

  // Remove the attributes at the function index from this attribute list.
  // Returns a new list because attribute lists are immutable.
  pub fn remove_fn_attributes(&self, c: &mut BlitzContext) -> Self {
    self.remove_attributes_at_index(c, AttrIndex::FunctionIndex as usize)
  }

  // Remove the specified attribute at the return value index from this
  // attribute list. Returns a new list because attribute lists are immutable.
  pub fn remove_ret_attirbute_by_kind(&self, c: &mut BlitzContext,
    kind: &AttrKind) -> Self
  {
    self.remove_attribute_at_index_by_kind(c,
      AttrIndex::ReturnIndex as  usize, kind)
  }

  pub fn remove_ret_attirbute_by_string() {}

  // Remove the specified attributes at the return value index from this
  // attribute list. Returns a new list because attribute lists are immutable.
  pub fn remove_ret_attributes(&self, c: &mut BlitzContext,
    mask: &AttributeMask) -> Self
  {
    self.remove_attributes_at_index_by_mask(c,
      AttrIndex::ReturnIndex as usize, mask)
  }

  // Remove the specified attribute at the specified arg index from this
  // attribute list. Returns a new list because attribute lists are immutable.
  pub fn remove_param_attribute_by_kind(&self, c: &mut BlitzContext,
    arg_no: usize, kind: &AttrKind) -> Self
  {
    self.remove_attribute_at_index_by_kind(c,
      arg_no + AttrIndex::FirstArgIndex as usize, kind)
  }

  pub fn remove_param_attribute_by_string() {}

  // Remove the specified attributes at the specified arg index from this
  // attribute list. Returns a new list because attribute lists are immutable.  
  pub fn remove_param_attributes(&self, c: &mut BlitzContext,
    arg_no: usize, mask: &AttributeMask) -> Self
  {
    self.remove_attributes_at_index_by_mask(c,
      arg_no + AttrIndex::FirstArgIndex as usize, mask)
  }

  // Remove attributes at the specified arg index from this attribute list.
  // Returns a new list because attribute lists are immutable.  
  pub fn remove_param_attributes_at_index(&self, c: &mut BlitzContext,
    arg_no: usize) -> Self
  {
    self.remove_attributes_at_index(c,
      arg_no + AttrIndex::FirstArgIndex as usize)
  }

  pub fn replace_attribute_type_at_index() {}

  // Add the dereferenceable attribute to the attribute set at the given index.
  // Returns a new list because attribute lists are immutable.
  pub fn add_dereferenceable_ret_attr(&self, c: &mut BlitzContext,
    bytes: u64) -> Self
  {
    let mut b = AttrBuilder::new(c);
    b.add_dereferenceable_attr(bytes);
    self.add_ret_attributes(c, &b)
  }

  // Add the dereferenceable attribute to the attribute set at the given arg index.
  // Returns a new list because attribute lists are immutable.
  pub fn add_dereferenceable_param_attr(&self, c: &mut BlitzContext,
    i: usize, bytes: u64) -> Self
  {
    let mut b = AttrBuilder::new(c);
    b.add_dereferenceable_attr(bytes);
    self.add_param_attributes(c, i, &b)
  }

  // Add the dereferenceable_or_null attribute to the attribute set at the
  // given arg index. Returns a new list because attribute lists are immutable.
  pub fn add_dereferenceable_or_null_param_attr(&self, c: &mut BlitzContext,
    arg_no: usize, bytes: u64) -> Self
  {
    let mut b = AttrBuilder::new(c);
    b.add_dereferenceable_or_null_attr(bytes);
    self.add_param_attributes(c, arg_no, &b)
  }

  // Add the allocsize attribute to the attribute set at the given arg index.
  // Returns a new list because attribute lists are immutable.
  pub fn add_alloc_size_param_attr(&self, c: &mut BlitzContext,
    arg_no: usize, elm_size_arg: usize, num_elms_arg: Option<usize>) -> Self
  {
    let mut b = AttrBuilder::new(c);
    b.add_alloc_size_attr(elm_size_arg, num_elms_arg);
    self.add_param_attributes(c, arg_no, &b)
  }

  // The attributes for the specified index are returned.
  pub fn get_attributes(&self, index: usize) -> AttributeSet {
    if self.pimpl.is_none() || index >= self.get_num_attr_sets() {
      //return AttributeSet::new(None);
      return AttributeSet::new_default();
    }
    let attr_set =
      self.pimpl.as_ref().unwrap().attr_sets.get(index);
    debug_assert!(attr_set.is_some(), "Target attr_set is None.");
    attr_set.unwrap().clone()
  }

  // The attributes for the argument or parameter at the given index
  // are returned.
  pub fn get_param_attrs(&self, arg_no: usize) -> AttributeSet {
    self.get_attributes(arg_no + AttrIndex::FirstArgIndex as usize)
  }

  // The attributes for the ret value are returned.
  pub fn get_ret_attrs(&self) -> AttributeSet {
    self.get_attributes(AttrIndex::ReturnIndex as usize)
  }

  // The function attributes are returned.
  pub fn get_fn_attrs(&self) -> AttributeSet {
    self.get_attributes(AttrIndex::FunctionIndex as usize)
  }

  // Return true if the attribute exists at the given index.
  pub fn has_attribute_at_index_by_kind(&self, index: usize, kind: &AttrKind) -> bool {
    self.get_attributes(index).has_attribute(kind)
  }

  pub fn has_attribute_at_index_by_string() {}

  // Return true if attribute exists at the given index.
  pub fn has_attributes_at_index(&self, index: usize) -> bool {
    self.get_attributes(index).has_attributes()
  }

  // Return true if the attribute exists for the given argument.
  pub fn has_param_attr(&self, arg_no: usize, kind: &AttrKind) -> bool {
    self.has_attribute_at_index_by_kind(
      arg_no + AttrIndex::FirstArgIndex as usize, kind)
  }

  // Return true if attributes exists for the given argument.
  pub fn has_param_attrs(&self, arg_no: usize) -> bool {
    self.has_attributes_at_index(arg_no + AttrIndex::FirstArgIndex as usize)
  }

  // Return true if the attribute exists for the return value.
  pub fn has_ret_attr(&self, kind: &AttrKind) -> bool {
    self.has_attribute_at_index_by_kind(
      AttrIndex::ReturnIndex as usize, kind)
  }

  // Return true if attributes exist for the return value.
  pub fn has_ret_attrs(&self) -> bool {
    self.has_attributes_at_index(AttrIndex::ReturnIndex as usize)
  }

  // Return true if the attribute exists for the function.
  pub fn has_fn_attr(&self, kind: &AttrKind) -> bool {
    self.pimpl.is_some() && self.pimpl.as_ref().unwrap().has_fn_attribute(kind)
  }

  // Return true if the attributes exist for the function.
  pub fn has_fn_attrs(&self) -> bool {
    self.has_attributes_at_index(AttrIndex::FunctionIndex as usize)
  }

  // Return true if the specified attribute is set for at least one 
  // parameter or for the return value.
  // If index is not None, the index of a parameter with the specified
  // attribute id provided.
  pub fn has_attr_somewhere(&self, kind: &AttrKind,
    index: Option<usize>) -> bool
  {
    self.pimpl.is_some() &&
    self.pimpl.as_ref().unwrap().has_attr_somewhere(kind, index)
  }

  // Return the attribute object that exists at the given idex.
  pub fn get_attribute_at_index(&self, index: usize,
    kind: &AttrKind) -> Option<Attribute>
  {
    self.get_attributes(index).get_attribute(kind)
  }

  // Return the attribute object that exists at the arg index.
  pub fn get_param_attr(&self, arg_no: usize, kind: &AttrKind) -> Option<Attribute> {
    self.get_attribute_at_index(arg_no, kind)
  }

  // Return the attribute object that exists for the function.
  pub fn get_fn_attr(&self, kind: &AttrKind) -> Option<Attribute> {
    self.get_attribute_at_index(AttrIndex::FunctionIndex as usize, kind)
  }

  // Return the alignment of the return value.
  pub fn get_ret_alignment(&self) -> Option<MaybeAlign> {
    self.get_attributes(AttrIndex::ReturnIndex as usize).get_alignment()
  }

  // Return the alignment for the specified function parameter.
  pub fn get_param_alignment(&self, arg_no: usize) -> Option<MaybeAlign> {
    self.get_attributes(arg_no + AttrIndex::FirstArgIndex as usize).
      get_alignment()
  }

  // Return the stack alignment for the specified function parameter.
  pub fn get_param_stack_alignment(&self, arg_no:usize) -> Option<MaybeAlign> {
    self.get_attributes(arg_no + AttrIndex::FirstArgIndex as usize).
      get_stack_alignment()
  }

  // Return the byval type for the specified function parameter.
  pub fn get_param_by_val_type(&self, index: usize) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as usize).
      get_by_val_type()
  }

  // Return the sret type for the specified function parameter.
  pub fn get_param_struct_ret_type(&self, index: usize) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as usize).
      get_struct_ret_type()
  }

  // Return the byref type for the specified function parameter.
  pub fn get_param_by_ref_type(&self, index: usize) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as usize).
      get_by_ref_type()
  }

  // Return the preallocated type for the specified function parameter.
  pub fn get_param_preallocated_type(&self, index: usize) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as usize).
      get_preallocated_type()
  }

  // Return the inalloca type for the specified function parameter.
  pub fn get_param_in_alloca_type(&self, index: usize) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as usize).
      get_in_alloca_type()
  }

  // Return the elementtype type for the specified function parameter.
  pub fn get_param_element_type(&self, index: usize) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as usize).
      get_element_type()
  }

  // Get the stack alignment of the function.
  pub fn get_fn_stack_alignment(&self) -> Option<MaybeAlign> {
    self.get_fn_attrs().get_stack_alignment()
  }

  // Get the stack alignment of the return type.
  pub fn get_ret_stack_alignment(&self) -> Option<MaybeAlign> {
    self.get_ret_attrs().get_stack_alignment()
  }

  // Get the number of dereferenceable bytes (or zero if unknown) of
  // the return value.
  pub fn get_ret_dereferenceable_bytes(&self) -> u64 {
    self.get_ret_attrs().get_dereferenceable_bytes()
  }

  // Get the number of dereferenceable bytes (or zero if unknown) of
  // an arg.
  pub fn get_param_dereferenceable_bytes(&self, index: usize) -> u64 {
    self.get_param_attrs(index).get_dereferenceable_bytes()
  }

  // Get the number of dereferenceable_or_null bytes (or zero if unknown)
  // of the return value.
  pub fn get_ret_dereferenceable_or_null_bytes(&self) -> u64 {
    self.get_ret_attrs().get_dereferenceable_or_null_bytes()
  }

  // Get the number of dereferenceable_or_null bytes (or zero if unknown)
  // of an arg.
  pub fn get_param_dereferenceable_or_null_bytes(&self, index: usize) -> u64 {
    self.get_param_attrs(index).get_dereferenceable_or_null_bytes()
  }

  // Get the diallowed floating-point classes of the return value.
  pub fn get_ret_no_fp_class(&self) -> FPClassTest {
    self.get_ret_attrs().get_no_fp_class()
  }

  // Get the diallowed floating-point classes of the argument value.
  pub fn get_param_no_fp_class(&self, arg_no: usize) -> FPClassTest {
    self.get_param_attrs(arg_no).get_no_fp_class()
  }

  // Get the unwind table kind requested for the function.
  pub fn get_uw_table_kind(&self) -> UWTableKind {
    self.get_fn_attrs().get_uw_table_kind()
  }

  pub fn get_alloc_kind(&self) -> AllocFnKind {
    self.get_fn_attrs().get_alloc_kind()
  }

  // Returns memory effects of the function.
  pub fn get_memory_effects(&self) -> MemoryEffects {
    self.get_fn_attrs().get_memory_effects()
  }

  pub fn get_as_string() {}
  pub fn has_parent_context() {}

  pub fn get_num_attr_sets(&self) -> usize {
    if self.pimpl.is_none() { return 0; }
    self.pimpl.as_ref().unwrap().num_trailing_objects()
  }

  // Return a pointer that uniquely identifies this attribute list.
  pub fn get_raw_pointer(&self) -> &Option<AttributeListImpl> {
    &self.pimpl
  }

  // Return true if there are no attributes.
  pub fn is_empty(&self) -> bool {
    self.pimpl.is_none()
  }

  pub fn dump() {}
}

// This class stores enough information to efficiently remove some
// attributes from an existing AttrBuilder, AttributeSet ot Attributelist.
#[derive(Debug, Clone)]
pub struct AttributeMask {
  attrs: [bool; 1000] // TODO: size
}

impl AttributeMask {
  pub fn new(attrset: &AttributeSet) -> Self {
    let mut instance = AttributeMask { attrs: [false; 1000] };
    let attrs = attrset.set_node.as_ref().unwrap().attrs.clone();
    for attr in attrs {
      instance.add_attribute(&attr);
    }
    instance
  }

  pub fn new_default() -> Self {
    AttributeMask { attrs: [false; 1000] }
  }

  // Add an attribute to the mask.
  pub fn add_attribute_by_kind(&mut self, val: &AttrKind) -> AttributeMask {
    let index = val.clone() as usize;
    self.attrs[index] = true;
    self.clone()
  }

  // Add the attribute object to the builder.
  pub fn add_attribute(&mut self, attr : &Attribute) {
    if attr.is_string_attribute() {
      // TODO
    } else {
      self.add_attribute_by_kind(attr.get_kind_as_enum());
    }
  }

  // Return true is the builder has the specified target-dependent attributes.
  pub fn contains_by_kind(&self, kind : &AttrKind) -> bool {
    let index = kind.clone() as usize;
    self.attrs[index]
  }

  // Return true if the mask contains the specified attribute.
  pub fn contains(&self, attr : &Attribute) -> bool {
    if attr.is_string_attribute() {
      return false; // TODO
    } else {
      return self.contains_by_kind(attr.get_kind_as_enum());
    }
  }
}

// This class is used in conjunction with the Attribute::get method to
// create an Attribute object. The object itself is uniquified.
// The builder's value, however, is not. So this can be a quick way to
// test for equality, presence of attributes, etc.
#[derive(Debug, Clone)]
pub struct AttrBuilder {
  //c: BlitzContext,
  pub attrs: Vec<Attribute>
}

impl AttrBuilder {
  pub fn new(_c: &BlitzContext) -> Self {
    AttrBuilder { attrs: Vec::new() }
  }

  pub fn new_from_attr_set(_c: &BlitzContext, attrs: &AttributeSet) -> Self {
    AttrBuilder {
      attrs: attrs.set_node.as_ref().unwrap().attrs.clone()
    }
  }

  pub fn clear(&mut self) {
    self.attrs.clear()
  }

  // Add an attribute to the builder.
  pub fn add_attribute_by_kind(&mut self, kind: &AttrKind) {
    let target =
      Attribute::get_by_int(blits_context_mut(), kind.clone(), 0);
    self.add_attribute_impl(kind, &target)
  }

  pub fn add_attribute_by_string() {}

  // Add the Attribute object to the builder.
  pub fn add_attribute(&mut self, attr: &Attribute) {
    if attr.is_string_attribute() {
      // TODO
    } else {
      let kind = attr.get_kind_as_enum().clone();
      self.add_attribute_impl(&kind, attr)
    }
  }

  // Remove an attribute from the builder.
  pub fn remove_attribute_by_kind(&mut self, kind: &AttrKind) {
    let mut has_attr = false;
    let mut index = 0;
    for attr in &self.attrs {
      if attr.has_attribute(kind) { has_attr = true; break; }
      index += 1;
    }
    if has_attr {
      self.attrs.remove(index);
    }
  }

  pub fn remove_attribute_by_string(&mut self) {}

  // Remove the target-dependent attribute from the builder.
  pub fn remove_attribute(&mut self, attr: Attribute) {
    if attr.is_string_attribute() {
      // TODO
    } else {
      let kind = attr.get_kind_as_enum().clone();
      self.remove_attribute_by_kind(&kind);
    }
  }

  // Add the attributes from the builder.
  // Attributes in the passed builder overwrite attributes in this builder
  // if they have the same key.
  pub fn merge(&mut self, b: &AttrBuilder) {
    let attrs = b.attrs.clone();
    for attr in attrs {
      self.add_attribute(&attr);
    }
  }

  // Remove the attributes from the builder.
  pub fn remove(&mut self, am: &AttributeMask) {
    let mut index = 0;
    let mut contains = false;
    for attr in &self.attrs {
      if am.contains(attr) { contains = true; break; }
      index += 1;
    }
    if contains { self.attrs.remove(index); }
  }

  // Return true if the builder has any attribute that's in the
  // specified builder.
  pub fn overlaps(&self, am: &AttributeMask) -> bool {
    for attr in &self.attrs {
      if am.contains(attr) { return true; }
    }
    false
  }

  // Return true if the builder has the specified attribute.
  pub fn contains_by_kind(&self, kind: &AttrKind) -> bool {
    if self.get_attribute_by_kind(kind).is_some() {
      return self.get_attribute_by_kind(kind).unwrap().is_valid();
    }
    false
  }

  pub fn contains_by_string() {}

  // Return true if the builder has IR-level attributes.
  pub fn has_attributes(&self) -> bool {
    !self.attrs.is_empty()
  }

  // Return attribute with the given kind.
  // The returned attribute will be invalid if the kind is not present
  // in the builder.
  pub fn get_attribute_by_kind(&self, kind: &AttrKind) -> Option<Attribute> {
    for attr in &self.attrs {
      if attr.has_attribute(kind) {
        return Some(attr.clone());
      }
    }
    None
  }

  pub fn get_attribute_by_string() {}

  // Return raw (possibly packed/encoded) value of integer attribute ot
  // None if not set.
  pub fn get_raw_int_attr(&self, kind: &AttrKind) -> Option<u64> {
    debug_assert!(Attribute::is_int_attr_kind(kind), "Not an int attribute.");
    let attr = self.get_attribute_by_kind(kind);
    if attr.is_some() && attr.as_ref().unwrap().is_valid() {
      return Some(attr.unwrap().get_value_as_int());
    }
    None
  }

  // Retrieve the alignment attribute, if it exists.
  pub fn get_alignment(&self) -> MaybeAlign {
    MaybeAlign::new(self.get_raw_int_attr(
      &AttrKind::Alignment).unwrap_or(0))
  }

  // Retrieve the stack alignment attribute, if it exists.
  pub fn get_stack_alignment(&self) -> MaybeAlign {
    MaybeAlign::new(self.get_raw_int_attr(
      &AttrKind::StackAlignment).unwrap_or(0))
  }

  // Retrieve the number of dereferenceable bytes, if the dereferenceable
  // attribute exists (zero is returned otherwise).
  pub fn get_dereferenceable_bytes(&self) -> u64 {
    self.get_raw_int_attr(&AttrKind::Dereferenceable).unwrap_or(0)
  }

  // Retrieve the number of dereferenceable_or_null bytes, if the
  // dereferenceable_or_null attribute exists (zero is returned otherwise).
  pub fn get_dereferenceable_or_null_bytes(&self) -> u64 {
    self.get_raw_int_attr(&AttrKind::DereferenceableOrNull).unwrap_or(0)
  }

  // Retrieve type for the given type attribute.
  pub fn get_type_attr(&self, kind: &AttrKind) -> Option<Box<dyn Type>> {
    debug_assert!(Attribute::is_type_attr_kind(kind), "Not a type attribute.");
    let attr = self.get_attribute_by_kind(kind);
    if attr.is_some() && attr.as_ref().unwrap().is_valid() {
      return attr.unwrap().get_value_as_type();
    }
    None
  }

  // Retrieve the byval type.
  pub fn get_by_val_type(&self) -> Option<Box<dyn Type>> {
    self.get_type_attr(&AttrKind::ByVal)
  }

  // Retrieve the sret type.
  pub fn get_struct_ret_type(&self) -> Option<Box<dyn Type>> {
    self.get_type_attr(&AttrKind::StructRet)
  }

  // Retrieve the byref type.
  pub fn get_by_ref_type(&self) -> Option<Box<dyn Type>> {
    self.get_type_attr(&AttrKind::ByRef)
  }

  // Retrieve the preallocated type.
  pub fn get_preallocated_type(&self) -> Option<Box<dyn Type>> {
    self.get_type_attr(&AttrKind::Preallocated)
  }

  // Retrieve the inalloca type.
  pub fn get_in_alloca_type(&self) -> Option<Box<dyn Type>> {
    self.get_type_attr(&AttrKind::InAlloca)
  }

  pub fn get_alloc_size_args() {}

  // Add integer attribute with raw value (packed/encoded if necessary).
  pub fn add_raw_int_attr(&mut self, kind: &AttrKind, value: u64) {
    self.add_attribute(&Attribute::get_by_int(blits_context_mut(),
      kind.clone(), value))
  }

  // This turns an alignment into the form used internally in Attribute.
  // This call has no effect if align is not set.
  pub fn add_alignment_attr(&mut self, align: &MaybeAlign) {
    if align.value() == 0 { return; }
    debug_assert!(align.value() <= value::MAXIMUM_ALIGNMENT, "Alignment is too large.");
    // Original code use align.value(), but it is not going well.
    self.add_raw_int_attr(&AttrKind::Alignment, align.shift_value() /*align.value()*/)
  }

  // This turns a stacj alignment into the form used internalldy in Attribute.
  // This call has no effect if align is not set.
  pub fn add_stack_alignment_attr(&mut self, align: &MaybeAlign) {
    if align.value() == 0 { return; }
    debug_assert!(align.value() <= 0x100, "Alignment is too large.");
    self.add_raw_int_attr(&AttrKind::StackAlignment, align.value())
  }

  // This turns the number of dereferenceable bytes into the form used
  // internally in Attribute.
  pub fn add_dereferenceable_attr(&mut self, bytes: u64) {
    if bytes == 0 { return; }
    self.add_raw_int_attr(&AttrKind::Dereferenceable, bytes)
  }

  // This turns the number of dereferenceable_or_null bytes into the form
  // used internally in Attribute.
  pub fn add_dereferenceable_or_null_attr(&mut self, bytes: u64) {
    if bytes == 0 { return; }
    self.add_raw_int_attr(&AttrKind::DereferenceableOrNull, bytes)
  }

  // This turns one (or two) ints into the form used internally in Attribute.
  pub fn add_alloc_size_attr(&mut self, elm_size_arg: usize, num_elms_arg: Option<usize>) {
    let val = AttrBuilder::pack_alloc_size_args(elm_size_arg, num_elms_arg);
    self.add_alloc_size_attr_from_raw_repr(val)
  }

  // This turns two ints into the form used internally in Attribute.
  pub fn add_vscale_range_attr(&mut self, min_value: usize, max_value: Option<usize>) {
    let val = AttrBuilder::pack_alloc_size_args(min_value, max_value);
    self.add_vscale_range_attr_from_raw_repr(val)
  }

  // Add a type attribute with the given type.
  pub fn add_type_attr(&mut self, kind: &AttrKind, t: Box<dyn Type>) {
    self.add_attribute(&Attribute::get_by_type(blits_context_mut(),
      kind.clone(), t))
  }

  // This turns a byval type into the form used internally in Attribute,
  pub fn add_by_val_attr(&mut self, t: Box<dyn Type>) {
    self.add_type_attr(&AttrKind::ByVal, t)
  }

  // This turns a sret type into the form used internally in Attribute,
  pub fn add_struct_ret_attr(&mut self, t: Box<dyn Type>) {
    self.add_type_attr(&AttrKind::StructRet, t)
  }

  // This turns a byref type into the form used internally in Attribute,
  pub fn add_by_ref_attr(&mut self, t: Box<dyn Type>) {
    self.add_type_attr(&AttrKind::ByRef, t)
  }

  // This turns a preallocated type into the form used internally in Attribute,
  pub fn add_preallocated_attr(&mut self, t: Box<dyn Type>) {
    self.add_type_attr(&AttrKind::Preallocated, t)
  }

  // This turns a inalloca type into the form used internally in Attribute,
  pub fn add_in_alloca_attr(&mut self, t: Box<dyn Type>) {
    self.add_type_attr(&AttrKind::InAlloca, t)
  }

  // Add an allocsize attribute, using the representaton returned by
  // Attribute.get_int_value()
  pub fn add_alloc_size_attr_from_raw_repr(&mut self, raw_args: u64) {
    debug_assert!(raw_args != 0, "Invalid allocsize arguments - given allocsize(0, 0).");
    self.add_raw_int_attr(&AttrKind::AllocSize, raw_args)
  }

  // Add an vscale_range attribute, using the representaton returned by
  // Attribute.get_int_value()
  pub fn add_vscale_range_attr_from_raw_repr(&mut self, raw_args: u64) {
    if raw_args == 0 { return; }
    self.add_raw_int_attr(&AttrKind::VScaleRange, raw_args)
  }
  
  // This turns the unwind table kind into the form used internally in Attribute.
  pub fn add_uw_table_attr(&mut self, kind: &UWTableKind) {
    if *kind == UWTableKind::None { return; }
    self.add_raw_int_attr(&AttrKind::UWTable, kind.clone() as u64)
  }

  // This turns the allocator kind into the form used internally in Attribute.
  pub fn add_alloc_kind_attr(&mut self, kind: &AllocFnKind) {
    self.add_raw_int_attr(&AttrKind::AllocKind, kind.clone() as u64)
  }

  // Add memory effect attribute.
  pub fn add_memory_attr(&mut self, me: &MemoryEffects) {
    self.add_raw_int_attr(&AttrKind::Memory, me.to_int_value() as u64)
  }

  // Add nofpclass attribute.
  pub fn add_no_fp_class_attr(&mut self, mask: &FPClassTest) {
    if *mask == FPClassTest::None { return; }
    self.add_raw_int_attr(&AttrKind::NoFPClass, mask.clone() as u64)
  }

  pub fn attrs(&self) -> &Vec<Attribute> {
    &self.attrs
  }

  fn add_attribute_impl(&mut self, kind: &AttrKind, target: &Attribute) {
    let mut index = 0;
    let mut has_attr = false;
    for attr in &self.attrs {
      if attr.has_attribute(kind) { has_attr = true; break; }
      index += 1;
    }

    if has_attr {
      self.attrs.remove(index);
      self.attrs.insert(index, target.clone());
    } else {
      self.attrs.push(target.clone());
    }
  }

  fn pack_alloc_size_args(elm_size_arg: usize, num_elms_arg: Option<usize>) -> u64 {
    //debug_assert!(num_elms_arg.is_none() || num_elms_arg.as_ref() != -1,
      //"Attempting to pack a reserved value.");
    (elm_size_arg << 32) as u64 | num_elms_arg.unwrap() as u64
  }
}

pub enum AttributeSafetyKind {
  AskSafeToDrop = 1,
  AskUnsafeToDrop = 2
}

pub fn is_no_fp_class_compatible_type() {}
pub fn type_incompatible() {}
pub fn get_ub_implying_attributes() {}
pub fn are_inline_compatible() {}
pub fn are_outline_compatible() {}
pub fn merge_attributes_for_inlining() {}
pub fn merge_attributes_for_outlining() {}
pub fn update_min_legal_vector_with_attr() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_uniquing() {
    let mut c = BlitzContext::new();
    let attr_a = Attribute::get_by_int(&mut c,
      AttrKind::AlwaysInline, 0);
    let attr_b = Attribute::get_by_int(&mut c,
      AttrKind::AlwaysInline, 0);
    assert_eq!(attr_a, attr_b);
  }

  #[test]
  fn test_add_attributes() {
    let mut c = BlitzContext::new();
    let mut b = AttrBuilder::new(&c);
    b.add_attribute_by_kind(&AttrKind::NoReturn);

    let mut al = AttributeList::new_default();
    let attrs = AttributeSet::get_by_builder(&mut c, &b);
    let b2 = AttrBuilder::new_from_attr_set(&c, &attrs);
    al = al.add_fn_attributes(&mut c, &b2);
    assert_eq!(al.has_fn_attr(&AttrKind::NoReturn), true);

    b.clear();
    b.add_attribute_by_kind(&AttrKind::SExt);
    al = al.add_ret_attributes(&mut c, &b);
    assert_eq!(al.has_ret_attr(&AttrKind::SExt), true);
    assert_eq!(al.has_fn_attr(&AttrKind::NoReturn), true);
  }

  #[test]
  fn test_remove_align() {
    let mut c = BlitzContext::new();
    let align_attr =
      Attribute::get_by_alignment(&mut c, Align::new(8));
    let stack_align_attr =
      Attribute::get_by_stack_alignment(&mut c, Align::new(32));

    let mut b_align_readonly = AttrBuilder::new(&c);
    b_align_readonly.add_attribute(&align_attr);
    b_align_readonly.add_attribute_by_kind(&AttrKind::ReadOnly);

    let mut b_align = AttributeMask::new_default();
    b_align.add_attribute(&align_attr);

    let mut b_stackalign_optnone = AttrBuilder::new(&c);
    b_stackalign_optnone.add_attribute(&stack_align_attr);
    b_stackalign_optnone.add_attribute_by_kind(&AttrKind::OptimizeNone);

    let mut b_stack_align = AttributeMask::new_default();
    b_stack_align.add_attribute(&stack_align_attr);

    let mut attrs =
      AttributeSet::get_by_builder(&mut c, &b_align_readonly);
    assert_eq!(attrs.get_alignment().unwrap(), MaybeAlign::new(8));
    assert_eq!(attrs.has_attribute(&AttrKind::ReadOnly), true);
    attrs = attrs.remove_attribute_by_kind(&mut c, &AttrKind::Alignment);
    assert_eq!(attrs.has_attribute(&AttrKind::Alignment), false);
    assert_eq!(attrs.has_attribute(&AttrKind::ReadOnly), true);

    attrs = AttributeSet::get_by_builder(&mut c, &b_align_readonly);
    attrs = attrs.remove_attributes(&mut c, &b_align);
    assert_eq!(attrs.get_alignment(), None);
    assert_eq!(attrs.has_attribute(&AttrKind::ReadOnly), true);

    let mut al = AttributeList::new_default();
    al = al.add_param_attributes(&mut c, 0, &b_align_readonly);
    al = al.add_ret_attributes(&mut c, &b_stackalign_optnone);
    assert_eq!(al.has_ret_attrs(), true);
    assert_eq!(al.has_ret_attr(&AttrKind::StackAlignment), true);
    assert_eq!(al.has_ret_attr(&AttrKind::OptimizeNone), true);
    assert_eq!(al.get_ret_stack_alignment().unwrap(), MaybeAlign::new(32));
    assert_eq!(al.has_param_attrs(0), true);
    assert_eq!(al.has_param_attr(0, &AttrKind::Alignment), true);
    assert_eq!(al.has_param_attr(0, &AttrKind::ReadOnly), true);
    assert_eq!(al.get_param_alignment(0).unwrap(), MaybeAlign::new(8));

    al = al.remove_param_attribute_by_kind(&mut c, 0, &AttrKind::Alignment);
    assert_eq!(al.has_param_attr(0, &AttrKind::Alignment), false);
    assert_eq!(al.has_param_attr(0, &AttrKind::ReadOnly), true);
    assert_eq!(al.has_ret_attr(&AttrKind::StackAlignment), true);
    assert_eq!(al.has_ret_attr(&AttrKind::OptimizeNone), true);
    assert_eq!(al.get_ret_stack_alignment().unwrap(), MaybeAlign::new(32));

    al = al.remove_ret_attirbute_by_kind(&mut c, &AttrKind::StackAlignment);
    assert_eq!(al.has_param_attr(0, &AttrKind::Alignment), false);
    assert_eq!(al.has_param_attr(0, &AttrKind::ReadOnly), true);
    assert_eq!(al.has_ret_attr(&AttrKind::StackAlignment), false);
    assert_eq!(al.has_ret_attr(&AttrKind::OptimizeNone), true);

    let mut al2 = AttributeList::new_default();
    al2 = al2.add_param_attributes(&mut c, 0, &b_align_readonly);
    al2 = al2.add_ret_attributes(&mut c, &b_stackalign_optnone);

    al2 = al2.remove_param_attributes(&mut c, 0, &b_align);
    assert_eq!(al2.has_param_attr(0, &AttrKind::Alignment), false);
    assert_eq!(al2.has_param_attr(0, &AttrKind::ReadOnly), true);
    assert_eq!(al2.has_ret_attr(&AttrKind::StackAlignment), true);
    assert_eq!(al2.has_ret_attr(&AttrKind::OptimizeNone), true);
    assert_eq!(al2.get_ret_stack_alignment().unwrap(), MaybeAlign::new(32));

    al2 = al2.remove_ret_attributes(&mut c, &b_stack_align);
    assert_eq!(al2.has_param_attr(0, &AttrKind::Alignment), false);
    assert_eq!(al2.has_param_attr(0, &AttrKind::ReadOnly), true);
    assert_eq!(al2.has_ret_attr(&AttrKind::StackAlignment), false);
    assert_eq!(al2.has_ret_attr(&AttrKind::OptimizeNone), true);
  }

  #[test]
  fn test_add_matching_align_attr() {
    let mut c = BlitzContext::new();
    let mut al = AttributeList::new_default();
    let align_0 = Attribute::get_by_alignment(&mut c, Align::new(8));
    let align_1 = Attribute::get_by_alignment(&mut c, Align::new(32));
    al = al.add_param_attribute(&mut c, vec![0], &align_0);
    al = al.add_param_attribute(&mut c, vec![1], &align_1);
    assert_eq!(al.get_param_alignment(0).unwrap(), MaybeAlign::new(8));
    assert_eq!(al.get_param_alignment(1).unwrap(), MaybeAlign::new(32));

    let mut b = AttrBuilder::new(&c);
    b.add_attribute_by_kind(&AttrKind::NonNull);
    b.add_alignment_attr(&MaybeAlign::new(8));
    al = al.add_param_attributes(&mut c, 0, &b);
    assert_eq!(al.get_param_alignment(0).unwrap(), MaybeAlign::new(8));
    assert_eq!(al.get_param_alignment(1).unwrap(), MaybeAlign::new(32));
    assert_eq!(al.has_param_attr(0, &AttrKind::NonNull), true);
  }
}