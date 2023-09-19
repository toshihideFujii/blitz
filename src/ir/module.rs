#![allow(dead_code)]

// This file contains the declarations for the Module class.

use crate::adt::dense_map::DenseMap;
use crate::adt::ilist::IPList;
use crate::adt::string_map::StringMap;
//use crate::support::error::Error;
use crate::support::memory_buffer::MemoryBuffer;
use super::blits_context::*;
use super::comdat::Comdat;
use super::data_layout::DataLayout;
use super::function::Function;
use super::global_alias::GlobalAlias;
use super::global_ifunc::GlobalIFunc;
use super::global_value::GlobalValueBase;
use super::global_variable::GlobalVariable;
use super::gv_materializer::GVMaterializer;
use super::metadata::NamedMDNode;
use super::symbol_table_list::SymbolTableList;
use super::value_symbol_table::ValueSymbolTable;

enum ModFlagBehavior {
  Error,
  Warning,
  Require,
  Override,
  Append,
  AppendUnique,
  Max,
  Min
}

pub fn is_valid_mod_flag_behavior() {}
pub fn is_valid_module_flag() {}

struct ModuleFlagEntry {}

// A Module instance is used to store all the information related to
// an Blitz module.
// Modules are the top level container of all other Blitz intermediate
// representation (IR) objects.
// Each module directly contains a list of globals variables, a list of
// functions, alist of libraries (or other modules) this module depends
// on, a symbol table, and various data about the target's characteristics.
#[derive(Debug)]
pub struct Module {
  context: BlitzContext,
  module_id: String,
  global_list: SymbolTableList<GlobalVariable>,
  function_list: SymbolTableList<Function>,
  alias_list: SymbolTableList<GlobalAlias>,
  ifunc_list: SymbolTableList<GlobalIFunc>,
  named_md_list: IPList<NamedMDNode>,
  global_scope_asm: String,
  value_symbol_table: ValueSymbolTable,
  comdat_symbol_table: StringMap<Comdat>,
  owned_memory_buffer: MemoryBuffer,
  materializer: Option<GVMaterializer>,
  source_file_name: String,
  target_triple: String,
  named_md_symbol_table: StringMap<NamedMDNode>,
  dl: DataLayout,
  current_intrinsic_ids: StringMap<u32>,
  uniqued_intrinsic_names: DenseMap<u32, u32> // TODO
}

impl Module {
  pub fn new() {}

  // Get the module identifier which is, essentially, the name of the module.
  pub fn get_module_identifier(&self) -> String {
    self.module_id.clone()
  }

  // Returns the number of non-debug IR instructions in the module.
  // This is equivalent to the sum of the IR instruction counts of
  // each function contained in the module.
  pub fn get_instruction_count(&self) -> usize {
    let mut num_instrs = 0;
    for i in 0..self.function_list.size() {
      let f = self.function_list.get(i).unwrap();
      num_instrs += f.get_instruction_count();
    }
    num_instrs
  }

  // Get the module's original source file name.
  // When compiling from bitcode, this is taken from a bitcode record
  // where it was recorded.
  // For other compiles it is the same as the module_id, which would
  // contain the source file name.
  pub fn get_source_file_name(&self) -> String {
    self.source_file_name.clone()
  }

  // Gete a short name for the module.
  // This is useful for debugging or logging.
  // It is essentially a convenience wrapper around get_module_identifier().
  pub fn get_name(&self) -> String {
    self.module_id.clone()
  }

  // Get the data layout string for the module's target platform.
  pub fn get_data_layout_str(&self) -> String {
    self.dl.get_string_representation()
  }

  // Get the data layout for the module's target platform.
  pub fn get_data_layout(&self) -> DataLayout {
    self.dl.clone()
  }

  // Get the target triple which is a string describing the target host.
  pub fn get_target_triple(&self) -> String {
    self.target_triple.clone()
  }

  // Get the global data context.
  pub fn get_context(&self) -> &BlitzContext {
    &self.context
  }

  // Get any module-scope online assembly blocks.
  pub fn get_module_inline_asm(&self) -> String {
    self.global_scope_asm.clone()
  }

  pub fn create_rng() {}
  pub fn should_emit_instr_count_changed_remark() {}

  // Set the module identifier.
  pub fn set_module_identifier(&mut self, id: String) {
    self.module_id = id;
  }

  // Set the module's original source file name.
  pub fn set_source_file_name(&mut self, name: String) {
    self.source_file_name = name;
  }

  // Set the data layout.
  pub fn set_data_layout(&mut self, desc: DataLayout) {
    self.dl = desc;
  }

  // Set the target triple.
  pub fn set_target_triple(&mut self, t: String) {
    self.target_triple = t;
  }

