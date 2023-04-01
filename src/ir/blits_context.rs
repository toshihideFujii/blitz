#![allow(dead_code)]

// This file declares BlitzContext, a container of global state in
// Blitz, such as the global type and constant uniquing tables.

use crate::adt::small_ptr_set::SmallPtrSet;
use super::module::Module;

enum SyncScope {
  SingleThread,
  System
}

// Pinned metadata names, which always have the same value.
// This is a compile-time performance optimization, not a crrectness
// optimization.
enum MDKind {
  Dbg,
  Tbaa,
  Prof,
  FpMath,
  Range,
  TbaaStruct,
  InvariantLoad,
  AliasScope,
  NoAlias,
  NonTemporal,
  MemParallelLoopAccess,
  NonNull,
  Dererenceable,
  DererenceableOrNull,
  MakeInmplicit,
  Unpredictable,
  InvariantGroup,
  Align,
  Loop,
  Type,
  SectionPrefix,
  AbsoluteSymbol,
  Associated,
  Callees,
  IrrLoop,
  AccessGroup,
  Callback,
  PreserveAccessIndex,
  VcallVisibility,
  Noundef,
  Annotation,
  Nosanitize,
  FuncSanitize,
  Exclude,
  Memprof,
  Callsite,
  KcfiType,
  Pcsections,
  DIAssingId
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
#[derive(Debug, Clone)]
pub struct BlitzContext {
  // The set of modules instantiated in this context, and which will be 
  // automatically deleted if this context is deleted.
  owned_modules: Box<SmallPtrSet<Module>>,
  // The main remark streamer used by all the other streamers (e.g. IR,
  // MIR, frontends, etc.).
  // This should only be used by the specific streamers, and never directly.
  main_remark_streamer: u32,
  diag_handler: u32,
  respect_diagnostic_filters: bool,
  diagnostics_hotness_requested: bool,
  // The minimum hotness value a diagnostic needs in order to be included in
  // optimization diagnostics.
  diagnostics_hotness_threshold: u64,
  diagnostics_mis_expect_tolerance: u32,
  mis_expect_warning_requested: bool,
  blitz_rs: u32,
  yield_callback: u32,
  yield_opaque_handle: u32,
  value_names: u32,
  int_constants: u32,
  fp_constants: u32,
  attrs_set: u32,
  attrs_lists: u32,
  attrs_set_nodes: u32,
  md_string_cache: u32,
  values_as_metadata: u32,
  metadata_as_values: u32,
  di_type_map: u32,
  distinct_md_nodes: u32,
  caz_constants: u32,
  array_constants: u32,
  struct_constants: u32,
  vector_constants: u32,
  cpn_constants: u32,
  ctn_constants: u32,
  uv_constants: u32,
  pv_constants: u32,
  cds_constants: u32,
  block_addresses: u32,
  ds0_local_equivalents: u32,
  no_cfi_values: u32,
  expr_constants: u32,
  inline_asms: u32,
  value_handles: u32,
  custom_md_kind_names: u32,
  value_metadata: u32,
  assignment_id_to_instrs: u32,
  global_object_sections: u32,
  global_value_partitions: u32,
  global_value_sanitize_metadata: u32,
  discriminator_table: u32,
  bundle_tag_cache: u32,
  ssc: u32,
  gc_names: u32,
  discars_value_names: u32,
  opaque_pointers: u32
}

impl BlitzContext {
  pub fn new() {}
  pub fn get_md_kind_id() {}
  pub fn get_md_kind_names() {}
  pub fn get_operand_bundle_tags() {}
  pub fn get_or_insert_bundle_tag() {}
  pub fn get_operand_bundle_tag_id() {}
  pub fn get_or_insert_sync_scope_id() {}
  pub fn get_sync_scope_names() {}
  pub fn sest_gc() {}
  pub fn should_discard_value_names() {}
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
}

struct KeyType {}
impl KeyType {}

struct FunctionTypeKeyInfo {}
impl FunctionTypeKeyInfo {
  pub fn get_empty_key() {}
  pub fn get_tombstone_key() {}
  pub fn get_hash_value() {}
}