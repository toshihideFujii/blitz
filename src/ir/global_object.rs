#![allow(dead_code)]

// This represents an independent object.
// That is a function, or a global variable, but not an alias.

//use std::any::Any;
use std::fmt::Debug;
//use crate::{
  //adt::twine::Twine,
  //ir::{blits_context::BlitzContext, type_::Type}
//};

use super::{comdat::Comdat, value::{Value, ValueType},
  global_value::{GlobalValue, /*LinkageTypes, GlobalValueBase*/},
  //constant::Constant, use_::Use
};

enum VCallVisibility {
  Public = 0,
  LinkageUnit = 1,
  TranslationUnit = 2
}

pub enum BitKind {
  LastAlignmentBit = 5,
  HasSectionHashEntryBit = 6,
  GlobalObjectBits = 7
}

pub const ALIGNMENT_BITS: u32 = BitKind::LastAlignmentBit as u32 + 1;
pub const ALIGNMENT_MASK: u32 = (1 << ALIGNMENT_BITS) - 1;
pub const GLOBAL_OBJECT_MASK: u32 = (1 << BitKind::GlobalObjectBits as u32) - 1;

pub trait GlobalObject : Debug + GlobalValue {
  fn get_alignment(&self) {}
  fn get_align(&self) {}
  fn set_alignment(&self) {}
  fn get_global_object_sub_class_data(&self) -> u32 { 0 }
  fn set_global_object_sub_class_data(&mut self, _v: u32) {}
  fn has_section(&self) {}
  fn get_section(&self) {}
  fn set_section(&self) {}

  fn has_combat(&self) -> bool {
    false //self.get_combat().is_some()
  }

  fn get_combat(&self) -> Option<Comdat> {
    None // TODO
  }

  fn set_combat(&self) {}
  fn copy_metadata(&self) {}
  fn add_type_metadata(&self) {}
  fn set_vcall_visibility_metadata(&self) {}
  fn get_vcall_visibility(&self) {}
  fn can_increase_alignment(&self) -> bool { false }

  fn class_of(&self, v: Box<dyn Value>) -> bool {
    v.get_value_id() == ValueType::FunctionVal ||
    v.get_value_id() == ValueType::GlobalVariableVal ||
    v.get_value_id() == ValueType::GlobalIFuncVal
  }

  fn set_global_object_flag(&self) {}
}