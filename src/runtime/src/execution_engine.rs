#![allow(dead_code)]

pub struct ExecutionEngine {}

pub struct JitOptions {
  enable_gdb_listener: bool,
  enable_perf_listener: bool,
  save_compiled_obj_file: bool,
}

pub fn create_form_module() {}

pub struct AotOptions {
  enable_gdb_listener: bool,
  enable_perf_listener: bool,
}

pub fn create_from_obj_file() {}

pub fn exported() {}

pub fn export_with_xla_runtime_abi() {}