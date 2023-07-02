#![allow(dead_code)]

// This file contains the declaration of the Instruction class,
// which is the base class for all of the instructions.

use crate::adt::string_ref::StringRef;
use super::{
  basic_block::BasicBlock,
  blits_context::{BlitzContext, MDKind},
  debug_loc::DebugLoc, function::Function,
  value::{Value, ValueType}, 
  type_::Type, metadata::MDNode
};

// These instructions are used to terminate a basic block of the program.
//#[derive(Debug, PartialEq, PartialOrd)]
enum TermOps {
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
  CallBr = 11
}

// Standard unary operators.
enum UnaryOps {
  FNeg = 12
}

// Standard binary operators.
enum BinaryOps {
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
  Xor = 30
}

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
  VaArg = 60,
  ExtractElement = 61,
  InsertElement = 62,
  ShuffleVector = 63,
  ExtractValue = 64,
  InsertValue = 65,
  LandingPad = 66,
  Freeze = 67
}

#[derive(Debug)]
pub struct Instruction {
  v_type: Box<dyn Type>,
  has_metadata: bool,
  parent: BasicBlock,
  dbg_loc: Option<DebugLoc>
}

impl Instruction {
  pub fn new() {}

  pub fn user_back() {}

  pub fn get_parent(&self) -> &BasicBlock {
    &self.parent
  }

  pub fn set_parent(&mut self, p: BasicBlock) {
    self.parent = p;
  }

  pub fn get_module(&self) /*-> &Module*/ {
    
  }

  // Return the function this instruction belongs to.
  pub fn get_function(&self) -> &Option<Function> {
    self.parent.get_parent()
  }
  
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

  pub fn is_terminator(&self) -> bool {
    Instruction::is_terminator_static(self.get_op_code())
  }

  pub fn is_unary_op(&self) -> bool {
    Instruction::is_unary_op_static(self.get_op_code())
  }

  pub fn is_binary_op(&self) -> bool {
    Instruction::is_binary_op_static(self.get_op_code())
  }

  pub fn is_int_div_rem(&self) -> bool {
    Instruction::is_int_div_rem_static(self.get_op_code())
  }

  pub fn is_shift(&self) -> bool {
    Instruction::is_shift_static(self.get_op_code())
  }

  pub fn is_cast(&self) -> bool {
    Instruction::is_cast_static(self.get_op_code())
  }

  pub fn is_funclet_pad(&self) -> bool {
    Instruction::is_funclet_pad_static(self.get_op_code())
  }

  pub fn is_exceptional_terminator() {}
  pub fn is_only_user_of_any_operand() {}

  pub fn is_logical_shift(&self) -> bool {
    self.get_op_code() == BinaryOps::Shl as u32 ||
    self.get_op_code() == BinaryOps::LShr as u32
  }

  pub fn is_arithmetic_shift(&self) -> bool {
    self.get_op_code() == BinaryOps::AShr as u32
  }

  pub fn is_bitwise_logic_op(&self) -> bool {
    Instruction::is_bitwise_logic_op_static(self.get_op_code())
  }

  pub fn is_terminator_static(opcode: u32) -> bool {
    TermOps::Ret as u32 <= opcode && opcode < TermOps::CallBr as u32 + 1
  }

  pub fn is_unary_op_static(opcode: u32) -> bool {
    UnaryOps::FNeg as u32 <= opcode && opcode < UnaryOps::FNeg as u32 + 1
  }

  pub fn is_binary_op_static(opcode: u32) -> bool {
    BinaryOps::Add as u32 <= opcode && opcode < BinaryOps::Xor as u32 + 1
  }

  pub fn is_int_div_rem_static(opcode: u32) -> bool {
    opcode == BinaryOps::UDiv as u32 || opcode == BinaryOps::SDiv as u32 ||
    opcode == BinaryOps::URem as u32 || opcode == BinaryOps::SRem as u32
  }

  pub fn is_shift_static(opcode: u32) -> bool {
    BinaryOps::Shl as u32 <= opcode && opcode <= BinaryOps::AShr as u32
  }

  pub fn is_bitwise_logic_op_static(opcode: u32) -> bool {
    opcode == BinaryOps::And as u32 || opcode == BinaryOps::Or as u32 ||
    opcode == BinaryOps::Xor as u32
  }

  // Determine if the opcode is one of the CastInst instructions.
  pub fn is_cast_static(opcode: u32) -> bool {
    CastOps::Trunc as u32 <= opcode && opcode < CastOps::AddrSpaceCast as u32 + 1
  }

  // Determine if the opcode is one of the FuncletPadInst instructions.
  pub fn is_funclet_pad_static(opcode: u32) -> bool {
    FuncletPadOps::CleanupPad as u32 <= opcode && opcode < FuncletPadOps::CatchPad as u32 + 1
  }

