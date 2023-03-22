#![allow(dead_code)]

// This file contains the declaration of the Instruction class,
// which is the base class for all of the instructions.

struct Instruction {}
impl Instruction {
  pub fn user_back() {}
  pub fn get_parent() {}
  pub fn get_module() {}
  pub fn get_function() {}
  pub fn remove_from_parent() {}
  pub fn erase_from_parent() {}
  pub fn insert_before() {}
  pub fn insert_after() {}
  pub fn insert_into() {}
  pub fn move_before() {}
  pub fn move_after() {}
  pub fn comes_before() {}
  pub fn get_insertion_poiint_after_def() {}
  pub fn get_op_code() {}
  pub fn get_opcode_name() {}
  pub fn is_terminator() {}
  pub fn is_unary_op() {}
  pub fn is_binary_op() {}
  pub fn is_int_div_rem() {}
  pub fn is_shift() {}
  pub fn is_cast() {}
  pub fn is_funclet_pad() {}
  pub fn is_exceptional_terminator() {}
}