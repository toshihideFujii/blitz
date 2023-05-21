#![allow(dead_code)]

// References to metadata that track RAUW.
use super::metadata::{Metadata, MetadataTracking};

struct TrackingMDRef {
  md: Option<Box<dyn Metadata>>
}

impl TrackingMDRef {
  pub fn new(md: Option<Box<dyn Metadata>>) -> Self {
    let instance = TrackingMDRef { md: md };
    instance.track();
    instance
  }

  pub fn get(&self) -> &Option<Box<dyn Metadata>> {
    &self.md
  }

  pub fn reset(&mut self) {
    self.untrack();
    self.md = None;
  }

  pub fn reset_by_md(&mut self, md: Option<Box<dyn Metadata>>) {
    self.untrack();
    self.md = md;
    self.track();
  }

  // Check whether this has a trivial destructor.
  pub fn has_trivial_destructor(&self) -> bool {
    self.md.is_none() || !MetadataTracking::is_replaceable(&self.md)
  }

  fn track(&self) {
    if self.md.is_some() {
      MetadataTracking::track(&self.md);
    }
  }

  fn untrack(&self) {
    if self.md.is_some() {
      MetadataTracking::untrack(&self.md);
    }
  }

  fn retrack(&self, x: &mut TrackingMDRef) {
    //debug_assert!(self.md == x.md, "Expected values to match.");
    if x.md.is_some() {
      MetadataTracking::retrack(&x.md, &self.md);
      x.md = None;
    }
  }
}

// Typed tracking ref.
pub struct TypedTrackingMDRef<T> {
  dummy: Option<T>,
  md_ref: TrackingMDRef
}

impl<T> TypedTrackingMDRef<T> {
  pub fn new() {}
  pub fn get() {}
  pub fn reset() {}
  pub fn has_trivial_destructor() {}
}