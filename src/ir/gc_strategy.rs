#![allow(dead_code)]

struct GCStrategy {
  name: String,
  use_statepoints: bool,
  use_rs4_gc: bool,
  needed_safe_points: bool,
  uses_metadata: bool
}

impl GCStrategy {
  pub fn new() {}

  // Return the name of the GC strategy.
  // This is the value of the collector name string specified on
  // functions which use this strategy.
  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  // Return true if this strategy is expecting the use of gc.statepoints,
  // and false otherwise.
  pub fn use_statepoints(&self) -> bool {
    self.use_statepoints
  }

  pub fn is_gc_managed_pointer() {}

  // Returns true if the RewriteStatepointsForGC pass should run on
  // functions using this GC.
  pub fn use_rs4_gc(&self) -> bool {
    debug_assert!(!self.use_rs4_gc || self.use_statepoints(),
      "GC strategy has use_rs4_gc but not use_statepoints set.");
    self.use_rs4_gc
  }

  // If set, appropriate metadata tables must be emitted by the back-end
  // (assembler, JIT, or otherwise).
  pub fn use_metadata(&self) -> bool {
    self.uses_metadata
  }

  // True if safe points need to be inferred on call sites.
  pub fn needs_safe_points(&self) -> bool {
    self.needed_safe_points
  }
}

pub fn get_gc_strategy() {}