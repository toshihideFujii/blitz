#![allow(dead_code)]

use std::collections::HashMap;

use common::blitz_data::{DebugOptions, Precision};
use hlo::hlo_module::HloModule;
use stream_executor::{device_memory_allocator::DeviceMemoryAllocator, platform::Platform, stream_executor::StreamExecutor};

use crate::{computation_placer::DeviceAssignment, executable::Executable};

// Abstract superclass describing the result of an ahead-of-time compilation.
pub struct AotCompilationResult {}

impl AotCompilationResult {
  pub fn serialize_as_string(&self) -> String {
    unimplemented!()
  }

  pub fn load_executable(
    &self,
    _compiler: &Compiler,
    _executor: &dyn StreamExecutor) -> Result<Executable, String>
  {
    unimplemented!()
  }

  // Returns the optimized HLO module if one was computed and the implementation
  // supports it.
  pub fn optimized_module(&self) -> &HloModule {
    unimplemented!()
  }

  pub fn custom_optimized_module(&self) -> &HloModule {
    unimplemented!()
  }
}

// Abstract superclass describing metadata produced during ahead-of-time
// compilation.
pub struct AotCompilationMetadata {}

impl AotCompilationMetadata {
  pub fn new() {}
  pub fn to_string() {}
}

pub struct TargetConfig {}

struct CompileOptions {}

// Abstract compiler interface that is subclassed for compilation on a
// particular platform.
//
// The compiler ties together high level optimization (HLO) and low level
// optimization (LLO) / codegen (CG) to generate efficient executables for the
// target platform.
//
// The platform-based compiler singletons are registered via module initializers
// in their corresponding Blitz compiler libraries, and are registered via the
// RegisterCompilerFactory API below.
//
// Thread-safety: subclasses of Compiler must be thread-safe, as multiple
// Blitz clients may be requesting compilation concurrently for a given
// platform.
pub struct Compiler {}

impl Compiler {
  // Returns the ID of the platform that this compiler targets.
  pub fn platform_id(&self) -> i64 {
    unimplemented!()
  }

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

  // Returns the compiler singleton pointer if it is available for the given
  // platform, or an error status if it is not.
  pub fn get_for_platform(_platform: &dyn Platform) -> Result<&Self, String>
  {
    unimplemented!()
  }

  pub fn shape_size_bytes_function() {}
  pub fn buffer_size_bytes_function() {}
  pub fn default_device_shape_representation() {}
  pub fn export() {}
  pub fn create_metrics_hook() {}

  // Map from platform kind to compiler factory.
  fn get_platform_compiler_factories() -> HashMap<i64, Self> {
    unimplemented!()
  }

  // Map from platform kind to compiler instance, if we made one already (based
  // on the factories above).
  fn get_platform_compilers() -> HashMap<i64, Self> {
    unimplemented!()
  }
}

// Abstract superclass describing options to an ahead_of-time compilation.
pub struct AotCompilationOptions {
  plaatform_id: i64,
  device_allocator: Option<DeviceMemoryAllocator>,
  debug_options: DebugOptions,
  static_device_assignment: Option<DeviceAssignment>,
  fusion_config: Vec<Vec<bool>>,
  executor: Option<Box<dyn StreamExecutor>>,
  profile_version: i64,
  cache_key: String,
  run_backend_only: bool,
  sanitize_dataflow: bool,
  sanitize_abilists_dataflow: Vec<String>,
  target_config: Option<TargetConfig>
}

impl AotCompilationOptions {
  pub fn new() {}

  // Returns the ID of the platform to which these options apply.
  pub fn platform_id(&self) -> i64 {
    self.plaatform_id
  }

  pub fn replica_count(&self) -> i64 { 0 }

  pub fn num_cores(&self) -> i64 { 0 }

  pub fn use_spmd_partitioning(&self) -> bool { false }

  pub fn use_auto_spmd_partitioning(&self) -> bool { false }

  pub fn auto_spmd_partitioning_mesh_shape(&self) -> Vec<i64> { vec![] } 

  pub fn auto_spmd_partitioning_mesh_ids(&self) -> Vec<i64> { vec![] }

  pub fn duplicaate_hlo(&self) -> bool { false }

  pub fn matrix_unit_operand_precision(&self) -> Precision {
    Precision::Default
  }

  // Optional allocator that may be used for allocating temp space on the device
  // during compilation.
  pub fn device_allocator(&self) -> &Option<DeviceMemoryAllocator> {
    &self.device_allocator
  }

  pub fn set_device_allocator(&mut self, device_allocator: DeviceMemoryAllocator) {
    self.device_allocator = Some(device_allocator);
  }

  pub fn debug_options(&self) -> &DebugOptions {
    &self.debug_options
  }

  pub fn mutable_debug_options(&mut self) -> &mut DebugOptions {
    &mut self.debug_options
  }

  pub fn has_static_device_assignment(&self) -> bool {
    self.static_device_assignment.is_some()
  }

  pub fn static_device_assignment(&self) -> &DeviceAssignment {
    assert!(self.static_device_assignment.is_some());
    self.static_device_assignment.as_ref().unwrap()
  }

  pub fn set_static_device_assignment(&mut self, device_assignment: DeviceAssignment) {
    self.static_device_assignment = Some(device_assignment);
  }

  pub fn fusion_config_collection(&self) {}
  pub fn set_fusion_config_collection(&mut self) {}

  pub fn fusion_config(&self) -> &Vec<Vec<bool>> {
    &self.fusion_config
  }

  pub fn set_fusion_config(&mut self, fusion_config: Vec<Vec<bool>>) {
    self.fusion_config = fusion_config;
  }

  pub fn executor(&self) -> &Option<Box<dyn StreamExecutor>> {
    &self.executor
  }

  pub fn set_executor(&mut self, executor: Box<dyn StreamExecutor>) {
    self.executor = Some(executor);
  }

  // Optional profile_version and cache key may be used to trigger recompilation
  // when a compilation cache is used.
  pub fn profile_version(&self) -> i64 {
    self.profile_version
  }

  pub fn set_profile_version(&mut self, profile_version: i64) {
    self.profile_version = profile_version;
  }

  pub fn cache_key(&self) -> String {
    self.cache_key.clone()
  }

  pub fn set_cache_key(&mut self, cache_key: String) {
    self.cache_key = cache_key;
  }

  pub fn run_backend_only(&self) -> bool {
    self.run_backend_only
  }

  pub fn set_run_backend_only(&mut self, run_backend_only: bool) {
    self.run_backend_only = run_backend_only;
  }

  pub fn sanitize_dataflow(&self) -> bool {
    self.sanitize_dataflow
  }

  pub fn set_sanitize_dataflow(&mut self, sanitize_dataflow: bool) {
    self.sanitize_dataflow = sanitize_dataflow;
  }

  pub fn sanitize_abilists_dataflow(&self) -> &Vec<String> {
    &self.sanitize_abilists_dataflow
  }

  pub fn set_sanitize_abilists_dataflow(&mut self, abilists: Vec<String>) {
    self.sanitize_abilists_dataflow = abilists;
  }

  pub fn target_config(&self) -> &Option<TargetConfig> {
    &self.target_config
  }

  pub fn set_target_config(&mut self, target_config: TargetConfig) {
    self.target_config = Some(target_config);
  }
}