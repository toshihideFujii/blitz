#![allow(dead_code)]

// This file defines various classes for working with Instructions
// and ConstantExprs.

use std::any::Any;
use super::user::User;
use super::instruction::{Instruction, OpCode};
use super::value::{Value, ValueType};
use super::type_::{Type, PointerType};

// This is a utility class that provides an abstraction for the common
// functionality between instructions and ConstantExprs.
pub trait Operator /*: User*/ {
  // Return the opcode for this Instruction or ConstantExpr.
  fn get_op_code(&self) {}

  fn has_poison_generating_flags(&self) -> bool { false }
  fn has_poison_generating_flags_or_metadata(&self) -> bool { false }
}

pub enum OverflowBinOpWrap {
  AnyWrap,
  NoUnsignedWrap,
  NoSignedWrap
}

// Utility class for integer operators which may exhibit overflow - Add,
// Sub, Mul, and Shl. It does not include SDiv, despite that operator having
// the potential overflow.
#[derive(Debug)]
pub struct OverflowingBinaryOperator {
  no_unsigned_wrap: bool,
  no_signed_wrap: bool
}

impl OverflowingBinaryOperator {
  pub fn new() -> Self {
    OverflowingBinaryOperator { no_unsigned_wrap: false, no_signed_wrap: false }
  }

  // Test whether this operation is known to never undergo unsigned overflow,
  // aka the nuw property.
  pub fn has_no_unsigned_wrap(&self) -> bool {
    self.no_unsigned_wrap
  }

  // Test whether this operation is known to never undergo signed overflow,
  // aka the nsw property.
  pub fn has_no_signed_wrap(&self) -> bool {
    self.no_signed_wrap
  }

  fn set_has_no_unsigned_wrap(&mut self, b: bool) {
    self.no_unsigned_wrap = b;
  }

  fn set_has_no_signed_wrap(&mut self, b: bool) {
    self.no_signed_wrap = b;
  }

  pub fn class_of(i: &dyn Instruction) -> bool {
    i.get_op_code() == OpCode::Add || i.get_op_code() == OpCode::Sub ||
    i.get_op_code() == OpCode::Mul || i.get_op_code() == OpCode::Shl
  }
}

impl Operator for OverflowingBinaryOperator {}

pub enum PossiblyExactOp {
  IsExact
}

// A udiv or sdiv instruction, which can be marked as "exact", indicating
// that no bits are destroyed.
#[derive(Debug)]
pub struct PossiblyExactOperator {
  is_exact: bool
}

impl PossiblyExactOperator {
  pub fn new() -> Self {
    PossiblyExactOperator { is_exact: false }
  }

  // Test whether this division is known to be exact, with zero reminder.
  pub fn is_exact(&self) -> bool {
    self.is_exact
  }

  pub fn is_possibly_exact_code(opcode: OpCode) -> bool {
    opcode == OpCode::SDiv || opcode == OpCode::UDiv ||
    opcode == OpCode::AShr || opcode == OpCode::LShr
  }

  fn set_is_exact(&mut self, b: bool) {
    self.is_exact = b;
  }

  pub fn class_of(i: &dyn Instruction) -> bool {
    PossiblyExactOperator::is_possibly_exact_code(i.get_op_code())
  }
}

impl Operator for PossiblyExactOperator {}

// Utility class for floating point operations which can have information
// about relaxed accuracy requirements attached to them.
#[derive(Debug)]
pub struct FPMathOperator {
  allow_reassoc: bool,
  no_nans: bool,
  no_infs: bool,
  no_signed_zeros: bool,
  allow_reciprocal: bool,
  allow_contract: bool,
  approx_func: bool
}

impl FPMathOperator {
  pub fn new() {}

  // Test if this operation allows all non-strict floating-point transforms.
  pub fn is_fast(&self) -> bool {
    self.allow_reassoc && self.no_nans && self.no_infs &&
    self.no_signed_zeros && self.allow_reciprocal && self.allow_contract &&
    self.approx_func
  }

  // Test if this operation may be simplified with reassociative transforms.
  pub fn has_allow_reassoc(&self) -> bool {
    self.allow_reassoc
  }

  // Test if this operation's arguments and results are assumed not-NaN.
  pub fn has_no_nans(&self) -> bool {
    self.no_nans
  }

  // Test if this operation's arguments and results are assumed not-infinite.
  pub fn has_no_infs(&self) -> bool {
    self.no_infs
  }

  // Test if this operation can ignore the sign of zero.
  pub fn has_no_signed_zeros(&self) -> bool {
    self.no_signed_zeros
  }

  // Test if this operation can use reciprocal multiply instead of division.
  pub fn has_allow_reciprocal(&self) -> bool {
    self.allow_reciprocal
  }

  // Test if this operation can be floating-point contracted (FMA).
  pub fn has_allow_contract(&self) -> bool {
    self.allow_contract
  }

  // Test if this operation allows approximation of math library function or
  // intrinsics.
  pub fn has_approx_func(&self) -> bool {
    self.approx_func
  }

  fn set_has_allow_reassoc(&mut self, b: bool) {
    self.allow_reassoc = b;
  }

