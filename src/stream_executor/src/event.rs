#![allow(dead_code)]

use crate::stream_executor::StreamExecutor;

enum Status {
  Unknown,
  Error,
  Pending,
  Complete,
}

// The Event class, when supported by a platform, enables low-overhead
// status reporting for a stram.
pub struct Event {
  stream_exec: StreamExecutor
}

impl Event {
  pub fn new() {}

  pub fn init() {}

  pub fn poll_for_status() {}

  pub fn wait_for_event_on_external_stream() {}
}