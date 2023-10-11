#![allow(dead_code)]

// This file contains the declaration of the Instruction class,
// which is the base class for all of the instructions.

use std::{any::Any, fmt::Debug};
use crate::adt::{string_ref::StringRef, twine::Twine};
use super::{
  basic_block::BasicBlock,
  blits_context::MDKind,
  debug_loc::DebugLoc, function::Function,
  value::{Value, ValueType}, 
  type_::Type, metadata::MDNode, use_::Use,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum OpCode {
  // TermOps
  Ret = 1,
  Br = 2,
  Switch = 3,
  IndirectBr = 4,
  Invoke = 5,
  Resume = 6,
  Unreachable = 7,
  CleanupRet = 8,
  CatchRet = 9,
  CatchSwitch = 10,
  CallBr = 11,
  // UnaryOps
  FNeg = 12,
  // BinaryOps
  Add = 13,
  FAdd = 14,
  Sub = 15,
  FSub = 16,
  Mul = 17,
  FMul = 18,
  UDiv = 19,
  SDiv = 20,
  FDiv = 21,
  URem = 22,
  SRem = 23,
  FRem = 24,
  Shl = 25,
  LShr = 26,
  AShr = 27,
  And = 28,
  Or = 29,
  Xor = 30,
  // MemoryOps
  Alloca,
  Load,
  Store,
  GetElementPtr,
  Fence,
  AtomicCmpXchg,
  AtomicRMW,
  // CastOps
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
  AddrSpaceCast = 50,
  // FuncletPadOps
  CleanupPad,
  CatchPad,
  // OtherOps
  ICmp = 53,
  FCmp = 54,
  Phi = 55,
  Call = 56,
  Select = 57,
  UserOp1 = 58,
  UserOp2 = 59,
  VAArg = 60,
  ExtractElement = 61,
  InsertElement = 62,
  ShuffleVector = 63,
  ExtractValue = 64,
  InsertValue = 65,
  LandingPad = 66,
  Freeze = 67,
  Unknown
}

// These instructions are used to terminate a basic block of the program.
//#[derive(Debug, PartialEq, PartialOrd)]
pub enum TermOps {
  Ret = 1,
  Br = 2,
  Switch = 3,
  IndirectBr = 4,
  Invoke = 5,
  Resume = 6,
  Unreachable = 7,
  CleanupRet = 8,
  CatchRet = 9,
  CatchSwitch = 10,
  CallBr = 11,
}

// Standard unary operators.
pub enum UnaryOps {
  FNeg = 12
}

// Standard binary operators.
pub enum BinaryOps {
  Add = 13,
  FAdd = 14,
  Sub = 15,
  FSub = 16,
  Mul = 17,
  FMul = 18,
  UDiv = 19,
  SDiv = 20,
  FDiv = 21,
  URem = 22,
  SRem = 23,
  FRem = 24,
  Shl = 25,
  LShr = 26,
  AShr = 27,
  And = 28,
  Or = 29,
  Xor = 30,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryOps {
  Alloca,
  Load,
  Store,
  GetElementPtr,
  Fence,
  AtomicCmpXchg,
  AtomicRMW
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CastOps {
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

enum FuncletPadOps {
  CleanupPad,
  CatchPad
}

pub enum OtherOps {
  ICmp = 53,
  FCmp = 54,
  Phi = 55,
  Call = 56,
  Select = 57,
  UserOp1 = 58,
  UserOp2 = 59,
  VAArg = 60,
  ExtractElement = 61,
  InsertElement = 62,
  ShuffleVector = 63,
  ExtractValue = 64,
  InsertValue = 65,
  LandingPad = 66,
  Freeze = 67
}

pub trait Instruction: Debug {
  fn get_parent(&self) -> &Option<BasicBlock> { &None }

  // Returns a member of one of the enums like Instruction::Add.
  fn get_op_code(&self) -> OpCode { OpCode::Unknown }

  fn is_terminator(&self) -> bool { false }

  fn is_exceptional_terminator(&self) -> bool { false }

  // Return true if this instruction has an AtomicOrdering of unordered
  // or higher.
  fn is_atomic(&self) -> bool { false }

  // Return the number of successors that this instruction has.
  // The instruction must be a terminator.
  fn get_num_successors(&self) -> usize { 0 }

  // Return the specified successor. This instruction must be a terminator.
  fn get_successor(&self, _index: u32) -> Option<&BasicBlock> { None }

  // Return true if the instruction is a blitz.lifetime.start or
  // blitz.lifetime.end marker.
  fn is_lifetime_start_or_end(&self) -> bool { false }

  // Return true if the instruction is a variety of EH-block.
  fn is_eh_pad(&self) -> bool { false }

  fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct InstructionBase {
  v_type: Box<dyn Type>,
  has_metadata: bool,
  parent: Option<BasicBlock>,
  dbg_loc: Option<DebugLoc>,
  order: u32
}

impl InstructionBase {
  pub fn new_ib(v_type: Box<dyn Type>, _i_type: u32, _ops: Option<Use>,
    _num_ops: u32, _insert_before: Option<Box<InstructionBase>>) -> Self
  {
    InstructionBase { v_type: v_type, has_metadata: false, parent: None,
      dbg_loc: None, order: 0 }
  }

  pub fn new_ie(v_type: Box<dyn Type>, _i_type: u32, _ops: Option<Use>,
    _num_ops: u32, _insert_at_end: Option<BasicBlock>) -> Self
  {
    InstructionBase { v_type: v_type, has_metadata: false, parent: None,
      dbg_loc: None, order: 0 }
  }

  pub fn user_back() {}

  pub fn get_parent(&self) -> &Option<BasicBlock> {
    &self.parent
  }

  pub fn set_parent(&mut self, p: BasicBlock) {
    self.parent = Some(p);
  }

  pub fn get_module(&self) /*-> &Module*/ {
    
  }

  // Return the function this instruction belongs to.
  pub fn get_function(&self) -> &Option<Function> {
    self.parent.as_ref().unwrap().get_parent()
  }
  
  pub fn remove_from_parent() {}
  pub fn erase_from_parent() {}
  pub fn insert_before() {}
  pub fn insert_after() {}
  pub fn insert_into(&self, _bb: BasicBlock) {}
  pub fn move_before() {}
  pub fn move_after() {}
  pub fn comes_before() {}
  pub fn get_insertion_poiint_after_def() {}

  // Returns a member of one of the enums like InstructionBase::Add.
  pub fn get_op_code(&self) -> OpCode { OpCode::Unknown }

  pub fn get_opcode_name() {}

  pub fn is_terminator(&self) -> bool {
    InstructionBase::is_terminator_static(self.get_op_code())
  }

  pub fn is_unary_op(&self) -> bool {
    InstructionBase::is_unary_op_static(self.get_op_code())
  }

  pub fn is_binary_op(&self) -> bool {
    InstructionBase::is_binary_op_static(&self.get_op_code())
  }

  pub fn is_int_div_rem(&self) -> bool {
    InstructionBase::is_int_div_rem_static(self.get_op_code())
  }

  pub fn is_shift(&self) -> bool {
    InstructionBase::is_shift_static(self.get_op_code())
  }

  pub fn is_cast(&self) -> bool {
    InstructionBase::is_cast_static(self.get_op_code())
  }

  pub fn is_funclet_pad(&self) -> bool {
    InstructionBase::is_funclet_pad_static(self.get_op_code())
  }

  pub fn is_exceptional_terminator() {}
  pub fn is_only_user_of_any_operand() {}

  pub fn is_logical_shift(&self) -> bool {
    self.get_op_code() == OpCode::Shl || self.get_op_code() == OpCode::LShr
  }

  pub fn is_arithmetic_shift(&self) -> bool {
    self.get_op_code() == OpCode::AShr
  }

  pub fn is_bitwise_logic_op(&self) -> bool {
    InstructionBase::is_bitwise_logic_op_static(self.get_op_code())
  }

  pub fn is_terminator_static(opcode: OpCode) -> bool {
    OpCode::Ret <= opcode && opcode <= OpCode::CallBr
  }

  pub fn is_unary_op_static(opcode: OpCode) -> bool {
    OpCode::FNeg <= opcode && opcode <= OpCode::FNeg
  }

  pub fn is_binary_op_static(opcode: &OpCode) -> bool {
    OpCode::Add <= *opcode && *opcode <= OpCode::Xor //BinaryOps::Xor as u32 + 1
  }

  pub fn is_int_div_rem_static(opcode: OpCode) -> bool {
    opcode == OpCode::UDiv || opcode == OpCode::SDiv ||
    opcode == OpCode::URem || opcode == OpCode::SRem
  }

  pub fn is_shift_static(opcode: OpCode) -> bool {
    OpCode::Shl <= opcode && opcode <= OpCode::AShr
  }

  pub fn is_bitwise_logic_op_static(opcode: OpCode) -> bool {
    opcode == OpCode::And || opcode == OpCode::Or || opcode == OpCode::Xor
  }

  // Determine if the opcode is one of the CastInst instructions.
  pub fn is_cast_static(opcode: OpCode) -> bool {
    OpCode::Trunc  <= opcode && opcode <= OpCode::AddrSpaceCast
  }

  // Determine if the opcode is one of the FuncletPadInst instructions.
  pub fn is_funclet_pad_static(opcode: OpCode) -> bool {
    OpCode::CleanupPad <= opcode && opcode <= OpCode::CatchPad
  }

  // Return true if the opcode is a terminator related to exception handling.
  pub fn is_exceptional_terminator_static(opcode: OpCode) -> bool {
    if opcode == OpCode::CatchSwitch || opcode == OpCode::CatchRet ||
       opcode == OpCode::CleanupRet || opcode == OpCode::Invoke ||
       opcode == OpCode::Resume
    {
      return true;
    }
    false
  }

  // Return true if this instruction has metadata attached to it other
  // than a debug location.
  pub fn has_metadata_other_than_debug_loc(&self) -> bool {
    self.has_metadata
  }

  pub fn has_metadata_by_kind(&self) -> bool { false }

  // Get the metadata of given kind attached to this instruction.
  pub fn get_metadata(&self, _kind_id: u32) -> Option<Box<dyn MDNode>> {
    //if !self.has_metadata() {
      return None;
    //}
    //self.get_metadata_by_id_impl(kind_id)
  }

  // Get the metadata of given kind attached to this instruction.
  pub fn get_metadata_by_kind(&self, _kind: StringRef) -> Option<Box<dyn MDNode>> {
    //if !self.has_metadata() {
      return None;
    //}
    //self.get_metadata_by_kind_impl(kind)
  }

  // Get all metadata attached to this instruction.
  // The first element of each pair returned is the kind_id, the second
  // element is the metadata value.
  pub fn get_all_metadata(&self) {
    if self.has_metadata() {
      self.get_all_metadata_impl(); // TODO
    }
  }

  pub fn get_all_metadata_other_than_debug_loc(&self) {}
  pub fn copy_metadata() {}
  pub fn swap_prof_metadata() {}
  pub fn drop_unknown_non_debug_metadata() {}
  pub fn add_annotation_metadata() {}
  pub fn get_aa_metadata() {}
  pub fn set_aa_metadata() {}
  pub fn extract_prof_total_weight() {}
  pub fn set_debug_loc() {}
  pub fn get_debug_loc() {}
  pub fn set_has_no_unsigned_wrap() {}
  pub fn set_has_no_signed_wrap() {}
  pub fn set_is_exact() {}
  pub fn has_no_unsigned_wrap() {}
  pub fn has_no_signed_wrap() {}
  pub fn has_poison_generating_flags() {}
  pub fn drop_poison_generating_flags() {}
  pub fn has_poison_generating_metadata() {}
  pub fn drop_poison_generating_metadata() {}
  pub fn has_poison_generating_flags_or_metadata() {}
  pub fn drop_poison_generating_flags_and_metadata() {}
  pub fn drop_ub_implying_attrs_and_unknown_metadata() {}
  pub fn is_exact() {}
  pub fn set_fast() {}
  pub fn set_has_allow_reassoc() {}
  pub fn set_has_no_nans() {}
  pub fn set_has_no_infs() {}
  pub fn set_has_no_signed_zeros() {}
  pub fn set_has_allow_reciprocal() {}
  pub fn set_has_allow_contract() {}
  pub fn set_has_approx_func() {}
  pub fn set_fast_math_flags() {}
  pub fn copy_fast_math_flags() {}
  pub fn is_fast() {}
  pub fn has_allow_reassoc() {}
  pub fn has_no_nans() {}
  pub fn has_no_infs() {}
  pub fn has_no_signed_zeros() {}
  pub fn has_allow_reciprocal() {}
  pub fn has_allow_contract() {}
  pub fn has_approx_func() {}
  pub fn get_fast_math_flags() {}
  pub fn copy_ir_flags() {}
  pub fn and_ir_flags() {}
  pub fn apply_merged_location() {}
  pub fn update_location_after_hoist() {}
  pub fn drop_location() {}
  pub fn merge_di_assign_id() {}
  pub fn is_associative() {}

  // Return true if the instruction is commutative.
  // Commutative operators satisfy: (x op y) == (y op x).
  pub fn is_commutative_static(opcode: &OpCode) -> bool {
    match opcode {
      OpCode::Add => return true,
      OpCode::FAdd => return true,
      OpCode::Mul => return true,
      OpCode::FMul => return true,
      OpCode::And => return true,
      OpCode::Or => return true,
      OpCode::Xor => return true,
      _ => return false,
    };
  }

  pub fn is_idempotent() {}
  pub fn is_nilpotent() {}
  pub fn may_write_to_memory() {}
  pub fn may_read_from_memory() {}
  pub fn may_read_or_write_memory() {}
  pub fn has_atomic_load() {}
  pub fn has_atomic_store() {}
  pub fn is_volatile() {}
  pub fn may_throw() {}
  pub fn is_fence_like() {}
  pub fn may_have_side_effects() {}
  pub fn is_safe_to_remove() {}
  pub fn will_return() {}
  pub fn is_launder_or_strip_invariant_group() {}
  pub fn is_debug_or_pseudo_inst() {}
  pub fn get_next_non_debug_instruction() {}
  pub fn get_prev_non_debugg_instruction() {}
  pub fn is_identical_to() {}
  pub fn is_identical_to_when_defined() {}
  pub fn is_same_operation_as() {}
  pub fn has_same_special_state() {}
  pub fn is_used_outside_of_block() {}
  pub fn get_num_successors(&self) -> usize { 0 }
  pub fn get_successor(&self, _index: u32) -> Option<&BasicBlock> { None }
  pub fn replace_successor_with() {}

  pub fn get_subclass_data(&self) {}
  pub fn set_subclass_data(&self) {}
  pub fn class_of() {}

  fn get_metadata_by_id_impl(&self, _kind_id: u32) -> Option<Box<dyn MDNode>> {
    //if kind_id == MDKind::MdDbg as u32 {
      //return self.dbg_loc.as_ref().unwrap().get_as_md_node();
    //}
    //self.get_metadata_by_id(kind_id)
    None
  }

  fn get_metadata_by_kind_impl(&self, _kind: StringRef) -> Option<Box<dyn MDNode>> {
    //self.get_metadata_by_id_impl(self.get_context().get_md_kind_id(kind))
    None
  }

  fn get_all_metadata_impl(&self) {}

}

impl Value for InstructionBase {
  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  // Return true if this instruction has any metadata attached to it.
  fn has_metadata(&self) -> bool {
    self.dbg_loc.is_some() || self.has_metadata
  }

  // Set the metadata of the specified kind to the specified node.
  fn set_metadata(&mut self, kind_id: u32, node: Option<Box<dyn MDNode>>) {
    if node.is_none() && !self.has_metadata() {
      return;
    }
    // Handle 'dbg' as a special case since it is not stored in the hash table.
    if kind_id == MDKind::MdDbg as u32 {
      //self.dbg_loc = Some(DebugLoc::new(node.unwrap()));
      return;
    }
    // Update DIAssignID to instruction(s) mapping.
    if kind_id == MDKind::MdDIAssingId as u32 {
      //debug_assert!(node.is_none() || !node.unwrap().is_temporary(),
      //"Temporary DIAssignIDs are invalid.");
      // TODO
    }

    // Value::set_metadata()
  }

  fn set_name(&self, _name: Twine) {}

  fn as_any(&self) -> &dyn Any {
    self
  }
}

struct InsertElementInst {}
impl InsertElementInst {
  pub fn new() {}
  pub fn is_valid_operands() {}
  pub fn get_type() {}
  pub fn class_of() {}
}

struct BranchInst {}
impl BranchInst {
  pub fn new() {}
}

struct SwitchInst {}
impl SwitchInst {
  pub fn new() {}
} 

struct IndirectBrInst {}
impl IndirectBrInst {
  pub fn new() {}
}

struct InvokeInst {}
impl InvokeInst {
  pub fn new() {}
}

struct CallBrInst {}
impl CallBrInst {
  pub fn new() {}
}

struct ResumeInst {}
impl ResumeInst {
  pub fn new() {}
}

struct CatchSwitchInst {}
impl CatchSwitchInst {
  pub fn new() {}
}

struct CleanupPadInst {}
impl CleanupPadInst {
  pub fn new() {}
}

struct CatchPadInst {}
impl CatchPadInst {
  pub fn new() {}
}

struct CatchReturnInst {}
impl CatchReturnInst {
  pub fn new() {}
}

struct CleanupReturnInst {}
impl CleanupReturnInst {
  pub fn new() {}
}

struct UnreachableInst {}
impl UnreachableInst {
  pub fn new() {}
}

struct TruncInst {}
impl TruncInst {
  pub fn new() {}
}

struct ZExtInst {}
impl ZExtInst {
  pub fn new() {}
}

struct SExtInst {}
impl SExtInst {
  pub fn new() {}
}

struct FPTruncInst {}
impl FPTruncInst {
  pub fn new() {}
} 

struct FPExtInst {}
impl FPExtInst {
  pub fn new() {}
}

struct UIToFPInst {}
impl UIToFPInst {
  pub fn new() {}
}

struct SIToFpInst {}
impl SIToFpInst {
  pub fn new() {}
}

struct FPToUIInst {}
impl FPToUIInst {
  pub fn new() {}
}

struct FPToSIInst {}
impl FPToSIInst {
  pub fn new() {}
}

struct IntToPtrInst {}
impl IntToPtrInst {
  pub fn new() {}
}

struct PtrToIntInst {}
impl PtrToIntInst {
  pub fn new() {}
}

struct BitCastInst {}
impl BitCastInst {
  pub fn new() {}
}

struct AddrSpaceCastInst {}
impl AddrSpaceCastInst {
  pub fn new() {}
}