#![allow(dead_code)]

// This file exposes the class definitions of all of the subclasses
// of the Instruction class. This is meant to be an easy way to get
// access to allinstruction subclasses.

use std::any::Any;
use crate::{support::{atomic_ordering::AtomicOrdering, alignment::Align},
  adt::{twine::Twine, ap_int::APInt}};
use super::{instr_types::{UnaryInstruction, CmpInst, Predicate, CallBase,
  OperandBundleDefType}, type_, type_::{Type, PointerType, VectorType, /*FunctionType*/},
    instruction::{InstructionBase, TermOps, OpCode, Instruction}, value::Value,
    attributes::{AttrKind, /*AttributeList*/},
    blits_context::{BlitzContext, SyncScopeID}, basic_block::BasicBlock,
    constants::ConstantInt, constant::Constant,
  /*basic_block::BasicBlock*/};

// An instruction to allocate memory on the stack.
#[derive(Debug)]
struct AllocaInst {
  v_type: Box<dyn Type>,
  addr_spce: u32,
  array_size: Box<dyn Value>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>,
  align: Option<Align>,
  parent: Option<BasicBlock>,
  used_with_in_alloca: bool,
}

impl AllocaInst {
  pub fn new_insert_before(t: Box<dyn Type>, addr_space: u32,
    array_size: Box<dyn Value>, name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    AllocaInst { v_type: t, addr_spce: addr_space, array_size: array_size,
      name: name, insert_before: ib, insert_at_end: None, align: None, parent: None,
      used_with_in_alloca: false }
  }

  pub fn new_insert_at_end(t: Box<dyn Type>, addr_space: u32,
    array_size: Box<dyn Value>, name: Twine, ie: Option<BasicBlock>) -> Self
  {
    AllocaInst { v_type: t, addr_spce: addr_space, array_size: array_size,
      name: name, insert_before: None, insert_at_end: ie, align: None, parent: None,
      used_with_in_alloca: false }
  }

  // Return true if there is an allocation size parameter to the
  // allocation instruction that is not 1.
  pub fn is_array_allocation(&self) -> bool {
    let ci =
      self.get_operand(0).unwrap().as_any().downcast_ref::<ConstantInt>();
    if ci.is_some() { return ci.unwrap().is_one(); }
    true
  }

  // Get the number of elements allocated.
  // For a simple allocation of a single element, this will return
  // a constant 1 value.
  pub fn get_array_size(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  // Overload to return most specific pointer type.
  pub fn get_ptr_type(&self) -> Option<PointerType> {
    None
  }

  // Return the address space for the allocation.
  pub fn get_address_space(&self) -> u32 {
    self.get_ptr_type().unwrap().get_address_space()
  }

  pub fn get_allocation_size() {}
  pub fn get_allocation_size_in_bits() {}

  // Return the type that is being allocated by the instruction.
  pub fn get_allocated_type(&self) -> &Box<dyn Type> {
    &self.v_type
  }

  // For use only in special circumstances that need to generically
  // transform a whole instruction (ex: IR linking and vectorization).
  pub fn set_allocated_type(&mut self, allocated_type: Box<dyn Type>) {
    self.v_type = allocated_type;
  }

  // Return the alignment of the memory that is being allocated by
  // the instruction.
  pub fn get_align(&self) -> &Option<Align> {
    &self.align
  }

  pub fn set_alignment(&mut self, align: Align) {
    self.align = Some(align);
  }

  // Return true if this alloca is in the entry block of the function and
  // is a constant size. If so, the code generator will fold it into the
  // prolog/epilog code, so it is basically free.
  pub fn is_static_alloca(&self) -> bool {
    // Must be constant size.
    if self.get_array_size().unwrap().as_any().downcast_ref::<ConstantInt>().is_none() {
      return false;
    }
    // Must be in the entry block.
    let parent = self.get_parent();
    parent.as_ref().unwrap().is_entry_block() && !self.is_used_with_in_alloca()
  }

  // Return true if this alloca is used as an inalloca argument to a call.
  // Such alloca are never considered static even if they are in the ebtry block.
  pub fn is_used_with_in_alloca(&self) -> bool {
    self.used_with_in_alloca
  }

  // Specify whether this alloca is used to represent the arguments to a call.
  pub fn set_used_with_in_alloca(&mut self, v: bool) {
    self.used_with_in_alloca = v;
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Alloca
  }
}

impl Instruction for AllocaInst {
  fn get_parent(&self) -> &Option<BasicBlock> {
    &self.parent
  }

  fn get_op_code(&self) -> OpCode {
    OpCode::Alloca
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl UnaryInstruction for AllocaInst {
  fn get_operand(&self, i: u32) -> Option<&Box<dyn Value>> {
    if i == 0 {
      return Some(&self.array_size);
    }
    None
  }
}

// An instruction for reading from memory.
// This uses the sub_class_data field in Value to store weheter or not the
// load is volatile.
#[derive(Debug)]
pub struct LoadInst {
  v_type: Box<dyn Type>,
  operand: Box<dyn Value>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>,
  volatile: bool,
  align: Option<Align>,
  ordering: AtomicOrdering,
  ssid: SyncScopeID
}

impl LoadInst {
  pub fn new_insert_before(t: Box<dyn Type>, operand: Box<dyn Value>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    LoadInst {
      v_type: t, operand: operand, name: name, insert_before: ib,
      insert_at_end: None, volatile: false, align: None, 
      ordering: AtomicOrdering::Unordered, ssid: SyncScopeID::SingleThread
    }
  }

  pub fn new_insert_at_end(t: Box<dyn Type>, operand: Box<dyn Value>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    LoadInst {
      v_type: t, operand: operand, name: name, insert_before: None,
      insert_at_end: ie, volatile: false, align: None,
      ordering: AtomicOrdering::Unordered,  ssid: SyncScopeID::SingleThread
    }
  }

  // Return true if this is a load from a volatile memory location.
  pub fn is_volatile(&self) -> bool {
    self.volatile
  }

  // Specify whether this is a volatile load or not.
  pub fn set_volatile(&mut self, volatile: bool) {
    self.volatile = volatile;
  }

  // Return the alignment of the access that is being performed.
  pub fn get_align(&self) -> Option<Align> {
    self.align
  }

  pub fn set_alignment(&mut self, align: Option<Align>) {
    self.align = align;
  }

  // Returns the ordering constraint of this load instruction.
  pub fn get_ordering(&self) -> &AtomicOrdering {
    &self.ordering
  }

  // Sets the ordering constraint of this load instruction.
  // May not be Release or AcquireRelease.
  pub fn set_ordering(&mut self, ordering: AtomicOrdering) {
    self.ordering = ordering;
  }

  // Returns the synchronization scope ID of this load instruction.
  pub fn get_sync_scope_id(&self) -> &SyncScopeID {
    &self.ssid
  }

  // Sets the synchronization scope ID of this load instruction.
  pub fn set_sync_scope_id(&mut self, ssid: SyncScopeID) {
    self.ssid = ssid;
  }

  // Sets the ordering constraint and the synchronization scope ID of
  // this load instruction.
  pub fn set_atomic(&mut self, ordering: AtomicOrdering, ssid: SyncScopeID) {
    self.set_ordering(ordering);
    self.set_sync_scope_id(ssid);
  }

  pub fn is_simple(&self) -> bool {
    !self.is_atomic() && !self.is_volatile()
  }

  pub fn is_unordered(&self) -> bool {
    (self.get_ordering() == &AtomicOrdering::NotAtomic ||
     self.get_ordering() == &AtomicOrdering::Unordered) &&
     !self.is_volatile()
  }

  pub fn get_pointer_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 0 }

  //pub fn get_pointer_operand_type(&self) -> &dyn Type {
    //self.get_pointer_operand().as_ref().unwrap().get_type()
  //}

  pub fn get_pointer_address_space(&self) {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Load 
  }
}

impl Instruction for LoadInst {
  fn get_op_code(&self) -> OpCode {
    OpCode::Load
  }