  fn set_has_no_nans(&mut self, b: bool) {
    self.no_nans = b;
  }

  fn set_has_no_infs(&mut self, b: bool) {
    self.no_infs = b;
  }

  fn set_has_no_signed_zeros(&mut self, b: bool) {
    self.no_signed_zeros = b;
  }

  fn set_has_allow_reciprocal(&mut self, b: bool) {
    self.allow_reciprocal = b;
  }

  fn set_has_allow_contract(&mut self, b: bool) {
    self.allow_contract = b;
  }

  fn set_has_approx_func(&mut self, b: bool) {
    self.approx_func = b;
  }
}

impl Operator for FPMathOperator {}

struct AddOperator {}

struct SubOperator {}

struct MulOperator {}

struct ShlOperator {}

struct SDivOperator {}

struct UDivOperator {}

struct AShrOperator {}

struct LShrOperator {}

struct ZExtOperator {}

#[derive(Debug)]
pub struct GEPOperator {
  is_in_bounds: bool
}

impl GEPOperator {
  pub fn new() {}

  // Test whether this is an inbounds GEP, as defined by LangRef.html.
  pub fn is_in_bounds(&self) -> bool {
    self.is_in_bounds
  }

  pub fn get_in_range_index(&self) {}

  pub fn get_pointer_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> usize { 0 }

  // Method to return the pointer operand as a PointerType.
  pub fn get_pointer_operand_type(&self) -> &dyn Type {
    self.get_pointer_operand().unwrap().get_type()
  }

  pub fn get_source_element_type(&self) {}

  pub fn get_result_element_type(&self) {}

  // Method to return the address space of the pointer operand.
  pub fn get_pointer_address_space(&self) -> usize {
    self.get_pointer_operand_type().get_pointer_address_space()
  }

  pub fn get_num_indices(&self) -> usize {
    self.get_num_operands() - 1
  }

  pub fn has_indices(&self) -> bool {
    self.get_num_operands() > 1
  }

  pub fn has_all_zero_indices(&self) {}

  pub fn has_all_constant_indices(&self) {}

  pub fn count_non_constant_indices(&self) {}

  pub fn get_max_preserved_alignment(&self) {}

  pub fn accumulate_constant_offset(&self) {}

  pub fn collect_offset(&self) {}
}

impl Operator for GEPOperator {}

impl User for GEPOperator {}
impl Value for GEPOperator {
  fn get_value_id(&self) -> ValueType {
    unimplemented!("Not implemented.");
  }

  fn get_type(&self) -> &dyn Type {
    unimplemented!("Not implemented.");
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

#[derive(Debug)]
pub struct PtrToIntOperator {}

impl PtrToIntOperator {
  pub fn new() -> Self {
    PtrToIntOperator {  }
  }

  pub fn get_pointer_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> usize { 0 }

  // Method to return the pointer operand as a PointerType.
  pub fn get_pointer_operand_type(&self) -> &dyn Type {
    self.get_pointer_operand().unwrap().get_type()
  }

  // Method to return the address space of the pointer operand.
  pub fn get_pointer_address_space(&self) -> usize {
    let ptr_t =
      self.get_pointer_operand_type().as_any().downcast_ref::<PointerType>();
    ptr_t.unwrap().get_address_space()
  }
}

impl Operator for PtrToIntOperator {}
impl User for PtrToIntOperator {}
impl Value for PtrToIntOperator {
  fn get_value_id(&self) -> ValueType {
    unimplemented!("Not implemented.");
  }

  fn get_type(&self) -> &dyn Type {
    unimplemented!("Not implemented.");
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

#[derive(Debug)]
pub struct BitCastOperator {}

impl BitCastOperator {
  pub fn new() -> Self {
    BitCastOperator {  }
  }

  pub fn get_src_type(&self)  -> &dyn Type {
    self.get_operand(0).unwrap().get_type()
  }

  pub fn get_dest_type(&self) -> &dyn Type {
    self.get_type()
  }
}

impl Operator for BitCastOperator {}
impl User for BitCastOperator {}
impl Value for BitCastOperator {
  fn get_value_id(&self) -> ValueType {
    unimplemented!("Not implemented.");
  }

  fn get_type(&self) -> &dyn Type {
    unimplemented!("Not implemented.");
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

#[derive(Debug)]
pub struct AddrSpaceCastOperator {}

impl AddrSpaceCastOperator {
  pub fn new() -> Self {
    AddrSpaceCastOperator {  }
  }

  pub fn get_pointer_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_src_address_space(&self) -> usize {
    self.get_pointer_operand().unwrap().get_type().get_pointer_address_space()
  }

  pub fn get_dest_address_space(&self) -> usize {
    self.get_type().get_pointer_address_space()
  }
}

impl Operator for AddrSpaceCastOperator {}
impl User for AddrSpaceCastOperator {}
impl Value for AddrSpaceCastOperator {
  fn get_value_id(&self) -> ValueType {
    unimplemented!("Not implemented.");
  }

  fn get_type(&self) -> &dyn Type {
    unimplemented!("Not implemented.");
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}