  // Return true if the opcode is a terminator related to exception handling.
  pub fn is_exceptional_terminator_static(opcode: u32) -> bool {
    if opcode == TermOps::CatchSwitch as u32 || opcode == TermOps::CatchRet as u32 ||
       opcode == TermOps::CleanupRet as u32 || opcode == TermOps::Invoke as u32 ||
       opcode == TermOps::Resume as u32
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
  pub fn get_metadata(&self, kind_id: u32) -> Option<MDNode> {
    if !self.has_metadata() {
      return None;
    }
    self.get_metadata_by_id_impl(kind_id)
  }

  // Get the metadata of given kind attached to this instruction.
  pub fn get_metadata_by_kind(&self, kind: StringRef) -> Option<MDNode> {
    if !self.has_metadata() {
      return None;
    }
    self.get_metadata_by_kind_impl(kind)
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
  pub fn is_commutative() {}
  pub fn is_idempotent() {}
  pub fn is_nilpotent() {}
  pub fn may_write_to_memory() {}
  pub fn may_read_from_memory() {}
  pub fn may_read_or_write_memory() {}
  pub fn is_atomic() {}
  pub fn has_atomic_load() {}
  pub fn has_atomic_store() {}
  pub fn is_volatile() {}
  pub fn may_throw() {}
  pub fn is_fence_like() {}
  pub fn may_have_side_effects() {}
  pub fn is_safe_to_remove() {}
  pub fn will_return() {}
  pub fn is_eh_pad() {}
  pub fn is_lifetime_start_or_end() {}
  pub fn is_launder_or_strip_invariant_group() {}
  pub fn is_debug_or_pseudo_inst() {}
  pub fn get_next_non_debug_instruction() {}
  pub fn get_prev_non_debugg_instruction() {}
  pub fn is_identical_to() {}
  pub fn is_identical_to_when_defined() {}
  pub fn is_same_operation_as() {}
  pub fn has_same_special_state() {}
  pub fn is_used_outside_of_block() {}
  pub fn get_num_successors() {}
  pub fn get_successor() {}
  pub fn replace_successor_with() {}
  pub fn class_of() {}

  fn get_metadata_by_id_impl(&self, kind_id: u32) -> Option<MDNode> {
    if kind_id == MDKind::MdDbg as u32 {
      return self.dbg_loc.as_ref().unwrap().get_as_md_node();
    }
    self.get_metadata_by_id(kind_id)
  }

  fn get_metadata_by_kind_impl(&self, kind: StringRef) -> Option<MDNode> {
    self.get_metadata_by_id_impl(self.get_context().get_md_kind_id(kind))
  }

  fn get_all_metadata_impl(&self) {}

}

impl Value for Instruction {
  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_context(&self) -> &BlitzContext {
    self.v_type.as_ref().get_context()
  }

  // Return true if this instruction has any metadata attached to it.
  fn has_metadata(&self) -> bool {
    self.dbg_loc.is_some() || self.has_metadata
  }

  // Set the metadata of the specified kind to the specified node.
  fn set_metadata(&mut self, kind_id: u32, node: Option<MDNode>) {
    if node.is_none() && !self.has_metadata() {
      return;
    }
    // Handle 'dbg' as a special case since it is not stored in the hash table.
    if kind_id == MDKind::MdDbg as u32 {
      self.dbg_loc = Some(DebugLoc::new(node.unwrap()));
      return;
    }
    // Update DIAssignID to instruction(s) mapping.
    if kind_id == MDKind::MdDIAssingId as u32 {
      debug_assert!(node.is_none() || !node.unwrap().is_temporary(),
      "Temporary DIAssignIDs are invalid.");
      // TODO
    }

    // Value::set_metadata()
  }
}

struct ICmpInst {}
impl ICmpInst {
  pub fn new() {}
  pub fn get_signed_predicate() {}
  pub fn get_unsigned_predicate() {}
  pub fn is_equality() {}
  pub fn is_commutative() {}
  pub fn is_relational() {}
  pub fn is_gt() {}
  pub fn is_lt() {}
  pub fn is_ge() {}
  pub fn is_le() {}
  pub fn predicates() {}
  pub fn swap_operands() {}
  pub fn compare() {}
  pub fn class_of() {}
}

struct FCmpInst {}
impl FCmpInst {
  pub fn new() {}
}

enum TailCallKind {
  None,
  Tail,
  MustTail,
  NoTail
}

struct CallInst {}
impl CallInst {
  pub fn new() {}
  pub fn get_tail_call_kind() {}
  pub fn is_tail_call() {}
  pub fn is_must_tail_call() {}
  pub fn is_no_tail_call() {}
  pub fn set_tail_call_kind() {}
  pub fn set_tail_call() {}
  pub fn can_return_twice() {}
  pub fn set_can_return_twice() {}
  pub fn class_of() {}
  pub fn update_prof_weight() {}
}

struct SelectInst {}
impl SelectInst {
  pub fn new() {}
  pub fn get_condition() {}
  pub fn get_true_value() {}
  pub fn get_false_value() {}
  pub fn set_condition() {}
  pub fn set_true_value() {}
  pub fn set_false_value() {}
  pub fn swap_values() {}
  pub fn are_invalid_operands() {}
  pub fn get_opcode() {}
  pub fn class_of() {}
}

struct VAArgInst {}
impl VAArgInst {
  pub fn new() {}
  pub fn get_pointer_operand() {}
  pub fn get_pointer_operand_index() {}
  pub fn class_of() {}
}

struct ExtractElementInst {}
impl ExtractElementInst {
  pub fn new() {}
  pub fn is_valid_operands() {}
  pub fn get_vector_operand() {}
  pub fn get_index_operand() {}
  pub fn get_vector_operand_type() {}
  pub fn class_of() {}
}

struct InsertElementInst {}
impl InsertElementInst {
  pub fn new() {}
  pub fn is_valid_operands() {}
  pub fn get_type() {}
  pub fn class_of() {}
}

struct ShuffleVectorInst {}
impl ShuffleVectorInst {
  pub fn new() {}
}

struct ExtractValueInst {}
impl ExtractValueInst {
  pub fn new() {}
}

struct InsertValueInst {}
impl InsertValueInst {
  pub fn new() {}
}

struct PhiNode {}
impl PhiNode {
  pub fn new() {}
}

struct LandingPadInst {}
impl LandingPadInst {
  pub fn new() {}
}

struct ReturnInst {}
impl ReturnInst {
  pub fn new() {}
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