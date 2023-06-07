#![allow(dead_code)]

// This file defines passes to print out IR in various granularities.
// The PrintModulePass pass simply prints out the entire module when
// it is executed.

struct PrintModulePass {}
impl PrintModulePass {
  pub fn new() {}
  pub fn run() {}
  pub fn is_required() {}
}

struct PrintFunctionPass {}
impl PrintFunctionPass {
  pub fn new() {}
  pub fn run() {}
  pub fn is_required() {}
}