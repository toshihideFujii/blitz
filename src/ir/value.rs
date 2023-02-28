#![allow(dead_code)]

/*
This file declares the value class.
*/

//use super::type_::Type;

#[derive(Debug, Clone)]
pub struct Value {
  //vtype_: Type
  is_used_by_md: u32,
  has_name: u32,
  has_meta_data: u32,
  has_hung_off_uses: u32,
  has_descriptor: u32
}

impl Value {
  pub fn new() {}
  pub fn dump() {}
  pub fn print() {}
  pub fn print_as_operand() {}
  pub fn get_type() {}
  pub fn get_context() {}

  pub fn has_name(&self) -> bool {
    if self.has_name != 0 {
      return true;
    } else {
      return false;
    }
  }
  
  pub fn get_value_name() {}
  pub fn set_value_name() {}

  fn get_name() {}

  fn set_name() {}

  fn take_name() {}

  fn replace_all_uses_with() {}

  fn replace_non_metadata_uses_with() {}

  fn replace_uses_with_if() {}

  fn replace_uses_outside_block() {}

  fn assert_module_is_materialized_impl() {}

  fn assert_module_is_materialized() {}

  fn use_empty() {}

  fn materialized_use_empty() {}

  fn has_one_use() {}

  fn has_n_uses() {}

  fn has_n_uses_or_more() {}

  fn has_one_user() {}

  fn get_single_undroppable_use() {}

  fn get_unique_undroppable_user() {}

  fn has_n_undroppable_uses() {}

  fn has_n_undroppable_uses_or_more() {}

  fn drop_droppable_uses() {}

  fn drop_droppable_uses_in() {}

  fn drop_droppable_use() {}

  fn is_used_in_basic_block() {}

  fn get_num_uses() {}

  fn add_use() {}

  fn get_value_id() {}

  fn get_raw_subclass_optional_data() {}

  fn clear_subclass_optional_data() {}

  fn has_same_subclass_optional_data() {}

  fn has_value_handle() {}

  fn is_used_by_metadata() {}

  fn is_transitive_used_by_metadata_only() {}
}