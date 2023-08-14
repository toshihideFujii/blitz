#![allow(dead_code)]

// This file exposes the class definitions of all of the subclasses
// of the Instruction class. This is meant to be an easy way to get
// access to allinstruction subclasses.

use crate::{support::atomic_ordering::AtomicOrdering, adt::twine::Twine};
use super::{instr_types::{UnaryInstruction, CmpInst, Predicate, CallBase,
  OperandBundleDefType}, type_, type_::{Type, /*FunctionType*/},
    instruction::{Instruction, TermOps, OpCode}, value::Value,
    attributes::{AttrKind, /*AttributeList*/}, blits_context::BlitzContext,
  /*basic_block::BasicBlock*/};

// An instruction to allocate memory on the stack.
struct AllocaInst {
  uinst: UnaryInstruction,
  allocated_type: Box<dyn Type>
}

impl AllocaInst {
  pub fn new() {}
  pub fn is_array_allocation() {}

  // Get the number of elements allocated.
  // For a simple allocation of a single element, this will return
  // a constant 1 value.
  pub fn get_array_size(&self) -> Option<Box<dyn Value>> {
    self.uinst.get_operand(0)
  }

  pub fn get_type() {}
  pub fn get_address_space() {}
  pub fn get_allocation_size() {}
  pub fn get_allocation_size_in_bits() {}

  // Return the type that is being allocated by the instruction.
  pub fn get_allocated_type(&self) -> &Box<dyn Type> {
    &self.allocated_type
  }

  // For use only in special circumstances that need to generically
  // transform a whole instruction (ex: IR linking and vectorization).
  pub fn set_allocated_type(&mut self, allocated_type: Box<dyn Type>) {
    self.allocated_type = allocated_type;
  }

  pub fn get_align() {}
  pub fn set_alignment() {}
  pub fn is_static_alloca() {}
  pub fn is_used_with_in_alloca() {}
  pub fn set_used_with_in_alloca() {}

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::Alloca
  }
}

// An instruction for reading from memory.
// This uses the sub_class_data field in Value to store weheter or
// not the load is volatile.
struct LoadInst {
  uinst: UnaryInstruction,
  ssid: u8
}

impl LoadInst {
  pub fn new() {}
  pub fn is_volatile(&self) -> bool { false }
  pub fn set_volatile() {}
  pub fn get_align() {}
  pub fn set_alignment() {}

  // Returns the ordering constraint of this load instruction.
  pub fn get_ordering(&self) -> AtomicOrdering {
    AtomicOrdering::NotAtomic
  }

  // Sets the ordering constraint of this load instruction.
  // May not be Release or AcquireRelease.
  pub fn set_ordering(&mut self, _ordering: AtomicOrdering) {}

  // Returns the synchronization scope ID of this load instruction.
  pub fn get_sync_scope_id(&self) -> u8 {
    self.ssid
  }

  // Sets the synchronization scope ID of this load instruction.
  pub fn set_sync_scope_id(&mut self, ssid: u8) {
    self.ssid = ssid;
  }

  pub fn set_atomic() {}

  pub fn is_atomic(&self) -> bool {
    self.uinst.inst.is_atomic()
  }

  pub fn is_simple(&self) -> bool {
    !self.is_atomic() && !self.is_volatile()
  }

  pub fn is_unordered(&self) -> bool {
    (self.get_ordering() == AtomicOrdering::NotAtomic ||
     self.get_ordering() == AtomicOrdering::Unordered) &&
     !self.is_volatile()
  }

  pub fn get_pointer_operand(&self) -> Option<Box<dyn Value>> {
    self.uinst.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 0 }
  pub fn get_pointer_address_space() {}

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::Load 
  }
}

// An instruction for storing to memory.
struct StoreInst {
  inst: Instruction,
  ssid: u8
}

impl StoreInst {
  pub fn new() {}

  // Return true if this is a store to a volatile memory location.
  pub fn is_volatile(&self) -> bool { false }

  pub fn set_volatile() {}

  pub fn get_operand(&self, _i: u32) -> Option<Box<dyn Value>> { None }
  pub fn set_operand(&self, _i: u32, _v: Option<Box<dyn Value>>) {}

