struct MarkingState {}

impl MarkingState {
  pub fn new() {}

  pub fn mark_and_push() {}

  pub fn push_marked() {}

  pub fn register_weak_reference_if_needed() {}

  pub fn register_weak_callback() {}

  pub fn register_movable_reference() {}

  pub fn process_weak_container() {}

  pub fn process_ephemeron() {}

  pub fn account_marked_bytes() {}
  pub fn marked_bytes() {}

  pub fn publish() {}

  pub fn marking_worklist() {}

  pub fn not_fully_constructed_worklist() {}

  pub fn previously_not_fully_constructed_worklist() {}

  pub fn weak_callback_worklist() {}

  pub fn write_barrier_worklist() {}

  pub fn concurent_marking_bailout_worklist() {}

  pub fn discovered_ephemeron_pairs_worklist() {}

  pub fn ephemeron_pairs_for_processing_worklist() {}

  pub fn weak_containers_worklist() {}

  pub fn retrace_marked_objects_worklist() {}

  pub fn movable_slots_worklist() {}

  pub fn notify_compaction_cancelled() {}

  pub fn did_discover_new_ephemeron_pairs() {}

  pub fn reset_did_discover_new_ephemeron_pairs() {}

  fn mark_no_push() {}

  fn register_weak_container() {}

  fn is_compaction_enabled() {}
}
