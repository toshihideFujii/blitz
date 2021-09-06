struct Sweeper {}

impl Sweeper {
  pub fn sweeping_in_progress() {}
  pub fn teardown() {}
  pub fn add_page() {}
  pub fn parallel_sweep_space() {}
  pub fn parallel_sweep_page() {}
  pub fn ensure_page_is_swept() {}
  pub fn schedule_incremental_sweeping_task() {}
  pub fn raw_sweep() {}

  // After calling this function sweeping is considered to be in progress
  // and the main thread can sweep lazily, but the background sweeper tasks are not running yet.
  pub fn start_sweeping() {}
  pub fn start_sweeper_tasks() {}
  pub fn ensure_completed() {}
  pub fn drain_sweeping_worklists() {}
  pub fn drain_sweeping_worklist_for_space() {}
  pub fn are_sweeper_task_running() {}

  // Support concurrent sweepers from main thread.
  pub fn support_concurrent_sweeping() {}

  pub fn get_swept_page_safe() {}

  pub fn add_page_for_iterability() {}
  pub fn start_iterability_tasks() {}
  pub fn ensure_iterability_completed() {}
  pub fn merge_old_to_new_remembered_sets_for_swept_pages() {}
}
