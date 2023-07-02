#![allow(dead_code)]

// This file contains the declaration of the BasicBlock class.

use crate::adt::twine::Twine;

use super::{
  function::Function,
  symbol_table_list::SymbolTableList,
  instruction::Instruction,
  value::{Value, ValueType}, type_::{Type}, blits_context::BlitzContext, module::Module
};

// This represents a single basic block in Blitz.
// A basic blocks is simply a container of instructions that execute
// sequentially.
#[derive(Debug)]
pub struct BasicBlock {
  v_type: Box<dyn Type>,
  inst_list: SymbolTableList<Instruction>,
  parent: Option<Function>
}

impl BasicBlock {
  // If the function parameter is specified, the basic block is automatically
  // inserted at either the end of the function (if insert_before is null),
  // or before the specified basic block.
  pub fn new(c: &BlitzContext, _name: &Twine, new_parent: Option<Function>,
    insert_before: Option<BasicBlock>) -> Self
  {
    let bb = BasicBlock {
      v_type: Box::new(super::type_::get_label_type(c)),
      inst_list: SymbolTableList::new(),
      parent: None
    };
    if new_parent.is_some() {
      // TODO
    } else {
      debug_assert!(insert_before.is_none(),
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
  pub fn get_module(&self) -> &Module {
    self.get_parent().as_ref().unwrap().get_parent()
  }

  // Returns the terminator instruction if the block is well formed or
  // null if the block is not well formed.
  pub fn get_terminator(&self) -> Option<&Instruction> {
    if self.inst_list.empty() ||
       self.inst_list.back().as_ref().unwrap().is_terminator(){
      return None;
    }
    self.inst_list.back()
  }

  pub fn get_terminating_deoptimize_call() {}
  pub fn get_post_dominating_deoptimize_call() {}
  pub fn get_terminating_must_tail_call() {}
  pub fn get_first_non_phi() {}
  pub fn get_first_non_phi_or_dbg() {}
  pub fn get_first_non_phi_or_dbg_or_lifetime() {}
  pub fn get_first_insertion_pt() {}
  pub fn get_first_non_phi_or_dbg_or_alloca() {}
  pub fn instructions_without_debug() {}
  pub fn size_without_debug() {}

  // Unlink 'this' from the containing function, but not delete it.
  pub fn remove_from_parent(&mut self) {
    // TODO: How to get index ?
    //self.get_parent().as_mut().unwrap().get_basic_block_list().remove(0);
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
  pub fn size() {}
  pub fn empty() {}
  pub fn front() {}
  pub fn back() {}
  pub fn phis() {}
  pub fn get_inst_list() {}
  pub fn get_sublist_access() {}
  pub fn get_value_symbol_table() {}
  pub fn drop_all_references() {}
  pub fn remove_predecessor() {}
  pub fn can_split_predecessors() {}
  pub fn split_basic_block() {}
  pub fn split_basic_block_before() {}
  pub fn has_address_taken() {}
  pub fn replace_phi_uses_with() {}
  pub fn replace_successors_phi_uses_with() {}
  pub fn is_ehpad() {}
  pub fn is_landing_pad() {}
  pub fn get_landing_pad_inst() {}
  pub fn is_legal_to_hoist_into() {}
  pub fn is_entry_block() {}
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
}


impl Value for BasicBlock {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_context(&self) -> &BlitzContext {
    self.v_type.get_context()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::BasicBlockVal
  }
}