#![allow(dead_code)]

// This file implements a model runner wrapping an AOTcompiled
// ML model. Only inference is supported.

struct ReleaseModeModelRunner {}
impl ReleaseModeModelRunner {
  pub fn new() {}
  pub fn class_of() {}
}

struct NoopSavedModelImpl {}
impl NoopSavedModelImpl {
  pub fn new() {}
  pub fn lookup_arg_index() {}
  pub fn lookup_result_index() {}
  pub fn run() {}
  pub fn result_data() {}
  pub fn arg_data() {}
}