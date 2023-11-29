#![allow(dead_code)]

// This file declares BlitzContext, a container of global state in
// Blitz, such as the global type and constant uniquing tables.

//use std::collections::HashSet;
use std::collections::HashMap;

use crate::{
  adt::{
    //small_ptr_set::SmallPtrSet,
    dense_map::DenseMap,
    ap_int::APInt,
    ap_float::APFloat,
    folding_set::FoldingSetNodeID,
    string_map::StringMap, string_ref::StringRef,
  },
  ir::{
    //module::Module,
    diagnostic_handler::DiagnositicHandler,
    blitz_remark_streamer::BlitzRemarkStreamer,
    constants::{ConstantInt, ConstantFP, ConstantTokenNone, ConstantAggregateZero},
    attribute_impl::{AttributeImpl, AttributeListImpl, AttributeSetNode},
    metadata::{MDString, /*ValueAsMetadata, Metadata, MetadataAsValue, MDNode*/},
    value::Value,
    debug_info_metadata::DICompositeType,
    type_::{/*Type,*/ TypeID, IntegerType, BasicType},
    //global_object::GlobalObject,
    /*global_value::GlobalValueBase,*/
    tracking_md_ref::TypedTrackingMDRef,
    //function::Function
  },
  //remarks::remark_streamer::RemarkStreamer
};

use super::{function::Function, type_::{FixedVectorType, ScalableVectorType}};

// Known synchronization scope IDs, which always have the same value.
// All synchronization scope IDs that Blitz has special knowledge of
// are listed here. Additionally, this scheme always Blitz to efficiently
// check for specific synchronization scope ID without comparing strings.
#[derive(Debug)]
pub enum SyncScopeID {
  // Synchronized with respect to signal handlers executing in the same thread.
  SingleThread,
  // Synchronized with respect to all concurrently executing threads.
  System
}

// Pinned metadata names, which always have the same value.
// This is a compile-time performance optimization, not a crrectness
// optimization.
pub enum MDKind {
  MdDbg = 0,
  MdTbaa = 1,
  MdProf = 2,
  MdFpMath = 3,
  MdRange = 4,
  MdTbaaStruct = 5,
  MdInvariantLoad = 6,
  MdAliasScope = 7,
  MdNoAlias = 8,
  MdNonTemporal = 9,
  MdMemParallelLoopAccess = 10,
  MdNonNull = 11,
  MdDererenceable = 12,
  MdDererenceableOrNull = 13,
  MdMakeInmplicit = 14,
  MdUnpredictable = 15,
  MdInvariantGroup = 16,
  MdAlign = 17,
  MdLoop = 18,
  MdType = 19,
  MdSectionPrefix = 20,
  MdAbsoluteSymbol = 21,
  MdAssociated = 22,
  MdCallees = 23,
  MdIrrLoop = 24,
  MdAccessGroup = 25,
  MdCallback = 26,
  MdPreserveAccessIndex = 27,
  MdVcallVisibility = 28,
  MdNoundef = 29,
  MdAnnotation = 30,
  Nosanitize = 31,
  MdFuncSanitize = 32,
  MdExclude = 33,
  MdMemprof = 34,
  MdCallsite = 35,
  MdKcfiType = 36,
  MdPcsections = 37,
  MdDIAssingId = 38
}

enum OperandBundle {
  Deopt,
  Funclet,
  GcTransition,
  CfGuardTarget,
  Preallocated,
  GcLive,
  ArcAttachedCall,
  PtrAuth,
  Kcfi
}

// This is an important class for using Blitz in a threaded context.
// It (opaquely) owns and manages the core global data of Blitz's core
// infrastructure, including the type and constant uniquing tables.
// BlitzContext itself provides no locking guarantees, so you should be
// careful to have one context per thread.
#[derive(Debug)]
pub struct BlitzContext {
  //pub p_impl: Option<Box<BlitzContextImpl>>,

  // The set of modules instantiated in this context, and which
  // will be automatically deleted if this context is deleted.
  //owned_modules: SmallPtrSet<Module>,

  // The main remark streamer used by all the other streamers (e.g.
  // Ir, MIR, frontends, etc.).
  // This should only be used by the specific srreamers, and never
  // directly.
  //main_remark_streamer: Option<RemarkStreamer>,
  diag_handler: Option<DiagnositicHandler>,
  respect_diagnostic_filters: bool,
  diagnostics_hotness_requested: bool,
  // The minimum hotness value a diagnostic needs in order to be
  // included in optimization diagnostics.
  diagnostics_hotness_threshold: Option<u64>,
  // The percentage of difference between profiling branch weights
  // and expect branch weights to tolerate when emiting MisExpect diagnostics.
  diagnostics_mis_expect_tolerance: Option<u32>,
  mis_expect_warning_requested: bool,
  // The specialized remark streamer used by Blitz's OptimizationRemarkEmitter.
  blitz_rs: Option<BlitzRemarkStreamer>,

