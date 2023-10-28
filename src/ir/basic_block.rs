#![allow(dead_code)]

// This file contains the declaration of the BasicBlock class.

use crate::adt::twine::Twine;

use super::{
  function::Function,
  //symbol_table_list::SymbolTableList,
  instruction::Instruction,
  value::{Value, ValueType},
  type_::Type, blits_context::blits_context_mut,
  module::Module, instructions::{PhiNode, LoadInst, StoreInst, LandingPadInst},
  intrinsic_inst::PseudoProbeInst, //intrinsic_inst::DbgInfoIntrinsic
};

// This represents a single basic block in Blitz.
// A basic blocks is simply a container of instructions that execute
// sequentially.
#[derive(Debug)]
pub struct BasicBlock {
  v_type: Box<dyn Type>,
  inst_list: Vec<Box<dyn Instruction>>, //SymbolTableList<Instruction>,
  pub name: Option<Twine>,
  parent: Option<Function>,
  insert_before: Option<Box<BasicBlock>>
}

impl BasicBlock {
  // If the function parameter is specified, the basic block is automatically
  // inserted at either the end of the function (if insert_before is null),
  // or before the specified basic block.
  pub fn new(name: Option<Twine>, parent: Option<Function>,
    insert_before: Option<Box<BasicBlock>>) -> Self
  {
    let bb = BasicBlock {
      v_type: Box::new(super::type_::get_label_type(&blits_context_mut())),
      inst_list: Vec::new(),
      name: name,
      parent: parent,
      insert_before: insert_before
    };
    if bb.parent.is_some() {
      // TODO
    } else {
      debug_assert!(bb.insert_before.is_none(),
        "Cannot insert block before another block with no function.");
    }
    bb
  }

  pub fn set_parent() {}

  // Return the enclosing method, or null if none.
  pub fn get_parent(&self) -> &Option<Function> {
    &self.parent
  }
  
  // Return the module owning the function this basic block belongs to.
  pub fn get_module(&self) -> &Option<Module> {
    //&self.get_parent().as_ref().unwrap().get_parent()
    &None
  }

  // Returns the terminator instruction if the block is well formed or
  // null if the block is not well formed.
  pub fn get_terminator(&self) -> Option<&Box<dyn Instruction>> {
    if self.inst_list.is_empty() ||
       !self.inst_list.last().unwrap().is_terminator(){
      return None;
    }
    self.inst_list.last()
  }

  pub fn get_terminating_deoptimize_call() {}
  pub fn get_post_dominating_deoptimize_call() {}
  pub fn get_terminating_must_tail_call() {}

  // Returns a pointer the first instruction in this block that is not a
  // PhiNode instruction.
  pub fn get_first_non_phi(&self)-> Option<&Box<dyn Instruction>> {
    for i in &self.inst_list {
      if i.as_any().downcast_ref::<PhiNode>().is_some() {
        return Some(i);
      }
    }
    None
  }

  // Returns a pointer to the first instruction in this block that is not a
  // PhiNode or a debug intrinsic, or any pseudo operation if skip_pseudo_op
  // is true.
  pub fn get_first_non_phi_or_dbg(&self, skip_pseudo_op: bool)
    -> Option<&Box<dyn Instruction>>
  {
    for i in &self.inst_list {
      if i.as_any().downcast_ref::<PhiNode>().is_some() /*||
        i.as_any().downcast_ref::<DbgInfoIntrinsic>().is_some()*/ {
        continue;
      }
      if skip_pseudo_op && i.as_any().downcast_ref::<PseudoProbeInst>().is_some() {
        continue;
      }
      return Some(i);
    }
    None
  }

  // Returns a pointer to the first instruction in this block that is not a
  // PhiNode, a debug intrinsic, or a lifetimme intrinsic, or any pseudo operation
  // if skip_pseudo_op is true.
  pub fn get_first_non_phi_or_dbg_or_lifetime(&self, skip_pseudo_op: bool)
    -> Option<&Box<dyn Instruction>>
  {
    for i in &self.inst_list {
      if i.as_any().downcast_ref::<PhiNode>().is_some() /*||
        i.as_any().downcast_ref::<DbgInfoIntrinsic>().is_some()*/ {
        continue;
      }
      if i.is_lifetime_start_or_end() {
        continue;
      }
      if skip_pseudo_op && i.as_any().downcast_ref::<PseudoProbeInst>().is_some() {
        continue;
      }
      return Some(i);
    }
    None
  }

  pub fn get_first_insertion_pt() {}
  pub fn get_first_non_phi_or_dbg_or_alloca() {}

  // Returns the first potential AsynchEH faulty instruction.
  // Currently it checks for loads/stores (which may dereference a null pointer)
  // and calls/invokes (which may propagate exceptions).
  pub fn get_first_may_fault_inst(&self) -> Option<&Box<dyn Instruction>> {
    for i in &self.inst_list {
      if i.as_any().downcast_ref::<LoadInst>().is_some() || 
      i.as_any().downcast_ref::<StoreInst>().is_some() { // TODO: or Callbase
        return Some(i);
      }
    }
    None
  }

