#![allow(dead_code)]

// This file contains the simple types necessary to represent the
// attributes associated with functions and their calls.

use crate::{
  adt::string_ref::StringRef,
  support::alignment::MaybeAlign
};

use super::{
  blits_context::BlitzContext,
  attribute_impl::AttributeImpl,
  type_::Type
};

enum AllocFnKind {
  Unknown,
  Alloc,
  Realloc,
  Free,
  Uninitialized,
  Zeroed,
  Aligned
}

#[derive(Debug, PartialEq)]
pub enum AttrKind {
  None,
  Alignment,
  StackAlignment,
  Dereferenceable,
  DereferenceableOrNull,
  EndAttrKinds,
  EmptyKey,
  TombstoneKey
}

// Functions, function parameters, and return types can have attributes
// to indicate how they should be treated by optimizations and code generation.
// This class represents one of those attributes.
pub struct Attribute {
  pimpl: Option<AttributeImpl>
}

impl Attribute {
  pub fn new() {}

  pub fn is_enum_attr_kind(&self) -> bool { false }
  pub fn is_int_attr_kind() {}
  pub fn is_type_attr_kind() {}
  pub fn can_use_as_fn_attr() {}
  pub fn can_use_as_param_attr() {}
  pub fn can_use_as_ret_attr() {}

  // Return a uniquified Attribute object.
  pub fn get(_context: BlitzContext, _kind: AttrKind, _val: u64) {}
  pub fn get_with_alignment() {}
  pub fn get_with_stack_alignment() {}
  pub fn get_with_dereferenceable_bytes() {}
  pub fn get_with_dereferenceable_or_null_bytes() {}
  pub fn get_with_alloc_size_args() {}
  pub fn gete_with_v_scale_range_args() {}
  pub fn get_with_by_val_type() {}
  pub fn get_with_struct_ret_type() {}
  pub fn get_with_by_ref_type() {}
  pub fn get_with_preallocated_type() {}
  pub fn get_with_in_alloca_type() {}
  pub fn get_with_uw_table_kind() {}
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

  pub fn get_alloc_size_args() {}
  pub fn get_vscale_range_min() {}
  pub fn get_vscale_range_max() {}
  pub fn get_uw_table_kind() {}
  pub fn get_alloc_kind() {}
  pub fn get_memory_effects() {}
  pub fn get_no_fp_class() {}
  pub fn get_as_string() {}
  pub fn has_parent_context() {}
  pub fn profile() {}
  pub fn get_raw_pointer() {}
  pub fn from_raw_pointer() {}
}

// This class holds the attributes for a function, its return value,
// and its parameters.
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
  pub fn add_param_attribute() {}
  pub fn add_param_attributes() {}
  pub fn remove_attribute_at_index() {}
  pub fn remove_attributes_at_index() {}
  pub fn remove_fn_attribute() {}
  pub fn remove_fn_attributes() {}
  pub fn remove_ret_attirbute() {}
  pub fn remove_ret_attributes() {}
  pub fn remove_param_attribute() {}
  pub fn remove_param_attributes() {}
  pub fn replace_attribute_type_at_index() {}
  pub fn add_dereferenceable_ret_attr() {}
  pub fn add_dereferenceable_param_attr() {}
  pub fn add_dereferenceable_or_null_param_attr() {}
  pub fn add_alloc_size_param_attr() {}
  pub fn get_attributes() {}
  pub fn get_param_attrs() {}
  pub fn get_ret_attrs() {}
  pub fn get_fn_attrs() {}
  pub fn has_attribute_at_index() {}
  pub fn has_param_attr() {}
  pub fn has_param_attrs() {}
  pub fn has_ret_attr() {}
  pub fn has_ret_attrs() {}
  pub fn has_fn_attr() {}
  pub fn has_fn_attrs() {}
  pub fn has_attr_somewhere() {}
  pub fn get_attribute_at_index() {}
  pub fn get_param_attr() {}
  pub fn get_fn_attr() {}
  pub fn get_ret_alignment() {}
  pub fn get_param_alignment() {}
  pub fn get_param_stack_alignment() {}
  pub fn get_param_by_val_type() {}
  pub fn get_param_struct_ret_type() {}
  pub fn get_param_by_ref_type() {}
  pub fn get_param_preallocated_type() {}
  pub fn get_param_in_alloca_type() {}
  pub fn get_param_element_type() {}
  pub fn get_fn_stack_alignment() {}
  pub fn get_ret_stack_alignment() {}
  pub fn get_ret_dereferenceable_bytes() {}
  pub fn get_param_dereferenceable_bytes() {}
  pub fn get_ret_dereferenceable_or_null_bytes() {}
}