  pub fn set_module_inline_asm() {}
  pub fn append_module_inline_asm() {}
  pub fn get_named_value() {}

  // Return the number of global values in the module.
  pub fn get_num_named_values(&self) -> usize {
    self.get_value_symbol_table().size()
  }

  pub fn get_md_kind_id() {}
  pub fn get_md_kind_names() {}
  pub fn get_operand_bundle_tags() {}
  pub fn get_identified_struct_types() {}
  pub fn get_unique_intrinsic_name() {}
  pub fn get_or_insert_function() {}
  pub fn get_function() {}
  pub fn get_global_variable() {}
  pub fn get_named_global() {}
  pub fn get_or_insert_global() {}
  pub fn get_named_alias() {}
  pub fn get_named_i_func() {}
  pub fn get_named_metadata() {}
  pub fn get_or_insert_combat() {}
  pub fn get_module_flags_metadata() {}
  pub fn get_module_flag() {}
  pub fn get_or_insert_module_flags_metadata() {}
  pub fn add_module_flag() {}
  pub fn set_module_flag() {}
  pub fn set_materializer() {}

  // Retrieves the GVMaterializer, if any, for this module.
  pub fn get_materializer(&self) -> Option<GVMaterializer> {
    self.materializer.clone()
  }

  pub fn is_materialized(&self) -> bool {
    self.materializer.is_some()
  }

  // Make sure the GlobalValue is fully read.
  pub fn materialize(&self, _gv: GlobalValueBase) {}
  pub fn materialize_all() {}
  pub fn materialize_metadata() {}

  // Detach global variable gv from the list but don't delete it.
  pub fn remove_global_variable(&mut self, _gv: &GlobalVariable) {
    for _i in 0..self.global_list.size() {
      //if self.global_list.get(i).unwrap() == gv {
        //self.global_list.remove(i);
        //return;
      //}
    }
  }
  pub fn erase_global_variable() {}

  // Insert global variable gv at the end of the global variable list
  // and take ownership.
  pub fn insert_global_variable(&mut self, _gv: &GlobalVariable) {
    //self.global_list.push_back(gv.clone());
  }

  // Get the Module's list of functions.
  pub fn get_function_list(&self) -> &SymbolTableList<Function> {
    &self.function_list
  }

  pub fn get_sublist_access(&self) -> &SymbolTableList<Function> {
    &self.function_list
  }

  // Detach alias from the list but don't delete it.
  pub fn remove_alias(&mut self, _alias: &GlobalAlias) {
    for _i in 0..self.alias_list.size() {
      //if self.alias_list.get(i).unwrap() == alias {
        //self.alias_list.remove(i);
        //return;
      //}
    }
  }

  pub fn erase_alias() {}

  // Insert alias at the end of the alias list and take ownership.
  pub fn insert_alias(&mut self, _alias: &GlobalAlias) {
    //self.alias_list.push_back(alias.clone());
  }

  // Detach ifunc from the list don't delete it.
  pub fn remove_ifunc(&mut self, ifunc: &GlobalIFunc) {
    for i in 0..self.ifunc_list.size() {
      if self.ifunc_list.get(i).unwrap() == ifunc {
        self.ifunc_list.remove(i);
        return;
      }
    }
  }

  pub fn erase_ifunc() {}

  // Insert ifunc at the end of the ifunc list and take ownership.
  pub fn insert_ifunc(&mut self, ifunc: &GlobalIFunc) {
    self.ifunc_list.push_back(ifunc.clone());
  }

  // Detach md_node from the list don't delete it.
  pub fn remove_named_md_node(&mut self, md_node: &NamedMDNode) {
    for i in 0..self.named_md_list.size() {
      if self.named_md_list.get(i).unwrap() == md_node {
        self.named_md_list.remove(i);
        return;
      }
    }
  }

  pub fn erase_named_md_node() {}

  // Insert md_node at the end of the md_node list and take ownership.
  pub fn insert_named_md_node(&mut self, _md_node: &NamedMDNode) {
    //self.named_md_list.push_back(md_node.clone());
  }

  // Get the Module's list of ifuncs.
  pub fn get_i_func_list(&self) -> &SymbolTableList<GlobalIFunc> {
    &self.ifunc_list
  }

  // Get the Module's list of named metadata.
  pub fn get_named_md_list(&self) /*-> IPList<NamedMDNode>*/ {
    //self.named_md_list.clone()
  }

  // Get the Mudule7s symbol table of global variable and function identifiers.
  pub fn get_value_symbol_table(&self) -> ValueSymbolTable {
    self.value_symbol_table.clone()
  }

  // Get the Module's symbol table for COMDAT's.
  pub fn get_comdat_symbol_table(&self) -> StringMap<Comdat> {
    self.comdat_symbol_table.clone()
  }
}