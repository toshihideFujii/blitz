#![allow(dead_code)]

struct OProfileWrapper {
  open_agent_func: fn (),
  major_version_func: fn () -> i32,
  minor_version_func: fn () -> i32,
  is_oprofile_running_func: fn () -> bool
}
impl OProfileWrapper {
  pub fn new() {}
  pub fn open_agent() {}
  pub fn close_agent() {}
  pub fn write_native_code() {}
  pub fn write_debug_line_info() {}
  pub fn unload_native_code() {}
  pub fn major_version() {}
  pub fn minor_version() {}
  pub fn is_agent_available() {}
  fn initialize() {}
  fn check_for_oprofile_proc_entry() {}
  fn is_oprofile_running() {}
}