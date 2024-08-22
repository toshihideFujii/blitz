#![allow(dead_code)]

// Tracks allocations for the Blitz service; allocations can be registered
// with shape/device/tag and resolved from a handle for later use.
pub struct AllocationTracker {}

impl AllocationTracker {
  pub fn new() {}
  pub fn register() {}
  pub fn register_replicated_buffers() {}
  pub fn unregister() {}
  pub fn deconstruct_tuple() {}
  pub fn resolve() {}
  pub fn resolve_for_replica() {}
}