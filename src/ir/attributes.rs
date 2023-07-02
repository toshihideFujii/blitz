#![allow(dead_code)]

// This file contains the simple types necessary to represent the
// attributes associated with functions and their calls.

use crate::{
  adt::{
    string_ref::StringRef, folding_set::FoldingSetNodeID,
    floating_point_mode::FPClassTest
  },
  support::{
    alignment::{MaybeAlign, Align}, code_gen::UWTableKind,
    mod_ref::MemoryEffects
  },
  ir::value::MAXIMUM_ALIGNMENT
};

use super::{
  blits_context::BlitzContext, attribute_impl::{AttributeImpl, AttributeSetNode},
  type_::{Type}
};

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
  pimpl: Option<AttributeImpl>
}

impl Attribute {
  pub fn new() -> Self {
    Attribute { pimpl: None }
  }

  pub fn is_enum_attr_kind(_kind: &AttrKind) -> bool { false }
  pub fn is_int_attr_kind(_kind: &AttrKind) -> bool { false }
  pub fn is_type_attr_kind(_kind: &AttrKind) -> bool { false }

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
  pub fn get_with_int(_context: &BlitzContext, kind: AttrKind, val: u64) -> Self {
    let is_int_attr = Attribute::is_int_attr_kind(&kind);
    debug_assert!(is_int_attr || Attribute::is_enum_attr_kind(&kind),
      "Not an enum or int attribute.");
    
    let mut id = FoldingSetNodeID::new();
    id.add_integer_u32(kind as u32);
    if is_int_attr {
      id.add_integer_u64(val);
    } else {
      debug_assert!(val == 0, "Value must be zero for enum attributes.");
    }

    // TODO

    Attribute { pimpl: None }
  }

  pub fn get_with_string(_context: &BlitzContext, _kind: AttrKind, _val: StringRef) -> Self {
    Attribute { pimpl: None }
  }

  pub fn get_with_type(_context: &BlitzContext, _kind: AttrKind, _type: Box<dyn Type>) -> Self {
    Attribute { pimpl: None }
  }

  pub fn get_with_alignment(context: &BlitzContext, align: Align) -> Self {
    debug_assert!(align.value() <= MAXIMUM_ALIGNMENT, "Alignment too large.");
    Attribute::get_with_int(context, AttrKind::Alignment, align.value())
  }

  pub fn get_with_stack_alignment(context: &BlitzContext, align: Align) -> Self {
    debug_assert!(align.value() <= 0x100, "Alignment too large.");
    Attribute::get_with_int(context, AttrKind::StackAlignment, align.value())
  }

  pub fn get_with_dereferenceable_bytes(context: &BlitzContext, bytes: u64) -> Self {
    debug_assert!(bytes != 0, "Bytes must be non-zero.");
    Attribute::get_with_int(context, AttrKind::Dereferenceable, bytes)
  }

  pub fn get_with_dereferenceable_or_null_bytes(context: &BlitzContext, bytes: u64) -> Self {
    debug_assert!(bytes != 0, "Bytes must be non-zero.");
    Attribute::get_with_int(context, AttrKind::DereferenceableOrNull, bytes)
  }

  pub fn get_with_alloc_size_args(context: &BlitzContext, elem_size_arg: u64,
    num_elems_arg: u64) -> Self
  {
    debug_assert!(!(elem_size_arg == 0 && num_elems_arg == 0),
      "Invalid allocsize arguments -- given allocsize(0, 0)");
    Attribute::get_with_int(context, AttrKind::AllocSize,
      Attribute::pack_alloc_size_args(elem_size_arg, num_elems_arg))
  }

  pub fn gete_with_v_scale_range_args(context: &BlitzContext, min_value: u64,
    max_value: u64) -> Self
  {
    Attribute::get_with_int(context, AttrKind::VScaleRange,
      Attribute::pack_vscale_range_args(min_value, max_value))
  }

  pub fn get_with_by_val_type(context: &BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_with_type(context, AttrKind::ByVal, t)
  }

  pub fn get_with_struct_ret_type(context: &BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_with_type(context, AttrKind::StructRet, t)
  }

