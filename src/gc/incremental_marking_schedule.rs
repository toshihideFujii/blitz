const NO_SET_ELAPSED_TIME_FOR_TESTING: f64 = -1.0;
const INVALID_LAST_ESTIMATED_LIVE_BYTES: i64 = -1;
const EPHEMERON_PAIRS_FLUSHING_RATIO_INCREMENTS: f64 = 0.25;

struct IncrementalMarkingSchedule {}

impl IncrementalMarkingSchedule {
  pub fn new() {}

  pub fn notify_incremental_marking_start() {}

  pub fn update_mutator_thread_marked_bytes() {}

  pub fn add_concurrently_marked_bytes() {}

  pub fn get_overall_marked_bytes() {}

  pub fn get_concurrently_marked_bytes() {}

  pub fn get_next_incremental_step_duration() {}

  pub fn set_elapsed_time_for_testing() {}

  pub fn should_flush_ephemeron_pairs() {}

  fn get_elapsed_time_in_ms() {}
}