  fn is_atomic(&self) -> bool {
    return self.get_ordering() != &AtomicOrdering::NotAtomic
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl UnaryInstruction for LoadInst {
  fn get_operand(&self, _i: u32) -> Option<&Box<dyn Value>> {
    Some(&self.operand)
  }
}

// An instruction for storing to memory.
#[derive(Debug)]
pub struct StoreInst {
  val_operand: Box<dyn Value>,
  ptr_operand: Box<dyn Value>,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>,
  volatile: bool,
  align: Option<Align>,
  ordering: AtomicOrdering,
  ssid: SyncScopeID
}

impl StoreInst {
  pub fn new_insert_before(v: Box<dyn Value>, addr: Box<dyn Value>,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    StoreInst {
      val_operand: v, ptr_operand: addr, insert_before: ib,
      insert_at_end: None, volatile: false, align: None,
      ordering: AtomicOrdering::Unordered, ssid: SyncScopeID::SingleThread
    }
  }

  pub fn new_insert_at_end(v: Box<dyn Value>, addr: Box<dyn Value>,
    ie: Option<BasicBlock>) -> Self
  {
    StoreInst {
      val_operand: v, ptr_operand: addr, insert_before: None,
      insert_at_end: ie, volatile: false, align: None,
      ordering: AtomicOrdering::Unordered, ssid: SyncScopeID::SingleThread
    }
  }

  // Return true if this is a store to a volatile memory location.
  pub fn is_volatile(&self) -> bool {
    self.volatile
  }

  // Specify whether this is a volatile store or not.
  pub fn set_volatile(&mut self, volatile: bool) {
    self.volatile = volatile;
  }

  pub fn get_operand(&self, i: u32) -> Option<&Box<dyn Value>> {
    if i == 0 { return Some(&self.val_operand); }
    else if i == 1 { return Some(&self.ptr_operand); }
    None
  }

  pub fn set_operand(&mut self, i: u32, v: Box<dyn Value>) {
    if i == 0 { self.val_operand = v; }
    else if i == 1 { self.ptr_operand = v; }
  }

  pub fn get_align(&self) -> Option<Align> {
    self.align
  }

  pub fn set_alignment(&mut self, align: Option<Align>) {
    self.align = align;
  }

  // Returns the ordering constraint of this store instruction.
  pub fn get_ordering(&self) -> &AtomicOrdering {
    &self.ordering
  }

  // Sets the ordering constraint of this store instruction.
  // May not be Acquire or AcquireRelease.
  pub fn set_ordering(&mut self, ordering: AtomicOrdering) {
    self.ordering = ordering;
  }

  // Returns the synchronization scope ID of this store instruction.
  pub fn get_sync_scope_id(&self) -> &SyncScopeID {
    &self.ssid
  }

  // Sets the synchronization scope ID of this store instruction.
  pub fn set_sync_scope_id(&mut self, ssid: SyncScopeID) {
    self.ssid = ssid;
  }

  // Sets the ordering constraint and the synchronization scope id
  // of this instruction.
  pub fn set_atomic(&mut self, ordering: AtomicOrdering, ssid: SyncScopeID) {
    self.set_ordering(ordering);
    self.set_sync_scope_id(ssid);
  }

  pub fn is_simple(&self) -> bool {
    !self.is_atomic() && !self.is_volatile()
  }

  pub fn is_unordered(&self) -> bool {
    (self.get_ordering() == &AtomicOrdering::NotAtomic || 
    self.get_ordering() == &AtomicOrdering::Unordered) &&
    !self.is_volatile()
  }

  pub fn get_value_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(1)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 1 }

  pub fn get_pointer_address_space(&self) {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Store
  }
}

impl Instruction for StoreInst {
  fn get_op_code(&self) -> OpCode {
    OpCode::Store
  }

