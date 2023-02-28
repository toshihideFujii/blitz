#![allow(dead_code)]

/*
This class defines the interface that one who uses
a Value must implement.
*/

#[derive(Debug, Clone)]
pub struct User {}

impl User {
  pub fn alloc_hungoff_uses() {}

  pub fn grow_hungoff_uses() {}

  pub fn get_operand_list() {}

  pub fn get_operand() {}

  pub fn set_operand() {}

  pub fn get_operand_use() {}

  pub fn get_num_operands() {}

  pub fn get_descriptor() {}

  pub fn set_global_variable_num_operands() {}

  pub fn set_num_hungoff_use_operands() {}

  pub fn is_droppable() {}

  pub fn drop_all_references() {}

  pub fn replace_uses_of_with() {}
}