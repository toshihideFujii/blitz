#![allow(dead_code)]

// This file contains the declaration of the Function class,
// which represents a single function/procedure.

use std::{any::Any, collections::HashSet};
use crate::{
  adt::{
    string_ref::StringRef, floating_point_mode::FPClassTest,
    twine::Twine, /*dense_set::DenseSet*/
  },
  ir::{
    type_::{FunctionType, Type},blits_context::BlitzContext,
    value::{ValueType, Value}, calling_conv::*,
    attributes::{AttributeList, AttrKind, Attribute},
    //symbol_table_list::SymbolTableList,
    basic_block::BasicBlock,
    metadata::Metadata,
    module::Module, argument::Argument, metadata::MDNode,
    debug_info_metadata::DISubprogram,
    global_value::{LinkageTypes, /*GlobalValueBase,*/ GlobalValue, GLOBAL_VALUE_SUB_CLASS_DATA_BITS},
    constant::Constant
  },
  support::{alignment::MaybeAlign, mod_ref::{MemoryEffects, ModRefInfo},
  code_gen::UWTableKind}
};

use super::global_object::{GlobalObject, BitKind, GLOBAL_OBJECT_MASK};

enum FnBitKind {
  IsMaterializableBit = 0
}

#[derive(PartialEq, Clone)]
pub enum ProfileCountType {
  Real,
  Sunthetic
}

// Class to represent profile counts.
// This class represents both real and synthetic profile counts.
pub struct ProfileCount {
  count: u64,
  pct: ProfileCountType
}

impl ProfileCount {
  pub fn new(count: u64, pct: ProfileCountType) -> Self {
    ProfileCount { count: count, pct: pct }
  }

  pub fn get_count(&self) -> u64 {
    self.count
  }

  pub fn get_type(&self) -> ProfileCountType {
    self.pct.clone()
  }

  pub fn is_synthetic(&self) -> bool {
    self.pct == ProfileCountType::Sunthetic
  }
}

#[derive(Debug)]
pub enum IntrinsicID {
  FAdd,
  FSub,
  FMul,
  FDiv,
  FRem,
  FPExt,
  SIToFP,
  UIToFP,
  FPToSI,
  FPToUI,
  FPTrunc,
  NotIntrinsic
}

#[derive(Debug)]
pub struct Function {
  v_type: FunctionType,
  v_id: ValueType,
  sub_class_data: u32,
  has_metadata: bool,
  parent: Option<Module>,
  int_id: IntrinsicID,
  has_blitz_reserved_name: bool,
  basic_blocks: Vec<BasicBlock>, //SymbolTableList<BasicBlock>,
  arguments: Vec<Argument>,
  attribute_sets: AttributeList
}

impl Function {
  // If the (optional) Module argument is specified, the function is
  // automatically inserted into the end of the function list for the Module.
  pub fn new(t: FunctionType, _linkage: LinkageTypes,
    _addr_space: usize, _name: Twine, parent_module: Option<Module>) -> Self
  {
    let mut func =
      Function {
        v_type: t,
        v_id: ValueType::FunctionVal,
        sub_class_data: 0,
        has_metadata: false,
        parent: parent_module,
        int_id: IntrinsicID::NotIntrinsic,
        has_blitz_reserved_name: false,
        basic_blocks: Vec::new(), //SymbolTableList::new(),
        arguments: Vec::new(),
        attribute_sets: AttributeList::new_default()
      };
    debug_assert!(FunctionType::is_valid_return_type(func.get_return_type()),
      "Invalid return type.");

    // We only need a symbol table for a function if the context keeps value names.
    if !func.get_context().should_discard_value_names() {
      // TODO
    }

    // If the function has arguments, mark them as lazily built.
    if func.v_type.get_num_params() > 0 {
      func.sub_class_data = 1;
    }

    if func.parent.is_some() {
      //func.parent.unwrap().get_function_list().push_back(func);
    }

    func.has_blitz_reserved_name = func.get_name().starts_with("blitz.");

    //if func.int_id != 0 {
      // TODO
    //}

    func
  }

