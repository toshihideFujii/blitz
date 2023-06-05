#![allow(dead_code)]

struct MemorySSAUpdater {}
impl MemorySSAUpdater {
  pub fn new() {}
  pub fn insert_def() {}
  pub fn insert_use() {}
  pub fn remove_edge() {}
  pub fn remove_duplicate_phi_edges_between() {}
  pub fn update_phis_when_inserting_unique_backedge_block() {}
  pub fn update_for_cloned_loop() {}
  pub fn update_for_cloned_block_into_pred() {}
  pub fn update_exit_blocks_for_cloned_loop() {}
  pub fn apply_updates() {}
  pub fn apply_insert_updates() {}
  pub fn move_before() {}
  pub fn move_after() {}
  pub fn move_to_place() {}
  pub fn move_all_after_splice_blocks() {}
  pub fn move_all_after_merge_blocks() {}
  pub fn wire_old_predecessors_to_new_immediate_predecessor() {}
  pub fn create_memory_access_in_bb() {}
  pub fn create_memory_access_before() {}
  pub fn create_memory_access_after() {}
  pub fn remove_memory_access() {}
  pub fn remove_blocks() {}
  pub fn change_to_unreachable() {}
  pub fn get_memory_ssa() {}
}