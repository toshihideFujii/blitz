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

// Abstract compiler interface that is subclassesd for compilation
// on a particular platform.
struct Compiler {}
impl Compiler {
  // Returns the id of the platform that this compiler targets.
  pub fn platform_id() {}

  // Runs hlo passes to optimize the given hlo module, returns the
  // optimized module.
  pub fn run_hlo_passes() {}

  // Performs scheduling and buffer assignment and returns the buffer
  // assignments.
  pub fn assign_buffers() {}

  // Compiles the hlo module for execution on a device given by the
  // executor, and returns an executable object or an error status.
  pub fn run_backend() {}

  // Returns a (deserialized) AotCompilationResult from a serialized
  // AotCompilationResult.
  pub fn load_aot_compilation_result() {}

  // Compiles a set of hlo modules that can run in parallel, potentially
  // comminucating data between the modules, and returns a corresponding
  // sequence of executable objects.
  pub fn compile() {}

  // Returns the backend configurations that the backend will consider
  // for the given hlo.
  pub fn compute_backend_configs() {}

  // Returns the backend configurations that the backend chooses by default
  // for the given hlo.
  pub fn compute_default_backend_config() {}

  // Compiles the hlo module group for ahead-of-time execution.
  pub fn compile_ahead_of_time() {}

  // Registers the compiler singleton for the platfform.
  pub fn register_compiler_factory() {}

  // Returns the compiler singleton pointer if it is available for the given
  // platrform.
  pub fn get_for_platform() {}

  // Returns a function that computes the size in bytes of the logical
  // buffer that contains a shape.
  pub fn shape_size_bytes_function() {}

  // Returns a function that computes the size in bytes of a given logical
  // buffer.
  pub fn buffer_size_bytes_function() {}

  // Returns a MetricsHookInterface object used to instrument Compiler's
  // compilation stages.
  pub fn create_metrics_hook() {}
}