  pub fn get_function(&self) -> &Function {
    self
  }

  pub fn get_parent(&self) -> &Option<Module> {
    &self.parent
  }

  // Returns the number of non-debug IR instructions in this function.
  // This is equivalent to the sum of the sizes of each basic block contained
  // within this function.
  pub fn get_instruction_count(&self) -> usize {
    let mut num_instrs = 0;
    for bb in &self.basic_blocks {
      num_instrs += bb.instruction_count_without_debug();
    }
    num_instrs
  }
  
  // Return the FunctionType for me.
  pub fn get_function_type(&self) -> &FunctionType{
    &self.v_type
  }

  // Returns the type of the ret val.
  pub fn get_return_type(&self) -> &Box<dyn Type> {
    self.v_type.get_return_type()
  }

  // Return true if this function takes a variable number of arguments.
  pub fn is_var_arg(&self) -> bool {
    self.get_function_type().is_var_arg()
  }

  pub fn is_materializable(&self) -> bool {
    self.get_global_object_sub_class_data() &
      (1 << FnBitKind::IsMaterializableBit as u32) != 0
  }

  pub fn set_is_materializable(&mut self, v: bool) {
    let mask = 1 << FnBitKind::IsMaterializableBit as u32;
    let mut val = 0; if v { val = mask; }
    let data = !mask & self.get_global_object_sub_class_data() | val;
    self.set_global_object_sub_class_data(data);
  }

  // This method returns the ID number of the specified function, or
  // Intrinsic::not_intrinsic if the function is not an intrinsic, or
  // if the pointer is null.
  // This value is always defined to be zero to allow easy checking for
  // whether a function is intrinsic or not.
  // The particular intrinsic functions which correspond to this value
  // are defined in intrinsics.rs.
  pub fn get_intrinsic_id(&self) -> &IntrinsicID {
    &self.int_id
  }

  // Return true if the function's name starts with "blitz.".
  // It's possible for this function to return true while get_intrinsic_id()
  // returns NotIntrinsic!
  pub fn is_intrinsic(&self) -> bool {
    self.has_blitz_reserved_name
  }

  // Return true if int_id is an intrinsic specific to a certain target.
  // If it is a generic intrinsic false is returned.
  pub fn is_target_intrinsic_id(_int_id: &IntrinsicID) -> bool {
    false
  }

  // Returns true if this function is an intrinsic and the intrinsic is
  // specific to a certain target. If this is not an intrinsic or a generic
  // intrinsic, false is returned.
  pub fn is_target_intrinsic(&self) -> bool {
    Function::is_target_intrinsic_id(&self.int_id)
  }

  // Returns true if the function is one of the "Constrained Floating-Point
  // Intrinsics". Returns false if not, and returns false when get_intrinsic_id()
  // returns Intrinsic::_not_intrinsic.
  pub fn is_constrained_fp_intrinsic(&self) -> bool {
    match self.get_intrinsic_id() {
      IntrinsicID::FAdd => return true,
      IntrinsicID::FSub => return true,
      IntrinsicID::FMul => return true,
      IntrinsicID::FDiv => return true,
      IntrinsicID::FRem => return true,
      IntrinsicID::FPExt => return true,
      IntrinsicID::FPToSI => return true,
      IntrinsicID::FPToUI => return true,
      IntrinsicID::FPTrunc => return true,
      _ => return false
    };
  }

  pub fn lookup_intrinsic_id(_nmae: StringRef) -> IntrinsicID {
    IntrinsicID::NotIntrinsic
  }

  // Recalculate the id for this function if it is an intrinsic defined in
  // blitz/intrinsics.rs.
  pub fn recalculate_intrinsic_id(&mut self) {
    let name = self.get_name();
    if !name.starts_with("blitz.") {
      self.has_blitz_reserved_name = false;
      self.int_id = IntrinsicID::NotIntrinsic;
      return;
    }
    self.has_blitz_reserved_name = true;
    self.int_id = Function::lookup_intrinsic_id(name);
  }

