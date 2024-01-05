#![allow(dead_code)]

enum Specialization {
  Enabled,
  Disabled,
  Always,
}

struct Options {
  specialization: Specialization,
}

struct Function {}

pub struct JitExecutable {
  mlir_module: String,
  opts: Options,
  functions: Vec<Function>,
  has_default_executable: bool,
  memory_region_name: String,
}

impl JitExecutable {
  pub fn new() {}
  pub fn inline_compilation_task_runner() {}
  pub fn instantiate() {}
  pub fn default_executable() {}
  pub fn get_executable() {}
  pub fn all_executables_compiled() {}

  pub fn mlir_module(&self) -> String {
    self.mlir_module
  }

  pub fn num_functions(&self) -> usize {
    self.functions.len()
  }
}