  pub int_constants: DenseMap<APInt, ConstantInt>,
  pub fp_constants: DenseMap<APFloat, ConstantFP>,
  pub attrs_set: HashMap<FoldingSetNodeID, AttributeImpl>, //FoldingSet<AttributeImpl>,
  pub attrs_lists: HashMap<FoldingSetNodeID, AttributeListImpl>, //FoldingSet<AttributeListImpl>,
  pub attrs_set_nodes: HashMap<FoldingSetNodeID, AttributeSetNode>, //FoldingSet<AttributeSetNode>,

  md_string_cache: StringMap<MDString>,
  //values_as_metadata: DenseMap<Value, ValueAsMetadata>,
  //metadata_as_values: DenseMap<Box<dyn Metadata>, MetadataAsValue>,

  di_type_map: DenseMap<MDString, DICompositeType>,
  //distinct_md_nodes: Vec<MDNode>,

  pub caz_constants: HashMap<String, ConstantAggregateZero>,
  the_true_val: Option<ConstantInt>,
  the_false_val: Option<ConstantInt>,

  // Basic type instances.
  pub void_type: BasicType,
  pub label_type: BasicType,
  pub fp128_type: BasicType,

  pub half_type: BasicType,
  pub b_float_type: BasicType,
  pub float_type: BasicType,
  pub double_type: BasicType,
  //metadata_type: Box<dyn Type>,
  //token_type: Box<dyn Type>,

  //x86_fp80_type: Box<dyn Type>,
  //ppc_fp128_type: Box<dyn Type>,
  pub x86_mmx_type: BasicType,
  //x86_amx_type: Box<dyn Type>,

  int_1_type: IntegerType,
  int_8_type: IntegerType,
  int_16_type: IntegerType,
  int_32_type: IntegerType,
  int_64_type: IntegerType,
  int_128_type: IntegerType,

  the_none_token: ConstantTokenNone,

  pub integer_types: DenseMap<u32, IntegerType>,
  //function_types: DenseSet<FunctionType>,
  //anon_struct_types: DenseSet<>
  pub fixed_vector_types: HashMap<String, FixedVectorType>,
  pub scalable_vector_types: HashMap<String, ScalableVectorType>,

  custom_md_kind_names: StringMap<u32>,
  pub value_metadata: HashMap<Box<dyn Value>, MDAttachments>,

  //global_object_sections: DenseMap<GlobalObject, StringRef>,
  //global_value_partitions: DenseMap<GlobalValueBase, StringRef>,
  bundle_tag_cache: StringMap<u32>,

  //gc_names: DenseMap<Function, String>
  pub discard_value_names: bool,
}

impl BlitzContext {
  pub fn new() -> Self {
    BlitzContext {
      //p_impl: None,
      //main_remark_streamer: None,
      diag_handler: None,
      respect_diagnostic_filters: false,
      diagnostics_hotness_requested: false,
      diagnostics_hotness_threshold: None,
      diagnostics_mis_expect_tolerance: None,
      mis_expect_warning_requested: false,
      blitz_rs: None,
      int_constants: DenseMap::new(),
      fp_constants: DenseMap::new(),
      attrs_set: HashMap::new(), //FoldingSet::new(),
      attrs_lists: HashMap::new(), //FoldingSet::new(),
      attrs_set_nodes: HashMap::new(), //FoldingSet::new(),
      md_string_cache: StringMap::new(),
      //metadata_as_values: DenseMap::new(),
      di_type_map: DenseMap::new(),
      //distinct_md_nodes: Vec::new(),
      caz_constants: HashMap::new(),
      the_true_val: None,
      the_false_val: None,
      void_type: BasicType::new(TypeID::Void),
      label_type: BasicType::new(TypeID::Label),
      fp128_type: BasicType::new(TypeID::Fp128),
      half_type: BasicType::new(TypeID::Half),
      b_float_type: BasicType::new(TypeID::BFloat),
      float_type: BasicType::new(TypeID::Float),
      double_type: BasicType::new(TypeID::Double),
      x86_mmx_type: BasicType::new(TypeID::X86Mmx),
      int_1_type: IntegerType::new(1),
      int_8_type: IntegerType::new(8),
      int_16_type: IntegerType::new(16),
      int_32_type: IntegerType::new(32),
      int_64_type: IntegerType::new(64),
      int_128_type: IntegerType::new(128),
      the_none_token: ConstantTokenNone::new(),
      integer_types: DenseMap::new(),
      fixed_vector_types: HashMap::new(),
      scalable_vector_types: HashMap::new(),
      custom_md_kind_names: StringMap::new(),
      value_metadata: HashMap::new(),
      //global_object_sections: DenseMap::new(),
      //global_value_partitions: DenseMap::new(),
      bundle_tag_cache: StringMap::new(),
      discard_value_names: false,
    }
    //instance.p_impl = Some(Box::new(BlitzContextImpl::new(&instance)));
    //instance
  }

  // Return a unique non-zero id for the specified metadata kind.
  pub fn get_md_kind_id(&self, _name: StringRef) -> u32 {
    0
  }

  pub fn get_md_kind_names() {}
  pub fn get_operand_bundle_tags() {}
  pub fn get_or_insert_bundle_tag() {}
  pub fn get_operand_bundle_tag_id() {}
  pub fn get_or_insert_sync_scope_id() {}
  pub fn get_sync_scope_names() {}