  // Get the calling convention of this function.
  pub fn get_calling_conv(&self) -> u32 {
    (self.get_subclass_data_from_value() >> 4) & CallingConv::MaxId as u32
  }

  // Set the calling convention of this function.
  pub fn set_calling_conv(&mut self, cc: u32) {
    let val = (self.get_subclass_data_from_value() & 0xc00f) | (cc << 4);
    self.set_value_subclass_data(val);
  }

  // Set the entry count for this function.
  // Entry count is the number of times this function was executed based on
  // pgo data. imports points to a set of GUIDs that needes to be imported
  // by the function for sample PGO, to enable the same inlines as the profiled
  // optimized bunary.
  //pub fn set_entry_count(&self, _count: ProfileCount,
  //  _imports: Option<DenseSet<GlobalValueBase>>) {}

  // Get the entry count for this function.
  // Entry count is the number of times the function was executed.
  // When allow_synthetic is false, only pgo_data will be returned.
  pub fn get_entry_count(&self, _allow_synthetic: bool) -> Option<ProfileCount> {
    None
  }

  // Return true if the function is annotated with profile data.
  pub fn has_profile_data(&self, include_synthetic: bool) -> bool {
    self.get_entry_count(include_synthetic).is_some()
  }

  // Return the set of GUIDs  that needs to be imported to the function for
  // sample PGO, to enable the same inlines as the profiled optimized binary.
  pub fn get_import_guids(&self) -> HashSet<u64> {
    HashSet::new()
  }

  pub fn set_section_prefix() {}

  // The name of the garbage collection algorithm to use during code generation.
  pub fn has_gc(&self) -> bool {
    self.get_subclass_data_from_value() & (1 << 14) != 0
  }

  pub fn get_gc() {}
  pub fn set_gc(&self) {}
  pub fn clear_gc() {}

  // Return the attribute list for this function.
  pub fn get_attributes(&self) -> &AttributeList {
    &self.attribute_sets
  }

  // Set the attribute list for this function.
  pub fn set_attributes(&mut self, attrs: AttributeList) {
    self.attribute_sets = attrs;
  }
  
  pub fn add_attribute_at_index(&self) {}

  // Add function attributes to this function.
  pub fn add_fn_attr(&self, _kind: AttrKind) {}

  pub fn add_fn_attrs(&self) {}
  pub fn add_ret_attr(&self, _kind: AttrKind) {}
  pub fn add_ret_attrs(&self) {}
  pub fn add_param_attr(&self) {}

  // Adds the attribute to the list of attributes for the given arg.
  pub fn add_param_attr_by_kind(&mut self, arg_no: usize,
    kind: &AttrKind) -> AttributeList
  {
    let mut c = self.get_context_mut().clone();
    let attr_sets = &self.attribute_sets;
    attr_sets.add_param_attribute_by_kind(&mut c, arg_no, kind)
  }

  pub fn add_param_attrs(&self) {}
  pub fn remove_attribute_at_index(&self) {}

  // Remove function attributes from this function.
  pub fn remove_fn_attr(&self, _kind: AttrKind) {}

  pub fn remove_fn_attrs(&self) {}
  pub fn remove_ret_attr(&self) {}
  pub fn remove_ret_attrs(&self) {}

  // Removes the attribute from the list of attributes.
  pub fn remove_param_attr(&mut self, arg_no: usize, kind: &AttrKind) {
    let mut c = self.get_context_mut().clone();
    self.attribute_sets.remove_param_attribute_by_kind(&mut c, arg_no, kind);
  }

  pub fn remove_param_attrs() {}

  // Return true if the function has the attribute.
  pub fn has_fn_atribute(&self, kind: &AttrKind) -> bool {
    self.attribute_sets.has_fn_attr(kind)
  }

  // Check if an attribute is in the list of attributes for the return value.
  pub fn has_ret_attribute(&self, kind: &AttrKind) -> bool {
    self.attribute_sets.has_ret_attr(kind)
  }

