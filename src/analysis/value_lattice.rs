#![allow(dead_code)]

struct ValueLatticeElement {}
impl ValueLatticeElement {
  pub fn new() {}
  pub fn get() {}
  pub fn get_not() {}
  pub fn get_range() {}
  pub fn get_overdefined() {}
  pub fn is_undef() {}
  pub fn is_unknown() {}
  pub fn is_unknown_or_undef() {}
  pub fn is_constant() {}
  pub fn is_not_constant() {}
  pub fn is_constant_range_including_undef() {}
  pub fn is_constant_range() {}
  pub fn is_overdefined() {}
  pub fn get_constant() {}
  pub fn get_not_constant() {}
  pub fn get_constant_range() {}
  pub fn as_constant_integer() {}
  pub fn mark_overdefined() {}
  pub fn mark_undef() {}
  pub fn mark_constant() {}
  pub fn mark_not_constant() {}
  pub fn mark_constant_range() {}
  pub fn merge_in() {}
  pub fn ge_compare() {}
  pub fn get_num_range_extensions() {}
}