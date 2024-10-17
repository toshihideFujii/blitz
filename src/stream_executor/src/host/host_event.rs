#![allow(dead_code)]

use crate::event::Event;

// This class is a host-side implementation of the Event interface. It is
// intended to be used with the HostStream implementation.
pub struct HostEvent {}

impl HostEvent {
  pub fn new() -> Self {
    HostEvent {  }
  }
  
  pub fn notification() {}
}

impl Event for HostEvent {
  fn poll_for_status(&self) -> crate::event::EventStatus {
    unimplemented!()
  }

  fn wait_for_event_on_external_stream(&self) -> Result<(), String> {
    unimplemented!()
  }
}