  fn is_atomic(&self) -> bool {
    return self.get_ordering() != &AtomicOrdering::NotAtomic
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

// An instruction for ordering other memory operations.
#[derive(Debug)]
pub struct FenceInst {
  ordering: AtomicOrdering,
  ssid: SyncScopeID,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl FenceInst {
  pub fn new_insert_before(ordering: AtomicOrdering, ssid: SyncScopeID,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    FenceInst { ordering: ordering, ssid: ssid, insert_before: ib,
      insert_at_end: None }
  }

  pub fn new_insert_at_end(ordering: AtomicOrdering, ssid: SyncScopeID,
    ie: Option<BasicBlock>) -> Self
  {
    FenceInst { ordering: ordering, ssid: ssid, insert_before: None,
      insert_at_end: ie }
  }

  // Returns the ordering constraint of this store instruction.
  pub fn get_ordering(&self) -> &AtomicOrdering {
    &self.ordering
  }

  // Sets the ordering constraint of this fence instruction.
  // May only be Acquire, Release, AcquireRelease, orSequentiallyConsistent.
  pub fn set_ordering(&mut self, ordering: AtomicOrdering) {
    self.ordering = ordering;
  }

  // Returns the synchronization scope ID of this fence instruction.
  pub fn get_sync_scope_id(&self) -> &SyncScopeID {
    &self.ssid
  }

  // Sets the synchronization scope ID of this fence instruction.
  pub fn set_sync_scope_id(&mut self, ssid: SyncScopeID) {
    self.ssid = ssid;
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Fence
  }
}

impl Instruction for FenceInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// An instruction that atomically checks whether a specified value is
// in a memory location, and, if it is, stores a new value there.
// The value returned by this instruction is a pair containing the original
// value as first element, and an i1 indicating success (true) or failure
// (false) as second element.
#[derive(Debug)]
pub struct AtomicCmpXchgInst {
  ptr: Box<dyn Value>,
  cmp: Box<dyn Value>,
  new_val: Box<dyn Value>,
  align: Align,
  volatile: bool,
  weak: bool,
  success_ordering: AtomicOrdering,
  failure_ordering: AtomicOrdering,
  ssid: SyncScopeID,
  ib: Option<Box<dyn Instruction>>,
  ie: Option<BasicBlock>
}

impl AtomicCmpXchgInst {
  pub fn new_insert_before(ptr: Box<dyn Value>, cmp: Box<dyn Value>,
    new_val: Box<dyn Value>, align: Align, success_ordering: AtomicOrdering,
    failure_ordering: AtomicOrdering, ssid: SyncScopeID, ib: Option<Box<dyn Instruction>>) -> Self
  {
    AtomicCmpXchgInst {
      ptr: ptr, cmp: cmp, new_val: new_val, align: align, volatile: false, weak: false,
      success_ordering: success_ordering, failure_ordering: failure_ordering,
      ssid: ssid, ib: ib, ie: None
    }
  }

  pub fn new_insert_at_end(ptr: Box<dyn Value>, cmp: Box<dyn Value>,
    new_val: Box<dyn Value>, align: Align, success_ordering: AtomicOrdering,
    failure_ordering: AtomicOrdering, ssid: SyncScopeID, ie: Option<BasicBlock>) -> Self
  {
    AtomicCmpXchgInst {
      ptr: ptr, cmp: cmp, new_val: new_val, align: align,volatile: false, weak: false,
      success_ordering: success_ordering, failure_ordering: failure_ordering,
      ssid: ssid, ib: None, ie: ie
    }
  }

  // Return the alignment of the memory that is being allocated by the instruction.
  pub fn get_align(&self) -> Align {
    self.align
  }

  pub fn set_alignment(&mut self, align: Align) {
    self.align = align;
  }

  // Return true if this is a cmpxchg from a volatile memory location.
  pub fn is_volatile(&self) -> bool {
    self.volatile
  }

  // Specify whether this is a volatile cmpxchg.
  pub fn set_volatile(&mut self, volatile: bool) {
    self.volatile = volatile;
  }

  // Return true if this cmpxchg may spuriously fail.
  pub fn is_weak(&self) -> bool {
    self.weak
  }

  pub fn set_weak(&mut self, weak: bool) {
    self.weak = weak;
  }

  pub fn get_operand(&self, i: u32) -> Option<&Box<dyn Value>> {
    if i == 0 {
      return Some(&self.ptr);
    } else if i == 1 {
      return Some(&self.cmp);
    } else if i == 2 {
      return Some(&self.new_val);
    }
    None
  }

  pub fn set_operand(&mut self, i: u32, v: Box<dyn Value>) {
    if i == 0 {
      self.ptr = v;
    } else if i == 1 {
      self.cmp = v;
    } else if i == 2 {
      self.new_val = v;
    }
  }

  pub fn is_valid_success_ordering(ordering: AtomicOrdering) -> bool {
    ordering != AtomicOrdering::NotAtomic && ordering != AtomicOrdering::Unordered
  }

  pub fn is_valid_failure_ordering(ordering: AtomicOrdering) -> bool {
    ordering != AtomicOrdering::NotAtomic && ordering != AtomicOrdering::Unordered &&
    ordering != AtomicOrdering::AcquireRelease && ordering != AtomicOrdering::Release
  }

  // Returns the success ordering constraint of this of this cmpxchg instruction.
  pub fn get_success_ordering(&self) -> &AtomicOrdering {
    &self.success_ordering
  }

  // Sets the success ordering constraint of this cmpxchg instruction.
  pub fn set_success_ordering(&mut self, ordering: AtomicOrdering) {
    debug_assert!(AtomicCmpXchgInst::is_valid_success_ordering(ordering),
      "Invalid CmpXchg success ordering.");
    self.success_ordering = ordering;
  }

  // Returns the failure ordering constraint of this of this cmpxchg instruction.
  pub fn get_failure_ordering(&self) -> &AtomicOrdering {
    &self.failure_ordering
  }

  // Sets the failure ordering constraint of this cmpxvhg instruction.
  pub fn set_failure_ordering(&mut self, ordering: AtomicOrdering) {
    debug_assert!(AtomicCmpXchgInst::is_valid_failure_ordering(ordering),
      "Invalid CmpXchg failure ordering.");
    self.failure_ordering = ordering;
  }

  // Returns a single ordering which is at least as strong as both the success
  // and failure orderings for this cmpxchg.
  pub fn get_merged_ordering(&self) -> &AtomicOrdering {
    if self.get_failure_ordering() == &AtomicOrdering::SequentiallyConsistent {
      return &AtomicOrdering::SequentiallyConsistent;
    } else if self.get_failure_ordering() == &AtomicOrdering::Acquire {
      if self.get_success_ordering() == &AtomicOrdering::Monotonic {
        return &AtomicOrdering::Acquire;
      }
      if self.get_success_ordering() == &AtomicOrdering::Release {
        return &AtomicOrdering::AcquireRelease;
      }
    }
    self.get_success_ordering()
  }

  // Returns the synchronization scope ID of this cmpxchg instruction.
  pub fn get_sync_scope_id(&self) -> SyncScopeID {
    self.ssid
  }

  // Sets the synchronization scope ID of this cmpxchg instruction.
  pub fn set_sync_scope_id(&mut self, ssid: SyncScopeID) {
    self.ssid = ssid;
  }

  pub fn get_pointer_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 0 }

  pub fn get_compare_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(1)
  }

  pub fn get_new_val_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(2)
  }

