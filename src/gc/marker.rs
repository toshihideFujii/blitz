enum CollectionType {
  Minor,
  Major,
}

enum IsForcedGC {
  NotForced,
  Forced,
}

enum WriteBarrierType {
  Dijkstra,
  Steele,
}

struct MarkingConfig {}

struct IncrementalMarkingTask {}

impl IncrementalMarkingTask {
  pub fn run() {}
}

struct Marker {}

impl Marker {
  pub fn new() {}

  pub fn enter_atomic_pause() {}

  pub fn advance_marking_with_limits() {}

  pub fn leave_atomic_pause() {}

  pub fn finish_marking() {}

  pub fn process_weakness() {}

  pub fn write_barrier_for_in_construction_object() {}

  pub fn write_barrier_for_object() {}

  pub fn heap() {}

  pub fn marking_worklists_for_testing() {}

  pub fn mutator_marking_state_for_testing() {}

  pub fn visitor() {}

  pub fn clear_all_worklists_for_testing() {}

  pub fn incremental_marking_step_for_testing() {}

  pub fn set_main_thread_marking_disabled_for_testing() {}

  pub fn wait_for_concurrent_marking_for_testing() {}

  pub fn notify_compaction_cancelled() {}

  pub fn is_marking() {}

  //pub fn visitor() {}

  pub fn conservative_visitor() {}

  pub fn stack_visitor() {}

  pub fn process_worklists_with_deadline() {}

  pub fn visit_roots() {}

  pub fn visit_cross_thread_persistents_if_needed() {}

  pub fn mark_not_fully_constructed_objects() {}

  pub fn schedule_incremental_marking_task() {}

  pub fn incremental_marking_step() {}

  pub fn advance_marking_on_allocation() {}
}
