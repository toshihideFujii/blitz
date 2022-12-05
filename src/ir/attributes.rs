#![allow(dead_code)]

/*
This file contains the simple types necessary to
represent the attributes associated with functions
and their calls.
*/

enum AllocFnKind {
  Unknown,
  Alloc,
  Realloc,
  Free,
  Uninitialized,
  Zeroed,
  Aligned
}

enum AttrKind {
  None,
  EndAttrKinds,
  EmptyKey,
  TombstoneKey
}

struct Attribute {}

impl Attribute {
  pub fn new() {}

  pub fn is_enum_attribute() {}

  pub fn is_int_attribute() {}

  pub fn is_string_attribute() {}

  pub fn is_type_attribute() {}

  pub fn is_valid() {}

  pub fn has_attribute() {}

  pub fn get_kind_as_enum() {}

  pub fn get_value_as_int() {}

  pub fn get_value_as_bool() {}

  pub fn get_kind_as_string() {}

  pub fn get_value_as_string() {}

  pub fn get_value_as_type() {}

  pub fn get_alignment() {}

  pub fn get_stack_alignment() {}

  pub fn get_dereferenceable_bytes() {}

  pub fn get_dereferenceable_or_null_bytes() {}
}

pub fn is_enum_attr_kind() {}

pub fn is_int_attr_kind() {}

pub fn is_type_attr_kind() {}

pub fn can_use_as_fn_attr() {}

pub fn can_use_as_param_attr() {}

pub fn can_use_as_ret_attr() {}

pub fn get() {}

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