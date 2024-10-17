#![allow(dead_code)]

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

// The Event class, when supported by a platform, enables low-overhead status
// reporting for a Stream. An Event is inserted at a location in a stream via
// the Stream::RecordEvent() API. From then on, the Event's status can be
// monitored via the nonblocking Event::PollForStatus() call.
pub trait Event {
  // Returns the current Status for the event.
  fn poll_for_status(&self) -> EventStatus;
  
  // Blocks `stream` on this event. `stream` is a raw platform-specific
  // stream (e.g. GpuStreamHandle).
  fn wait_for_event_on_external_stream(&self) -> Result<(), String>;
}