  // Check if an attributes is in the list of attributes.
  pub fn has_param_attribute(&self, arg_no: usize, kind: &AttrKind) -> bool {
    self.attribute_sets.has_param_attr(arg_no, kind)
  }

  // Gets the attribute from the list of attributes.
  pub fn get_attribute_at_index(&self, i: usize, kind: &AttrKind) -> Option<Attribute> {
    self.attribute_sets.get_attribute_at_index(i, kind)
  }

  pub fn get_fn_attribute() {}

  // Gets the specified attribute from the list of attributes.
  pub fn get_param_attribute(&self, arg_no: usize, kind: &AttrKind) -> Option<Attribute> {
    self.attribute_sets.get_param_attr(arg_no, kind)
  }

  pub fn remove_param_undef_implying_attrs() {}
  pub fn get_fn_stack_align() {}
  pub fn has_stack_protector_fn_attr() {}
  pub fn add_dereferenceable_param_attr() {}
  pub fn add_dereferenceable_or_null_param_attr() {}
  pub fn get_param_alignment() {}

  pub fn get_param_align(&self, arg_no: usize) -> Option<MaybeAlign> {
    self.attribute_sets.get_param_alignment(arg_no)
  }

  pub fn get_param_stack_align(&self, arg_no: usize) -> Option<MaybeAlign> {
    self.attribute_sets.get_param_stack_alignment(arg_no)
  }

  pub fn get_param_by_val_type(&self, arg_no: usize) -> Option<Box<dyn Type>> {
    self.attribute_sets.get_param_by_val_type(arg_no)
  }

  // Extract the sret type for a parameter.
  pub fn get_param_struct_ret_type(&self, arg_no: usize) -> Option<Box<dyn Type>> {
    self.attribute_sets.get_param_struct_ret_type(arg_no)
  }

  // Extract the inalloca type for a parameter.
  pub fn get_param_in_alloca_type(&self, arg_no: usize) -> Option<Box<dyn Type>> {
    self.attribute_sets.get_param_in_alloca_type(arg_no)
  }

  // Extract the byref type for a parameter.
  pub fn get_param_by_ref_type(&self, arg_no: usize) -> Option<Box<dyn Type>> {
    self.attribute_sets.get_param_by_ref_type(arg_no)
  }

  pub fn get_param_preallocated_type() {}

  // Extract the number of dereferenceable bytes for a parameter.
  pub fn get_param_dereferenceable_bytes(&self, arg_no: usize) -> u64 {
    self.attribute_sets.get_param_dereferenceable_bytes(arg_no)
  }

  // Extract the number of dereferenceable_or_null bytes for a parameter.
  pub fn get_param_dereferenceable_or_null_bytes(&self, arg_no: usize) -> u64 {
    self.attribute_sets.get_param_dereferenceable_or_null_bytes(arg_no)
  }

  // Extract the nofpclass attribute for a parameter.
  pub fn get_param_no_fp_class(&self, arg_no: usize) -> FPClassTest {
    self.attribute_sets.get_param_no_fp_class(arg_no)
  }

  // Determine if the function is presplit coroutine.
  pub fn is_presplit_coroutine(&self) -> bool {
    self.has_fn_atribute(&AttrKind::PresplitCoroutine)
  }

  pub fn set_presplit_coroutine(&self) {
    self.add_fn_attr(AttrKind::PresplitCoroutine)
  }

  pub fn set_splitted_coroutine(&self) {
    self.remove_fn_attr(AttrKind::PresplitCoroutine)
  }

  pub fn get_memory_effects(&self) -> MemoryEffects {
    self.attribute_sets.get_memory_effects()
  }

  pub fn set_memory_effects(&self, _me: MemoryEffects) {}

  // Determine if the function does not access memory.
  pub fn does_not_access_memory(&self) -> bool {
    self.get_memory_effects().does_not_access_memory()
  }

  pub fn set_does_not_access_memory(&self) {
    self.set_memory_effects(MemoryEffects::none())
  }

  // Determine if the function does not access or only reads memory.
  pub fn only_reads_memory(&self) -> bool {
    self.get_memory_effects().only_reads_memory()
  }