  // Return a const iterator range over the instructions in the block, skipping
  // any debug instructions. Skip any pseudo operations as well if skip_pseudo_op
  // is true.
  pub fn instructions_without_debug(&mut self, skip_pseudo_op: bool)
    -> &Vec<Box<dyn Instruction>>
  {
    self.inst_list.retain(|i| // TODO: DbgInfoIntrinsic
      !skip_pseudo_op && i.as_any().downcast_ref::<PseudoProbeInst>().is_some());
    &self.inst_list
  }

  // Return the size of the basic block ignoring debug instructions.
  pub fn size_without_debug(&mut self) -> usize {
    self.instructions_without_debug(true).len()
  }

  // Unlink 'this' from the containing function, but not delete it.
  pub fn remove_from_parent(&mut self) {
  }

  pub fn earse_from_parent() {}
  pub fn move_before() {}
  pub fn move_after() {}
  pub fn insert_into() {}
  pub fn get_single_predecessor() {}
  pub fn get_unique_predecessor() {}
  pub fn has_n_predecessors() {}
  pub fn has_n_predecessors_or_more() {}
  pub fn get_single_successor() {}
  pub fn get_unique_successor() {}
  pub fn print() {}
  pub fn begin() {}
  pub fn end() {}
  pub fn rbegin() {}
  pub fn rend() {}

  pub fn size(&self) -> usize {
    self.inst_list.len()
  }

  pub fn empty(&self) -> bool {
    self.inst_list.is_empty()
  }

  pub fn front(&self) -> Option<&Box<dyn Instruction>> {
    self.inst_list.first()
  }

  pub fn back(&self) -> Option<&Box<dyn Instruction>> {
    self.inst_list.last()
  }

  pub fn phis() {}
  pub fn get_value_symbol_table() {}
  pub fn drop_all_references() {}
  pub fn remove_predecessor() {}
  pub fn can_split_predecessors() {}
  pub fn split_basic_block() {}
  pub fn split_basic_block_before() {}
  pub fn has_address_taken() {}
  pub fn replace_phi_uses_with() {}
  pub fn replace_successors_phi_uses_with() {}

  // Return true if this basic block is an exception handling block.
  pub fn is_eh_pad(&self) -> bool {
    self.get_first_non_phi().unwrap().is_eh_pad()
  }

  // Return true if this basic block is a landing pad.
  // Being a 'landing pad' means that the basic block is the destination
  // of the 'unwind' edge of an invoke instruction.
  pub fn is_landing_pad(&self) -> bool {
    let inst = self.get_first_non_phi();
    if inst.unwrap().as_any().downcast_ref::<LandingPadInst>().is_some() {
      return true;
    }
    false
  }

  // Return the landingpad instruction associated with the landingpad.
  pub fn get_landing_pad_inst(&self) -> Option<&LandingPadInst> {
    let inst = self.get_first_non_phi();
    inst.unwrap().as_any().downcast_ref::<LandingPadInst>()
  }

  // Return true if it is legal to hoist instructions into this block.
  pub fn is_legal_to_hoist_into(&self) -> bool {
    // No terminator means the block is under construction.
    let term = self.get_terminator();
    if term.is_none() { return true; }

    // If the block has no successors, there can be no instructions to hoist.
    debug_assert!(term.unwrap().get_num_successors() > 0);

    // Instructions should not be hoisted across exception handling boundaries.
    !term.unwrap().is_exceptional_terminator()
  }

  // Return true if this is the entry block of the containing function.
  // This method can only be used on blocks that have a parent function.
  pub fn is_entry_block(&self) -> bool {
    let f = self.get_parent();
    debug_assert!(f.is_some(), "Block must have a parent function to use this API.");
    //self == f.unwrap().get_entry_block()
    self.equal(f.as_ref().unwrap().get_entry_block().unwrap())
  }

  pub fn get_irr_loop_header_weight() {}
  pub fn is_instr_order_valid() {}
  pub fn invalidate_orders() {}
  pub fn renumber_instructions() {}
  pub fn validate_instr_ordering() {}

  fn get_basic_block_bits() {}
  fn set_basic_block_bits() {}
  fn adjust_block_address_ref_count() {}
  fn set_value_subclass_data() {}
  fn skip_debug_intrinsics() {}

  fn get_inst_list(&self) -> &Vec<Box<dyn Instruction>> {
    &self.inst_list
  }

  fn equal(&self, rhs: &BasicBlock) -> bool {
    if self.v_type.get_type_id() != rhs.v_type.get_type_id() {
      return false;
    }
    if self.inst_list.len() != rhs.inst_list.len() {
      return false;
    }
    // TODO: Add more equality checking.
    true
  }
}


impl Value for BasicBlock {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::BasicBlockVal
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}