  pub fn get_with_by_ref_type(context: &BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_with_type(context, AttrKind::ByRef, t)
  }

  pub fn get_with_preallocated_type(context: &BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_with_type(context, AttrKind::Preallocated, t)
  }

  pub fn get_with_in_alloca_type(context: &BlitzContext, t: Box<dyn Type>) -> Self {
    Attribute::get_with_type(context, AttrKind::InAlloca, t)
  }

  pub fn get_with_uw_table_kind(context: &BlitzContext, kind: UWTableKind) -> Self {
    Attribute::get_with_int(context, AttrKind::UWTable, kind as u64)
  }

  pub fn get_with_memory_effects(context: &BlitzContext, me: MemoryEffects) -> Self {
    Attribute::get_with_int(context, AttrKind::Memory, me.to_int_value() as u64)
  }

  pub fn get_with_no_fp_class(context: &BlitzContext, class_mask: FPClassTest) -> Self {
    Attribute::get_with_int(context, AttrKind::NoFPClass, class_mask as u64)
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
  pub fn get_value_as_type(&self) -> &Option<Box<dyn Type>> {
    if self.pimpl.is_some() {
      self.pimpl.as_ref().unwrap().get_value_as_type()
    } else {
      return &None;
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
  set_node: Option<AttributeSetNode>
}

impl AttributeSet {
  pub fn new(asn: Option<AttributeSetNode>) -> Self {
    AttributeSet { set_node: asn }
  }

  pub fn get() {}
  pub fn add_attribute() {}
  pub fn add_attributes() {}
  pub fn remove_attribute() {}
  pub fn remove_attributes() {}
  pub fn get_num_attributes() {}
  pub fn has_attributes() {}

  // Return true if the attribute exists in this set.s
  pub fn has_attribute(&self, kind: AttrKind) -> bool {
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
// its parameters.
// You access the the attributes for each of them via an index into the
// AttributeList object.
#[derive(Debug)]
pub struct AttributeList {}

impl AttributeList {
  pub fn new() {}
  pub fn get() {}
  pub fn add_attribute_at_index() {}
  pub fn add_attributes_at_index() {}
  pub fn add_fn_attribute() {}
  pub fn add_fn_attributes() {}
  pub fn add_ret_attribute() {}
  pub fn add_ret_attributes() {}

  // Add an argument attribute to the list.
  pub fn add_param_attribute(&self, _c: &BlitzContext, _arg_no: u32, _kind: AttrKind) {}

  pub fn add_param_attributes() {}
  pub fn remove_attribute_at_index() {}
  pub fn remove_attributes_at_index() {}
  pub fn remove_fn_attribute() {}
  pub fn remove_fn_attributes() {}
  pub fn remove_ret_attirbute() {}
  pub fn remove_ret_attributes() {}

  // Remove the specified attribute at the specified arg index from this
  // attribute list.
  pub fn remove_param_attribute(&self, _c: &BlitzContext, _arg_no: u32, _kind: AttrKind) {}

  pub fn remove_param_attributes() {}
  pub fn replace_attribute_type_at_index() {}
  pub fn add_dereferenceable_ret_attr() {}
  pub fn add_dereferenceable_param_attr() {}
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

  pub fn has_param_attrs() {}
  pub fn has_ret_attr() {}
  pub fn has_ret_attrs() {}
  pub fn has_fn_attr() {}
  pub fn has_fn_attrs() {}
  pub fn has_attr_somewhere() {}

  // Return the attribute object that exists at the given idex.
  pub fn get_attribute_at_index(&self, _index: u32, _kind: AttrKind) -> Attribute {
    Attribute::new()
  }

  // Return the attribute object that exists at the arg index.
  pub fn get_param_attr(&self, arg_no:u32, kind: AttrKind) -> Attribute {
    self.get_attribute_at_index(arg_no, kind)
  }

  pub fn get_fn_attr() {}
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

  pub fn get_param_preallocated_type() {}

  pub fn get_param_in_alloca_type(&self, index: u32) -> Option<Box<dyn Type>> {
    self.get_attributes(index + AttrIndex::FirstArgIndex as u32).get_in_alloca_type()
  }

  pub fn get_param_element_type() {}
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
}