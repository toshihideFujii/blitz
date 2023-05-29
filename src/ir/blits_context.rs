#![allow(dead_code)]

// This file declares BlitzContext, a container of global state in
// Blitz, such as the global type and constant uniquing tables.

use crate::adt::string_ref::StringRef;

use super::blitz_context_impl::BlitzContextImpl;

enum SyncScope {
  SingleThread,
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
#[derive(Debug, Clone, PartialEq)]
pub struct BlitzContext {
  p_impl: Option<Box::<BlitzContextImpl>>
}

impl BlitzContext {
  pub fn new() -> Self {
    let mut instance = BlitzContext { p_impl: None };
    instance.p_impl = Some(Box::new(BlitzContextImpl::new(&instance)));
    instance
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

  pub fn get_impl(&self) -> BlitzContextImpl {
    let pimpl = self.p_impl.clone();
    pimpl.unwrap().as_ref().clone()
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