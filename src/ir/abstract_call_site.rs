#![allow(dead_code)]

// This file defines the AbstractCallSite class, which is a wrapper
// that allows treating direct, indirect and callback calls the same.

// The encoding of a callback with regards to the underlying instruction.
struct CallbackInfo {
  parameter_encoding: Vec<i32>
}

struct AbstractCallSite {}
impl AbstractCallSite {
  pub fn new() {}
  pub fn get_callback_uses() {}
  pub fn get_instruction() {}
  pub fn is_direct_call() {}
  pub fn is_indirect_call() {}
  pub fn is_callback_call() {}
  pub fn is_callee() {}
  pub fn get_num_arg_operands() {}
  pub fn get_call_arg_operand_no() {}
  pub fn get_call_arg_operand() {}
  pub fn get_call_arg_operand_no_for_callee() {}
  pub fn get_callee_use_for_callback() {}
  pub fn get_called_operand() {}
  pub fn get_called_function() {}
  pub fn for_each_callback_call_site() {}
  pub fn for_each_callback_function() {}
}