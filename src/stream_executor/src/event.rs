#![allow(dead_code)]

use crate::stream_executor::StreamExecutor;

// Potential states for an Event. If PollForStatus() returns anything aside
// from kPending or kComplete, an error has occurred; kUnknown is a bad state.
// Not all implementations are able to return all enumeration values. Refer to
// the platform-specific implementation for details.
pub enum EventStatus {
  Unknown,
  Error,
  Pending,
  Complete,
}

// The Event class, when supported by a platform, enables low-overhead
// status reporting for a stram.
pub struct Event {
  stream_exec: dyn StreamExecutor
}

impl Event {
  pub fn new() {}

  // Performs any platform-specific or potentially error-generating
  // initialization.
  pub fn init(&self) -> bool {
    unimplemented!()
  }

  // Returns the current Status for the event.
  pub fn poll_for_status(&self) -> EventStatus {
    unimplemented!()
  }

  // Blocks `stream` on this event. `stream` is a raw platform-specific
  // stream (e.g. GpuStreamHandle).
  pub fn wait_for_event_on_external_stream(&self) {}
}