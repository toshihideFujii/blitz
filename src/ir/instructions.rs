#![allow(dead_code)]

// This file exposes the class definitions of all of the subclasses
// of the Instruction class. This is meant to be an easy way to get
// access to allinstruction subclasses.

use std::any::Any;
use crate::{support::{atomic_ordering::AtomicOrdering, alignment::Align},
  adt::{twine::Twine, ap_int::APInt}};
use super::{instr_types::{UnaryInstruction, CmpInst, Predicate, //CallBase,
  OperandBundleDef, CallBase, FuncletPadInst, CastInst},
  type_::{Type, PointerType, VectorType, ArrayType, FunctionType, UnknownType, /*FunctionType*/},
  instruction::{OpCode, Instruction}, value::{Value, ValueType},
  //attributes::{AttrKind, /*AttributeList*/},
  blits_context::{/*BlitzContext,*/ SyncScopeID}, basic_block::BasicBlock,
  constants::ConstantInt, constant::Constant,
  /*basic_block::BasicBlock*/
  user::User,
};

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
  pub fn get_address_space(&self) -> usize {
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

impl Value for AllocaInst {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}

impl User for AllocaInst {
  fn get_operand(&self, i: usize) -> Option<&Box<dyn Value>> {
    if i == 0 { return Some(&self.array_size); }
    None
  }
}
impl Instruction for AllocaInst {
  fn get_parent(&self) -> &Option<BasicBlock> {
    &self.parent
  }

  fn get_op_code(&self) -> OpCode {
    OpCode::Alloca
  }

  fn as_any_inst(&self) -> &dyn Any {
    self
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
  pub fn get_align(&self) -> &Option<Align> {
    &self.align
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

impl Value for LoadInst {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}

impl User for LoadInst {
  fn get_operand(&self, _i: usize) -> Option<&Box<dyn Value>> {
    Some(&self.operand)
  }
}

impl Instruction for LoadInst {
  fn get_op_code(&self) -> OpCode {
    OpCode::Load
  }

  fn is_atomic(&self) -> bool {
    return self.get_ordering() != &AtomicOrdering::NotAtomic
  }

  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

impl UnaryInstruction for LoadInst {}

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

  pub fn get_align(&self) -> &Option<Align> {
    &self.align
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

impl Value for StoreInst {
  fn get_type(&self) -> &dyn Type {
    self.val_operand.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}

impl User for StoreInst {}
impl Instruction for StoreInst {
  fn get_op_code(&self) -> OpCode {
    OpCode::Store
  }

  fn is_atomic(&self) -> bool {
    return self.get_ordering() != &AtomicOrdering::NotAtomic
  }

  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

// An instruction for ordering other memory operations.
#[derive(Debug)]
pub struct FenceInst {
  ordering: AtomicOrdering,
  ssid: SyncScopeID,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>,
  t: Box<dyn Type>
}

impl FenceInst {
  pub fn new_insert_before(ordering: AtomicOrdering, ssid: SyncScopeID,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    FenceInst { ordering: ordering, ssid: ssid, insert_before: ib,
      insert_at_end: None, t: Box::new(UnknownType::new()) }
  }

  pub fn new_insert_at_end(ordering: AtomicOrdering, ssid: SyncScopeID,
    ie: Option<BasicBlock>) -> Self
  {
    FenceInst { ordering: ordering, ssid: ssid, insert_before: None,
      insert_at_end: ie, t: Box::new(UnknownType::new()) }
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

impl Value for FenceInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}

impl User for FenceInst {}
impl Instruction for FenceInst {
  fn as_any_inst(&self) -> &dyn Any {
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
  pub fn get_align(&self) -> &Align {
    &self.align
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
    debug_assert!(AtomicCmpXchgInst::is_valid_success_ordering(ordering.clone()),
      "Invalid CmpXchg success ordering.");
    self.success_ordering = ordering;
  }

  // Returns the failure ordering constraint of this of this cmpxchg instruction.
  pub fn get_failure_ordering(&self) -> &AtomicOrdering {
    &self.failure_ordering
  }

  // Sets the failure ordering constraint of this cmpxvhg instruction.
  pub fn set_failure_ordering(&mut self, ordering: AtomicOrdering) {
    debug_assert!(AtomicCmpXchgInst::is_valid_failure_ordering(ordering.clone()),
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
  pub fn get_sync_scope_id(&self) -> &SyncScopeID {
    &self.ssid
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
impl Value for AtomicCmpXchgInst {
  fn get_type(&self) -> &dyn Type {
    self.ptr.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for AtomicCmpXchgInst {}
impl Instruction for AtomicCmpXchgInst {
  fn as_any_inst(&self) -> &dyn Any {
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

  pub fn get_operation(&self) -> &BinOp {
    &self.operation
  }

  pub fn get_operation_name(&self, _op: BinOp) {}

  pub fn is_fp_operation(op: &BinOp) -> bool {
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
  pub fn get_align(&self) -> &Align {
    &self.align
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
  pub fn get_sync_scope_id(&self) -> &SyncScopeID {
    &self.ssid
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
impl Value for AtomicRMWInst {
  fn get_type(&self) -> &dyn Type {
    self.val.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for AtomicRMWInst {}
impl Instruction for AtomicRMWInst {
  fn as_any_inst(&self) -> &dyn Any {
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
    let len = idx_list.len() as u32;
    GetElementPtrInst {
      pointee_type: pointee_type, ptr: ptr, idx_list: idx_list,
      values: len + 1, name: name, src_elt_type: None, result_elt_type: None,
      insert_before: ib, insert_at_end: None
    }
  }

  pub fn new_insert_at_end(pointee_type: Box<dyn Type>, ptr: Box<dyn Value>,
    idx_list: Vec<Box<dyn Value>>, name: Twine, ie: Option<BasicBlock>) -> Self
  {
    let len = idx_list.len() as u32;
    GetElementPtrInst {
      pointee_type: pointee_type, ptr: ptr, idx_list: idx_list,
      values: len + 1, name: name, src_elt_type: None, result_elt_type: None,
      insert_before: None, insert_at_end: ie
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
impl Value for GetElementPtrInst {
  fn get_type(&self) -> &dyn Type {
    self.pointee_type.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for GetElementPtrInst {}
impl Instruction for GetElementPtrInst {
  fn as_any_inst(&self) -> &dyn Any {
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
    //self.set_predicate(*self.get_swapped_predicate());
    //let temp = self.lhs;
    //self.lhs = self.rhs;
    //self.rhs = temp;
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
impl Value for ICmpInst {
  fn get_type(&self) -> &dyn Type {
    self.rhs.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for ICmpInst {}
impl Instruction for ICmpInst {
  fn as_any_inst(&self) -> &dyn Any {
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
    //self.set_predicate(*self.get_swapped_predicate());
    //let temp = self.lhs;
    //self.lhs = self.rhs;
    //self.rhs = temp;
  }

  pub fn predicates() {}
  pub fn compare() {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::FCmp
  }

  fn assert_ok() {}
}
impl Value for FCmpInst {
  fn get_type(&self) -> &dyn Type {
    self.rhs.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for FCmpInst {}
impl Instruction for FCmpInst {
  fn as_any_inst(&self) -> &dyn Any {
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
    let p = self.get_predicate();
    p == &Predicate::FCmpOeq || p == &Predicate::FCmpOne ||
    p == &Predicate::FCmpUeq || p == &Predicate::FCmpUne
  }

  fn is_commutative(&self) -> bool {
    let p = self.get_predicate();
    self.is_equality() ||
    p == &Predicate::FCmpFalse || p == &Predicate::FCmpTrue ||
    p == &Predicate::FCmpOrd || p == &Predicate::FCmpUno
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
pub trait CallInst: CallBase {
  fn compute_num_operands(&self) {}
  fn craete_malloc(&self) {}
  fn create_free(&self) {}

  fn get_tail_call_kind(&self) -> TailCallKind {
    TailCallKind::None
  }

  fn is_tail_call(&self) -> bool {
    let kind = self.get_tail_call_kind();
    kind == TailCallKind::Tail || kind == TailCallKind::MustTail
  }

  fn is_must_tail_call(&self) -> bool {
    self.get_tail_call_kind() == TailCallKind::MustTail
  }

  fn is_no_tail_call(&self) -> bool {
    self.get_tail_call_kind() == TailCallKind::NoTail
  }

  fn set_tail_call_kind(&self, _tck: TailCallKind) {}

  // Return true if the call can return twice.
  fn can_return_twice(&self) -> bool {
    //self.call_base.has_fn_attr(AttrKind::ReturnsTwice)
    false
  }

  fn set_can_return_twice(&mut self) {
    //self.call_base.add_fn_attr(AttrKind::ReturnsTwice)
  }

  fn update_prof_weight(&self) {}
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
    //let temp = self.s1;
    //self.s1 = self.s2;
    //self.s2 = temp;
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
impl Value for SelectInst {
  fn get_type(&self) -> &dyn Type {
    self.c.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for SelectInst {}
impl Instruction for SelectInst {
  fn as_any_inst(&self) -> &dyn Any {
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
impl Value for VAArgInst {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for VAArgInst {
  fn get_operand(&self, i: usize) -> Option<&Box<dyn Value>> {
    if i == 0 { return Some(&self.list); }
    None
  }
}
impl Instruction for VAArgInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
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
impl Value for ExtractElementInst {
  fn get_type(&self) -> &dyn Type {
    self.vec.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for ExtractElementInst {}
impl Instruction for ExtractElementInst {
  fn as_any_inst(&self) -> &dyn Any {
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
impl Value for InsertElementInst {
  fn get_type(&self) -> &dyn Type {
    self.vec.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for InsertElementInst {}
impl Instruction for InsertElementInst {
  fn as_any_inst(&self) -> &dyn Any {
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
impl Value for ShuffleVectorInst {
  fn get_type(&self) -> &dyn Type {
    self.v1.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for ShuffleVectorInst {}
impl Instruction for ShuffleVectorInst {
  fn as_any_inst(&self) -> &dyn Any {
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
impl Value for ExtractValueInst {
  fn get_type(&self) -> &dyn Type {
    self.agg.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for ExtractValueInst {
  fn get_operand(&self, i: usize) -> Option<&Box<dyn Value>> {
    if i == 0 { return Some(&self.agg); }
    None
  }
}
impl Instruction for ExtractValueInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

impl UnaryInstruction for ExtractValueInst {}

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
impl Value for InsertValueInst {
  fn get_type(&self) -> &dyn Type {
    self.val.get_type()
  }

  fn get_value_id(&self) -> ValueType {
      ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for InsertValueInst {}
impl Instruction for InsertValueInst {
  fn as_any_inst(&self) -> &dyn Any {
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

  pub fn get_operand(&self, _i: u32) -> Option<&Box<dyn Value>> {
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
impl Value for PhiNode {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for PhiNode {}
impl Instruction for PhiNode {
  fn as_any_inst(&self) -> &dyn Any {
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
  reserved_space: usize,
  name: Twine,
  cleanup: bool,
  clauses: Vec<Box<dyn Constant>>,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl LandingPadInst {
  pub fn new_insert_before(ret_t: Box<dyn Type>, reserved_space: usize,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    LandingPadInst {
      ret_t: ret_t, reserved_space: reserved_space,
      name: name, cleanup: false, clauses: Vec::new(), insert_before: ib,
      insert_at_end: None
    }
  }

  pub fn new_insert_at_end(ret_t: Box<dyn Type>, reserved_space: usize,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    LandingPadInst {
      ret_t: ret_t, reserved_space: reserved_space,
      name: name, cleanup: false, clauses: Vec::new(), insert_before: None,
      insert_at_end: ie
    }
  }

  pub fn get_num_operands(&self) -> usize {
    self.clauses.len()
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
  pub fn add_clause(&mut self, clause: Box<dyn Constant>) {
    //let op_no = self.get_num_operands();
    //self.grow_operands(1);
    //debug_assert!(op_no < self.reserved_space, "Crowing didn't work.");
    // set_num_hung_off_use_operands()
    self.clauses.push(clause);
  }

  // Get the value of the clause at index. Use is_catch/is_filter to
  // determine what type of clause this is.
  pub fn get_clause(&self, index: usize) -> &Box<dyn Constant> {
    &self.clauses[index]
  }

  // Return true if the clause and index is a catch clause.
  pub fn is_catch(&self, index: usize) -> bool {
    let t = &self.clauses[index].get_type();
    if t.as_any().downcast_ref::<ArrayType>().is_none() { return true; }
    false
  }

  // Return true if the clause and index is a filter clause.
  pub fn is_filter(&self, index: usize) -> bool {
    let t = &self.clauses[index].get_type();
    if t.as_any().downcast_ref::<ArrayType>().is_some() { return true; }
    false
  }

  // Get the number of clauses for this landing pad.
  pub fn get_num_clauses(&self) -> usize {
    self.clauses.len()
  }

  // Grow the size of the operand list to accommodate the new number of clauses.
  pub fn reserve_clauses(&mut self, size: usize) {
    self.clauses.reserve(size);
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::LandingPad
  }

  fn grow_operands(&mut self, _i: usize) {}
}

impl Value for LandingPadInst {
  fn get_type(&self) -> &dyn Type {
    self.ret_t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for LandingPadInst {}
impl Instruction for LandingPadInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

// Return a value (possibly void), from a function.
// Execution does not continue in this function any longer.
#[derive(Debug)]
struct ReturnInst {
  retval: Option<Box<dyn Value>>,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl ReturnInst {
  pub fn new_insert_before(retval: Option<Box<dyn Value>>,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    ReturnInst { retval: retval, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(retval: Option<Box<dyn Value>>,
    ie: Option<BasicBlock>) -> Self
  {
    ReturnInst { retval: retval, insert_before: None, insert_at_end: ie }
  }

  pub fn get_return_value(&self) -> Option<&Box<dyn Value>> {
    if self.get_num_operands() != 0 {
      return self.get_operand(0);
    }
    None
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Ret
  }
}

impl Value for ReturnInst {
  fn get_type(&self) -> &dyn Type {
    self.retval.as_ref().unwrap().get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}

impl User for ReturnInst {
  fn get_operand(&self, i: usize) -> Option<&Box<dyn Value>> {
    if i == 0  {
      let ret = self.retval.as_ref().unwrap();
      return Some(ret);
    }
    None
  }

  fn set_operand(&mut self, i: usize, v: Box<dyn Value>) {
    debug_assert!(i == 0, "Set value at index 0 only.");
    self.retval = Some(v);
  }

  fn get_num_operands(&self) -> usize {
    if self.retval.is_some() { return 1; }
    0
  }
}

impl Instruction for ReturnInst {
  fn get_num_successors(&self) -> usize { 0 }

  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

// Conditional or unconditional branch instruction.
#[derive(Debug)]
pub struct BranchInst {
  if_true: Box<dyn Value>,
  if_false: Option<Box<dyn Value>>,
  cond: Option<Box<dyn Value>>,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl BranchInst {
  pub fn new_insert_before(if_true: Box<BasicBlock>,
    if_false: Option<Box<BasicBlock>>, cond: Option<Box<dyn Value>>,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    let mut false_val: Option<Box<dyn Value>> = None;
    if if_false.is_some() { false_val = Some(if_false.unwrap()); }
    BranchInst { if_true: if_true, if_false: false_val, cond: cond,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(if_true: Box<BasicBlock>,
    if_false: Option<Box<BasicBlock>>, cond: Option<Box<dyn Value>>,
    ie: Option<BasicBlock>) -> Self
  {
    let mut false_val: Option<Box<dyn Value>> = None;
    if if_false.is_some() { false_val = Some(if_false.unwrap()); }
    BranchInst { if_true: if_true, if_false: false_val, cond: cond,
      insert_before: None, insert_at_end: ie }
  }

  pub fn is_unconditional(&self) -> bool {
    self.get_num_operands() == 1
  }

  pub fn is_conditional(&self) -> bool {
    self.get_num_operands() == 3
  }

  pub fn get_condition(&self) -> &Box<dyn Value> {
    debug_assert!(self.is_conditional(), "Cannot get condition of an uncond branch.");
    self.cond.as_ref().unwrap()
  }

  pub fn set_condition(&mut self, v: Box<dyn Value>) {
    debug_assert!(self.is_conditional(), "Cannot get condition of an uncond branch.");
    self.cond = Some(v);
  }

  pub fn swap_successors() {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Br
  }
}

impl Value for BranchInst {
  fn get_type(&self) -> &dyn Type {
    self.if_true.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}

impl User for BranchInst {
  fn get_operand(&self, i: usize) -> Option<&Box<dyn Value>> {
    if i == 0 && self.cond.is_some() {
      let cond = self.cond.as_ref().unwrap();
      return Some(cond);
    } else if i == 1 && self.if_false.is_some() {
      let if_false = self.if_false.as_ref().unwrap();
      return Some(if_false);
    } else if i == 2 {
      return Some(&self.if_true);
    }
    None
  }

  fn set_operand(&mut self, _i: usize, _v: Box<dyn Value>) {}

  fn get_num_operands(&self) -> usize {
    if self.if_false.is_none() && self.cond.is_none() {
      return 1;
    } else if self.if_false.is_some() && self.cond.is_none() {
      return 2;
    } else if self.if_false.is_some() && self.cond.is_some() {
      return 3;
    }
    panic!("Invalidate condition.");
  }
}

impl Instruction for BranchInst {
  fn get_successor(&self, i: usize) -> Option<&BasicBlock> {
    debug_assert!(i < self.get_num_successors(), "Successor # out of range for branch.");
    if i == 0 {
      return self.if_true.as_any().downcast_ref::<BasicBlock>();
    } else if i == 1 && self.if_false.is_some() {
      return self.if_false.as_ref().unwrap().as_any().downcast_ref::<BasicBlock>();
    }
    None
  }

  fn set_successor(&mut self, i: usize, bb: BasicBlock) {
    debug_assert!(i < self.get_num_successors(), "Successor # out of range for branch.");
    if i == 0 { self.if_true = Box::new(bb); }
    else if i == 1 { self.if_false = Some(Box::new(bb)); }
  }

  fn get_num_successors(&self) -> usize {
    if self.is_conditional() { return 2; } else { return 1; }
  }

  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

// Multiway switch
#[derive(Debug)]
pub struct SwitchInst {
  value: Box<dyn Value>,
  default: BasicBlock,
  num_cases: usize,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl SwitchInst {
  pub fn new_insert_before(value: Box<dyn Value>, default: BasicBlock,
    num_cases: usize, ib: Option<Box<dyn Instruction>>) -> Self
  {
    SwitchInst { value: value, default: default, num_cases: num_cases,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(value: Box<dyn Value>, default: BasicBlock,
    num_cases: usize, ie: Option<BasicBlock>) -> Self
  {
    SwitchInst { value: value, default: default, num_cases: num_cases,
      insert_before: None, insert_at_end: ie }
  }

  pub fn get_condition() {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Switch
  }

  fn grow_operands() {}
}
impl Value for SwitchInst {
  fn get_type(&self) -> &dyn Type {
    self.value.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for SwitchInst {}
impl Instruction for SwitchInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

// Indirect branch instruction.
#[derive(Debug)]
pub struct IndirectBrInst {
  address: Box<dyn Value>,
  num_dests: usize,
  insert_bedore: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl IndirectBrInst {
  pub fn new_insert_before(address: Box<dyn Value>, num_dests: usize,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    IndirectBrInst { address: address, num_dests: num_dests,
      insert_bedore: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(address: Box<dyn Value>, num_dests: usize,
    ie: Option<BasicBlock>) -> Self
  {
    IndirectBrInst { address: address, num_dests: num_dests,
      insert_bedore: None, insert_at_end: ie }
  }

  pub fn get_address(&self) -> &Box<dyn Value> {
    &self.address
  }

  pub fn set_adderss(&mut self, address: Box<dyn Value>) {
    self.address = address;
  }

  // Return the number of possible destinations in this indirectbr instruction.
  pub fn get_num_destinations(&self) -> usize {
    self.num_dests
  }

  //pub fn get_destination(&self, i: usize) -> BasicBlock {}

  // Add a destination.
  pub fn add_destination(&mut self, _dest: BasicBlock) {}

  pub fn class_of(&self, i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::IndirectBr
  }

  fn grow_operands() {}
}

impl Value for IndirectBrInst {
  fn get_type(&self) -> &dyn Type {
    self.address.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for IndirectBrInst {}
impl Instruction for IndirectBrInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

// Invoke instruction.
#[derive(Debug)]
pub struct InvokeInst {
  func_t: FunctionType,
  func: Box<dyn Value>,
  if_normal: BasicBlock,
  if_exception: BasicBlock,
  args: Vec<Box<dyn Value>>,
  bundles: Vec<OperandBundleDef<Box<dyn Value>>>,
  num_operands: usize,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl InvokeInst {
  // The nummber of operands for this call beyond the called function,
  // arguments, and operand bundles.
  const NUM_EXTRA_OPERANDS: usize = 2;

  pub fn new_insert_before(func_t: FunctionType, func: Box<dyn Value>,
    if_normal: BasicBlock, if_exception: BasicBlock, args: Vec<Box<dyn Value>>,
    bundles: Vec<OperandBundleDef<Box<dyn Value>>>, name: Twine,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    let num_operands =
      InvokeInst::compute_num_operands(args.len(), 0);
    InvokeInst {
      func_t: func_t, func: func, if_normal: if_normal,
      if_exception: if_exception, args: args, bundles: bundles,
      num_operands: num_operands, name: name, insert_before: ib,
      insert_at_end: None
    }
  }

  pub fn new_insert_at_end(func_t: FunctionType, func: Box<dyn Value>,
    if_normal: BasicBlock, if_exception: BasicBlock, args: Vec<Box<dyn Value>>,
    bundles: Vec<OperandBundleDef<Box<dyn Value>>>, name: Twine,
    ie: Option<BasicBlock>) -> Self
  {
    let num_operands =
      InvokeInst::compute_num_operands(args.len(), 0);
    InvokeInst {
      func_t: func_t, func: func, if_normal: if_normal,
      if_exception: if_exception, args: args, bundles: bundles,
      num_operands: num_operands, name: name, insert_before: None,
      insert_at_end: ie
    }
  }

  // Compute the number of operands to allocate.
  pub fn compute_num_operands(num_args: usize, num_bundle_inputs: usize) -> usize {
    1 + InvokeInst::NUM_EXTRA_OPERANDS + num_args + num_bundle_inputs
  }

  pub fn get_normal_dest(&self) -> &BasicBlock {
    &self.if_normal
  }

  pub fn set_normal_dest(&mut self, b: BasicBlock) {
    self.if_normal = b;
  }

  pub fn get_unwind_dest(&self) -> &BasicBlock {
    &self.if_exception
  }

  pub fn set_unwind_dest(&mut self, b: BasicBlock) {
    self.if_exception = b;
  }

  // Get the landingpad instruction from the landing pad block
  // (the unwind destination).
  pub fn  get_landing_pad_inst(&self) -> Option<&LandingPadInst> {
    let inst =
      self.get_unwind_dest().get_first_non_phi();
    if inst.is_some() {
      return inst.unwrap().as_any_inst().downcast_ref::<LandingPadInst>()
    }
    None
  }

  pub fn get_successor(&self, i: usize) -> &BasicBlock {
    debug_assert!(i < 2, "Successor # out of range for invoke.");
    if i == 0 {
      return self.get_normal_dest();
    } else {
      return self.get_unwind_dest();
    }
  }

  pub fn set_successor(&mut self, i: usize, new_succ: BasicBlock) {
    debug_assert!(i < 2, "Successor # out of range for invoke.");
    if i == 0 {
      self.set_normal_dest(new_succ);
    } else {
      self.set_unwind_dest(new_succ);
    }
  }

  pub fn get_num_successors(&self) -> usize { 2 }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Invoke
  }
}

impl Value for InvokeInst {
  fn get_type(&self) -> &dyn Type {
    &self.func_t
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for InvokeInst {}
impl CallBase for InvokeInst {}

impl Instruction for InvokeInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

// CallBr instruction, tracking function calls that may not return control
// but instead transfer it to a third location.
#[derive(Debug)]
pub struct CallBrInst {
  func_t: FunctionType,
  func: Box<dyn Value>,
  default_dest: BasicBlock,
  indirect_dests: Vec<BasicBlock>,
  args: Vec<Box<dyn Value>>,
  bundles: Vec<OperandBundleDef<Box<dyn Value>>>,
  num_operands: usize,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl CallBrInst {
  pub fn new_insert_before(func_t: FunctionType, func: Box<dyn Value>,
    default_dest: BasicBlock, indirect_dests: Vec<BasicBlock>,
    args: Vec<Box<dyn Value>>, bundles: Vec<OperandBundleDef<Box<dyn Value>>>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    let num_operands = CallBrInst::compute_num_operands(args.len(),
      indirect_dests.len(), 0);
    CallBrInst {
      func_t: func_t, func: func, default_dest: default_dest,
      indirect_dests: indirect_dests, args: args, bundles: bundles,
      num_operands: num_operands, name: name, insert_before: ib,
      insert_at_end: None
    }
  }

  pub fn new_insert_at_end(func_t: FunctionType, func: Box<dyn Value>,
    default_dest: BasicBlock, indirect_dests: Vec<BasicBlock>,
    args: Vec<Box<dyn Value>>, bundles: Vec<OperandBundleDef<Box<dyn Value>>>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    let num_operands = CallBrInst::compute_num_operands(args.len(),
      indirect_dests.len(), bundles.len());
    CallBrInst {
      func_t: func_t, func: func, default_dest: default_dest,
      indirect_dests: indirect_dests, args: args, bundles: bundles,
      num_operands: num_operands, name: name, insert_before: None,
      insert_at_end: ie
    }
  }

  pub fn compute_num_operands(num_args: usize, num_indirect_dests: usize,
    num_bundle_inputs: usize) -> usize
  {
    2 + num_indirect_dests + num_args + num_bundle_inputs
  }

  // Return the number of callbr indirect dest labels.
  pub fn get_num_indirect_dests(&self) -> usize {
    self.indirect_dests.len()
  }

  // Return the i-th indirect dest label.
  pub fn get_indirect_dest_label(&self) {}
  pub fn get_indirect_dest_label_use(&self) {}

  // Return the destination basic blocks.
  pub fn get_default_dest(&self) -> &BasicBlock {
    &self.default_dest
  }

  pub fn get_indirect_dest(&self, i: usize) -> &BasicBlock {
    &self.indirect_dests[i]
  }

  pub fn get_indirect_dests(&self) -> &Vec<BasicBlock> {
    &self.indirect_dests
  }

  pub fn set_default_dest(&mut self, b: BasicBlock) {
    self.default_dest = b;
  }

  pub fn set_indirect_dest(&mut self, i: usize, b: BasicBlock) {
    self.indirect_dests[i] = b;
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::CallBr
  }
}

impl Value for CallBrInst {
  fn get_type(&self) -> &dyn Type {
    &self.func_t
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for CallBrInst {}
impl CallBase for CallBrInst {}

impl Instruction for CallBrInst {
  fn get_num_successors(&self) -> usize {
    self.get_num_indirect_dests() + 1
  }

  fn get_successor(&self, i: usize) -> Option<&BasicBlock> {
    debug_assert!(i < self.get_num_successors() + 1,
      "Successor # out of range for callbr.");
    if i == 0 {
      return Some(self.get_default_dest());
    } else {
      return Some(self.get_indirect_dest(i - 1));
    }
  }

  fn set_successor(&mut self, i: usize, b: BasicBlock) {
    debug_assert!(i < self.get_num_indirect_dests() + 1,
      "Successor # out of range for callbr.");
    if i == 0 {
      self.set_default_dest(b);
    } else {
      self.set_indirect_dest(i - 1, b);
    }
  }

  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

// Resume the propagation of an exception.
#[derive(Debug)]
pub struct ResumeInst {
  exn: Box<dyn Value>,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl ResumeInst {
  pub fn new_insert_before(exn: Box<dyn Value>,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    ResumeInst { exn: exn, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(exn: Box<dyn Value>,
    ie: Option<BasicBlock>) -> Self
  {
    ResumeInst { exn: exn, insert_before: None, insert_at_end: ie }
  }

  pub fn get_value(&self) -> &Box<dyn Value> {
    &self.exn
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Resume
  }
}
impl Value for ResumeInst {
  fn get_type(&self) -> &dyn Type {
    self.exn.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for ResumeInst {}
impl Instruction for ResumeInst {
  fn get_num_successors(&self) -> usize { 0 }

  fn get_successor(&self, _i: usize) -> Option<&BasicBlock> {
    panic!("ResumeInst has no successors.");
  }

  fn set_successor(&mut self, _i: usize, _b: BasicBlock) {
    panic!("ResumeInst has no successors.");
  }

  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

#[derive(Debug)]
pub struct CatchSwitchInst {
  parent_pad: Box<dyn Value>,
  unwind_dest: Option<BasicBlock>,
  num_handlers: usize,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl CatchSwitchInst {
  pub fn new_insert_before(parent_pad: Box<dyn Value>,
    unwind_dest: Option<BasicBlock>, num_handlers: usize, name: Twine,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    CatchSwitchInst { parent_pad: parent_pad, unwind_dest: unwind_dest,
      num_handlers: num_handlers, name: name, insert_before: ib,
      insert_at_end: None }
  }

  pub fn new_insert_at_end(parent_pad: Box<dyn Value>,
    unwind_dest: Option<BasicBlock>, num_handlers: usize, name: Twine,
    ie: Option<BasicBlock>) -> Self
  {
    CatchSwitchInst { parent_pad: parent_pad, unwind_dest: unwind_dest,
      num_handlers: num_handlers, name: name, insert_before: None,
      insert_at_end: ie }
  }

  pub fn get_operand(&self, i: usize) -> Option<&Box<dyn Value>> {
    if i == 0 { return Some(&self.parent_pad); }
    else if i == 1 {
      if self.has_unwind_dest() {
        //let v = *self.unwind_dest.unwrap().as_any().downcast_ref::<dyn Value>().unwrap();
        //let t = Some(&Box::new(v));
        //return t;
      }
    }
    None
  }

  pub fn set_operand(&mut self, _i: usize, _v: Box<dyn Value>) {}

  pub fn get_num_operands(&self) -> usize { 0 } // TODO

  pub fn get_parent_pad(&self) -> &Box<dyn Value> {
    &self.parent_pad
  }

  pub fn set_parent_pad(&mut self, parent_pad: Box<dyn Value>) {
    self.parent_pad = parent_pad;
  }

  pub fn has_unwind_dest(&self) -> bool {
    self.unwind_dest.is_some()
  }

  pub fn unwinds_to_caller(&self) -> bool {
    !self.has_unwind_dest()
  }

  pub fn get_unwind_dest(&self) -> &Option<BasicBlock> {
    &self.unwind_dest
  }

  pub fn set_unwind_dest(&mut self, unwind_dest: BasicBlock) {
    debug_assert!(self.has_unwind_dest());
    self.unwind_dest = Some(unwind_dest);
  }

  // Return the number of 'handlers' in this catchswitch instruction,
  // except the default handler.
  pub fn get_num_handlers(&self) -> usize {
    self.num_handlers
  }

  pub fn handlers(&self) {}

  // Add an entry to the switch instruction...
  pub fn add_handler(&mut self, _dest: BasicBlock) {}

  pub fn remove_handler(&mut self) {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::CatchSwitch
  }

  fn grow_operands() {}
}
impl Value for CatchSwitchInst {
  fn get_type(&self) -> &dyn Type {
    self.parent_pad.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for CatchSwitchInst {}
impl Instruction for CatchSwitchInst {
  fn get_num_successors(&self) -> usize {
    self.get_num_operands() - 1
  }

  fn get_successor(&self, _i: usize) -> Option<&BasicBlock> {
    let v = self.unwind_dest.as_ref().unwrap();
    Some(v)
  }

  fn set_successor(&mut self, _i: usize, _b: BasicBlock) {}

  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

#[derive(Debug)]
pub struct CleanupPadInst {
  parent_pad: Box<dyn Value>,
  args: Vec<Box<dyn Value>>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl CleanupPadInst {
  pub fn new_insert_before(parent_pad: Box<dyn Value>,
    args: Vec<Box<dyn Value>>, name: Twine,ib: Option<Box<dyn Instruction>>) -> Self
  {
    CleanupPadInst { parent_pad: parent_pad, args: args, name: name,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(parent_pad: Box<dyn Value>,
    args: Vec<Box<dyn Value>>, name: Twine,ie: Option<BasicBlock>) -> Self
  {
    CleanupPadInst { parent_pad: parent_pad, args: args, name: name,
      insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::CleanupPad
  }
}
impl Value for CleanupPadInst {
  fn get_type(&self) -> &dyn Type {
    self.parent_pad.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for CleanupPadInst {}
impl Instruction for CleanupPadInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

impl FuncletPadInst for CleanupPadInst {
  fn arg_size(&self) -> usize {
    self.args.len()
  }

  fn get_parent_pad(&self) -> Option<&Box<dyn Value>> {
    Some(&self.parent_pad)
  }

  fn set_parent_pad(&mut self, parent_pad: Box<dyn Value>) {
    self.parent_pad = parent_pad;
  }

  fn get_arg_operand(&self, i: usize) -> Option<&Box<dyn Value>> {
    Some(&self.args[i])
  }

  fn set_arg_operand(&mut self, i: usize, v: Box<dyn Value>) {
    self.args[i] = v;
  }
}

#[derive(Debug)]
pub struct CatchPadInst {
  catch_switch: CatchSwitchInst,
  args: Vec<Box<dyn Value>>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl CatchPadInst {
  pub fn new_insert_before(catch_switch: CatchSwitchInst,
    args: Vec<Box<dyn Value>>, name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    CatchPadInst { catch_switch: catch_switch, args: args, name: name,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(catch_switch: CatchSwitchInst,
    args: Vec<Box<dyn Value>>, name: Twine, ie: Option<BasicBlock>) -> Self
  {
    CatchPadInst { catch_switch: catch_switch, args: args, name: name,
      insert_before: None, insert_at_end: ie }
  }

  pub fn get_catch_switch(&self) -> &CatchSwitchInst {
    &self.catch_switch
  }

  pub fn set_catch_switch(&mut self, catch_switch: CatchSwitchInst) {
    self.catch_switch = catch_switch;
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::CatchPad
  }
}
impl Value for CatchPadInst {
  fn get_type(&self) -> &dyn Type {
    self.catch_switch.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for CatchPadInst {}
impl Instruction for CatchPadInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

impl FuncletPadInst for CatchPadInst {
  fn arg_size(&self) -> usize {
    self.args.len()
  }

  fn get_arg_operand(&self, i: usize) -> Option<&Box<dyn Value>> {
    Some(&self.args[i])
  }

  fn set_arg_operand(&mut self, i: usize, v: Box<dyn Value>) {
    self.args[i] = v;
  }
}

#[derive(Debug)]
pub struct CatchReturnInst {
  catch_pad: CatchPadInst,
  bb: BasicBlock,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl CatchReturnInst {
  pub fn new_insert_before(catch_pad: CatchPadInst,
    bb: BasicBlock, ib: Option<Box<dyn Instruction>>) -> Self
  {
    CatchReturnInst { catch_pad: catch_pad, bb: bb, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(catch_pad: CatchPadInst,
    bb: BasicBlock, ie: Option<BasicBlock>) -> Self
  {
    CatchReturnInst { catch_pad: catch_pad, bb: bb, insert_before: None, insert_at_end: ie }
  }

  pub fn get_catch_pad(&self) -> &CatchPadInst {
    &self.catch_pad
  }

  pub fn set_catch_pad(&mut self, catch_pad: CatchPadInst) {
    self.catch_pad = catch_pad;
  }

  pub fn get_catch_switch_parent_pad(&self) {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::CatchRet
  }
}

impl Value for CatchReturnInst {
  fn get_type(&self) -> &dyn Type {
    self.bb.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for CatchReturnInst {}
impl Instruction for CatchReturnInst {
  fn get_successor(&self, _i: usize) -> Option<&BasicBlock> {
    Some(&self.bb)
  }

  fn set_successor(&mut self, _i: usize, b: BasicBlock) {
    self.bb = b;
  }

  fn get_num_successors(&self) -> usize { 1 }

  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

#[derive(Debug)]
pub struct CleanupReturnInst {
  cleanup_pad: Box<dyn Value>,
  unwind_bb: Option<BasicBlock>,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl CleanupReturnInst {
  pub fn new_insert_before(cleanup_pad: Box<dyn Value>,
    unwind_bb: Option<BasicBlock>, ib: Option<Box<dyn Instruction>>) -> Self
  {
    CleanupReturnInst { cleanup_pad: cleanup_pad, unwind_bb: unwind_bb,
      insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(cleanup_pad: Box<dyn Value>,
    unwind_bb: Option<BasicBlock>, ie: Option<BasicBlock>) -> Self
  {
    CleanupReturnInst { cleanup_pad: cleanup_pad, unwind_bb: unwind_bb,
      insert_before: None, insert_at_end: ie }
  }

  pub fn has_unwind_dest(&self) -> bool {
    self.unwind_bb.is_some()
  }

  pub fn unwinds_to_caller(&self) -> bool {
    !self.has_unwind_dest()
  }

  pub fn get_cleanup_pad(&self) {}
  pub fn set_cleanup_pad() {}

  pub fn get_unwind_dest(&self) -> Option<&BasicBlock> {
    Some(self.unwind_bb.as_ref().unwrap())
  }

  pub fn set_unwind_dest(&mut self, new_dest: BasicBlock) {
    self.unwind_bb = Some(new_dest);
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::CleanupRet
  }
}
impl Value for CleanupReturnInst {
  fn get_type(&self) -> &dyn Type {
    self.cleanup_pad.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for CleanupReturnInst {}
impl Instruction for CleanupReturnInst {
  fn get_num_successors(&self) -> usize {
    if self.has_unwind_dest() { return 1; } else { return 0; }
  }

  fn get_successor(&self, i: usize) -> Option<&BasicBlock> {
    debug_assert!(i == 0);
    self.get_unwind_dest()
  }

  fn set_successor(&mut self, i: usize, b: BasicBlock) {
    debug_assert!(i == 0);
    self.set_unwind_dest(b);
  }

  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

// This function has undefined behavior. In particular, the presence of
// this instruction indicates some higher level knowledge that the end of
// the block cannot be reached.
#[derive(Debug)]
pub struct UnreachableInst {
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>,
  t: Box<dyn Type> // dummy
}

impl UnreachableInst {
  pub fn new_insert_before(ib: Option<Box<dyn Instruction>>) -> Self {
    UnreachableInst { insert_before: ib, insert_at_end: None,
      t: Box::new(UnknownType::new()) }
  }

  pub fn new_insert_at_end(ie: Option<BasicBlock>) -> Self {
    UnreachableInst { insert_before: None, insert_at_end: ie,
      t: Box::new(UnknownType::new()) }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Unreachable
  }
}
impl Value for UnreachableInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for UnreachableInst {}
impl Instruction for UnreachableInst {
  fn get_num_successors(&self) -> usize { 0 }

  fn get_successor(&self, _i: usize) -> Option<&BasicBlock> {
    unreachable!("UnreachableInst has no successors.");
  }

  fn set_successor(&mut self, _i: usize, _b: BasicBlock) {
    unreachable!("UnreachableInst has no successors.");
  }

  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

// This class represents a truncation of integer types.
#[derive(Debug)]
pub struct TruncInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl TruncInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    TruncInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    TruncInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Trunc
  }
}
impl Value for TruncInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for TruncInst {}
impl Instruction for TruncInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}
impl UnaryInstruction for TruncInst {}
impl CastInst for TruncInst {}

// This class represents zero extension of integer types.
#[derive(Debug)]
pub struct ZExtInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl ZExtInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    ZExtInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    ZExtInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::ZExt
  }
}

impl Value for ZExtInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for ZExtInst {}
impl Instruction for ZExtInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}

impl UnaryInstruction for ZExtInst {}
impl CastInst for ZExtInst {}


// This class represents sign extension of integer types.
#[derive(Debug)]
pub struct SExtInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl SExtInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    SExtInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    SExtInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::SExt
  }
}

impl Value for SExtInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for SExtInst {}
impl Instruction for SExtInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for SExtInst {}
impl CastInst for SExtInst {}

// This class represents a truncation of floating point types.
#[derive(Debug)]
pub struct FPTruncInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl FPTruncInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    FPTruncInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    FPTruncInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::FPTrunc
  }
}

impl Instruction for FPTruncInst {
  fn as_any_inst(&self) -> &dyn Any {
    self
  }
}
impl Value for FPTruncInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for FPTruncInst {}
impl UnaryInstruction for FPTruncInst {}
impl CastInst for FPTruncInst {}

// This class represents an extension of floating point types.
#[derive(Debug)]
pub struct FPExtInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl FPExtInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    FPExtInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    FPExtInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::FPExt
  }
}
impl Value for FPExtInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for FPExtInst {}
impl Instruction for FPExtInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for FPExtInst {}
impl CastInst for FPExtInst {}

// This class represents a cast unsigned integer to floating point.
#[derive(Debug)]
pub struct UItoFPInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl UItoFPInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    UItoFPInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    UItoFPInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::UIToFP
  }
}

impl Value for UItoFPInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for UItoFPInst {}
impl Instruction for UItoFPInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for UItoFPInst {}
impl CastInst for UItoFPInst {}

// This class represents a cast from signed integer to floating point.
#[derive(Debug)]
pub struct SIToFPInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl SIToFPInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    SIToFPInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    SIToFPInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::SIToFp
  }
}

impl Value for SIToFPInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for SIToFPInst {}
impl Instruction for SIToFPInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for SIToFPInst {}
impl CastInst for SIToFPInst {}

// This class represents a cast from floating point to unsigned integer.
#[derive(Debug)]
pub struct FPToUIInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl FPToUIInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    FPToUIInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    FPToUIInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::FPToUI
  }
}

impl Value for FPToUIInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for FPToUIInst {}
impl Instruction for FPToUIInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for FPToUIInst {}
impl CastInst for FPToUIInst {}

// This class represents a cast from floating point to signed integer.
#[derive(Debug)]
pub struct FPToSIInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl FPToSIInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    FPToSIInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    FPToSIInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::FPToSI
  }
}

impl Value for FPToSIInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for FPToSIInst {}
impl Instruction for FPToSIInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for FPToSIInst {}
impl CastInst for FPToSIInst {}

#[derive(Debug)]
pub struct IntToPtrInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl IntToPtrInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    IntToPtrInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    IntToPtrInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  // Returns the address space of this instrunction's pointer type.
  pub fn get_address_space(&self) -> usize {
    0
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::IntToPtr
  } 
}
impl Value for IntToPtrInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for IntToPtrInst {}
impl Instruction for IntToPtrInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for IntToPtrInst {}
impl CastInst for IntToPtrInst {}

// This class represents a cast from a pointer to an integer.
#[derive(Debug)]
pub struct PtrToIntInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl PtrToIntInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    PtrToIntInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    PtrToIntInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn get_pointer_operand(&self) {}
  pub fn get_pointer_operand_index(&self) {}
  pub fn get_pointer_address_space(&self) {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::PtrToInt
  }
}
impl Value for PtrToIntInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for PtrToIntInst {}
impl Instruction for PtrToIntInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for PtrToIntInst {}
impl CastInst for PtrToIntInst {}

// This class represents a no-op cast from one type to another.
#[derive(Debug)]
pub struct BitCastInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl BitCastInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    BitCastInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    BitCastInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::BitCast
  }
}

impl Value for BitCastInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for BitCastInst {}
impl Instruction for BitCastInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for BitCastInst {}
impl CastInst for BitCastInst {}

// This class represents a conversion between pointers from one address
// space to another.
#[derive(Debug)]
pub struct AddrSpaceCastInst {
  s: Box<dyn Value>,
  t: Box<dyn Type>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl AddrSpaceCastInst {
  pub fn new_insert_before(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ib: Option<Box<dyn Instruction>>) -> Self
  {
    AddrSpaceCastInst { s: s, t: t, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, t: Box<dyn Type>,
    name: Twine, ie: Option<BasicBlock>) -> Self
  {
    AddrSpaceCastInst { s: s, t: t, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn get_pointer_operand(&self) {}
  pub fn get_pointer_operand_index(&self) {}
  pub fn get_src_address_space(&self) {}
  pub fn get_dest_address_space(&self) {}

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::AddrSpaceCast
  }
}

impl Value for AddrSpaceCastInst {
  fn get_type(&self) -> &dyn Type {
    self.t.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for AddrSpaceCastInst {}
impl Instruction for AddrSpaceCastInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for AddrSpaceCastInst {}
impl CastInst for AddrSpaceCastInst {}

// This class represents a freeze function that returns random concrete
// value if an operand is either a poison value or an undef value.
#[derive(Debug)]
pub struct FreezeInst {
  s: Box<dyn Value>,
  name: Twine,
  insert_before: Option<Box<dyn Instruction>>,
  insert_at_end: Option<BasicBlock>
}

impl FreezeInst {
  pub fn new_insert_before(s: Box<dyn Value>, name: Twine,
    ib: Option<Box<dyn Instruction>>) -> Self
  {
    FreezeInst { s: s, name: name, insert_before: ib, insert_at_end: None }
  }

  pub fn new_insert_at_end(s: Box<dyn Value>, name: Twine,
    ie: Option<BasicBlock>) -> Self
  {
    FreezeInst { s: s, name: name, insert_before: None, insert_at_end: ie }
  }

  pub fn class_of(i: Box<dyn Instruction>) -> bool {
    i.get_op_code() == OpCode::Freeze
  }
}

impl Value for FreezeInst {
  fn get_type(&self) -> &dyn Type {
    self.s.get_type()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::InstructionVal
  }

  fn as_any(&self) -> &dyn Any { self }
}
impl User for FreezeInst {}
impl Instruction for FreezeInst { fn as_any_inst(&self) -> &dyn Any { self } }
impl UnaryInstruction for FreezeInst {}

#[cfg(test)]
mod tests {
  use crate::ir::{
    type_::{self, FixedVectorType, /*ScalableVectorType*/},
    type_::{IntegerType, ScalableVectorType},
    blits_context::{BlitzContext, blits_context_mut},
    constants::ConstantInt, constant, instr_types};
  use super::*;

  #[test]
  fn test_return_inst() {
    let mut c = BlitzContext::new();
    let r0 =
      ReturnInst::new_insert_before(None, None);
    assert_eq!(r0.get_num_operands(), 0);

    let i1 = IntegerType::get(&mut c, 1);
    let one = ConstantInt::get(&i1, 1, true);
    let r1 =
      ReturnInst::new_insert_before(Some(Box::new(one)), None);
    assert_eq!(r1.get_num_operands(), 1);
    let val =
      r1.get_operand(0).unwrap().as_any().downcast_ref::<ConstantInt>();
    assert_eq!(val.unwrap().is_one(), true);
  }

  #[test]
  fn test_branch_inst() {
    let bb0 = BasicBlock::new(None, None,
      None);
    let br0 = BranchInst::new_insert_before(Box::new(bb0),
      None, None, None);
    assert_eq!(br0.is_unconditional(), true);
    assert_eq!(br0.is_conditional(), false);
    assert_eq!(br0.get_num_successors(), 1);
    assert_eq!(br0.get_num_operands(), 1);

    let i1 = IntegerType  ::get(&blits_context_mut(), 1);
    let one = ConstantInt::get(&i1, 1, true);

    let bb0_1 = BasicBlock::new(
      Some(Twine::new_from_string(String::from("bb0_1"))),
      None, None);
    let bb1 = BasicBlock::new(
      Some(Twine::new_from_string(String::from("bb1"))),
      None, None);
    let br1 = BranchInst::new_insert_before(Box::new(bb0_1),
      Some(Box::new(bb1)), Some(Box::new(one)), None);
    assert_eq!(br1.is_unconditional(), false);
    assert_eq!(br1.is_conditional(), true);
    assert_eq!(br1.get_num_successors(), 2);
    assert_eq!(br1.get_num_operands(), 3);

    let cond1 =
      br1.get_operand(0).unwrap().as_any().downcast_ref::<ConstantInt>();
    assert_eq!(cond1.unwrap().is_one(), true);
    let cond2 =
      br1.get_condition().as_any().downcast_ref::<ConstantInt>();
    assert_eq!(cond2.unwrap().is_one(), true);

    let if_false_1 =
      br1.get_operand(1).unwrap().as_any().downcast_ref::<BasicBlock>();
    assert_eq!(if_false_1.as_ref().unwrap().name.as_ref().unwrap().str(),
      Some(String::from("bb1")));
    let if_false_2 = br1.get_successor(1);
    assert_eq!(if_false_2.as_ref().unwrap().name.as_ref().unwrap().str(),
      Some(String::from("bb1")));

    let if_true_1 =
      br1.get_operand(2).unwrap().as_any().downcast_ref::<BasicBlock>();
    assert_eq!(if_true_1.as_ref().unwrap().name.as_ref().unwrap().str(),
      Some(String::from("bb0_1")));
    let if_true_2 = br1.get_successor(0);
    assert_eq!(if_true_2.as_ref().unwrap().name.as_ref().unwrap().str(),
      Some(String::from("bb0_1")));
  }

  #[test]
  fn test_cast_inst() {
    let i8_t = type_::get_int_8_type(&blits_context_mut());
    let v8x8_t =
      FixedVectorType::get(Box::new(i8_t), 8);
  
    let i64_t = type_::get_int_64_type(&blits_context_mut());
    let v8x64_t =
      FixedVectorType::get(Box::new(i64_t), 8);
        
    let x86mmx_t = type_::get_x86_mmx_type(&blits_context_mut());
    let c8 = constant::get_null_value(&Box::new(&v8x8_t));
    let c64 = constant::get_null_value(&Box::new(&v8x64_t));

    assert_eq!(instr_types::get_cast_op_code(c64.as_ref(), true, &v8x8_t, true), OpCode::Trunc);
    assert_eq!(instr_types::get_cast_op_code(c8.as_ref(), true, &v8x64_t, true), OpCode::SExt);

    let i64_t_2 = type_::get_int_64_type(&blits_context_mut());
    assert_eq!(instr_types::is_bit_castable(&v8x8_t, &x86mmx_t), false);
    assert_eq!(instr_types::is_bit_castable(&x86mmx_t, &v8x8_t), false);
    assert_eq!(instr_types::is_bit_castable(&i64_t_2, &x86mmx_t), false);
    assert_eq!(instr_types::is_bit_castable(&v8x64_t, &v8x8_t), false);
    assert_eq!(instr_types::is_bit_castable(&v8x8_t, &v8x64_t), false);

    // Check address space casts are rejected since we don't know the sizes here.
    let i32_t = type_::get_int_32_type(&blits_context_mut());
    let i32_t_1 = type_::get_int_32_type(&blits_context_mut());
    let i32_t_2 = type_::get_int_32_type(&blits_context_mut());
    let i32_t_3 = type_::get_int_32_type(&blits_context_mut());

    let i32_ptr_t = PointerType::get(Box::new(i32_t), 0);
    let i32_ptr_t_1 = PointerType::get(Box::new(i32_t_1), 0);
    let i32_ptr_as1_t = PointerType::get(Box::new(i32_t_2), 1);
    let i32_ptr_as1_1_t = PointerType::get(Box::new(i32_t_3), 1);

    let i64_t_3 = type_::get_int_64_type(&blits_context_mut());
    let i64_ptr_as1_t = PointerType::get(Box::new(i64_t_3), 1);

    let v2_i32_ptr_t = FixedVectorType::get(Box::new(i32_ptr_t_1), 2);
    let v2_i32_ptr_as1_t = FixedVectorType::get(Box::new(i32_ptr_as1_t), 2);
    let v2_i64_ptr_as1_t = FixedVectorType::get(Box::new(i64_ptr_as1_t), 2);
    let v2_ptr_32 = constant::get_null_value(&Box::new(&v2_i32_ptr_t));

    assert_eq!(instr_types::is_bit_castable(&i32_ptr_t, &i32_ptr_as1_1_t), false);
    assert_eq!(instr_types::is_bit_castable(&i32_ptr_as1_1_t, &i32_ptr_t), false);
    assert_eq!(instr_types::is_bit_castable(&v2_i32_ptr_t, &v2_i32_ptr_as1_t), false);
    assert_eq!(instr_types::is_bit_castable(&v2_i32_ptr_as1_t, &v2_i64_ptr_as1_t), true);
    assert_eq!(instr_types::get_cast_op_code(v2_ptr_32.as_ref(), true, &v2_i32_ptr_as1_t, true),
      OpCode::AddrSpaceCast);

    // Test mismatched number of elements for pointers.
    let i64_t_4 = type_::get_int_64_type(&blits_context_mut());
    let i64_ptr_as1_1_t = PointerType::get(Box::new(i64_t_4), 1);
    let v4_i64_ptr_as1_t = FixedVectorType::new_from_ptr(i64_ptr_as1_1_t, 4);

    let i32_t_4 = type_::get_int_32_type(&blits_context_mut());
    let i32_ptr_as1_2_t = PointerType::get(Box::new(i32_t_4), 1);
    let v4_i32_ptr_as1_t = FixedVectorType::new_from_ptr(i32_ptr_as1_2_t, 4);
    assert_eq!(instr_types::is_bit_castable(&v2_i32_ptr_as1_t, &v4_i64_ptr_as1_t), false);
    assert_eq!(instr_types::is_bit_castable(&v4_i64_ptr_as1_t, &v2_i32_ptr_as1_t), false);
    assert_eq!(instr_types::is_bit_castable(&v2_i32_ptr_as1_t, &v4_i32_ptr_as1_t), false);
    assert_eq!(instr_types::is_bit_castable(&i32_ptr_t, &v2_i32_ptr_t), false);
    assert_eq!(instr_types::is_bit_castable(&v2_i32_ptr_t, &i32_ptr_t), false);

    let i64_t_5 = type_::get_int_64_type(&blits_context_mut());
    let i64_ptr_t = PointerType::get(Box::new(i64_t_5), 0);
    let float_t = type_::get_float_type(&blits_context_mut());
    let double_t = type_::get_double_type(&blits_context_mut());
    let i32_t_5 = type_::get_int_32_type(&blits_context_mut());
    let i16_t = type_::get_int_16_type(&blits_context_mut());
    let half_t = type_::get_half_type(&blits_context_mut());
    let i32_t_6 = type_::get_int_32_type(&blits_context_mut());
    let v2_i32_t = FixedVectorType::get(Box::new(i32_t_6), 2);
    let i64_t_6 = type_::get_int_64_type(&blits_context_mut());
    assert_eq!(instr_types::is_bit_castable(&i32_ptr_t, &i64_ptr_t), true);
    assert_eq!(instr_types::is_bit_castable(&double_t, &float_t), false);
    assert_eq!(instr_types::is_bit_castable(&float_t, &double_t), false);
    assert_eq!(instr_types::is_bit_castable(&float_t, &float_t), true);
    assert_eq!(instr_types::is_bit_castable(&float_t, &i32_t_5), true);
    assert_eq!(instr_types::is_bit_castable(&i16_t, &half_t), true);
    assert_eq!(instr_types::is_bit_castable(&i32_t_5, &float_t), true);
    assert_eq!(instr_types::is_bit_castable(&v2_i32_t, &i64_t_6), true);

    let i16_t_1 = type_::get_int_16_type(&blits_context_mut());
    let v4_i16_t = FixedVectorType::get(Box::new(i16_t_1), 4);
    assert_eq!(instr_types::is_bit_castable(&v2_i32_t, &v4_i16_t), true);
    assert_eq!(instr_types::is_bit_castable(&i32_t_5, &i64_t_6), false);
    assert_eq!(instr_types::is_bit_castable(&i64_t_6, &i32_t_5), false);

    let i64_t_7 = type_::get_int_64_type(&blits_context_mut());
    let i64_ptr_t_1 = PointerType::get(Box::new(i64_t_7), 0);
    let v2_i64_ptr_t =  FixedVectorType::get(Box::new(i64_ptr_t_1), 2);
    let i64_t_8 = type_::get_int_64_type(&blits_context_mut());
    let v2_i64_t = FixedVectorType::get(Box::new(i64_t_8), 2);
    assert_eq!(instr_types::is_bit_castable(&v2_i32_ptr_t, &i64_t_6), false);
    assert_eq!(instr_types::is_bit_castable(&i64_t_6, &v2_i32_ptr_t), false);
    assert_eq!(instr_types::is_bit_castable(&v2_i64_ptr_t, &v2_i32_ptr_t), true);
    assert_eq!(instr_types::is_bit_castable(&v2_i32_ptr_t, &v2_i64_ptr_t), true);
    assert_eq!(instr_types::is_bit_castable(&v2_i32_t, &v2_i64_t), false);
    assert_eq!(instr_types::is_bit_castable(&v2_i64_t, &v2_i32_t), false);

    let i32_t_7 = type_::get_int_32_type(&blits_context_mut());
    let i32_ptr_t_1 = PointerType::get(Box::new(i32_t_7), 0);
    let v4_i32_ptr_t = FixedVectorType::get(Box::new(i32_ptr_t_1), 4);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&v4_i32_ptr_t)).get_type(),
      &v2_i32_ptr_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&v2_i32_ptr_t)).get_type(),
      &v4_i32_ptr_t), false);

    assert_eq!(instr_types::cast_is_valid(OpCode::AddrSpaceCast,
      constant::get_null_value(&Box::new(&v4_i32_ptr_as1_t)).get_type(),
      &v2_i32_ptr_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::AddrSpaceCast,
      constant::get_null_value(&Box::new(&v2_i32_ptr_t)).get_type(),
      &v4_i32_ptr_as1_t), false);

    // Address space cast of fixed/scalable vectors of pointers to scalable/fixed
    // vector of pointers.
    let i32_t_8 = type_::get_int_32_type(&blits_context_mut());
    let i32_ptr_as1_3_t = PointerType::get(Box::new(i32_t_8), 1);
    let vscale_v4_i32_ptr_as1_t =
      ScalableVectorType::new_from_ptr(i32_ptr_as1_3_t, 4);
    assert_eq!(instr_types::cast_is_valid(OpCode::AddrSpaceCast,
      constant::get_null_value(&Box::new(&vscale_v4_i32_ptr_as1_t)).get_type(),
      &v4_i32_ptr_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::AddrSpaceCast,
      constant::get_null_value(&Box::new(&v4_i32_ptr_t)).get_type(),
      &vscale_v4_i32_ptr_as1_t), false);

    // Address space cast of scalabel vectors of pointers to scalable vector
    // of pointers.
    let i32_t_9 = type_::get_int_32_type(&blits_context_mut());
    let i32_ptr_t_2 = PointerType::get(Box::new(i32_t_9), 0);
    let vscale_v2_i32_ptr_t = ScalableVectorType::new_from_ptr(i32_ptr_t_2, 2);
    let i64_t_9 = type_::get_int_64_type(&blits_context_mut());
    let i64_ptr_t_2 = PointerType::get(Box::new(i64_t_9), 0);
    let vscale_v4_i64_ptr_t = ScalableVectorType::new_from_ptr(i64_ptr_t_2, 4);
    assert_eq!(instr_types::cast_is_valid(OpCode::AddrSpaceCast,
      constant::get_null_value(&Box::new(&vscale_v4_i32_ptr_as1_t)).get_type(),
      &vscale_v2_i32_ptr_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::AddrSpaceCast,
      constant::get_null_value(&Box::new(&vscale_v2_i32_ptr_t)).get_type(),
      &vscale_v4_i32_ptr_as1_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::AddrSpaceCast,
      constant::get_null_value(&Box::new(&vscale_v4_i64_ptr_t)).get_type(),
      &vscale_v4_i32_ptr_as1_t), true);

    // Same number of lanes, different address scapce.
    let i32_t_10 = type_::get_int_32_type(&blits_context_mut());
    let i32_ptr_t_3 = PointerType::get(Box::new(i32_t_10), 0);
    let vscale_v4_i32_ptr_t = ScalableVectorType::new_from_ptr(i32_ptr_t_3, 4);
    assert_eq!(instr_types::cast_is_valid(OpCode::AddrSpaceCast,
      constant::get_null_value(&Box::new(&vscale_v4_i32_ptr_as1_t)).get_type(),
      &vscale_v4_i32_ptr_t), true);

    // Same number of lanes, same address scapce.
    assert_eq!(instr_types::cast_is_valid(OpCode::AddrSpaceCast,
      constant::get_null_value(&Box::new(&vscale_v4_i64_ptr_t)).get_type(),
      &vscale_v4_i32_ptr_t), false);

    // Bit casting fixed/scalable vector to scalable/fixed vectors.
    let i32_t_11 = type_::get_int_32_type(&blits_context_mut());
    let vscale_v2_i32_t = ScalableVectorType::new_from_int(i32_t_11, 2);
    let i64_t_10 = type_::get_int_64_type(&blits_context_mut());
    let vscale_v2_i64_t = ScalableVectorType::new_from_int(i64_t_10, 2);
    let i16_t_2 = type_::get_int_16_type(&blits_context_mut());
    let vscale_v4_i16_t = ScalableVectorType::new_from_int(i16_t_2, 4);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&v2_i32_t)).get_type(),
      &vscale_v2_i32_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&v2_i64_t)).get_type(),
      &vscale_v2_i64_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&v4_i16_t)).get_type(),
      &vscale_v4_i16_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&vscale_v2_i32_t)).get_type(),
      &v2_i32_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&vscale_v2_i64_t)).get_type(),
      &v2_i64_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&vscale_v4_i16_t)).get_type(),
      &v4_i16_t), false);

    // Bit casting scalable vectors to scalable vectors.
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&vscale_v4_i16_t)).get_type(),
      &vscale_v2_i32_t), true);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&vscale_v2_i32_t)).get_type(),
      &vscale_v4_i16_t), true);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&vscale_v2_i64_t)).get_type(),
      &vscale_v2_i32_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&vscale_v2_i32_t)).get_type(),
      &vscale_v2_i64_t), false);

    // Bit casting to/from <vscale x 1 x t>
    let i16_t_3 = type_::get_int_16_type(&blits_context_mut());
    let v1_i16_t = FixedVectorType::new_from_int(i16_t_3, 1);
    let i16_t_4 = type_::get_int_16_type(&blits_context_mut());
    let vscale_v1_i16_t = ScalableVectorType::new_from_int(i16_t_4, 1);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&vscale_v1_i16_t)).get_type(),
      &v1_i16_t), false);
    assert_eq!(instr_types::cast_is_valid(OpCode::BitCast,
      constant::get_null_value(&Box::new(&v1_i16_t)).get_type(),
      &vscale_v1_i16_t), false);
  }
}