  pub fn get_pointer_address_space() {}

  // Returns the strongest permitted ordering on failure, given the desired
  // ordering on success.
  pub fn get_strongest_failure_ordering(success_ordering: AtomicOrdering)
  -> AtomicOrdering
  {
    match success_ordering {
      AtomicOrdering::Release => return AtomicOrdering::Monotonic,
      AtomicOrdering::Monotonic => return AtomicOrdering::Monotonic,
      AtomicOrdering::AcquireRelease => return AtomicOrdering::Acquire,
      AtomicOrdering::Acquire => return AtomicOrdering::Acquire,
      AtomicOrdering::SequentiallyConsistent => return AtomicOrdering::SequentiallyConsistent,
      _ => panic!("Invalid cmpxchg success ordering.")
    };
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::AtomicCmpXchg
  }
}

impl Instruction for AtomicCmpXchgInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

#[derive(Debug, Clone, PartialEq)]
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

// An instruction that atomically reads a memory location, combines
// it with another value, and then stores the result back.
// Returns the old value.
#[derive(Debug)]
pub struct AtomicRMWInst {
  operation: BinOp,
  ptr: Box<dyn Value>,
  val: Box<dyn Value>,
  align: Align,
  volatile: bool,
  ordering: AtomicOrdering,
  ssid: SyncScopeID,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl AtomicRMWInst {
  pub fn new_insert_before(operation: BinOp, ptr: Box<dyn Value>,
    val: Box<dyn Value>, align: Align, ordering: AtomicOrdering, ssid: SyncScopeID,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    AtomicRMWInst {
      operation: operation, ptr: ptr, val: val, align: align, volatile: false,
      ordering: ordering, ssid: ssid, insert_before: ib, insert_at_end: None
    }
  }

  pub fn new_insert_at_end(operation: BinOp, ptr: Box<dyn Value>,
    val: Box<dyn Value>, align: Align, ordering: AtomicOrdering, ssid: SyncScopeID,
    ie: Option<BasicBlock>) -> Self
  {
    AtomicRMWInst {
      operation: operation, ptr: ptr, val: val, align: align, volatile: false,
      ordering: ordering, ssid: ssid, insert_before: None, insert_at_end: ie
    }
  }

  pub fn get_operation(&self) -> BinOp {
    self.operation
  }

  pub fn get_operation_name(&self, _op: BinOp) {}

  pub fn is_fp_operation(op: BinOp) -> bool {
    match op {
      BinOp::FAdd => return true,
      BinOp::FSub => return true,
      BinOp::FMax => return true,
      BinOp::FMin => return true,
      _ => return false
    };
  }

  pub fn set_operation(&mut self, operation: BinOp) {
    self.operation = operation;
  }

  // Return the alignment of the memory that is being allocated by the
  // instruction.
  pub fn get_align(&self) -> Align {
    self.align
  }

  pub fn set_alignment(&mut self, align: Align) {
    self.align = align;
  }

  // Return true if this is a RMW on a volatile memory location
  pub fn is_volatile(&self) -> bool {
    self.volatile
  }

  // Specify whether this is a volatile RMW or not.
  pub fn set_volatile(&mut self, volatile: bool) {
    self.volatile = volatile;
  }

  pub fn get_operand(&self, i: u32) -> Option<&Box<dyn Value>> {
    if i == 0 {
      return Some(&self.ptr);
    } else if i == 1 {
      return Some(&self.val);
    }
    None
  }

  pub fn set_operand(&mut self, i: u32, v: Box<dyn Value>) {
    if i == 0 {
      self.ptr = v;
    } else if i == 1 {
      self.val = v;
    }
  }

  // Returns the ordering constarint of this rmw instrunction.
  pub fn get_ordering(&self) -> &AtomicOrdering {
    &self.ordering
  }

  // Sets the ordering constraint of this rmw instruction.
  pub fn set_ordering(&mut self, ordering: AtomicOrdering) {
    debug_assert!(ordering != AtomicOrdering::NotAtomic,
      "Atomic instructions can only be atomic.");
    debug_assert!(ordering != AtomicOrdering::Unordered,
      "Atomic instructions can not be unordered.");
    self.ordering = ordering;
  }

  // Returns the synchronization scope ID of this rmw instruction.
  pub fn get_sync_scope_id(&self) -> SyncScopeID {
    self.ssid
  }

  // Sets the synchronization scope ID of this rmw instruction.
  pub fn set_sync_scope_id(&mut self, ssid: SyncScopeID) {
    self.ssid = ssid;
  }

  pub fn get_pointer_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 0 }

  pub fn get_val_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(1)
  }

  pub fn get_pointer_address_space() {}

