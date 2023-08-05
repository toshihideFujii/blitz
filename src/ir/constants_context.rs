#![allow(dead_code)]

// This file defines various helper methods and classes used by
// BlitzContextImpl for creating and managing constants.

use super::{constant::Constant, type_::Type};

struct CastConstantExpr {}

struct BinaryConstantExpr {}

struct SelectConstantExpr {}

struct ExtractElementConstantExpr {}

struct InsertElementConstantExpr {}

struct ShuffleVectorConstantExpr {}

struct GetElementPtrConstantExpr {}

struct CompareConstantExpr {}

struct MapInfo {}
impl MapInfo {
  pub fn new() {}
  pub fn get_empty_key() {}
  pub fn get_tombstone_key() {}
  pub fn get_hash_value() {}
  pub fn is_equal() {}
}

pub struct ConstantUniqueMap {}
impl ConstantUniqueMap {
  pub fn new() {}
  pub fn get_or_create() {}
  pub fn remove() {}
  pub fn replace_operands_in_place() {}
  pub fn dump() {}
}

pub struct ConstantExprKeyType {
  opcode: u32,
  subclass_optional_data: u32,
  subclass_data: u32,
  ops: Vec<Box<dyn Constant>>,
  shuffle_mask: Option<Vec<i32>>,
  explicit_type: Option<Box<dyn Type>>
}

impl ConstantExprKeyType {
  pub fn new(opcode: u32, ops: Vec<Box<dyn Constant>>,
    subclass_data: u32, subclass_optional_data: u32) -> Self
  {
    ConstantExprKeyType {
      opcode: opcode, subclass_optional_data: subclass_optional_data,
      subclass_data: subclass_data, ops: ops, shuffle_mask: None, explicit_type: None
    }
  }

  pub fn get_shuffle_mask_if_valid() {}
  pub fn get_source_element_type_if_valid() {}
  pub fn get_hash() {}
  pub fn delete_constant() {}
}