#![allow(dead_code)]

// This file defines the DataLayout-independent constant foldin
// interface. When possible, the DataLayout-aware constant folding
// interface in analysis/constnt_folding.rs should be preferred.

use crate::ir::instruction::Instruction;

use super::constant::Constant;

pub fn constant_fold_cast_instruction() {}
pub fn constant_fold_select_instruction() {}
pub fn constant_fold_extract_element_instruction() {}
pub fn constant_fold_insert_element_instruction() {}
pub fn constant_fold_shuffle_vector_instruction() {}
pub fn constant_fold_extract_value_instruction() {}
pub fn constant_fold_insert_value_instruction() {}
pub fn constant_fold_unary_instruction() {}

pub fn constant_fold_binary_instruction(opcode: u32,
  _c1: &Box<dyn Constant>, _c2: &Box<dyn Constant>) -> Option<Box<dyn Constant>>
{
    debug_assert!(Instruction::is_binary_op_static(opcode),
      "Non-binary instruction detected.");

    //Some(*c1)
    None
}

pub fn constant_fold_compare_instruction() {}
pub fn constant_fold_get_element() {}