#![allow(dead_code)]

// This file defines the MemoryDependenceAnalysis analysis pass.

struct MemDepResult {}
impl MemDepResult {
  pub fn new() {}
  pub fn get_def() {}
  pub fn get_clobber() {}
  pub fn get_non_local() {}
  pub fn get_non_func_local() {}
  pub fn get_unknown() {}
  pub fn is_clobber() {}
  pub fn is_def() {}
  pub fn is_local() {}
  pub fn is_non_local() {}
  pub fn is_non_func_local() {}
  pub fn is_unknown() {}
  pub fn get_inst() {}
  fn is_dirty() {}
  fn get_dirty() {}
}

struct NonLocalDepEntry {}
impl NonLocalDepEntry {
  pub fn new() {}
  pub fn get_bb() {}
  pub fn set_result() {}
  pub fn get_result() {}
}

struct NonLocalDepResult {}
impl NonLocalDepResult {
  pub fn new() {}
  pub fn get_bb() {}
  pub fn set_result() {}
  pub fn get_result() {}
  pub fn get_address() {}
}

struct MemoryDependenceResults {}
impl MemoryDependenceResults {
  pub fn new() {}
  pub fn invalidate() {}
  pub fn get_default_block_scan_limit() {}
  pub fn get_dependency() {}
  pub fn get_non_local_call_dependency() {}
  pub fn get_non_local_pointer_dependency() {}
  pub fn remove_instruction() {}
  pub fn invalidate_cached_pointer_info() {}
  pub fn invalidate_cached_predecessors() {}
  pub fn get_pointer_dependency_from() {}
  pub fn get_simple_pointer_dependency_from() {}
  pub fn get_invariant_group_pointer_dependency() {}
  pub fn release_memory() {}
  pub fn get_clobber_offset() {}
}

struct MemoryDependenceAnalysis {}
impl MemoryDependenceAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct MemoryDependenceWrapperPass {}
impl MemoryDependenceWrapperPass {
  pub fn new() {}
  pub fn run_on_function() {}
  pub fn release_memory() {}
  pub fn get_analysis_usage() {}
  pub fn get_mem_dep() {}
}