#![allow(dead_code)]

// This file defines the JitEventListener interface, which lets
// users get callbacks when significant events happen during the
// JIT compilation process.

struct JitEventListener {}
impl JitEventListener {
  pub fn new() {}
  pub fn notify_object_loaded() {}
  pub fn notify_freeing_object() {}
  pub fn create_gdb_registration_listener() {}
  pub fn create_intel_jit_event_listener() {}
  pub fn create_oprofile_jit_event_listener() {}
  pub fn create_perf_jit_event_listener() {}
  fn anchor() {}
}