#![allow(dead_code)]

// This class is a host-side implementation of the Event interface. It is
// intended to be used with the HostStream implementation.
pub struct HostEvent {}

impl HostEvent {
  pub fn new() {}
  pub fn notification() {}
  pub fn poll_for_status() {}
}