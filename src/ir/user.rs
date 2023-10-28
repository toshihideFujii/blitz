#![allow(dead_code)]

// This class defines the interface that one who uses a Value
// must implement.
// Each instance of the Value class keeps track of what User's
// have handles to it.

//#[derive(Debug, Clone)]
//pub struct User {}

use super::value::Value;

pub trait User: Value {
  fn alloc_hungoff_uses(&self) {}
  fn grow_hungoff_uses(&self) {}
  fn get_operand_list(&self) {}

  fn get_operand(&self, _i: usize) -> Option<&Box<dyn Value>> { None }
  fn set_operand(&mut self, _i: usize, _v: Box<dyn Value>) {}
  fn get_operand_use(&self) {}
  fn get_num_operands(&self) -> usize { 0 }

  fn get_descriptor(&self) {}
  fn set_global_variable_num_operands(&self) {}
  fn set_num_hungoff_use_operands(&self) {}
  fn is_droppable(&self) {}
  fn drop_all_references(&self) {}
  fn replace_uses_of_with(&self) {}
}