struct ConcurrentMarker {}

impl ConcurrentMarker {
  pub fn new() {}

  pub fn start() {}

  pub fn cancel() {}

  pub fn join_for_testing() {}

  pub fn notify_incremental_mutator_step_completed() {}

  pub fn is_active() {}

  pub fn heap() {}

  pub fn marking_worklists() {}

  pub fn incremental_marking_schedule() {}

  pub fn create_concurrent_marking_visitor() {}

  fn incremental_marking_priority_if_needed() {}
}