  pub fn get_align() {}
  pub fn set_alignment() {}

  // Returns the ordering constraint of this store instruction.
  pub fn get_ordering(&self) -> AtomicOrdering {
    AtomicOrdering::NotAtomic
  }

  pub fn set_ordering(&mut self, _ordering: AtomicOrdering) {}

  // Returns the synchronization scope ID of this store instruction.
  pub fn get_sync_scope_id(&self) -> u8 {
    self.ssid
  }

  // Sets the synchronization scope ID of this store instruction.
  pub fn set_sync_scope_id(&mut self, ssid: u8) {
    self.ssid = ssid;
  }

  // Sets the ordering constraint and the synchronization scope id
  // of this instruction.
  pub fn set_atomic(&mut self, ordering: AtomicOrdering, ssid: u8) {
    self.set_ordering(ordering);
    self.set_sync_scope_id(ssid);
  }

  pub fn is_simple(&self) -> bool {
    !self.inst.is_atomic() && !self.is_volatile()
  }

  pub fn is_unordered(&self) -> bool {
    (self.get_ordering() == AtomicOrdering::NotAtomic || 
    self.get_ordering() == AtomicOrdering::Unordered) &&
    !self.is_volatile()
  }

  pub fn get_value_operand(&self) -> Option<Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand(&self) -> Option<Box<dyn Value>> {
    self.get_operand(1)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 1 }
  pub fn get_pointer_address_space() {}

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::Store
  }
}

// An instruction for ordering other memory operations.
struct FenceInst {
  inst: Instruction,
  ssid: u8
}

impl FenceInst {
  pub fn new() {}

  // Returns the ordering constraint of this store instruction.
  pub fn get_ordering(&self) -> AtomicOrdering {
    AtomicOrdering::NotAtomic
  }

  pub fn set_ordering(&mut self, _ordering: AtomicOrdering) {}

  // Returns the synchronization scope ID of this fence instruction.
  pub fn get_sync_scope_id(&self) -> u8 {
    self.ssid
  }

  // Sets the synchronization scope ID of this fence instruction.
  pub fn set_sync_scope_id(&mut self, ssid: u8) {
    self.ssid = ssid;
  }

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::Fence
  }
}

// An instruction that atomically checks whether a specified value is
// in a memory location, and, if it is, stores a new value there.
struct AtomicCmpXchgInst {
  inst: Instruction,
  ssid: u8
}

impl AtomicCmpXchgInst {
  pub fn new() {}
  pub fn get_align() {}
  pub fn set_alignment() {}
  pub fn is_volatile() {}
  pub fn set_volatile() {}

  // Return true if this cmpxchg may suriously fail.
  pub fn is_weak(&self) -> bool { false }

  pub fn set_weak() {}
  pub fn get_operand(&self, _i: u32) -> Option<Box<dyn Value>> { None }
  pub fn set_operand(&mut self, _i: u32, _v: Option<Box<dyn Value>>) {}

  pub fn is_valid_success_ordering(ordering: AtomicOrdering) -> bool {
    ordering != AtomicOrdering::NotAtomic && ordering != AtomicOrdering::Unordered
  }

  pub fn is_valid_failure_ordering(ordering: AtomicOrdering) -> bool {
    ordering != AtomicOrdering::NotAtomic && ordering != AtomicOrdering::Unordered &&
    ordering != AtomicOrdering::AcquireRelease && ordering != AtomicOrdering::Release
  }

  // Returns the success ordering constraint of this of this cmpxchg instruction.
  pub fn get_success_ordering(&self) -> AtomicOrdering {
    AtomicOrdering::Acquire
  }

  pub fn set_success_ordering() {}

  // Returns the failure ordering constraint of this of this cmpxchg instruction.
  pub fn get_failure_ordering(&self) -> AtomicOrdering {
    AtomicOrdering::Acquire
  }

  pub fn set_failure_ordering() {}
  pub fn get_merged_ordering() {}

  pub fn get_sync_scope_id(&self) -> u8 {
    self.ssid
  }

  pub fn set_sync_scope_id(&mut self, ssid: u8) {
    self.ssid = ssid;
  }

