#![allow(dead_code)]

// This file contains the simple types necessary to represent the
// attributes associated with functions and their calls.

use crate::{
  adt::{
    string_ref::StringRef, folding_set::FoldingSetNodeID,
    floating_point_mode::FPClassTest,
  },
  support::{
    alignment::{MaybeAlign, Align}, code_gen::UWTableKind,
    mod_ref::MemoryEffects
  },
  ir::value
};

use super::{
  blits_context::BlitzContext,
  attribute_impl::{AttributeImpl, AttributeSetNode, AttrEntryKind, AttributeListImpl},
  type_::Type
};

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    let attr_impl =
      c.get_impl().as_ref().unwrap().attrs_set.get(&id);
    if attr_impl.is_none() {
      let mut kind_id = AttrEntryKind::EnumAttrEntry;
      if is_int_attr { kind_id = AttrEntryKind::IntAttrEntry }
      let v = AttributeImpl::new(kind_id, kind.clone(), val);
      c.get_mut_impl().attrs_set.insert(id, v.clone());
      return Attribute::new(Some(v));
    } else {
      return Attribute::new(Some(attr_impl.unwrap().clone()));
    }
  }

  pub fn get_by_string(_c: &BlitzContext, _kind: AttrKind, _val: StringRef) -> Self {
    Attribute { pimpl: None }
  }

  // TODO
  pub fn get_by_type(c: &BlitzContext, kind: AttrKind, _t: Box<dyn Type>) -> Self {
    debug_assert!(Attribute::is_type_attr_kind(&kind), "Not a type attribute.");
    let mut id = FoldingSetNodeID::new();
    id.add_integer_u32(kind.clone() as u32);
    // id.add(t)

    let attr_impl =
      c.get_impl().as_ref().unwrap().attrs_set.get(&id);
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

  pub fn get_by_by_val_type(context: &BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_by_type(context, AttrKind::ByVal, t)
  }

  pub fn get_by_struct_ret_type(context: &BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_by_type(context, AttrKind::StructRet, t)
  }

  pub fn get_by_by_ref_type(context: &BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_by_type(context, AttrKind::ByRef, t)
  }

  pub fn get_by_preallocated_type(context: &BlitzContext,
    t: Box<dyn Type>) -> Self
  {
    Attribute::get_by_type(context, AttrKind::Preallocated, t)
  }

  pub fn get_by_in_alloca_type(context: &BlitzContext,
    t: Box<dyn Type>) -> Self
  {
    Attribute::get_by_type(context, AttrKind::InAlloca, t)
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
  pub fn has_attribute(&self, kind: AttrKind) -> bool {
    if self.pimpl.is_some() {
      return self.pimpl.as_ref().unwrap().has_attribute(kind);
    } else {
      return kind == AttrKind::None;
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
    debug_assert!(self.has_attribute(AttrKind::Alignment),
      "Trying to get alignment from non-alignment attribute.");
    MaybeAlign::new(self.pimpl.as_ref().unwrap().get_value_as_int())
  }

  // Returns the stack alignment field of an attribute as a byte
  // alignment value.
  pub fn get_stack_alignment(&self) -> MaybeAlign {
    debug_assert!(self.has_attribute(AttrKind::StackAlignment),
      "Trying to get alignment from non-alignment attribute.");
    MaybeAlign::new(self.pimpl.as_ref().unwrap().get_value_as_int())
  }

  // Returns the number of dereferenceable bytes from the 
  // dereferenceable attribute.
  pub fn get_dereferenceable_bytes(&self) -> u64 {
    debug_assert!(self.has_attribute(AttrKind::Dereferenceable),
      "Trying to get dereferenceable bytes from non-dereferenceable attribute.");
    self.pimpl.as_ref().unwrap().get_value_as_int()
  }

  // Returns the number of dereferenceable_or_null bytes from the
  // dereferenceable_or_null attribute.
  pub fn get_dereferenceable_or_null_bytes(&self) -> u64 {
    debug_assert!(self.has_attribute(AttrKind::DereferenceableOrNull),
      "Trying to get dereferenceable bytes from non-dereferenceable attribute.");
    self.pimpl.as_ref().unwrap().get_value_as_int()
  }

  // Returns the argument numbers for the allocsize attribute.
  pub fn get_alloc_size_args(&self) -> (u64, u64) {
    debug_assert!(self.has_attribute(AttrKind::AllocSize),
      "Trying to get allocsize args from non-allocsize attribute.");
    Attribute::unpack_alloc_size_args(self.pimpl.as_ref().unwrap().get_value_as_int())
  }

  // Returns the minimum value for the vscale_range attribute.
  pub fn get_vscale_range_min(&self) -> u64 {
    debug_assert!(self.has_attribute(AttrKind::VScaleRange),
      "Trying to get vscale args from non-vscale attribute.");
    Attribute::unpack_vscale_range_args(self.pimpl.as_ref().unwrap().get_value_as_int()).0
  }

  // Returns the maximum value for the vscale_range attribute.
  pub fn get_vscale_range_max(&self) -> u64 {
    debug_assert!(self.has_attribute(AttrKind::VScaleRange),
      "Trying to get vscale args from non-vscale attribute.");
    Attribute::unpack_vscale_range_args(self.pimpl.as_ref().unwrap().get_value_as_int()).1
  }

  // Returns the unwind table kind.
  pub fn get_uw_table_kind(&self) -> UWTableKind {
    debug_assert!(self.has_attribute(AttrKind::UWTable),
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
    debug_assert!(self.has_attribute(AttrKind::AllocKind),
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
    debug_assert!(self.has_attribute(AttrKind::Memory),
      "Can only call get_memory_effects() on memory attribute.");
    MemoryEffects::create_from_int_value(
      self.pimpl.as_ref().unwrap().get_value_as_int() as u32)
  }

  pub fn get_no_fp_class(&self) -> FPClassTest {
    debug_assert!(self.has_attribute(AttrKind::NoFPClass),
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
  pub fn has_parent_context() {}
  pub fn profile() {}

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeSet {
  pub set_node: Option<AttributeSetNode>
}

impl AttributeSet {
  pub fn new(asn: Option<AttributeSetNode>) -> Self {
    AttributeSet { set_node: asn }
  }

  pub fn get_by_builder(c: &BlitzContext, b: &AttrBuilder) -> Self {
    AttributeSet::new(AttributeSetNode::get_by_builder(c, b))
  }

  pub fn get_by_attrs(c: &BlitzContext, attrs: &Vec<Attribute>) -> Self {
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
  pub fn add_attributes(&self, c: &BlitzContext, attrs: AttributeSet) -> AttributeSet {
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

  pub fn remove_attribute() {}
  pub fn remove_attributes() {}
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
  pub fn get_attribute(&self, kind: AttrKind) -> Option<Attribute> {
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
  pub fn has_parent_context() {}
}


pub enum AttrIndex {
  ReturnIndex = 0,
  FunctionIndex = !0,
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

  pub fn get_impl(c: &mut BlitzContext, attr_sets: Vec<AttributeSet>) -> AttributeList {
    debug_assert!(attr_sets.is_empty(), "Pointless AttributeListImpl.");
    let mut id = FoldingSetNodeID::new();
    AttributeListImpl::profile(&mut id, &attr_sets);

    let attr_list_impl =
      c.get_impl().as_ref().unwrap().attrs_lists.get(&id);

    if attr_list_impl.is_none() {
      // TODO
      return AttributeList::new(Some(attr_list_impl.unwrap().clone()))
    } else {
      return AttributeList::new(Some(attr_list_impl.unwrap().clone()))
    }
  }

  pub fn add_attribute_at_index(&self, _c: &BlitzContext,
    _i: u32, _attr: Attribute) -> Self
  {
    AttributeList::new(None)
  }

  pub fn add_attributes_at_index() {}

  pub fn add_fn_attribute(&self, _c: &BlitzContext, _kind: AttrKind) -> Self {
    AttributeList::new(None)
  }

  pub fn add_fn_attributes() {}

  pub fn add_ret_attribute(&self, _c: &BlitzContext, _attr: Attribute) -> Self {
    AttributeList::new(None)
  }

  pub fn add_ret_attributes() {}

  // Add an argument attribute to the list.
  pub fn add_param_attribute(&self, _c: &BlitzContext, _arg_no: u32, _kind: AttrKind) -> Self {
    AttributeList::new(None)
  }

  pub fn add_param_attributes() {}

  pub fn remove_attribute_at_index(&self, _c: &BlitzContext, _i: u32, _kind: AttrKind) -> Self {
    AttributeList::new(None)
  }

  pub fn remove_attributes_at_index() {}

  pub fn remove_fn_attribute(&self, _c: &BlitzContext, _kind: AttrKind) -> Self {
    AttributeList::new(None)
  }

  pub fn remove_fn_attributes() {}

  pub fn remove_ret_attirbute(&self, _c: &BlitzContext, _kind: AttrKind) -> Self {
    AttributeList::new(None)
  }

  pub fn remove_ret_attributes() {}

  // Remove the specified attribute at the specified arg index from this
  // attribute list.
  pub fn remove_param_attribute(&self, _c: &BlitzContext, _arg_no: u32, _kind: AttrKind) -> Self {
    AttributeList::new(None)
  }

  pub fn remove_param_attributes() {}
  pub fn replace_attribute_type_at_index() {}

  pub fn add_dereferenceable_ret_attr(&self, _c: &BlitzContext, _bytes: u64) -> Self {
    AttributeList::new(None)
  }

  pub fn add_dereferenceable_param_attr(&self, _c: &BlitzContext, _i: u32, _bytes: u64) -> Self {
    AttributeList::new(None)
  }

  pub fn add_dereferenceable_or_null_param_attr() {}
  pub fn add_alloc_size_param_attr() {}

  pub fn get_attributes(&self, _index: u32) -> AttributeSet {
    AttributeSet::new(None)
  }

  pub fn get_param_attrs(&self, _arg_no: u32) -> AttributeSet {
    AttributeSet::new(None)
  }

  pub fn get_ret_attrs() {}
  pub fn get_fn_attrs() {}

  // Return true if the attribute exists at the given index.
  pub fn has_attribute_at_index(&self, _index: u32, _kind: AttrKind) -> bool {
    false
  }

  // Return true if the attribute exists for the given argument.
  pub fn has_param_attr(&self, arg_no: u32, kind: AttrKind) -> bool {
    self.has_attribute_at_index(arg_no + AttrIndex::FirstArgIndex as u32, kind)
  }

  pub fn has_param_attrs(&self) -> bool { false }

  // Return true if the attribute exists for the return value.
  pub fn has_ret_attr(&self, _kind: AttrKind) -> bool { false }

  pub fn has_ret_attrs(&self) -> bool { false }

  // Return true if the attribute exists for the function.
  pub fn has_fn_attr(&self, _kind: AttrKind) -> bool { false }

  pub fn has_fn_attrs(&self) -> bool { false }
  pub fn has_attr_somewhere(&self, _kind: AttrKind) -> bool { false }

  // Return the attribute object that exists at the given idex.
  pub fn get_attribute_at_index(&self, _index: u32, _kind: AttrKind) -> Attribute {
    Attribute::new(None)
  }

  // Return the attribute object that exists at the arg index.
  pub fn get_param_attr(&self, arg_no:u32, kind: AttrKind) -> Attribute {
    self.get_attribute_at_index(arg_no, kind)
  }

  pub fn get_fn_attr(&self, _kind: AttrKind) -> Attribute {
    Attribute::new(None)
  }

  pub fn get_ret_alignment() {}

  pub fn get_param_alignment(&self, _arg_no: u32) -> MaybeAlign {
    MaybeAlign::new(0)
  }

  pub fn get_param_stack_alignment(&self, _arg_no:u32) -> MaybeAlign {
    MaybeAlign::new(0)
  }

  pub fn get_param_by_val_type(&self, index: u32) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as u32).get_by_val_type()
  }

  pub fn get_param_struct_ret_type(&self, index: u32) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as u32).get_struct_ret_type()
  }

  pub fn get_param_by_ref_type(&self, index: u32) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as u32).get_by_ref_type()
  }

  pub fn get_param_preallocated_type(&self, index: u32) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as u32).get_by_ref_type() // TODO
  }

  pub fn get_param_in_alloca_type(&self, index: u32) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as u32).get_in_alloca_type()
  }

  pub fn get_param_element_type(&self, index: u32) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as u32).get_in_alloca_type() // TODO
  }

  pub fn get_fn_stack_alignment() {}
  pub fn get_ret_stack_alignment() {}
  pub fn get_ret_dereferenceable_bytes() {}

  pub fn get_param_dereferenceable_bytes(&self, _index: u32) -> u64 {
    0
  }

  pub fn get_param_dereferenceable_or_null_bytes(&self, _index: u32) -> u64 {
    0
  }

  pub fn get_param_no_fp_class(&self, _index: u32) -> FPClassTest {
    FPClassTest::None
  }

  pub fn get_uw_table_kind(&self) -> UWTableKind {
    UWTableKind::None
  }

  pub fn get_alloc_kind() {}

  // Returns memory effects of the function.
  pub fn get_memory_effects(&self) -> MemoryEffects {
    MemoryEffects::new(0)
  }

  pub fn get_as_string() {}
  pub fn has_parent_context() {}
}

