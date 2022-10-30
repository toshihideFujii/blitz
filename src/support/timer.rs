
#![allow(dead_code)]

struct TimeRecord {}

impl TimeRecord {
  pub fn get_current_time() {}

  pub fn get_process_time() {}

  pub fn get_user_time() {}

  pub fn get_system_time() {}

  pub fn get_wall_time() {}

  pub fn get_mem_used() {}

  pub fn get_instructions_executed() {}

  pub fn print() {}
}

// This class is used to track the amount of time spent between invocations
// of its start_timer()/stop_timer() methods.
struct Timer {}

impl Timer {
  pub fn init() {}

  pub fn get_name() {}

  pub fn get_description() {}

  pub fn is_initialized() {}

  // Check if the timer is currently running.
  pub fn is_running() {}

  pub fn has_triggered() {}

  pub fn start_timer() {}

  pub fn clear() {}

  pub fn get_total_time() {}
}

struct TimerGroup {}

impl TimerGroup {
  pub fn set_name() {}

  pub fn print() {}

  pub fn clear() {}

  pub fn print_all() {}

  pub fn clear_all() {}

  pub fn print_json_values() {}

  pub fn print_all_json_values() {}

  pub fn construct_for_statistics() {}
}