  pub fn get_pointer_operand(&self) -> Option<Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 0 }

  pub fn get_compare_operand(&self) -> Option<Box<dyn Value>> {
    self.get_operand(1)
  }

  pub fn get_new_val_operand(&self) -> Option<Box<dyn Value>> {
    self.get_operand(2)
  }

  pub fn get_pointer_address_space() {}
  pub fn get_strongest_failure_ordering() {}

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::AtomicCmpXchg
  }
}

pub enum BinOp {
  Xchg,
  Add,
  Sub,
  And,
  Nand,
  Or,
  Xor,
  Max,
  Min,
  UMax,
  UMin,
  FAdd,
  FSub,
  FMax,
  FMin,
  UIncWrap
}

struct AtomicRMWInst {
  inst: Instruction
}

impl AtomicRMWInst {
  pub fn new() {}
  pub fn get_operation() {}
  pub fn get_operation_name() {}

  pub fn is_fp_operation(op: BinOp) -> bool {
    match op {
      BinOp::FAdd => return true,
      BinOp::FSub => return true,
      BinOp::FMax => return true,
      BinOp::FMin => return true,
      _ => return false
    };
  }

  pub fn set_operation() {}
  pub fn get_align() {}
  pub fn set_alignment() {}
  pub fn is_volatile() {}

  pub fn get_operand(&self, _i: u32) -> Option<Box<dyn Value>>{
    None
  }

  pub fn set_operand(&mut self, _i: u32, _v: Option<Box<dyn Value>>) {}

  pub fn get_ordering() {}
  pub fn set_ordering() {}
  pub fn get_sync_scope_id() {}
  pub fn set_sync_scope_id() {}

  pub fn get_pointer_operand(&self) -> Option<Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 0 }

  pub fn get_val_operand(&self) -> Option<Box<dyn Value>> {
    self.get_operand(1)
  }

  pub fn get_pointer_address_space() {}
  pub fn is_floating_pointer_operation() {}

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::AtomicRMW
  }
}

// An instruction for type-safe pointer arithmetic to access elements
// of arrays and structs.
struct GetElementPtrInst {
  inst: Instruction,
  src_elt_type: Option<Box<dyn Type>>,
  result_elt_type: Option<Box<dyn Type>>
}

impl GetElementPtrInst {
  pub fn new() {}
  pub fn create_in_bounds() {}
  pub fn get_operand(&self, _i: u32) -> Option<Box<dyn Value>> { None }
  pub fn set_operand(&self, _i: u32, _v: Option<Box<dyn Value>>) {}

  pub fn get_num_operands(&self) -> u32 { 0 }

  pub fn get_source_element_type(&self) -> &Option<Box<dyn Type>> {
    &self.src_elt_type
  }

  pub fn set_source_element_type(&mut self, src_elt_type: Option<Box<dyn Type>>) {
    self.src_elt_type = src_elt_type;
  }

  pub fn set_result_element_type(&mut self, result_elt_type: Option<Box<dyn Type>>) {
    self.result_elt_type = result_elt_type;
  }

  pub fn get_result_element_type(&self) -> &Option<Box<dyn Type>> {
    &self.result_elt_type
  }

  pub fn get_address_space() {}
  pub fn get_pointer_address_space() {}
  pub fn get_indexed_type() {}
  pub fn get_type_at_index() {}

  pub fn get_pointer_operand(&self) -> Option<Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 0 }

  pub fn get_pointer_operand_type() {}
  pub fn get_gep_return_type() {}

  pub fn get_num_indices(&self) -> u32 {
    self.get_num_operands() - 1
  }

  pub fn has_indices(&self) -> bool {
    self.get_num_operands() > 1
  }

  pub fn has_all_zero_indices() {}
  pub fn has_all_constant_indices() {}
  pub fn set_is_in_bounds() {}
  pub fn is_in_bounds(&self) -> bool { false }

  pub fn accumulate_constant_offset() {}
  pub fn collect_offset() {}

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::GetElementPtr
  }
}

// This instruction compares its operands according to the predicate
// given to the constructor. It only operates on integers or pointers.
// The operands must be identical types.
struct ICmpInst {
  cmp_inst: CmpInst
}

