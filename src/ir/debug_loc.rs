#![allow(dead_code)]

// This file defines a number of light weight data structures
// used to describe and track debug location information.

use super::{tracking_md_ref::TypedTrackingMDRef, metadata::MDNode,
  debug_info_metadata::DILocation};

// A debug info location.
// This class is a wrapper around a tracking reference to an
// DILocation pointer.
#[derive(Debug)]
pub struct DebugLoc {
  loc: TypedTrackingMDRef
}

impl DebugLoc {
  // Constructs from an DILocation.
  pub fn new(l: DILocation) -> Self {
    DebugLoc { loc: TypedTrackingMDRef::new(Box::new(l)) }
  }
  
  // Get the underlying DILocation.
  pub fn get(&self) -> Option<&DILocation> {
    if self.loc.get().as_ref().is_some() {
      let metadata = self.loc.get().as_ref().unwrap();
      return metadata.as_any().downcast_ref::<DILocation>();
    }
    None
  }

  // Check whether this has a trivial destructor.
  pub fn has_trivial_destructor(&self) -> bool {
    self.loc.has_trivial_destructor()
  }

  pub fn append_inlined_at() {}

  pub fn get_line(&self) -> u32 {
    debug_assert!(self.get().is_some(), "Expected valid DebigLoc.");
    self.get().as_ref().unwrap().get_line()
  }

  pub fn get_col(&self) -> u32 {
    debug_assert!(self.get().is_some(), "Expected valid DebigLoc.");
    self.get().as_ref().unwrap().get_column()
  }

  pub fn get_scope() {}
  pub fn get_inlined_at() {}
  pub fn get_inlined_at_scope() {}
  pub fn replace_inlined_at_subprogram() {}
  pub fn get_fn_debug_loc() {}
  pub fn get_as_md_node(&self) -> Option<Box<dyn MDNode>> { None }

  // Check if the DebugLoc corresponds to an implicit code.
  pub fn is_implicit_code(&self) -> bool {
    if self.get().is_some() {
      return self.get().as_ref().unwrap().is_implicit_code();
    }
    true
  }

  pub fn set_implicit_code(&mut self, _implicit_code: bool) {
    if self.get().is_some() {
      //let mut loc_ref = self.get();
      //let loc = loc_ref.as_deref_mut().unwrap();
    }
  }
  pub fn dump() {}
  pub fn print() {}
}