#![allow(dead_code)]

// This file defines the interface for lazy computation of
// value constraint information.

struct LazyValueInfo {}
impl LazyValueInfo {
  pub fn new() {}
  pub fn get_predicate_on_edge() {}
  pub fn get_predicate_at() {}
  pub fn get_constant() {}
  pub fn get_constant_range() {}
  pub fn get_constant_range_at_use() {}
  pub fn get_constant_on_edge() {}
  pub fn get_constant_range_on_edge() {}
  pub fn thread_edge() {}
  pub fn erase_block() {}
  pub fn clear() {}
  pub fn print_lvi() {}
  pub fn release_memory() {}
  pub fn invalidate() {}
}

struct LazyValueAnalysis {}
impl LazyValueAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct LazyValueInfoWrapperPass {}
impl LazyValueInfoWrapperPass {
  pub fn new() {}
  pub fn get_lvi() {}
  pub fn get_analysis_usage() {}
  pub fn release_memory() {}
  pub fn run_on_function() {}
}