#![allow(dead_code)]

pub struct Executable {
  name: String,
  functions: Vec<Function>,
  specialization: Option<usize>,
}

impl Executable {
  pub fn initialize_call_frame() {}
  pub fn return_results() {}
  pub fn execute() {}
  pub fn name() {}
  pub fn specialization() {}
  pub fn num_functions() {}
  pub fn is_async() {}
  pub fn function_name() {}
  pub fn num_results() {}
  pub fn signature() {}
  pub fn runtime_signature() {}
  pub fn time_to_compile() {}
  pub fn obj_file() {}

  pub fn load_from_obj_file() {}
  pub fn get_arguments_memory_layout() {}
  pub fn get_results_memory_layout() {}
  pub fn get_user_data() {}
  pub fn get_diagnostic_engine() {}
  pub fn call() {}
  pub fn requires_blas() {}
}

struct ArgumentsMemoryLayout {
  num_args_ptrs: usize,
  num_ptrs: Vec<usize>,
  offsets: Vec<usize>,
}

struct ResultsMemoryLayout {
  has_async_results: bool,
  size: usize,
  offsets: Vec<usize>,
}

struct ExecuteOpts {}

struct LoadFunction {}

struct Function {
  arguments_memory_layout: ArgumentsMemoryLayout,
  results_memory_layout: ResultsMemoryLayout,
  requires_blas: bool,
}