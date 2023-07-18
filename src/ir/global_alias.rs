#![allow(dead_code)]

// This file contains the declaration of the GlobalAlias class,
// which represents a single function or variable alias in the IR.

use super::{value::{Value, ValueType}, global_value::{GlobalValue, LinkageTypes}};

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalAlias {
  gv: GlobalValue
}

impl GlobalAlias {
  pub fn new() {}
  pub fn create() {}
  pub fn copy_attributes_from() {}
  pub fn remove_from_parent() {}
  pub fn erase_from_parent() {}
  pub fn set_aliasee() {}
  pub fn get_aliasee() {}
  pub fn get_aliasee_object() {}

  pub fn is_valid_linkage(l: LinkageTypes) -> bool {
    GlobalValue::is_external_linkage(&l) || GlobalValue::is_local_linkage(&l) ||
    GlobalValue::is_weak_linkage(&l) || GlobalValue::is_link_once_linkage(&l) ||
    GlobalValue::is_available_externally_linkage(&l)
  }

  pub fn class_of(v: Box<dyn Value>) -> bool {
    v.get_value_id() == ValueType::GlobalAliasVal
  }
}