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

/* 
#[derive(Debug)]
pub struct GlobalObjectBase {
  gv: GlobalValueBase
  //obj_comdata: Box<Option<Comdat>>
}

impl GlobalObjectBase {
  pub fn new(t: Box<dyn Type>, v_id: ValueType, ops: Option<Use>, num_ops: usize,
    linkage: &LinkageTypes, name: Twine, addr_space: usize) -> Self
  {
    let gv = GlobalValueBase::new(t, v_id, ops,
      num_ops,linkage, name, addr_space);
    GlobalObjectBase { gv: gv }
  }
}

impl Value for GlobalObjectBase {
  fn get_type(&self) -> &dyn Type {
    //self.gv.get_type()
    GlobalValueBase::get_type(&self.gv)
  }

  fn get_context(&self) -> &BlitzContext {
    self.gv.get_context()
  }

  fn get_context_mut(&mut self) -> &mut BlitzContext {
    self.gv.get_context_mut()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::GlobalVariableVal
  }

  fn get_subclass_data_from_value(&self) -> u32 {
    0 //self.sub_class_data
  }

  fn set_value_subclass_data(&mut self, val: u32) {
    //self.sub_class_data = val;
  }
}

impl Constant for GlobalObjectBase {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl GlobalValue for GlobalObjectBase {}

impl GlobalObject for GlobalObjectBase {
  fn get_alignment(&self) {}
  fn get_align(&self) {}
  fn set_alignment(&self) {}
  fn get_global_object_sub_class_data(&self) {}
  fn set_global_object_sub_class_data(&self) {}
  fn has_section(&self) {}
  fn get_section(&self) {}
  fn set_section(&self) {}

  fn has_combat(&self) -> bool {
    self.get_combat().is_some()
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
*/
