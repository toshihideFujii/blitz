struct SweepingConfig {}

struct Sweeper {}

impl Sweeper {
  pub fn new() {}

  pub fn can_discard_memeory() {}

  pub fn start() {}
  pub fn finish_if_running() {}
  pub fn notify_done_if_needed() {}

  pub fn sweep_for_allocation_if_running() {}

  pub fn is_sweeping_on_mutator_thread() {}
  pub fn is_sweeping_in_progress() {}

  pub fn perform_sweep_on_mutator_thread() {}

  fn wait_for_concurrent_sweeping_for_testing() {}
}