  pub fn set_only_reads_memory(&self) {
    self.set_memory_effects(MemoryEffects::read_only())
  }

  // Determine if the function does not access ot only writes memory.
  pub fn only_writes_memory(&self) -> bool {
    self.get_memory_effects().only_writes_memory()
  }

  pub fn set_only_writes_memory(&self) {
    self.set_memory_effects(MemoryEffects::write_only())
  }

  // Determine if the call can access memory only using pointers
  // based on its arguments.
  pub fn only_accesses_arg_memory(&self) -> bool {
    self.get_memory_effects().only_accesses_arg_pointees()
  }

  pub fn set_only_accesses_arg_memory(&self) {
    self.set_memory_effects(
      MemoryEffects::arg_mem_only(ModRefInfo::ModRef))
  }

  // Determine if the function may only access memory that is
  // inaccessible from the IR.
  pub fn only_accesses_inaccessible_memory(&self) -> bool {
    self.get_memory_effects().only_accesses_inaccessible_mem()
  }

  pub fn set_only_accesses_inaccessible_memory(&self) {
    self.set_memory_effects(
      MemoryEffects::inaccessible_mem_only(ModRefInfo::ModRef))
  }

  // Determine if the function may only access memory that is
  // either inaccessible from the IR or pointed to by its arguments.
  pub fn only_accesses_inaccessible_mem_or_arg_mem(&self) -> bool {
    self.get_memory_effects().only_accesses_inaccessible_or_arg_mem()
  }
  
  pub fn set_only_accesses_inaccessible_mem_or_arg_mem(&self) {
    self.set_memory_effects(
      MemoryEffects::inaccessible_or_arg_mem_only(ModRefInfo::ModRef))
  }

  // Determine if the function cannot return.
  pub fn does_not_return(&self) -> bool {
    self.has_fn_atribute(&AttrKind::NoReturn)
  }

  pub fn set_does_not_return(&self) {
    self.add_fn_attr(AttrKind::NoReturn)
  }

  // Determine if the function should not perform indirect branch tracking.
  pub fn does_no_cf_check(&self) -> bool {
    self.has_fn_atribute(&AttrKind::NoCfCheck)
  }

  // Determine if the function cannot unwind.
  pub fn does_not_throw(&self) -> bool {
    self.has_fn_atribute(&AttrKind::NoUnwind)
  }

  pub fn set_does_not_throw(&self) {
    self.add_fn_attr(AttrKind::NoUnwind)
  }

  // Determine if the call cannot duplicated.
  pub fn cannot_duplicate(&self) -> bool {
    self.has_fn_atribute(&AttrKind::NoDuplicate)
  }

  pub fn set_cannot_duplicate(&self) {
    self.add_fn_attr(AttrKind::NoDuplicate)
  }

  // Determine if the call is convergent.
  pub fn is_convergent(&self) -> bool {
    self.has_fn_atribute(&AttrKind::Convergent)
  }

  pub fn set_convergent(&self) {
    self.add_fn_attr(AttrKind::Convergent)
  }

  pub fn set_not_convergent(&self) {
    self.remove_fn_attr(AttrKind::Convergent)
  }

  // Determine if the call has sideeffects.
  pub fn is_speculatable(&self) -> bool {
    self.has_fn_atribute(&AttrKind::Speculatable)
  }

  pub fn set_speculatable(&self) {
    self.add_fn_attr(AttrKind::Speculatable)
  }

  // Determine if the call might deallocate memory.
  pub fn does_not_free_memory(&self) -> bool {
    self.only_reads_memory() || self.has_fn_atribute(&AttrKind::NoFree)
  }

  pub fn set_does_not_free_memory(&self) {
    self.add_fn_attr(AttrKind::NoFree)
  }

  // Determine if the call can synchronize with other threads.
  pub fn has_no_sync(&self) -> bool {
    self.has_fn_atribute(&AttrKind::NoSync)
  }

  pub fn set_no_sync(&self) {
    self.add_fn_attr(AttrKind::NoSync)
  }