  pub fn is_floating_pointer_operation(&self) -> bool {
    AtomicRMWInst::is_fp_operation(self.get_operation())
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::AtomicRMW
  }
}

impl Instruction for AtomicRMWInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// An instruction for type-safe pointer arithmetic to access elements
// of arrays and structs.
#[derive(Debug)]
pub struct GetElementPtrInst {
  pointee_type: Box<dyn Type>,
  ptr: Box<dyn Value>,
  idx_list: Vec<Box<dyn Value>>,
  values: u32,
  name: Twine,
  src_elt_type: Option<Box<dyn Type>>,
  result_elt_type: Option<Box<dyn Type>>,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl GetElementPtrInst {
  pub fn new_insert_before(pointee_type: Box<dyn Type>, ptr: Box<dyn Value>,
    idx_list: Vec<Box<dyn Value>>, name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    GetElementPtrInst {
      pointee_type: pointee_type, ptr: ptr, idx_list: idx_list,
      values: idx_list.len() as u32 + 1, name: name,
      src_elt_type: None, result_elt_type: None, insert_before: ib, insert_at_end: None
    }
  }

  pub fn new_insert_at_end(pointee_type: Box<dyn Type>, ptr: Box<dyn Value>,
    idx_list: Vec<Box<dyn Value>>, name: Twine, ie: Option<BasicBlock>) -> Self
  {
    GetElementPtrInst {
      pointee_type: pointee_type, ptr: ptr, idx_list: idx_list,
      values: idx_list.len() as u32 + 1, name: name,
      src_elt_type: None, result_elt_type: None, insert_before: None, insert_at_end: ie
    }
  }

  pub fn create_in_bounds() {}

  pub fn get_operand(&self, i: u32) -> Option<&Box<dyn Value>> {
    if i == 0 { return Some(&self.ptr); }
    None
  }

  pub fn set_operand(&mut self, i: u32, v: Box<dyn Value>) {
    if i == 0 { self.ptr = v; }
  }

  pub fn get_num_operands(&self) -> u32 { 1 }

  pub fn get_source_element_type(&self) -> &Option<Box<dyn Type>> {
    &self.src_elt_type
  }

  pub fn set_source_element_type(&mut self, src_elt_type: Option<Box<dyn Type>>) {
    self.src_elt_type = src_elt_type;
  }

  pub fn get_result_element_type(&self) -> &Option<Box<dyn Type>> {
    &self.result_elt_type
  }

  pub fn set_result_element_type(&mut self, result_elt_type: Option<Box<dyn Type>>) {
    self.result_elt_type = result_elt_type;
  }

  pub fn get_address_space() {}
  pub fn get_pointer_address_space() {}
  pub fn get_indexed_type() {}
  pub fn get_type_at_index() {}

  pub fn get_pointer_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 0 }

