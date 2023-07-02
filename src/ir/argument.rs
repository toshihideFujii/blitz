#![allow(dead_code)]

// This file declares the Argument class.

use crate::{support::alignment::MaybeAlign, adt::floating_point_mode::FPClassTest};
use super::{
  blits_context::BlitzContext, function::{Function},
  value::{Value, ValueType}, type_::Type, attributes::{AttrKind, Attribute, AttributeSet}
};

struct Argument {
  v_type: Box<dyn Type>,
  parent: Option<Function>,
  arg_no: u32
}

impl Argument {
  pub fn new(t: Box<dyn Type>, parent: Option<Function>, arg_no: u32) -> Self {
    Argument { v_type: t, parent: parent, arg_no: arg_no }
  }

  pub fn set_parent(&mut self, parent: Function) {
    self.parent = Some(parent);
  }

  pub fn get_parent(&self) -> &Option<Function> {
    &self.parent
  }

  // Return the index of this formal argument in its containing function.
  pub fn get_arg_no(&self) -> u32 {
    debug_assert!(self.parent.is_some(), "Can't get number of unparented arg.");
    self.arg_no
  }

  // Return true if this argument has the nonnull attribute.
  // Also returns true if at least one byte is known to be dereferenceable
  // and the pointer is in addrspace(0).
  pub fn has_non_null_attr(&self, allow_undef_or_poison: bool) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    let parent = self.get_parent().as_ref().unwrap();
    if parent.has_param_attribute(self.get_arg_no(), AttrKind::NonNull) &&
      (allow_undef_or_poison ||
       parent.has_param_attribute(self.get_arg_no(), AttrKind::NoUndef)) {
      return true;
    } else if self.get_dereferenceable_bytes() > 0 &&
      !self.null_pointer_is_defined(self.get_parent(),
        self.get_type().get_pointer_address_space()) {
      return true;
    }
    false
  }

  // If the argument has the dereferenceable attribute, return the number
  // of bytes known to be dereferenceable.
  pub fn get_dereferenceable_bytes(&self) -> u64 {
    debug_assert!(self.get_type().is_pointer_type(),
      "Only pointers have dereferenceable bytes.");
    self.get_parent().as_ref().unwrap().get_param_dereferenceable_bytes(self.get_arg_no())
  }

  // If this argument has the dereferenceable_or_null attribute, return the
  // number of bytes known to be dereferenceable.
  pub fn get_dereferenceable_or_null_bytes(&self) -> u64 {
    debug_assert!(self.get_type().is_pointer_type(),
      "Only pointers have dereferenceable bytes.");
    self.get_parent().as_ref().unwrap().get_param_dereferenceable_or_null_bytes(self.get_arg_no())
  }

  // If this argument has nofpclass attribute, return the mask representing
  // disallowed floating-point values.
  pub fn get_no_fp_class(&self) -> FPClassTest {
    self.get_parent().as_ref().unwrap().get_param_no_fp_class(self.get_arg_no())
  }

  // Return true if this argument has the byval attribute.
  pub fn has_by_val_attr(&self) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    self.has_attribute(AttrKind::ByVal)
  }

  // Return true if this argument has the byref attribute.
  pub fn has_by_ref_attr(&self) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    self.has_attribute(AttrKind::ByRef)
  }

  // Return true if this argument has the byval, inalloca, or
  // preallocated attribute.
  // These arguments represent arguments being passed by value,
  // with an ssociated copy between the caller and callee.
  pub fn has_pass_pointee_by_value_copy_attr(&self) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    let attrs = self.get_parent().as_ref().unwrap().get_attributes();
    let arg_no = self.get_arg_no();
    attrs.has_param_attr(arg_no, AttrKind::ByVal) ||
    attrs.has_param_attr(arg_no, AttrKind::InAlloca) ||
    attrs.has_param_attr(arg_no, AttrKind::Preallocated)
  }

  pub fn get_pass_pointee_by_value_copy_size() {}

  // Return true if this argument has the byval, sret, inalloca,
  // preallocated, or byref attribute.
  // These attributes represent arguments being passed by value
  // (which may or may not involve a stack copy).
  pub fn has_pointee_in_memory_value_attr(&self) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    let attrs = self.get_parent().as_ref().unwrap().get_attributes();
    let arg_no = self.get_arg_no();
    attrs.has_param_attr(arg_no, AttrKind::ByVal) ||
    attrs.has_param_attr(arg_no, AttrKind::StructRet) ||
    attrs.has_param_attr(arg_no, AttrKind::InAlloca) ||
    attrs.has_param_attr(arg_no, AttrKind::Preallocated) ||
    attrs.has_param_attr(arg_no, AttrKind::ByRef)
  }

  // If has_pointee_in_memory_value_attr returns true, the in-memory ABI type is returned.
  pub fn get_pointee_in_memory_value_type(&self) -> Option<Box<dyn Type>> {
    let param_attrs = self.get_parent().as_ref().unwrap()
      .get_attributes().get_param_attrs(self.get_arg_no());
    Argument::get_memory_param_alloc_type(&param_attrs)
  }

  // If this is a byval or inalloca argument, return its alignment.
  pub fn get_param_align(&self) -> MaybeAlign {
    debug_assert!(self.get_type().is_pointer_type(), "Only pointers have alignments.");
    self.get_parent().as_ref().unwrap().get_param_align(self.get_arg_no())
  }

  pub fn get_param_stack_align(&self) -> MaybeAlign {
    self.get_parent().as_ref().unwrap().get_param_stack_align(self.get_arg_no())
  }

  // If true is a byval argument, return its type.
  pub fn get_param_by_val_type(&self) -> Option<Box<dyn Type>> {
    debug_assert!(self.get_type().is_pointer_type(), "Only pointers have byval types.");
    self.get_parent().as_ref().unwrap().get_param_by_val_type(self.get_arg_no())
  }

  // If true is an sret argument, return its type.
  pub fn get_param_struct_ret_type(&self) -> Option<Box<dyn Type>> {
    debug_assert!(self.get_type().is_pointer_type(), "Only pointers have sret types.");
    self.get_parent().as_ref().unwrap().get_param_struct_ret_type(self.get_arg_no())
  }

  // If true is a byref argument, return its type.
  pub fn get_param_by_ref_type(&self) -> Option<Box<dyn Type>> {
    debug_assert!(self.get_type().is_pointer_type(), "Only pointers have byref types.");
    self.get_parent().as_ref().unwrap().get_param_by_ref_type(self.get_arg_no())
  }

  // If this is an inalloca argument, return its type.
  pub fn get_param_in_alloca_type(&self) -> Option<Box<dyn Type>> {
    debug_assert!(self.get_type().is_pointer_type(), "Only pointers have inalloca types.");
    self.get_parent().as_ref().unwrap().get_param_in_alloca_type(self.get_arg_no())
  }

  // Return true if this argument has the nest attribute.
  pub fn has_nest_attr(&self) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    self.has_attribute(AttrKind::Nest)
  }

  // Return true if this argument has the noalias attribute.
  pub fn has_no_alias_attr(&self) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    self.has_attribute(AttrKind::NoAlias)
  }

  // Return true if this argument has the nocapture attribute.
  pub fn has_no_capture_attr(&self) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    self.has_attribute(AttrKind::NoCapture)
  }

  // Return true if this argument has the nofree attribute.
  pub fn has_no_free_attr(&self) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    self.has_attribute(AttrKind::NoFree)
  }

  // Return true if this argument has the sret attribute.
  pub fn has_struct_ret_attr(&self) -> bool {
    self.has_attribute(AttrKind::StructRet)
  }

  // Return true if this argument has the inreg attribute.
  pub fn has_in_reg_attr(&self) -> bool {
    self.has_attribute(AttrKind::InReg)
  }

  // Return true if this argument has the returned attribute.
  pub fn has_returned_attr(&self) -> bool {
    self.has_attribute(AttrKind::Returned)
  }

  pub fn only_reads_memory() {}

  // Return true if this argument has the inalloca attribute.
  pub fn has_in_alloca_attr(&self) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    self.has_attribute(AttrKind::InAlloca)
  }

  // Return true if this argument has the preallocated attribute.
  pub fn has_preallocated_attr(&self) -> bool {
    if !self.get_type().is_pointer_type() {
      return false;
    }
    self.has_attribute(AttrKind::Preallocated)
  }

  // Return true if this argument has the zext attribute.
  pub fn has_zext_attr(&self) -> bool {
    self.has_attribute(AttrKind::ZExt)
  }

  // Return true if this argument has the sext attribute.
  pub fn has_sext_attr(&self) -> bool {
    self.has_attribute(AttrKind::SExt)
  }

  pub fn add_attrs() {}
  pub fn add_attr() {}

  pub fn add_attr_by_kind(&self, kind: AttrKind) {
    self.get_parent().as_ref().unwrap().add_param_attr_by_kind(self.get_arg_no(), kind)
  }

  pub fn remove_attrs() {}

  // Remove attributes from an argument.
  pub fn remove_attr(&self, kind: AttrKind) {
    self.get_parent().as_ref().unwrap().remove_param_attr(self.get_arg_no(), kind)
  }

  // Check if an argument has a given attribute.
  pub fn has_attribute(&self, kind: AttrKind) -> bool {
    self.get_parent().as_ref().unwrap().has_param_attribute(self.get_arg_no(), kind)
  }

  pub fn get_attribute(&self, kind: AttrKind) -> Attribute {
    self.get_parent().as_ref().unwrap().get_param_attribute(self.get_arg_no(), kind)
  }

  // Method for support type inquiry through isa, cast, and dyn_cast.
  pub fn class_of(v: Box<dyn Value>) -> bool {
    v.get_value_id() == ValueType::ArgumentVal
  }

  fn null_pointer_is_defined(&self, _f: &Option<Function>, _v: u32) -> bool {
    false
  }

  fn get_memory_param_alloc_type(param_attrs: &AttributeSet) -> Option<Box<dyn Type>> {
    if param_attrs.get_by_val_type().is_some() {
      return param_attrs.get_by_val_type();
    }
    if param_attrs.get_by_ref_type().is_some() {
      return param_attrs.get_by_ref_type();
    }
    if param_attrs.get_preallocated_type().is_some() {
      return param_attrs.get_preallocated_type();
    }
    if param_attrs.get_in_alloca_type().is_some() {
      return param_attrs.get_in_alloca_type();
    }
    if param_attrs.get_struct_ret_type().is_some() {
      return param_attrs.get_struct_ret_type();
    }
    None
  }
}

impl Value for Argument {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_context(&self) -> &BlitzContext {
    self.v_type.get_context()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::ArgumentVal
  }
}