// This class stores enough information to efficiently remove some
// attributes from an existing AttrBuilder, AttributeSet ot Attributelist.
pub struct AttributeMask {}

impl AttributeMask {
  pub fn new() {}
  pub fn add_attribute() {}
  pub fn contains() {}
}

// This class is used in conjunction with the Attribute::get method to
// create an Attribute object. The object itself is uniquified.
// The builder's value, however, is not. So this can be a quick way to
// test for equality, presence of attributes, etc.
pub struct AttrBuilder {
  c: BlitzContext,
  pub attrs: Vec<Attribute>
}

impl AttrBuilder {
  pub fn new(c: &BlitzContext) -> Self {
    AttrBuilder { c: c.clone(), attrs: Vec::new() }
  }

  pub fn new_from_attr_set(c: &BlitzContext, attrs: &AttributeSet) -> Self {
    AttrBuilder {
      c: c.clone(),
      attrs: attrs.set_node.as_ref().unwrap().attrs.clone()
    }
  }

  pub fn clear(&mut self) {
    self.attrs.clear()
  }

  // Add an attribute to the builder.
  pub fn add_attribute_by_kind(&mut self, kind: &AttrKind) {
    let mut c = self.c.clone();
    let target = Attribute::get_by_int(&mut c, kind.clone(), 0);
    self.add_attribute_impl(kind, target)
  }