impl ICmpInst {
  pub fn new() {}
  pub fn get_signed_predicate() {}
  pub fn get_unsigned_predicate() {}
  pub fn is_equality() {}
  pub fn is_commutative() {}
  pub fn is_relational() {}

  // Return true if the predicate is SGT or UGT.
  pub fn is_gt(p: Predicate) -> bool {
    p == Predicate::IcmpSgt || p == Predicate::IcmpUgt
  }

  // Return true if the predicate is SLT or ULT.
  pub fn is_lt(p: Predicate) -> bool {
    p == Predicate::IcmpSlt || p == Predicate::IcmpUlt
  }

  // Return true if the predicate is SGE or UGE.
  pub fn is_ge(p: Predicate) -> bool {
    p == Predicate::IcmpSge || p == Predicate::IcmpUge
  }

  // Return true if the predicate is SLE or ULE.
  pub fn is_le(p: Predicate) -> bool {
    p == Predicate::IcmpSle || p == Predicate::IcmpUle
  }

  pub fn predicates() {}
  pub fn swap_operands() {}
  pub fn compare() {}

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::ICmp
  }
}

// This instruction compares its operands according to the predicate given
// to the constructor. It only operates on floating point values or packedvectors
// of floating point values. The operands must be identical types. Represents
// a floating point comparison operator.
struct FCmpInst {
  cmp_inst: CmpInst
}

impl FCmpInst {
  pub fn new() {}

  // Returns true if this instruction is EQ or NE.
  // Determine if this is an equality predicaate.
  pub fn is_equality(&self, p: Predicate) -> bool {
    p == Predicate::FcmpOeq || p == Predicate::FcmpOne ||
    p == Predicate::FcmpUeq || p == Predicate::FcmpUne
  }
  
  pub fn is_commutative() {}
  pub fn is_relational() {}
  pub fn swap_operands() {}
  pub fn predicates() {}
  pub fn compare() {}

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::FCmp
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TailCallKind {
  None,
  Tail,
  MustTail,
  NoTail
}

// This class represents a function call, abstracting a target machine's
// calling convention. This class uses low bit of the syb_class_data field
// to indicate whether or not this is a tail call. The rest of the bits
// hold the calling convention of the call.
struct CallInst {
  call_base: CallBase
}

impl CallInst {
  /*
  pub fn new_ib(ft: FunctionType, _func: Box<dyn Value>,
    args: Vec<Box<dyn Value>>, bundles: Vec<OperandBundleDefType<Box<dyn Value>>>,
    _name: Twine, ib: Option<Box<Instruction>>) -> Self
  {
    let call_inst = CallInst {
      call_base: CallBase::new_ib(ft.get_return_type(),
      OtherOps::Call as u32, None,
      args.len() as u32 + CallBase::count_bundle_inputs(bundles) as u32 + 1,
      ib, AttributeList::new(), ft)
    };
    call_inst
  }

  pub fn new_ie(ft: FunctionType, _func: Box<dyn Value>,
    args: Vec<Box<dyn Value>>, bundles: Vec<OperandBundleDefType<Box<dyn Value>>>,
    _name: Twine, ie: Option<BasicBlock>) -> Self
  {
    CallInst { call_base: CallBase::new_ie(ft.get_return_type(),
      OtherOps::Call as u32, None,
      args.len() as u32 + CallBase::count_bundle_inputs(bundles) as u32 + 1,
      ie, AttributeList::new(), ft)
    }
  }
  */

  fn init(&mut self, func: Box<dyn Value>, _args: Vec<Box<dyn Value>>,
    _bundles: Vec<OperandBundleDefType<Box<dyn Value>>>, name: Twine)
  {
    // Set operands in order of their index to match use-list-order prediction.
    // copy(args, op);
    self.call_base.set_called_operand(func);

    self.call_base.inst.set_name(name);
  }

  pub fn compute_num_operands() {}
  pub fn craete_malloc() {}
  pub fn create_free() {}

  pub fn get_tail_call_kind(&self) -> TailCallKind {
    TailCallKind::None
  }

  pub fn is_tail_call(&self) -> bool {
    let kind = self.get_tail_call_kind();
    kind == TailCallKind::Tail || kind == TailCallKind::MustTail
  }

