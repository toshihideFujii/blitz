#![allow(dead_code)]

pub struct AotCompilationResult {}

impl AotCompilationResult {
  pub fn new() {}
  pub fn serialize_as_string() {}
  pub fn load_executable() {}
}

struct TargetConfig {}

struct CompileOptions {}

pub struct Compiler {}

impl Compiler {
  pub fn platform_id() {}
  pub fn run_hlo_passes() {}
  pub fn assign_buffers() {}
  pub fn run_backend() {}
  pub fn run_backend_with_buffer_assignment() {}
  pub fn load_compilation_result() {}
  pub fn compile() {}
  pub fn compute_backend_configs() {}
  pub fn compute_default_backend_config() {}
  pub fn compile_ahead_of_time() {}
  pub fn register_compiler_factory() {}
  pub fn get_for_pkatform() {}
  pub fn shape_size_bytes_function() {}
  pub fn buffer_size_bytes_function() {}
  pub fn default_device_shape_representation() {}
  pub fn export() {}
  pub fn create_metrics_hook() {}
  fn get_platform_compiler_factories() {}
  fn get_platform_compilers() {}
}

// Abstract superclass describing options to an ahead_of-time compilation.
struct AotCompilationOptions {}

impl AotCompilationOptions {
  pub fn new() {}
  pub fn platform_id() {}
  pub fn replica_count() {}
  pub fn num_cores() {}
  pub fn use_spmd_partitioning() {}
  pub fn auto_spmd_partitioning_mesh_shape() {} 
}