  pub fn add_attribute_by_string() {}

  // Add the Attribute object to the builder.
  pub fn add_attribute(&mut self, attr: Attribute) {
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
      if attr.has_attribute(kind.clone()) { has_attr = true; break; }
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
      self.add_attribute(attr);
    }
  }

  pub fn remove() {}
  pub fn overlaps() {}
  pub fn contains_by_kind() {}
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
      if attr.has_attribute(kind.clone()) {
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
    self.add_attribute(Attribute::get_by_int(&mut self.c.clone(),
      kind.clone(), value))
  }

  // This turns an alignment into the form used internally in Attribute.
  // This call has no effect if align is not set.
  pub fn add_alignment_attr(&mut self, align: &MaybeAlign) {
    if align.value() == 0 { return; }
    debug_assert!(align.value() <= value::MAXIMUM_ALIGNMENT, "Alignment is too large.");
    self.add_raw_int_attr(&AttrKind::Alignment, align.value())
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

  pub fn add_alloc_size_attr() {}
  pub fn add_vscale_range_attr() {}

  pub fn add_type_attr(&mut self, _kind: &AttrKind, _t: Box<dyn Type>) {
    
  }

  pub fn add_by_val_attr() {}
  pub fn add_struct_ret_attr() {}
  pub fn add_by_ref_attr() {}
  pub fn add_preallocated_attr() {}
  pub fn add_in_alloca_attr() {}

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

  fn add_attribute_impl(&mut self, kind: &AttrKind, target: Attribute) {
    let mut index = 0;
    let mut has_attr = false;
    for attr in &self.attrs {
      if attr.has_attribute(kind.clone()) { has_attr = true; break; }
      index += 1;
    }

    if has_attr {
      self.attrs.remove(index);
      self.attrs.insert(index, target);
    } else {
      self.attrs.push(target);
    }
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
}