  pub fn is_must_tail_call(&self) -> bool {
    self.get_tail_call_kind() == TailCallKind::MustTail
  }

  pub fn is_no_tail_call(&self) -> bool {
    self.get_tail_call_kind() == TailCallKind::NoTail
  }

  pub fn set_tail_call_kind(_tck: TailCallKind) {}

  // Return true if the call can return twice.
  pub fn can_return_twice(&self) -> bool {
    self.call_base.has_fn_attr(AttrKind::ReturnsTwice)
  }

  pub fn set_can_return_twice(&mut self) {
    self.call_base.add_fn_attr(AttrKind::ReturnsTwice)
  }

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::Call
  }

  pub fn update_prof_weight() {}
}

struct SelectInst {}

struct VAArgInst {}

struct ExtractElementInst {}

// This instruction inserts a single (scalar) element into a VectorType
// value.
struct InsertElementInst {}

struct ShuffleVectorInst {}

struct ExtractValueInst {}

struct InsertValueInst {}

struct PhiNode {}

struct LandingPadInst {}

// Return a value (possibly void), from a function.
// Exception does not continue in this function any longer.
struct ReturnInst {
  inst: Instruction,
  retval: Option<Box<dyn Value>>
}

impl ReturnInst {
  pub fn new_ib(c: &mut BlitzContext, retval: Option<Box<dyn Value>>,
    ib: Option<Box<Instruction>>) -> Self
  {
    ReturnInst {
      inst: Instruction::new_ib(Box::new(type_::get_void_type(c)),
        TermOps::Ret as u32,
        None, 0, ib), // TOSO: ops, num_ops
      retval: retval
    }
  }

  pub fn get_return_value(&self) -> &Option<Box<dyn Value>> {
    if self.get_num_operands() != 0 {
      return self.get_operand(0);
    }
    &None
  }

  pub fn get_num_successors(&self) -> u32 { 0 }

  pub fn get_operand(&self, i: u32) -> &Option<Box<dyn Value>> {
    if i == 0 && self.retval.is_some() {
      return &self.retval;
    }
    &None
  }

  pub fn set_operand(&mut self, i: u32, v: Option<Box<dyn Value>>) {
    debug_assert!(i == 0, "Set value at index 0 only.");
    self.retval = v;
  }

  pub fn get_num_operands(&self) -> u32 { 
    if self.retval.is_some() {
      return 1;
    }
    0
  }

  pub fn class_of(i: Instruction) -> bool {
    i.get_op_code() == OpCode::Ret
  }
}

// Conditional or unconditional branch instruction.
struct BranchInst {}

struct SwitchInst {}

// Indirect branch instruction.
struct IndirectBrInst {}

struct InvokeInst {}

struct CallBrInst {}

// Resume the propagation of an exception.
struct ResumeInst {}

struct CatchSwitchInst {}

struct CleanuppadInst {}

struct CatchPadInst {}

struct CatchReturnInst {}

struct CleanupReturnInst {}

struct UnreachableInst {}

struct TruncInst {}

// This class represents zero extension of integer types.
struct ZExtInst {}

// This class represents sign extension of integer types.
struct SExtInst {}

struct FPTruncInst {}

struct FPExtInst {}

struct UItoFPInst {}

struct SIToFPInst {}

struct FPToUIInst {}

struct FPToSIInst {}

struct IntToPtrInst {}

struct PtrToIntInst {}

struct BitCastInst {}

struct AddrSpaceCastInst {}

struct FreezeInst {}

#[cfg(test)]
mod tests {
  use crate::ir::{type_::IntegerType, constants::ConstantInt};
  use super::*;

  #[test]
  fn test_return_inst() {
    let mut c = BlitzContext::new();
    let r0 = ReturnInst::new_ib(&mut c, None, None);
    assert_eq!(r0.get_num_operands(), 0);

    let i1 = IntegerType::get(&mut c, 1);
    let one = ConstantInt::get(&i1, 1, true);
    let r1 = ReturnInst::new_ib(&mut c, Some(Box::new(one)), None);
    assert_eq!(r1.get_num_operands(), 1);
    //assert_eq!(r1.get_operand(0).unwrap().as_ref(), one);
  }
}