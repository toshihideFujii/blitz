#![allow(dead_code)]

struct MemoryAccess {}
impl MemoryAccess {
  pub fn new() {}
  pub fn class_of() {}
  pub fn get_block() {}
  pub fn print() {}
  pub fn dump() {}
  pub fn set_block() {}
  pub fn get_id() {}
}

struct MemoryUseOrDef {}
impl MemoryUseOrDef {
  pub fn new() {}
  pub fn get_memory_inst() {}
  pub fn get_defininf_access() {}
  pub fn class_of() {}
  pub fn is_optimized() {}
  pub fn get_optimized() {}
  pub fn set_optimized() {}
  pub fn reset_optimized() {}
}

struct MemoryUse {}
impl MemoryUse {
  pub fn new() {}
  pub fn class_of() {}
  pub fn is_optimized() {}
  pub fn get_optimized() {}
  pub fn set_optimized() {}
  pub fn reset_optimized() {}
  fn delete_me() {}
}

struct MemoryDef {}
impl MemoryDef {
  pub fn new() {}
  pub fn class_of() {}
  pub fn is_optimized() {}
  pub fn get_optimized() {}
  pub fn set_optimized() {}
  pub fn reset_optimized() {}
  pub fn print() {}
  pub fn get_id() {}
}

struct MemoryPhi {}
impl MemoryPhi {
  pub fn new() {}
  pub fn blocks() {}
  pub fn vincoming_values() {}
  pub fn get_num_incoming_values() {}
  pub fn get_incoming_value() {}
  pub fn get_operand_num_for_incoming_value() {}
  pub fn get_incoming_value_num_for_operand() {}
  pub fn get_incoming_block() {}
  pub fn set_incoming_block() {}
  pub fn add_incoming() {}
  pub fn get_basic_block_index() {}
  pub fn get_incoming_value_for_block() {}
  pub fn unordered_delete_incoming() {}
  pub fn unordered_delete_incoming_if() {}
  pub fn unordered_delete_incoming_block() {}
  pub fn unordered_delete_incoming_value() {}
  pub fn class_of() {}
  pub fn print() {}
  pub fn get_id() {}
  pub fn alloc_hungoff_uses() {}
}

struct MemorySSA {}
impl MemorySSA {
  pub fn new() {}
  pub fn get_walker() {}
  pub fn get_skip_self_walker() {}
  pub fn get_memory_access() {}
  pub fn get_dom_tree() {}
  pub fn dump() {}
  pub fn print() {}
  pub fn is_live_on_entry_def() {}
  pub fn get_live_on_entry_def() {}
  pub fn get_block_accesses() {}
  pub fn get_block_defs() {}
  pub fn locally_dominates() {}
  pub fn dominates() {}
  pub fn verify_memory_ssa() {}
  pub fn ensure_optimized_uses() {}
  pub fn get_aa() {}
  pub fn verify_ordering_domination_and_def_uses() {}
  pub fn verify_domination_numbers() {}
  pub fn verify_prev_def_in_phis() {}
  pub fn get_writable_block_accesses() {}
  pub fn get_writable_block_defs() {}
  pub fn move_to() {}
  pub fn rename_pass() {}
  pub fn remove_from_lookups() {}
  pub fn remove_from_lists() {}
  pub fn insert_into_lists_before() {}
  pub fn create_defined_access() {}
}

struct MemorySSAUtil {}
impl MemorySSAUtil {
  pub fn new() {}
  pub fn def_clobbers_use_or_def() {}
}

struct MemorySSAPrinterLegacyPass {}
impl MemorySSAPrinterLegacyPass {
  pub fn new() {}
  pub fn run_on_function() {}
  pub fn get_analysis_usage() {}
}

struct MemorySSAAnalysis {}
impl MemorySSAAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct MemorySSAPrinterPass {}
impl MemorySSAPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

struct  MemorySSAWalkerPrinterPass {}
impl MemorySSAWalkerPrinterPass {
  pub fn new() {}
  pub fn run() {}
}