#![allow(dead_code)]

// This file contains the declaration of the Instruction class,
// which is the base class for all of the instructions.

enum CastOps {
  Trunc = 38,
  ZExt = 39,
  SExt = 40,
  FPToUI = 41,
  FPToSI = 42,
  UIToFP = 43,
  SIToFp = 44,
  FPTrunc = 45,
  FPExt = 46,
  PtrToInt = 47,
  IntToPtr = 48,
  BitCast = 49,
  AddrSpaceCast = 50
}

pub enum OtherOps {
  ICmp = 53,
  FCmp = 54,
  Phi = 55,
  Call = 56,
  Select = 57,
  UserOp1 = 58,
  UserOp2 = 59,
  VaArg = 60,
  ExtractElement = 61,
  InsertElement = 62,
  ShuffleVector = 63,
  ExtractValue = 64,
  InsertValue = 65,
  LandingPad = 66,
  Freeze = 67
}

#[derive(Debug, PartialEq)]
pub struct Instruction {}
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

  // Returns a member of one of the enums like Instruction::Add.
  pub fn get_op_code(&self) -> u32 { 0 }

  pub fn get_opcode_name() {}
  pub fn is_terminator() {}
  pub fn is_unary_op() {}
  pub fn is_binary_op() {}
  pub fn is_int_div_rem() {}
  pub fn is_shift() {}

  pub fn is_cast(&self) -> bool {
    Instruction::is_cast_static(self.get_op_code())
  }

  pub fn is_funclet_pad() {}
  pub fn is_exceptional_terminator() {}
  pub fn is_only_user_of_any_operand() {}
  pub fn is_bitwise_logic_op() {}

  // Determine if the opcode is one of the cast_inst instructions.
  pub fn is_cast_static(opcode: u32) -> bool {
    CastOps::Trunc as u32 <= opcode && opcode < CastOps::AddrSpaceCast as u32
  }
}