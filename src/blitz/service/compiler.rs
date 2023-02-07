#![allow(dead_code)]

// Abstract superclass describing the result of an ahead-of-time
// compilation.
struct AotCompilationResult {}

impl AotCompilationResult {
  pub fn serialize_as_string() {}
  pub fn load_executable() {}
}

// Abstract superclass describing options to an ahead-of-time
// compilation.
struct AotCompilationOptions {}

impl AotCompilationOptions {
  pub fn platform_id() {}

  pub fn replica_count() {}
  pub fn num_cores() {}
  pub fn use_spmd_partitioning() {}
  pub fn duplicate_hlo() {}
  pub fn matrix_unit_operand_precision() {}

  pub fn device_allocator() {}
  pub fn debug_options() {}

  pub fn has_static_device_assignment() {}
  pub fn fusion_config() {}

  pub fn executor() {}

  pub fn profile_version() {}

  pub fn cache_key() {}

  pub fn run_backend_only() {}

  pub fn sanitize_dataflow() {}

  pub fn target_config() {}
}