  // Method to return the pointer operand as a PointerType.
  pub fn get_pointer_operand_type(&self) -> &dyn Type {
    self.get_pointer_operand().unwrap().get_type()
  }

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

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::GetElementPtr
  }
}

impl Instruction for GetElementPtrInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// This instruction compares its operands according to the predicate
// given to the constructor. It only operates on integers or pointers.
// The operands must be identical types.
// Represent an integer comparison operator.
#[derive(Debug)]
pub struct ICmpInst {
  pred: Predicate,
  lhs: Box<dyn Value>,
  rhs: Box<dyn Value>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl ICmpInst {
  pub fn new_insert_before(pred: Predicate, lhs: Box<dyn Value>,
    rhs: Box<dyn Value>, name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    ICmpInst { pred: pred, lhs: lhs, rhs: rhs, name: name,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(pred: Predicate, lhs: Box<dyn Value>,
    rhs: Box<dyn Value>, name: Twine, ie: Option<BasicBlock>) -> Self
  {
    ICmpInst { pred: pred, lhs: lhs, rhs: rhs, name: name,
      insert_before: None, insert_at_end: ie }
  }

  // Return true if the predicate is SGT or UGT.
  pub fn is_gt(p: &Predicate) -> bool {
    *p == Predicate::ICmpSgt || *p == Predicate::ICmpUgt
  }

  // Return true if the predicate is SLT or ULT.
  pub fn is_lt(p: &Predicate) -> bool {
    *p == Predicate::ICmpSlt || *p == Predicate::ICmpUlt
  }

  // Return true if the predicate is SGE or UGE.
  pub fn is_ge(p: &Predicate) -> bool {
    *p == Predicate::ICmpSge || *p == Predicate::ICmpUge
  }

  // Return true if the predicate is SLE or ULE.
  pub fn is_le(p: &Predicate) -> bool {
    *p == Predicate::ICmpSle || *p == Predicate::ICmpUle
  }

  pub fn predicates() {}

  // Exchange the two operands to this instruction in such a way that it
  // does not modify the semantics of the instruction.
  // The predicate value may be changed to retain the same result if the
  // predicate is order dependent(e.g. ult).
  pub fn swap_operands(&mut self) {
    self.set_predicate(*self.get_swapped_predicate());
    let temp = self.lhs;
    self.lhs = self.rhs;
    self.rhs = temp;
  }

  // Return result of 'lhs pred rhs' comparison.
  pub fn compare(lhs: &APInt, rhs: &APInt, pred: Predicate) -> bool {
    match pred {
      Predicate::ICmpEq => return lhs.eq(rhs),
      Predicate::ICmpNe => return lhs.ne(rhs),
      // TODO
      //Predicate::ICmpUgt => return lhs.ugt(rhs),
      //Predicate::ICmpUge => return lhs.uge(rhs),
      _ => panic!("Unexpected non-integer predicate.")
    }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::ICmp
    // TODO
  }

  fn assert_ok(&self) {
    debug_assert!(self.is_int_predicate(), "Invalid ICmp predicate value.");
  }
}

impl Instruction for ICmpInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl CmpInst for ICmpInst {
  fn get_predicate(&self) -> &Predicate {
    &self.pred
  }

  fn set_predicate(&mut self, p: Predicate) {
    self.pred = p;
  }

  fn is_equality(&self) -> bool {
    self.get_predicate() == &Predicate::ICmpEq ||
    self.get_predicate() == &Predicate::ICmpNe
  }

  fn get_signed_predicate(&self) -> &Predicate {
    match *self.get_predicate() {
      Predicate::ICmpEq => return self.get_predicate(),
      Predicate::ICmpNe => return self.get_predicate(),
      Predicate::ICmpSgt => return self.get_predicate(),
      Predicate::ICmpSlt => return self.get_predicate(),
      Predicate::ICmpSge => return self.get_predicate(),
      Predicate::ICmpSle => return self.get_predicate(),
      Predicate::ICmpUgt => return &Predicate::ICmpSgt,
      Predicate::ICmpUlt => return &Predicate::ICmpSlt,
      Predicate::ICmpUge => return &Predicate::ICmpSge,
      Predicate::ICmpUle => return &Predicate::ICmpSle,
      _ => panic!("Unknown icmp predicate.")
    }
  }

  fn get_unsigned_predicate(&self) -> &Predicate {
    match *self.get_predicate() {
      Predicate::ICmpEq => return self.get_predicate(),
      Predicate::ICmpNe => return self.get_predicate(),
      Predicate::ICmpUgt => return self.get_predicate(),
      Predicate::ICmpUlt => return self.get_predicate(),
      Predicate::ICmpUge => return self.get_predicate(),
      Predicate::ICmpUle => return self.get_predicate(),
      Predicate::ICmpSgt => return &Predicate::ICmpUgt,
      Predicate::ICmpSlt => return &Predicate::ICmpUlt,
      Predicate::ICmpSge => return &Predicate::ICmpUge,
      Predicate::ICmpSle => return &Predicate::ICmpUle,
      _ => panic!("Unknown icmp predicate.")
    }
  }

  fn get_swapped_predicate(&self) -> &Predicate {
    match *self.get_predicate() {
      Predicate::ICmpEq => return self.get_predicate(),
      Predicate::ICmpNe => return self.get_predicate(),
      Predicate::ICmpSgt => return &Predicate::ICmpSlt,
      Predicate::ICmpSlt => return &Predicate::ICmpSgt,
      Predicate::ICmpSge => return &Predicate::ICmpSle,
      Predicate::ICmpSle => return &Predicate::ICmpSge,
      Predicate::ICmpUgt => return &Predicate::ICmpUlt,
      Predicate::ICmpUlt => return &Predicate::ICmpUgt,
      Predicate::ICmpUge => return &Predicate::ICmpUle,
      Predicate::ICmpUle => return &Predicate::ICmpUge,
      _ => panic!("Unknown icmp predicate.")
    }
  }
}

// This instruction compares its operands according to the predicate given
// to the constructor. It only operates on floating point values or packedvectors
// of floating point values. The operands must be identical types. Represents
// a floating point comparison operator.
#[derive(Debug)]
pub struct FCmpInst {
  pred: Predicate,
  lhs: Box<dyn Value>,
  rhs: Box<dyn Value>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl FCmpInst {
  pub fn new_insert_before(pred: Predicate, lhs: Box<dyn Value>,
    rhs: Box<dyn Value>, name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    FCmpInst { pred: pred, lhs: lhs, rhs: rhs, name: name,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(pred: Predicate, lhs: Box<dyn Value>,
    rhs: Box<dyn Value>, name: Twine, ie: Option<BasicBlock>) -> Self
  {
    FCmpInst { pred: pred, lhs: lhs, rhs: rhs, name: name,
      insert_before: None, insert_at_end: ie }
  }

  pub fn swap_operands(&mut self) {
    self.set_predicate(*self.get_swapped_predicate());
    let temp = self.lhs;
    self.lhs = self.rhs;
    self.rhs = temp;
  }

  pub fn predicates() {}
  pub fn compare() {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::FCmp
  }

  fn assert_ok() {}
}

impl Instruction for FCmpInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl CmpInst for FCmpInst {
  fn get_predicate(&self) -> &Predicate {
    &self.pred
  }

  fn set_predicate(&mut self, p: Predicate) {
    self.pred = p;
  }

  fn is_equality(&self) -> bool {
    let p = *self.get_predicate();
    p == Predicate::FCmpOeq || p == Predicate::FCmpOne ||
    p == Predicate::FCmpUeq || p == Predicate::FCmpUne
  }

  fn is_commutative(&self) -> bool {
    let p = *self.get_predicate();
    self.is_equality() ||
    p == Predicate::FCmpFalse || p == Predicate::FCmpTrue ||
    p == Predicate::FCmpOrd || p == Predicate::FCmpUno
  }

  fn get_swapped_predicate(&self) -> &Predicate {
    let p = self.get_predicate();
    match *p {
      Predicate::FCmpFalse => return p,
      Predicate::FCmpTrue => return p,
      Predicate::FCmpOeq => return p,
      Predicate::FCmpOne => return p,
      Predicate::FCmpUeq => return p,
      Predicate::FCmpUne => return p,
      Predicate::FCmpOrd => return p,
      Predicate::FCmpUno => return p,
      Predicate::FCmpOgt => return &Predicate::FCmpOlt,
      Predicate::FCmpOlt => return &Predicate::FCmpOgt,
      Predicate::FCmpOge => return &Predicate::FCmpOle,
      Predicate::FCmpOle => return &Predicate::FCmpOge,
      Predicate::FCmpUgt => return &Predicate::FCmpUlt,
      Predicate::FCmpUlt => return &Predicate::FCmpUgt,
      Predicate::FCmpUge => return &Predicate::FCmpUle,
      Predicate::FCmpUle => return &Predicate::FCmpUge,
      _ => panic!("Unknown cmp predicate.")
    }
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

  pub fn class_of(i: InstructionBase) -> bool {
    i.get_op_code() == OpCode::Call
  }

  pub fn update_prof_weight() {}
}

// This class represents the Blitz 'select' instruction.
#[derive(Debug)]
pub struct SelectInst {
  c: Box<dyn Value>,
  s1: Box<dyn Value>,
  s2: Box<dyn Value>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl SelectInst {
  pub fn new_insert_before(c: Box<dyn Value>, s1: Box<dyn Value>,
    s2: Box<dyn Value>, name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    SelectInst { c: c, s1: s1, s2: s2, name: name,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(c: Box<dyn Value>, s1: Box<dyn Value>,
    s2: Box<dyn Value>, name: Twine, ie: Option<BasicBlock>) -> Self
  {
    SelectInst { c: c, s1: s1, s2: s2, name: name,
      insert_before: None, insert_at_end: ie }
  }

  pub fn get_condition(&self) -> &Box<dyn Value> { &self.c }
  pub fn get_true_value(&self) -> &Box<dyn Value> { &self.s1 }
  pub fn get_false_value(&self) -> &Box<dyn Value> { &self.s2 }

  pub fn set_condition(&mut self, c: Box<dyn Value>) { self.c = c; }
  pub fn set_true_value(&mut self, s1: Box<dyn Value>) { self.s1 = s1; }
  pub fn set_false_value(&mut self, s2: Box<dyn Value>) { self.s2 = s2; }

  // Swap the true and false values of the select instruction.
  // This doesn't swap prof metadata.
  pub fn swap_values(&mut self) {
    let temp = self.s1;
    self.s1 = self.s2;
    self.s2 = temp;
  }

  pub fn are_invalid_operands() {}

  pub fn get_operand(&self, i: u32) -> Option<&Box<dyn Value>> {
    if i == 0 { return Some(&self.c); }
    else if i == 1 { return Some(&self.s1); }
    else if i == 2 { return Some(&self.s2); }
    None
  }

  pub fn set_operand(&mut self, i: u32, v: Box<dyn Value>) {
    if i == 0 { self.c = v; }
    else if i == 1 { self.s1 = v; }
    else if i == 2 { self.s2 = v; }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Select
  }
}

impl Instruction for SelectInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// THis class represents the va_arg Blitz instruction, which returns
// an argument of the specified type given a va_list and increments that list.
#[derive(Debug)]
pub struct VAArgInst {
  list: Box<dyn Value>,
  v_type: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl VAArgInst {
  pub fn new_insert_before(list: Box<dyn Value>, t: Box<dyn Type>, name: Twine,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    VAArgInst { list: list, v_type: t, name: name, insert_before: ib,
      insert_at_end: None }
  }

  pub fn new_insert_at_end(list: Box<dyn Value>, t: Box<dyn Type>, name: Twine,
    ie: Option<BasicBlock>) -> Self
  {
    VAArgInst { list: list, v_type: t, name: name, insert_before: None,
      insert_at_end: ie }
  }

  pub fn get_pointer_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_pointer_operand_index(&self) -> u32 { 0 }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::VAArg
  }
}

impl Instruction for VAArgInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl UnaryInstruction for VAArgInst {
  fn get_operand(&self, i: u32) -> Option<&Box<dyn Value>> {
    if i == 0 { return Some(&self.list); }
    None
  }
}

// This instruction extracts a single (scalar) element from VectorTypr value.
#[derive(Debug)]
pub struct ExtractElementInst {
  vec: Box<dyn Value>,
  idx: Box<dyn Value>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl ExtractElementInst {
  pub fn new_insert_before(vec: Box<dyn Value>, idx: Box<dyn Value>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    ExtractElementInst { vec: vec, idx: idx, name: name,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(vec: Box<dyn Value>, idx: Box<dyn Value>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    ExtractElementInst { vec: vec, idx: idx, name: name,
      insert_before: None, insert_at_end: ie }
  }

  pub fn is_valid_operands() {}

  pub fn get_vector_operand(&self) -> &Box<dyn Value> {
    &self.vec
  }

  pub fn get_index_operand(&self) -> &Box<dyn Value>{
    &self.idx
  }

  pub fn get_vector_operand_type(&self) -> Option<&dyn VectorType>{
    //self.get_vector_operand().get_type().as_any().downcast_ref::<dyn VectorType>()
    None
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::ExtractElement
  }
}

impl Instruction for ExtractElementInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// This instruction inserts a single (scalar) element into a VectorType value.
#[derive(Debug)]
pub struct InsertElementInst {
  vec: Box<dyn Value>,
  new_elt: Box<dyn Value>,
  idx: Box<dyn Value>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl InsertElementInst {
  pub fn new_insert_before(vec: Box<dyn Value>, new_elt: Box<dyn Value>,
    idx: Box<dyn Value>, name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    InsertElementInst { vec: vec, new_elt: new_elt, idx: idx, name: name,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(vec: Box<dyn Value>, new_elt: Box<dyn Value>,
    idx: Box<dyn Value>, name: Twine, ie: Option<BasicBlock>) -> Self
  {
    InsertElementInst { vec: vec, new_elt: new_elt, idx: idx, name: name,
      insert_before: None, insert_at_end: ie }
  }

  pub fn is_valid_operands() {}

  pub fn get_vector_type() {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::InsertElement
  }
}

impl Instruction for InsertElementInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// This instruction constructs a fixed permutation of two input vectors.
// For each element of the result vector, the shuffle mask selects an
// element from one of the input vectors to copy to the result.
#[derive(Debug)]
pub struct ShuffleVectorInst {
  v1: Box<dyn Value>,
  mask: Box<dyn Value>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl ShuffleVectorInst {
  pub fn new_insert_before(v1: Box<dyn Value>, mask: Box<dyn Value>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    ShuffleVectorInst { v1: v1, mask: mask, name: name,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(v1: Box<dyn Value>, mask: Box<dyn Value>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    ShuffleVectorInst { v1: v1, mask: mask, name: name,
      insert_before: None, insert_at_end: ie }
  }

  pub fn compute() {}
  pub fn is_valid_operands() {}
  pub fn get_vector_type() {}
  pub fn get_mask_value() {}
  pub fn get_shuffle_mask() {}
  pub fn get_shuffle_mask_for_bitcode() {}
  pub fn convert_shuffle_mask_for_bitcode() {}
  pub fn set_shuffle_mask() {}
  pub fn changes_length() {}
  pub fn increases_length() {}
  pub fn is_single_source_mask() {}
  pub fn is_single_source() {}
  pub fn is_identity_mask() {}
  pub fn is_identity() {}
  pub fn is_identity_with_padding() {}
  pub fn is_identity_with_extract() {}
  pub fn is_concat() {}
  pub fn is_select_mask() {}
  pub fn is_select() {}
  pub fn is_reverse_mask() {}
  pub fn is_reverse() {}
  pub fn is_zero_elt_splat_mask() {}
  pub fn is_zero_elt_splat() {}
  pub fn is_transpose_mask() {}
  pub fn is_transpose() {}
  pub fn is_splice_mask() {}
  pub fn is_splice() {}
  pub fn is_extract_subvector_mask() {}
  pub fn is_insert_subvector_mask() {}
  pub fn is_replication_mask() {}
  pub fn is_one_use_single_source_mask() {}
  pub fn compute_shuffle_mask() {}
  pub fn is_interleave() {}
  pub fn is_interleave_mask() {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::ShuffleVector
  }
}

impl Instruction for ShuffleVectorInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// This instruction extracts a struct member or array element value
// from an aggregate value.
#[derive(Debug)]
pub struct ExtractValueInst {
  agg: Box<dyn Value>,
  idxs: Vec<u32>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl ExtractValueInst {
  pub fn new_insert_before(agg: Box<dyn Value>, idxs: Vec<u32>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    ExtractValueInst { agg: agg, idxs: idxs, name: name,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(agg: Box<dyn Value>, idxs: Vec<u32>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    ExtractValueInst { agg: agg, idxs: idxs, name: name,
      insert_before: None, insert_at_end: ie }
  }

  pub fn get_indexed_type() {}

  pub fn get_aggregate_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_aggregate_operand_index(&self) -> u32 { 0 }

  pub fn get_indices(&self) ->& Vec<u32> {
    &self.idxs
  }

  pub fn get_num_indices(&self) -> usize {
    self.idxs.len()
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::ExtractValue
  }
}

impl Instruction for ExtractValueInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl UnaryInstruction for ExtractValueInst {
  fn get_operand(&self, i: u32) -> Option<&Box<dyn Value>> {
    if i == 0 { return Some(&self.agg); }
    None
  }
}

// This instruction inserts a stract field of array element value
// into an aggregate value.
#[derive(Debug)]
pub struct InsertValueInst {
  agg: Box<dyn Value>,
  val: Box<dyn Value>,
  idxs: Vec<u32>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl InsertValueInst {
  pub fn new_insert_before(agg: Box<dyn Value>, val: Box<dyn Value>,
    idxs: Vec<u32>, name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    InsertValueInst { agg: agg, val: val, idxs: idxs, name: name,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(agg: Box<dyn Value>, val: Box<dyn Value>,
    idxs: Vec<u32>, name: Twine, ie: Option<BasicBlock>) -> Self
  {
    InsertValueInst { agg: agg, val: val, idxs: idxs, name: name,
      insert_before: None, insert_at_end: ie }
  }

  pub fn get_operand(&self, i: u32) -> Option<&Box<dyn Value>> {
    if i == 0 { return Some(&self.agg); }
    else if i == 1 { return Some(&self.val); }
    None
  }

  pub fn set_operand(&mut self, i: u32, v: Box<dyn Value>) {
    if i == 0 { self.agg = v; }
    else if i == 1 { self.val = v; }
  }

  pub fn get_aggregate_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(0)
  }

  pub fn get_aggregate_operand_index(&self) -> u32 { 0 }

  pub fn get_inserted_value_operand(&self) -> Option<&Box<dyn Value>> {
    self.get_operand(1)    
  }

  pub fn get_inserted_value_operand_index(&self) -> u32 { 1 }

  pub fn get_indices(&self) -> &Vec<u32> {
    &self.idxs
  }

  pub fn get_num_indices(&self) -> usize {
    self.idxs.len()
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::InsertValue
  }
}

impl Instruction for InsertValueInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// The PhiNode class is used to represent the magical mystical Phi node,
// that can not exist in nature, but can be synthesized in a computer
// scientist's overactive imagination.
#[derive(Debug)]
pub struct PhiNode {
  v_type: Box<dyn Type>,
  reserved_space: u32,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl PhiNode {
  pub fn new_insert_before(t: Box<dyn Type>, reserved_space: u32, name: Twine,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    debug_assert!(!t.is_token_type(), "Phi nodes cannot have token type.");
    let phi = PhiNode { v_type: t, reserved_space: reserved_space,
      name: name, insert_before: ib, insert_at_end: None };
    phi.alloc_hungoff_uses(phi.reserved_space);
    phi
  }

  pub fn new_insert_at_end(t: Box<dyn Type>, reserved_space: u32, name: Twine,
    ie: Option<BasicBlock>) -> Self
  {
    debug_assert!(!t.is_token_type(), "Phi nodes cannot have token type.");
    let phi = PhiNode { v_type: t, reserved_space: reserved_space,
      name: name, insert_before: None, insert_at_end: ie };
    phi.alloc_hungoff_uses(phi.reserved_space);
    phi
  }

  pub fn get_operand(&self, i: u32) -> Option<&Box<dyn Value>> {
    //if i == 0 { return Some(&self.); }
    None
  }

  pub fn set_operand() {}
  pub fn get_num_operands(&self) -> u32 { 0 }

  pub fn get_num_incoming_values(&self) {}
  pub fn has_constant_value(&self) {}
  pub fn has_constant_or_undef_value(&self) -> bool { false }
  pub fn is_complete(&self) -> bool { false }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Phi
  }

  fn alloc_hungoff_uses(&self, _n: u32) {}
}

impl Instruction for PhiNode {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// The landingpad instruction holds all of the information necessary to
// generate correct exception handling.
// The landingpad instruction cannot be moved from the top of a landing
// pad block, which itself is accessible only from the 'unwind' edge of
// an invoke. This uses the sub_class_data field in Value to store whether
// or not the landingpad is a cleanup.
#[derive(Debug)]
pub struct LandingPadInst {
  ret_t: Box<dyn Type>,
  reserved_space: u32,
  name: Twine,
  cleanup: bool,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl LandingPadInst {
  pub fn new_insert_before(ret_t: Box<dyn Type>, reserved_space: u32,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    LandingPadInst { ret_t: ret_t, reserved_space: reserved_space,
      name: name, cleanup: false, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(ret_t: Box<dyn Type>, reserved_space: u32,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    LandingPadInst { ret_t: ret_t, reserved_space: reserved_space,
      name: name, cleanup: false, insert_before: None, insert_at_end: ie }
  }

  // Return true if this landingpad instruction is a cleanup.
  // I.e. it should be run when unwinding even if its landing pad doesn't
  // catch the exception.
  pub fn is_cleanup(&self) -> bool {
    self.cleanup
  }

  // Indicate that this landingpad instruction is a cleanup.
  pub fn set_cleanup(&mut self, cleanup: bool) {
    self.cleanup = cleanup;
  }

  // Add a catch or filter clause to the landing pad.
  pub fn add_clause(&self, _clause_val: Box<dyn Constant>) {}

  // Get the value of the clause at index. Use is_catch/is_filter to
  // determine what type of clause this is.
  pub fn get_clause(&self) {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::LandingPad
  }

  fn grow_operands() {}
}

impl Instruction for LandingPadInst {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// Return a value (possibly void), from a function.
// Exception does not continue in this function any longer.
struct ReturnInst {
  inst: InstructionBase,
  retval: Option<Box<dyn Value>>
}

impl ReturnInst {
  pub fn new_ib(c: &mut BlitzContext, retval: Option<Box<dyn Value>>,
    ib: Option<Box<InstructionBase>>) -> Self
  {
    ReturnInst {
      inst: InstructionBase::new_ib(Box::new(type_::get_void_type(c)),
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

  pub fn class_of(i: InstructionBase) -> bool {
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