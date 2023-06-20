#![allow(dead_code)]

// This file contains the declaration of the MachineOperand class.

enum MachineOperandType {
  Register,
  Immediate,
  CImmediate,
  FPImmediate,
  MachineBasicBlock,
  FrameIndex,
  ConstantPoolIndex,
  TargetIndex,
  JumpTableIndex,
  ExternalSymbol,
  GlobalAddress,
  BlockAddress,
  RegisterMask,
  RegisterLiveOut,
  Metadata,
  MCSymbol,
  CFIIndex,
  IntrinsicID,
  Predicate,
  ShuffleMask,
  DbgInstrRef
}

struct MachineOperand {}
impl MachineOperand {
  pub fn new() {}
  pub fn get_type() {}
  pub fn get_target_flags() {}
  pub fn set_target_flags() {}
  pub fn add_target_flag() {}
  pub fn get_parent() {}
  pub fn clear_parent() {}
  pub fn get_operand_no() {}
  pub fn print_sub_reg_idx() {}
  pub fn print_target_flags() {}
  pub fn print_symbol() {}
  pub fn print_stack_object_reference() {}
  pub fn print_operand_offset() {}
  pub fn print_ir_slot_number() {}
  pub fn print() {}
  pub fn dump() {}
}