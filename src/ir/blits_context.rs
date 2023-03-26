#![allow(dead_code)]

enum SyncScope {
  SingleThread,
  System
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

#[derive(Debug, Clone)]
pub struct BlitzContext {}

impl BlitzContext {
  pub fn get_md_kind_id() {}

  pub fn get_md_kind_names() {}

  pub fn get_operand_bundle_tags() {}

  pub fn get_or_insert_bundle_tag() {}
}