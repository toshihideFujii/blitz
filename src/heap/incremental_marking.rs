use crate::common::globals;

enum StepOrigin {
  Blitz,
  Task,
}

enum StepResult {
  NoImmediateWork,
  MoreWorkRemaining,
  WaitingForFinalization,
}

enum State {
  Stopped,
  Marking,
  Complete,
}

enum CompletionAction {
  GCViaStackGuard,
  NoGCViaStackGuard,
}

enum GCRequestType {
  None,
  CompleteMarking,
  Finalization,
}

const YOUNG_GENERATION_ALLOCATED_THRESHOLD: usize = 64 * globals::KB;
const OLD_GENERATION_ALLOCATED_THRESHOLD: usize = 256 * globals::KB;
const MIN_STEP_SIZE_IN_BYTES: usize = 64 * globals::KB;

struct PauseBlackAllocationScope {}

impl PauseBlackAllocationScope {}

struct Observer {}

struct IncrementalMarking {}

impl IncrementalMarking {
  pub fn new() {}

  pub fn marking_state() {}

  pub fn atomic_marketing_state() {}

  pub fn non_atomic_marketing_state() {}

  pub fn notify_left_trimming() {}

  pub fn transer_color() {}

  pub fn state() {}

  pub fn finalize_marking_completed() {}

  pub fn set_weak_closure_was_over_approximated_for_testing() {}

  pub fn is_stopped() {}

  pub fn is_marking() {}

  pub fn is_marking_incomplete() {}

  pub fn is_complete() {}

  pub fn is_ready_to_over_approximate_weak_closure() {}

  pub fn needs_finalization() {}

  pub fn request_type() {}

  pub fn reset_request_type() {}

  pub fn can_be_activated() {}

  pub fn was_activated() {}

  pub fn start() {}

  pub fn finalize_incrementally() {}

  pub fn update_marking_worklist_after_scavenge() {}
  pub fn update_marked_bytes_after_scavenge() {}

  pub fn hurry() {}

  pub fn finalize() {}

  pub fn stop() {}

  pub fn finalize_marking() {}

  pub fn marking_complete() {}

  pub fn epilogue() {}

  pub fn advance_with_deadline() {}

  pub fn finalize_sweeping() {}

  pub fn continue_concurrent_sweeping() {}

  pub fn support_concurrent_sweeping() {}

  pub fn step() {}

  pub fn should_do_embedder_step() {}
  pub fn embedder_step() {}

  pub fn restart_if_not_marking() {}

  pub fn white_to_grey_and_push() {}

  pub fn mark_black_and_visit_object_due_to_layout_change() {}

  pub fn mark_black_and_revisit_object() {}

  pub fn mark_black_background() {}

  pub fn is_compacting() {}

  pub fn process_black_allocated_object() {}

  pub fn heap() {}

  pub fn incremental_marking_job() {}

  pub fn black_allocation() {}

  pub fn start_black_allocation_for_testing() {}

  pub fn local_marking_worklists() {}

  pub fn deactivate() {}

  pub fn ensure_black_allocated() {}

  pub fn is_bellow_activation_thresholds() {}

  pub fn increment_live_bytes_background() {}

  pub fn start_marking() {}

  pub fn start_black_allocation() {}
  pub fn pause_black_allocation() {}
  pub fn finish_black_allocation() {}

  pub fn mark_roots() {}

  pub fn should_retain_map() {}

  pub fn retain_maps() {}

  pub fn publish_write_barrier_worklists() {}

  pub fn schedule_bytes_to_mark_based_on_time() {}
  pub fn schedule_bytes_to_mark_based_on_allocation() {}

  pub fn step_size_to_keep_up_with_allocations() {}
  pub fn step_size_to_make_progress() {}
  pub fn add_scheduled_bytes_to_mark() {}

  pub fn fast_forward_schedule() {}
  pub fn fast_forward_schedule_if_close_to_finalization() {}

  pub fn fetch_bytes_marked_concurrently() {}

  pub fn compute_step_size_in_bytes() {}

  pub fn advance_on_allocation() {}

  pub fn set_state() {}

  pub fn current_time_to_marking_task() {}
}