  // Determine if the function is known not to recurse, directly
  // or indirectly.
  pub fn does_not_recurse(&self) -> bool {
    self.has_fn_atribute(&AttrKind::NoRecurse)
  }

  pub fn set_does_not_recurse(&self) {
    self.add_fn_attr(AttrKind::NoRecurse)
  }

  // Determine if the function is required to make forward progress.
  pub fn must_progress(&self) -> bool {
    self.has_fn_atribute(&AttrKind::MustProgress) ||
    self.has_fn_atribute(&AttrKind::WillReturn)
  }

  pub fn set_must_progress(&self) {
    self.add_fn_attr(AttrKind::MustProgress)
  }

  /// Determine if the function will return.
  pub fn will_return(&self) -> bool {
    self.has_fn_atribute(&AttrKind::WillReturn)
  }

  pub fn set_will_return(&self) {
    self.add_fn_attr(AttrKind::WillReturn)
  }

  // Get what kind of unwind table entry to generate for this function.
  pub fn get_uw_table_kind(&self) -> UWTableKind {
    self.attribute_sets.get_uw_table_kind()
  }

  pub fn has_uw_table(&self) -> bool {
    self.get_uw_table_kind() != UWTableKind::None
  }

  pub fn set_uw_table_kind(&self) {}

  // True if this function needs an unwind table.
  pub fn needs_unwind_table_entry(&self) -> bool {
    self.has_uw_table() || !self.does_not_throw() || self.has_personality_fn()
  }

  // Determine if the function returns a structure through first or
  // second pointer argument.
  pub fn has_struct_ret_attr(&self) -> bool {
    self.attribute_sets.has_param_attr(0, &AttrKind::StructRet) ||
    self.attribute_sets.has_param_attr(1, &AttrKind::StructRet)
  }

  // Determine if the parameter or return value is marked with no_alias
  // attribute.
  pub fn return_does_not_alias(&self) -> bool {
    self.attribute_sets.has_ret_attr(&AttrKind::NoAlias)
  }

  pub fn set_return_does_not_alias(&self) {
    self.add_ret_attr(AttrKind::NoAlias)
  }

  // Do not optimize this function (-O0).
  pub fn has_opt_none(&self) -> bool {
    self.has_fn_atribute(&AttrKind::OptimizeNone)
  }

  // Optimize this function for minimum size (-Oz).
  pub fn has_min_size(&self) -> bool {
    self.has_fn_atribute(&AttrKind::MinSize)
  }

  // Optimize this function for size (-Os) or minimum size (-Oz).
  pub fn has_opt_size(&self) -> bool {
    self.has_fn_atribute(&AttrKind::OptimizeForSize) || self.has_min_size()
  }
  
  pub fn get_denormal_mode() {}
  pub fn copy_attributes_from() {}
  pub fn delete_body() {}
  pub fn remove_from_parent() {}
  pub fn earse_from_parent() {}
  pub fn steal_argument_list_from() {}

  // Get the underlying elements of the Function.
  pub fn get_basic_block_list(&self) -> &Vec<BasicBlock> /*&SymbolTableList<BasicBlock>*/ {
    &self.basic_blocks
  }

  pub fn get_sublist_access() {}
  pub fn get_entry_block() {}
  pub fn get_value_symbol_table() {}

  pub fn size(&self) -> usize {
    //self.basic_blocks.size()
    self.basic_blocks.len()
  }

  pub fn empty(&self) -> bool {
    //self.basic_blocks.empty()
    self.basic_blocks.is_empty()
  }

  pub fn front(&self) -> Option<&BasicBlock> {
    //self.basic_blocks.front()
    self.basic_blocks.first()
  }

  pub fn back(&self) -> Option<&BasicBlock> {
    //self.basic_blocks.back()
    self.basic_blocks.last()
  }

  pub fn arg_begin() {}
  pub fn arg_end() {}

  pub fn get_arg(&self, i: usize) -> Option<&Argument> {
    self.arguments.get(i)
  }

