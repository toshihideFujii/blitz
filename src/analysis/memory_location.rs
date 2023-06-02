#![allow(dead_code)]

// This file provides utility analysis objects describing memory locations.
// These are used by the Alias Analysis infrastructure and more specialized
// memory analysis layers.

struct LocationSize {}
impl LocationSize {
  pub fn new() {}
  pub fn precise() {}
  pub fn upper_bound() {}
  pub fn after_pointer() {}
  pub fn before_or_after_pointer() {}
  pub fn map_tombstone() {}
  pub fn map_empty() {}
  pub fn union_with() {}
  pub fn has_value() {}
  pub fn get_value() {}
  pub fn is_precise() {}
  pub fn is_zero() {}
  pub fn maybe_before_pointer() {}
  pub fn print() {}
  pub fn to_raw() {}
}

struct MemoryLocation {}
impl MemoryLocation {
  pub fn new() {}
  pub fn print() {}
  pub fn get() {}
  pub fn get_or_none() {}
  pub fn get_for_source() {}
  pub fn get_for_dest() {}
  pub fn get_for_argument() {}
  pub fn get_after() {}
  pub fn get_before_or_after() {}
  pub fn get_size_or_unknown() {}
  pub fn get_with_new_ptr() {}
  pub fn get_with_new_size() {}
  pub fn get_without_aa_tags() {}
}