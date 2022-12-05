#![allow(dead_code)]

use std::cmp;
use std::ops;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::adt::string_ref::StringRef;

#[derive(Debug, PartialEq, Clone, Copy)]
struct TimeRecord {
  wall_time_: Duration, // Wall clock time elapsed in seconds.
  user_time_: Duration, // User time elapsed.
  system_time_: Duration, // System time elapsed.
  mem_used_: usize, // Memory allocated (in bytes).
  instructions_executed_: u64 // Number of instructions executed.
}

impl TimeRecord {
  pub fn new() -> Self {
    Self {
      wall_time_: Duration::new(0, 0),
      user_time_: Duration::new(0, 0),
      system_time_: Duration::new(0, 0),
      mem_used_: 0,
      instructions_executed_: 0
    }
  }

  // Get the current time and memory usage.
  pub fn get_current_time(start: bool) -> TimeRecord {
    let mut result = TimeRecord::new();
    if start {
      result.mem_used_ = get_mem_usage();
      result.instructions_executed_ = get_current_instruction_executed();
    } else {
      result.instructions_executed_ = get_current_instruction_executed();
      result.mem_used_ = get_mem_usage();
    }
    result.wall_time_ = SystemTime::now().duration_since(UNIX_EPOCH).expect("msg");
    result
  }

  pub fn get_process_time(&self) -> Duration {
    self.user_time_ + self.system_time_
  }

  pub fn get_user_time(&self) -> Duration {
    self.user_time_
  }

  pub fn get_system_time(&self) -> Duration {
    self.system_time_
  }

  pub fn get_wall_time(&self) -> Duration {
    self.wall_time_
  }

  pub fn get_mem_used(&self) -> usize {
    self.mem_used_
  }

  pub fn get_instructions_executed(&self) -> u64 {
    self.instructions_executed_
  }

  pub fn print() {}
}

/*
impl cmp::Ord for TimeRecord {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.wall_time_.total_cmp(&other.wall_time_)
  }
}
*/

impl cmp::PartialOrd for TimeRecord {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.wall_time_.partial_cmp(&other.wall_time_)
  }
}

impl ops::AddAssign<TimeRecord> for TimeRecord {
  fn add_assign(&mut self, rhs: TimeRecord) {
    self.wall_time_ += rhs.wall_time_;
    self.user_time_ += rhs.user_time_;
    self.system_time_ += rhs.system_time_;
    self.mem_used_ += rhs.mem_used_;
    self.instructions_executed_ += rhs.instructions_executed_;
  }
}

impl ops::SubAssign<TimeRecord> for TimeRecord {
  fn sub_assign(&mut self, rhs: TimeRecord) {
    self.wall_time_ -= rhs.wall_time_;
    self.user_time_ -= rhs.user_time_;
    self.system_time_ -= rhs.system_time_;
    self.mem_used_ -= rhs.mem_used_;
    self.instructions_executed_ -= rhs.instructions_executed_;
  }
}

// This class is used to track the amount of time spent between invocations
// of its start_timer()/stop_timer() methods.
#[derive(Debug, PartialEq, Clone)]
struct Timer {
  time_: TimeRecord, // The total time captured.
  start_time_: TimeRecord, // The time start_timer() was last called.
  name_: String, // The name of this time variable.
  description_: String, // Description of this time variable.
  running_: bool, // Is the timer currently running?
  triggered_: bool, // Has the timer ever been triggered?
  tg_: TimerGroup,
  // prev_: Timer,
  // next_: Timer
}

impl Timer {
  pub fn new(timer_name_: StringRef, timer_description_: StringRef) -> Self {
    Self {
      time_: TimeRecord::new(),
      start_time_: TimeRecord::new(),
      name_: timer_name_.data(),
      description_: timer_description_.data(),
      running_: false,
      triggered_: false,
      tg_: TimerGroup::new_default()
    }
  }

  pub fn get_name(&self) -> String {
    self.name_.clone()
  }

  pub fn get_description(&self) -> String {
    self.description_.clone()
  }

  // Check if the timer is currently running.
  pub fn is_running(&self) -> bool {
    self.running_
  }

  // Check if start_timer() has ever been called on this timer.
  pub fn has_triggered(&self) -> bool {
    self.triggered_
  }

  // Start the timer running.
  pub fn start_timer(&mut self) {
    if self.running_ == true {
      panic!("Cannot start a running timer.");
    }
    self.running_ = true;
    self.triggered_ = true;
    // sign_posts.start_interval() // TODO
    self.start_time_ = TimeRecord::get_current_time(true);
  }

  // Stop the timer.
  pub fn stop_timer(&mut self) {
    if self.running_ == false {
      panic!("Cannot stop a paused timer.");
    }
    self.running_ = false;
    self.time_ += TimeRecord::get_current_time(false);
    self.time_ -= self.start_time_;
    // sigm_posts.end_interval() // TODO
  }

  // Clear the timer state.
  pub fn clear(&mut self) {
    self.running_ = false;
    self.triggered_ = false;
    self.time_ = TimeRecord::new();
    self.start_time_ = TimeRecord::new();
  }

  // Return the duration for which this timer has been running.
  pub fn get_total_time(&self) -> TimeRecord {
    self.time_
  }
}

#[derive(Debug, PartialEq, Clone)]
struct TimerGroup {
  name_: String,
  description_: String,
  timer_vec_: Vec<Timer>,
  //prev_: Box<TimerGroup>,
  //next_: Box<TimerGroup>
}

impl TimerGroup {
  pub fn new_default() -> Self {
    Self {
      name_: String::from("misc"),
      description_: String::from("Miscellaneous Ungrouped Timers"),
      timer_vec_: Vec::new()
    }
  }

  pub fn set_name(&mut self, new_name: StringRef, new_description_: StringRef) {
    self.name_ = new_name.data();
    self.description_ = new_description_.data();
  }

  pub fn print() {}

  pub fn clear() {}

  pub fn print_all() {}

  pub fn clear_all() {}

  pub fn print_json_values() {}

  pub fn print_all_json_values() {}

  pub fn construct_for_statistics() {}

  pub fn aquire_default_group() {}

  fn add_timer(&mut self, t: Timer) {
    self.timer_vec_.push(t)
  }

  fn remove_timer(&mut self, t: &Timer) {
    let mut it = self.timer_vec_.iter();
    let index = it.position(|x| x == t).unwrap();
    let result = self.timer_vec_.remove(index);
    assert_eq!(&result, t);
  }

  fn prepare_to_print_list() {}

  fn print_queued_timers() {}

  fn print_json_value() {}
}

fn get_mem_usage() -> usize {
  0
}

fn get_current_instruction_executed() -> u64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  //#[test]
  //fn test_additivity() {}

  #[test]
  fn test_check_if_triggered() {
    let mut t1 = Timer::new(StringRef::new_from_string("T1"),
      StringRef::new_from_string("T1"));
    assert_eq!(t1.has_triggered(), false);
    t1.start_timer();
    assert_eq!(t1.has_triggered(), true);
    t1.stop_timer();
    assert_eq!(t1.has_triggered(), true);
    t1.clear();
    assert_eq!(t1.has_triggered(), false);
  }
}