  pub fn args(&self) -> &Vec<Argument> {
    &self.arguments
  }

  pub fn arg_size(&self) -> usize {
    self.arguments.len()
  }

  pub fn arg_empty(&self) -> bool {
    self.arguments.is_empty()
  }

  // Check whether this function has a persomnality function.
  pub fn has_personality_fn(&self) -> bool {
    self.get_subclass_data_from_value() & (1<<3) != 0
  }

  pub fn get_personality_fn() {}
  pub fn set_personality_fn() {}

  // Check whether this function has a prefix data.
  pub fn has_prefix_data(&self) -> bool {
    self.get_subclass_data_from_value() & (1<<1) != 0
  }

  pub fn get_prefix_data() {}
  pub fn set_prefix_data() {}

  // Check whether this function has a prologue data.
  pub fn has_prologue_data(&self) -> bool {
    self.get_subclass_data_from_value() & (1<<2) != 0
  }

  pub fn get_prologue_data() {}
  pub fn set_prologue_data() {}

  pub fn print() {}

  pub fn view_cfg() {}
  pub fn view_cfg_only() {}

  // Methods for support type inquiry through isa, cast, and dyn_cast.
  pub fn class_of(v: Box<dyn Value>) -> bool {
    v.get_value_id() == ValueType::FunctionVal
  }

  pub fn drop_all_references() {}
  pub fn has_address_taken() {}
  pub fn is_def_trivially_dead() {}
  pub fn calls_function_that_returns_twice() {}

  // Set the attached subprogram.
  pub fn set_sub_program(&self, _sp: DISubprogram) {
    //self.set_metadata(kind_id, node)
  }

  pub fn is_debug_info_for_profiling() {}
  pub fn null_pointer_is_defined() {}
}

impl Value for Function {
  fn get_type(&self) -> &dyn Type {
    &self.v_type
  }

  fn get_context(&self) -> &BlitzContext {
    self.v_type.get_context()
  }

  fn get_context_mut(&mut self) -> &mut BlitzContext {
    self.v_type.get_context_mut()
  }

  fn get_value_id(&self) -> ValueType {
    self.v_id.clone()
  }

  fn get_subclass_data_from_value(&self) -> u32 {
    self.sub_class_data
  }

  fn set_value_subclass_data(&mut self, val: u32) {
    self.sub_class_data = val;
  }

  fn get_metadata_by_string(&self, _kind: StringRef) -> Option<Box<dyn Metadata>> {
    if !self.has_metadata() { return None; }
    None
  }

  fn set_metadata(&mut self, _kind_id: u32, _node: Option<Box<dyn MDNode>>) {}
}

impl Constant for Function {
  fn as_any(&self) -> &dyn Any { self }
}

impl GlobalValue for Function {
  fn get_global_value_sub_class_data(&self) -> u32 {
    self.sub_class_data
  }

  fn set_global_value_sub_class_data(&mut self, v: u32) {
    debug_assert!(v < (1 << GLOBAL_VALUE_SUB_CLASS_DATA_BITS),
      "It will not fit.");
    self.sub_class_data = v;
  }

  fn is_no_builtin_fn_def(&self) -> bool {
    if self.empty() {
      return false;
    }
    self.has_fn_atribute(&AttrKind::NoBuiltin)
  }

  // Functions are definitions if they have a body.
  fn is_declaration(&self) -> bool {
    self.empty() && !self.is_materializable()
  }
}

impl GlobalObject for Function {
  fn get_global_object_sub_class_data(&self) -> u32 {
    let value_data = self.get_global_value_sub_class_data();
    value_data >> BitKind::GlobalObjectBits as u32
  }

  fn set_global_object_sub_class_data(&mut self, v: u32) {
    let old_data = self.get_global_value_sub_class_data();
    let val = (old_data & GLOBAL_OBJECT_MASK) | (v << BitKind::GlobalObjectBits as u32);
    self.set_global_value_sub_class_data(val);
    debug_assert!(self.get_global_object_sub_class_data() == v,
      "Representation error.");
  }
}