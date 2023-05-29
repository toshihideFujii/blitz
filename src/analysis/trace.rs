#![allow(dead_code)]

// This class represents a single trace of Blitz basic blocks.
// A trace is a single entry, multiple exit, region of code that
// is often hot.
// Trace-based optimizations treat traces almost like they are a
// large, strange, basic block: because the trace path is assumed
// to be hot, optimizations for the fall-through path are made at
// the expense of the non-fall-through paths.

struct Trace {}
impl Trace {
  pub fn new() {}
  pub fn get_entry_basic_block() {}
  pub fn get_block() {}
  pub fn get_function() {}
  pub fn get_module() {}
  pub fn get_block_index() {}
  pub fn contains() {}
  pub fn dominates() {}
  pub fn size() {}
  pub fn empty() {}
  pub fn erase() {}
  pub fn print() {}
  pub fn dump() {}
}