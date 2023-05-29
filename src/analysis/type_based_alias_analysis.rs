#![allow(dead_code)]

// This is the interface for a metadata-based TBAA.
// See the source file for details on the algorithm.

// A simple AA result that uses TBAA metadata to answer queries.
struct TypeBasedAAResult {}

impl TypeBasedAAResult {
  pub fn new() {}
  pub fn alias() {}
  pub fn get_mod_ref_info_mask() {}
  pub fn get_memory_effects() {}
  pub fn get_mod_ref_info() {}
}

// Analysis pass probviding a never-invalidated alias analysis result.
struct TypeBasedAA {}
impl TypeBasedAA {
  pub fn new() {}
  pub fn run() {}
}

// Legacy wrapper pass tp provide the TypeBasedAAResult object.
struct TypeBasedAAWrapperPass {}
impl TypeBasedAAWrapperPass {
  pub fn new() {}
  pub fn get_result() {}
  pub fn do_initialization() {}
  pub fn do_finalization() {}
  pub fn get_analysis_usage() {}
}

pub fn create_type_based_aa_wrapper_pass() {}