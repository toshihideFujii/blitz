#![allow(dead_code)]

struct GCStrategy {}
impl GCStrategy {
  pub fn new() {}
  pub fn get_name() {}
  pub fn use_statepoints() {}
  pub fn is_gc_managed_pointer() {}
  pub fn use_rs4_gc() {}
  pub fn use_metadata() {}
  pub fn needs_safe_points() {}
}

pub fn get_gc_strategy() {}