  // Define the GC for a function.
  pub fn set_gc(&mut self, _func: &Function, _name: String) {}

  // Return the GC for a function.
  pub fn get_gc(&self, _func: &Function) -> String {
    return String::new()
  }

  // Remove the GC for a function.
  pub fn delete_gc(&mut self, _func: &Function) {}

  // Return true if the context runtime configuration is set to discard
  // all value names. When true, only GlobalValue names will be available
  // in the IR.
  pub fn should_discard_value_names(&self) -> bool {
    self.discard_value_names
  }

  pub fn set_discard_value_names() {}
  pub fn is_odr_uniquing_debug_types() {}
  pub fn enable_debug_type_odr_uniquing() {}
  pub fn disable_debug_type_odr_uniquing() {}
  pub fn set_diagnostic_handler_callback() {}
  pub fn set_diagnostic_handler() {}
  pub fn get_diagnostic_handler_callback() {}
  pub fn get_diagnostic_context() {}
  pub fn get_diag_handler_ptr() {}
  pub fn get_diagnostic_handler() {}
  pub fn get_diagnostics_hotness_requested() {}
  pub fn set_diagnostics_hotness_requested() {}
  pub fn set_diagnostics_mis_expect_tolerance() {}
  pub fn get_diagnostics_hotness_threshold() {}
  pub fn is_diagnostics_hotness_threshold_set_from_psi() {}
  pub fn get_main_remark_streamer() {}
  pub fn set_main_remark_streamer() {}
  pub fn get_blitz_remark_streamer() {}
  pub fn set_blitz_remark_streamer() {}
  pub fn get_diagnostic_message_prefix() {}
  pub fn diagnose() {}
  pub fn set_yield_callback() {}
  pub fn yield_() {}
  pub fn emit_error() {}
  pub fn get_opt_pass_gate() {}
  pub fn set_opt_pass_gate() {}
  pub fn set_opaque_pointers() {}
  pub fn supports_typed_pointers() {}

  fn add_module() {}
  fn remove_module() {}

  //pub fn get_impl(&self) -> &Option<Box<BlitzContextImpl>> {
    //&self.p_impl
  //}

  //pub fn get_mut_impl(&mut self) -> &mut BlitzContextImpl {
    //self.p_impl.as_mut().unwrap()
  //}
  pub fn get_true_value(&self) -> Option<ConstantInt> {
    self.the_true_val.clone()
  }

  pub fn set_true_value(&mut self, value: Option<ConstantInt>) {
    self.the_true_val = value;
  }

  pub fn get_false_value(&self) -> Option<ConstantInt> {
    self.the_false_val.clone()
  }

  pub fn set_false_value(&mut self, value: Option<ConstantInt>) {
    self.the_false_val = value;
  }

  pub fn get_int_1_type(&self) -> IntegerType {
    self.int_1_type.clone()
  }

  pub fn get_int_8_type(&self) -> IntegerType {
    self.int_8_type.clone()
  }

  pub fn get_int_16_type(&self) -> IntegerType {
    self.int_16_type.clone()
  }

  pub fn get_int_32_type(&self) -> IntegerType {
    self.int_32_type.clone()
  }

  pub fn get_int_64_type(&self) -> IntegerType {
    self.int_64_type.clone()
  }

  pub fn get_int_128_type(&self) -> IntegerType {
    self.int_128_type.clone()
  }
}

static mut BLITZ_CONTEXT: Option<BlitzContext> = None;

pub fn blits_context() -> &'static BlitzContext {
  unsafe {
    if BLITZ_CONTEXT.is_none() {
      BLITZ_CONTEXT = Some(BlitzContext::new())
    }
    BLITZ_CONTEXT.as_ref().unwrap()
  }
}

pub fn blits_context_mut() -> &'static mut BlitzContext {
  unsafe {
    if BLITZ_CONTEXT.is_none() {
      BLITZ_CONTEXT = Some(BlitzContext::new())
    }
    BLITZ_CONTEXT.as_mut().unwrap()
  }
}

struct KeyType {}
impl KeyType {}

struct FunctionTypeKeyInfo {}
impl FunctionTypeKeyInfo {
  pub fn get_empty_key() {}
  pub fn get_tombstone_key() {}
  pub fn get_hash_value() {}
}

#[derive(Debug)]
pub struct Attachment {
  md_kind: u32,
  node: TypedTrackingMDRef
}

#[derive(Debug)]
pub struct MDAttachments {
  attachments: Vec<Attachment>
}

impl MDAttachments {
  pub fn new() {}

  pub fn empty(&self) -> bool {
    self.attachments.is_empty()
  }

  pub fn size(&self) -> usize {
    self.attachments.len()
  }

  // Returns the first attachment with the given id or None if no such
  // attachment exists.
  pub fn lookup(&self, _id: u32) {}

  pub fn get() {}
  pub fn get_all() {}
  pub fn set() {}
  pub fn insert() {}
  pub fn erase() {}
  pub fn remove_if() {}
}