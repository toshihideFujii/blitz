#![allow(dead_code)]

// This file defines the DomTreeUpdater class, which provides a
// uniform way to update dominator tree related data structures.

struct DomTreeUpdater {}
impl DomTreeUpdater {
  pub fn new() {}
  pub fn is_lazy() {}
  pub fn is_eager() {}
  pub fn has_dom_tree() {}
  pub fn has_post_dom_tree() {}
  pub fn has_pending_deleted_bb() {}
  pub fn is_bb_pending_deletion() {}
  pub fn has_pending_updates() {}
  pub fn has_pending_dom_tree_updates() {}
  pub fn has_pending_post_dom_tree_updates() {}
  pub fn apply_updates() {}
  pub fn apply_updates_permissive() {}
  pub fn recalculate() {}
  pub fn delete_bb() {}
  pub fn callback_delete_bb() {}
  pub fn get_dom_tree() {}
  pub fn get_post_dom_tree() {}
  pub fn flush() {}
  pub fn dump() {}

  fn validate_delete_bb() {}
  fn force_flush_delete_bb() {}
  fn apply_dom_tree_updates() {}
  fn apply_posst_dom_tree_updates() {}
  fn try_flush_deleted_bb() {}
  fn drop_out_of_date_updates() {}
  fn erase_del_bb_node() {}
  fn is_update_valid() {}
  fn is_self_dominance() {}
}