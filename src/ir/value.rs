#![allow(dead_code)]

// This file declares the value class.

use crate::adt::string_ref::StringRef;
use super::{type_::Type, use_::Use,
  blits_context::BlitzContext,
  metadata::MDNode
};

// The maximum alignment for instructions.
// This is the greatest alignment value supported by load, store, and
// alloca instructions, and global values.
pub const MAX_ALIGNMENT_EXPONENT: u64 = 32;
pub const MAXIMUM_ALIGNMENT: u64 = 1 << MAX_ALIGNMENT_EXPONENT;

// Concrete subclass of Value.
// An enumeration for keeping track of the concrete subclass of Value
// that is actually instantiated.
// Values of this enumeration are kept in the subclass field.
// They are used for concrete type identification.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
  FunctionVal,
  GlobalAliasVal,
  GlobalIFuncVal,
  GlobalVariableVal,
  BlockAddressVal,
  ConstantExprVal,
  DS0LocalEquivalentVal,
  ConstantArrayVal,
  ConstantStructVal,
  ConstantVectorVal,
  UndefValueVal,
  PoisonValueVal,
  ConstantAggregateZeroVal,
  ConstantDataArrayVal,
  ConstantDataVectorVal,
  ConstantIntVal,
  ConstantFPVal,
  ConstantTargetNoneVal,
  ConstantPointerNullVal,
  ConstantTokenNoneVal,
  ArgumentVal,
  BasicBlockVal,
  InstructionVal
}

// Blitz value representation
// This is a very important Blitz class.
// It is the base class of all values computed by a program that may
// be used as operands to other values.
// Value is the super class of other important classes such as Instruction
// and Function.
// All values have a type. Type is not a subclass of Value.
// Some values can have a name and they belong to some Module.
// Setting the name on the value automatically updates the mmodule's symbol
// table.
pub trait Value {
  fn dump(&self) {}
  fn print(&self) {}
  fn print_as_operand(&self) {}

  // All values are typed, get the type of this value.
  fn get_type(&self) -> &dyn Type;
  
  // All values hold a context through their type.
  fn get_context(&self) -> &BlitzContext;

  fn has_name(&self) -> bool { false }
  fn get_value_name(&self) {}
  fn set_value_name(&self) {}

  // Return a constant reference to the value's name.
  fn get_name(&self) -> StringRef { StringRef::new() }

  fn set_name(&self) {}
  fn take_name(&self) {}
  fn replace_all_uses_with(&self) {}
  fn replace_non_metadata_uses_with(&self) {}
  fn replace_uses_with_if(&self) {}
  fn replace_uses_outside_block(&self) {}
  fn assert_module_is_materialized_impl(&self) {}
  fn assert_module_is_materialized(&self) {}

  fn use_empty(&self) -> bool { false }
  fn materialized_use_empty(&self) {}

  // Return true if there is exactly one use of this value.
  // This is specialized because it is a common request and does not
  // require traversing the whole use list.
  fn has_one_use(&self) -> bool { false }

  // Return true if this Value has exactly n uses.
  fn has_n_uses(&self, _n: usize) -> bool { false }

  // Return true if this value has n uses or more.
  fn has_n_uses_or_more(&self, _n: usize) -> bool { false }

  // Return true if there is exactly one user of this value.
  // Note that this is not the same as has_one_use().
  // If a value has one use, then there certainly is a single user.
  // But if value has several uses, it is possible that all uses are
  // in a single user, or not.
  fn has_one_user(&self) -> bool { false }

  fn get_single_undroppable_use(&self) {}
  fn get_unique_undroppable_user(&self) {}
  fn has_n_undroppable_uses(&self) {}
  fn has_n_undroppable_uses_or_more(&self) {}
  fn drop_droppable_uses(&self) {}
  fn drop_droppable_uses_in(&self) {}
  fn drop_droppable_use(&self) {}
  fn is_used_in_basic_block(&self) {}

  // This method computes the number of uses of this Value.
  fn get_num_uses(&self) -> usize { 0 }

  // This method should only be used by the Use class.
  fn add_use(&mut self, _u: Use) {}

  // Return an ID for the concrete type of this object.
  // This is used to implement the classof checks.
  // This should not be used for any other purpose, as the values may
  // change as Blitz evolves.
  fn get_value_id(&self) -> ValueType;

  fn get_raw_subclass_optional_data(&self) {}
  fn clear_subclass_optional_data(&self) {}
  fn has_same_subclass_optional_data(&self) {}
  fn has_value_handle(&self) {}
  fn is_used_by_metadata(&self) {}
  fn get_metadata_by_id(&self, _kind_id: u32) -> Option<MDNode> { None }
  fn get_all_metadata(&self) {}

  // Return true if this value has any metadata attached to it.
  fn has_metadata(&self) -> bool { false }

  // Set a particular kind of metadata attachment.
  fn set_metadata(&mut self, _kind_id: u32, _node: Option<MDNode>) {}
  fn add_metadata(&self) {}
  fn erase_metadata(&self) {}
  fn clear_metadata(&self) {}
  fn strip_pointer_casts(&self) {}
  fn can_be_freed(&self) {}
  fn do_phi_translation(&self) {}
  fn is_transitive_used_by_metadata_only(&self) {}

  fn get_subclass_data_from_value(&self) -> u32 { 0 }
  fn set_value_subclass_data(&mut self, _val: u32) {}
}